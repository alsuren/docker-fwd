fn main() {
    run().unwrap()
}

fn run() -> Result<(), std::io::Error> {
    // TODO:
    // Parse -v flags, and replace them appropriately
    // Parse -p x:y flags and create corresponding -L x:localhost:y flags
    // Explode if anyone tries to port-forward and also run in the background (-d)
    let args: Vec<_> = std::env::args().skip(1).collect();

    let pwd = format!("./{}", std::env::current_dir()?.to_str().unwrap());

    let mut command = std::process::Command::new("ssh");
    command
        .args(["localhost", "./bin/in", &pwd, "docker"])
        .args(args)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());
    dbg!(&command);
    let mut process = command.spawn()?;
    process.wait()?;

    Ok(())
}
