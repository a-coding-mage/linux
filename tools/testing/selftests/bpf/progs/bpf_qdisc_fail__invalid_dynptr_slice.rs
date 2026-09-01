// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include "bpf_experimental.h"
// #include "bpf_qdisc_common.h"
// #include "bpf_misc.h"

extern "C" {
    fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice(
        ptr: *mut bpf_dynptr,
        offset: u32,
        buffer: *mut core::ffi::c_void,
        buffer__sz: u32,
    ) -> *mut core::ffi::c_void;
    fn bpf_qdisc_skb_drop(skb: *mut sk_buff, to_free: *mut bpf_sk_buff_ptr);
}

extern "C" {
    static NET_XMIT_DROP: i32;
}

#[repr(C)]
pub struct __sk_buff {
    _data: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _data: [u8; 0],
}

#[repr(C)]
pub struct Qdisc {
    _data: [u8; 0],
}

#[repr(C)]
pub struct bpf_sk_buff_ptr {
    _data: [u8; 0],
}

#[repr(C)]
pub struct nlattr {
    _data: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
    _data: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _data: [u8; 0],
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct Qdisc_ops {
    pub enqueue: *mut core::ffi::c_void,
    pub dequeue: *mut core::ffi::c_void,
    pub init: *mut core::ffi::c_void,
    pub reset: *mut core::ffi::c_void,
    pub destroy: *mut core::ffi::c_void,
    pub id: *const u8,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut proto: i32 = 0;

// SEC("struct_ops")
// __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn invalid_dynptr_slice(
    skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut bpf_sk_buff_ptr,
) -> i32 {
    let mut ptr: bpf_dynptr = core::mem::zeroed();
    let mut hdr: *mut ethhdr;

    let _ = sch;

    bpf_dynptr_from_skb(skb as *mut __sk_buff, 0, &mut ptr);

    hdr = bpf_dynptr_slice(
        &mut ptr,
        0,
        core::ptr::null_mut(),
        core::mem::size_of::<ethhdr>() as u32,
    ) as *mut ethhdr;
    if hdr.is_null() {
        bpf_qdisc_skb_drop(skb, to_free);
        return NET_XMIT_DROP;
    }

    bpf_qdisc_skb_drop(skb, to_free);

    proto = (*hdr).h_proto as i32;

    NET_XMIT_DROP
}

// SEC("struct_ops")
// __auxiliary
#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn bpf_qdisc_test_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let _ = sch;
    core::ptr::null_mut()
}

// SEC("struct_ops")
// __auxiliary
#[no_mangle]
#[link_section = "struct_ops"]
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
#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn bpf_qdisc_test_reset(sch: *mut Qdisc) {
    let _ = sch;
}

// SEC("struct_ops")
// __auxiliary
#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn bpf_qdisc_test_destroy(sch: *mut Qdisc) {
    let _ = sch;
}

// SEC(".struct_ops")
#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut test: Qdisc_ops = Qdisc_ops {
    enqueue: invalid_dynptr_slice as *mut core::ffi::c_void,
    dequeue: bpf_qdisc_test_dequeue as *mut core::ffi::c_void,
    init: bpf_qdisc_test_init as *mut core::ffi::c_void,
    reset: bpf_qdisc_test_reset as *mut core::ffi::c_void,
    destroy: bpf_qdisc_test_destroy as *mut core::ffi::c_void,
    id: b"bpf_qdisc_test\0".as_ptr(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
