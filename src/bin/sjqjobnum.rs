use std::{
    io::{Read, Write},
    net::TcpStream,
};

use clap::Parser;
use env_logger::Env;
use eyre::Context;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser, long, short)]
    pub server_addr: Option<String>,
}
fn main() -> Result<(), eyre::Report> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Cli::try_parse()?;

    let addr = args.server_addr.unwrap_or("::1:5233".to_string());
    let mut stream = TcpStream::connect(addr).wrap_err("Failed to connect to server")?;
    // send 1 to indicate that we are querying the number of jobs
    stream
        .write_all(&1u32.to_le_bytes())
        .wrap_err("Failed to send request type")?;
    let mut buf = [0u8; 4];
    stream
        .read_exact(&mut buf)
        .wrap_err("Failed to read response type")?;
    let num_jobs = u32::from_le_bytes(buf);
    log::info!("number of queueing jobs: {}", num_jobs);
    stream
        .read_exact(&mut buf)
        .wrap_err("Failed to read response type")?;
    let num_running = u32::from_le_bytes(buf);
    log::info!("number of running jobs: {}", num_running);

    Ok(())
}
