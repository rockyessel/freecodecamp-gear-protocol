#![no_std]

use gstd::msg;

/// The `handle` function is the entry point for handling incoming messages in the smart contract.
/// This function is marked with `#[no_mangle]` to ensure it can be called from the smart contract
/// runtime, bypassing Rust's name mangling.
///
/// This function performs the following steps:
/// 1. Sends a predefined joke message as a reply to any incoming message.
///
/// # Panics
/// This function will panic if:
/// - The reply message cannot be sent.
#[no_mangle]
extern "C" fn handle() {
    // Send a predefined joke message as a reply to any incoming message.
    // In an HTTP GET request, this would be analogous to sending a response.
    msg::reply_bytes(
        "What did the dirt say to the rain? If you keep this up, my name will be mud!",
        0,
    )
    .expect("Unable to reply");
}
