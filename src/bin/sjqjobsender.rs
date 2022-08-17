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
    #[clap(value_parser, long, short)]
    pub server_addr: Option<String>,
}
fn sending_jobs(addr: &str, jobs: &[impl AsRef<str>]) -> Result<(), eyre::Report> {
    log::info!("connecting to server: {}", addr);
    let mut tcp_stream = TcpStream::connect(addr).wrap_err("Failed to connect to server")?;
    log::info!("connected to server");
    // write 0 to indicate that we are sending jobs
    tcp_stream
        .write_all(&0u32.to_le_bytes())
        .wrap_err("Failed to send request type")?;
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
    let addr = args.server_addr.unwrap_or("::1:5233".to_string());
    sending_jobs(&addr, &args.scripts)
}
