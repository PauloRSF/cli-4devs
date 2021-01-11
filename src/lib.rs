use reqwest;

mod constants;

pub async fn fourdevs_fetch(params: &[(&str, &str)]) -> String {
    let client: reqwest::Client = reqwest::Client::new();
    let response = client.post(constants::URL).form(&params).send().await;

    assert!(
        response.is_ok(),
        "{} error fetching the data from 4devs!",
        constants::TAG
    );

    let response_data = response.unwrap().text().await;

    assert!(
        response_data.is_ok(),
        "{} error decoding the response from 4devs!",
        constants::TAG
    );

    return response_data.unwrap();
}
