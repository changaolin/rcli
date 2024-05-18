use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn}; // Add this import

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, HeaderMap, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            HeaderMap::new(),
            format!("File {} note found", p.display()),
        )
    } else {
        match tokio::fs::metadata(&p).await {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let mut content = String::new();
                    content.push_str("<html><body><ul>");
                    let mut entries = tokio::fs::read_dir(p).await.unwrap();
                    while let Some(entry) = entries.next_entry().await.unwrap() {
                        let path = entry.path();
                        let path = path.strip_prefix(&state.path).unwrap();
                        let path = path.to_str().unwrap();
                        content.push_str(&format!(
                            r#"<li><a href="/tower/{}">{}</a></li>"#,
                            path,
                            entry.file_name().to_string_lossy()
                        ));
                    }
                    content.push_str("</ul></body></html>");
                    // 设置Header 返回html
                    let mut headers = HeaderMap::new();
                    headers.insert("Content-Type", "text/html".parse().unwrap());
                    (StatusCode::OK, headers, content)
                } else {
                    match tokio::fs::read_to_string(p).await {
                        Ok(content) => {
                            info!("Read {} bytes", content.len());
                            (StatusCode::OK, HeaderMap::new(), content)
                        }
                        Err(e) => {
                            warn!("Error reading file: {:?}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                HeaderMap::new(),
                                e.to_string(),
                            )
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    HeaderMap::new(),
                    e.to_string(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, _, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
