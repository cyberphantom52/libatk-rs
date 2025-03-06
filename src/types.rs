#[derive(Debug)]
pub enum Error {
    InvalidBufferLength {
        expected: usize,
        actual: usize,
    },
    InvalidCommandId(u8),
    InvalidEEPROMAddress(u16),
    DataTooLarge(usize),
    InvalidDataLength {
        offset: usize,
        data_len: usize,
        allowed: usize,
    },
    InvalidOffset(usize),
    OffsetNotAligned(usize),
    HidError(hidapi::HidError),
    ParseError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Error::InvalidBufferLength { expected, actual } => {
                format!(
                    "Invalid buffer length: expected {}, got {}",
                    expected, actual
                )
            }
            Error::InvalidCommandId(id) => format!("Invalid CommandID: {}", id),
            Error::InvalidEEPROMAddress(addr) => format!("Invalid EEPROM Address: {}", addr),
            Error::InvalidOffset(offset) => format!("Invalid Offset: {}", offset),
            Error::HidError(e) => e.to_string(),
            Error::DataTooLarge(len) => format!("Length is larger than the maximum possible: {}", len),
            Error::InvalidDataLength {
                offset,
                data_len,
                allowed,
            } => format!("Invalid data len: Tried to write {} bytes, Only {} bytes can be written at offset {}, ", data_len, allowed, offset),
            Error::OffsetNotAligned(offset) => format!(
                "Provided offset is not aligned to a byte pair boundary: {}",
                offset
            ),
            Error::ParseError(e) => e.clone(),
        };

        write!(f, "{}", message)
    }
}

impl std::error::Error for Error {}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum CommandId {
    /// Not a valid CommandID, used for initialization
    Zero = 0x0,

    DownLoadData = 0x1,
    DownLoadDriverStatus,
    GetWirelessMouseOnline,
    GetBatteryLevel,
    SetWirelessDonglePair,
    GetWirelessDonglePairResult,
    SetEEPROM,
    GetEEPROM,
    RestoreFactory,
    ReportMouseStatus,
    Reserved1,
    Reserved2,
    EnterUSBUpgradeMode,
    GetCurrentConfig,
    SetCurrentConfig,
    GetMouseCIDMID,
    Reserved3,
    GetMouseVersion,
    DongleExitPair,
    Set4KRGBMode,
    Get4KRGBMode,
    SetFarDistanceMode,
    GetFarDistanceMode,
    SetDongleLightMode,
    GetDongleLightMode,
    ReportMouseUpgradeErrorStatus,
    ReportMouseUpgradeStatus,
}

