use clap::{arg, command, Command};

pub fn init() -> Command {
    command!()
        .arg_required_else_help(true)
        .arg(arg!(-b --bulk "bulk resolve"))
        .arg(arg!(-r --rand "random select"))
        .arg(arg!([domain] "FQDN domain name to resolve"))
}
