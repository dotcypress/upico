/// CM4 core
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod platform {
    pub const OCP_REPORTING: bool = false;
    pub const AUX_SWITCH: bool = false;
    pub const PICO_PATH: &str = "/media/pi/RPI-RP2";
    pub const PIN_PICO_BOOT: usize = 27;
    pub const PIN_VDD_EN: usize = 26;
    pub const PIN_USB_EN: usize = 21;
    //TODO: fix pcb routing
    pub const PIN_PICO_RUN: usize = 22;
    pub const PIN_AUX_EN: usize = 23;
    pub const PIN_AUX_OCP: usize = 29;
    pub const PIN_VDD_OCP: usize = 25;
    pub const PIN_USB_OCP: usize = 20;
}

/// R-01 core
#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
pub mod platform {
    pub const OCP_REPORTING: bool = true;
    pub const AUX_SWITCH: bool = true;
    pub const PICO_PATH: &str = "/media/cpi/RPI-RP2";
    pub const PIN_PICO_RUN: usize = 38;
    pub const PIN_PICO_BOOT: usize = 37;
    pub const PIN_AUX_EN: usize = 40;
    pub const PIN_VDD_EN: usize = 36;
    pub const PIN_USB_EN: usize = 31;
    pub const PIN_AUX_OCP: usize = 39;
    pub const PIN_VDD_OCP: usize = 35;
    pub const PIN_USB_OCP: usize = 30;
}
