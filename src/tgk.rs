use chrono::NaiveDateTime;
use regex::Regex;
use std::time::Duration;

use errors::StribotError;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Name};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct TempReading {
    datetime: NaiveDateTime,
    temperature: f64,
}

#[derive(PartialEq, Debug)]
pub struct MinMax {
    min: TempReading,
    max: TempReading,
}

pub fn current_temperature() -> Result<f64, StribotError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let mut resp = client.get("http://tgk1.org/utils/external_view.php").send()?;
    if !resp.status().is_success() {
        return Err(StribotError::StatusError)
    }

    let text = resp.text()?;
    match parse_temperature(&text) {
        Some(t) => Ok(t),
        None => Err(StribotError::ParsingError)
    }
}

pub fn current_minmax() -> Result<MinMax, StribotError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut resp = client.get("http://tgk1.org/utils/table_ext.php").send()?;
    if !resp.status().is_success() {
        return Err(StribotError::StatusError)
    }

    let text = resp.text()?;
    match parse_minmax(&text, None) {
        Some(v) => Ok(v),
        None => Err(StribotError::ParsingError)
    }
}


fn parse_temperature(body: &str) -> Option<f64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?[\d.,]+)&deg;C").unwrap();
    }

    match RE.captures(body) {
        Some(caps) => {
            let degrees = caps.get(1).unwrap().as_str();

            Some(degrees.parse().unwrap())
        },
        None => None,
    }
}

fn parse_minmax(body: &str, not_before: Option<NaiveDateTime>) -> Option<MinMax> {
    let document = Document::from(body);
    let mut temp_records: Vec<TempReading> = Vec::new();

    for row in document.find(Name("table").descendant(Name("tr"))).skip(1) {
        let mut cells = row.find(Name("td"));
        let mut datetime: NaiveDateTime;
        let mut temperature: f64;

        if let Some(dt) = parse_datetime_cell(cells.next()) {
            // println!("{}", datetime);
            datetime = dt;
        } else {
            continue;
        }

        if let Some(temp) = parse_temperature_cell(cells.next()) {
            temperature = temp;
        } else {
            continue;
        }

        let add = match not_before {
            Some(not_before) => datetime >= not_before,
            None => true,
        };

        if add {
            temp_records.push(TempReading { datetime, temperature });
        }
    }

    let mut min: Option<TempReading> = None;
    let mut max: Option<TempReading> = None;

    for record in temp_records {
        match min {
            Some(min_reading) => {
                if record.temperature < min_reading.temperature {
                    min = Some(record);
                }
            },
            None => {
                min = Some(record);
            },
        }

        match max {
            Some(max_reading) => {
                if record.temperature > max_reading.temperature {
                    max = Some(record);
                }
            },
            None => {
                max = Some(record);
            },
        }
    }

    match (min, max) {
        (Some(min), Some(max)) => 
            Some(MinMax { 
                min,
                max,
            }),
        _ => None,
    }
}

fn parse_datetime_cell(cell: Option<Node>) -> Option<NaiveDateTime> {
    match cell {
        Some(td) => match NaiveDateTime::parse_from_str(&td.text(), "%Y-%m-%d %H:%M:%S") {
            Ok(result) => Some(result),
            Err(_err) => None,
        }
        None => None,
    }
}

fn parse_temperature_cell(cell: Option<Node>) -> Option<f64> {
    match cell {
        Some(td) => match td.text().replace(",", ".").parse() {
            Ok(res) => Some(res),
            Err(_) => None,
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use tgk::MinMax;
    use tgk::TempReading;
    use tgk::parse_temperature;
    use tgk::parse_minmax;

    #[test]
    fn temp_parsing() {
        let contents = include_str!("../resources/tgk.html");

        assert_eq!(parse_temperature(&contents).unwrap(), -10.6);
    }

    #[test]
    fn test_parse_minmax() {
        let contents = include_str!("../resources/tgk_table.html");
        let expected = MinMax {
            min: TempReading {
                temperature: -14.3,
                datetime: NaiveDate::from_ymd(2018, 11, 27).and_hms(8, 42, 0),
            },
            max: TempReading {
                temperature: -7.6,
                datetime: NaiveDate::from_ymd(2018, 11, 26).and_hms(15, 17, 0),
            },
        };

        assert_eq!(parse_minmax(&contents, None).unwrap(),
                   expected);
    }
}