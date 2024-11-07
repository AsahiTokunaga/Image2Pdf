use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use anyhow::{Ok, Result};
use image::{DynamicImage, ImageFormat};
use jpeg_to_pdf::JpegToPdf;
use walkdir::{DirEntry, WalkDir};

const ALLOW_EXTENSIONS: [&str; 6] = ["jpg", "jpeg", "JPG", "JPEG", "png", "PNG"];

fn main() -> Result<()> {
    jpg2pdf("input")?;
    Ok(())
}

fn jpg2pdf<P: AsRef<Path> + ToString + Copy>(root: P) -> Result<()> {
    let mut progress: u32 = 0;
    let mut images: Vec<Vec<u8>> = Vec::new();
    for entry in WalkDir::new(root).sort_by_file_name().contents_first(true) {
        let entry: DirEntry = entry?;
        if entry
            .path()
            .parent()
            .unwrap()
            .with_extension("pdf")
            .as_path()
            .is_file()
        {
            continue;
        }
        if entry.path().is_dir() {
            if images.is_empty() {
                continue;
            }
            let pdf: File = File::create(format!("{}.pdf", entry.path().to_str().unwrap()))?;
            match JpegToPdf::new()
                .add_images(images.clone())
                .create_pdf(&mut BufWriter::new(pdf))
            {
                std::result::Result::Ok(_) => {
                    println!(
                        "[Done] Created PDF: {}",
                        format!("{}.pdf", entry.path().to_str().unwrap())
                    );
                    images.clear();
                    progress = 0;
                    continue;
                }
                Err(e) => {
                    println!(
                        "[FAILED] Couldn't Create PDF: {}",
                        format!("{}.pdf", entry.path().to_str().unwrap())
                    );
                    println!("{}", e);
                    images.clear();
                    progress = 0;
                    continue;
                }
            }
        }
        let extension: &str = entry.path().extension().unwrap().to_str().unwrap();
        if ALLOW_EXTENSIONS.iter().any(|&ex| ex == extension) {
            let all_files: u32 = get_file_count(entry.path().parent().unwrap().to_str().unwrap())?;
            if ["png", "PNG"]
                .iter()
                .any(|&ex| ex == entry.path().extension().unwrap())
            {
                png_to_jpg(entry.path())?;
                images.push(fs::read(entry.path().with_extension("jpg"))?);
                progress += 1;
                println!(
                    "[{}/{}] Added Image: {}",
                    progress,
                    all_files,
                    entry.path().to_str().unwrap()
                );
                continue;
            }
            images.push(fs::read(entry.path())?);
            progress += 1;
            println!(
                "[{}/{}] Added Image: {}",
                progress,
                all_files,
                entry.path().display()
            );
        }
    }
    Ok(())
}

fn get_file_count<P: AsRef<Path>>(path: P) -> Result<u32> {
    let mut file_count: u32 = 0;
    for entry in WalkDir::new(path) {
        let entry: DirEntry = entry?;
        if entry.file_type().is_file() {
            file_count += 1;
        }
    }
    Ok(file_count)
}

fn png_to_jpg<P: AsRef<Path> + Copy>(path: P) -> Result<()> {
    let image: DynamicImage = image::open(path)?;
    image.save_with_format(path.as_ref().with_extension("jpg"), ImageFormat::Jpeg)?;
    fs::remove_file(path)?;
    Ok(())
}
