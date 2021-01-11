use clap::{App, Arg, ArgMatches};
use regex::Regex;

use crate::constants;

pub const CREDIT_CARD_BRANDS: [&str; 10] = [
    "master", "visa", "amex", "diners", "discover", "enroute", "jcb", "voyager", "hipercard", "aura",
];

fn get_field_re(field_name: &str, exp: &str) -> Regex {
    return Regex::new(&format!(
        "<div id=\"{}\".*>({})<span",
        field_name, exp
    ))
    .unwrap();
}

pub fn build_args() -> App<'static> {
    return App::new("credit_card")
        .about("Generates credit cards info.")
        .arg(
            Arg::new("brand")
                .about("Specifies the card's brand")
                .short('b')
                .long("brand")
                .default_value(CREDIT_CARD_BRANDS[0])
                .takes_value(true)
                .possible_values(&CREDIT_CARD_BRANDS),
        )
        .arg(
            Arg::new("punctuated")
                .about("Prints the zip_code with punctuation")
                .short('p')
                .long("punctuated"),
        );
}

#[tokio::main]
pub async fn execute(args: &ArgMatches) {
    let params = [
        ("acao", "gerar_cc"),
        (
            "pontuacao",
            if args.is_present("punctuated") {
                "S"
            } else {
                "N"
            },
        ),
        ("bandeira", args.value_of("brand").unwrap_or("")),
    ];

    let data = cli_4devs::fourdevs_fetch(&params).await;

    let fields = [
        ("Number", "cartao_numero", "[0-9\\s]+"),
        ("Expiration date", "data_validade", "\\d{2}/\\d{2}/\\d{4}"),
        ("CVV", "codigo_seguranca", "\\d{3,4}"),
    ];

    for field in &fields {
        let (name, param_name, exp) = field;
        let re = get_field_re(param_name, exp);
        let re_match = re.captures(&data);

        assert!(
            re_match.is_some(),
            "{} couldn't find the \"{}\" field in the response!",
            constants::TAG,
            name
        );

        println!("{}: {}", name, re_match.unwrap().get(1).unwrap().as_str());
    }
}
