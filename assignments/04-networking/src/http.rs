use std::{fmt::Display, str::FromStr, string::ParseError};

use crate::error::AspirinEatsError;

/// Simple wrapper for an HTTP Request
#[derive(Debug)]
pub struct HttpRequest {
    /// The HTTP method used in the request (GET, POST, etc)
    pub method: Option<String>,

    /// The path requested by the client
    pub path: Option<String>,

    /// The body of the request
    pub body: Option<String>,
}

impl FromStr for HttpRequest {
    type Err = String;

    // Parse a string into an HTTP Request
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let body: Option<String> = match s.find("\r\n\r\n") {
            Some(idx) => Some(String::from(&s[(idx + "\r\n\r\n".len())..])),
            None => None,
        };

        let method: Option<String> = match s.find(" ") {
            Some(idx) => Some(String::from(&s[..idx])),
            None => None,
        };

        let remaining_substr = if let Some(ref method_str) = method {
            &s[(method_str.len() + 1)..] // Slicing from after the method
        } else {
            s // If no method was found, remaining substring is unchanged
        };

        let path: Option<String> = match remaining_substr.find(" ") {
            Some(idx) => Some(String::from(&remaining_substr[..idx])),
            None => None,
        };

        let http_req = Ok(HttpRequest { method, path, body });

        return http_req;
    }
}

pub struct HttpResponse {
    status_code: u16,
    status_text: String,
    body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: &str, body: &str) -> Self {
        HttpResponse {
            status_code,
            status_text: status_text.to_string(),
            body: body.to_string(),
        }
    }
}

impl Display for HttpResponse {
    /// Convert an HttpResponse struct to a valid HTTP Response
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code, self.status_text, self.body
        )
    }
}

impl From<AspirinEatsError> for HttpResponse {
    /// Given an error type, convert it to an appropriate HTTP Response
    fn from(value: AspirinEatsError) -> Self {
        match value {
            AspirinEatsError::InvalidRequest => {
                HttpResponse::new(400, "Bad Request", "Invalid Request")
            }
            AspirinEatsError::NotFound => HttpResponse::new(404, "Not Found", "Resource not found"),
            AspirinEatsError::MethodNotAllowed => {
                HttpResponse::new(405, "Method Not Allowed", "Method not allowed")
            }
            AspirinEatsError::Io(_) => {
                HttpResponse::new(500, "Internal Server Error", "Internal Server Error")
            }
            _ => HttpResponse::new(500, "Unknown Error", "Unknown Error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_from_str() {
        let request = "GET /orders HTTP/1.1\r\nHost: localhost:8080\r\n\r\nthis is the body.";
        let http_request = HttpRequest::from_str(request).unwrap();
        assert_eq!(http_request.method, Some("GET".to_string()));
        assert_eq!(http_request.path, Some("/orders".to_string()));
        assert_eq!(http_request.body, Some("this is the body.".to_string()));
    }

    #[test]
    fn test_http_response_to_string() {
        let response = HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!");
        assert_eq!(
            response.to_string(),
            "HTTP/1.1 200 OK\r\n\r\nWelcome to Aspirin Eats!"
        );
    }

    #[test]
    fn test_http_response_from_aspirin_eats_error() {
        let error = AspirinEatsError::InvalidRequest;
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 400);
        assert_eq!(response.status_text, "Bad Request");
        assert_eq!(response.body, "Invalid Request");

        let error = AspirinEatsError::NotFound;
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 404);
        assert_eq!(response.status_text, "Not Found");
        assert_eq!(response.body, "Resource not found");

        let error = AspirinEatsError::MethodNotAllowed;
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 405);
        assert_eq!(response.status_text, "Method Not Allowed");
        assert_eq!(response.body, "Method not allowed");

        let error = AspirinEatsError::Io(std::io::Error::new(std::io::ErrorKind::Other, "test"));
        let response: HttpResponse = error.into();
        assert_eq!(response.status_code, 500);
        assert_eq!(response.status_text, "Internal Server Error");
        assert_eq!(response.body, "Internal Server Error");
    }
}
