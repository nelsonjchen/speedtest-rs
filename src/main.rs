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
                               .help("Display a list of speedtest.net servers sorted by distance"))
                      .get_matches();

    println!("Retrieving speedtest.net configuration...", );
    let config = speedtest::get_configuration().unwrap();
    println!("Retrieving speedtest.net server list...", );
    let server_list = speedtest::get_server_list_with_config(Some(&config)).unwrap();
    let server_list_sorted = server_list.servers_sorted_by_distance(&config);

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
    println!("Testing from {} ({})...", config.isp, config.ip);
    println!("Selecting best server based on latency...");
    info!("Five Closest Servers");
    let five_closest_servers = &server_list_sorted[0..5];
    for server in five_closest_servers {
        info!("Close Server: {:?}", server);
    }
    let latecy_test_result = speedtest::get_best_server_based_on_latency(five_closest_servers)
                                 .unwrap();
    println!("Hosted by {} ({}) [{:.2} km]: {}.{} ms",
             latecy_test_result.server.sponsor,
             latecy_test_result.server.name,
             latecy_test_result.server.distance.unwrap(),
             latecy_test_result.latency.num_milliseconds(),
             latecy_test_result.latency.num_microseconds().unwrap() % 1000,
         );
    let best_server = latecy_test_result.server;
    print!("Testing download speed");
    speedtest::test_download_with_progress(best_server, print_dot);
    println!("");
}

fn print_dot() {
    print!(".");
    io::stdout().flush().unwrap();
}
