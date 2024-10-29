use std::fs::File as StdFile;
use std::io::{self, Read, Cursor};
use std::path::Path;
use google_drive3::{DriveHub, api::File};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;

pub async fn upload_file(hub: &DriveHub<HttpsConnector<HttpConnector>>, file_path: &str) -> Result<File, Box<dyn std::error::Error>> {
    // Check if the file exists before trying to open it
    if !Path::new(file_path).exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        )));
    }

    // Ignore temporary files or Google backup files
    if file_path.contains("~$") || file_path.contains("gooutstream") {
        return Ok(File::default()); // Return an empty file if ignored
    }

    // Open the local file
    let mut file = StdFile::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?; // Read the entire file into a buffer

    // Create a Cursor to allow reading from the buffer
    let cursor = Cursor::new(buffer);

    // Determine the MIME type of the file
    let mime_type = mime_guess::from_path(file_path).first_or_octet_stream();

    // Prepare the file metadata for Google Drive
    let drive_file = File {
        name: Some(Path::new(file_path).file_name().unwrap().to_string_lossy().to_string()), // Only the file name
        mime_type: Some(mime_type.to_string()),
        ..Default::default()
    };

    // Upload the file using the Cursor and MIME type
    let (_, uploaded_file) = hub.files().create(drive_file)
        .upload(cursor, mime_type) // Pass the cursor and MIME type
        .await?;

    Ok(uploaded_file) // Return the uploaded file
}





