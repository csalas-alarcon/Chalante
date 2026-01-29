// src/download.rs
use tokio::process::Command;
use std::process::Stdio;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::sync::mpsc::UnboundedSender;

/// Installs the llama.cpp engine and captures both stdout and stderr to prevent terminal leakage.
pub async fn install_engine(tx: UnboundedSender<String>) {
    let mut child = Command::new("sh")
        .arg("./scripts/engine.sh")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // Capture stderr to prevent "chaotic" terminal logs
        .spawn()
        .expect("Failed to execute engine.sh");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    // The select! macro listens to both streams at once
    loop {
        tokio::select! {
            res = stdout_reader.next_line() => {
                match res {
                    Ok(Some(line)) => { let _ = tx.send(line); }
                    Ok(None) => break, // stdout closed, engine.sh is finishing
                    Err(_) => break,
                }
            }
            res = stderr_reader.next_line() => {
                if let Ok(Some(line)) = res {
                    // Prepend [LOG] or filter if you want to distinguish errors/progress
                    let _ = tx.send(format!("[LOG] {}", line));
                }
            }
        }
    }
    
    let _ = child.wait().await;
    let _ = tx.send("Engine Installation Complete!".to_string());
}

/// Installs the models and ensures no stderr output leaks into the TUI.
pub async fn install_models(tx: UnboundedSender<String>) {
    let mut child = Command::new("sh")
        .arg("./scripts/models.sh")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // Ensure curl/wget logs don't overwrite the UI
        .spawn()
        .expect("Failed to execute models.sh");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    loop {
        tokio::select! {
            res = stdout_reader.next_line() => {
                match res {
                    Ok(Some(line)) => { let _ = tx.send(format!("Downloading: {}", line)); }
                    Ok(None) => break,
                    Err(_) => break,
                }
            }
            res = stderr_reader.next_line() => {
                if let Ok(Some(line)) = res {
                    let _ = tx.send(format!("[DL-LOG] {}", line));
                }
            }
        }
    }
    
    let _ = child.wait().await;
    let _ = tx.send("Models Installed!".to_string());
}