use crate::{util::*, Error, Header};
use nom::character::complete::space0;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
  pub start: f64,
  pub stop: f64,
  pub data: Vec<f64>,
}

impl Table {
  pub fn parse<'a>(header: &Header, input: &'a str) -> Error<'a, Self> {
    let (input, start) = parse_f64(input)?;
    let (input, _) = space0(input)?;
    let (input, stop) = parse_f64(input)?;

    let mut data = vec![];
    let mut input_iter = input;

    for _ in 0..header.table_size {
      let input = input_iter;

      let (input, val) = parse_f64(input)?;
      let (input, exp) = parse_f64(input)?;

      data.push(val * f64::powf(10.0, exp));
      input_iter = input;
    }

    let input = input_iter;

    Ok((input, Self { start, stop, data }))
  }

  pub fn is_in_range(&self, jd: f64) -> bool {
    jd >= self.start && jd < self.stop
  }

  pub fn is_in_range_partial(&self, jd: f64) -> bool {
    jd >= self.start || jd < self.stop
  }
}
