use rs_latlon::Location;

fn main() {
  let location: Location = Location::new(45.120052, 125.639648, 1920, 1080, 0.1);

  let (lat, lon) = location.from_xy(10f64, 10f64);

  println!("{}, {}", lat, lon);
}
