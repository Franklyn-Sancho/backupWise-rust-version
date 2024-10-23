use std::fs;
use std::path::Path;

pub fn backup_file(src: &str, dest: &str) {
    let source = Path::new(src);
    let destination = Path::new(dest).join(source.file_name().unwrap());

    // Check if the file exists and copy
    if source.is_file() {
        fs::copy(&source, &destination).expect("Error copying file");
        println!("Backup completed: {:?}", destination);
    }
}
