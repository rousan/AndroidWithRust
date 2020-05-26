use crate::prelude::*;
use crate::{
    bridge::{self, MessageData},
    message,
};
use std::path::{Path, PathBuf};
use tokio::fs::{self, File, OpenOptions};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;

static mut INIT_DATA: Option<MessageData> = None;

pub async fn start_server() -> crate::Result<()> {
    let addr = "0.0.0.0:6142";
    let mut listener = TcpListener::bind(addr)
        .await
        .context("Failed to bind server to a port")?;

    info!("Server is listening at: {}", addr);
    bridge::send_message(message::what::SERVER_STARTED, MessageData::empty());

    while let Some(socket) = listener.next().await {
        match socket {
            Ok(socket) => {
                tokio::spawn(async move {
                    if let Err(err) = process_socket(socket).await {
                        error!("Process socket error: {:?}", err);
                    }
                });
            }
            Err(err) => {
                error!("Socket err: {}", err);
            }
        }
    }

    Ok(())
}

async fn process_socket(mut socket: TcpStream) -> crate::Result<()> {
    let file_name_len = socket.read_u16().await.context("Failed to read file name length")? as usize;
    let mut file_name_bytes = vec![0; file_name_len];

    socket
        .read_exact(&mut file_name_bytes)
        .await
        .context("Failed to read file name bytes")?;

    let file_name = String::from_utf8(file_name_bytes).context("Failed to convert file name bytes as string")?;
    let dest_path = PathBuf::from(data("output_path"));

    fs::create_dir_all(dest_path.as_path())
        .await
        .context("Failed to create destination path")?;

    let dest_file_path = dest_path.join(file_name);

    info!("Destination file path: {}", dest_file_path.to_str().unwrap_or(""));

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(dest_file_path.as_path())
        .await
        .context("Failed to open destination file to write")?;

    io::copy(&mut socket, &mut file)
        .await
        .context("Failed to copy socket data to the destination file")?;

    info!("File received");

    Ok(())
}

pub fn set_init_data(data: MessageData) {
    unsafe {
        INIT_DATA.replace(data);
    }
}

fn data(key: &str) -> String {
    unsafe { INIT_DATA.as_ref().unwrap().get_string(key).unwrap() }
}

pub async fn send_file(ip: &str, path: &str) -> crate::Result<()> {
    let src_full_path = Path::new(path);

    let filename = src_full_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("file");

    let mut socket = TcpStream::connect((ip, 6142))
        .await
        .context("Failed to connect with the peer server")?;

    socket
        .write_u16(filename.len() as u16)
        .await
        .context("Failed to write file name length")?;

    socket
        .write_all(filename.as_bytes())
        .await
        .context("Failed to write file name bytes")?;

    let mut file = OpenOptions::new()
        .read(true)
        .open(src_full_path)
        .await
        .context("Failed to open file to send")?;

    io::copy(&mut file, &mut socket)
        .await
        .context("Failed to copy source file data to the socket")?;

    info!("File sent");

    Ok(())
}
