use std::{env, fs, io};

enum FileSize {
   Bytes(u64),
   Kilobytes(u64),
   Megabytes(u64),
   Gigabytes(u64),
}

impl FileSize {
    fn from_bytes(bytes: u64) -> FileSize {
       match bytes {
           0..=1024 => FileSize::Bytes(bytes),
           1025..=1048576 => FileSize::Kilobytes(bytes / 1024),
           1048577..=1073741824 => FileSize::Megabytes(bytes / 1048576),
           _ => FileSize::Gigabytes(bytes / 1073741824),
       }
    }

    fn to_string(&self) -> String {
        match self {
            FileSize::Bytes(b) => format!("{} bytes", b),
            FileSize::Kilobytes(kb) => format!("{} KB", kb),
            FileSize::Megabytes(mb) => format!("{} MB", mb),
            FileSize::Gigabytes(gb) => format!("{} GB", gb),
        }
    }
}

fn file_size(filename: &str) -> io::Result<FileSize> {
    let metadata = fs::metadata(filename)?;
    let size = metadata.len();
    Ok(FileSize::from_bytes(size))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filename = args.get(0).expect("Please provide a filename");
    println!("Filename {:?}", filename);
    let size = file_size(filename).expect("Couldn't get file size");
    println!("File size: {}", size.to_string());
}