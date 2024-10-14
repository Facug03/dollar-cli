use chrono::DateTime;
use colored::Colorize;
use reqwest::{blocking, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct DollarResponse {
  moneda: String,
  nombre: String,
  compra: f32,
  venta: f32,
  fechaActualizacion: String,
}

#[derive(Debug)]
struct Dollar {
  currency: String,
  name: String,
  buy: i32,
  sell: i32,
  last_update: String,
}

fn main() {
  let base_url = "https://dolarapi.com/v1";

  println!("Welcome!");

  let all_prices = get_all_prices(base_url).expect("There was an error getting prices!");

  println!("Here is the info of prices today!");
  println!("Currency {}", all_prices[0].currency);

  print_dollar_info(&all_prices);
}

fn get_all_prices(base_url: &str) -> Result<Vec<Dollar>, Error> {
  let url_all_prices = format!("{}/dolares", base_url);

  let resp = blocking::get(&url_all_prices)?;
  let resp_json: Vec<DollarResponse> = resp.json::<Vec<DollarResponse>>()?;

  let dollars: Vec<Dollar> = resp_json
    .into_iter()
    .map(|dolar_response| Dollar {
      currency: dolar_response.moneda,
      name: dolar_response.nombre,
      buy: dolar_response.compra as i32,
      sell: dolar_response.venta as i32,
      last_update: format_date(&dolar_response.fechaActualizacion),
    })
    .collect();

  Ok(dollars)
}

fn format_date(date_str: &str) -> String {
  let parsed_data = DateTime::parse_from_rfc3339(date_str).expect("Failed to parse date");

  parsed_data.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn print_dollar_info(dollars: &Vec<Dollar>) {
  for dollar in dollars {
    println!("-- {}", dollar.name.green());
    println!("Buy: {} | Sell: {}", dollar.buy, dollar.sell);
    println!("Last Updated: {}", dollar.last_update);
  }
}
