use std::{io::Read, net::TcpListener, process, thread};

use clap::Parser;
use env_logger::Env;
use sjqjobrunner::cli::Cli;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Cli::parse();
    println!("{:?}", args);
    let num_cpus = args.max_cpus.unwrap_or(num_cpus::get());
    log::info!("Running with {} cpus", num_cpus);
    let (tx, rx) = crossbeam_channel::unbounded();
    let mut thread_handles = vec![];
    for i in 0..num_cpus {
        log::info!("building thread {i}");
        let rx = rx.clone();
        thread_handles.push(thread::spawn(move || {
            log::info!("thread {i} starting to work");

            while let Ok(msg) = rx.recv() {
                log::info!("thread {i} Running job {}", msg);
                let output = execute_script(msg);
                log::info!("thread {i} Finished job with output {}", output);
            }
        }));
    }
    let listener = TcpListener::bind("0.0.0.0:5233").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        log::info!("New connection from {}", stream.peer_addr().unwrap());
        let mut size: [u8; 4] = [0, 0, 0, 0];
        while let Ok(_) = stream.read_exact(&mut size) {
            let size = u32::from_le_bytes(size);
            let mut data = vec![0; size as usize];
            if let Err(_) = stream.read_exact(&mut data) {
                log::error!("Failed to read data from stream");
                break;
            }
            let data = String::from_utf8(data).unwrap();
            println!("received data: {:?}", data);
            tx.send(data).unwrap();
        }
    }
    drop(tx);
    for thread in thread_handles {
        thread.join().unwrap();
    }
}

pub fn execute_script(script: String) -> String {
    println!("{}", script);
    let ouput = process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8_lossy(&ouput.stdout);
    let stderr = String::from_utf8_lossy(&ouput.stderr);
    format!(
        "stdout: {}, stderr: {}",
        stdout.to_string(),
        stderr.to_string(),
    )
}
