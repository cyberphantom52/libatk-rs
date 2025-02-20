# libatk-rs

libatk-rs is a Rust library that provides an abstraction over the reverse engineered ATK Mouse communication protocol, enabling others to easily communicate with and configure their devices. It offers a simple and type-safe interface for constructing commands and interacting with HID devices via the `hidapi` crate.

## Modules

- **command**
  Provides the `Command` struct and the `CommandDescriptor` trait. The trait defines properties required for constructing command messages, such as data offsets, report IDs, and overall command lengths. The `Command` struct encapsulates all command fields (command ID, status, EEPROM address, payload, and checksum) along with methods for data updates and serialization.

- **device**
  Contains the `Device` struct which wraps around a HID device from the `hidapi` crate. It offers high-level functions to send commands and read responses from the device. The sending functionality automatically prepends the required report ID before writing to the device.

- **types**
Contains values for `CommandId` and `EEPROMAddress` that were reverse engineered from the ATK Mouse communication protocol.


## Usage

Add `libatk-rs` to your Cargo.toml:

```toml
[dependencies]
libatk-rs = "0.1.0"  # replace with the current version
```

Below is a simple example of how to use libatk-rs in your project:

```rust
use libatk_rs::prelude::*;

// Replace these with actual vendor, product, usage_page, and usage values.
let vendor_id = 0x1234;
let product_id = 0x5678;
let usage_page = 0xFF00;
let usage = 0x01;

// Create a new device instance.
let device = Device::new(vendor_id, product_id, usage_page, usage)
    .expect("Device not found or failed to open");

// Create a default command (your specific command type must implement CommandDescriptor).
let mut command = Command::<YourCommandDescriptorType>::default();

// Modify command fields as required.
command.set_id(CommandId::DownLoadData);
command.set_status(0x01);
command.set_eeprom_address(EEPROMAddress::ReportRate);
command.set_data_len(10);

// Optionally, you can update data payload.
command.set_data(&[0x10, 0x20, 0x30], 0)
    .expect("Failed to set data");

println!("Sending command:\n{}", command);

// Send the command to the device.
device.send(command)
    .expect("Failed to send command");

// Read a response from the device.
let response = device.read().expect("Failed to read from device");
println!("Response: {:?}", response);
```

### Implementing a new Command

To create a new command, you need to define a struct that implements the `CommandDescriptor` trait. This trait requires you to define the base offset, report ID, and command length for the command.

The following examples shows implementing `GetBatteryStatus` command for VXE R1 Pro:
```rust
use libatk_rs::prelude::*;

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]]
struct GetBatteryStatus;

impl std::fmt::Display for Command<GetBatteryStatus> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Battery Level: {}% | Charge: {}C | Voltage: {}V",
            self.level(),
            self.charge(),
            self.voltage()
        )
    }
}

impl Command<GetBatteryStatus> {
    /// Method to query the battery status from the device.
    pub fn query() -> Command<GetBatteryStatus> {
        let mut command = Command::default();

        command.set_id(CommandId::GetBatteryLevel);

        command
    }

    pub fn level(&self) -> u8 {
        self.data()[0x0]
    }

    pub fn charge(&self) -> u8 {
        self.data()[0x1]
    }

    pub fn voltage(&self) -> f32 {
        self.data()[0x2] as f32 / 10f32
    }
}
```

## Contributing

Contributions are welcome! Please follow standard Rust coding conventions and include tests for new features or bug fixes. Pull requests should be aimed at keeping the code clean and maintainable.

## License

This project is licensed under the GPLv3 License. See the [LICENSE](LICENSE) file for details.
