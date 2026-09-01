#![allow(non_camel_case_types)]

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>

pub const SK_PASS: i32 = 1;

#[repr(C)]
pub struct sk_msg_md {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = "sk_msg1"]
pub unsafe extern "C" fn bpf_prog1(msg: *mut sk_msg_md) -> i32 {
    let _ = msg;
    SK_PASS
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
