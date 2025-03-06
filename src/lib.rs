pub mod command;
pub mod device;
pub mod types;

pub mod prelude {
    pub use crate::command::{Command, CommandBuilder, CommandDescriptor};
    pub use crate::device::Device;
    pub use crate::types::{CommandId, EEPROMAddress, Error};
    pub use libatk_derive::{command_extension, Command};
}
