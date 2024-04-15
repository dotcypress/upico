use clap::{builder::PossibleValue, *};
use config::*;
use extender::*;
use gpio::*;
use service::*;
use std::path::Path;
use std::time::Duration;
use std::*;

mod config;
mod extender;
mod gpio;
mod service;

#[derive(Debug)]
pub enum AppError {
    InvalidLine,
    InvalidGpioLine,
    InvalidAdcChannel,
    InvalidLedMode,
    MountFailed,
    IoError(io::Error),
    ServiceError(io::Error),
    GpioError(io::Error),
    DecodeError(string::FromUtf8Error),
    ParseIntError(num::ParseIntError),
    ProtocolError(rmp_serde::decode::Error),
    UsbError(rusb::Error),
}

pub type AppResult = Result<(), AppError>;

fn main() {
    if let Err(err) = run() {
        match err {
            AppError::InvalidLine => println!("Invalid power line name"),
            AppError::InvalidAdcChannel => println!("Invalid ADC channel"),
            AppError::InvalidLedMode => println!("Invalid LED mode"),
            AppError::InvalidGpioLine => println!("Invalid GPIO number"),
            AppError::MountFailed => println!("Failed to mount Pico drive"),
            AppError::GpioError(err) => println!("GPIO error: {}", err),
            AppError::IoError(err) => println!("IO error: {}", err),
            AppError::ServiceError(err) => println!("Service error: {}", err),
            AppError::DecodeError(err) => println!("Decode error: {}", err),
            AppError::ProtocolError(err) => println!("Protocol error: {}", err),
            AppError::UsbError(rusb::Error::NoDevice) => println!("Pico extender not found.\nCommand for flashing extender firmware: \"upico gpio install\"."),
            AppError::UsbError(err) => println!("USB error: {}", err),
            AppError::ParseIntError(err) => println!("Parse error: {}", err),
        };
    }
}

fn cli() -> Command {
    let mount_arg = arg!(mount: -m "Mount Pico disk");
    let dev_arg = arg!(-d <PICO_DEV> "Path to Pico disk device").default_value("/dev/sda1");
    let line_arg = arg!(<LINE> "Power line").required(true);
    let line_arg = if platform::AUX_SWITCH {
        line_arg.value_parser([
            PossibleValue::new("aux"),
            PossibleValue::new("vdd"),
            PossibleValue::new("usb"),
        ])
    } else {
        line_arg.value_parser([PossibleValue::new("vdd"), PossibleValue::new("usb")])
    };

    Command::new("upico")
        .about("uPico control app")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("service").about("Start service").hide(true))
        .subcommand(Command::new("reset").about("Reset Pico"))
        .subcommand(
            Command::new("boot")
                .arg(mount_arg.clone())
                .arg(dev_arg.clone())
                .about("Reset Pico and enter USB bootloader"),
        )
        .subcommand(
            Command::new("gpio")
                .about("GPIO utils")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get")
                        .arg(arg!([PIN] "GPIO pin number (0-15)."))
                        .about("Print GPIO status"),
                )
                .subcommand(
                    Command::new("set")
                        .arg(
                            arg!(<CONFIG> "Comma separated GPIO config (0=1,3=0,7=i,..).")
                                .value_delimiter(',')
                                .required(true),
                        )
                        .about("Set GPIO config"),
                )
                .subcommand(
                    Command::new("led")
                        .arg(arg!(<STATUS> "LED status (on, off).").required(true))
                        .about("Set LED status"),
                )
                .subcommand(
                    Command::new("install")
                        .arg(
                            arg!(-p <PICO_PATH> "Path to mounted Pico disk").default_value(
                                if Path::new("/home/cpi").exists() {
                                    "/media/cpi/RPI-RP2"
                                } else {
                                    "/media/pi/RPI-RP2"
                                },
                            ),
                        )
                        .arg(mount_arg.clone())
                        .arg(dev_arg.clone())
                        .about("Install GPIO extender firmware to Pico"),
                ),
        )
        .subcommand(
            Command::new("install")
                .about("Install firmware to Pico")
                .arg_required_else_help(true)
                .arg(arg!(<FIRMWARE> "Path to UF2 firmware file").required(true))
                .arg(
                    arg!(-p <PICO_PATH> "Path to mounted Pico disk").default_value(
                        if Path::new("/home/cpi").exists() {
                            "/media/cpi/RPI-RP2"
                        } else {
                            "/media/pi/RPI-RP2"
                        },
                    ),
                )
                .arg(mount_arg)
                .arg(dev_arg),
        )
        .subcommand(
            Command::new("power")
                .about("Power management")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("on").about("Power on").arg(line_arg.clone()))
                .subcommand(Command::new("off").about("Power off").arg(line_arg.clone()))
                .subcommand(Command::new("cycle").about("Power cycle").arg(line_arg))
                .subcommand(
                    Command::new("status")
                        .about("Print power status")
                        .hide(!platform::OCP_REPORTING),
                ),
        )
        .subcommand(
            Command::new("pinout")
                .arg(arg!(full: -f "Print pin functions"))
                .about("Print pinout diagram"),
        )
}

