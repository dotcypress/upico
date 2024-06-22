#[cfg(feature = "r01")]
pub mod platform {
    pub const OCP_REPORTING: bool = true;
    pub const AUX_SWITCH: bool = true;
    pub const PIN_PICO_BOOT: usize = 37;
    pub const PIN_VDD_EN: usize = 36;
    pub const PIN_USB_EN: usize = 31;
    pub const PIN_PICO_RUN: usize = 38;
    pub const PIN_AUX_EN: usize = 40;
    pub const PIN_AUX_OCP: usize = 39;
    pub const PIN_VDD_OCP: usize = 35;
    pub const PIN_USB_OCP: usize = 30;
}

#[cfg(feature = "cm4")]
pub mod platform {
    pub const AUX_SWITCH: bool = true;
    pub const OCP_REPORTING: bool = false;
    pub const PIN_PICO_BOOT: usize = 27;
    pub const PIN_VDD_EN: usize = 26;
    pub const PIN_USB_EN: usize = 21;
    pub const PIN_PICO_RUN: usize = 6;
    pub const PIN_AUX_EN: usize = 16;
    //TODO: fix pcb routing
    pub const PIN_AUX_OCP: usize = 29;
    pub const PIN_VDD_OCP: usize = 25;
    pub const PIN_USB_OCP: usize = 20;
}

#[cfg(feature = "cm4-bookworm")]
pub mod platform {
    pub const AUX_SWITCH: bool = true;
    pub const OCP_REPORTING: bool = true;
    pub const PIN_PICO_BOOT: usize = 2;
    pub const PIN_VDD_EN: usize = 25;
    pub const PIN_USB_EN: usize = 29;
    pub const PIN_PICO_RUN: usize = 22;
    pub const PIN_AUX_EN: usize = 27;
    pub const PIN_AUX_OCP: usize = 11;
    pub const PIN_VDD_OCP: usize = 6;
    pub const PIN_USB_OCP: usize = 28;
}

#[cfg(feature = "a06")]
pub mod platform {
    pub const OCP_REPORTING: bool = false;
    pub const AUX_SWITCH: bool = false;
    pub const PIN_PICO_BOOT: usize = 37;
    pub const PIN_VDD_EN: usize = 36;
    pub const PIN_USB_EN: usize = 31;
    //TODO: fix pcb routing
    pub const PIN_PICO_RUN: usize = 42;
    pub const PIN_AUX_EN: usize = 40;
    pub const PIN_AUX_OCP: usize = 36;
    pub const PIN_VDD_OCP: usize = 35;
    pub const PIN_USB_OCP: usize = 30;
}

#[cfg(feature = "a04")]
pub mod platform {
    pub const OCP_REPORTING: bool = todo!();
    pub const AUX_SWITCH: bool = todo!();
    pub const PIN_PICO_BOOT: usize = todo!();
    pub const PIN_VDD_EN: usize = todo!();
    pub const PIN_USB_EN: usize = todo!();
    pub const PIN_PICO_RUN: usize = todo!();
    pub const PIN_AUX_EN: usize = todo!();
    pub const PIN_AUX_OCP: usize = todo!();
    pub const PIN_VDD_OCP: usize = todo!();
    pub const PIN_USB_OCP: usize = todo!();
}
