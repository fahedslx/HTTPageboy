use crate::core::request::handle_request_async;
use crate::core::request_handler::Rh;
use crate::core::response::Response;
use crate::runtime::r#async::shared;
use crate::runtime::shared::print_server_info;
use async_trait::async_trait;
use smol::io::AsyncWriteExt;
use smol::net::{TcpListener, TcpStream};
use smol::spawn;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[async_trait]
impl shared::AsyncStream for TcpStream {
    async fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        AsyncWriteExt::write_all(self, buf).await
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        AsyncWriteExt::flush(self).await
    }

    async fn shutdown(&mut self) -> std::io::Result<()> {
        AsyncWriteExt::close(self).await
    }
}

/// A non-blocking HTTP server powered by Smol.
pub struct Server(pub shared::GenericServer<TcpListener>);

impl Deref for Server {
    type Target = shared::GenericServer<TcpListener>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Server {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Server {
    /// Creates a new server and binds to the specified URL.
    pub async fn new(
        serving_url: &str,
        routes_list: Option<HashMap<(crate::core::request_type::Rt, String), Rh>>,
    ) -> std::io::Result<Self> {
        let listener = TcpListener::bind(serving_url).await?;
        Ok(Server(shared::GenericServer {
            listener,
            routes: Arc::new(routes_list.unwrap_or_default()),
            files_sources: Arc::new(Vec::new()),
            auto_close: true,
        }))
    }

    /// Starts the server and begins accepting connections.
    pub async fn run(&self) {
        print_server_info(self.listener.local_addr().unwrap(), self.auto_close);
        loop {
            if let Ok((mut stream, _)) = self.listener.accept().await {
                let routes = self.routes.clone();
                let files = self.files_sources.clone();
                let close_flag = self.auto_close;

                spawn(async move {
                    let (mut req, early) =
                        crate::core::request::parse_stream_smol(&mut stream, &routes, &files).await;
                    let resp = match early {
                        Some(r) => r,
                        None => handle_request_async(&mut req, &routes, &files)
                            .await
                            .unwrap_or_else(Response::new),
                    };
                    shared::send_response(&mut stream, &resp, close_flag).await;
                })
                .detach();
            }
        }
    }
}