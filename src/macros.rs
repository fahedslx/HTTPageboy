/// Generates a `parse_stream` function for a specific async runtime.
///
/// This macro abstracts the common logic of reading and parsing an HTTP request
/// from a TCP stream, while allowing the caller to specify the runtime-specific
/// types and traits (stream type, BufReader, and I/O extension traits).
macro_rules! create_async_parse_stream {
    (
        $(#[$outer:meta])*
        $func_name:ident,
        $stream_ty:ty,
        $buf_reader:ty,
        $async_read_ext:path,
        $async_buf_read_ext:path
    ) => {
        $(#[$outer])*
        pub async fn $func_name(
            stream: &mut $stream_ty,
            routes: &std::collections::HashMap<(crate::core::request_type::Rt, String), crate::core::request_handler::Rh>,
            file_bases: &[String],
        ) -> (crate::core::request::Request, Option<crate::core::response::Response>) {
            use $async_read_ext;
            use $async_buf_read_ext;

            let mut reader = <$buf_reader>::new(stream);
            let mut raw = String::new();

            // Read headers only
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).await.ok().filter(|&n| n > 0).is_none() {
                    break;
                }
                raw.push_str(&line);
                if raw.contains("\r\n\r\n") {
                    break;
                }
            }

            // Extract method from the first line
            let method = raw
                .lines()
                .next()
                .and_then(|l| l.split_whitespace().next())
                .unwrap_or("");

            // Determine content length
            let content_length = raw
                .lines()
                .find_map(|l| {
                    if l.to_ascii_lowercase().starts_with("content-length:") {
                        l.split(':').nth(1)?.trim().parse::<usize>().ok()
                    } else {
                        None
                    }
                })
                .unwrap_or(0);

            if content_length > 0 {
                // Read exactly content_length bytes
                let mut buf = vec![0; content_length];
                let _ = reader.read_exact(&mut buf).await;
                raw.push_str(&String::from_utf8_lossy(&buf));
            } else if method != "GET" {
                // Read all until EOF for POST/PUT/DELETE without Content-Length
                let mut rest = String::new();
                let _ = reader.read_to_string(&mut rest).await;
                raw.push_str(&rest);
            }

            crate::core::request::Request::parse_raw_async(raw, routes, file_bases).await
        }
    };
}