use crate::prelude::*;
use tokio::fs;

pub async fn on_file_write() -> crate::Result<()> {
    let path = "/storage/emulated/0/Android/data/io.rousan.androidwithrust/files/abcd";

    fs::write(path, "hello world2")
        .await
        .context("Failed to write to the file")?;

    Ok(())
}
