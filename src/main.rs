use chrono::{DateTime, Utc};
use clap::Parser; // Renamed
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Style as TabledStyle,
        object::{Columns, Rows},
    },
}; // Renamed

mod window;
pub mod shell;

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Type"}]
    e_type: EntryType,
    #[tabled{rename="Size B"}]
    len_bytes: u64,
    #[tabled{rename="Modified"}]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "My first Rust CLI program")]
struct CLI {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

fn main() {
    window::initialize_window::run();
}

// fn main() {
//     let cli = CLI::parse();
//     let path = cli.path.unwrap_or(PathBuf::from("."));

//     if let Ok(does_exist) = fs::exists(&path) {
//         if does_exist {
//             if cli.json {
//                 let get_files = get_files(&path);
//                 println!("{}", serde_json::to_string_pretty(&get_files).unwrap_or("cannot parse text".to_string()))
//             } else {
//                 print_table(path);
//                 sys_monitor::sysinfo();
//             }
//         } else {
//             println!("{}", "Path does not exist".red().bold());
//         }
//     } else {
//         println!("{}", "error reading directory".red().bold());
//     }
// }

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data);
            }
        }
    }
    data
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        data.push(FileEntry {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("unknown name".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: meta.len(),
            modified: if let Ok(modi) = meta.modified() {
                let date: DateTime<Utc> = modi.into();
                format!("{}", date.format("%a %b %e %y"))
            } else {
                String::default()
            },
        });
    }
}

fn print_table(path: PathBuf) {
    let get_files = get_files(&path);
    let mut table = Table::new(get_files);
    table.with(TabledStyle::extended());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(1), Color::FG_BRIGHT_WHITE);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_BLUE);
    println!("{}", table);
}
