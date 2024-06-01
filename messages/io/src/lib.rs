#![no_std]

// Import necessary components from the `gmeta` and `gstd` crates
use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, Vec};

/// Represents the metadata structure for handling messages.
/// This struct does not have any fields and is used solely
/// for implementing the `Metadata` trait.
pub struct MessageMetadata;

/// Represents a static mutable vector to store messages.
/// This is marked as unsafe because mutable static variables
/// are inherently unsafe in Rust due to potential data races.
pub static mut MESSAGES: Vec<Message> = Vec::new();

/// Represents a message with an identifier and content.
/// This struct is used to encapsulate the message data.
///
/// # Fields
/// - `id`: The identifier of the actor (user or smart contract) sending the message.
/// - `content`: The actual content of the message.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Message {
    pub id: ActorId,
    pub content: String,
}

/// Implementation of the `Metadata` trait for the `MessageMetadata` struct.
/// This trait defines the types used for initializing, handling, and querying
/// the state of the smart contract.
impl Metadata for MessageMetadata {
    /// Defines the types used for the initialization phase.
    /// - `Init` expects an input of type `Message` and returns a `String`.
    type Init = InOut<Message, String>;
    
    /// Defines the types used for handling incoming messages.
    /// - `Handle` expects an input of type `Message` and returns a `String`.
    type Handle = InOut<Message, String>;
    
    /// Defines the type used for querying the state of the smart contract.
    /// - `State` outputs a vector of `Message` without expecting any input.
    type State = Out<Vec<Message>>;
    
    /// Defines the type used for replies.
    /// - `Reply` is set to `()`, indicating no specific reply type is used.
    type Reply = ();
    
    /// Defines other types used in the smart contract.
    /// - `Others` is set to `()`, indicating no additional types are used.
    type Others = ();
    
    /// Defines the type used for signals.
    /// - `Signal` is set to `()`, indicating no signals are used.
    type Signal = ();
}
