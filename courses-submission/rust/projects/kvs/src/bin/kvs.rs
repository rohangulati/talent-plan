extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("get").arg(Arg::with_name("KEY").required(true)))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("KEY").required(true))
                .arg(Arg::with_name("VALUE").required(true)),
        )
        .subcommand(SubCommand::with_name("rm").arg(Arg::with_name("KEY").required(true)))
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_match)) => {
            eprintln!("unimplemented");
            exit(1)
        }
        ("get", Some(_match)) => {
            eprintln!("unimplemented");
            exit(1)
        }
        ("rm", Some(_match)) => {
            eprintln!("unimplemented");
            exit(1)
        }
        _ => unreachable!(),
    }
}
