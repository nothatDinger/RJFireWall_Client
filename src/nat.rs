use std::str::FromStr;
use ipnetwork::Ipv4Network;
use crate::common::{MyParseError};
#[derive(Debug)]
pub struct NatRule{
   pub src_net : Ipv4Network,
   pub dst_ip : Ipv4Network, 
   pub min_port: u16,
   pub max_port: u16,
}

impl FromStr for NatRule {
    type Err = MyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.trim_start_matches("NatRule: ").split_whitespace().collect();
        let src_net = words[0].parse::<Ipv4Network>()?;
        let dst_ip = words[1].parse::<Ipv4Network>()?;
        let min_port = words[2].parse::<u16>()?;
        let max_port = words[3].parse::<u16>()?;
        Ok( NatRule{ src_net, dst_ip, min_port, max_port })
    }
}

impl std::fmt::Display for NatRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",format!("NatRule: {} {}      {}      {}     ",
                self.src_net, self.dst_ip, self.min_port, self.max_port,)
        )
    }
}
