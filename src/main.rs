use clap::{crate_authors, crate_description, crate_name, crate_version, load_yaml, value_t_or_exit};
use futures::channel::mpsc::{channel, Sender};
use futures::{Stream, StreamExt};
use http::StatusCode;
use reqwest::Client;
use std::fs::File;
use std::io::BufRead;
use std::process::exit;
use std::time::Duration;

fn push_lines(reader: &mut dyn BufRead, sendto: &mut Sender<String>) {
    for line in reader.lines() {
        if let Ok(line) = line {
            loop {
                let status = sendto.try_send(line.to_owned());

                match status {
                    Err(e) if e.is_full() => continue,
                    _ => break,
                }
            }
        }
    }
}

fn get_lines(f: Option<File>, threads: usize) -> impl Stream<Item = String> {
    let (mut tx, rx) = channel(threads);

    std::thread::spawn(move || match f {
        None => {
            let stdin = std::io::stdin();
            let mut reader = &mut stdin.lock() as &mut dyn BufRead;
            push_lines(&mut reader, &mut tx);
        }
        Some(f) => {
            let mut reader = &mut std::io::BufReader::new(f) as &mut dyn BufRead;
            push_lines(&mut reader, &mut tx);
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
async fn process_urls(quiet: bool, threads: usize, timeout: u64, useragent: &str, file: Option<File>) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout))
        .user_agent(useragent)
        .build()
        .unwrap();

    let statuses = get_lines(file, threads)
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

    let file = value_t_or_exit!(matches.value_of("FILE"), String);
    let file: Option<File> = match file {
        _ if file == "-" => None,
        _ => match File::open(file) {
            Ok(f) => Some(f),
            Err(e) => {
                eprintln!("Cannot read specified file: {}", e);
                exit(1)
            }
        },
    };

    let success = process_urls(quiet, threads, timeout, &useragent, file);

    if !success {
        exit(1);
    }
}
