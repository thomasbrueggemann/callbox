use std::env;

use twilio_agnostic::{Client, OutboundMessage};

#[macro_use]
extern crate rocket;

#[post("/")]
async fn index() -> &'static str {
    let client = Client::new(
        &env::var("TWILIO_ACCOUNT_ID").unwrap(),
        &env::var("TWILIO_AUTH_TOKEN").unwrap(),
    );

    let from = env::var("MESSAGE_FROM").unwrap();
    let to = env::var("MESSAGE_TO").unwrap();
    let body = env::var("MESSAGE_BODY").unwrap();

    println!("Initiate call from {} to {}", &from, &to);

    client
        .send_message(OutboundMessage::new(&from, &to, &body))
        .await
        .unwrap();

    "Message sent..."
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", routes![index]).launch().await?;

    Ok(())
}
