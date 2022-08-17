use std::{io::Write, net::TcpStream};

use clap::Parser;
use env_logger::Env;
use eyre::Context;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// the scripts to be sent to the job runner
    #[clap(value_parser)]
    pub scripts: Vec<String>,
}
fn sending_jobs(jobs: &[impl AsRef<str>]) -> Result<(), eyre::Report> {
    log::info!("connecting to server: {}", "localhost:5233");
    let mut tcp_stream =
        TcpStream::connect("127.0.0.1:5233").wrap_err("Failed to connect to server")?;
    log::info!("connected to server");
    for i in jobs {
        let i = i.as_ref();
        let len = (i.len() as u32).to_le_bytes();
        let data = i.as_bytes();
        log::info!("sending job {}", i);
        tcp_stream
            .write_all(&len)
            .wrap_err("failed to write job length")?;
        tcp_stream
            .write_all(&data)
            .wrap_err("failed to write job")?;
        log::info!("sent job {}", i);
    }
    Ok(())
}
fn main() -> Result<(), eyre::Report> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Cli::try_parse()?;
    if args.scripts.is_empty() {
        return Err(eyre::Error::msg("No script provided"));
    }
    sending_jobs(&args.scripts)
}
