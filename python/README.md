# The fully Python Telegram Transcriber Bot
To avoid having to set up a static IP/ngrok, I (greenw0lf) decided to implement every aspect of the bot in Python, from the part that interacts with the Telegram Bot API to the code that runs Whisper on the audio files (that already existed from the initial commits).

I have also expanded on the original implementation that only supported voice messages to include all sorts of sources that contain audio, including video messages, as well as audio and video files.

## Dependencies & Configuration

Follow the steps in the main README of the repository, under "The Whisper Server".

The Whisper implementation used is [faster-whisper](https://github.com/SYSTRAN/faster-whisper) and the Telegram Bot API wrapper used is [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot).

## Environment Configuration

Create a file in `full_python_bot` called `.env` in which you need to add:

- `BOT_TOKEN`: [The Telegram bot token](https://core.telegram.org/bots#how-do-i-create-a-bot)
- `CHAT_IDS`: A comma delimited list of whitelisted chat IDs, surrounded by [ ] (e.g.: `[102312,-325756,2902931]`)

    [Here's a thread on how to get the chat IDs](https://stackoverflow.com/a/69302407/12756474)

## Running

Simply run the `bot.py` file (make sure you are in the `full_python_bot` folder):
```
python bot.py
```

## Logging

These are the elements that are currently logged:
- Time it takes to load Whisper
- Time it takes to evaluate the audio
- How much of the audio is removed by the Voice Activity Detection (VAD) filter
- The duration of the audio being processed
- The language detected and its probability
- The resulting text that is sent as a reply by the bot
