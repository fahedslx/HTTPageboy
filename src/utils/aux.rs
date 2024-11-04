pub fn normalize_path(path: &String) -> String {
	let current_dir: String = std::env::current_dir().unwrap().to_str().unwrap().to_string();
	let normalized_path: String;
	if path.starts_with('/') {
		normalized_path = path.to_string();
	}
	else if path.starts_with("./") {
		normalized_path = current_dir.to_string() + &path[1..];
	}
	else {
		normalized_path = current_dir.to_string() + "/" + &path;
	}

	return normalized_path;
}


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
