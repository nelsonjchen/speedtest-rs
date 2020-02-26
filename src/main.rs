use crate::speedtest_csv::SpeedTestCsvResult;
use chrono::Utc;
use clap::{crate_version, App, Arg};
use log::info;
use std::io::{self, Write};

mod distance;
mod error;
mod speedtest;
mod speedtest_csv;

fn main() -> Result<(), error::Error> {
    env_logger::init();

    let matches = App::new("speedtest-rs")
        .version(&crate_version!()[..])
        .about("Command line interface for testing internet bandwidth using speedtest.net.")
        .arg(
            Arg::with_name("list")
                .long("list")
                .help("Display a list of speedtest.net servers sorted by distance"),
        )
        .arg(
            Arg::with_name("share")
                .long("share")
                .help("Generate and provide an URL to the speedtest.net share results image"),
        )
        .arg(
            Arg::with_name("bytes")
                .long("bytes")
                .help("Display values in bytes instead of bits."),
        )
        .arg(
            Arg::with_name("simple")
                .long("simple")
                .help("Suppress verbose output, only show basic information"),
        )
        .arg(Arg::with_name("csv").long("csv").help("Output"))
        .arg(
            Arg::with_name("csv-header")
                .long("csv-header")
                .help("Output"),
        )
        .get_matches();

    // This appears to be purely informational.
    if matches.is_present("csv-header") {
        let results = speedtest_csv::SpeedTestCsvResult::default();

        println!("{}", results.header_serialize());
        return Ok(());
    }

    let machine_format = matches.is_present("csv") || matches.is_present("json");

    if !matches.is_present("simple") && !machine_format {
        println!("Retrieving speedtest.net configuration...");
    }
    let config = speedtest::get_configuration()?;
    if !matches.is_present("simple") && !machine_format {
        println!("Retrieving speedtest.net server list...");
    }
    let server_list = speedtest::get_server_list_with_config(Some(&config))?;
    let mut server_list_sorted = server_list.servers_sorted_by_distance(&config);

    if matches.is_present("list") {
        for server in server_list_sorted {
            println!(
                "{:4}) {} ({}, {}) [{}]",
                server.id,
                server.sponsor,
                server.name,
                server.country,
                server
                    .distance
                    .map_or("None".to_string(), |d| format!("{:.2} km", d)),
            );
        }
        return Ok(());
    }

    if !matches.is_present("simple") && !machine_format {
        println!("Testing from {} ({})...", config.isp, config.ip);
        println!("Selecting best server based on latency...");
    }

    info!("Five Closest Servers");
    server_list_sorted.truncate(5);
    for server in &server_list_sorted {
        info!("Close Server: {:?}", server);
    }
    let latency_test_result = speedtest::get_best_server_based_on_latency(&server_list_sorted[..])?;

    if !machine_format {
        if !matches.is_present("simple") {
            println!(
                "Hosted by {} ({}) [{:.2} km]: {}.{} ms",
                latency_test_result.server.sponsor,
                latency_test_result.server.name,
                latency_test_result
                    .server
                    .distance
                    .map_or("None".to_string(), |d| format!("{:.2} km", d)),
                latency_test_result.latency.num_milliseconds(),
                latency_test_result.latency.num_microseconds().unwrap_or(0) % 1000,
            );
        } else {
            println!(
                "Ping: {}.{} ms",
                latency_test_result.latency.num_milliseconds(),
                latency_test_result.latency.num_microseconds().unwrap_or(0) % 1000,
            );
        }
    }

    let best_server = latency_test_result.server;

    let download_measurement;

    if !matches.is_present("simple") && !machine_format {
        print!("Testing download speed");
        download_measurement = speedtest::test_download_with_progress(best_server, print_dot)?;
        println!();
    } else {
        download_measurement = speedtest::test_download_with_progress(best_server, || {})?;
    }

    if !machine_format {
        if matches.is_present("bytes") {
            println!(
                "Download: {:.2} Mbyte/s",
                ((download_measurement.kbps() / 8) as f32 / 1000.00)
            );
        } else {
            println!(
                "Download: {:.2} Mbit/s",
                (download_measurement.kbps()) as f32 / 1000.00
            );
        }
    }

    let upload_measurement;

    if !matches.is_present("simple") && !machine_format {
        print!("Testing upload speed");
        upload_measurement = speedtest::test_upload_with_progress(best_server, print_dot)?;
        println!();
    } else {
        upload_measurement = speedtest::test_upload_with_progress(best_server, || {})?;
    }

    if !machine_format {
        if matches.is_present("bytes") {
            println!(
                "Upload: {:.2} Mbyte/s",
                ((upload_measurement.kbps() / 8) as f32 / 1000.00)
            );
        } else {
            println!(
                "Upload: {:.2} Mbit/s",
                (upload_measurement.kbps() as f32 / 1000.00)
            );
        }
    }

    let speedtest_result = speedtest::SpeedTestResult {
        download_measurement: &download_measurement,
        upload_measurement: &upload_measurement,
        server: &best_server,
        latency_measurement: &latency_test_result,
    };

    if matches.is_present("csv") {
        let speedtest_csv_result = SpeedTestCsvResult {
            server_id: &best_server.id.to_string(),
            sponsor: &best_server.sponsor,
            server_name: &best_server.name,
            timestamp: &Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
            distance: &(latency_test_result
                .server
                .distance
                .map_or("".to_string(), |d| format!("{:.14}", d)))[..],
            ping: &format!(
                "{}.{}",
                latency_test_result.latency.num_milliseconds(),
                latency_test_result.latency.num_microseconds().unwrap_or(0) % 1000
            ),
            download: &(download_measurement.kbps() as f32 / 1000.00).to_string(),
            upload: &(upload_measurement.kbps() as f32 / 1000.00).to_string(),
            share: &if matches.is_present("share") {
                speedtest::get_share_url(&speedtest_result)?
            } else {
                "".to_string()
            },
            ip_address: &config.ip,
        };
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(io::stdout());
        wtr.serialize(speedtest_csv_result)?;
        wtr.flush()?;
        return Ok(());
    }

    if matches.is_present("share") && !machine_format {
        info!("Share Request {:?}", speedtest_result);
        println!(
            "Share results: {}",
            speedtest::get_share_url(&speedtest_result)?
        );
    }

    Ok(())
}

fn print_dot() {
    print!(".");
    io::stdout().flush().unwrap();
}
