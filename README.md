# Backupwise - A File Backup Application CLI in RUST

This is the Rust version of Backupwise, a simple application for monitoring a directory and creating backups of files when they are created or modified. This utility utilizes Zenity for directory selection and system notifications for backup completion.

## Features

- Monitor a specified directory for file changes (creation and modification).
- Automatically create backups of files in a designated backup directory.
- User-friendly interface for selecting directories using Zenity.
- System notifications upon successful backup.

## Requirements

- Rust (1.XX or higher)
- Zenity installed on your system

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/backupwise.git
   cd backupwise
   ```

2. Install the dependencies and zenity
    ```bash
    cargo build
    sudo apt install zenity 
    ```

## Usage

1. Run the application
   ```bash
   cargo run
   ```

2. Wait a second and Follow the prompts to select the directory you want to monitor and the directory where backups will be 

3. The application will start monitoring the selected directory. It will create backups automatically when files are created or modified.

## Contribuition

Feel free to submit issues or pull requests to improve this utility.
