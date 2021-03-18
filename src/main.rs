use rs_latlon::Location;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let location: Location = Location::new(45.120052, 125.639648, 1920, 1080, 0.1).unwrap();

  let (lat, lon) = location.from_xy(10f64, 10f64).unwrap();

  println!("{}, {}", lat, lon);

  Ok(())
}
