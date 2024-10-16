use std::collections::HashMap;
use std::{fmt, str::FromStr};

use derive_more::{Display, Error};

#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub rl_method: String,
    pub rl_target: String,
    pub rl_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Clone, Copy, Debug, Display, Error)]
pub struct ParseRequestError;

impl FromStr for Request {
    type Err = ParseRequestError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let mut rl = lines.next().ok_or(ParseRequestError)?.split_whitespace();
        let rl_method = rl.next().ok_or(ParseRequestError)?.to_owned();
        let rl_target = rl.next().ok_or(ParseRequestError)?.to_owned();
        let rl_version = rl.next().ok_or(ParseRequestError)?.to_owned();

        let headers = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.split_once(':').map(|(k, v)| (k.trim().to_owned(), v.trim().to_owned())))
            .collect::<Option<_>>()
            .ok_or(ParseRequestError)?;

        let body = lines.collect();

        Ok(Request { rl_method, rl_target, rl_version, headers, body })
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {} {}", self.rl_method, self.rl_target, self.rl_version)?;

        for (name, value) in &self.headers {
            writeln!(f, "{}: {}", name, value)?;
        }

        writeln!(f)?; // Empty line

        write!(f, "{}", self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_request() -> Result<(), ParseRequestError> {
        let req: Request = "GET / HTTP/1.1\nFoo: bar\n\nHello, world!".parse()?;

        let foo = Request { 
            rl_method: "GET".to_owned(), 
            rl_target: "/".to_owned(), 
            rl_version: "HTTP/1.1".to_owned(), 
            headers: HashMap::from([("Foo".to_owned(), "bar".to_owned())]), 
            body: "Hello, world!".to_owned(),
        };

        assert_eq!(req, foo);

        Ok(())
    }
}