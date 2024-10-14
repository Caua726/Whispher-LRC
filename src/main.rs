use std::path::{Path, PathBuf};
use rfd::FileDialog;
use std::process::Command;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

static MODELO_WHISPER: &str = "tiny";
static FORMATO_LEGENDA: &str = "srt";
static PALAVRAS_LINHA: &str = "5";
static TEMP_DIR: &str = "/home/caua/.cache/lrc-musica/tmp.wav";
static TMP_SRT: &str = "/home/caua/.cache/lrc-musica/tmp.srt";
static DIRETORIO_PADRAO: &str = "/home/caua/Sync";

fn openfiledialog_box() -> Option<PathBuf> {
    let file = FileDialog::new()
    .add_filter("musica", &["mp3"])
    .set_directory(DIRETORIO_PADRAO)
    .pick_file();
    file
}

fn converter_wav() {
    if Path::new(TEMP_DIR).exists() {
        let _ = Command::new("rm")
        .arg("-f")
        .arg(TEMP_DIR)
        .status();
    }
    if let Some(arquivo) = openfiledialog_box() {
        let _ = Command::new("ffmpeg")
        .arg("-i")
        .arg(arquivo)
        .arg(TEMP_DIR)
        .status();
    }
}

fn whisper() {
    let _ = Command::new("whisper")
    .arg(TEMP_DIR)
    .arg("--model")
    .arg(MODELO_WHISPER)
    .arg("--output_format")
    .arg(FORMATO_LEGENDA)
    .arg("--word_timestamps")
    .arg("True")
    .arg("--max_words_per_line")
    .arg(PALAVRAS_LINHA)
    .status();
}

fn timestamp(segundos: f64) -> String {
    let minutos = (segundos / 60.0).floor();
    let segundos = segundos % 60.0;
    format!("{:02}:{:05.2}", minutos, segundos)
}

fn lrc() -> std::io::Result<()> {
    if let Some(arquivo) = openfiledialog_box() {
        let lrc_nome = arquivo.with_extension("lrc");
        let srt = File::open(TMP_SRT)?;
        let ler_srt = BufReader::new(srt);
        let mut lrc = File::create(&lrc_nome)?;

        for line in ler_srt.lines() {
            let line = line?;

            if line.contains("-->") {
                let timestamp_str: Vec<&str> = line.split(" --> ").collect();
                if timestamp_str.len() == 2 {
                    let primeiro_tempo = timestamp(timestamp_str[0].parse::<f64>().unwrap());
                    write!(lrc, "[{}] ", primeiro_tempo)?;
                }
            } else if !line.trim().is_empty() {
                writeln!(lrc, "{}", line.trim())?;
            }
        }
    }
    Ok(())
}


fn mover_renomear() {
    if let Some(arquivo) = openfiledialog_box() {
        if let Some(nome_arquivo) = arquivo.file_stem() {
            let nome = format!("{}.{}", nome_arquivo.to_str().unwrap(), FORMATO_LEGENDA);
            let destino = arquivo.with_file_name(&nome);

            let renomear_result = fs::rename(format!("{}.{}", TEMP_DIR, FORMATO_LEGENDA), &destino);
            match renomear_result {
                Ok(_) => println!("Arquivo renomeado e movido para {:?}", destino),
                Err(e) => println!("Erro ao renomear e mover o arquivo: {:?}", e),
            }

            // Converter para .lrc após renomear
            let _ = lrc();
        }
    }
}

fn main() {
    converter_wav();
    whisper();
    mover_renomear();
    println!("Hello, world!");
}
