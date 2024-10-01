use serde::Deserialize;
use rust_decimal::prelude::*;

#[derive(Deserialize,Debug)]
struct PriceResponse {
    data: CryptoRateUsd,
}

#[allow(non_snake_case)]
#[derive(Deserialize,Debug)]
struct CryptoRateUsd{
    rateUsd: String,
}

pub fn get_bitcoin_price()->Result<Decimal,Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://api.coincap.io/v2/rates/litecoin")?.json::<PriceResponse>()?;
    let price = match Decimal::from_str(&body.data.rateUsd) {
        Ok(num) => num,
        Err(_) => {
            println!("Error on converting");
            let a = Decimal::new(0, 1);
            a
        },       
    };

    Ok(price)
}




