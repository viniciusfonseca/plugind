use serde::Serialize;
use serde_json::json;

pub struct Json;

impl Json {

    pub fn response(status: u16, body: impl Serialize) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let body = serde_json::to_vec(&body)?;
        buf.extend_from_slice(format!("HTTP/1.1 {}\r\n", status).as_bytes());
        buf.extend_from_slice(b"Content-Type: application/json\r\n");
        buf.extend_from_slice(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
        buf.extend_from_slice(&body);
        Ok(buf)
    }

    pub fn error(status: u16, message: &str) -> anyhow::Error {
        let mut buf = String::new();
        let body = json!({ "message": message });
        let body = serde_json::to_string(&body).unwrap();
        buf.push_str(&format!("HTTP/1.1 {}\r\n", status));
        buf.push_str("Content-Type: application/json\r\n");
        buf.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
        buf.push_str(&body);
        anyhow::Error::msg(buf)
    }
}

pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

impl TryFrom<Vec<u8>> for HttpResponse {

    type Error = anyhow::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let mut headers = [httparse::EMPTY_HEADER; 128];
        let mut res = httparse::Response::new(&mut headers);
        let parse_status = res.parse(&bytes)?;
        if let httparse::Status::Complete(len) = parse_status {
            Ok(HttpResponse { status: res.code.unwrap_or(0), body: bytes[..len].to_vec() })
        }
        else {
            Err(anyhow::Error::msg("Failed to parse HTTP response"))
        }
    }
}