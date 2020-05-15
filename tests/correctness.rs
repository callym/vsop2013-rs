use vsop2013_rs::{Planet, Vsop2013, VsopError};

#[test]
fn test_correctness() -> Result<(), VsopError> {
  let vsop = Vsop2013::new("./data")?;

  let ndat = 5;
  let year = vec![-4500, -3000, -1500, 0, 1500, 3000];
  let tzero = vec![
    77432.5, 625307.5, 1173182.5, 1721057.5, 2268932.5, 2816818.5,
  ];
  let step = 136798.0;

  let mut s = String::new();

  for (i, tzero) in tzero.iter().enumerate() {
    s.push('\n');
    s.push_str(&format!("  *** {:0<4} {:0<4}", year[i], year[i] + 1500));
    s.push('\n');
    s.push('\n');

    for planet in Planet::iter() {
      let name: &str = planet.into();
      let name = name.to_uppercase();

      for n in 0..ndat {
        let jd = tzero + (n as f64 * step);
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
  }

  let control = std::fs::read_to_string("./data/VSOP2013_ctl.txt")?;

  for (c, r) in control.lines().zip(s.lines()) {
    assert_eq!(c.trim(), r.trim());
  }

  Ok(())
}
