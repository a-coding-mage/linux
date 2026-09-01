// SPDX-License-Identifier: GPL-2.0

// C source dependencies:
// #include <vmlinux.h>
// #include "bpf_experimental.h"
// #include "bpf_qdisc_common.h"

use core::ffi::{c_char, c_int, c_void};
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
pub struct Qdisc_ops {
    pub enqueue: *mut c_void,
    pub dequeue: *mut c_void,
    pub reset: *mut c_void,
    pub destroy: *mut c_void,
    pub id: *const c_char,
}

unsafe extern "C" {
    fn bpf_qdisc_skb_drop(skb: *mut sk_buff, to_free: *mut bpf_sk_buff_ptr);
}

const NET_XMIT_DROP: c_int = 1;

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops")]
pub unsafe extern "C" fn bpf_qdisc_test_enqueue(
    skb: *mut sk_buff,
    _sch: *mut Qdisc,
    to_free: *mut bpf_sk_buff_ptr,
) -> c_int {
    unsafe {
        bpf_qdisc_skb_drop(skb, to_free);
    }
    NET_XMIT_DROP
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops")]
pub unsafe extern "C" fn bpf_qdisc_test_dequeue(_sch: *mut Qdisc) -> *mut sk_buff {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops")]
pub unsafe extern "C" fn bpf_qdisc_test_reset(_sch: *mut Qdisc) {}

#[unsafe(no_mangle)]
#[unsafe(link_section = "struct_ops")]
pub unsafe extern "C" fn bpf_qdisc_test_destroy(_sch: *mut Qdisc) {}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".struct_ops")]
pub static mut test: Qdisc_ops = Qdisc_ops {
    enqueue: bpf_qdisc_test_enqueue as *mut c_void,
    dequeue: bpf_qdisc_test_dequeue as *mut c_void,
    reset: bpf_qdisc_test_reset as *mut c_void,
    destroy: bpf_qdisc_test_destroy as *mut c_void,
    id: b"bpf_qdisc_test\0".as_ptr() as *const c_char,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
