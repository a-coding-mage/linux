// SPDX-License-Identifier: GPL-2.0

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

pub static mut xdpf_sz: i32 = 0;
pub static mut sinfo_sz: i32 = 0;
pub static mut data_len: i32 = 0;
pub static mut pull_len: i32 = 0;

pub const XDP_PACKET_HEADROOM: i32 = 256;

extern "C" {
    static __PAGE_SIZE: i32;

    fn bpf_xdp_pull_data(ctx: *mut xdp_md, len: i32) -> i32;
}

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct xdp_frame {
    _unused: [u8; 0],
}

pub const XDP_PASS: i32 = 2;
pub const XDP_DROP: i32 = 1;

#[no_mangle]
#[link_section = "xdp.frags"]
pub unsafe extern "C" fn xdp_find_sizes(ctx: *mut xdp_md) -> i32 {
    xdpf_sz = core::mem::size_of::<xdp_frame>() as i32;
    sinfo_sz = __PAGE_SIZE
        - XDP_PACKET_HEADROOM
        - ((*ctx).data_end as i64 - (*ctx).data as i64) as i32;

    XDP_PASS
}

#[no_mangle]
#[link_section = "xdp.frags"]
pub unsafe extern "C" fn xdp_pull_data_prog(ctx: *mut xdp_md) -> i32 {
    let data_end: *mut u8 = (*ctx).data_end as i64 as *mut u8;
    let data: *mut u8 = (*ctx).data as i64 as *mut u8;
    let mut val_p: *mut u8;
    let err: i32;

    if data_len != data_end.offset_from(data) as i32 {
        return XDP_DROP;
    }

    err = bpf_xdp_pull_data(ctx, pull_len);
    if err != 0 {
        return XDP_DROP;
    }

    val_p = ((*ctx).data as i64 as *mut u8).add(1024);
    if val_p.add(1) > ((*ctx).data_end as i64 as *mut core::ffi::c_void) as *mut u8 {
        return XDP_DROP;
    }

    if *val_p != 0xbb {
        return XDP_DROP;
    }

    XDP_PASS
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
