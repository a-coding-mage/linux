// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/xdp.c */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
    pub egress_ifindex: u32,
}

#[repr(C)]
pub struct map_array_ro_def {
    pub type_: u32,
    pub max_entries: u32,
    pub map_flags: u32,
}

pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_F_RDONLY_PROG: u32 = 1 << 7;

#[link_section = ".maps"]
#[no_mangle]
pub static map_array_ro: map_array_ro_def = map_array_ro_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    map_flags: BPF_F_RDONLY_PROG,
};

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_xdp_store_bytes(ctx: *mut xdp_md, offset: u32, from: *const core::ffi::c_void, len: u32) -> i64;
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn xdp_using_ifindex_from_netdev(_ctx: *mut xdp_md) {
    /*
     * __description("XDP, using ifindex from netdev")
     * __success __retval(1)
     */
    core::arch::asm!(
        "r0 = 0;",
        "r2 = *(u32*)(r1 + {xdp_md_ingress_ifindex});",
        "if r2 < 1 goto l0_{0};",
        "r0 = 1;",
        "l0_{0}: exit;",
        const 0,
        xdp_md_ingress_ifindex = const core::mem::offset_of!(xdp_md, ingress_ifindex),
        options(noreturn)
    );
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn xdp_store_bytes_from_ro_map(_ctx: *mut xdp_md) {
    /*
     * __description("XDP, using xdp_store_bytes from RO map")
     * __success __retval(0)
     */
    core::arch::asm!(
        "r6 = r1;",
        "r1 = 0;",
        "*(u64*)(r10 - 8) = r1;",
        "r2 = r10;",
        "r2 += -8;",
        "r1 = {map_array_ro} ll;",
        "call {bpf_map_lookup_elem};",
        "if r0 == 0 goto l0_{0};",
        "r1 = r6;",
        "r2 = 0;",
        "r3 = r0;",
        "r4 = 8;",
        "call {bpf_xdp_store_bytes};",
        "l0_{0}: exit;",
        const 0,
        map_array_ro = sym map_array_ro,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_xdp_store_bytes = sym bpf_xdp_store_bytes,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
