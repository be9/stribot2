use errors::StribotError;
use regex::Regex;
use reqwest::Url;
use std::time::{Duration, SystemTime};

pub fn current_temperature() -> Result<f64, StribotError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;

    let tick = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let rand = rand::random::<f64>().to_string();

    let url = Url::parse_with_params("http://weather.nsu.ru/loadata.php?std=three",
                                     &[("tick", tick), ("rand", rand)]).unwrap();

    let mut resp = client.get(url).send()?;
    if !resp.status().is_success() {
        return Err(StribotError::StatusError)
    }

    let text = resp.text()?;
    match parse_temperature(&text) {
        Ok(t) => Ok(t),
        Err(_) => Err(StribotError::ParsingError)
    }
}

fn parse_temperature(body: &str) -> Result<f64, ()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Температура около НГУ (-?[\d.,]+) C").unwrap();
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
    use nsu::parse_temperature;

    #[test]
    fn temp_parsing() {
        let mut file = File::open("resources/nsu.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert_eq!(parse_temperature(&contents).unwrap(), -7.8);
    }
}