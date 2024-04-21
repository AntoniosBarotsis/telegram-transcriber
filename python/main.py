from fastapi import FastAPI, UploadFile
from whisper import transcribe


app = FastAPI()


@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.post("/")
async def create_upload_file(file: UploadFile):
    transcription = transcribe(file.file)
    return transcription
