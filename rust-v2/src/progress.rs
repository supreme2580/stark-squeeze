use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct FileProgress {
    progress_bar: ProgressBar,
}

impl FileProgress {
    pub fn new(total_size: u64) -> Self {
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
            .unwrap()
            .progress_chars("█▇▆▅▄▃▂▁  "));
        
        pb.enable_steady_tick(Duration::from_millis(100));
        
        FileProgress {
            progress_bar: pb
        }
    }

    pub fn update(&self, progress: u64, message: &str) {
        self.progress_bar.set_position(progress);
        self.progress_bar.set_message(message.to_string());
    }

    pub fn finish(&self, message: &str) {
        self.progress_bar.finish_with_message(message.to_string());
    }
}

pub fn join_by_5(data: &[u8]) -> Result<Vec<u8>, String> {
    let total_size = data.len() as u64;
    let progress = FileProgress::new(total_size);
    
    println!("🚀 Starting process with total size: {} bytes", total_size);
    
    let mut result = Vec::new();
    for (i, chunk) in data.chunks(5).enumerate() {
        // Convertir el chunk a un número de 5 bits
        let mut value = 0u8;
        for (j, &byte) in chunk.iter().enumerate() {
            value |= (byte & 1) << (4 - j);
        }
        result.push(value);

        // Actualizar progreso
        progress.update((i * 5 + chunk.len()) as u64, "Processing chunks...");
    }
    
    progress.finish("✨ Process completed!");
    Ok(result)
} 