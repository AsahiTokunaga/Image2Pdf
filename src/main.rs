use anyhow::{Ok as anyhowOk, Result};
use image::{ImageFormat, RgbImage};
use jpeg_to_pdf::JpegToPdf;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::{env, fs};
use walkdir::{DirEntry, WalkDir};

const ALLOW_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "avif"];

fn main() -> Result<()> {
    for args in env::args().skip(1) {
        to_pdf(&args)?;
    }
    anyhowOk(())
}

fn to_pdf<P: AsRef<Path>>(path: &P) -> Result<()> {
    let paths: Vec<DirEntry> = WalkDir::new(path)
        .sort_by_file_name()
        .contents_first(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .collect();
    let mut progress: u32 = 1;
    let file_count: u32 = get_file_count(path)?;
    let mut images: Vec<Vec<u8>> = Vec::new();
    for entry in paths {
        if entry.path().is_dir() && images.is_empty() {
            continue;
        }
        if entry.path().is_dir() {
            let pdf: File = File::create(entry.path().with_extension("pdf"))?;
            match JpegToPdf::new()
                .add_images(images.clone())
                .create_pdf(&mut BufWriter::new(pdf)) {
                Ok(_) => println!("[Done] Created PDF: {}", entry.path().with_extension("pdf").display()),
                Err(e) => {
                    println!("[FAILED] Couldn't Create PDF: {}", entry.path().with_extension("pdf").display());
                    println!("{}", e);
                }
            }
            images.clear();
            continue;
        }
        let extention: String = entry.path().extension().unwrap().to_str().unwrap().to_lowercase();
        if ALLOW_EXTENSIONS.iter().any(|&allow_ex| allow_ex == extention) {
            let image_path: PathBuf = if ["png", "avif"].contains(&extention.as_str()) {
                to_jpg(entry.path())?;
                entry.path().with_extension("jpg").to_path_buf()
            } else {
                entry.path().to_path_buf()
            };
            let image: Vec<u8> = fs::read(&image_path)?;
            images.push(image);
            println!("[{}/{}] Added Image: {}", progress, file_count, image_path.display());
            progress += 1;
        }
    }
    anyhowOk(())
}

fn to_jpg<P: AsRef<Path>>(path: P) -> Result<()> {
    let image: RgbImage = image::open(&path)?.to_rgb8();
    image.save_with_format(path.as_ref().with_extension("jpg"), ImageFormat::Jpeg)?;
    fs::remove_file(path)?;
    anyhowOk(())
}

fn get_file_count<P: AsRef<Path>>(path: P) -> Result<u32> {
    let mut count: u32 = 0;
    for _ in WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| {
            let extention: String = entry.path().extension().unwrap().to_str().unwrap().to_lowercase();
            ALLOW_EXTENSIONS.contains(&extention.as_str())
        })
    {
        count += 1;
    }
    anyhowOk(count)
}
