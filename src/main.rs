use std::env;
use twilio::{Client, OutboundCall};

#[macro_use]
extern crate rocket;

#[post("/")]
async fn index() -> &'static str {
    let client = Client::new(
        &env::var("TWILIO_ACCOUNT_ID").unwrap(),
        &env::var("TWILIO_AUTH_TOKEN").unwrap(),
    );

    let from = env::var("CALL_FROM").unwrap();
    let to = env::var("CALL_TO").unwrap();
    let callback_url = format!("{}/callback", env::var("BASE_URL").unwrap());

    println!("Initiate call from {} to {}", &from, &to);

    client
        .make_call(OutboundCall::new(&from, &to, &callback_url))
        .await
        .unwrap();

    "Call in progress..."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
