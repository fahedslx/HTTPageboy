#[allow(dead_code)]
pub enum StatusCode {
	Ok = 200,
	BadRequest = 400,
	Unauthorized = 401,
	Forbidden = 403,
	NotFound = 404,
	MethodNotAllowed = 405,
	InternalServerError = 500,
}

impl ToString for StatusCode {
	fn to_string (&self) -> String {
		let status_code = match self {
			StatusCode::Ok => "200 OK",
			StatusCode::BadRequest => "400 Bad Request",
			StatusCode::Unauthorized => "401 Unauthorized",
			StatusCode::Forbidden => "403 Forbidden",
			StatusCode::NotFound => "404 Not Found",
			StatusCode::MethodNotAllowed => "405 Method Not Allowed",
			StatusCode::InternalServerError => "500 Internal Server Error",
		}.to_string();

		return status_code;
	}
}
