#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Planet {
  Mercury,
  Venus,
  Earth,
  Mars,
  Jupiter,
  Saturn,
  Uranus,
  Neptune,
  Pluto,
}

impl Planet {
  pub fn iter() -> impl Iterator<Item = Planet> {
    PlanetIter(None)
  }
}

struct PlanetIter(Option<Planet>);

impl Iterator for PlanetIter {
  type Item = Planet;

  fn next(&mut self) -> Option<Self::Item> {
    use Planet::*;

    let next = match self.0 {
      None => Some(Mercury),
      Some(Mercury) => Some(Venus),
      Some(Venus) => Some(Earth),
      Some(Earth) => Some(Mars),
      Some(Mars) => Some(Jupiter),
      Some(Jupiter) => Some(Saturn),
      Some(Saturn) => Some(Uranus),
      Some(Uranus) => Some(Neptune),
      Some(Neptune) => Some(Pluto),
      Some(Pluto) => None,
    };

    self.0 = next;

    next
  }
}

impl Into<usize> for Planet {
  fn into(self) -> usize {
    match self {
      Planet::Mercury => 0,
      Planet::Venus => 1,
      Planet::Earth => 2,
      Planet::Mars => 3,
      Planet::Jupiter => 4,
      Planet::Saturn => 5,
      Planet::Uranus => 6,
      Planet::Neptune => 7,
      Planet::Pluto => 8,
    }
  }
}

impl Into<u32> for Planet {
  fn into(self) -> u32 {
    self as usize as u32
  }
}

impl std::cmp::PartialEq<u32> for Planet {
  fn eq(&self, other: &u32) -> bool {
    let s: u32 = (*self).into();

    s == *other
  }
}

impl Into<&'static str> for Planet {
  fn into(self) -> &'static str {
    match self {
      Planet::Mercury => "Mercury",
      Planet::Venus => "Venus",
      Planet::Earth => "Earth-Moon",
      Planet::Mars => "Mars",
      Planet::Jupiter => "Jupiter",
      Planet::Saturn => "Saturn",
      Planet::Uranus => "Uranus",
      Planet::Neptune => "Neptune",
      Planet::Pluto => "Pluto",
    }
  }
}
