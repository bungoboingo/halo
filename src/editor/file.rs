use crate::FragmentShader;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io;

const FILE_EXT_FILTERS: [&'static str; 1] = ["wgsl"];

pub async fn load(path: PathBuf) -> Result<(PathBuf, Arc<FragmentShader>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

pub async fn save(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
    println!("Saving shader at path: {path:?}");
    let path = if let Some(path) = path {
        path
    } else {
        //TODO this lags UI
        rfd::AsyncFileDialog::new()
            .add_filter("supported shader extensions", &FILE_EXT_FILTERS)
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::SaveDialogueClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}

pub async fn open() -> Result<(PathBuf, Arc<String>), Error> {
    let shader = rfd::AsyncFileDialog::new()
        .add_filter("supported shader extensions", &FILE_EXT_FILTERS)
        .set_title("Open a WGSL file...")
        .pick_file()
        .await
        .ok_or(Error::OpenDialogueClosed)?;

    load(shader.path().to_owned()).await
}

#[derive(Debug, Clone)]
pub enum Error {
    IoError(io::ErrorKind),
    SaveDialogueClosed,
    OpenDialogueClosed,
}