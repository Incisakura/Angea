use std::env;
use std::process;

use angea::shell;
use angea::systemd::Systemd;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        help();
        process::exit(1);
    }
    match args[1].as_str() {
        "boot"      => boot(),
        "shell"     => shell(),
        "shutdown"  => shutdown(),
        _           => help()
    }
}

fn boot() {
    Systemd::new();
}

fn shell() {
    shell::enter();
}

fn shutdown() {
    Systemd::from_proc().unwrap().shutdown();
}

fn help() {
    print!("Angea v0.0.1
Usage: angea <command>
Command:
    boot        Boot systemd as daemon
    shell       Init bash shell in container
    shutdown    Kill runing systemd
    help        This message
");
}