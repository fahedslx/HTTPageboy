use crate::core::response::Response;
use async_trait::async_trait;
use std::io::Result;

#[async_trait]
pub trait AsyncStream: Send + Sync {
    async fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    async fn flush(&mut self) -> Result<()>;
    async fn shutdown(&mut self) -> Result<()>;
}

pub async fn send_response<S: AsyncStream>(stream: &mut S, resp: &Response, close: bool) {
  let conn_hdr = if close { "Connection: close\r\n" } else { "" };
  let head = format!(
    "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n",
    resp.status,
    resp.content_type,
    resp.content.len(),
    conn_hdr,
  );
  let _ = stream.write_all(head.as_bytes()).await;
  if resp.content_type.starts_with("image/") {
    let _ = stream.write_all(&resp.content).await;
  } else {
    let text = String::from_utf8_lossy(&resp.content);
    let _ = stream.write_all(text.as_bytes()).await;
  }
  let _ = stream.flush().await;
  if close {
    let _ = stream.shutdown().await;
  }
}
