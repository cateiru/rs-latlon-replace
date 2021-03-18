# Rs LatLon Replace

## What is this?

parse canvas x/y to lat/lon. and reverse is also possible.

example:

```text
canvas 1920x1080
┌──────────────────────────┐
│                          │
│                          │
│           X              │
│                          │
│                          │
│                          │
└──────────────────────────┘

X: location at (x, y)
↓↓↓
(lat, lon)
```

## Usage

Please write in `cargo.toml`.

```toml
[dependencies]
rs_latlon = { git = "https://github.com/yuto51942/rs-latlon-replace" }
```

## LICENSE

MIT
