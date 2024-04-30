from dotenv import load_dotenv
from telegram import Update
from telegram.ext import (
    filters,
    MessageHandler,
    ContextTypes,
    ApplicationBuilder,
)
import logging
import os
import ast


logging.basicConfig(
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    level=logging.INFO
)


async def request_transcribe(update: Update, context: ContextTypes.DEFAULT_TYPE):
    # TODO: take the audio and give to Whisper to transcribe
    # Another TODO: reply with "Unable to transcribe" when Whisper returns empty
    context.bot.send_message(
        chat_id=update.effective_chat.id,
        text=update.message.text
    )


if __name__ == '__main__':
    load_dotenv()

    transcript_handler = MessageHandler(
        filters.VOICE & filters.Chat(ast.literal_eval(os.environ["CHAT_IDS"])),
        request_transcribe
    )
    application = ApplicationBuilder().token(os.environ["BOT_TOKEN"]).build()
    application.add_handler(transcript_handler)

    application.run_polling()
