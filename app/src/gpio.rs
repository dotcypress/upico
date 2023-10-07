use crate::*;
use gpio_cdev::*;
use std::time::Duration;

pub struct Gpio {
    pico: MultiLineHandle,
    ocp: MultiLineHandle,
    aux_en: LineHandle,
    vdd_en: LineHandle,
    usb_en: LineHandle,
}

impl Gpio {
    pub fn new(chip: &str) -> Result<Gpio, gpio_cdev::Error> {
        let mut chip = Chip::new(chip)?;

        let pico = chip.get_lines(&[r01::PICO_RUN, r01::PICO_BOOT])?.request(
            LineRequestFlags::OUTPUT,
            &[1, 1],
            "upico_mcu_ctl",
        )?;

        let aux_en =
            chip.get_line(r01::AUX_EN)?
                .request(LineRequestFlags::OUTPUT, 0, "upico_aux_en")?;

        let vdd_en =
            chip.get_line(r01::VDD_EN)?
                .request(LineRequestFlags::OUTPUT, 0, "upico_vdd_en")?;

        let usb_en =
            chip.get_line(r01::USB_EN)?
                .request(LineRequestFlags::OUTPUT, 1, "upico_usb_en")?;

        let ocp = chip
            .get_lines(&[r01::AUX_OCP, r01::VDD_OCP, r01::USB_OCP])?
            .request(LineRequestFlags::INPUT, &[0, 0, 0], "upico_ocp")?;

        Ok(Self {
            pico,
            ocp,
            aux_en,
            vdd_en,
            usb_en,
        })
    }

    pub fn reset_pico(&mut self, boot: bool) {
        self.pico.set_values(&[0, !boot as _]).ok();
        thread::sleep(Duration::from_millis(100));
        self.pico.set_values(&[1, 1]).ok();
    }

    pub fn power_on(&mut self, line: PowerLine) {
        match line {
            PowerLine::Aux => self.aux_en.set_value(1),
            PowerLine::Vdd => self.vdd_en.set_value(1),
            PowerLine::Usb => self.usb_en.set_value(1),
        }
        .ok();
    }

    pub fn power_off(&mut self, line: PowerLine) {
        match line {
            PowerLine::Aux => self.aux_en.set_value(0),
            PowerLine::Vdd => self.vdd_en.set_value(0),
            PowerLine::Usb => self.usb_en.set_value(0),
        }
        .ok();
    }

    pub fn power_cycle(&mut self, line: PowerLine) {
        self.power_off(line);
        thread::sleep(Duration::from_millis(100));
        self.power_on(line);
    }

    pub fn power_report(&mut self) -> PowerReport {
        let ocp = self.ocp.get_values().unwrap_or_default();
        PowerReport {
            aux: PowerState {
                on: self.aux_en.get_value().unwrap_or_default() != 0,
                ocp: ocp[0] != 0,
            },
            vdd: PowerState {
                on: self.vdd_en.get_value().unwrap_or_default() != 0,
                ocp: ocp[1] != 0,
            },
            usb: PowerState {
                on: self.usb_en.get_value().unwrap_or_default() != 0,
                ocp: ocp[2] != 0,
            },
        }
    }
}

mod r01 {
    // GPIO38 - PD12
    pub const PICO_RUN: u32 = 108;

    // GPIO37 - PE10
    pub const PICO_BOOT: u32 = 138;

    // GPIO40 - PD22
    pub const AUX_EN: u32 = 118;

    // GPIO36 - PE11
    pub const VDD_EN: u32 = 139;

    // GPIO31 - PE14
    pub const USB_EN: u32 = 142;

    // GPIO39 - PD11
    pub const AUX_OCP: u32 = 107;

    // GPIO35 - PE12
    pub const VDD_OCP: u32 = 140;

    // GPIO30 - PE15
    pub const USB_OCP: u32 = 143;
}
