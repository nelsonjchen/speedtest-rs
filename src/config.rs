#[cfg(test)]
mod tests {
    extern crate xml;
    use std::io::{File, BufferedReader};
    use self::xml::reader::EventReader;
    use self::xml::reader::events::*;

    #[test]
    fn read_xml() {
        let file = File::open(&Path::new("data/speedtest-config.php.xml")).unwrap();
        let reader = BufferedReader::new(file);

        let mut parser = EventReader::new(reader);
        for e in parser.events() {
            match e {
                XmlEvent::StartElement {name, attributes: _, namespace: _ } => {
                }
                _ => {}
            }
        }
    }
}
