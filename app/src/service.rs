use crate::*;
use rmp_serde::*;
use serde::*;
use std::{
    io::{ErrorKind, Read, Write},
    os::unix::{net::*, prelude::PermissionsExt},
    time::Duration,
};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PowerLine {
    Aux,
    Vdd,
    Usb,
}

impl TryFrom<&String> for PowerLine {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let line = match value.to_lowercase().as_str() {
            "aux" => Self::Aux,
            "vdd" => Self::Vdd,
            "usb" => Self::Usb,
            _ => return Err(()),
        };
        Ok(line)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PowerState {
    pub on: bool,
    pub ocp: bool,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PowerReport {
    pub aux: PowerState,
    pub vdd: PowerState,
    pub usb: PowerState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Reset,
    EnterBootloader,
    PowerStatus,
    PowerOn(PowerLine),
    PowerCycle(PowerLine),
    PowerOff(PowerLine),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Response {
    Done,
    PowerReport(PowerReport),
}

pub struct Service {
    gpio: Gpio,
}

impl Service {
    const SOCKET: &'static str = "/tmp/upico.sock";

    pub fn start(chip: &str, pins: GpioPins) -> AppResult {
        let err = UnixStream::connect(Service::SOCKET).map_err(|err| err.kind());
        if let Err(ErrorKind::ConnectionRefused) = err {
            fs::remove_file(Service::SOCKET).map_err(AppError::ServiceError)?
        }
        let listener = UnixListener::bind(Service::SOCKET).map_err(AppError::ServiceError)?;
        let mut perms = fs::metadata(Service::SOCKET)
            .map_err(AppError::IoError)?
            .permissions();
        perms.set_mode(0o766);
        fs::set_permissions(Service::SOCKET, perms).map_err(AppError::IoError)?;

        let gpio = Gpio::new(chip, pins).map_err(AppError::GpioError)?;
        let mut service = Self { gpio };

        let mut scratch = [0; 64];
        for mut stream in listener.incoming().flatten() {
            stream
                .read(&mut scratch)
                .map_err(AppError::ServiceError)
                .and_then(|n| {
                    from_slice::<Request>(&scratch[0..n]).map_err(AppError::ProtocolError)
                })
                .map(|req| service.request(req))
                .map(|res| to_vec(&res).unwrap())
                .map(|packet| stream.write(&packet))
                .ok();
        }
        Ok(())
    }

    pub fn send(req: Request) -> Result<Response, AppError> {
        let mut stream = UnixStream::connect(Service::SOCKET).map_err(AppError::ServiceError)?;

        let packet = to_vec(&req).unwrap();
        stream.write(&packet).map_err(AppError::IoError)?;
        thread::sleep(Duration::from_millis(250));

        let mut scratch = [0; 64];
        let n = stream.read(&mut scratch).map_err(AppError::ServiceError)?;
        from_slice(&scratch[0..n]).map_err(AppError::ProtocolError)
    }

    fn request(&mut self, req: Request) -> Response {
        match req {
            Request::PowerOn(line) => self.gpio.power_on(line),
            Request::PowerOff(line) => self.gpio.power_off(line),
            Request::PowerCycle(line) => self.gpio.power_cycle(line),
            Request::Reset => self.gpio.reset_pico(false),
            Request::EnterBootloader => self.gpio.reset_pico(true),
            Request::PowerStatus => {
                let report = self.gpio.power_report();
                return Response::PowerReport(report);
            }
        }
        Response::Done
    }
}
