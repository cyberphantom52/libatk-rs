use crate::types::{CommandId, EEPROMAddress};

/// A trait that defines certain properties for command types.
///
/// Implementors of this trait must provide:
/// - The offset within the command buffer where the data field starts.
/// - The report ID used for calculating checksums.
/// - The total length of the command (in bytes).
pub trait CommandDescriptor {
    /// Returns the offset of the first byte of the data field.
    ///
    /// The offset is used when extracting or modifying the data payload.
    fn base_offset() -> usize;

    /// Returns the report ID as a u8.
    ///
    /// The report ID is used when calculating the command checksum.
    fn report_id() -> u8;

    /// Returns the total length (in bytes) of the command.
    fn cmd_len() -> usize;
}

/*
The command layout is as follows:

┌────────────┬───────────────┬────────────────┬───────────────────┬──────────────┬──────────┐
│ Command ID │ Command Status│ EEPROM Address │ Data Valid Length │     Data     │ Checksum │
│   1 Byte   │    1 Byte     │   2 Bytes      │      1 Byte       │  10 Bytes    │  1 Byte  │
└────────────┴───────────────┴────────────────┴───────────────────┴──────────────┴──────────┘

Note:
- `base_offset` represents the offset from the start of the command to the beginning of the data field.
*/

/// A generic command that stores the various fields (header, data payload, checksum)
/// of a command. The command is parameterized using a type which implements the
/// `CommandDescriptor` trait for command-specific size and layout definitions.
pub struct Command<T: CommandDescriptor> {
    command_id: CommandId,
    status: u8,
    eeprom_address: EEPROMAddress,
    data_len: usize,
    data: Vec<u8>,
    checksum: u8,
    _cmd: std::marker::PhantomData<T>,
}

impl<T: CommandDescriptor> Clone for Command<T> {
    fn clone(&self) -> Self {
        Self {
            command_id: self.command_id,
            status: self.status,
            eeprom_address: self.eeprom_address,
            data_len: self.data_len,
            data: self.data.clone(),
            checksum: self.checksum,
            _cmd: std::marker::PhantomData,
        }
    }
}

impl<T: CommandDescriptor> std::fmt::Debug for Command<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X?}", self.as_bytes())
    }
}

impl<T: CommandDescriptor> std::fmt::Display for Command<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {:?}\nStatus: {}\nAddress: {:?}\nData Length: {}\nData: {:X?}\nChecksum: {}",
            self.command_id,
            self.status,
            self.eeprom_address,
            self.data_len,
            &self.data[..self.data_len],
            self.checksum
        )
    }
}

impl<T: CommandDescriptor> Default for Command<T> {
    fn default() -> Self {
        Self {
            command_id: CommandId::Zero,
            status: 0,
            eeprom_address: EEPROMAddress::ReportRate,
            data_len: 0,
            data: vec![0u8; T::cmd_len() - T::base_offset() - 1],
            checksum: 0,
            _cmd: std::marker::PhantomData,
        }
    }
}

impl<T: CommandDescriptor> TryFrom<&[u8]> for Command<T> {
    type Error = String;

    fn try_from(raw: &[u8]) -> Result<Self, Self::Error> {
        if raw.len() != T::cmd_len() {
            return Err(format!(
                "Invalid buffer length: expected {}, got {}",
                T::cmd_len(),
                raw.len()
            ));
        }

        let command_id = raw[0x0].into();
        let status = raw[0x1];
        let eeprom_address = u16::from_be_bytes([raw[0x2], raw[0x3]]).into();
        let data_len = raw[0x4] as usize;
        let data = raw[T::base_offset()..T::base_offset() + data_len].to_vec();
        let checksum = raw[0xf];

        Ok(Self {
            command_id,
            status,
            eeprom_address,
            data_len,
            data,
            checksum,
            _cmd: std::marker::PhantomData,
        })
    }
}

impl<T: CommandDescriptor> Command<T> {
    /// Sets a slice of data into the command's data payload at a given offset.
    ///
    /// # Arguments
    ///
    /// * `data` - The slice of data to set.
    /// * `offset` - The offset in the data payload where this slice should be copied.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the data is set successfully.
    /// * `Err(&'static str)` if the provided data length plus offset exceeds the valid data length.
    pub fn set_data(&mut self, data: &[u8], offset: usize) -> Result<(), &'static str> {
        if offset + data.len() > self.data_len() {
            return Err("Invalid data length");
        }

