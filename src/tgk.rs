use regex::Regex;
use std::error::Error;
use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub enum TgkError {
    Reqwest(reqwest::Error),
    StatusError,
    ParsingError,
}

pub fn current_temperature() -> Result<f64, TgkError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut resp = client.get("http://tgk1.org/utils/external_view.php").send()?;
    if !resp.status().is_success() {
        return Err(TgkError::StatusError)
    }

    let text = resp.text()?;
    println!("{}", text);
    match parse_temperature(&text) {
        Ok(t) => Ok(t),
        Err(_) => Err(TgkError::ParsingError)
    }
}

fn parse_temperature(body: &str) -> Result<f64, ()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?[\d.,]+)&deg;C").unwrap();
    }

    match RE.captures(body) {
        Some(caps) => {
            let degrees = caps.get(1).unwrap().as_str();

            Ok(degrees.parse().unwrap())
        },
        None => Err(()),
    }
}

impl fmt::Display for TgkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TgkError::Reqwest(ref err) => err.fmt(f),
            TgkError::StatusError => write!(f, "HTTP status error."),
            TgkError::ParsingError => write!(f, "HTML parsing error."),
        }
    }
}

impl Error for TgkError {
    fn description(&self) -> &str {
        match *self {
            TgkError::Reqwest(ref err) => err.description(),
            TgkError::StatusError => "status error",
            TgkError::ParsingError => "parsing error",
        }
    }
}

impl From<reqwest::Error> for TgkError {
    fn from(err: reqwest::Error) -> TgkError {
        TgkError::Reqwest(err)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use tgk::parse_temperature;

    #[test]
    fn temp_parsing() {
        let mut file = File::open("resources/tgk.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert_eq!(parse_temperature(&contents).unwrap(), -10.6);
    }
}