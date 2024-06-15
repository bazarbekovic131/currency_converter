use iced::{button, text_input, Align, Application, Button, Column, Command, Element, Row, Settings, Text, TextInput};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Message {
    Convert,
    AmountChanged(String),
    FromCurrencyChanged(String),
    ToCurrencyChanged(String),
}

struct ConverterApp {
    amount: String,
    from_currency: String,
    to_currency: String,
    result: String,
    currencies: HashMap<String, &'static Currency>,
    amount_input: text_input::State,
    from_input: text_input::State,
    to_input: text_input::State,
    convert_button: button::State,
}

impl Application for ConverterApp {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ConverterApp, Command<Self::Message>) {
        let currencies = init_currencies();
        (
            ConverterApp {
                amount: String::new(),
                from_currency: String::new(),
                to_currency: String::new(),
                result: String::new(),
                currencies,
                amount_input: text_input::State::new(),
                from_input: text_input::State::new(),
                to_input: text_input::State::new(),
                convert_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Currency Converter")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Convert => {
                if let Ok(amount) = self.amount.parse::<f64>() {
                    let from_currency = self.from_currency.to_lowercase();
                    let to_currency = self.to_currency.to_lowercase();
                    let amount = Amount {
                        amount,
                        currency: &from_currency,
                    };

                    if let Some(to_currency) = self.currencies.get(&to_currency) {
                        match Currency::convert(amount, to_currency, &self.currencies) {
                            Ok(converted_amount) => {
                                self.result = format!("Converted amount: {} {}", converted_amount, to_currency.symbol);
                            }
                            Err(e) => {
                                self.result = e;
                            }
                        }
                    } else {
                        self.result = format!("Invalid to currency: {}", self.to_currency);
                    }
                } else {
                    self.result = String::from("Invalid amount.");
                }
            }
            Message::AmountChanged(value) => {
                self.amount = value;
            }
            Message::FromCurrencyChanged(value) => {
                self.from_currency = value;
            }
            Message::ToCurrencyChanged(value) => {
                self.to_currency = value;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Currency Converter").size(30))
            .push(
                TextInput::new(
                    &mut self.amount_input,
                    "Amount",
                    &self.amount,
                    Message::AmountChanged,
                )
                    .padding(10),
            )
            .push(
                TextInput::new(
                    &mut self.from_input,
                    "From Currency",
                    &self.from_currency,
                    Message::FromCurrencyChanged,
                )
                    .padding(10),
            )
            .push(
                TextInput::new(
                    &mut self.to_input,
                    "To Currency",
                    &self.to_currency,
                    Message::ToCurrencyChanged,
                )
                    .padding(10),
            )
            .push(
                Button::new(&mut self.convert_button, Text::new("Convert"))
                    .on_press(Message::Convert),
            )
            .push(Text::new(&self.result).size(20))
            .into()
    }
}