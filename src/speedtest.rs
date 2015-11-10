use std::io::Read;
use std::path::Path;
use std::cmp::Ordering::Less;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::sync_channel;
use std::thread;
use hyper::Client;
use hyper::header::{Connection, UserAgent};
use hyper::client::Response;
use time::{now, Duration};
use xml::reader::EventReader;
use xml::reader::events::XmlEvent::*;
use distance::{EarthLocation, compute_distance};
use error::Error;

#[derive(Debug)]
pub struct ParseError(String);

pub struct SpeedTestConfig {
    ip: String,
    lat: String,
    lon: String,
    isp: String,
}

impl SpeedTestConfig {
    fn new<R: Read>(parser: &mut EventReader<R>) -> ::Result<SpeedTestConfig> {
        let mut ip: Option<String> = None;
        let mut lat: Option<String> = None;
        let mut lon: Option<String> = None;
        let mut isp: Option<String> = None;
        for event in parser.events() {
            match event {
                StartElement { ref name, ref attributes, ..} => {
                    match name.local_name.as_ref() {
                        "client" => {
                            for attribute in attributes {
                                match attribute.name.local_name.as_ref() {
                                    "ip" => {
                                        ip = Some(attribute.value.clone());
                                    }
                                    "lat" => {
                                        match attribute.value.parse::<f32>() {
                                            Ok(_) => {
                                                lat = Some(attribute.value.clone());
                                            }
                                            _ => {
                                                lat = None;
                                            }
                                        }
                                    }
                                    "lon" => {
                                        match attribute.value.parse::<f32>() {
                                            Ok(_) => {
                                                lon = Some(attribute.value.clone());
                                            }
                                            _ => {
                                                lon = None;
                                            }
                                        }
                                    }
                                    "isp" => {
                                        isp = Some(attribute.value.clone());
                                    }
                                    _ => {}
                                }
                            }
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        match (ip, lat, lon, isp) {
            (Some(ip), Some(lat), Some(lon), Some(isp)) => {
                Ok(SpeedTestConfig {
                    ip: ip,
                    lat: lat,
                    lon: lon,
                    isp: isp,
                })
            }
            _ => {
                Err(Error::ConfigParseError)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpeedTestServer {
    pub country: String,
    host: String,
    id: String,
    lat: String,
    lon: String,
    name: String,
    sponsor: String,
    url: String,
    url2: String,
}

pub struct SpeedTestServersConfig {
    servers: Vec<SpeedTestServer>,
}


impl SpeedTestServersConfig {
    fn new<R: Read>(parser: &mut EventReader<R>) -> ::Result<SpeedTestServersConfig> {
        let mut servers: Vec<SpeedTestServer> = Vec::new();

        for event in parser.events() {
            match event {
                StartElement { ref name, ref attributes, ..} => {
                    match name.local_name.as_ref() {
                        "server" => {
                            let mut country: Option<String> = None;
                            let mut host: Option<String> = None;
                            let mut id: Option<String> = None;
                            let mut lat: Option<String> = None;
                            let mut lon: Option<String> = None;
                            let mut name: Option<String> = None;
                            let mut sponsor: Option<String> = None;
                            let mut url: Option<String> = None;
                            let mut url2: Option<String> = None;
                            for attribute in attributes {
                                match attribute.name.local_name.as_ref() {
                                    "country" => {
                                        country = Some(attribute.value.clone());
                                    }
                                    "host" => {
                                        host = Some(attribute.value.clone());
                                    }
                                    "id" => {
                                        id = Some(attribute.value.clone());
                                    }
                                    "lat" => {
                                        lat = Some(attribute.value.clone());
                                    }
                                    "lon" => {
                                        lon = Some(attribute.value.clone());
                                    }
                                    "name" => {
                                        name = Some(attribute.value.clone());
                                    }
                                    "sponsor" => {
                                        sponsor = Some(attribute.value.clone());
                                    }
                                    "url" => {
                                        url = Some(attribute.value.clone());
                                    }
                                    "url2" => {
                                        url2 = Some(attribute.value.clone());
                                    }
                                    _ => {
                                        // eh?
                                    }
                                }
                            }
                            match (country, host, id, lat, lon, name, sponsor, url, url2) {
                                (Some(country),
                                 Some(host),
                                 Some(id),
                                 Some(lat),
                                 Some(lon),
                                 Some(name),
                                 Some(sponsor),
                                 Some(url),
                                 Some(url2)) => {
                                    servers.push(SpeedTestServer {
                                        country: country,
                                        host: host,
                                        id: id,
                                        lat: lat,
                                        lon: lon,
                                        name: name,
                                        sponsor: sponsor,
                                        url: url,
                                        url2: url2,
                                    });
                                }
                                _ => {
                                    // eh
                                }
                            }
                        }
                        _ => {
                            // I don't care about other tags.
                        }
                    }
                }
                _ => {
                    // not using other parts of the xml library right now.
                }
            }
        }
        Ok(SpeedTestServersConfig { servers: servers })
    }


    pub fn servers_sorted_by_distance(&self,
                                      config: &SpeedTestConfig)
                                      -> Option<Vec<SpeedTestServer>> {
        let location = EarthLocation {
            latitude: config.lat.parse::<f32>().unwrap(),
            longitude: config.lon.parse::<f32>().unwrap(),
        };
        let mut sorted_servers = self.servers.clone();
        sorted_servers.sort_by(|a, b| {
            let a_location = EarthLocation {
                latitude: a.lat.parse::<f32>().unwrap(),
                longitude: a.lon.parse::<f32>().unwrap(),
            };
            let b_location = EarthLocation {
                latitude: b.lat.parse::<f32>().unwrap(),
                longitude: b.lon.parse::<f32>().unwrap(),
            };
            let a_distance = compute_distance(&location, &a_location);
            let b_distance = compute_distance(&location, &b_location);
            a_distance.partial_cmp(&b_distance).unwrap_or(Less)
        });
        Some(sorted_servers)
    }
}

pub fn download_configuration() -> ::Result<Response> {
    info!("Downloading Configuration from speedtest.net");
    let client = Client::new();
    // Creating an outgoing request.
    let res = try!(client.get("http://www.speedtest.net/speedtest-config.php")
                         .header(Connection::close())
                         .header(UserAgent("hyper/speedtest-rust 0.01".to_owned()))
                         .send());
    Ok(res)
}

pub fn get_configuration() -> ::Result<SpeedTestConfig> {
    let config_body = try!(download_configuration());
    info!("Parsing Configuration");
    let mut config_parser = EventReader::new(config_body);
    let spt_config = SpeedTestConfig::new(&mut config_parser);
    info!("Parsed Configuration");
    spt_config
}

pub fn download_server_list() -> ::Result<Response> {
    info!("Download Server List");
    let client = Client::new();
    let server_res = try!(client.get("http://www.speedtest.net/speedtest-servers-static.php")
                                    .header(Connection::close())
                                    .header(UserAgent("hyper/speedtest-rust 0.01".to_owned()))
                                    .send());
    info!("Downloaded Server List");
    Ok(server_res)
}

pub fn get_server_list() -> ::Result<SpeedTestServersConfig> {
    let config_body = try!(download_server_list());
    info!("Parsing Configuration");
    let mut config_parser = EventReader::new(config_body);
    let spt_config = SpeedTestServersConfig::new(&mut config_parser);
    info!("Parsed Configuration");
    spt_config
}

pub fn run_speedtest() {
    let client = Client::new();
    let spt_config = get_configuration().unwrap();
    info!("IP: {}, ISP: {}", spt_config.ip, spt_config.isp);
    let spt_server_config = get_server_list().unwrap();

    let servers_sorted_by_distance = spt_server_config.servers_sorted_by_distance(&spt_config)
                                                      .unwrap();
    info!("Five Closest Servers");
    let five_closest_servers = &servers_sorted_by_distance[0..5];
    for server in five_closest_servers {
        info!("Close Server: {:?}", server);
    }

    info!("Testing for fastest server");
    let mut fastest_server = None;
    let mut fastest_latency = Duration::max_value();
    for server in five_closest_servers {
        let path = Path::new(&server.url);
        let latency_path = format!("{}/latency.txt", path.parent().unwrap().display());
        info!("Downloading: {:?}", latency_path);
        let start_time = now();
        let mut res = client.get(&latency_path)
                            .header(Connection::close())
                            .header(UserAgent("hyper/speedtest-rust 0.01".to_owned()))
                            .send()
                            .unwrap();
        let mut server_body: Vec<u8> = vec![];
        res.read_to_end(&mut server_body).unwrap();
        let latency = now() - start_time;
        info!("It took {} ms", latency.num_milliseconds());

        if latency < fastest_latency {
            fastest_server = Some(server);
            fastest_latency = latency;
        }
    }
    info!("Fastest Server @ {}ms : {:?}",
          fastest_latency.num_milliseconds(),
          fastest_server);

    test_download(&fastest_server.unwrap());
    test_upload(&fastest_server.unwrap());
}

fn test_download(server: &SpeedTestServer) {
    info!("Testing Download speed");
    let root_path = Path::new(&server.url).parent().unwrap();
    debug!("Root path is: {}", root_path.display());
    let start_time = Arc::new(now());
    let total_size;

    let sizes = [350, 500, 750, 1000, 1500, 2000, 2500, 3000, 3500, 4000];
    let len_sizes = sizes.len();
    let complete = Arc::new(RwLock::new(vec![]));
    let (tx, rx) = sync_channel(6);
    let root_path = root_path.to_path_buf();
    let prod_thread = thread::spawn(move || {
        for size in &sizes {
            let size = size.clone();
            let root_path = root_path.clone();
            let thread = thread::spawn(move || {
                let path = root_path.to_path_buf()
                                    .join(format!("random{0}x{0}.jpg", size));
                let client = Client::new();
                let mut res = client.get(path.to_str().unwrap())
                                    .header(Connection::close())
                                    .header(UserAgent("hyper/speedtest-rust 0.01".to_owned()))
                                    .send()
                                    .unwrap();
                let mut buffer = [0; 10240];
                let mut size: usize = 0;
                loop {
                    match res.read(&mut buffer) {
                        Ok(0) => {
                            break;
                        }
                        Ok(n) => {
                            size = size + n
                        }
                        _ => {
                            panic!("Something has gone wrong.")
                        }
                    }
                }
                info!("Done {}, {}", path.display(), size);
                size
            });
            tx.send(thread).unwrap();
        }
    });

    let cons_complete = complete.clone();

    let cons_thread = thread::spawn(move || {
        while cons_complete.read().unwrap().len() < len_sizes {
            let thread = rx.recv().unwrap();
            let mut complete = (*cons_complete).write().unwrap();
            complete.push(thread.join().unwrap());
        }
    });
    prod_thread.join().unwrap();
    cons_thread.join().unwrap();
    total_size = (*complete).read().unwrap().iter().fold(0, |val, i| val + i);
    let latency = now() - *start_time;
    info!("It took {} ms to download {} bytes",
          latency.num_milliseconds(),
          total_size);
    let bps = total_size as i64 / (latency.num_milliseconds() / 1000);
    info!("{} bytes per second", bps);
}

fn test_upload(server: &SpeedTestServer) {
    info!("Testing Upload");
    let upload_path = Path::new(&server.url).to_path_buf().clone();
    let total_size: usize;
    let start_time = Arc::new(now());
    let small_sizes = [250000; 25];
    let large_sizes = [500000; 25];
    let sizes = small_sizes.iter().chain(large_sizes.iter()).cloned().collect::<Vec<usize>>();
    let len_sizes = sizes.len();
    let complete = Arc::new(RwLock::new(vec![]));
    let (tx, rx) = sync_channel(6);

    let thread_start_time = start_time.clone();
    let prod_thread = thread::spawn(move || {
        for size in &sizes {
            let size = size.clone();
            let path = upload_path.to_path_buf().clone();
            let start_time = thread_start_time.clone();
            let thread = thread::spawn(move || {
                info!("Uploading {} to {}", size, path.display());
                if (now() - *start_time) > Duration::seconds(10) {
                    info!("Canceled Uploading {} of {}", size, path.display());
                    return 0;
                }
                let body_loop = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().cycle();
                let client = Client::new();
                let body = format!("content1={}", body_loop.take(size).collect::<String>());
                let mut res = client.post(path.to_str().unwrap())
                                    .body(body.as_bytes())
                                    .header(Connection::close())
                                    .header(UserAgent("hyper/speedtest-rust 0.01".to_owned()))
                                    .send()
                                    .unwrap();
                let mut buffer = [0; 10240];
                loop {
                    match res.read(&mut buffer) {
                        Ok(0) => {
                            break;
                        }
                        Ok(_) => {}
                        _ => {
                            panic!("Something has gone wrong.")
                        }
                    }
                }
                info!("Done {}, {}", path.display(), size);
                size
            });
            tx.send(thread).unwrap();
        }
    });

    let cons_complete = complete.clone();

    let cons_thread = thread::spawn(move || {
        while cons_complete.read().unwrap().len() < len_sizes {
            let thread = rx.recv().unwrap();
            let mut complete = (*cons_complete).write().unwrap();
            complete.push(thread.join().unwrap());
        }
    });

    prod_thread.join().unwrap();
    cons_thread.join().unwrap();
    total_size = (*complete).read().unwrap().iter().fold(0, |val, i| val + i);
    let latency = now() - *start_time;
    info!("It took {} ms to upload {} bytes",
          latency.num_milliseconds(),
          total_size);
    info!("{} bytes per second",
          total_size as i64 / (latency.num_milliseconds() / 1000));
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml::reader::EventReader;

    #[test]
    fn test_parse_config_xml() {
        let mut parser =
            EventReader::new(include_bytes!("../tests/config/config.php.xml") as &[u8]);
        let config = SpeedTestConfig::new(&mut parser).unwrap();
        assert_eq!("174.79.12.26", config.ip);
        assert_eq!("32.9954", config.lat);
        assert_eq!("-117.0753", config.lon);
        assert_eq!("Cox Communications", config.isp);
    }

    #[test]
    fn test_parse_speedtest_servers_xml() {
        let mut parser =
            EventReader::new(include_bytes!("../tests/confi\
                                             g/stripped-ser\
                                             vers-static.\
                                             php.xml") as &[u8]);
        let spt_server_config = SpeedTestServersConfig::new(&mut parser).unwrap();
        assert!(spt_server_config.servers.len() > 5);
        let server = spt_server_config.servers.get(1).unwrap();
        assert!(server.url2.len() > 0);
        assert!(server.country.len() > 0);
    }

    #[test]
    fn test_fastest_server() {
        let spt_config = SpeedTestConfig {
            ip: "127.0.0.1".to_string(),
            lat: "32.9954".to_string(),
            lon: "-117.0753".to_string(),
            isp: "xxxfinity".to_string(),
        };
        let mut parser =
            EventReader::new(include_bytes!("../tests/confi\
                                             g/geo-test-ser\
                                             vers-static.\
                                             php.xml") as &[u8]);
        let config = SpeedTestServersConfig::new(&mut parser).unwrap();
        let closest_server = &config.servers_sorted_by_distance(&spt_config).unwrap()[0];
        assert_eq!("Los Angeles, CA", closest_server.name);
    }
}
