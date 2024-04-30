import time


def transcribe(audio, model):
    print("Running Whisper...")
    start = time.time()  # in seconds

    # TODO: Add a failsafe to run on CPU if CUDA error
    segments, _ = model.transcribe(audio, vad_filter=True)

    text = ""

    for segment in segments:
        text = text + segment.text.strip() + " "

    runtime = time.time() - start
    print("Transcription done! Time to transcribe (in s): " + str(runtime))
    print("Result: " + text)

    return text
