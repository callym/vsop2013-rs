use nom::IResult;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

mod header;
use header::Header;

mod partial;
use partial::PartialVsop2013;

mod planet;
pub use planet::Planet;

mod planet_data;
use planet_data::Table;

mod results;
pub use results::VsopResults;

mod util;

type Error<'a, T> = IResult<&'a str, T, nom::error::VerboseError<&'a str>>;

#[derive(Error, Debug)]
pub enum VsopError {
  #[error("error opening file")]
  IoError(#[from] std::io::Error),
  #[error("error parsing raw file")]
  NomError(#[from] nom::Err<(String, nom::error::ErrorKind)>),
  #[error("error loading binary file")]
  BincodeError(#[from] bincode::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vsop2013 {
  pub files: Vec<PartialVsop2013>,
}

impl Vsop2013 {
  pub fn new(path: impl AsRef<Path>) -> Result<Self, VsopError> {
    let base = PathBuf::from(path.as_ref());

    let files = util::FILES
      .iter()
      .map(|path| {
        let path = base.join(path);
        let bin_path = PathBuf::from(format!("{}.bin", path.to_string_lossy()));

        if bin_path.exists() {
          PartialVsop2013::from_bin_file(bin_path)
        } else {
          let file = PartialVsop2013::from_raw_file(path)?;

          file.save(bin_path)?;

          Ok(file)
        }
      })
      .collect::<Result<Vec<_>, _>>()?;

    Ok(Self { files })
  }

  pub fn from_file(path: impl AsRef<Path>) -> Result<Self, VsopError> {
    let input = std::fs::read(path)?;

    let res = bincode::deserialize(&input)?;

    Ok(res)
  }

  pub fn save(&self, path: impl AsRef<Path>) -> Result<(), VsopError> {
    let encoded: Vec<u8> = bincode::serialize(self)?;
    std::fs::write(path, encoded)?;

    Ok(())
  }

  pub fn new_from_range(&self, mut start: f64, mut end: f64) -> Option<Self> {
    if start > end {
      std::mem::swap(&mut start, &mut end);
    }

    let partials = &self.files;

    util::assert_all_eq("Header Version", partials.iter().map(|p| p.header.version));

    util::assert_all_eq("Header Interval", partials.iter().map(|p| p.header.interval));

    util::assert_all_eq("Header Table Size", partials.iter().map(|p| p.header.table_size));

    Planet::iter()
      .for_each(|planet| {
        util::assert_all_eq(
          "Coefficient Count",
          partials
            .iter()
            .map(move |p| p.header.structure[planet as usize])
            .map(|p| p.coefficient_count)
        );
      });

    Planet::iter()
      .for_each(|planet| {
        util::assert_all_eq(
          "Offset",
          partials
            .iter()
            .map(move |p| p.header.structure[planet as usize])
            .map(|p| p.offset)
        );
      });

    Planet::iter()
      .for_each(|planet| {
        util::assert_all_eq(
          "Sub Intervals",
          partials
            .iter()
            .map(move |p| p.header.structure[planet as usize])
            .map(|p| p.sub_intervals)
        );
      });

    let header = &partials.first()?.header;

    let start_low = start - header.interval;
    let end_high = end + header.interval;

    let tables = partials
      .iter()
      .map(|p| p.tables.clone())
      .flatten()
      .filter(|t| t.start >= start_low && t.stop < end_high)
      .collect::<Vec<_>>();

    assert!(tables.len() > 0);

    let header = Header {
      version: 2013,
      span_start: tables.first()?.start,
      span_end: tables.last()?.stop,
      interval: header.interval,
      tables: tables.len() as u32,
      table_size: header.table_size,
      structure: header.structure.clone(),
    };

    let partial = PartialVsop2013 { header, tables };

    Some(Self {
      files: vec![partial],
    })
  }

  pub fn new_from_range_planets(&self, start: f64, end: f64, planets: &[Planet]) -> Option<Self> {
    let mut range = self.new_from_range(start, end)?;

    let header = &range.files[0].header.clone();

    let planets_u32 = planets.iter()
      .map(|p| {
        let p: u32 = (*p).into();

        p + 1
      })
      .collect::<Vec<u32>>();

    let mut offset_prev = 1;

    #[derive(Debug, Clone)]
    struct Ranges {
      planet: u32,
      min: u32,
      max: u32,
    }

    let mut ranges = vec![];
    for desc in header.structure.iter().skip(1) {
      ranges.push(Ranges {
        planet: desc.planet,
        min: offset_prev - 1,
        max: desc.offset,
      });

      offset_prev = desc.offset + 1;
    }

    let ranges_to_keep = ranges
      .iter()
      .filter(|k| planets_u32.contains(&k.planet))
      .collect::<Vec<_>>();

    let ranges_to_delete = ranges
      .iter()
      .filter(|k| !planets_u32.contains(&k.planet))
      .collect::<Vec<_>>();

    let mut keep: Vec<Ranges> = ranges_to_keep
      .iter()
      .map(|p| (*p).clone())
      .collect::<Vec<_>>();

    for keep in keep.iter_mut() {
      for del in &ranges_to_delete {
        if keep.planet > del.planet {
          let diff = del.max - del.min;

          keep.min -= diff;
          keep.max -= diff;
        }
      }
    }

    for table in range.files[0].tables.iter_mut() {
      table.data = table.data
        .iter()
        .enumerate()
        .filter(|(i, _)| {
          let i = (*i) as u32;
          for del in &ranges_to_delete {
            if i >= del.min && i < del.max {
              return false;
            }
          }

          true
        })
        .map(|(_, i)| *i)
        .collect::<Vec<_>>();
    }

    let mut header = header.clone();

    for desc in header.structure.iter_mut() {
      for keep in &keep {
        if desc.planet == keep.planet {
          desc.offset = keep.max;
        }
      }
    }

    header.structure = header.structure
      .iter()
      .filter(|d| planets_u32.contains(&d.planet))
      .map(|d| {
        let mut d = d.clone();
        let k = &keep.iter().find(|k| k.planet == d.planet)?;
        d.offset = k.max;
        Some(d)
      })
      .flatten()
      .collect::<Vec<_>>();

    range.files[0].header = header;

    Some(range)
  }

  pub fn get_partial(&self, jd: f64) -> Option<&PartialVsop2013> {
    self.files.iter().find(|f| f.is_in_range(jd))
  }

  pub fn get_table(&self, jd: f64) -> Option<&Table> {
    self
      .files
      .iter()
      .find(|f| f.is_in_range(jd))
      .map(|f| f.get_table(jd))
      .flatten()
  }

  pub fn tables(&self) -> impl Iterator<Item = &Table> {
    self.files.iter()
      .map(|f| &f.tables)
      .flatten()
  }

  pub fn calculate(&self, planet: Planet, jd: f64) -> Option<VsopResults> {
    let partial = self.get_partial(jd)?;

    Some(partial.calculate(planet, jd))
  }
}
