wit_bindgen::generate!({ generate_all });

use exports::wasmcloud::messaging::handler::Guest;
use wasi::logging::logging::*;
use wasmcloud::messaging::*;

struct Cat;

impl Guest for Cat {
    fn handle_message(msg: types::BrokerMessage) -> Result<(), String> {
        if let Some(reply_to) = msg.reply_to {
            consumer::publish(&types::BrokerMessage {
                subject: reply_to,
                reply_to: None,
                body: b"meow".to_vec(),
            })
        } else {
            log(
                Level::Warn,
                "",
                "No reply_to field in message, ignoring message",
            );
            Ok(())
        }
    }
}

export!(Cat);
