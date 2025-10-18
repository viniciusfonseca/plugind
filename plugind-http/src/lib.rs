pub struct Json;

impl Json {

    pub fn response(status: u16, body: serde_json::Value) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let body = serde_json::to_vec(&body)?;
        buf.extend_from_slice(format!("HTTP/1.1 {}\r\n", status).as_bytes());
        buf.extend_from_slice(b"Content-Type: application/json\r\n");
        buf.extend_from_slice(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
        buf.extend_from_slice(&body);
        Ok(buf)
    }
}
