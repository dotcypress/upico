use cortex_m::prelude::_embedded_hal_adc_OneShot;
use rp2040_hal::{
    adc::*,
    gpio::{self, bank0::*, *},
    pac::PIO0,
    pio,
    Adc,
};
use embedded_hal::digital::v2::OutputPin;
use usb_device::{class_prelude::*, control::*};

pub type Led = Pin<Gpio25, FunctionSio<SioOutput>, hal::gpio::PullDown>;

pub type AdcPins = (
    AdcPin<gpio::Pin<Gpio26, FunctionNull, PullDown>>,
    AdcPin<gpio::Pin<Gpio27, FunctionNull, PullDown>>,
    AdcPin<gpio::Pin<Gpio28, FunctionNull, PullDown>>,
    AdcPin<gpio::Pin<Gpio29, FunctionNull, PullDown>>,
);

pub struct UpicoClass {
    adc: Adc,
    adc_pins: AdcPins,
    iface: InterfaceNumber,
    led: Led,
    rx: pio::Rx<(PIO0, pio::SM0)>,
    tx: pio::Tx<(PIO0, pio::SM0)>,
    pin_dirs: u32,
}

impl UpicoClass {
    pub fn new<B: UsbBus>(
        alloc: &UsbBusAllocator<B>,
        rx: pio::Rx<(PIO0, pio::SM0)>,
        tx: pio::Tx<(PIO0, pio::SM0)>,
        adc: Adc,
        adc_pins: AdcPins,
        led: Led,
    ) -> UpicoClass {
        Self {
            adc,
            adc_pins,
            led,
            rx,
            tx,
            pin_dirs: 0,
            iface: alloc.interface(),
        }
    }
}

impl<B: UsbBus> UsbClass<B> for UpicoClass {
    fn get_configuration_descriptors(
        &self,
        writer: &mut DescriptorWriter,
    ) -> usb_device::Result<()> {
        writer.interface(self.iface, 0xff, 0x00, 0x00)?;
        Ok(())
    }

    fn control_out(&mut self, xfer: ControlOut<B>) {
        let req = xfer.request();
        if req.request_type != RequestType::Vendor || req.recipient != Recipient::Device {
            return;
        }
        match req.request {
            0x00 if xfer.data().len() == 8 => {
                let state = u32::from_le_bytes(xfer.data()[0..4].try_into().unwrap());
                self.tx.write(0b01100000_00000000);
                self.tx.write(state);

                self.pin_dirs = u32::from_le_bytes(xfer.data()[4..8].try_into().unwrap());
                self.tx.write(0b01100000_10000000);
                self.tx.write(self.pin_dirs);

                xfer.accept()
            }
            0x01 => {
                self.led.set_state(PinState::from(req.value != 0)).unwrap();
                xfer.accept()
            }
            _ => xfer.reject(),
        }
        .ok();
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = xfer.request();
        if req.request_type != RequestType::Vendor || req.recipient != Recipient::Device {
            return;
        }
        match req.request {
            0x00 => {
                self.tx.write(0b01000000_00000000);
                if let Some(data) = self.rx.read() {
                    let mut res = [0; 8];
                    res[0..4].copy_from_slice(&data.to_le_bytes());
                    res[4..8].copy_from_slice(&self.pin_dirs.to_le_bytes());
                    xfer.accept_with(&res)
                } else {
                    xfer.reject()
                }
            }
            0x01 => {
                let ch0: u16 = self.adc.read(&mut self.adc_pins.0).unwrap_or_default();
                let ch1: u16 = self.adc.read(&mut self.adc_pins.1).unwrap_or_default();
                let ch2: u16 = self.adc.read(&mut self.adc_pins.2).unwrap_or_default();
                let ch3: u16 = self.adc.read(&mut self.adc_pins.3).unwrap_or_default();

                let mut res = [0; 8];
                res[0..2].copy_from_slice(&ch0.to_le_bytes());
                res[2..4].copy_from_slice(&ch1.to_le_bytes());
                res[4..6].copy_from_slice(&ch2.to_le_bytes());
                res[6..8].copy_from_slice(&ch3.to_le_bytes());
                xfer.accept_with(&res)
            }
            _ => xfer.reject(),
        }
        .ok();
    }
}
