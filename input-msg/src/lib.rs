#![no_std]

use gstd::{msg, prelude::*};

/// The `handle` function is the entry point for handling incoming messages in the smart contract.
/// This function is marked with `#[no_mangle]` to ensure it can be called from the smart contract
/// runtime, bypassing Rust's name mangling.
///
/// This function performs the following steps:
/// 1. Loads the incoming message from the input payload.
/// 2. Formats a reply message that includes the received message.
/// 3. Sends the formatted reply message back to the caller.
///
/// # Panics
/// This function will panic if:
/// - The message payload cannot be decoded.
/// - The reply message cannot be sent.
#[no_mangle]
extern "C" fn handle() {
    // Load the incoming message from the input payload. This message is expected to be of type `String`.
    // If decoding fails, the function will panic with the error message "Unable to create string".
    let new_msg: String = msg::load().expect("Unable to create string");

    // Create a formatted reply message that includes the received message.
    let reply_msg = format!("We've received your query {}", new_msg);

    // Send the formatted reply message back to the caller. If sending the reply fails, the function
    // will panic with the error message "Unable to reply."
    msg::reply_bytes(reply_msg, 0).expect("Unable to reply.");
}