impl TryFrom<u8> for CommandId {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0..=0x1b => unsafe { Ok(std::mem::transmute(value)) },
            _ => Err(Error::InvalidCommandId(value)),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum EEPROMAddress {
    ReportRate = 0x0,
    ReportRateCrc = 0x1,
    MaxDpi = 0x2,
    MaxDpiCrc = 0x3,
    CurrentDpi = 0x4,
    CurrentDpiCrc = 0x5,
    SilentHeight = 0xa,
    SilentHeightCrc = 0xb,

    // Pairwise DPI Profiles and Colors
    DpiPair1 = 0xc,
    DpiPair3 = 0x14,
    DpiPair5 = 0x1c,
    DpiPair7 = 0x24,
    DpiPair1Color = 0x2c,
    DpiPair3Color = 0x34,
    DpiPair5Color = 0x3c,
    DpiPair7Color = 0x44,

    // RGB Lighting
    DpiRgbLightingEffects = 0x4c,
    DpiRgbLightingEffectsCrc = 0x4d,
    DpiRgbLongBrightBrightness = 0x4e,
    DpiRgbLongBrightBrightnessCrc = 0x4f,
    DpiRgbLongBrightSpeed = 0x50,
    DpiRgbLongBrightSpeedCrc = 0x51,
    DpiRgbEnable = 0x52,
    DpiRgbEnableCrc = 0x53,

    ArticleLampR = 0x54,
    ArticleLampG = 0x55,
    ArticleLampB = 0x56,
    ArticleLampCRC = 0x57,
    ArticleLampEffects = 0x58,
    ArticleLampEffectsCRC = 0x59,
    ArticleLampLongBrightness = 0x5a,
    ArticleLampLongBrightnessCRC = 0x5b,
    ArticleLampBreathingSpeed = 0x5c,
    ArticleLampBreathingSpeedCRC = 0x5d,
    ArticleLampEnergySaving = 0x5e,
    ArticleLampEnergySavingCRC = 0x5f,

    StabilizationTime = 0xa9,
    StabilizationTimeCRC = 0xaa,
    MotionSync = 0xab,
    MotionSyncCRC = 0xac,
    CloseLedTime = 0xad,
    CloseLedTimeCRC = 0xae,
    LinearCorrection = 0xaf,
    LinearCorrectionCRC = 0xb0,
    RippleControl = 0xb1,
    RippleControlCRC = 0xb2,
    MoveCloseLights = 0xb3,
    MoveCloseLightsCRC = 0xb4,
    SensorEnable = 0xb5,
    SensorEnableCRC = 0xb6,
    SensorTime = 0xb7,
    SensorTimeCRC = 0xb8,
    SensorMode = 0xb9,
    SensorModeCRC = 0xba,
    RfTxTime = 0xbb,
    RfTxTimeCRC = 0xbc,

    // Keys
    Key0 = 0x60,
    Key1 = 0x64,
    Key2 = 0x68,
    Key3 = 0x6c,
    Key4 = 0x70,
    Key5 = 0x74,
    Key6 = 0x78,
    Key7 = 0x7c,
    Key8 = 0x80,
    Key9 = 0x84,
    Key10 = 0x88,
    Key11 = 0x8c,
    Key12 = 0x90,
    Key13 = 0x94,
    Key14 = 0x98,
    Key15 = 0x9c,

    // Shortcut keys
    KeyShortcuts0 = 0x100,
    KeyShortcuts1 = 0x120,
    KeyShortcuts2 = 0x140,
    KeyShortcuts3 = 0x160,
    KeyShortcuts4 = 0x180,
    KeyShortcuts5 = 0x1a0,
    KeyShortcuts6 = 0x1c0,
    KeyShortcuts7 = 0x1e0,
    KeyShortcuts8 = 0x200,
    KeyShortcuts9 = 0x220,
    KeyShortcuts10 = 0x240,
    KeyShortcuts11 = 0x260,
    KeyShortcuts12 = 0x280,
    KeyShortcuts13 = 0x2a0,
    KeyShortcuts14 = 0x2c0,
    KeyShortcuts15 = 0x2e0,

    // Macros
    Macro0 = 0x300,
    Macro1 = 0x480,
    Macro2 = 0x600,
    Macro3 = 0x780,
    Macro4 = 0x900,
    Macro5 = 0xa80,
    Macro6 = 0xc00,
    Macro7 = 0xd80,
    Macro8 = 0xf00,
    Macro9 = 0x1080,
    Macro10 = 0x1200,
    Macro11 = 0x1380,
    Macro12 = 0x1500,
    Macro13 = 0x1680,
    Macro14 = 0x1800,
    Macro15 = 0x1980,
}

impl TryFrom<u16> for EEPROMAddress {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0 | 0x1 | 0x2 | 0x3 | 0x4 | 0x5 | 0xa | 0xb | 0xc | 0x14 | 0x1c | 0x24 | 0x2c
            | 0x34 | 0x3c | 0x44 | 0x4c | 0x4d | 0x4e | 0x4f | 0x50 | 0x51 | 0x52 | 0x53 | 0x54
            | 0x55 | 0x56 | 0x57 | 0x58 | 0x59 | 0x5a | 0x5b | 0x5c | 0x5d | 0x5e | 0x5f | 0xa9
            | 0xaa | 0xab | 0xac | 0xad | 0xae | 0xaf | 0xb0 | 0xb1 | 0xb2 | 0xb3 | 0xb4 | 0xb5
            | 0xb6 | 0xb7 | 0xb8 | 0xb9 | 0xba | 0xbb | 0xbc | 0x60 | 0x64 | 0x68 | 0x6c | 0x70
            | 0x74 | 0x78 | 0x7c | 0x80 | 0x84 | 0x88 | 0x8c | 0x90 | 0x94 | 0x98 | 0x9c
            | 0x100 | 0x120 | 0x140 | 0x160 | 0x180 | 0x1a0 | 0x1c0 | 0x1e0 | 0x200 | 0x220
            | 0x240 | 0x260 | 0x280 | 0x2a0 | 0x2c0 | 0x2e0 | 0x300 | 0x480 | 0x600 | 0x780
            | 0x900 | 0xa80 | 0xc00 | 0xd80 | 0xf00 | 0x1080 | 0x1200 | 0x1380 | 0x1500
            | 0x1680 | 0x1800 | 0x1980 => unsafe { Ok(std::mem::transmute(value)) },
            _ => Err(Error::InvalidEEPROMAddress(value)),
        }
    }
}
