use tempfile::TempDir;
use vsop2013_rs::{Planet, Vsop2013, VsopError};

#[test]
fn test_subset() -> Result<(), VsopError> {
  let start = 2_268_932.5;
  let end = 2_816_124.5;

  let vsop = Vsop2013::new("./data")?;

  let subset = vsop.new_from_range(start, end + 1.0).unwrap();

  let dir = TempDir::new()?;

  let path = dir.path();
  let path = path.join("./subset_test.bin");

  subset.save(&path)?;

  let vsop = Vsop2013::from_file(&path)?;

  let mut s = String::new();

  let step = 136798.0;

  s.push('\n');
  s.push_str("  *** 1500 3000");
  s.push('\n');
  s.push('\n');

  for planet in Planet::iter() {
    let name: &str = planet.into();
    let name = name.to_uppercase();

    for n in 0..5 {
      let jd = start + (n as f64 * step);
      let r = vsop.calculate(planet, jd).unwrap();

      if r.position[0] != 0.0 {
        s.push_str(&format!(
          "  {:11} JD{:9.1}  X :{:16.12} ua    Y :{:16.12} ua    Z :{:16.12} ua\n",
          name, jd, r.position[0], r.position[1], r.position[2]
        ));
        s.push_str(&format!(
          "                           X':{:16.12} ua/d  Y':{:16.12} ua/d  Z':{:16.12} ua/d\n",
          r.velocity[0], r.velocity[1], r.velocity[2]
        ));
      }
    }
  }

  let control = std::fs::read_to_string("./data/VSOP2013_ctl.txt")?;
  let mut lines = control
    .lines()
    .peekable();

  loop {
    let peek = lines.peek();
    if peek != Some(&"  *** 1500 3000") {
      lines.next();
      continue;
    }

    break;
  }

  for (c, r) in lines.zip(s.lines().skip(1)) {
    assert_eq!(c.trim(), r.trim());
  }

  Ok(())
}
