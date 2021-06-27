use anyhow::Result;
use argh::FromArgs;
use std::fs::File;

#[derive(FromArgs)]
/// Probe URLs for their status code in high speed.
pub struct Settings {
    /// only print URLs which cannot be reached
    #[argh(switch, short = 'q')]
    pub quiet: bool,

    /// number of simultaneous URLs to process
    #[argh(option, default = "30")]
    pub threads: usize,

    /// seconds to wait before giving up a URL
    #[argh(option, default = "5")]
    pub timeout: u64,

    /// the User-Agent header to be used for the requests
    #[argh(option, default = "String::from(\"urlprobe\")")]
    pub useragent: String,

    /// input file that contains a list of URLs
    #[argh(positional, from_str_fn(validate_input))]
    pub input: Option<File>,
}

fn validate_input(gadget: &str) -> Result<File, String> {
    match File::open(gadget) {
        Ok(f) => Ok(f),
        Err(_) => Err("Unable to open specifed file".to_string()),
    }
}
