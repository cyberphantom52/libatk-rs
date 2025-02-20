pub mod command;
pub mod device;
pub mod types;

pub mod prelude {
    pub use crate::command::{Command, CommandDescriptor};
    pub use crate::device::Device;
    pub use crate::types::{CommandId, EEPROMAddress};
    pub use libatk_derive::CommandDescriptor;
}
