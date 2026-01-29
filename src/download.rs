// src/download.rs
use tokio::process::Command;
use std::process::Stdio;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::sync::mpsc::UnboundedSender;

pub async fn install_engine(tx: UnboundedSender<String>) {
    let mut child = Command::new("sh")
        .arg("./scripts/engine.sh")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // This was leaking because it wasn't being read!
        .spawn()
        .expect("Failed to execute engine.sh");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap(); // Take the stderr handle!

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    loop {
        tokio::select! {
            // Check stdout
            res = stdout_reader.next_line() => {
                match res {
                    Ok(Some(line)) => { let _ = tx.send(line); }
                    Ok(None) => break, // Exit loop when stdout closes
                    Err(_) => break,
                }
            }
            // Check stderr and send it to the SAME pipe
            res = stderr_reader.next_line() => {
                if let Ok(Some(line)) = res {
                    let _ = tx.send(format!("[LOG] {}", line));
                }
            }
        }
    }
    
    let _ = child.wait().await;
    let _ = tx.send("Engine Installation Complete!".to_string());
}