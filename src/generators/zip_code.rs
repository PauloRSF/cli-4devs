use clap::{App, Arg, ArgMatches};
use regex::Regex;

use crate::constants;

fn get_field_re(field_name: &str, exp: &str) -> Regex {
    return Regex::new(&format!(
        "<div id=\"{}\" class=\"output-txt\"><span>({})</span>",
        field_name, exp
    ))
    .unwrap();
}

pub fn build_args() -> App<'static> {
    return App::new("zip_code")
        .about("Generates zip codes.")
        .arg(
            Arg::new("state")
                .about("Specifies the origin state of the zip code")
                .short('s')
                .long("state")
                .takes_value(true)
                .possible_values(&constants::STATE_CHOICES),
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
        ("acao", "gerar_cep"),
        (
            "somente_numeros",
            if args.is_present("punctuated") {
                "S"
            } else {
                "N"
            },
        ),
        ("cep_estado", args.value_of("state").unwrap_or("")),
        ("cep_cidade", ""), // TODO: add city selection if state option is present
    ];

    let data = cli_4devs::fourdevs_fetch(&params).await;

    let fields = [
        ("Zip code", "cep", "[0-9\\-]+"),
        ("Address", "endereco", "[\\w\\s,'/\\-]+"),
        ("District", "bairro", "[\\w\\s,'/\\-]+"),
        ("City", "cidade", "[\\w\\s,'/\\-]+"),
        ("State", "estado", "\\w{2}"),
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
