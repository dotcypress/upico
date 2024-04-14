#![no_std]
#![no_main]

extern crate panic_probe;
extern crate rp2040_hal as hal;
extern crate rtic;

use defmt_rtt as _;

mod upico;

use cortex_m::singleton;
use hal::adc;
use hal::gpio::*;
use hal::pac;
use hal::pio::{PIOBuilder, PIOExt};
use hal::timer::{monotonic::Monotonic, *};
use hal::usb::UsbBus;
use pio::Assembler;
use upico::*;
use usb_device::class_prelude::*;
use usb_device::prelude::*;

#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

pub const XTAL_FREQ_HZ: u32 = 12_000_000_u32;

#[rtic::app(device = pac, peripherals = true, dispatchers = [SW0_IRQ, SW1_IRQ])]
mod app {
    use super::*;

    #[monotonic(binds = TIMER_IRQ_0, default = true)]
    type Oracle = Monotonic<Alarm0>;

    #[local]
    struct Local {
        upico: UpicoClass,
        usb_dev: UsbDevice<'static, hal::usb::UsbBus>,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut resets = ctx.device.RESETS;
        let mut watchdog = hal::Watchdog::new(ctx.device.WATCHDOG);
        let clocks = hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut resets,
            &mut watchdog,
        )
        .ok()
        .expect("Clocks init failed");

        let mut timer = hal::Timer::new(ctx.device.TIMER, &mut resets, &clocks);
        let alarm = timer.alarm_0().expect("Alarm0 init failed");
        let mono = Monotonic::new(timer, alarm);

        let sio = hal::Sio::new(ctx.device.SIO);
        let pins = Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut resets,
        );

        let adc = adc::Adc::new(ctx.device.ADC, &mut resets);
        let adc_pins = (
            adc::AdcPin::new(pins.gpio26),
            adc::AdcPin::new(pins.gpio27),
            adc::AdcPin::new(pins.gpio28),
            adc::AdcPin::new(pins.gpio29),
        );

        pins.gpio0.into_function::<FunctionPio0>();
        pins.gpio1.into_function::<FunctionPio0>();
        pins.gpio2.into_function::<FunctionPio0>();
        pins.gpio3.into_function::<FunctionPio0>();
        pins.gpio4.into_function::<FunctionPio0>();
        pins.gpio5.into_function::<FunctionPio0>();
        pins.gpio6.into_function::<FunctionPio0>();
        pins.gpio7.into_function::<FunctionPio0>();
        pins.gpio8.into_function::<FunctionPio0>();
        pins.gpio9.into_function::<FunctionPio0>();
        pins.gpio10.into_function::<FunctionPio0>();
        pins.gpio11.into_function::<FunctionPio0>();
        pins.gpio12.into_function::<FunctionPio0>();
        pins.gpio13.into_function::<FunctionPio0>();
        pins.gpio14.into_function::<FunctionPio0>();
        pins.gpio15.into_function::<FunctionPio0>();

        let mut asm = Assembler::new();
        let mut wrap_target = asm.label();
        let mut wrap_source = asm.label();
        asm.bind(&mut wrap_target);
        asm.out(pio::OutDestination::EXEC, 32);
        asm.bind(&mut wrap_source);
        let (mut pio, sm, _, _, _) = ctx.device.PIO0.split(&mut resets);
        let program = pio.install(&asm.assemble_program()).unwrap();
        let (sm, rx, tx) = PIOBuilder::from_program(program)
            .autopull(true)
            .autopush(true)
            .pull_threshold(32)
            .push_threshold(32)
            .in_pin_base(0)
            .out_pins(0, 16)
            .build(sm);
        sm.start();

        let usb_regs = ctx.device.USBCTRL_REGS;
        let usb_dpram = ctx.device.USBCTRL_DPRAM;
        let usb_bus = UsbBus::new(usb_regs, usb_dpram, clocks.usb_clock, true, &mut resets);
        let usb_bus: &'static UsbBusAllocator<UsbBus> =
            singleton!(: UsbBusAllocator<UsbBus> = UsbBusAllocator::new(usb_bus))
                .expect("USB init failed");

        let upico = UpicoClass::new(usb_bus, rx, tx, adc, adc_pins);
        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0xbc07))
            .manufacturer("vitaly.codes")
            .product("uPico GPIO Extender")
            .build();
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::USBCTRL_IRQ);
        };

        (Shared {}, Local { upico, usb_dev }, init::Monotonics(mono))
    }

    #[task(binds = USBCTRL_IRQ, local = [usb_dev, upico])]
    fn usb_irq(ctx: usb_irq::Context) {
        ctx.local.usb_dev.poll(&mut [ctx.local.upico]);
    }
}
