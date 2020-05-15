use crate::{util::*, Error, Planet};
use nom::{combinator::map_parser, multi::many0};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TableDescriptor {
  pub planet: u32,
  pub coefficient_count: u32,
  pub offset: u32,
  pub sub_intervals: u32,
}

impl TableDescriptor {
  pub fn parse<'a>(input: &'a str) -> Error<Vec<Self>> {
    let mut structure = vec![TableDescriptor::empty(); PLANETS];

    let (input, offset) = map_parser(until_line_ending, many0(parse_u32))(input)?;

    let (input, coefficient_count) = map_parser(until_line_ending, many0(parse_u32))(input)?;

    let (input, sub_intervals) = map_parser(until_line_ending, many0(parse_u32))(input)?;

    for i in 0..PLANETS {
      structure[i] = Self {
        planet: i as u32,
        coefficient_count: coefficient_count[i],
        offset: offset[i],
        sub_intervals: sub_intervals[i],
      };
    }

    Ok((input, structure))
  }

  fn empty() -> Self {
    Self {
      planet: 0,
      coefficient_count: 0,
      offset: 0,
      sub_intervals: 0,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
  pub version: i32,
  pub span_start: f64,
  pub span_end: f64,
  pub interval: f64,
  pub tables: u32,
  pub table_size: u32,
  pub structure: Vec<TableDescriptor>,
}

impl Header {
  pub fn parse<'a>(input: &'a str) -> Error<Self> {
    let (input, version) = parse_i32_line(input)?;
    let (input, span_start) = parse_f64_line(input)?;
    let (input, span_end) = parse_f64_line(input)?;
    let (input, interval) = parse_f64_line(input)?;
    let (input, tables) = parse_u32_line(input)?;
    let (input, table_size) = parse_u32_line(input)?;
    let (input, structure) = TableDescriptor::parse(input)?;

    Ok((
      input,
      Header {
        version,
        span_start,
        span_end,
        interval,
        tables,
        table_size,
        structure,
      },
    ))
  }

  pub fn get_structure(&self, planet: Planet) -> Option<&TableDescriptor> {
    self.structure
      .iter()
      .find(|d| d.planet == (planet as u32))
  }
}
