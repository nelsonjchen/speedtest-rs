use std::num::Float;

struct SpeedTestConfig;

pub struct EarthLocation {
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

pub fn compute_distance(origin: EarthLocation, destination: EarthLocation) -> f32 {
    let radius: f32 = 6371.0;
    let d_lat = (origin.latitude - destination.latitude).to_radians();
    let d_long = (origin.longitude - destination.longitude).to_radians();
    let a = (d_lat/2.0).sin()
        * (d_lat/2.0).sin()
        + origin.latitude.to_radians().cos()
        * destination.latitude.to_radians().cos()
        * (d_long/2.0).sin()
        * (d_long/2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let d = radius * c;
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::Float;

    #[test]
    fn test_distance() {
        let origin = EarthLocation { latitude: 32.9545, longitude: -117.2333};
        let destination = EarthLocation { latitude: 70.0733, longitude: 29.7497};
        let distance = compute_distance(origin, destination);
        let diff = (distance - 8255.1).abs();
        println!("distance: {} diff: {}", distance, diff);
        assert!(diff < 0.2);
    }
}
