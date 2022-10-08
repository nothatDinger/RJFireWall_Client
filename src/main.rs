pub mod rule;
pub mod common;
pub mod socket;
pub mod client;
pub mod nat;
use clap::{Parser, Subcommand, Args};
use client::KernelResponse;

use crate::{rule::Rule, client::{cmd_add_rule, cmd_add_nat_rule, delFilterRule, delNATRule, setDefaultAction, getLogs, getAllFilterRules, getAllNATRules, getAllConns}, nat::NatRule};                                                                                                                                

#[derive(Parser)]
#[command(author="Dinger", version="1.0", about="a client for fw")]
pub struct Cli{
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands{
    //add a normal rule or nat rule to FireWall
    Add(Add),
    Del(Del),
    Ls(Ls),
    Default
}
#[derive(Args)]
#[command(about = "add a normal rule or nat rule to FireWall", 
          long_about = None,arg_required_else_help(true))]
pub struct Add {
    #[arg(short, long, action = clap::ArgAction::Set,
        help="<src_net> <dst_net> <src_port_min> <src_port_max> <dst_port_min> <dst_port_max> <protocol> <action> <log>")]
    rule: Option<String>,
    #[arg(short, long, action = clap::ArgAction::Set,
        help="<src_ip> <nat_ip>")]
    nat: Option<String>,

}
#[derive(Args)]
#[command(about = "delete a normal rule or nat rule to FireWall", 
          long_about = None,arg_required_else_help(true))]
pub struct Del{
    #[arg(short='r', long="rule", action = clap::ArgAction::Set,
            help="the name of the rule you want to delete")]
    rule: Option<String>,
    #[arg(short, long, action = clap::ArgAction::Append,
        help="the NO. of the NAT rule you want to delete")]
    nat: Option<i32>
}
#[derive(Args)]
#[command(about = "list the table of rule/nat/conncetion/log", 
          long_about = None,arg_required_else_help(true))]
pub struct Ls{
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    rule: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    nat: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    connection: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    log: bool
}
#[derive(Args)]
#[command(about = "set the rule to default", 
          long_about = None)]
pub struct Default{
}
fn main() {
    
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(add) => {
            if let Some(rule) = &add.rule{
                println!("add rule: {}", rule);
                let rule = rule.parse::<Rule>().unwrap();
                unsafe{
                    client::dealResponseAtCmd(
                        cmd_add_rule(rule)
                    );
                }
            }
            if let Some(nat) = &add.nat{
                println!("add nat rule: {}", nat);
                let nat = nat.parse::<NatRule>().unwrap();
                unsafe{
                    client::dealResponseAtCmd(
                        cmd_add_nat_rule(nat)
                    );
                }
            }

        }
        Commands::Del(del) =>{
            if let Some(name) = &del.rule{
                println!("del rule name: {}", name);
                let r = std::ffi::CString::new(name.to_string()).unwrap();
                unsafe{
                    client::dealResponseAtCmd(
                        delFilterRule(r.into_raw())
                    );
                }
            }
            if let Some(number) =&del.nat{
                println!("del nat number: {}", number);
                unsafe{
                    client::dealResponseAtCmd(
                    delNATRule(*number)
                 );
                }
            }
        }
        Commands::Ls(ls) => {
            println!("ls: {} {} {} {}",ls.rule,ls.nat,ls.log,ls.connection);
            unsafe{
                let rsp: KernelResponse = {
                    if ls.rule {
                        getAllFilterRules()
                    } else
                    if ls.nat{
                        getAllNATRules()
                    } else
                    if ls.log {
                        getLogs(32)
                    } else {
                        getAllConns()
                    }
                };
                client::dealResponseAtCmd(rsp);
            };
  
        }
        Commands::Default => {
            println!("default");
            unsafe{
                setDefaultAction(0);
            }
        }
    }
}
