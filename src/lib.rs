use std::{process::{ChildStdout, Command, Stdio}, io::{BufRead, Read}};
use colored::*;

pub struct Shell;

impl Shell {    
    pub async fn run(command: &str, print: bool) -> Result<(), anyhow::Error> {
        if print { println!("{}", command.blue().italic()) };
        let mut output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
    
        let status = output.wait()?;
        if !status.success() {
            let mut err_str = String::new();
            output.stderr.unwrap().read_to_string(&mut err_str)?;
            return Err(anyhow::anyhow!("Command failed with exit code: {:?}, stderr: {}", status.code(), err_str));
        }
    
        if print { print_output(output.stdout.unwrap()) };
    
        Ok(())
    }
    
    pub async fn run_with_exit_status(command: &str, print: bool) -> Result<i32, anyhow::Error> {
        if print { println!("{}", command.blue().italic()) };
        let mut output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()?;
    
        let status = output.wait()?.code().unwrap_or(1);
    
        if print { print_output(output.stdout.unwrap()) };
    
        Ok(status)
    }
}

fn print_output(stdout: ChildStdout) {
    // let stdout = output.clone().stdout;
    let cursor = std::io::Cursor::new(stdout).into_inner();
    let reader = std::io::BufReader::new(cursor);
    
    for line in reader.lines() {
        let line = line.expect("failed to read line");
        println!("{}", line);
    }
}