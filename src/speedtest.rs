use std::num::Float;

struct SpeedTestConfig;

struct EarthLocation {
    latitude: f32,
    longitude: f32
}

pub fn speedtest() {
    println!("Starting Speed Test");
    let config = get_config();
}

fn get_config() -> SpeedTestConfig {
    SpeedTestConfig
}

fn fast_earth_distance(origin: EarthLocation, destination: EarthLocation) -> f32 {
    let radius: f32 = 6371.0;
    let d_lat = (origin.latitude - destination.latitude).to_radians();
    let d_long = (origin.latitude - destination.latitude).to_radians();
    let a = (d_lat/2.0).sin()
        * (d_lat/2.0).sin()
        + origin.latitude.to_radians().cos()
        * destination.latitude.to_radians().cos()
        * (d_long/2.0).to_radians().sin()
        * (d_long/2.0).to_radians().sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let d = radius * c;
    d
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_distance() {

    }
}
