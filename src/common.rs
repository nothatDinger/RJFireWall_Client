use std::{fmt,str::FromStr,error::Error, num::ParseIntError};
use ipnetwork::IpNetworkError;
use num_enum::IntoPrimitive;
#[derive(Debug,IntoPrimitive)]
#[repr(u8)]
pub enum TransportType {
    Tcp=0,
    Udp,
    Icmp,
}
impl Default for TransportType {
    fn default() -> TransportType {
        TransportType::Tcp
    }
}
impl fmt::Display for TransportType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransportType::Tcp => write!(f,"TCP"),
            TransportType::Udp => write!(f,"UDP"),
            TransportType::Icmp => write!(f,"ICMP")
        }
    }
}
#[derive(Debug)]
pub struct ParseTransportTypeError;
impl fmt::Display for ParseTransportTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "invalid TransportType")
    }
}
impl Error for ParseTransportTypeError {
    fn description(&self) -> &str {
        "Invalid TransportType"
    }
}
impl FromStr for TransportType {
    type Err = ParseTransportTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {   
        match s {
            "TCP" => Ok(TransportType::Tcp),
            "UDP" => Ok(TransportType::Udp),
            "ICMP" => Ok(TransportType::Icmp),
            _      => Err(ParseTransportTypeError)
        }
    }
}
#[derive(Debug)]
pub enum MyParseError{
    InvalidTransportType(ParseTransportTypeError),
    IpNetworkError(IpNetworkError),
    ParseIntError(ParseIntError),
}

impl fmt::Display for MyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::common::MyParseError::*;
        match *self {
            InvalidTransportType(ref s) => write!(f, "invalid transport type: {}", s),
            IpNetworkError(ref s) => write!(f, "invalid ip and mask: {}", s),
            ParseIntError(ref s) => write!(f, "str to int error: {}", s),
        }
    }
}

impl Error for MyParseError {
    fn description(&self) -> &str {
        use crate::common::MyParseError::*;
        match *self {
            InvalidTransportType(_) => "address is invalid",
            IpNetworkError(_) => "prefix is invalid",
            ParseIntError(_) => "cidr is invalid",
        }
    }
}

impl From<ParseTransportTypeError> for MyParseError {
    fn from(error: ParseTransportTypeError) -> Self {
        MyParseError::InvalidTransportType(error)
    }
}
impl From<IpNetworkError> for MyParseError {
    fn from(error: IpNetworkError) -> Self {
        MyParseError::IpNetworkError(error)
    }
}
impl From<ParseIntError> for MyParseError {
    fn from(error: ParseIntError) -> Self {
        MyParseError::ParseIntError(error)
    }
}