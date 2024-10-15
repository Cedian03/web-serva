use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Response {
    pub sl_version: String,
    pub sl_code: String,
    pub sl_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(status_line: (String, String, String), headers: HashMap<String, String>, body: String) -> Response {
        let (sl_version, sl_code, sl_text) = status_line;
        Response { sl_version, sl_code, sl_text, headers, body }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {} {}", self.sl_version, self.sl_code, self.sl_text)?;

        for (name, value) in &self.headers {
            writeln!(f, "{}: {}", name, value)?;
        }

        writeln!(f)?; // Empty line

        write!(f, "{}", self.body)
    }
}

