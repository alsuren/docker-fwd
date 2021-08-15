fn main() {
    run().unwrap()
}

fn run() -> Result<(), std::io::Error> {
    // TODO:
    // Parse -v flags, and replace them appropriately
    // Parse -p x:y flags and create corresponding -L x:localhost:y flags
    // Explode if anyone tries to -P (port forward everything that's exposed)
    // Explode if anyone tries to port-forward and also run in the background (-d)
    let args: Vec<_> = std::env::args().skip(1).collect();

    // We map "$PWD" outside the vm to "$HOME/$PWD" inside.
    let pwd = format!("./{}", std::env::current_dir()?.to_str().unwrap());

    let mut command = std::process::Command::new("ssh");
    command
        // ssh -t allocates a tty so that ^C works correctly
        .args(["-t", "localhost"])
        // We assume that https://crates.io/crates/in-directory is installed already.
        .args(["./bin/in", &pwd])
        // Forward the rest of the docker args unchanged
        .arg("docker")
        .args(args);

    // TODO: switch to pretty_env_logger or something
    eprintln!("Running: {:?}", &command);

    // It might be possible to `exec` here, but I'm keeping my options
    // open, in case I need to do something with file synchronisation
    // or port forwarding on demand, in the background.
    command
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());
    let mut process = command.spawn()?;
    process.wait()?;

    Ok(())
}
