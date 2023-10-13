use crate::*;
use gpio_cdev::*;
use std::time::Duration;

pub struct GpioPins {
    pico_run: u32,
    pico_boot: u32,
    aux_en: u32,
    vdd_en: u32,
    usb_en: u32,
    aux_ocp: u32,
    vdd_ocp: u32,
    usb_ocp: u32,
}

pub struct Gpio {
    pico: MultiLineHandle,
    ocp: MultiLineHandle,
    aux_en: LineHandle,
    vdd_en: LineHandle,
    usb_en: LineHandle,
}

impl Gpio {
    pub fn new(chip: &str, pins: GpioPins) -> Result<Gpio, gpio_cdev::Error> {
        let mut chip = Chip::new(chip)?;

        let pico = chip.get_lines(&[pins.pico_run, pins.pico_boot])?.request(
            LineRequestFlags::OUTPUT,
            &[1, 1],
            "up_mcu_ctl",
        )?;

        let aux_en =
            chip.get_line(pins.aux_en)?
                .request(LineRequestFlags::OUTPUT, 0, "up_aux_en")?;

        let vdd_en =
            chip.get_line(pins.vdd_en)?
                .request(LineRequestFlags::OUTPUT, 0, "up_vdd_en")?;

        let usb_en =
            chip.get_line(pins.usb_en)?
                .request(LineRequestFlags::OUTPUT, 1, "up_usb_en")?;

        let ocp = chip
            .get_lines(&[pins.aux_ocp, pins.vdd_ocp, pins.usb_ocp])?
            .request(LineRequestFlags::INPUT, &[0, 0, 0], "up_ocp")?;

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
        self.pico.set_values(&[1, !boot as _]).ok();
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
                ocp: ocp[0] == 0,
            },
            vdd: PowerState {
                on: self.vdd_en.get_value().unwrap_or_default() != 0,
                ocp: ocp[1] == 0,
            },
            usb: PowerState {
                on: self.usb_en.get_value().unwrap_or_default() != 0,
                ocp: ocp[2] == 0,
            },
        }
    }
}

pub const R01_PINS: GpioPins = GpioPins {
    // GPIO38 - PD12
    pico_run: 108,
    // GPIO37 - PE10
    pico_boot: 138,
    // GPIO40 - PD22
    aux_en: 118,
    // GPIO36 - PE11
    vdd_en: 139,
    // GPIO31 - PE14
    usb_en: 142,
    // GPIO39 - PD11
    aux_ocp: 107,
    // GPIO35 - PE12
    vdd_ocp: 140,
    // GPIO30 - PE15
    usb_ocp: 143,
};

pub const CM4_PINS: GpioPins = GpioPins {
    // GPIO38 - gpio6 -
    pico_run: 0,
    // GPIO37 - gpio27 -
    pico_boot: 0,
    // GPIO40 - gpio16 -
    aux_en: 0,
    // GPIO36 - gpio26 -
    vdd_en: 0,
    // GPIO31 - gpio21 -
    usb_en: 0,
    // GPIO39 - gpio7 -
    aux_ocp: 0,
    // GPIO35 - gpio25 -
    vdd_ocp: 0,
    // GPIO30 - gpio20 -
    usb_ocp: 0,
};

pub const A04_PINS: GpioPins = GpioPins {
    // GPIO38
    pico_run: 0,
    // GPIO37
    pico_boot: 0,
    // GPIO40
    aux_en: 0,
    // GPIO36
    vdd_en: 0,
    // GPIO31
    usb_en: 0,
    // GPIO39
    aux_ocp: 0,
    // GPIO35
    vdd_ocp: 0,
    // GPIO30
    usb_ocp: 0,
};

pub const A06_PINS: GpioPins = GpioPins {
    // GPIO38
    pico_run: 0,
    // GPIO37
    pico_boot: 0,
    // GPIO40
    aux_en: 0,
    // GPIO36
    vdd_en: 0,
    // GPIO31
    usb_en: 0,
    // GPIO39
    aux_ocp: 0,
    // GPIO35
    vdd_ocp: 0,
    // GPIO30
    usb_ocp: 0,
};
