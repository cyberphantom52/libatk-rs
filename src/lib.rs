pub mod command;
pub mod device;
pub mod types;

pub mod prelude {
    pub use crate::command::{Command, CommandBuilder, CommandDescriptor};
    pub use crate::device::Device;
    pub use crate::types::{CommandId, EEPROMAddress};
    pub use libatk_derive::{command_extension, CommandDescriptor};
}
