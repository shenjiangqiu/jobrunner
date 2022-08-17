use std::process::Command;
pub mod cli;
pub fn run_job(shell_script: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(shell_script)
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        run_job("echo 'Hello, world!' | grep o | wc");
    }
}
