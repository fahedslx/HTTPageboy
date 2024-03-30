use std::fmt::Debug;

pub type Rt = RequestType;

pub enum RequestType {
	GET,
	POST,
	PUT,
	DELETE,
	HEAD,
	OPTIONS,
	CONNECT,
	PATCH,
}

impl ToString for RequestType {
	fn to_string (&self) -> String {
		let request_type = match self {
			Rt::GET => "GET",
			Rt::POST => "POST",
			Rt::PUT => "PUT",
			Rt::DELETE => "DELETE",
			Rt::HEAD => "HEAD",
			Rt::OPTIONS => "OPTIONS",
			Rt::CONNECT => "CONNECT",
			Rt::PATCH => "PATCH",
		}.to_string();

		return request_type;
	}
}

impl PartialEq for RequestType {
	fn eq (&self, other: &Self) -> bool {
		return self.to_string() == other.to_string();
	}		
}

impl Clone for RequestType {
	fn clone(&self) -> Self {
		match self {
			Rt::GET => Rt::GET,
			Rt::POST => Rt::POST,
			Rt::PUT => Rt::PUT,
			Rt::DELETE => Rt::DELETE,
			Rt::HEAD => Rt::HEAD,
			Rt::OPTIONS => Rt::OPTIONS,
			Rt::CONNECT => Rt::CONNECT,
			Rt::PATCH => Rt::PATCH,
		}
	}
}

impl Debug for RequestType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Rt::GET => write!(f, "GET"),
			Rt::POST => write!(f, "POST"),
			Rt::PUT => write!(f, "PUT"),
			Rt::DELETE => write!(f, "DELETE"),
			Rt::HEAD => write!(f, "HEAD"),
			Rt::OPTIONS => write!(f, "OPTIONS"),
			Rt::CONNECT => write!(f, "CONNECT"),
			Rt::PATCH => write!(f, "PATCH"),
		}
	}
}
