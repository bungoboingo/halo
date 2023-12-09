use crate::FragmentShader;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::Arc;

const PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/preferences.json");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_shader_path: Option<PathBuf>,
    pub auto_validate: bool,
}

pub async fn load() -> Result<(Preferences, Arc<FragmentShader>), Error> {
    let file = tokio::fs::read_to_string(PATH)
        .await
        .map_err(|_| Error::Io)?;

    let prefs: Preferences = serde_json::from_str(&file).map_err(|_| Error::Deserialize)?;

    let shader = if let Some(shader_path) = &prefs.last_shader_path {
        tokio::fs::read_to_string(shader_path)
            .await
            .map_err(|e| {
                println!("Error reading shader at path: {shader_path:?} -- {e:?}");
                Error::Io
            })?
    } else {
        include_str!("viewer/shaders/default_frag.wgsl").to_string()
    };

    Ok((prefs, Arc::new(shader)))
}

pub async fn save(preferences: Preferences) -> Result<(), Error> {
    let pref = serde_json::to_string(&preferences).map_err(|_| Error::Serialize)?;

    tokio::fs::write(&PATH, pref).await.map_err(|_| Error::Io)
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    Io,
    Deserialize,
    Serialize,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
