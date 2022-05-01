use std::env;
use std::time::Duration;

use futures::StreamExt;
use pw_telegram_bot_fork::*;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use crate::types::assembly_ai::{AssemblyAiTranscript, AssemblyAiTranscriptStatus, AssemblyAiUpload};

mod types;

const ASSEMBLY_AI_TOKEN: &str = "";
const OPENAI_TOKEN: &str = "";
const TELEGRAM_API_TOKEN: &str = "";

#[inline(always)]
fn get_assembly_ai_token() -> String {
    env::var("ASSEMBLY_AI_TOKEN")
        .unwrap_or_else(|_| ASSEMBLY_AI_TOKEN.to_string())
}

#[inline(always)]
fn get_open_ai_token() -> String {
    env::var("OPENAI_TOKEN")
        .unwrap_or_else(|_| OPENAI_TOKEN.to_string())
}

#[inline(always)]
fn get_telegram_api_token() -> String {
    env::var("TELEGRAM_API_TOKEN")
        .unwrap_or_else(|_| TELEGRAM_API_TOKEN.to_string())
}

fn build_file_url(
    token: &str,
    file_path: &str,
) -> String {
    format!(
        "https://api.telegram.org/file/bot{}/{}",
        get_telegram_api_token(),
        file_path,
    )
}

async fn get_file(
    token: &str,
    file_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = build_file_url(token, file_path);
    let mut response = reqwest::get(&url).await?;

    let mut out = response.bytes_stream();

    let mut buffer = Vec::new();

    while let Some(chunk) = out.next().await {
        buffer.extend(chunk?);
    }

    Ok(buffer)
}

fn build_gpt3_prompt(
    text: &str,
) -> String {
    format!(
        "----\nMessage from user:\n----{}\n----\nSummary:\n----",
        text,
    )
}

async fn get_gpt3_summary(
    text: &str,
) -> Result<openai_api_fork::api::Completion, Box<dyn std::error::Error>> {
    let client =
        openai_api_fork::Client::new(
            &get_open_ai_token(),
        );

    let args = openai_api_fork::api::CompletionArgs::builder()
        .prompt(build_gpt3_prompt(text))
        .engine("text-davinci-002")
        .max_tokens(1024)
        .temperature(1.0)
        .presence_penalty(0.8)
        .frequency_penalty(0.8)
        .top_p(1.0)
        .stop(vec!["----".into()])
        .build()?;

    Ok(client.complete_prompt(args).await?)
}

async fn upload_audio_file(
    audio_file: Vec<u8>,
) -> Result<AssemblyAiUpload, Box<dyn std::error::Error>> {
    let endpoint = "https://api.assemblyai.com/v2/upload";

    let mut headers = HeaderMap::new();

    headers.insert(
        "authorization",
        get_assembly_ai_token().parse().unwrap(),
    );

    headers.insert(
        "transfer-encoding",
        "chunked".parse().unwrap(),
    );

    let file_stream = reqwest::Body::from(audio_file);

    let response = reqwest::Client::new()
        .post(endpoint)
        .body(file_stream)
        .headers(headers)
        .send()
        .await?;

    Ok(response.json::<AssemblyAiUpload>().await?)
}

async fn request_transcript(
    audio_url: &str
) -> Result<AssemblyAiTranscript, Box<dyn std::error::Error>> {
    let endpoint = "https://api.assemblyai.com/v2/transcript";

    let json = json!({
        "audio_url": audio_url
    });

    let mut headers = HeaderMap::new();

    headers.insert("authorization", get_assembly_ai_token().parse().unwrap());
    headers.insert("content-type", "application/json".parse().unwrap());

    let response = reqwest::Client::new()
        .post(endpoint)
        .json(&json)
        .headers(headers)
        .send()
        .await?;

    Ok(response.json::<AssemblyAiTranscript>().await?)
}

async fn get_transcript_by_id(
    transcript_id: &str
) -> Result<AssemblyAiTranscript, Box<dyn std::error::Error>> {
    let endpoint =
        &format!("https://api.assemblyai.com/v2/transcript/{}", transcript_id);

    let mut headers = HeaderMap::new();

    headers.insert("authorization", get_assembly_ai_token().parse().unwrap());

    let response = reqwest::Client::new()
        .get(endpoint)
        .headers(headers)
        .send()
        .await?;

    Ok(response.json::<AssemblyAiTranscript>().await?)
}

#[macro_export]
macro_rules! ok_or_continue {
    ( $x:expr $(,)? ) => {
        match $x {
            Ok(x) => x,
            Err(e) => {
                dbg!(e);

                continue;
            },
        }
    };
}

