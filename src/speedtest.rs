use xml::{Element, Parser, ElementBuilder};

pub struct SpeedTestConfig{
    ip: String,
    lat: String,
    lon: String,
    isp: String,
}


pub fn run_speedtest() {
    println!("TODO");
}

pub fn parse_config_xml(config_xml: String) -> SpeedTestConfig {
    let elem: Element = config_xml.parse().unwrap();
    SpeedTestConfig{
        ip: "127.0.0.1".to_string(),
        lat: "37.4192".to_string(),
        lon: "-122.0574".to_string(),
        isp: "Time Warner Cable".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::string::String;

    #[test]
    fn test_parse_config_xml() {
        // TODO: File Reading with try!(File::Open()). Not sure why it considers
        // the result of File::Open() a ().
        let config_xml_string: String = r#"
        <?xml version="1.0" encoding="UTF-8"?>
<settings>
	<client ip="174.79.12.26" lat="32.9954" lon="-117.0753" isp="Cox Communications" isprating="3.1" rating="0" ispdlavg="18259" ispulavg="5021" loggedin="0"/>
</settings>
        "#.to_string();
        parse_config_xml(config_xml_string);
    }
}
