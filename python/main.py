from contextlib import asynccontextmanager
from fastapi import FastAPI, UploadFile
from full_python_bot.whisper import transcribe
from faster_whisper import WhisperModel
import gc
import time


model = {}


@asynccontextmanager
async def lifespan(app: FastAPI):
    # Load the ML model
    print("Loading Whisper...")
    start = time.time()
    model["whisper"] = WhisperModel(
        "large-v2",
        device='cuda',
        compute_type="float16",
    )
    print("Model loaded! Time to load: " + str(time.time() - start))
    yield
    # Clean up the ML models and release the resources
    print("Cleaning up...")
    model.clear()
    gc.collect()
    print("Done! Shutting down...")


app = FastAPI(lifespan=lifespan)


@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.post("/")
async def create_upload_file(file: UploadFile):
    transcription = transcribe(file.file, model["whisper"])
    return transcription
