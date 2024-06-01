#![no_std]

// External crate
use gstd::{msg, prelude::*};

// Internal crate
use messages_io::*;

/// The `init` function is the entry point for initializing the smart contract.
/// This function is marked with `#[no_mangle]` to ensure it can be called
/// from the smart contract runtime, bypassing Rust's name mangling.
///
/// This function performs the following steps:
/// 1. Loads the initial message from the input payload.
/// 2. Creates a new `Message` instance from the loaded data.
/// 3. Pushes the new message to the global `MESSAGES` vector.
/// 4. Sends a reply indicating successful initialization.
///
/// # Panics
/// This function will panic if:
/// - The message payload cannot be decoded.
/// - The reply message cannot be sent.
#[no_mangle]
extern "C" fn init() {
    // Load the initial message from the input payload
    let init: Message = msg::load().expect("Unable to decode Message");

    // Create a new Message instance from the loaded data
    let init_message = Message {
        id: init.id,
        content: init.content,
    };

    // Push the new message to the global MESSAGES vector
    unsafe { MESSAGES.push(init_message) };

    // Send a reply indicating successful initialization
    msg::reply_bytes("Successfully initialized", 0).expect("Failed to send reply message.");
}

/// The `handle` function is the entry point for handling incoming messages.
/// This function is marked with `#[no_mangle]` to ensure it can be called
/// from the smart contract runtime, bypassing Rust's name mangling.
///
/// This function performs the following steps:
/// 1. Loads the incoming message from the input payload.
/// 2. Creates a new `Message` instance from the loaded data.
/// 3. Pushes the new message to the global `MESSAGES` vector.
/// 4. Sends a reply indicating the message was sent successfully.
///
/// # Panics
/// This function will panic if:
/// - The message payload cannot be decoded.
/// - The reply message cannot be sent.
#[no_mangle]
extern "C" fn handle() {
    // Load the incoming message from the input payload
    let message_handler: Message = msg::load().expect("Unable to decode Message");

    // Create a new Message instance from the loaded data
    let message = Message {
        id: message_handler.id,
        content: message_handler.content,
    };

    // Push the new message to the global MESSAGES vector
    unsafe { MESSAGES.push(message) };

    // Send a reply indicating the message was sent successfully
    msg::reply_bytes("Message sent successfully.", 0).expect("Failed to send reply message.");
}

/// The `state` function is the entry point for querying the state of the smart contract.
/// This function is marked with `#[no_mangle]` to ensure it can be called
/// from the smart contract runtime, bypassing Rust's name mangling.
///
/// This function performs the following steps:
/// 1. Clones the current state of the global `MESSAGES` vector.
/// 2. Sends the cloned state as a reply.
///
/// # Panics
/// This function will panic if:
/// - The state message cannot be sent.
#[no_mangle]
extern "C" fn state() {
    // Clone the current state of the global MESSAGES vector
    msg::reply(unsafe { MESSAGES.clone() }, 0).expect("Failed to clone and send a reply state");
}
