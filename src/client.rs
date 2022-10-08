

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IPRule {
    pub name: [::std::os::raw::c_char; 12usize],
    pub saddr: ::std::os::raw::c_uint,
    pub smask: ::std::os::raw::c_uint,
    pub daddr: ::std::os::raw::c_uint,
    pub dmask: ::std::os::raw::c_uint,
    pub sport: ::std::os::raw::c_uint,
    pub dport: ::std::os::raw::c_uint,
    pub protocol: u8,
    pub action: ::std::os::raw::c_uint,
    pub log: ::std::os::raw::c_uint,
    pub nx: *mut IPRule,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IPLog {
    pub tm: ::std::os::raw::c_long,
    pub saddr: ::std::os::raw::c_uint,
    pub daddr: ::std::os::raw::c_uint,
    pub sport: ::std::os::raw::c_ushort,
    pub dport: ::std::os::raw::c_ushort,
    pub protocol: u8,
    pub len: ::std::os::raw::c_uint,
    pub action: ::std::os::raw::c_uint,
    pub nx: *mut IPLog,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConnLog {
    pub saddr: ::std::os::raw::c_uint,
    pub daddr: ::std::os::raw::c_uint,
    pub sport: ::std::os::raw::c_ushort,
    pub dport: ::std::os::raw::c_ushort,
    pub protocol: u8,
    pub natType: ::std::os::raw::c_int,
    pub nat: NATRecord,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NATRecord {
    pub saddr: ::std::os::raw::c_uint,
    pub smask: ::std::os::raw::c_uint,
    pub daddr: ::std::os::raw::c_uint,
    pub sport: ::std::os::raw::c_ushort,
    pub dport: ::std::os::raw::c_ushort,
    pub nowPort: ::std::os::raw::c_ushort,
    pub nx: *mut NATRecord,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernelResponseHeader {
    pub bodyTp: ::std::os::raw::c_uint,
    pub arrayLen: ::std::os::raw::c_uint,
}

#[doc = " @brief 内核回应包"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernelResponse {
    pub code: ::std::os::raw::c_int,
    pub data: *mut ::std::os::raw::c_void,
    pub header: *mut KernelResponseHeader,
    pub body: *mut ::std::os::raw::c_void,
}

extern "C" {
    #[doc = " @brief 与内核交换数据"]
    #[doc = " @param smsg: 发送的消息"]
    #[doc = " @param slen: 发送消息的长度"]
    #[doc = " @return KernelResponse: 接收到的回应，其中data字段记得free"]
    pub fn exchangeMsgK(
        smsg: *mut ::std::os::raw::c_void,
        slen: ::std::os::raw::c_uint,
    ) -> KernelResponse;
}
extern "C" {
    pub fn addFilterRule(
        after: *mut ::std::os::raw::c_char,
        name: *mut ::std::os::raw::c_char,
        sip: *mut ::std::os::raw::c_char,
        dip: *mut ::std::os::raw::c_char,
        sport: ::std::os::raw::c_uint,
        dport: ::std::os::raw::c_uint,
        proto: ::std::os::raw::c_uchar,
        log: ::std::os::raw::c_uint,
        action: ::std::os::raw::c_uint,
    ) -> KernelResponse;
}
extern "C" {
    pub fn delFilterRule(name: *mut ::std::os::raw::c_char) -> KernelResponse;
}
extern "C" {
    pub fn getAllFilterRules() -> KernelResponse;
}
extern "C" {
    pub fn addNATRule(
        sip: *mut ::std::os::raw::c_char,
        natIP: *mut ::std::os::raw::c_char,
        minport: ::std::os::raw::c_ushort,
        maxport: ::std::os::raw::c_ushort,
    ) -> KernelResponse;
}
extern "C" {
    pub fn delNATRule(num: ::std::os::raw::c_int) -> KernelResponse;
}
extern "C" {
    pub fn getAllNATRules() -> KernelResponse;
}
extern "C" {
    pub fn setDefaultAction(action: ::std::os::raw::c_uint) -> KernelResponse;
}
extern "C" {
    pub fn getLogs(num: ::std::os::raw::c_uint) -> KernelResponse;
}
extern "C" {
    pub fn getAllConns() -> KernelResponse;
}
extern "C" {
    pub fn IPstr2IPint(
        ipStr: *const ::std::os::raw::c_char,
        ip: *mut ::std::os::raw::c_uint,
        mask: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn IPint2IPstr(
        ip: ::std::os::raw::c_uint,
        mask: ::std::os::raw::c_uint,
        ipStr: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn IPint2IPstrNoMask(
        ip: ::std::os::raw::c_uint,
        ipStr: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn IPint2IPstrWithPort(
        ip: ::std::os::raw::c_uint,
        port: ::std::os::raw::c_ushort,
        ipStr: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn showRules(rules: *mut IPRule, len: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn showNATRules(rules: *mut NATRecord, len: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn showLogs(logs: *mut IPLog, len: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn showConns(logs: *mut ConnLog, len: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dealResponseAtCmd(rsp: KernelResponse);
}

use std::fmt::Debug;

use std::io::stdin;
use std::ffi::CString;

pub fn cmd_add_rule(r: crate::rule::Rule) -> KernelResponse{
    println!("please name the rule:");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();

    let after = CString::new("\n").expect("CString::new failed");
    let name = CString::new(name.trim()).expect("CString::new failed");
    let src_net = CString::new(r.src_net.to_string()).expect("CString::new failed");
    let dst_net = CString::new(r.dst_net.to_string()).expect("CString::new failed");
    unsafe{
        addFilterRule(
        after.into_raw(),
        name.into_raw() ,
        src_net.into_raw() ,
        dst_net.into_raw(),
        (r.src_port_min << 16) | (r.src_port_max & 0xffff),
        (r.dst_port_min << 16) | (r.dst_port_max & 0xffff),
        r.protocol.into(), 
        r.log,
        r.action)
    }    
}

pub fn cmd_add_nat_rule(r: crate::nat::NatRule)-> KernelResponse{
    
    let sip = CString::new(r.src_net.to_string()).unwrap();
    let nat_ip = CString::new(r.dst_ip.to_string()).unwrap();
    unsafe{
        addNATRule(sip.into_raw(), nat_ip.into_raw(), r.min_port, r.max_port)
    }
}

// pub const RSP_ONLY_HEAD: u32 = 10;
// pub const RSP_MSG: u32 = 11;
// pub const RSP_IPRULES: u32 = 12;
// pub const RSP_IPLOGS: u32 = 13;
// pub const RSP_NATRULES: u32 = 14;
// pub const RSP_CONN_LOGS: u32 = 15;
// pub fn show_result(r: KernelResponse){
//     if r.code < 0 {
//         return;
//     }
//     unsafe{
//         match (*r.header).bodyTp {
//             RSP_ONLY_HEAD => {
//                 println!("succeeded to delete {} rules.\n",(*r.header).arrayLen);
//             }
//             RSP_MSG => {
//                 println!("From Kernel: {}\n",Box::from_raw(r.body as *mut &str));
//             }
//             RSP_IPRULES => {
//                 showRules(r.body, (*r.header).arrayLen)
//             }
//             RSP_NATRULES => {
            
//             }
//             RSP_CONN_LOGS => {
            
//             }
//             _ => {
            
//             }
//         }
//     }
// }