use std::str::FromStr;
use ipnetwork::Ipv4Network;
use crate::common::{TransportType, MyParseError};
#[derive(Debug)]
pub struct Rule{
   pub src_net : Ipv4Network,
   pub dst_net : Ipv4Network, 
   pub src_port: u32,
   pub dst_port: u32,
   pub protocol: TransportType,
   pub action: u32,
   pub log: u32,
}

impl FromStr for Rule {
    type Err = MyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.trim_start_matches("Rule: ").split_whitespace().collect();
        let src_net = words[0].parse::<Ipv4Network>()?;
        let dst_net = words[1].parse::<Ipv4Network>()?;
        let src_port = words[2].parse::<u32>()?;
        let dst_port = words[3].parse::<u32>()?;
        let protocol = words[4].parse::<TransportType>()?;
        let action = words[5].parse::<u32>()?;
        let log = words[6].parse::<u32>()?;
        Ok( Rule{ src_net, dst_net, src_port, dst_port, protocol, action, log })
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",format!("Rule: {} {}      {}      {}       {}      {}    {}",
                self.src_net, self.dst_net, self.src_port, self.dst_port,
                self.protocol, self.action, self.log)
        )
    }
}
