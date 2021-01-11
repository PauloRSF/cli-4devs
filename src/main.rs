use clap::{App, AppSettings};

mod constants;
mod generators;

use generators::cpf;

fn main() {
    let app_args = App::new("4Devs CLI")
        .bin_name("4devs")
        .version("0.0.1")
        .author("PauloRSF <paulorsouzaf@gmail.com>")
        .about("A CLI for 4Devs data generation services.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cpf::build_args())
        .get_matches();

    let (sub, sub_args) = app_args.subcommand().unwrap();

    match sub {
        "cpf" => cpf::execute(sub_args),
        _ => (),
    }
}
