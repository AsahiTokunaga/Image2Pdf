use anyhow::{Ok as anyhowOk, Result};
use futures::future::join_all;
use image::{ImageFormat, ImageReader, RgbImage};
use jpeg_to_pdf::JpegToPdf;
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
    let except_jpeg_images: Vec<PathBuf> = WalkDir::new(path)
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
    for image_path in except_jpeg_images {
        let to_jpg_task: JoinHandle<()> = tokio::spawn(async move {
            let opened_image= match ImageReader::open(&image_path) {
                Ok(item) => item.decode(),
                Err(e) => {
                    println!("[FAILED] Couldn't Open Image: {} : {}", image_path.display(), e);
                    return;
                }
            };
            let image: RgbImage = match opened_image {
                Ok(item) => item.to_rgb8(),
                Err(e) => {
                    println!("[FAILED] Couldn't Convert to RGB8: {}", e);
                    return;
                }
            };
            match image.save_with_format(image_path.with_extension("jpg"), ImageFormat::Jpeg) {
                Ok(_) => println!("[DONE] Saved Jpeg Image Successfully: {}", image_path.with_extension("jpg").display()),
                Err(e) => println!("[FAILED] Couldn't Save Jpeg Image: {}", e)
            }
            match fs::remove_file(image_path) {
                Ok(_) => (),
                Err(e) => println!("[FAILED] Couldn't Remove Image: {}", e)
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
                .map(|entry| 
                    match fs::read(entry.path()) {
                        Ok(item) => item,
                        Err(e) => {
                            println!("[FAILED] Couldn't Add Jpeg Image: {}", e);
                            return Vec::new();
                        }
                    }
                )
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
