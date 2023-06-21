use std::io::Read;

use actix_multipart::form::tempfile::TempFile;
use uuid::Uuid;
use crate::STATIC_DATA_PATH;

pub async fn try_persist_file(path: &str, tf: TempFile) -> Option<String> {
    let mut buff = Vec::with_capacity(tf.size);
    match tf.file.as_file().read_to_end(&mut buff) {
        Ok(_) => {
            let new_name = Uuid::new_v4().to_string();
            let ext = if let Some(mime) = tf.content_type {
                format!(".{}", mime.subtype().to_string())
            } else {
                String::default()
            };

            let path = format!("{STATIC_DATA_PATH}/{path}{new_name}{ext}");
            _ = tokio::fs::write(&path, buff).await;

            Some(path)
        }
        _ => None,
    }
}
