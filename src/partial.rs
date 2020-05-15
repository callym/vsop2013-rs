use crate::{Error, Header, Planet, Table, VsopError, VsopResults};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialVsop2013 {
  pub header: Header,
  pub tables: Vec<Table>,
}

impl PartialVsop2013 {
  pub fn parse<'a>(input: &'a str) -> Error<Self> {
    let (input, header) = Header::parse(input)?;

    let mut tables = vec![];
    let mut input_iter = input;

    for _ in 0..header.tables {
      let input = input_iter;

      let (input, table) = Table::parse(&header, input)?;

      tables.push(table);
      input_iter = input;
    }
    let input = input_iter;

    Ok((input, Self { header, tables }))
  }

  pub fn from_raw_file(path: impl AsRef<Path>) -> Result<Self, VsopError> {
    let input = std::fs::read_to_string(path)?;

    match Self::parse(&input) {
      Ok((_, vsop)) => Ok(vsop),
      Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
        println!("{}", nom::error::convert_error(&input, e));

        panic!();
      },
      _ => panic!(),
    }
  }

  pub fn from_bin_file(path: impl AsRef<Path>) -> Result<Self, VsopError> {
    let input = std::fs::read(path)?;

    let res = bincode::deserialize(&input)?;

    Ok(res)
  }

  pub fn save(&self, path: impl AsRef<Path>) -> Result<(), VsopError> {
    let encoded: Vec<u8> = bincode::serialize(self)?;
    std::fs::write(path, encoded)?;

    Ok(())
  }

  pub fn is_in_range(&self, jd: f64) -> bool {
    jd >= self.header.span_start && jd < self.header.span_end
  }

  pub fn is_in_range_partial(&self, jd: f64) -> bool {
    jd >= self.header.span_start || jd < self.header.span_end
  }

  pub fn get_table(&self, jd: f64) -> Option<&Table> {
    self.tables.iter().find(|t| jd >= t.start && jd < t.stop)
  }

  pub fn calculate(&self, planet: Planet, jd: f64) -> VsopResults {
    assert!(jd >= self.header.span_start && jd < self.header.span_end);

    let structure = match self.header.get_structure(planet) {
      Some(s) => s,
      None => {
        panic!("{:?}, {:?}\n{:#?}", planet, jd, self.header);
      }
    };

    let table = self.get_table(jd).unwrap();

    let iad = structure.offset - 1;
    let ncf = structure.coefficient_count;
    let nsi = structure.sub_intervals;

    let delta = self.header.interval / nsi as f64;

    let ik = (jd - table.start) / delta;
    let ik = ik as u32;
    let ik = if ik == nsi { ik - 1 } else { ik };

    let iloc = iad + (6 * ncf * ik);
    let dj0 = table.start + (ik as f64 * delta);

    let x = (2.0 * (jd - dj0) / delta) - 1.0;

    let mut tn = vec![0.0; ncf as usize];

    tn[0] = 1.0;
    tn[1] = x;

    for i in 2..ncf as usize {
      tn[i] = (2.0 * x * tn[i - 1]) - tn[i - 2];
    }

    let mut r = [0.0; 6];

    for i in 0..6 {
      for j in 0..ncf {
        let jp = ncf - j - 1;
        let jt = iloc + (ncf * i) + jp;

        r[i as usize] = r[i as usize] + (tn[jp as usize] * table.data[jt as usize]);
      }
    }

    r.into()
  }
}
