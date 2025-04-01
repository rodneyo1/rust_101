pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    // 1 km/h = 1000m / 3600s = 5/18 m/s
    km_h * 5.0 / 18.0
}