use std::io::Read;

use actix_multipart::form::tempfile::TempFile;
use uuid::Uuid;

const THUMBNAILS_PATH: &str = "static_data/users/thumbnails";

pub async fn try_persist_thumbnail(tf: TempFile) -> Option<String> {
    let mut buff = Vec::with_capacity(tf.size);
    match tf.file.as_file().read_to_end(&mut buff) {
        Ok(_) => {
            let new_name = Uuid::new_v4().to_string();
            let ext = if let Some(ct) = tf.content_type {
                format!(".{}", ct.subtype().to_string())
            } else {
                String::default()
            };

            let path = format!("{THUMBNAILS_PATH}/{new_name}{ext}");
            _ = tokio::fs::write(&path, buff).await;

            Some(path)
        }
        _ => None,
    }
}
