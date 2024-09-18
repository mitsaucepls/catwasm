wit_bindgen::generate!({ generate_all });

use core::str;

use exports::wasmcloud::messaging::handler::Guest;
use wasi::logging::logging::*;
use wasmcloud::messaging::*;

struct Cat;

impl Guest for Cat {
    fn handle_message(msg: types::BrokerMessage) -> Result<(), String> {
        log(Level::Info, "", str::from_utf8(&msg.body).map_err(|err| err.to_string())?);
        consumer::publish(&types::BrokerMessage {
            subject: String::from("wasmcloud.catcomponent"),
            reply_to: Some(String::from("wasmcloud.catconsumer")),
            body: b"rawr".to_vec(),
        })
    }
}

export!(Cat);
