use crate::command::{Command, CommandDescriptor};
use hidapi::HidDevice;

static MAX_REPORT_LENGTH: usize = 64;

/// A wrapper around a HID device that simplifies communication by exposing functionality for sending commands
/// and reading responses.
///
/// The Device struct encapsulates a [hidapi::HidDevice] and provides methods to send commands with their specific
/// report IDs and read responses from the device (while handling the report ID in the data).
///
/// # Examples
///
/// ```no_run
/// // Create a new device by specifying vendor id, product id, usage page and usage.
/// let device = Device::new(0x1234, 0x5678, 0xFF00, 0x01)
///     .expect("Device not found or failed to open");
///
/// // Create your command according to your custom CommandDescriptor implementation:
/// let command = Command::new(...);
///
/// // Send command to the device.
/// device.send(command).expect("Failed to send command");
///
/// // Read the response from the device.
/// let response = device.read().expect("Failed to read response");
/// println!("Response: {:?}", response);
/// ```
pub struct Device(HidDevice);

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = self.0.get_device_info().unwrap();
        let product_string = info.product_string().unwrap();
        let manufacturer_string = info.manufacturer_string().unwrap();
        let serial_number_string = info.serial_number().unwrap();
        let path = info.path().to_str().unwrap();
        write!(
            f,
            "Device: {}\nManufacturer: {}\nSerial Number: {}\nPath: {}",
            product_string, manufacturer_string, serial_number_string, path
        )
    }
}

impl Device {
    /// Creates a new Device instance by searching for a HID device matching the given vendor id, product id,
    /// usage page, and usage.
    ///
    /// # Arguments
    ///
    /// * `vendor_id` - The vendor identifier of the device.
    /// * `product_id` - The product identifier of the device.
    /// * `usage_page` - The usage page of the device.
    /// * `usage` - The usage identifier of the device.
    ///
    /// # Returns
    ///
    /// * `Ok(Device)` if a matching device is found and successfully opened.
    /// * `Err(hidapi::HidError)` if no matching device is found or opening the device fails.
    ///
    /// # Examples
    /// ```no_run
    /// let device = Device::new(0x1234, 0x5678, 0xFF00, 0x01)
    ///     .expect("Failed to open device");
    /// ```
    pub fn new(
        vendor_id: u16,
        product_id: u16,
        usage_page: u16,
        usage: u16,
    ) -> Result<Self, hidapi::HidError> {
        let context = hidapi::HidApi::new().unwrap();

        let device = context
            .device_list()
            .filter(|&d| {
                d.product_id() == product_id
                    && d.vendor_id() == vendor_id
                    && d.usage_page() == usage_page
                    && d.usage() == usage
            })
            .next()
            .ok_or(hidapi::HidError::HidApiError {
                message: format!(
                    "Device not found: vendor_id={} product_id={} usage_page={} usage={}",
                    vendor_id, product_id, usage_page, usage
                ),
            })?;

        Ok(Device(device.open_device(&context)?))
    }

    /// Sends a command to the device.
    ///
    /// This function takes a command that implements the CommandDescriptor trait, prepends the report ID,
    /// and sends the complete command over the HID interface.
    ///
    /// # Type Parameters
    ///
    /// * `T`: A type that implements the CommandDescriptor trait and represents the specific command descriptor.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to be sent.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` indicating the number of bytes written if the write operation is successful.
    /// * `Err(hidapi::HidError)` if the write operation fails.
    ///
    /// # Examples
    /// ```no_run
    /// let bytes_written = device.send(command).expect("Failed to send command");
    /// println!("Bytes written: {}", bytes_written);
    /// ```
    pub fn send<T: CommandDescriptor>(
        &self,
        command: Command<T>,
    ) -> Result<usize, hidapi::HidError> {
        // Prepend Report ID to the command
        let data = [[T::report_id()].as_ref(), command.as_bytes().as_ref()].concat();
        self.0.write(&data)
    }

    /// Reads data from the device.
    ///
    /// This method reads a report from the underlying HID device into a fixed-size buffer, strips off the first byte
    /// (which is assumed to be the Report ID), and returns the remaining bytes as a vector.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` containing the data read from the device (without the report ID) if successful.
    /// * `Err(hidapi::HidError)` if the read operation fails.
    ///
    /// # Examples
    /// ```no_run
    /// let response = device.read().expect("Failed to read from device");
    /// println!("Response: {:?}", response);
    /// ```
    pub fn read(&self) -> Result<Vec<u8>, hidapi::HidError> {
        let mut buf = [0u8; MAX_REPORT_LENGTH];
        let bytes_read = self.0.read(&mut buf)?;

        // Remove Report ID from the response
        Ok(buf[1..bytes_read].to_vec())
    }

    /// Executes a command by sending it to the device and reading the response.
    ///
    /// This is a safe wrapper around the `send` and `read` as it ensures that the returned command type is same as the input command type.
    ///
    /// # Returns
    ///
    /// * `Ok(Command<T>)` if the command execution is successful.
    /// * `Err(hidapi::HidError)` if the command execution fails.
    ///
    /// # Examples
    /// ```no_run
    /// let response = device.execute(command).expect("Failed to execute command");
    /// println!("Response: {:?}", response);
    /// ```
    pub fn execute<T: CommandDescriptor>(
        &self,
        command: Command<T>,
    ) -> Result<Command<T>, hidapi::HidError> {
        self.send(command)?;
        let response = self.read()?;

        Command::try_from(response.as_ref()).map_err(|e| hidapi::HidError::HidApiError {
            message: format!("Failed to convert response to command: {}", e),
        })
    }
}
