use std::f32::consts;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EarthLocation {
    pub latitude: f32,
    pub longitude: f32,
}

pub fn compute_distance(origin: &EarthLocation, destination: &EarthLocation) -> f32 {
    let radius: f32 = 6371.0;
    let d_lat = to_radians(origin.latitude - destination.latitude);
    let d_long = to_radians(origin.longitude - destination.longitude);
    let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
        + to_radians(origin.latitude).cos()
            * to_radians(destination.latitude).cos()
            * (d_long / 2.0).sin()
            * (d_long / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    radius * c
}

fn to_radians(degree: f32) -> f32 {
    let value: f32 = consts::PI;
    degree * (value / 180.0f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let origin = EarthLocation {
            latitude: 32.9545,
            longitude: -117.2333,
        };
        let destination = EarthLocation {
            latitude: 70.0733,
            longitude: 29.7497,
        };
        let distance = compute_distance(&origin, &destination);
        let diff = (distance - 8255.1).abs();
        println!("distance: {} diff: {}", distance, diff);
        assert!(diff < 0.2);
    }
}
