use clap::{crate_authors, crate_description, crate_name, crate_version, load_yaml, value_t_or_exit};
use futures::channel::mpsc::channel;
use futures::{Stream, StreamExt};
use http::StatusCode;
use reqwest::Client;
use std::io::BufRead;
use std::process::exit;
use std::time::Duration;

fn stdin(threads: usize) -> impl Stream<Item = String> {
    let (mut tx, rx) = channel(threads);

    std::thread::spawn(move || {
        for line in std::io::stdin().lock().lines() {
            if let Ok(line) = line {
                loop {
                    let status = tx.try_send(line.to_owned());

                    match status {
                        Err(e) if e.is_full() => continue,
                        _ => break,
                    }
                }
            }
        }
    });

    rx
}

async fn process_url(url: String, client: &Client, quiet: bool) -> bool {
    let response = client.get(&url).send().await;

    match response {
        Ok(r) => match r.status() {
            StatusCode::OK => {
                if !quiet {
                    println!("{} - {}", r.status().as_str(), url);
                }
            }
            _ => {
                println!("{} - {}", r.status().as_str(), url);
                return false;
            }
        },
        Err(e) => {
            eprintln!("ERR - {} ({})", url, e);
            return false;
        }
    }

    true
}

#[tokio::main]
async fn process_urls(quiet: bool, threads: usize, timeout: u64, useragent: &str) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout))
        .user_agent(useragent)
        .build()
        .unwrap();

    let statuses = stdin(threads)
        .map(|url| {
            let client = &client;

            async move { process_url(url, client, quiet).await }
        })
        .buffer_unordered(threads);

    let success: bool = statuses.fold(true, |acc, s| async move { acc && s }).await;

    success
}

fn main() {
    let args_yaml = load_yaml!("args.yml");

    let matches = clap::App::from_yaml(args_yaml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let quiet = matches.is_present("quiet");
    let threads = value_t_or_exit!(matches.value_of("threads"), usize);
    let timeout = value_t_or_exit!(matches.value_of("timeout"), u64);
    let useragent = value_t_or_exit!(matches.value_of("useragent"), String);

    let success = process_urls(quiet, threads, timeout, &useragent);

    if !success {
        exit(1);
    }
}
