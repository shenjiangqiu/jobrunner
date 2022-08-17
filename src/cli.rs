use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// max number of cpus to use
    #[clap(value_parser = valid_cpu_count, short, long)]
    pub max_cpus: Option<usize>,
}
pub fn valid_cpu_count(input: &str) -> Result<usize, String> {
    let num = input
        .parse::<usize>()
        .map_err(|_| "Invalid number of cpus")?;
    match num {
        0 => Err("Invalid number of cpus of 0".to_string()),
        1..=200 => Ok(num),
        _ => Err(format!("Invalid number of cpus of {num}, too large")),
    }
}
