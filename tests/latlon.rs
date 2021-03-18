use rs_latlon::Location;

#[test]
fn latlon_error() {
  let location: Location = Location::new(0.0, 0.0, 100, 100, 1f64).unwrap();

  assert_eq!(location.from_latlon(10.0, 10.0).unwrap(), (10.0, 10.0));
  assert_eq!(location.from_latlon(-80.0, 100.0).unwrap(), (100.0, 100.0));

  assert_eq!(location.from_canvas(100.0, 100.0).unwrap(), (-80.0, 100.0));
}

#[test]
fn set_latlon() {
  let location: Location = Location::new(1.0, 1.0, 100, 100, 1f64).unwrap();

  assert_eq!(location.start_lat, 1.0);
  assert_eq!(location.start_lon, 1.0);
}
