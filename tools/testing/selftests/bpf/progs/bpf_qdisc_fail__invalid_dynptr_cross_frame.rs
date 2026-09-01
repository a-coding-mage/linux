// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include "bpf_experimental.h"
// #include "bpf_qdisc_common.h"
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::ptr;

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

pub const NET_XMIT_DROP: i32 = 1;

unsafe extern "C" {
    fn bpf_kfree_skb(skb: *mut sk_buff);
    fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice(
        ptr: *const bpf_dynptr,
        offset: u32,
        buffer: *mut c_void,
        buffer__sz: u32,
    ) -> *mut c_void;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut proto: i32 = 0;

#[inline(never)]
unsafe extern "C" fn free_skb(skb: *mut sk_buff) -> i32 {
    unsafe {
        bpf_kfree_skb(skb);
    }
    0
}

// SEC("struct_ops")
// __failure __msg("invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalid_dynptr_cross_frame(
    skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut bpf_sk_buff_ptr,
) -> i32 {
    let mut ptr: bpf_dynptr = unsafe { core::mem::zeroed() };
    let mut hdr: *mut ethhdr;

    let _ = sch;
    let _ = to_free;

    unsafe {
        bpf_dynptr_from_skb(skb as *mut __sk_buff, 0, &mut ptr);
    }

    hdr = unsafe {
        bpf_dynptr_slice(
            &ptr,
            0,
            ptr::null_mut(),
            core::mem::size_of::<ethhdr>() as u32,
        ) as *mut ethhdr
    };
    if hdr.is_null() {
        return NET_XMIT_DROP;
    }

    unsafe {
        free_skb(skb);
    }

    unsafe {
        proto = (*hdr).h_proto as i32;
    }

    NET_XMIT_DROP
}

// SEC("struct_ops")
// __auxiliary
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_qdisc_test_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let _ = sch;
    ptr::null_mut()
}

// SEC("struct_ops")
// __auxiliary
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_qdisc_test_init(
    sch: *mut Qdisc,
    opt: *mut nlattr,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let _ = sch;
    let _ = opt;
    let _ = extack;
    0
}

// SEC("struct_ops")
// __auxiliary
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_qdisc_test_reset(sch: *mut Qdisc) {
    let _ = sch;
}

// SEC("struct_ops")
// __auxiliary
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_qdisc_test_destroy(sch: *mut Qdisc) {
    let _ = sch;
}

// SEC(".struct_ops")
#[unsafe(link_section = ".struct_ops")]
#[unsafe(no_mangle)]
pub static mut test: Qdisc_ops = Qdisc_ops {
    enqueue: invalid_dynptr_cross_frame as *mut c_void,
    dequeue: bpf_qdisc_test_dequeue as *mut c_void,
    init: bpf_qdisc_test_init as *mut c_void,
    reset: bpf_qdisc_test_reset as *mut c_void,
    destroy: bpf_qdisc_test_destroy as *mut c_void,
    id: b"bpf_qdisc_test\0".as_ptr(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
