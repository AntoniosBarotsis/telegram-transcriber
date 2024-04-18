from faster_whisper import WhisperModel
import time


def transcribe(audio, device="cuda"):
    print("Running Whisper...")
    start = time.time()  # in seconds
    model = WhisperModel(
        "large-v2",
        device=device,
        compute_type="float16" if device == "cuda" else "float32",
    )

    # TODO: Add a failsafe to run on CPU if CUDA error
    segments, _ = model.transcribe(audio, vad_filter=True)

    text = ""

    for segment in segments:
        text = text + segment.text.strip() + " "

    runtime = time.time() - start
    print("Transcription done! Time to transcribe (in s): " + str(runtime))

    return text
