use crate::*;
use std::time::Duration;

pub struct Gpio {}

impl Gpio {
    pub fn try_new() -> Result<Gpio, io::Error> {
        Self::set_pin_mode_out(platform::PIN_PICO_RUN, true)?;
        Self::set_pin_mode_out(platform::PIN_PICO_BOOT, true)?;
        Self::set_pin_mode_out(platform::PIN_AUX_EN, false)?;
        Self::set_pin_mode_out(platform::PIN_VDD_EN, false)?;
        Self::set_pin_mode_out(platform::PIN_USB_EN, false)?;
        Self::set_pin_mode_in(platform::PIN_AUX_OCP)?;
        Self::set_pin_mode_in(platform::PIN_VDD_OCP)?;
        Self::set_pin_mode_in(platform::PIN_USB_OCP)?;
        Ok(Self {})
    }

    pub fn reset_pico(&mut self, boot: bool) -> Result<(), io::Error> {
        Self::set_pin_state(platform::PIN_VDD_EN, true)?;
        Self::set_pin_state(platform::PIN_PICO_RUN, false)?;
        Self::set_pin_state(platform::PIN_PICO_BOOT, !boot)?;
        thread::sleep(Duration::from_millis(200));
        Self::set_pin_state(platform::PIN_VDD_EN, false)?;
        Self::set_pin_state(platform::PIN_PICO_RUN, true)?;
        if boot {
            thread::sleep(Duration::from_millis(100));
            Self::set_pin_state(platform::PIN_PICO_BOOT, true)?;
        }
        Ok(())
    }

    pub fn set_power_enabled(&mut self, line: PowerLine, enabled: bool) -> Result<(), io::Error> {
        match line {
            PowerLine::Aux => Self::set_pin_state(platform::PIN_AUX_EN, !enabled),
            PowerLine::Vdd => Self::set_pin_state(platform::PIN_VDD_EN, !enabled),
            PowerLine::Usb => Self::set_pin_state(platform::PIN_USB_EN, !enabled),
        }
    }

    pub fn power_cycle(&mut self, line: PowerLine) -> Result<(), io::Error> {
        self.set_power_enabled(line, false)?;
        thread::sleep(Duration::from_millis(100));
        self.set_power_enabled(line, true)
    }

    pub fn power_report(&mut self) -> Result<PowerReport, io::Error> {
        Ok(PowerReport {
            aux: PowerState {
                on: !Self::get_pin_state(platform::PIN_AUX_EN)?,
                ocp: !Self::get_pin_state(platform::PIN_AUX_OCP)?,
            },
            vdd: PowerState {
                on: !Self::get_pin_state(platform::PIN_VDD_EN)?,
                ocp: !Self::get_pin_state(platform::PIN_VDD_OCP)?,
            },
            usb: PowerState {
                on: !Self::get_pin_state(platform::PIN_USB_EN)?,
                ocp: !Self::get_pin_state(platform::PIN_USB_OCP)?,
            },
        })
    }

    fn set_pin_mode_out(pin: usize, def_state: bool) -> Result<(), io::Error> {
        process::Command::new("gpio")
            .args(["mode", &pin.to_string(), "out"])
            .output()?;
        Self::set_pin_state(pin, def_state)
    }

    fn set_pin_mode_in(pin: usize) -> Result<(), io::Error> {
        process::Command::new("gpio")
            .args(["mode", &pin.to_string(), "in"])
            .output()?;
        Ok(())
    }

    fn get_pin_state(pin: usize) -> Result<bool, io::Error> {
        let stdout = process::Command::new("gpio")
            .args(["read", &pin.to_string()])
            .stdout(process::Stdio::piped())
            .output()?
            .stdout;
        Ok(!stdout.is_empty() && stdout[0] == b'1')
    }

    fn set_pin_state(pin: usize, state: bool) -> Result<(), io::Error> {
        process::Command::new("gpio")
            .args(["write", &pin.to_string(), if state { "1" } else { "0" }])
            .output()?;
        Ok(())
    }
}