fn print_power_state(line: &str, state: PowerState) {
    println!(
        "{line}:  {} {}",
        if state.on { "ON " } else { "OFF" },
        if state.ocp { "[OCP]" } else { "" }
    );
}

fn parse_power_line(cmd: &ArgMatches) -> Result<PowerLine, AppError> {
    cmd.get_one::<String>("LINE")
        .unwrap()
        .try_into()
        .map_err(|_| AppError::InvalidLine)
}

fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn wait_for_path(path: &Path) {
    for _ in 0..100 {
        if path.exists() {
            sleep(200);
            break;
        }
        sleep(100);
    }
}

fn mount_pico(disk: &str) -> Result<String, AppError> {
    wait_for_path(Path::new(disk));
    for _ in 0..50 {
        if let Ok(output) = process::Command::new("udisksctl")
            .args(["mount", "-b", disk])
            .stdout(process::Stdio::piped())
            .output()
        {
            if output.status.success() {
                let res = String::from_utf8(output.stdout).map_err(AppError::DecodeError)?;
                return res
                    .split(" at ")
                    .last()
                    .map(|s| s.trim().to_owned())
                    .ok_or(AppError::MountFailed);
            }
            sleep(100);
        }
    }
    Err(AppError::MountFailed)
}

fn run() -> AppResult {
    match cli().get_matches().subcommand() {
        Some(("service", _)) => Service::start()?,
        Some(("pinout", cmd)) => {
            if cmd.get_flag("full") {
                println!("{}", include_str!("resources/pinout_full.ansi"));
            } else {
                println!("{}", include_str!("resources/pinout.ansi"));
            }
        }
        Some(("reset", _)) => {
            Service::send(Request::Reset)?;
        }
        Some(("boot", cmd)) => {
            Service::send(Request::EnterBootloader)?;
            if cmd.get_flag("mount") {
                let disk = cmd.get_one::<String>("PICO_DEV").unwrap();
                mount_pico(disk)?;
            }
        }
        Some(("install", cmd)) => {
            Service::send(Request::EnterBootloader)?;
            let mut path = if cmd.get_flag("mount") {
                let disk = cmd.get_one::<String>("PICO_DEV").unwrap();
                mount_pico(disk)?
            } else {
                let path = cmd.get_one::<String>("PICO_PATH").unwrap().to_string();
                wait_for_path(Path::new(&path));
                path
            };
            path.push_str("/fw.uf2");
            let firmware = cmd.get_one::<String>("FIRMWARE").unwrap();
            fs::copy(firmware, path).map_err(AppError::IoError)?;
        }
        Some(("power", cmd)) => match cmd.subcommand() {
            Some(("on", cmd)) => {
                let line = parse_power_line(cmd)?;
                Service::send(Request::PowerOn(line))?;
            }
            Some(("off", cmd)) => {
                let line = parse_power_line(cmd)?;
                Service::send(Request::PowerOff(line))?;
            }
            Some(("cycle", cmd)) => {
                let line = parse_power_line(cmd)?;
                Service::send(Request::PowerCycle(line))?;
            }
            Some(("status", _)) if platform::OCP_REPORTING => {
                if let Response::PowerReport(report) = Service::send(Request::PowerStatus)? {
                    print_power_state("AUX", report.aux);
                    print_power_state("VDD", report.vdd);
                    print_power_state("USB", report.usb);
                }
            }
            _ => {}
        },
        Some(("gpio", cmd)) => match cmd.subcommand() {
            Some(("set", cmd)) => {
                let mut gpio_state = Extender::read_digital().map_err(AppError::UsbError)?;
                if let Some(configs) = cmd.get_many::<String>("CONFIG") {
                    for pin_config in configs {
                        if let Some((pin, mode)) = pin_config.split_once('=') {
                            let pin: u8 = pin.parse().map_err(AppError::ParseIntError)?;
                            if pin >= 16 {
                                return Err(AppError::InvalidGpioLine);
                            }
                            match mode {
                                "i" => gpio_state.set_mode(pin, false),
                                "0" => {
                                    gpio_state.set_mode(pin, true);
                                    gpio_state.set_level(pin, false);
                                }
                                "1" => {
                                    gpio_state.set_mode(pin, true);
                                    gpio_state.set_level(pin, true);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Extender::write_digital(gpio_state).map_err(AppError::UsbError)?;
            }
            Some(("get", cmd)) => {
                if let Some(pin) = cmd
                    .get_one::<String>("PIN")
                    .map(|s| s.parse::<u8>().unwrap_or_default())
                {
                    match pin {
                        0..=15 => {
                            let gpio_state =
                                Extender::read_digital().map_err(AppError::UsbError)?;
                            let level = if gpio_state.get_level(pin) { "1" } else { "0" };
                            println!("{}", level);
                        }
                        26..=29 => {
                            let values = Extender::read_analog().map_err(AppError::UsbError)?;
                            println!("{}", values[pin as usize - 26]);
                        }
                        _ => return Err(AppError::InvalidGpioLine),
                    }
                } else {
                    let gpio_state = Extender::read_digital().map_err(AppError::UsbError)?;
                    for pin in 0..16 {
                        let level = if gpio_state.get_level(pin) { "1" } else { "0" };
                        let mode = if gpio_state.get_mode(pin) {
                            "Output"
                        } else {
                            "Input"
                        };
                        println!("GPIO{}\t{}\t{}", pin, mode, level);
                    }
                    let values = Extender::read_analog().map_err(AppError::UsbError)?;
                    for (idx, val) in values.iter().enumerate() {
                        println!("GPIO{}\tAnalog\t{}", idx + 26, val);
                    }
                }
            }
            Some(("led", cmd)) => match cmd.get_one::<String>("STATUS") {
                Some(status) => match status.as_str() {
                    "on" => Extender::set_led(true).map_err(AppError::UsbError)?,
                    "off" => Extender::set_led(false).map_err(AppError::UsbError)?,
                    _ => return Err(AppError::InvalidLedMode),
                },
                _ => return Err(AppError::InvalidLedMode),
            },

            Some(("install", cmd)) => {
                Service::send(Request::EnterBootloader)?;
                let mut path = if cmd.get_flag("mount") {
                    let disk = cmd.get_one::<String>("PICO_DEV").unwrap();
                    mount_pico(disk)?
                } else {
                    let path = cmd.get_one::<String>("PICO_PATH").unwrap().to_string();
                    wait_for_path(Path::new(&path));
                    path
                };
                path.push_str("/fw.uf2");
                fs::write(path, include_bytes!("resources/extender.uf2"))
                    .map_err(AppError::IoError)?;
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}
