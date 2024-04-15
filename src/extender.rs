use rusb::*;
use std::time::Duration;

pub struct GpioState {
    levels: u32,
    pin_dirs: u32,
}

impl GpioState {
    pub fn new(levels: u32, pin_dirs: u32) -> Self {
        Self { levels, pin_dirs }
    }

    pub fn get_mode(&self, pin: u8) -> bool {
        (self.pin_dirs >> pin) & 1 == 1
    }

    pub fn get_level(&self, pin: u8) -> bool {
        (self.levels >> pin) & 1 == 1
    }

    pub fn set_mode(&mut self, pin: u8, mode: bool) {
        if mode {
            self.pin_dirs |= 1 << pin;
        } else {
            self.pin_dirs &= !(1 << pin);
        }
    }

    pub fn set_level(&mut self, pin: u8, level: bool) {
        if level {
            self.levels |= 1 << pin;
        } else {
            self.levels &= !(1 << pin);
        }
    }
}

pub struct Extender;

impl Extender {
    pub fn read_analog() -> rusb::Result<[u16; 4]> {
        let dev = Self::open_device()?;
        let mut scratch = [0; 8];
        let req_type = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
        dev.read_control(
            req_type,
            0x01,
            0x00,
            0x00,
            &mut scratch,
            Duration::from_millis(100),
        )?;
        Ok([
            u16::from_le_bytes(scratch[0..2].try_into().unwrap()),
            u16::from_le_bytes(scratch[2..4].try_into().unwrap()),
            u16::from_le_bytes(scratch[4..6].try_into().unwrap()),
            u16::from_le_bytes(scratch[6..8].try_into().unwrap()),
        ])
    }

    pub fn read_digital() -> rusb::Result<GpioState> {
        let dev = Self::open_device()?;
        let mut scratch = [0; 8];
        let req_type = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
        dev.read_control(
            req_type,
            0x00,
            0x00,
            0x00,
            &mut scratch,
            Duration::from_millis(100),
        )?;
        let res = GpioState::new(
            u32::from_le_bytes(scratch[0..4].try_into().unwrap()),
            u32::from_le_bytes(scratch[4..8].try_into().unwrap()),
        );
        Ok(res)
    }

    pub fn write_digital(state: GpioState) -> rusb::Result<()> {
        let dev = Self::open_device()?;
        let mut payload = [0; 8];
        payload[0..4].copy_from_slice(&state.levels.to_le_bytes());
        payload[4..8].copy_from_slice(&state.pin_dirs.to_le_bytes());
        let req_type = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        dev.write_control(
            req_type,
            0x00,
            0x00,
            0x00,
            &payload,
            Duration::from_millis(100),
        )?;
        Ok(())
    }

    pub fn set_led(on: bool) -> rusb::Result<()> {
        let dev = Self::open_device()?;
        let req_type = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        dev.write_control(
            req_type,
            0x01,
            on as _,
            0x00,
            &[],
            Duration::from_millis(100),
        )?;
        Ok(())
    }

    fn open_device() -> rusb::Result<DeviceHandle<GlobalContext>> {
        rusb::open_device_with_vid_pid(0x1209, 0xbc07).ok_or(rusb::Error::NoDevice)
    }
}
