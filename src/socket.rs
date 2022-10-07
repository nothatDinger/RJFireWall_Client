

use neli::{
consts::{nl::*, socket::*},
genl::{Genlmsghdr, Nlattr},
nl::{Nlmsghdr, NlPayload},
socket::{self},
types::{Buffer, GenlBuffer}, neli_enum, attr::AttrHandle,
};
use std::process;
#[neli_enum(serialized_type = "u16")]
pub enum NlFoobarXmplAttribute {
    Unspec = 0,
    Msg = 1,
}

impl neli::consts::genl::NlAttrType for NlFoobarXmplAttribute {}
#[neli_enum(serialized_type = "u8")]
pub enum NlFoobarXmplOperation {
    Unspec = 0,
    Echo = 1,
}

impl neli::consts::genl::Cmd for NlFoobarXmplOperation {}

fn exchange_message(smsg: &String) -> String {
    let mut s = socket::NlSocketHandle::connect(
        NlFamily::Generic,
        Some(0),
        &[],
        ).unwrap();
    
    let mut attrs: GenlBuffer<NlFoobarXmplAttribute, Buffer> = GenlBuffer::new();
    //let mut attr1 = Nlattr::new(true, false, 0, Vec::<u8>::new()).unwrap();
    //attrs.push(Nlattr::new(false, false, 1, "this is a string").unwrap();
    attrs.push(
        Nlattr::new(
            false,
            false,
            NlFoobarXmplAttribute::Msg,
            smsg.clone(),
        )
        .unwrap(),
    );
    let genmsghdr = Genlmsghdr::new(
        NlFoobarXmplOperation::Echo,
        1,
        attrs,
    );
    let nlmsghdr = Nlmsghdr::new(
        None,
        0,
        NlmFFlags::empty(),
        None,
        Some(process::id()),
        NlPayload::Payload(genmsghdr),
    );
    println!("Send to kernel: '{}'", &smsg);

    s.send(nlmsghdr).expect("Send doesn't work");

    let res: Nlmsghdr<u16, Genlmsghdr<NlFoobarXmplOperation, NlFoobarXmplAttribute>> =
        s.recv().expect("should receive a message").unwrap(); 
    
    let attr_handle = res.get_payload().unwrap().get_attr_handle();
    let received: String = attr_handle
        .get_attr_payload_as_with_len::<String>(NlFoobarXmplAttribute::Msg)
        .unwrap();
    received
}