use clap::{App, Arg, ArgMatches};

use crate::constants;

pub fn build_args() -> App<'static> {
    return App::new("cpf")
        .about("Generates CPFs.")
        .arg(
            Arg::new("state")
                .about("Specifies the origin state of the CPF")
                .short('s')
                .long("state")
                .takes_value(true)
                .possible_values(&constants::STATE_CHOICES),
        )
        .arg(
            Arg::new("punctuated")
                .about("Prints the CPF with punctuation")
                .short('p')
                .long("punctuated"),
        );
}

#[tokio::main]
pub async fn execute(args: &ArgMatches) {
    let params = [
        ("acao", "gerar_cpf"),
        (
            "pontuacao",
            if args.is_present("punctuated") {
                "S"
            } else {
                "N"
            },
        ),
        ("cpf_estado", args.value_of("state").unwrap_or("")),
    ];

    let cpf = cli_4devs::fourdevs_fetch(&params).await;

    println!("CPF: {}", cpf);
}
