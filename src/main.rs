use anyhow::{Ok, Result};
use image::{DynamicImage, ImageFormat};
use jpeg_to_pdf::JpegToPdf;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::{env, fs};
use walkdir::{DirEntry, WalkDir};

const ALLOW_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "avif"];

fn main() -> Result<()> {
    for args in env::args().skip(1) {
        to_pdf(&args)?;
    }
    Ok(())
}

fn to_pdf<P: AsRef<Path>>(path: &P) -> Result<()> {
    let paths: Vec<DirEntry> = WalkDir::new(path)
        .sort_by_file_name()
        .contents_first(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .collect();
    let mut progress: u32 = 0;
    let file_count: u32 = get_file_count(path)?;
    let mut images: Vec<Vec<u8>> = Vec::new();
    for entry in paths {
        if entry.path().is_dir() {
            if images.is_empty() {
                continue;
            }
            let pdf: File = File::create(entry.path().with_extension("pdf"))?;
            JpegToPdf::new()
                .add_images(images.clone())
                .create_pdf(&mut BufWriter::new(pdf))?;
            println!(
                "[Done] Created PDF: {}",
                entry.path().with_extension("pdf").display()
            );
            images.clear();
            continue;
        }
        if ALLOW_EXTENSIONS
            .into_iter()
            .any(|ex| ex.eq_ignore_ascii_case(entry.path().extension().unwrap().to_str().unwrap()))
        {
            if ["png", "avif"].iter().any(|&ex| {
                ex.eq_ignore_ascii_case(entry.path().extension().unwrap().to_str().unwrap())
            }) {
                to_jpg(entry.path())?;
                images.push(fs::read(entry.path().with_extension("jpg"))?);
                println!(
                    "[{}/{} Progress: {}%] Added Image: {}",
                    progress,
                    file_count,
                    ((progress as f64 / file_count as f64) * 100.0) as u32,
                    entry.path().display()
                );
                progress += 1;
                continue;
            }
            images.push(fs::read(entry.path())?);
            println!(
                "[{}/{} Progress: {}%] Added Image: {}",
                progress,
                file_count,
                ((progress as f64 / file_count as f64) * 100.0) as u32,
                entry.path().display()
            );
            progress += 1;
        }
    }
    Ok(())
}

fn to_jpg<P: AsRef<Path>>(path: P) -> Result<()> {
    let image: DynamicImage = image::open(&path)?;
    image.save_with_format(path.as_ref().with_extension("jpg"), ImageFormat::Jpeg)?;
    fs::remove_file(path)?;
    Ok(())
}

fn get_file_count<P: AsRef<Path>>(path: P) -> Result<u32> {
    let mut count: u32 = 0;
    for _ in WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().is_file()
                && ALLOW_EXTENSIONS.iter().any(|&ex| {
                    ex.eq_ignore_ascii_case(entry.path().extension().unwrap().to_str().unwrap())
                })
        })
    {
        count += 1;
    }
    Ok(count)
}
