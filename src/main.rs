use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::io;

// Defining the structure for the API response
#[derive(Deserialize)]
struct ApiResponse {
    result: String,
    base_code: String,
    time_last_update_utc: String,
    time_next_update_utc: String,
    conversion_rates: HashMap<String, f64>,
}

#[tokio::main]
async fn main () -> Result<(), Error> {
    // API ENDPOINT
    let api_url = "https://v6.exchangerate-api.com/v6/9084516a191ef4d617f9c14e/latest/USD";

    let response = reqwest::get(api_url).await?.json::<ApiResponse>().await?;

    //Check to see if API Call was successful
    if response.result != "success" {
        println!("Failed to fetch Exchange rates Data!");
    }

    println!("Base currency is: {}", response.base_code);
    println!("Rates last updated: {}", response.time_last_update_utc);
    println!("Rates next update: {}", response.time_next_update_utc);


    //prompt user for input
    let ( to_currency, amount_to_convert) = get_user_conversion_request();


    if let Some(rate) = response.conversion_rates.get(&to_currency) {
        let converted_amount = amount_to_convert * rate;

        println!("{} {} is equivalent to {:.2} {}", 
        amount_to_convert, response.base_code, converted_amount, to_currency);
    } 
    else{
        println!("Conversion rate for {} not found", to_currency);
    }

    Ok(())
}


// Function to get user input
fn get_user_conversion_request() -> (String, f64) {
    let mut to_currency = String::new();
    let mut amount = String::new();

    println!("Enter the target currency (e.g., EUR):");
    io::stdin().read_line(&mut to_currency).unwrap();
    let to_currency = to_currency.trim().to_uppercase();

    println!("Enter the amount to convert:");
    io::stdin().read_line(&mut amount).unwrap();
    let amount: f64 = amount.trim().parse().unwrap_or(0.0);

    (to_currency, amount)
}