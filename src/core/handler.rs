#![allow(unused_imports)]
use crate::Request;
use crate::Response;

#[cfg(feature = "sync")]
pub type Handler = fn(&Request) -> Response;

#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
pub type Handler = fn(&Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>;

/// Adapter: convierte un handler sync en uno async listo para usar.
/// Solo se usa en builds async, pero permite que `main` siga igual.
#[cfg(any(feature = "async_tokio", feature = "async_std", feature = "async_smol"))]
pub fn sync_handler_adapter(h: fn(&Request) -> Response) -> Handler {
  Box::new(move |req: &Request| {
    let resp = h(req);
    Box::pin(async move { resp })
  })
}
