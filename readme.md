<h1 align="center">transcript-bot</h1>

<h5 align="center">Sits in Telegram groups, listens for audio and voice files, transcribes them using AssemblyAI and summarises them using OpenAI GPT3.</h5>

<div align="center">
  <a href="https://crates.io/crates/tg-join-leave-bot">
    crates.io
  </a>
  —
  <a href="https://github.com/19h/tg-join-leave-bot">
    Github
  </a>
</div>

<br />

```shell script
$ cargo install transcript-bot
$ export ASSEMBLY_AI_TOKEN=<your-token> 
$ export OPENAI_TOKEN=<your token>
$ export TELEGRAM_API_TOKEN=<your-token>
$ transcript-bot
```

#### Notes

Make sure you trust the group. Otherwise this can get rather expensive.

#### License

~~ MIT License ~~

Copyright (c) 2022 Kenan Sulayman

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
