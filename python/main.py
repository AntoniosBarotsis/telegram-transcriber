from fastapi import FastAPI, UploadFile
import shutil
from whisper import transcribe
import os


app = FastAPI()


@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.post("/")
async def create_upload_file(file: UploadFile):
    file_location = f"files/{file.filename}"
    with open(file_location, "wb+") as file_object:
        shutil.copyfileobj(file.file, file_object)

    transcription = transcribe(file_location)
    os.remove(file_location)

    return transcription
