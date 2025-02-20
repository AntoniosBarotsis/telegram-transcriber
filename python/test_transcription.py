from full_python_bot.whisper import transcribe
from faster_whisper import WhisperModel


model = WhisperModel(
        model_size_or_path="large-v2",
        device='cuda',
        compute_type="float16",
    )
print(transcribe('test/sample1.flac', model))
