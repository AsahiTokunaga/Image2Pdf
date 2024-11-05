use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use anyhow::{Ok, Result};
use jpeg_to_pdf::JpegToPdf;
use walkdir::{WalkDir, DirEntry};

fn main() -> Result<()> {
    jpg2pdf("input")?;
    Ok(())
}

fn jpg2pdf<P: AsRef<Path> + ToString + Copy>(root: P) -> Result<()> {
    let mut progress: u32 = 0;
    let mut images: Vec<Vec<u8>> = Vec::new();
    for entry in WalkDir::new(root).sort_by_file_name().contents_first(true) {
        let entry: DirEntry = entry?;
        if entry.path().is_dir() {
            if images.is_empty() {
                continue;
            }
            let pdf: File = File::create(format!("{}.pdf", entry.path().to_str().unwrap()))?;
            JpegToPdf::new()
                .add_images(images.clone())
                .create_pdf(&mut BufWriter::new(pdf))?;
            println!("[Done] Created PDF: {}", format!("{}.pdf", entry.path().to_str().unwrap()));
            images.clear();
            progress = 0;
            continue;
        }
        if entry.path().extension().unwrap() == "jpg" {
            let all_files: u32 = get_file_count(entry.path().parent().unwrap().to_str().unwrap())?;
            images.push(fs::read(entry.path())?);
            progress += 1;
            println!("[{}/{}] Added Image: {}", progress, all_files, entry.path().to_str().unwrap());
        }
    }
    Ok(())
}

fn get_file_count<D: AsRef<Path>>(dir: D) -> Result<u32> {
    let mut file_count: u32 = 0;
    for entry in WalkDir::new(dir) {
        let entry: DirEntry = entry?;
        if entry.file_type().is_file() {
            file_count += 1;
        }
    }
    Ok(file_count)
}
