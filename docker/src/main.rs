mod args;
use std::process::exit;

use argh::FromArgs;

fn main() {
    run().unwrap()
}

fn run() -> Result<(), std::io::Error> {
    // TODO:
    // Parse -v flags, and replace them appropriately
    // Parse -p x:y flags and create corresponding -L x:localhost:y flags
    // Explode if anyone tries to -P (port forward everything that's exposed)
    // Explode if anyone tries to port-forward and also run in the background (-d)
    let args: Vec<_> = std::env::args().collect();
    let strs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let mut ssh_flags: Vec<String> = Vec::new();
    match args::Docker::from_args(&["docker"], &strs[1..]) {
        Ok(args::Docker {
            subcommand: args::Subcommand::Run(run),
        }) => {
            if !run.volume.is_empty() {
                dbg!(&run.volume);
            }
            for mapping in run.publish.iter() {
                let (l, r) = mapping
                    .split_once(':')
                    .expect("-p mappings must contain a :");
                ssh_flags.push(format!("-L{l}:localhost:{r}", l = l, r = r))
            }
        }
        Err(e) => {
            dbg!(&e);
            if std::env::var("PEDANTIC").is_ok() {
                exit(1)
            }
        }
    }

    // We map "$PWD" outside the vm to "$HOME/$PWD" inside.
    let pwd = format!("./{}", std::env::current_dir()?.to_str().unwrap());

    let mut command = std::process::Command::new("ssh");
    command
        .args(ssh_flags)
        // ssh -t allocates a tty so that ^C works correctly
        .args(["-t", "localhost"])
        // We assume that https://crates.io/crates/in-directory is installed already.
        .args(["./bin/in", &pwd])
        // Forward the rest of the docker args unchanged
        .arg("docker")
        .args(&args[1..]);

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
    // TODO: if `cargo watch -x run` kills us, we don't kill the docker
    // image properly (^C works fine though).
    process.wait()?;

    Ok(())
}
