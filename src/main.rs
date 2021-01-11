use clap::{App, AppSettings};

mod constants;

fn main() {
    let app_args = App::new("4Devs CLI")
        .bin_name("4devs")
        .version("0.0.1")
        .author("PauloRSF <paulorsouzaf@gmail.com>")
        .about("A CLI for 4Devs data generation services.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let (sub, sub_args) = app_args.subcommand().unwrap();

    match sub {
        _ => (),
    }
}
