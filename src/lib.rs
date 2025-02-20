pub mod command;
pub mod types;

pub mod prelude {
    pub use crate::command::{Command, CommandDescriptor};
    pub use crate::types::{CommandId, EEPROMAddress};
    pub use atk_command_derive::CommandDescriptor;
}
