// SPDX-License-Identifier: GPL-2.0

// Rust translation of dependencies originally included from:
// <vmlinux.h>, "bpf_experimental.h", "bpf_qdisc_common.h", "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::ptr;

const NET_XMIT_DROP: i32 = 1;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
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
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
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
pub struct Qdisc_ops {
    pub enqueue: *mut c_void,
    pub dequeue: *mut c_void,
    pub init: *mut c_void,
    pub reset: *mut c_void,
    pub destroy: *mut c_void,
    pub id: *const u8,
}

extern "C" {
    fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_clone(ptr: *mut bpf_dynptr, clone: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice(
        ptr: *mut bpf_dynptr,
        offset: u32,
        buffer: *mut c_void,
        buffer__sz: u32,
    ) -> *mut c_void;
    fn bpf_qdisc_skb_drop(skb: *mut sk_buff, to_free: *mut bpf_sk_buff_ptr);
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut proto: i32 = 0;

// SEC("struct_ops")
// __success
#[no_mangle]
pub unsafe extern "C" fn dynptr_use_after_invalidate_clone(
    skb: *mut sk_buff,
    _sch: *mut Qdisc,
    to_free: *mut bpf_sk_buff_ptr,
) -> i32 {
    let mut ptr: bpf_dynptr = core::mem::zeroed();
    let mut ptr_clone: bpf_dynptr = core::mem::zeroed();
    let mut hdr: *mut ethhdr;

    bpf_dynptr_from_skb(skb as *mut __sk_buff, 0, &mut ptr);

    bpf_dynptr_clone(&mut ptr, &mut ptr_clone);

    hdr = bpf_dynptr_slice(
        &mut ptr_clone,
        0,
        ptr::null_mut(),
        core::mem::size_of::<ethhdr>() as u32,
    ) as *mut ethhdr;
    if hdr.is_null() {
        bpf_qdisc_skb_drop(skb, to_free);
        return NET_XMIT_DROP;
    }

    *(&mut ptr as *mut bpf_dynptr as *mut i32) = 0;

    proto = (*hdr).h_proto as i32;

    bpf_qdisc_skb_drop(skb, to_free);

    NET_XMIT_DROP
}

// SEC("struct_ops")
// __auxiliary
#[no_mangle]
pub unsafe extern "C" fn bpf_qdisc_test_dequeue(_sch: *mut Qdisc) -> *mut sk_buff {
    ptr::null_mut()
}

// SEC("struct_ops")
// __auxiliary
#[no_mangle]
pub unsafe extern "C" fn bpf_qdisc_test_init(
    _sch: *mut Qdisc,
    _opt: *mut nlattr,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    0
}

// SEC("struct_ops")
// __auxiliary
#[no_mangle]
pub unsafe extern "C" fn bpf_qdisc_test_reset(_sch: *mut Qdisc) {}

// SEC("struct_ops")
// __auxiliary
#[no_mangle]
pub unsafe extern "C" fn bpf_qdisc_test_destroy(_sch: *mut Qdisc) {}

#[link_section = ".struct_ops"]
#[no_mangle]
pub static mut test: Qdisc_ops = Qdisc_ops {
    enqueue: dynptr_use_after_invalidate_clone as *mut c_void,
    dequeue: bpf_qdisc_test_dequeue as *mut c_void,
    init: bpf_qdisc_test_init as *mut c_void,
    reset: bpf_qdisc_test_reset as *mut c_void,
    destroy: bpf_qdisc_test_destroy as *mut c_void,
    id: b"bpf_qdisc_test\0".as_ptr(),
};
