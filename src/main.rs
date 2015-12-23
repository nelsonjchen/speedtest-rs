extern crate speedtest_rs;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

use clap::{App, Arg};
// use speedtest_rs::speedtest::run_speedtest;
use speedtest_rs::speedtest;
use std::io::{self, Write};

#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();
    let matches = App::new("speedtest-rs")
                      .version(&crate_version!()[..])
                      .about("Command line interface for testing internet bandwidth using \
                              speedtest.net.")
                      .arg(Arg::with_name("list")
                               .long("list")
                               .help("Display a list of speedtest.net servers sorted by distance"))
                      .arg(Arg::with_name("share")
                               .long("share")
                               .help("Generate and provide a URL to the speedtest.net share \
                                      results image"))
                      .arg(Arg::with_name("bytes")
                               .long("bytes")
                               .help("Display values in bytes instead of bits."))
                      .arg(Arg::with_name("simple")
                               .long("simple")
                               .help("Suppress verbose output, only show basic informatio"))
                      .get_matches();

    if !matches.is_present("simple") {
        println!("Retrieving speedtest.net configuration...");
    }
    let config = speedtest::get_configuration().unwrap();
    if !matches.is_present("simple") {
        println!("Retrieving speedtest.net server list...");
    }
    let server_list = speedtest::get_server_list_with_config(Some(&config)).unwrap();
    let mut server_list_sorted = server_list.servers_sorted_by_distance(&config);

    if matches.is_present("list") {
        for server in server_list_sorted {
            println!("{:4}) {} ({}, {}) [{:.2} km]",
             server.id,
             server.sponsor,
             server.name,
             server.country,
             server.distance.unwrap(),
         );
        }
        return;
    }
    if !matches.is_present("simple") {
        println!("Testing from {} ({})...", config.isp, config.ip);
        println!("Selecting best server based on latency...");
    }
    info!("Five Closest Servers");
    server_list_sorted.truncate(5);
    for server in &server_list_sorted {
        info!("Close Server: {:?}", server);
    }
    let latecy_test_result = speedtest::get_best_server_based_on_latency(&server_list_sorted[..])
                                 .unwrap();
    if !matches.is_present("simple") {
        println!("Hosted by {} ({}) [{:.2} km]: {}.{} ms",
             latecy_test_result.server.sponsor,
             latecy_test_result.server.name,
             latecy_test_result.server.distance.unwrap(),
             latecy_test_result.latency.num_milliseconds(),
             latecy_test_result.latency.num_microseconds().unwrap() % 1000,
         );
    } else {
        println!("Ping: {}.{} ms",
            latecy_test_result.latency.num_milliseconds(),
            latecy_test_result.latency.num_microseconds().unwrap() % 1000,
        );
    }
    let best_server = latecy_test_result.server;

    let download_measurement;

    if !matches.is_present("simple") {
        print!("Testing download speed");
        download_measurement = speedtest::test_download_with_progress(best_server, print_dot)
                                   .unwrap();
        println!("");
    } else {
        download_measurement = speedtest::test_download_with_progress(best_server, do_nothing)
                                   .unwrap();
    }

    if matches.is_present("bytes") {
        println!("Download: {:.2} Mbyte/s",
                 (download_measurement.size / 800) as f32 /
                 download_measurement.duration.num_milliseconds() as f32);
    } else {
        println!("Download: {:.2} Mbit/s",
                 (download_measurement.size / 100) as f32 /
                 download_measurement.duration.num_milliseconds() as f32);
    }

    let upload_measurement;

    if !matches.is_present("simple") {
        print!("Testing upload speed");
        upload_measurement = speedtest::test_upload_with_progress(best_server, print_dot).unwrap();
        println!("");
    } else {
        upload_measurement = speedtest::test_upload_with_progress(best_server, do_nothing).unwrap();
    }

    if matches.is_present("bytes") {
        println!("Upload: {:.2} Mbyte/s",
                 (upload_measurement.size / 800) as f32 /
                 upload_measurement.duration.num_milliseconds() as f32);
    } else {
        println!("Upload: {:.2} Mbyte/s",
                 (upload_measurement.size / 100) as f32 /
                 upload_measurement.duration.num_milliseconds() as f32);
    }

    if matches.is_present("share") {
        let request = speedtest::ShareUrlRequest {
            download_measurement: &download_measurement,
            upload_measurement: &upload_measurement,
            server: &best_server,
            latency_measurement: &latecy_test_result,
        };
        info!("Share Request {:?}", request);
        println!("Share results: {}", speedtest::get_share_url(&request));
    }
}

fn print_dot() {
    print!(".");
    io::stdout().flush().unwrap();
}

fn do_nothing() {}
