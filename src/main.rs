
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crc::{crc16, Hasher16, crc32, Hasher32, crc64, Hasher64};
use rocket::http::{RawStr};
use rocket::request::{Form, FromFormValue};

#[derive(Debug)]
enum Crc16Polynomial {
    X25,
    Usb,
    Custom(u16),
}

impl<'v> FromFormValue<'v> for Crc16Polynomial {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Crc16Polynomial, &'v RawStr> {
        match form_value.as_str() {
            "usb" => Ok(Crc16Polynomial::Usb),
            "x25" => Ok(Crc16Polynomial::X25),
            custom => {
                match str::parse::<u16>(custom) {
                    Ok(n) => Ok(Crc16Polynomial::Custom(n)),
                    Err(_) => Err("unknown polynomial".into()),
                }
            },
        }
    }
}

#[derive(Debug, FromForm)]
struct Crc16Options {
    polynomial: Crc16Polynomial,
}

#[post("/crc16?<options..>", data = "<payload>")]
fn crc16(payload: Vec<u8>, options: Option<Form<Crc16Options>>) -> String {
    let checksum = if let Some(choice) = options {
        let polynomial = match choice.polynomial {
            Crc16Polynomial::X25 => crc::crc16::X25,
            Crc16Polynomial::Usb => crc::crc16::USB,
            Crc16Polynomial::Custom(n) => n,
        };

        let mut digest = crc16::Digest::new(polynomial);
        digest.write(payload.as_slice());
        digest.sum16()
    } else {
        crc16::checksum_usb(payload.as_slice())
    };
    format!("{}\r\n", checksum)
}


#[derive(Debug)]
enum Crc32Polynomial {
    Ieee,
    Castagnoli,
    Koopman,
    Custom(u32),
}

impl<'v> FromFormValue<'v> for Crc32Polynomial {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Crc32Polynomial, &'v RawStr> {
        match form_value.as_str() {
            "ieee" => Ok(Crc32Polynomial::Ieee),
            "castagnoli" => Ok(Crc32Polynomial::Castagnoli),
            "koopman" => Ok(Crc32Polynomial::Koopman),
            custom => {
                match str::parse::<u32>(custom) {
                    Ok(n) => Ok(Crc32Polynomial::Custom(n)),
                    Err(_) => Err("unknown polynomial".into()),
                }
            },
        }
    }
}

#[derive(Debug, FromForm)]
struct Crc32Options {
    polynomial: Crc32Polynomial,
}

#[post("/crc32?<options..>", data = "<payload>")]
fn crc32(payload: Vec<u8>, options: Option<Form<Crc32Options>>) -> String {
    let checksum = if let Some(choice) = options {
        let polynomial = match choice.polynomial {
            Crc32Polynomial::Ieee => crc::crc32::IEEE,
            Crc32Polynomial::Castagnoli => crc::crc32::CASTAGNOLI,
            Crc32Polynomial::Koopman => crc::crc32::KOOPMAN,
            Crc32Polynomial::Custom(n) => n,
        };

        let mut digest = crc32::Digest::new(polynomial);
        digest.write(payload.as_slice());
        digest.sum32()

    } else {
        crc32::checksum_ieee(payload.as_slice())
    };
    format!("{}\r\n", checksum)
}

#[derive(Debug)]
enum Crc64Polynomial {
    Ecma,
    Iso,
    Custom(u64),
}

impl<'v> FromFormValue<'v> for Crc64Polynomial {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Crc64Polynomial, &'v RawStr> {
        match form_value.as_str() {
            "ecma" => Ok(Crc64Polynomial::Ecma),
            "iso" => Ok(Crc64Polynomial::Iso),
            custom => {
                match str::parse::<u64>(custom) {
                    Ok(n) => Ok(Crc64Polynomial::Custom(n)),
                    Err(_) => Err("unknown polynomial".into()),
                }
            },
        }
    }
}

#[derive(Debug, FromForm)]
struct Crc64Options {
    polynomial: Crc64Polynomial,
}

#[post("/crc64?<options..>", data = "<payload>")]
fn crc64(payload: Vec<u8>, options: Option<Form<Crc64Options>>) -> String {
    let checksum = if let Some(choice) = options {
        let polynomial = match choice.polynomial {
            Crc64Polynomial::Ecma => crc::crc64::ECMA,
            Crc64Polynomial::Iso => crc::crc64::ISO,
            Crc64Polynomial::Custom(n) => n,
        };

        let mut digest = crc64::Digest::new(polynomial);
        digest.write(payload.as_slice());
        digest.sum64()
    } else {
        crc64::checksum_iso(payload.as_slice())
    };
    format!("{}\r\n", checksum)
}


fn main() {
    rocket::ignite().mount("/", routes![crc16, crc32, crc64]).launch();
}
