// SPDX-License-Identifier: GPL-2.0

// Dependencies from C includes:
// <vmlinux.h>, "bpf_experimental.h", "bpf_qdisc_common.h", "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_void};
use core::ptr;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Qdisc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sk_buff_ptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Qdisc_ops {
    pub enqueue: *mut c_void,
    pub dequeue: *mut c_void,
    pub init: *mut c_void,
    pub reset: *mut c_void,
    pub destroy: *mut c_void,
    pub id: *const c_char,
}

pub const NET_XMIT_DROP: i32 = 1;

unsafe extern "C" {
    fn bpf_dynptr_from_skb(
        skb: *mut __sk_buff,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_qdisc_skb_drop(skb: *mut sk_buff, to_free: *mut bpf_sk_buff_ptr);
    fn bpf_dynptr_slice(
        ptr: *const bpf_dynptr,
        offset: u32,
        buffer__opt: *mut c_void,
        buffer__szk: u32,
    ) -> *mut c_void;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(no_mangle)]
pub static mut proto: i32 = 0;

// SEC("struct_ops")
// __failure __msg("Expected an initialized dynptr as R1")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalid_dynptr(
    skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut bpf_sk_buff_ptr,
) -> i32 {
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut hdr: *mut ethhdr;

    bpf_dynptr_from_skb(skb as *mut __sk_buff, 0, ptr.as_mut_ptr());

    bpf_qdisc_skb_drop(skb, to_free);

    hdr = bpf_dynptr_slice(
        ptr.as_ptr(),
        0,
        ptr::null_mut(),
        core::mem::size_of::<ethhdr>() as u32,
    ) as *mut ethhdr;
    if hdr.is_null() {
        return NET_XMIT_DROP;
    }

    proto = (*hdr).h_proto as i32;

    NET_XMIT_DROP
}

// SEC("struct_ops")
// __auxiliary
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_qdisc_test_dequeue(_sch: *mut Qdisc) -> *mut sk_buff {
    ptr::null_mut()
}

// SEC("struct_ops")
// __auxiliary
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_qdisc_test_init(
    _sch: *mut Qdisc,
    _opt: *mut nlattr,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    0
}

// SEC("struct_ops")
// __auxiliary
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_qdisc_test_reset(_sch: *mut Qdisc) {}

// SEC("struct_ops")
// __auxiliary
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_qdisc_test_destroy(_sch: *mut Qdisc) {}

// SEC(".struct_ops")
#[unsafe(link_section = ".struct_ops")]
#[unsafe(no_mangle)]
pub static mut test: Qdisc_ops = Qdisc_ops {
    enqueue: invalid_dynptr as *mut c_void,
    dequeue: bpf_qdisc_test_dequeue as *mut c_void,
    init: bpf_qdisc_test_init as *mut c_void,
    reset: bpf_qdisc_test_reset as *mut c_void,
    destroy: bpf_qdisc_test_destroy as *mut c_void,
    id: b"bpf_qdisc_test\0".as_ptr() as *const c_char,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
