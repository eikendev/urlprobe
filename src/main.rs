mod settings;

use futures::channel::mpsc::{Sender, channel};
use futures::{Stream, StreamExt};
use reqwest::{Client, StatusCode};
use settings::Settings;
use std::fs::File;
use std::io::BufRead;
use std::process::exit;
use std::time::Duration;

fn push_lines(reader: &mut dyn BufRead, sendto: &mut Sender<String>) {
    for line in reader.lines().map_while(Result::ok) {
        loop {
            let status = sendto.try_send(line.to_owned());

            match status {
                Err(e) if e.is_full() => continue,
                _ => break,
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
            eprintln!("ERR - {url} ({e})");
            return false;
        }
    }

    true
}

#[tokio::main]
async fn process_urls(s: settings::Settings) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(s.timeout))
        .user_agent(s.useragent)
        .build()
        .unwrap();

    let quiet = s.quiet;
    let statuses = get_lines(s.input, s.threads)
        .map(|url| {
            let client = &client;

            async move { process_url(url, client, quiet).await }
        })
        .buffer_unordered(s.threads);

    let success: bool = statuses.fold(true, |acc, s| async move { acc && s }).await;

    success
}

fn main() {
    human_panic::setup_panic!();

    let settings: Settings = argh::from_env();

    let success = process_urls(settings);

    if !success {
        exit(1);
    }
}
