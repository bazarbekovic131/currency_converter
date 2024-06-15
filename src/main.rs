use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Currency {
    name: &'static str,
    preciousness: f64,
    code: &'static str,
    symbol: &'static str,
}

struct Amount<'a> {
    amount: f64,
    currency: &'a str,
}

impl Currency {
   const fn new(name: &'static str, preciousness: f64, code: &'static str, symbol: &'static str) -> Currency {
        Currency {
            name,
            preciousness,
            code,
            symbol,
        }
    }

    fn convert(amount: Amount, to_currency: &Currency, currency_map: &HashMap<String, &Currency>) -> Result<f64, String> {
        let from_currency = currency_map.get(amount.currency)
            .ok_or_else(|| format!("Invalid from currency: {}", amount.currency))?;
        Ok(amount.amount * (from_currency.preciousness / to_currency.preciousness))
    }
}

fn init_currencies() -> HashMap<String, &'static Currency> {
    static USD: Currency = Currency::new("US Dollar", 1.0, "USD", "$");
    static TENGE: Currency = Currency::new("Tenge", 0.0022, "KZT", "₸");
    static EURO: Currency = Currency::new("Euro", 1.07, "EUR", "€");
    static YEN: Currency = Currency::new("Japanese Yen", 0.0064, "JPY", "¥");
    static POUND: Currency = Currency::new("British Pound", 1.27, "GBP", "£");
    static RUBLE: Currency = Currency::new("Ruble", 0.01, "RUB", "₽");

    let currencies = vec![&USD, &TENGE, &EURO, &YEN, &POUND, &RUBLE];
    let mut currency_map = HashMap::new();

    for &currency in &currencies {
        currency_map.insert(currency.code.to_lowercase(), currency);
        currency_map.insert(currency.name.to_lowercase(), currency);
        currency_map.insert(currency.symbol.to_lowercase(), currency);
    }

    currency_map
}

fn main() {
    let currency_map = init_currencies();

    // for debugging
    // println!("{:?}", currency_map);

    loop {
        println!("Enter the amount and currencies in the format 'amount from_currency to_currency' (e.g., '20 USD to KZT'):");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Exit command entered");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() < 4 {
            println!("Invalid input format. Please use the format 'amount from_currency to to_currency'.");
            return;
        }

        let to_index = parts.iter().position(|&x| x.eq_ignore_ascii_case("to")).unwrap_or(1);
        if to_index == 1 || to_index == parts.len() - 1 {
            println!("Invalid input format. Please use the format 'amount from_currency to to_currency'.");
            return;
        }

        let from_currency = parts[1..to_index].join(" ");
        let to_currency = parts[to_index + 1..].join(" ");

        let amount: f64 = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid amount.");
                return;
            }
        };

        let amount = Amount {
            amount,
            currency: &from_currency,
        };

        let to_currency = match currency_map.get(&to_currency) {
            Some(currency) => currency,
            None => {
                println!("Invalid to currency: {}", to_currency);
                return;
            }
        };

        match Currency::convert(amount, to_currency, &currency_map) {
            Ok(converted_amount) => println!("Converted amount: {} {}", converted_amount, to_currency.symbol),
            Err(e) => println!("Error: {}", e),
        }
        }
}
