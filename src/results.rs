#[derive(Debug, Copy, Clone)]
pub struct VsopResults {
  pub position: [f64; 3],
  pub velocity: [f64; 3],
}

impl From<[f64; 6]> for VsopResults {
  fn from(data: [f64; 6]) -> Self {
    let position = [data[0], data[1], data[2]];

    let velocity = [data[3], data[4], data[5]];

    Self { position, velocity }
  }
}
