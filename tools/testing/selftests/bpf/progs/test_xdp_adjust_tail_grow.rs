// SPDX-License-Identifier: GPL-2.0
// Rust translation of includes:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

use core::ffi::c_int;

#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}

pub const XDP_ABORTED: c_int = 0;
pub const XDP_DROP: c_int = 1;
pub const XDP_TX: c_int = 3;

unsafe extern "C" {
    fn bpf_xdp_get_buff_len(xdp: *mut xdp_md) -> c_int;
    fn bpf_xdp_adjust_tail(xdp: *mut xdp_md, delta: c_int) -> c_int;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn _xdp_adjust_tail_grow(xdp: *mut xdp_md) -> c_int {
    let data_len: c_int = unsafe { bpf_xdp_get_buff_len(xdp) };
    let mut offset: c_int = 0;
    /* SKB_DATA_ALIGN(sizeof(struct skb_shared_info)) */
    #[cfg(target_arch = "s390x")]
    let tailroom: c_int = 512;
    #[cfg(target_arch = "powerpc")]
    let tailroom: c_int = 384;
    #[cfg(not(any(target_arch = "s390x", target_arch = "powerpc")))]
    let tailroom: c_int = 320;

    /* Data length determine test case */

    if data_len == 54 {
        /* sizeof(pkt_v4) */
        offset = 4096; /* test too large offset, 4k page size */
    } else if data_len == 53 {
        /* sizeof(pkt_v4) - 1 */
        offset = 65536; /* test too large offset, 64k page size */
    } else if data_len == 74 {
        /* sizeof(pkt_v6) */
        offset = 40;
    } else if data_len == 64 {
        offset = 128;
    } else if data_len == 128 {
        /* Max tail grow 3520 */
        offset = 4096 - 256 - tailroom - data_len;
    } else if data_len == 9000 {
        offset = 10;
    } else if data_len == 9001 {
        offset = 4096;
    } else if data_len == 90000 {
        offset = 10; /* test a small offset, 64k page size */
    } else if data_len == 90001 {
        offset = 65536; /* test too large offset, 64k page size */
    } else {
        return XDP_ABORTED; /* No matching test */
    }

    if unsafe { bpf_xdp_adjust_tail(xdp, offset) } != 0 {
        return XDP_DROP;
    }
    XDP_TX
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
