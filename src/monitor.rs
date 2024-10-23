use notify::{Watcher, RecursiveMode, Event, EventKind, RecommendedWatcher, Config};
use std::sync::mpsc::channel;
use std::path::Path;
use crate::backup::backup_file;
use crate::notification::send_backup_notification;

pub fn watch_directory(src: &str, dest: &str) {
    // Create a channel to receive event notifications
    let (tx, rx) = channel();

    // Create a recommended watcher to monitor changes in the directory
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Err(e) = tx.send(res) {
                eprintln!("Error sending notification: {:?}", e);
            }
        },
        Config::default(), // Pass default configuration
    ).expect("Error creating watcher");

    // Watch the specified directory recursively
    watcher.watch(Path::new(src), RecursiveMode::Recursive).expect("Error watching directory");

    // Process events in a loop
    loop {
        match rx.recv() {
            Ok(Ok(event)) => handle_event(event, dest),
            Ok(Err(e)) => eprintln!("Error monitoring: {:?}", e),
            Err(e) => eprintln!("Error receiving event: {:?}", e),
        }
    }
}

fn handle_event(event: Event, dest: &str) {
    // Check the event type and handle accordingly
    for path in event.paths {
        let path_str = path.to_str().unwrap_or_default();
        match event.kind {
            EventKind::Create(_) => {
                println!("File created: {:?}", path);
                backup_file(path_str, dest);
                send_backup_notification(path_str);
            },
            EventKind::Modify(_) => {
                println!("File modified: {:?}", path);
                backup_file(path_str, dest);
                send_backup_notification(path_str);
            },
            _ => (),
        }
    }
}






