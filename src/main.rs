use anyhow::{Ok as anyhowOk, Result};
use futures;
use futures::future::join_all;
use image::{ImageFormat, ImageReader, RgbImage};
use jpeg_to_pdf::JpegToPdf;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::{env, fs};
use tokio::task::JoinHandle;
use walkdir::{DirEntry, WalkDir};

const ALLOW_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "avif"];

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let mut tasks = Vec::new();
    for arg in env::args().skip(1) {
        let to_pdf_task = tokio::spawn(to_pdf(arg));
        tasks.push(to_pdf_task);
    }
    join_all(tasks).await;
    anyhowOk(())
}

async fn to_pdf<P: AsRef<Path>>(path: P) -> Result<()> {
    let dirs: Vec<DirEntry> = WalkDir::new(&path)
        .contents_first(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .collect();
    let png_images: Vec<PathBuf> = WalkDir::new(path)
        .contents_first(true)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry|
            ["png", "avif"].contains(
                &entry
                    .path()
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .as_str()
            )
        )
        .map(|entry| entry.path().to_path_buf())
        .collect();
    let mut to_jpg_tasks: Vec<JoinHandle<()>> = Vec::new();
    for image_path in png_images {
        let to_jpg_task: JoinHandle<()> = tokio::spawn(async move {
            if image_path.extension().unwrap() == OsStr::new("png") {
                let png_image: RgbImage = match image::open(&image_path) {
                    Ok(image) => image.to_rgb8(),
                    Err(e) => {
                        println!("[FAILED] Couldn't Open Image: {}", e);
                        return;
                    }
                };
                match png_image
                    .save_with_format(image_path.with_extension("jpg"), ImageFormat::Jpeg)
                {
                    Ok(_) => println!("[DONE] Converted Png to Jpeg: {}", image_path.display()),
                    Err(e) => println!("[FAILED] Couldn't Save Png Image With Jpeg: {}", e),
                }
                match fs::remove_file(&image_path) {
                    Ok(_) => (),
                    Err(e) => println!("[FAILED] Couldn't Remove Png Image File {} : {}", image_path.display(), e),
                }
            } else {
                let open_image = match ImageReader::open(&image_path) {
                    Ok(item) => item.decode(),
                    Err(e) => {
                        println!("[FAILED] Couldn't Open Image: {}", e);
                        return;
                    }
                };
                let avif_image: RgbImage = match open_image {
                    Ok(item) => item.to_rgb8(),
                    Err(e) => {
                        println!("[FAILED] Couldn't Open Avif File: {}", e);
                        return;
                    }
                };
                match avif_image
                    .save_with_format(image_path.with_extension("jpg"), ImageFormat::Jpeg)
                {
                    Ok(_) => println!("[DONE] Converted Avif to Jpeg: {}", image_path.display()),
                    Err(e) => println!("[FAILED] Couldn't Save Avif Image with Jpeg: {}", e),
                }
                match fs::remove_file(image_path) {
                    Ok(_) => (),
                    Err(e) => println!("[FAILED] Couldn't Remove Avif Image File: {}", e),
                }
            }
        });
        to_jpg_tasks.push(to_jpg_task);
    }
    join_all(to_jpg_tasks).await;
    let mut create_pdf_tasks: Vec<JoinHandle<()>> = Vec::new();
    for dir in dirs {
        let create_pdf_task: JoinHandle<()> = tokio::spawn(async move {
            let images: Vec<Vec<u8>> = WalkDir::new(dir.path())
                .contents_first(true)
                .sort_by_file_name()
                .max_depth(1)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_file())
                .filter(|entry| 
                    ALLOW_EXTENSIONS.contains(
                        &entry
                            .path()
                            .extension()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_lowercase()
                            .as_str()
                    )
                )
                .filter_map(|entry| fs::read(entry.path()).ok())
                .collect();
            if images.is_empty() { return; }
            let pdf = File::create(format!("{}.pdf", &dir.path().display())).ok().unwrap();
            let mut writer = BufWriter::new(pdf);
            let result_create_pdf = JpegToPdf::new()
                .add_images(images.clone())
                .create_pdf(&mut writer);
            match result_create_pdf {
                Ok(_) => println!("[DONE] Created PDF Successfully: {}", format!("{}.pdf", dir.path().display())),
                Err(e) => println!("[FAILED] Couldn't Create PDF: {}", e)
            }
        });
        create_pdf_tasks.push(create_pdf_task);
    }
    join_all(create_pdf_tasks).await;
    anyhowOk(())
}
