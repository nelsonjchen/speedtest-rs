struct SpeedTestConfig;

pub fn speedtest() {
    println!("Starting Speed Test");
    let config = get_config();
}

fn get_config() -> SpeedTestConfig {
    SpeedTestConfig
}

#[cfg(test)]
mod tests {
    use super::*;
}
