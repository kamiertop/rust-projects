use std::path::{Path};
use anyhow::Result;
use axum::{
	Router,
};
use tower_http::services::ServeDir;
use tracing::{info};

// 主函数，用于启动 HTTP 服务器并服务指定目录
pub async fn process_http_serve(path: &Path, port: Option<u16>) -> Result<()> {
	let path_buf = path.to_path_buf();

	let dir_service = ServeDir::new(path_buf.clone()).
		append_index_html_on_directories(true).
		precompressed_gzip().
		precompressed_br().
		precompressed_deflate().
		precompressed_zstd();

	let router = Router::new()
		.fallback_service(dir_service);

	let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port.unwrap_or(4070)));

	// 绑定 TCP 监听器
	let listener = tokio::net::TcpListener::bind(addr).await?;

	info!("Starting HTTP server on port {}", port.unwrap_or(4070));

	// 启动 Axum 服务器
	axum::serve(listener, router).await?;

	Ok(())
}