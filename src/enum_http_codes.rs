enum HttpStatusCode {
	Ok,
	Created,
	Accepted,
	NoContent,
	BadRequest,
	Unauthorized,
	Forbidden,
	NotFound,
	MethodNotAllowed,
	InternalServerError,
}

impl HttpStatusCode {
	fn code(&self) -> u16 {
		match self {
			Self::Ok => 200,
			Self::Created => 201,
			Self::Accepted => 202,
			Self::NoContent => 204,
			Self::BadRequest => 400,
			Self::Unauthorized => 401,
			Self::Forbidden => 403,
			Self::NotFound => 404,
			Self::MethodNotAllowed => 405,
			Self::InternalServerError => 500,
		}
	}

	fn message(&self) -> &str {
		match self {
			Self::Ok => "OK",
			Self::Created => "Created",
			Self::Accepted => "Accepted",
			Self::NoContent => "No Content",
			Self::BadRequest => "Bad Request",
			Self::Unauthorized => "Unauthorized",
			Self::Forbidden => "Forbidden",
			Self::NotFound => "Not Found",
			Self::MethodNotAllowed => "Method Not Allowed",
			Self::InternalServerError => "Internal Server Error",
		}
	}
}
