use app::*;
use clap::*;
use gpio::*;
use service::*;
use std::*;

mod app;
mod gpio;
mod service;

fn main() {
    if let Err(err) = run() {
        match err {
            AppError::InvalidLine => println!("Invalid power line name"),
            AppError::MountFailed => println!("Failed to mount Pico drive"),
            AppError::GpioError(err) => println!("GPIO error: {}", err),
            AppError::IoError(err) => println!("IO error: {}", err),
            AppError::ServiceError(err) => println!("Service error: {}", err),
            AppError::DecodeError(err) => println!("Decode error: {}", err),
            AppError::ProtocolError(err) => println!("Protocol error: {}", err),
        };
    }
}

fn cli() -> Command {
    let mount_arg = arg!(mount: -m "Mount Pico disk");
    let dev_arg = arg!(-d <PICO_DEV> "Path to Pico disk device").default_value("/dev/sda1");
    let line_arg = arg!(<LINE> "Power line (AUX, VDD, USB)").required(true);
    Command::new("upico")
        .about("uPico control app")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("reset").about("Reset Pico"))
        .subcommand(
            Command::new("boot")
                .arg(mount_arg.clone())
                .arg(dev_arg.clone())
                .about("Reset Pico and enter USB bootloader"),
        )
        .subcommand(
            Command::new("flash")
                .about("Flash microcontroller")
                .arg(arg!(<FIRMWARE> "Path to UF2 firmware file").required(true))
                .arg(
                    arg!(-p <PICO_PATH> "Path to mounted Pico disk")
                        .required_unless_present("mount"),
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
                .subcommand(Command::new("status").about("Power status")),
        )
        .subcommand(
            Command::new("service")
                .about("Start service")
                .arg(arg!([CHIP] "GPIO chip device").default_value("/dev/gpiochip0")),
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

fn run() -> AppResult {
    match cli().get_matches().subcommand() {
        Some(("service", cmd)) => {
            let gpio_chip = cmd.get_one::<String>("CHIP").unwrap();
            Service::start(gpio_chip)?;
        }
        Some(("reset", _)) => {
            Service::send(Request::Reset)?;
        }
        Some(("boot", cmd)) => {
            Service::send(Request::EnterBootloader)?;
            if cmd.get_flag("mount") {
                let disk = cmd.get_one::<String>("PICO_DEV").unwrap();
                mount_pico_dev(disk)?;
            }
        }
        Some(("flash", cmd)) => {
            Service::send(Request::EnterBootloader)?;
            let firmware = cmd.get_one::<String>("FIRMWARE").unwrap();
            let path = if cmd.get_flag("mount") {
                let disk = cmd.get_one::<String>("PICO_DEV").unwrap();
                let mut path = mount_pico_dev(disk)?;
                path.push_str("/fw.uf2");
                path
            } else {
                cmd.get_one::<String>("PICO_PATH").unwrap().to_string()
            };
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
            Some(("status", _)) => {
                if let Response::PowerReport(report) = Service::send(Request::PowerStatus)? {
                    print_power_state("AUX", report.aux);
                    print_power_state("VDD", report.vdd);
                    print_power_state("USB", report.usb);
                }
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}
