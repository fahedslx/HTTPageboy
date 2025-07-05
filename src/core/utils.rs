pub fn get_content_type_quick(filename: &String) -> String {
  let extension: Option<&str> = filename.split('.').last();

  let content_type: &str = match extension {
    Some("png") => "image/png",
    Some("jpg") | Some("jpeg") => "image/jpeg",
    Some("gif") => "image/gif",
    Some("bmp") => "image/bmp",
    Some("svg") => "image/svg+xml",
    Some("webp") => "image/webp",
    Some("html") => "text/html",
    Some("css") => "text/css",
    Some("js") => "application/javascript",
    Some("json") => "application/json",
    Some("xml") => "application/xml",
    Some("pdf") => "application/pdf",
    Some("doc") | Some("docx") => "application/msword",
    Some("xls") | Some("xlsx") => "application/vnd.ms-excel",
    Some("ppt") | Some("pptx") => "application/vnd.ms-powerpoint",
    Some("zip") => "application/zip",
    Some("rar") => "application/x-rar-compressed",
    Some("txt") => "text/plain",
    Some("csv") => "text/csv",
    Some("mp3") => "audio/mpeg",
    Some("wav") => "audio/wav",
    Some("mp4") => "video/mp4",
    Some("avi") => "video/x-msvideo",
    Some("mov") => "video/quicktime",
    Some("ogg") => "audio/ogg",
    Some("ogv") => "video/ogg",
    Some("oga") => "audio/ogg",
    Some("ico") => "image/x-icon",
    _ => "application/octet-stream",
  };

  return content_type.to_string();
}

pub fn secure_path(base: &str, req_path: &str) -> Option<String> {
  let path = req_path.split('?').next().unwrap_or("");
  let path = path.trim_start_matches('/');
  let mut full = std::path::Path::new(base).join(path);
  // println!("🔍 secure_path base: {:?}, full: {:?}", base, full);
  if full.is_dir() {
    full = full.join("index.html");
  }
  if let Ok(canon) = full.canonicalize() {
    // println!("🔍 canonical full path: {:?}", canon);
    if let Ok(abs_base) = std::path::Path::new(base).canonicalize() {
      // println!("🔍 canonical base path: {:?}", abs_base);
      if canon.starts_with(&abs_base) {
        return Some(canon.to_string_lossy().to_string());
      }
    }
  }
  None
}
