import whisper
from pydub import AudioSegment
import os
from PyQt5.QtWidgets import QApplication, QFileDialog

# Função para converter segundos em formato de minutos:segundos
def format_timestamp(seconds):
    minutes, seconds = divmod(seconds, 60)
    return f"{int(minutes):02}:{seconds:05.2f}"

# Função para abrir diálogo de seleção de arquivo usando PyQt5
def selecionar_arquivo_mp3():
    app = QApplication([])
    arquivo, _ = QFileDialog.getOpenFileName(None, "Selecione o arquivo MP3", "", "MP3 Files (*.mp3)")
    return arquivo

# Carregar o modelo Whisper medium (medium)
model = whisper.load_model("medium")

# Escolher o arquivo MP3
mp3_file = selecionar_arquivo_mp3()
if not mp3_file:
    print("Nenhum arquivo selecionado.")
    exit()  # Sair do script se nenhum arquivo for selecionado

# Converter MP3 para WAV usando pydub
audio = AudioSegment.from_mp3(mp3_file)
wav_file = "audio.wav"
audio.export(wav_file, format="wav")

# Transcrever o áudio com Whisper
print("Transcrevendo o áudio, isso pode levar alguns minutos...")
result = model.transcribe(wav_file)

# Criar o arquivo .lrc
lrc_filename = os.path.splitext(mp3_file)[0] + ".lrc"
with open(lrc_filename, "w") as lrc_file:
    for segment in result['segments']:
        start_time = segment['start']
        text = segment['text'].strip()  # Remover espaços desnecessários
        timestamp = format_timestamp(start_time)
        lrc_file.write(f"[{timestamp}] {text}\n")

print(f"Transcrição e sincronização concluídas! Arquivo .lrc gerado: {lrc_filename}")
