use google_drive3::DriveHub;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use notify::{Watcher, RecursiveMode, Event, EventKind, RecommendedWatcher, Config};
use std::sync::mpsc::channel;
use std::path::Path;
use crate::backup::backup_file;
use crate::driver_integration::upload_file;
use crate::notification::send_backup_notification;
use crate::logger::{log_event, log_file_change};

pub async fn watch_directory(src: &str, dest: &str, hub: &DriveHub<HttpsConnector<HttpConnector>>) {
    // Create a channel to receive event notifications
    let (tx, rx) = channel();

    // Create a recommended watcher to monitor changes in the directory
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Err(e) = tx.send(res) {
                // Handle errors while sending notifications
                eprintln!("Error sending notification: {:?}", e);
            }
        },
        Config::default(), // Use default configuration
    ).expect("Error creating watcher");

    // Watch the specified directory recursively
    watcher.watch(Path::new(src), RecursiveMode::Recursive).expect("Error watching directory");

    // Process events in a loop
    loop {
        match rx.recv() {
            Ok(Ok(event)) => handle_event(event, dest, hub).await, // Call handle_event with the hub
            Ok(Err(e)) => eprintln!("Error monitoring: {:?}", e),
            Err(e) => eprintln!("Error receiving event: {:?}", e),
        }
    }
}

async fn handle_event(event: Event, dest: &str, hub: &DriveHub<HttpsConnector<HttpConnector>>) {
    // Check the event type and handle accordingly
    for path in event.paths {
        let path_str = path.to_str().unwrap_or_default();

        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                // Backup the local file
                backup_file(path_str, dest);

                // Check if the file still exists
                if !Path::new(path_str).exists() {
                    return; // Exit if the file was deleted
                }

                // Upload the file to Google Drive
                if let Err(e) = upload_file(hub, path_str).await {
                    eprintln!("Error uploading file: {:?}", e);
                }

                // Log the event
                log_event("File Modified or Created", &format!("File: {}", path_str));
                send_backup_notification(path_str);
            },
            _ => (), // Ignore other event types
        }
    }
}








