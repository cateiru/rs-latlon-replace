pub struct Location {
  pub start_lat: f64,
  pub start_lon: f64,
  pub width: usize,
  pub height: usize,
  pub zoom_level: f64,
}

impl Location {
  /// Create Location instance.
  ///
  /// lat: width
  /// lon: height
  ///
  /// ### Arguments
  ///
  /// - `lat` - lat of image start (0, 0)
  /// - `lon` - lon of image start (0, 0)
  /// - `width` - width of image size.
  /// - `height` - height of image size.
  /// - `zoom_level` - zoom level.
  pub fn new(lat: f64, lon: f64, width: usize, height: usize, zoom_level: f64) -> Self {
    assert!(
      zoom_level != 0f64,
      "Error: zoom_level({}) is not 0.",
      zoom_level
    );

    let lat: f64 = parse_lat(lat);
    let lon: f64 = parse_lon(lon);

    Self {
      start_lat: lat,
      start_lon: lon,
      width: width,
      height: height,
      zoom_level: zoom_level,
    }
  }

  /// Create Location instance width a center.
  ///
  /// lat: width
  /// lon: height
  ///
  /// ### Arguments
  ///
  /// - `lat` - lat of center.
  /// - `lon` - lon of center.
  /// - `width` - width of image size.
  /// - `height` - height of image size.
  /// - `zoom_level` - zoom level.
  pub fn new_center(lat: f64, lon: f64, width: usize, height: usize, zoom_level: f64) -> Self {
    assert!(
      zoom_level != 0f64,
      "Error: zoom_level({}) is not 0.",
      zoom_level
    );

    let half_lat: f64 = (width as f64 / 2f64) * zoom_level;
    let half_lon: f64 = (height as f64 / 2f64) * zoom_level;

    let start_lat: f64 = mod_positive((lat + 90f64) - half_lat, 180f64) - 90f64;
    let start_lon: f64 = mod_positive((lon + 180f64) - half_lon, 360f64) - 180f64;

    Self {
      start_lat: start_lat,
      start_lon: start_lon,
      width: width,
      height: height,
      zoom_level: zoom_level,
    }
  }

  /// Convert xy to latlon.
  ///
  /// ### Arguments
  ///
  /// - `x` - width.
  /// - `y` - height.
  ///
  /// ### Returns
  ///
  /// - `lat` - lat.
  /// - `lon` - lon.
  pub fn from_xy(&self, x: f64, y: f64) -> (f64, f64) {
    assert!(
      x <= self.width as f64,
      format!("The maximum width is {}. out of range: {}", self.width, x)
    );
    assert!(
      y <= self.height as f64,
      format!("The maximum height is {}. out of range: {}", self.height, y)
    );

    (
      parse_lat(self.start_lat + (x * self.zoom_level)),
      parse_lon(self.start_lon + (y * self.zoom_level)),
    )
  }

  /// Convert latlon to xyz.
  ///
  /// ### Arguments
  ///
  /// - `lat` - lat.
  /// - `lon` - lon.
  ///
  /// ### Returns
  ///
  /// - `x` - width.
  /// - `y` - height.
  pub fn from_latlon(&self, lat: f64, lon: f64) -> (f64, f64) {
    let distance_x = mod_positive(lat - self.start_lat, 180f64);
    let distance_y = mod_positive(lon - self.start_lon, 360f64);

    let x = distance_x / self.zoom_level;
    let y = distance_y / self.zoom_level;

    assert!(
      x <= self.width as f64,
      format!("The maximum width is {}. out of range: {}", self.width, x)
    );
    assert!(
      y <= self.height as f64,
      format!("The maximum height is {}. out of range: {}", self.height, y)
    );

    (x, y)
  }
}

/// Parse to a range of lat.
///
/// ### Arguments
///
/// - `lat` - lat.
///
/// ### Returns
///
/// - f64 - parsed lat location. -90 <= x <= 90
fn parse_lat(lat: f64) -> f64 {
  mod_positive(lat + 90f64, 180f64) - 90f64
}

/// Parse to a range of lon.
///
/// ### Arguments
///
/// - `lon` - lon.
///
/// ### Returns
///
/// - f64 - parsed lon location. -180 <= x <= 180
fn parse_lon(lon: f64) -> f64 {
  mod_positive(lon + 180f64, 360f64) - 180f64
}

/// Calculate the modulo operation.
/// example:
///  - 10 % 100 = 10
///  - 200 % 100 = 0
///  - -80 % 100 = 20
///
/// ### Arguments
///
/// - `a` - numerator
/// - `b` - denominator
///
/// ### Returns
///
/// - f64 - answer
fn mod_positive(a: f64, b: f64) -> f64 {
  (a).rem_euclid(b)
}
