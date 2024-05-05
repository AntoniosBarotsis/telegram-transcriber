from dotenv import load_dotenv
from telegram import Update, ReplyParameters
from telegram.ext import (
    filters,
    MessageHandler,
    ContextTypes,
    ApplicationBuilder
)
import logging
import os
import ast
from whisper import transcribe
from faster_whisper import WhisperModel
import time
import gc


logging.basicConfig(
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    level=logging.INFO
)
logger = logging.getLogger(__name__)


logger.info("Loading Whisper...")
start = time.time()
model = WhisperModel(
        "large-v2",
        device='cuda',
        compute_type="float16",
    )
logger.info("Model loaded! Time to load: " + str(time.time() - start))


async def req_transcribe(
        update: Update,
        context: ContextTypes.DEFAULT_TYPE):
    if update.message.voice:
        msg = await context.bot.get_file(update.message.voice)
    elif update.message.video_note:
        msg = await context.bot.get_file(update.message.video_note)
    elif update.message.audio:
        msg = await context.bot.get_file(update.message.audio)
    elif update.message.video:
        msg = await context.bot.get_file(update.message.video)

    path = await msg.download_to_drive()

    transcript = transcribe(str(path), model)
    if transcript == "":
        transcript = "<NO SPEECH DETECTED>"

    await context.bot.send_message(
        chat_id=update.effective_chat.id,
        text=transcript,
        reply_parameters=ReplyParameters(update.message.message_id)
    )

    os.remove(str(path))


if __name__ == '__main__':
    load_dotenv()

    transcript_handler = MessageHandler(
        (filters.VOICE |
         filters.VIDEO_NOTE |
         filters.AUDIO |
         filters.VIDEO) &
        filters.Chat(ast.literal_eval(os.environ["CHAT_IDS"])),
        req_transcribe
    )
    application = ApplicationBuilder().token(os.environ["BOT_TOKEN"]).build()
    application.add_handler(transcript_handler)

    application.run_polling()

    print("Cleaning up...")
    del model
    gc.collect()
    print("Done! Shutting down...")
