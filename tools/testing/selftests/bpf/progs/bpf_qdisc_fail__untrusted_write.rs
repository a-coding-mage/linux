// SPDX-License-Identifier: GPL-2.0

// C includes translated as external dependency intent:
// <vmlinux.h>
// "bpf_experimental.h"
// "bpf_qdisc_common.h"
// "bpf_misc.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

extern "C" {
    fn bpf_qdisc_skb_drop(skb: *mut sk_buff, to_free: *mut bpf_sk_buff_ptr);
}

const NET_XMIT_DROP: c_int = 1;

#[repr(C)]
pub struct sk_buff {
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
pub struct Qdisc {
    pub next_sched: *mut Qdisc,
    pub limit: u32,
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

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[no_mangle]
#[link_section = "struct_ops"]
// __failure
// __msg("only read is supported")
pub unsafe extern "C" fn untrusted_write(
    skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut bpf_sk_buff_ptr,
) -> c_int {
    let next: *mut Qdisc = (*sch).next_sched;

    /*
     * sch is trusted, but the walk of next_sched yields a plain
     * PTR_TO_BTF_ID which may fault on a dereference. A store through
     * it does not get an exception table entry, there is no probed
     * store to rewrite it into, hence it has to be rejected before
     * bpf_qdisc_btf_struct_access() gets to allow the write to limit.
     */
    (*next).limit = 1000;

    bpf_qdisc_skb_drop(skb, to_free);
    NET_XMIT_DROP
}

#[no_mangle]
#[link_section = "struct_ops"]
// __auxiliary
pub unsafe extern "C" fn bpf_qdisc_test_dequeue(_sch: *mut Qdisc) -> *mut sk_buff {
    ptr::null_mut()
}

#[no_mangle]
#[link_section = "struct_ops"]
// __auxiliary
pub unsafe extern "C" fn bpf_qdisc_test_init(
    _sch: *mut Qdisc,
    _opt: *mut nlattr,
    _extack: *mut netlink_ext_ack,
) -> c_int {
    0
}

#[no_mangle]
#[link_section = "struct_ops"]
// __auxiliary
pub unsafe extern "C" fn bpf_qdisc_test_reset(_sch: *mut Qdisc) {}

#[no_mangle]
#[link_section = "struct_ops"]
// __auxiliary
pub unsafe extern "C" fn bpf_qdisc_test_destroy(_sch: *mut Qdisc) {}

#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut test: Qdisc_ops = Qdisc_ops {
    enqueue: untrusted_write as *mut c_void,
    dequeue: bpf_qdisc_test_dequeue as *mut c_void,
    init: bpf_qdisc_test_init as *mut c_void,
    reset: bpf_qdisc_test_reset as *mut c_void,
    destroy: bpf_qdisc_test_destroy as *mut c_void,
    id: b"bpf_qdisc_test\0".as_ptr() as *const c_char,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
