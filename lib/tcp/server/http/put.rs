use bytes::Bytes;
use std::convert::Infallible;
use std::path::Path;
use tokio::fs;
use tokio::fs::create_dir_all;
use warp::Filter;
use warp::Reply;
use warp::http::Response;
use warp::hyper::Body;

use super::storage::STORAGE_PATH;

#[derive(Debug, serde::Deserialize)]
pub struct FileQuery {
    pub path: String,
}

pub async fn put(query: FileQuery, body: Bytes) -> Result<impl Reply, Infallible> {
    if query.path.trim().is_empty() {
        return Ok(Response::builder()
            .status(400)
            .body(Body::from("Provided file path is empty".to_string()))
            .unwrap());
    }

    let dest_dir = &*STORAGE_PATH;
    if let Err(e) = create_dir_all(&dest_dir).await {
        return Ok(Response::builder()
            .status(500)
            .body(Body::from(format!(
                "Failed to create destination directory: {}",
                e
            )))
            .unwrap());
    }

    let filename = Path::new(&query.path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("file");

    let dest_path = dest_dir.join(filename);

    match fs::write(&dest_path, &body).await {
        Ok(_) => {
            let msg = format!("File saved successfully to {:?}", dest_path);
            Ok(Response::builder()
                .status(200)
                .body(Body::from(msg))
                .unwrap())
        }
        Err(e) => Ok(Response::builder()
            .status(500)
            .body(Body::from(format!("Failed to save file: {}", e)))
            .unwrap()),
    }
}

pub fn router() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path("upload")
        .and(warp::post())
        .and(warp::query::<FileQuery>())
        .and(warp::body::bytes())
        .and_then(put)
}
