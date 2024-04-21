from faster_whisper import WhisperModel
import time
import gc


def transcribe(audio, device="cuda"):
    print("Running Whisper...")
    start = time.time()  # in seconds
    model = WhisperModel(
        "large-v2",
        device=device,
        compute_type="float16" if device == "cuda" else "float32",
    )
    print("Model loaded!")

    # TODO: Add a failsafe to run on CPU if CUDA error
    segments, _ = model.transcribe(audio, vad_filter=True)
    print("Get transcribed B-)")

    text = ""

    for segment in segments:
        text = text + segment.text.strip() + " "

    runtime = time.time() - start
    print("Transcription done! Time to transcribe (in s): " + str(runtime))
    print("Result: " + text)

    del model
    gc.collect()

    return text
