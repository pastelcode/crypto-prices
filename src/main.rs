mod models;

use std::io::Write;

use models::coin_data::CoinData;

fn main() {
    print!("Enter the name of the coin (example: ethereum): ");
    std::io::stdout().flush().unwrap();
    let mut coin = String::new();
    if let Err(error) = std::io::stdin().read_line(&mut coin) {
        eprintln!("[ERROR]: An error occurred reading your coin: {}", error);
        return;
    }

    println!("Loading coin data...");
    let coin_result = get_coin_data(&coin.trim().to_lowercase());
    if let Err(ureq::Error::Status(404, _)) = coin_result {
        eprintln!("[ERROR]: The coin '{}' was not found", coin);
        return;
    } else if let Err(error) = coin_result {
        eprintln!(
            "[ERROR]: An error ocurred while getting the coin price: {}",
            error
        );
        return;
    }
    let coin_data = coin_result.unwrap();
    println!(
        "1 {coin} = {usd} USD",
        coin = coin_data.symbol.to_uppercase(),
        usd = coin_data.market_data.current_price.usd
    );
}

fn get_coin_data(coin: &str) -> Result<CoinData, ureq::Error> {
    let body = ureq::get(&format!(
        "https://api.coingecko.com/api/v3/coins/{}?localization=false",
        coin
    ))
    .call()?
    .into_string()?;
    let coin_data = serde_json::from_str::<CoinData>(&body).unwrap();
    Ok(coin_data)
}
