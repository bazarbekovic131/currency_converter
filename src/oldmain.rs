use std::io::{self, Write};

fn find_currency<'a>(query: &str, currencies: &'a [Currency]) -> Option<&'a Currency> {
    currencies.iter().find(|&c| c.code.eq_ignore_ascii_case(query)
        || c.name.eq_ignore_ascii_case(query)
        || c.symbol.eq_ignore_ascii_case(query))
}

fn main() {
    let currencies = init_currencies();

    println!("Enter the amount and currencies in the format 'amount from_currency to_currency' (e.g., '20 USD to KZT'):");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 4 || parts[2] != "to" {
        println!("Invalid input format. Please use the format 'amount from_currency to_currency'.");
        return;
    }

    let amount: f64 = match parts[0].parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid amount.");
            return;
        }
    };
    let to_index = parts.iter().position(|&x| x.eq_ignore_ascii_case("to")).unwrap_or(1);
    if to_index == 1 || to_index == parts.len() - 1 {
        println!("Invalid input format. Please use the format 'amount from_currency to to_currency'.");
        return;
    }

    let from_currency = parts[1..to_index].join(" ");
    let to_currency = parts[to_index + 1..].join(" ");

    let amount = Amount {
        amount,
        currency: from_currency.clone(),
    };

    let to_currency = match find_currency(&to_currency, &currencies) {
        Some(currency) => currency,
        None => {
            println!("Invalid to currency: {}", to_currency);
            return;
        }
    };

    match Currency::convert(amount, to_currency, &currencies) {
        Ok(converted_amount) => println!("Converted amount: {} {}", converted_amount, to_currency.symbol),
        Err(e) => println!("Error: {}", e),
    }
}



fn init_currencies() -> Vec<Currency> {
    let usd = Currency::new("US Dollar", 1.0, "USD", "$");
    let tenge = Currency::new("Tenge", 0.0022, "KZT", "₸");
    let euro = Currency::new("Euro", 1.07, "EUR", "€");
    let yen = Currency::new("Japanese Yen", 0.0064, "JPY", "¥");
    let pound = Currency::new("British Pound", 1.27, "GBP", "£");
    let ruble = Currency::new("Ruble", 0.01,"RUB","₽" );
    vec![usd, tenge, euro, yen, pound, ruble]
}

struct Currency {
    name: String,
    preciousness: f64,
    code: String,
    symbol: String,
}

struct Amount {
    amount: f64,
    currency: String,
}

impl Currency {
    fn new(name: &str, preciousness: f64, code: &str, symbol: &str) -> Currency {
        Currency {
            name: name.to_string(),
            preciousness,
            code: code.to_string(),
            symbol: symbol.to_string(),
        }
    }
    //
    // fn convert(amount: Amount, to_currency: &Currency, currencies: &[Currency]) -> f64 {
    //     let from_currency = currencies.iter().find(|&c| c.code == amount.currency).expect("Invalid input currency");
    //     amount.amount * (from_currency.preciousness / to_currency.preciousness)
    // }

    fn convert(amount: Amount, to_currency: &Currency, currencies: &[Currency]) -> Result<f64, String> {
        let from_currency = find_currency(&amount.currency, currencies)
            .ok_or_else(|| format!("Invalid from currency: {}", amount.currency))?;
        Ok(amount.amount * (from_currency.preciousness / to_currency.preciousness))
    }
}