        self.data[offset..offset + data.len()].copy_from_slice(data);
        self.set_checksum();
        Ok(())
    }

    /// Sets a single byte in the command's data payload.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte value to be set.
    /// * `offset` - The index in the data payload where the byte should be written.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the value is successfully set.
    /// * `Err(&'static str)` if the offset is out of bounds.
    pub fn set_data_byte(&mut self, value: u8, offset: usize) -> Result<(), &'static str> {
        if offset > self.data_len() - 1 {
            return Err("Provided offset is greater than the length of the data payload");
        }

        self.data[offset] = value;
        self.set_checksum();
        Ok(())
    }

    /// Sets a byte value at an even offset and also writes its complementary checksum byte.
    ///
    /// This method first writes the provided `value` at the given *even* `offset` and then writes
    /// the checksum (calculated as `0x55.warpping_sum(value)`) at the following byte.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte value to be set.
    /// * `offset` - The starting offset (must be even) in the data payload.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the operation is successful.
    /// * `Err(&'static str)` if the offset is not aligned (not even) or out of bounds.
    pub fn set_data_byte_with_checksum(
        &mut self,
        value: u8,
        offset: usize,
    ) -> Result<(), &'static str> {
        if offset % 2 != 0 {
            return Err("Provided offset is not aligned to a byte pair boundary");
        }

        self.set_data_byte(value, offset)?;
        self.set_data_byte(0x55u8.wrapping_sub(value), offset + 1)?;

        Ok(())
    }

    pub fn id(&self) -> CommandId {
        self.command_id
    }

    /// Sets the command ID and updates the checksum afterward.
    pub fn set_id(&mut self, id: CommandId) {
        self.command_id = id;
        self.set_checksum();
    }

    pub fn status(&self) -> u8 {
        self.status
    }

    /// Sets the status byte and updates the checksum afterward.
    pub fn set_status(&mut self, status: u8) {
        self.status = status;
        self.set_checksum();
    }

    /// Returns the EEPROM address associated with the command.
    pub fn eeprom_address(&self) -> EEPROMAddress {
        self.eeprom_address
    }

    /// Sets the EEPROM address and updates the checksum.
    pub fn set_eeprom_address(&mut self, address: EEPROMAddress) {
        self.eeprom_address = address;
        self.set_checksum();
    }

    /// Returns the valid length of the data payload.
    pub fn data_len(&self) -> usize {
        self.data_len
    }

    /// Sets the valid data length.
    ///
    /// # Panics
    ///
    /// Panics if the provided length exceeds the maximum available space computed via:
    /// `T::cmd_len() - T::base_offset()`
    pub fn set_data_len(&mut self, len: usize) {
        if len as usize > T::cmd_len() - T::base_offset() {
            panic!("Invalid data length");
        }

        self.data_len = len;
        self.set_checksum();
    }

    fn set_checksum(&mut self) {
        let sum: u8 = {
            let mut sum = T::report_id() as u16;
            sum += self.command_id as u16;
            sum += self.status as u16;
            sum += self.eeprom_address as u16;
            sum += self.data_len as u16;
            sum += self.data.iter().fold(0, |acc, &byte| acc + byte as u16);
            (sum & 0xff) as u8
        };
        let checksum = 0x55u8.wrapping_sub(sum);
        self.checksum = checksum;
    }

    /// Serializes the command into a vector of bytes.
    ///
    /// The serialization follows this order:
    /// 1. Command ID
    /// 2. Status
    /// 3. EEPROM address as big-endian bytes
    /// 4. Valid data length
    /// 5. Data payload
    /// 6. Checksum
    ///
    /// # Returns
    ///
    /// A vector containing the bytewise representation of the command.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut raw = vec![self.command_id as u8, self.status];
        raw.extend_from_slice(&(self.eeprom_address as u16).to_be_bytes());
        raw.push(self.data_len as u8);
        raw.extend_from_slice(&self.data);
        raw.push(self.checksum);
        raw
    }
}
