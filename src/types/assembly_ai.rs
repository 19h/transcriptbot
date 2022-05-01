use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssemblyAiUpload {
    pub upload_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssemblyAiTranscriptStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssemblyAiTranscript {
    pub id: String,
    pub language_model: String,
    pub acoustic_model: String,
    pub language_code: String,
    pub status: AssemblyAiTranscriptStatus,
    pub audio_url: String,
    pub text: Option<String>,
    pub error: Option<String>,
    #[serde(default)]
    pub words: Option<Vec<Word>>,
    pub confidence: Option<f64>,
    pub audio_duration: Option<i64>,
    pub auto_highlights_result: Option<AutoHighlightsResult>,
    pub content_safety: Option<bool>,
    pub iab_categories: Option<bool>,
    pub punctuate: bool,
    pub format_text: bool,
    pub speed_boost: bool,
    pub auto_highlights: bool,
    pub filter_profanity: bool,
    pub redact_pii: bool,
    pub redact_pii_audio: bool,
    pub speaker_labels: bool,
    pub language_detection: bool,
    pub disfluencies: bool,
    pub sentiment_analysis: bool,
    pub auto_chapters: bool,
    pub entity_detection: bool,
    pub utterances: Value,
    pub dual_channel: Value,
    pub webhook_url: Value,
    pub webhook_status_code: Value,
    pub audio_start_from: Value,
    pub audio_end_at: Value,
    pub word_boost: Vec<Value>,
    pub boost_param: Value,
    pub redact_pii_audio_quality: Value,
    pub redact_pii_policies: Value,
    pub redact_pii_sub: Value,
    pub content_safety_labels: Value,
    pub iab_categories_result: Value,
    pub custom_spelling: Value,
    pub chapters: Value,
    pub sentiment_analysis_results: Value,
    pub entities: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Word {
    pub text: String,
    pub start: i64,
    pub end: i64,
    pub confidence: f64,
    pub speaker: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoHighlightsResult {
    pub status: String,
    pub results: Vec<Result>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub count: i64,
    pub rank: f64,
    pub text: String,
    pub timestamps: Vec<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    pub start: i64,
    pub end: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result2 {
    pub text: String,
    pub labels: Vec<Label>,
    pub timestamp: Timestamp2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub relevance: f64,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp2 {
    pub start: i64,
    pub end: i64,
}
