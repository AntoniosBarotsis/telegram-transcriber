# Telegram Transcriber Bot

A simple Telegram bot that replies to voice messages with their transcription.

The project is split into a [Whisper](https://github.com/openai/whisper) server (`./python`) and
the telegram bot (`./rust`).

<!-- ## The Whisper Server -->

## The Telegram Bot

### Dependencies

- [ffmpeg](https://ffmpeg.org/download.html)
- [openssl](https://docs.rs/openssl/latest/openssl/#automatic)

### Configuration

Create a `./rust/.env` file and fill in the following parameters:

- `TELOXIDE_TOKEN`: [The Telegram bot token](https://core.telegram.org/bots#how-do-i-create-a-bot)
- `CHAT_IDS`: A comma delimited list of whitelisted chat ids. 
  [Here's a thread on how to get those](https://stackoverflow.com/a/69302407/12756474)
- `WHISPER_URL`: The URL of the Whisper server
- `RUST_LOG` (optional): Use this to configure logging. I have it set to `RUST_LOG=info,telegram_transcriber=debug`
  but `RUST_LOG=info` is probably enough for you.

### Running

This is just another Rust project so you can just `cargo run`. I wanted to run this on my Raspberry
Pi and just to save you a few headaches, if you only connect to your Pi via SSH, try running it with
`nohup cargo r -r &`, this will run it in the background even after you disconnect from the SSH
session and all the logs will be dumped in `./rust/nohup.out`.

### Cross Compiling

We are running the bot on a Raspberry Pi which takes sometime to compile (especially on release).
OpenSSL always causes some issues with cross compilation but thankfully,
[`cargo cross`](https://github.com/cross-rs/cross) helps with that. We opted to solve the issue by
optionally statically compiling OpenSSL into the binary which you can do with the following:

```sh
$ cross build --target aarch64-unknown-linux-gnu -r -F openssl-vendored
```

This is admitedly not very fast but it is fast*er*. If you really want you can check out
[this](https://github.com/cross-rs/cross/wiki/Recipes#pre-build) to link it dynamically instead,
just know you might need to fiddle with your apt sources or build some libs from source.