#[macro_export]
macro_rules! some_or_continue {
    ( $x:expr $(,)? ) => {
        match $x {
            Some(x) => x,
            None => continue,
        }
    };
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = get_telegram_api_token();
    let api = Api::new(token);

    let mut stream = api.stream();

    'main_loop: while let Some(update) = stream.next().await {
        let update = ok_or_continue!(update);

        if let UpdateKind::Message(message) = update.kind {
            let mut file_id: String;
            let mut file_path: String;

            match message.kind {
                MessageKind::Audio { ref data, .. } => {
                    file_id = data.file_id.clone();

                    file_path =
                        some_or_continue!(
                            ok_or_continue!(
                                api.send(
                                    pw_telegram_bot_fork::requests::GetFile::new(
                                        &data,
                                    ),
                                ).await,
                            ).file_path
                        );
                },
                MessageKind::Voice { ref data, .. } => {
                    file_id = data.file_id.clone();

                    file_path =
                        some_or_continue!(
                            ok_or_continue!(
                                api.send(
                                    pw_telegram_bot_fork::requests::GetFile::new(
                                        &data,
                                    ),
                                ).await,
                            ).file_path
                        );
                },
                _ => {
                    continue;
                }
            }

            println!(
                "Getting file (file id: {}, file path: {})..",
                file_id,
                file_path,
            );

            let audio_file =
                ok_or_continue!(
                    get_file(
                        &get_assembly_ai_token(),
                        &file_path,
                    ).await
                );

            println!(
                "Uploading file (file id: {})..",
                file_id,
            );

            let assembly_ai_file_upload =
                ok_or_continue!(
                    upload_audio_file(
                        audio_file,
                    ).await,
                );

            println!(
                "Successfully uploaded file (file id: {})..",
                file_id,
            );

            println!(
                "Requesting transcript (file id: {}, upload url: {})..",
                file_id,
                &assembly_ai_file_upload.upload_url,
            );

            let created_transcript =
                ok_or_continue!(
                    request_transcript(
                        &assembly_ai_file_upload.upload_url,
                    ).await,
                );

            println!(
                "Created transscript (id: {}, file id: {})",
                &created_transcript.id,
                file_id,
            );

            let final_transcript =
                {
                    let mut transcript: AssemblyAiTranscript;

                    let now = std::time::Instant::now();

                    let mut i = 0;

                    loop {
                        if i > 120 {
                            println!(
                                "Timeout waiting for transcript (id: {})! (elapsed: {:?})",
                                &created_transcript.id,
                                now.elapsed(),
                            );

                            continue 'main_loop;
                        }

                        println!(
                            "Polling transscript (id: {})... (elapsed: {:?})",
                            &created_transcript.id,
                            now.elapsed(),
                        );

                        transcript =
                            match get_transcript_by_id(
                                &created_transcript.id,
                            ).await {
                                Err(err) => {
                                    println!(
                                        "Error trying to get transcript (id: {})! (elapsed: {:?}) (err: {:?})",
                                        &created_transcript.id,
                                        now.elapsed(),
                                        err,
                                    );

                                    continue 'main_loop;
                                }
                                Ok(transcript) => transcript,
                            };

                        println!(
                            "status (id: {}): {:?}",
                            &transcript.id,
                            &transcript.status,
                        );

                        if &transcript.status == &AssemblyAiTranscriptStatus::Completed {
                            break;
                        }

                        if &transcript.status == &AssemblyAiTranscriptStatus::Error {
                            println!(
                                "Error during transcription (id: {})! (elapsed: {:?}) (err: {:?})",
                                &created_transcript.id,
                                now.elapsed(),
                                &transcript.error,
                            );

                            continue 'main_loop;
                        }

                        tokio::time::sleep(
                            Duration::from_secs(1),
                        ).await;
                    };

                    transcript
                };

            if let Some(text) = final_transcript.text {
                println!(
                    "Requesting summary for final transcript.. (id: {}, text: {})",
                    &final_transcript.id,
                    &text.trim(),
                );

                match get_gpt3_summary(&text).await {
                    Ok(summary) => {
                        println!(
                            "Got summary for final transcript! (id: {}, summary: {})",
                            &final_transcript.id,
                            &summary,
                        );

                        if let Some(choice) = summary.choices.get(0) {
                            println!(
                                "Sending summarized transcript... (id: {})",
                                &final_transcript.id,
                            );

                            api.send(
                                message.text_reply(
                                    choice.text.trim(),
                                ),
                            ).await?;

                            continue 'main_loop;
                        }
                    },
                    Err(err) => {
                        println!(
                            "Failed to get summary for final transcript! (id: {}, err: {:?})",
                            &final_transcript.id,
                            &err,
                        );
                    }
                }

                println!(
                    "Sending non-summarized transcript.. (id: {})",
                    &final_transcript.id,
                );

                api.send(message.text_reply(text)).await?;
            }
        }
    }
    Ok(())
}
