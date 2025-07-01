use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use walkdir::WalkDir;
use regex::Regex;
use memmap2::Mmap;
use rayon::prelude::*;

const VALID_EXTENSIONS: &[&str] = &[
    "txt", "log", "md", "csv", "json", "xml", "html", "htm", "eml", "rtf", "odt", "ods", "docx", "xlsx", "java", "kt", "epub"
];

pub fn search_folder(folder_path: &Path, pattern: &str, output: Option<&Path>) -> anyhow::Result<usize> {
    let regex = Regex::new(pattern)?;
    let files: Vec<PathBuf> = WalkDir::new(folder_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file() &&
            e.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext_str| VALID_EXTENSIONS.contains(&ext_str.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    println!("🔍 Number of files to be checked: {}", files.len());

    let total_matches = Mutex::new(0usize);
    let writer = output.map(|path| Mutex::new(BufWriter::new(File::create(path).unwrap())));

    files.par_iter().for_each(|file_path| {
        if let Ok(file) = File::open(file_path) {
            if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                let content = String::from_utf8_lossy(&mmap);
                let mut matches = vec![];

                for line in content.lines() {
                    if regex.is_match(line) {
                        matches.push(line.to_string());
                    }
                }

                if !matches.is_empty() {
                    *total_matches.lock().unwrap() += matches.len();

                    if let Some(ref writer_mutex) = writer {
                        let mut w = writer_mutex.lock().unwrap();
                        writeln!(w, "{} =", file_path.display()).ok();
                        for line in matches {
                            writeln!(w, "{}", line).ok();
                        }
                        writeln!(w).ok();
                    }
                }
            }
        }
    });

    Ok(*total_matches.lock().unwrap())
}

