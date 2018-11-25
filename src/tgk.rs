use regex::Regex;
use std::time::Duration;

use errors::StribotError;

pub fn current_temperature() -> Result<f64, StribotError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut resp = client.get("http://tgk1.org/utils/external_view.php").send()?;
    if !resp.status().is_success() {
        return Err(StribotError::StatusError)
    }

    let text = resp.text()?;
    println!("{}", text);
    match parse_temperature(&text) {
        Ok(t) => Ok(t),
        Err(_) => Err(StribotError::ParsingError)
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