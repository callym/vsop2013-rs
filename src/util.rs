use crate::Error;
use nom::{
  bytes::complete::take_till,
  character::complete::{line_ending, space0},
  combinator::{map_parser, map_res, opt},
};

pub const PLANETS: usize = 9;

pub const FILES: [&'static str; 6] = [
  "VSOP2013.m4000",
  "VSOP2013.m2000",
  "VSOP2013.m1000",
  "VSOP2013.p1000",
  "VSOP2013.p2000",
  "VSOP2013.p4000",
];

pub fn assert_all_eq<T, U>(message: impl AsRef<str>, iter: T) -> bool
where
  T: IntoIterator<Item = U>,
  U: PartialEq + std::fmt::Debug,
{
  let mut iter = iter.into_iter();

  let first = match iter.next() {
    Some(v) => v,
    None => return true,
  };

  iter.all(|item| {
    if item != first {
      println!("{}", message.as_ref());
      assert_eq!(item, first);
    }

    item == first
  })
}

pub fn until_line_ending<'a>(input: &'a str) -> Error<&'a str> {
  let (input, _) = space0(input)?;
  let (input, res) = take_till(|i| i == '\n' || i == '\r')(input)?;
  let (input, _) = opt(line_ending)(input)?;

  Ok((input, res))
}

pub fn until_space<'a>(input: &'a str) -> Error<&'a str> {
  let (input, _) = space0(input)?;
  let (input, res) = take_till(|i| i == '\n' || i == '\r' || i == ' ')(input)?;
  let (input, _) = opt(line_ending)(input)?;

  Ok((input, res))
}

pub fn parse_i32<'a>(input: &'a str) -> Error<i32> {
  map_res(until_space, |res| i32::from_str_radix(res, 10))(input)
}

pub fn parse_u32<'a>(input: &'a str) -> Error<u32> {
  map_res(until_space, |res| u32::from_str_radix(res, 10))(input)
}

pub fn parse_f64<'a>(input: &'a str) -> Error<f64> {
  map_res(until_space, |res| res.parse())(input)
}

pub fn parse_i32_line<'a>(input: &'a str) -> Error<i32> {
  map_parser(until_line_ending, parse_i32)(input)
}

pub fn parse_u32_line<'a>(input: &'a str) -> Error<u32> {
  map_parser(until_line_ending, parse_u32)(input)
}

pub fn parse_f64_line<'a>(input: &'a str) -> Error<f64> {
  map_parser(until_line_ending, parse_f64)(input)
}
