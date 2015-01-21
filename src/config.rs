#[cfg(test)]
mod tests {
    extern crate xml;

    use std::io::{File, BufferedReader};

    #[test]
    fn read_xml() {
        let file = File::open(&Path::new("data/speedtest-config.php")).unwrap();
        let mut reader = BufferedReader::new(file);

        println!("{}", &mut reader.read_to_string().ok().expect("not a string").as_slice());
    }
}
