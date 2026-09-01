// SPDX-License-Identifier: GPL-2.0

/* This logic is lifted from a real-world use case of packet parsing, used in
 * the open source library katran, a layer 4 load balancer.
 *
 * This test demonstrates how to parse packet contents using dynptrs. The
 * original code (parsing without dynptrs) can be found in test_parse_tcp_hdr_opt.c
 */

/* C dependencies:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include <linux/tcp.h>
 * #include <stdbool.h>
 * #include <linux/ipv6.h>
 * #include <linux/if_ether.h>
 * #include "test_tcp_hdr_options.h"
 * #include "bpf_kfuncs.h"
 */

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

extern "C" {
    fn bpf_dynptr_from_xdp(xdp: *mut xdp_md, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice(
        ptr: *const bpf_dynptr,
        offset: u32,
        buffer: *mut c_void,
        buffer__sz: u32,
    ) -> *mut c_void;
}

extern "C" {
    type bpf_dynptr;
    type xdp_md;
    type ethhdr;
    type ipv6hdr;
}

#[repr(C)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub seq: u32,
    pub ack_seq: u32,
    pub res1_doff: u8,
    pub flags: u8,
    pub window: u16,
    pub check: u16,
    pub urg_ptr: u16,
}

impl tcphdr {
    #[inline]
    unsafe fn doff(&self) -> u8 {
        self.res1_doff >> 4
    }
}

const TCPOPT_EOL: u8 = 0;
const TCPOPT_NOP: u8 = 1;
const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Kind number used for experiments */
pub const tcp_hdr_opt_kind_tpr: u32 = 0xFD;
/* Length of the tcp header option */
pub const tcp_hdr_opt_len_tpr: u32 = 6;
/* maximum number of header options to check to lookup server_id */
pub const tcp_hdr_opt_max_opt_checks: u32 = 15;

#[no_mangle]
pub static mut server_id: u32 = 0;

unsafe fn parse_hdr_opt(
    ptr: *mut bpf_dynptr,
    off: *mut u32,
    hdr_bytes_remaining: *mut u8,
    server_id: *mut u32,
) -> i32 {
    let mut kind: u8;
    let hdr_len: u8;
    let mut buffer: [u8; size_of::<u8>() + size_of::<u8>() + size_of::<u32>()] = [0; 6];
    let data: *mut u8;

    ptr::write_bytes(buffer.as_mut_ptr(), 0, buffer.len());

    data = bpf_dynptr_slice(
        ptr,
        *off,
        buffer.as_mut_ptr() as *mut c_void,
        buffer.len() as u32,
    ) as *mut u8;
    if data.is_null() {
        return -1;
    }

    kind = *data.add(0);

    if kind == TCPOPT_EOL {
        return -1;
    }

    if kind == TCPOPT_NOP {
        *off = (*off).wrapping_add(1);
        *hdr_bytes_remaining = (*hdr_bytes_remaining).wrapping_sub(1);
        return 0;
    }

    if *hdr_bytes_remaining < 2 {
        return -1;
    }

    hdr_len = *data.add(1);
    if hdr_len > *hdr_bytes_remaining {
        return -1;
    }

    if kind as u32 == tcp_hdr_opt_kind_tpr {
        if hdr_len as u32 != tcp_hdr_opt_len_tpr {
            return -1;
        }

        ptr::copy_nonoverlapping(
            data.add(2),
            server_id as *mut u8,
            size_of::<u32>(),
        );
        return 1;
    }

    *off = (*off).wrapping_add(hdr_len as u32);
    *hdr_bytes_remaining = (*hdr_bytes_remaining).wrapping_sub(hdr_len);
    0
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_ingress_v6(xdp: *mut xdp_md) -> i32 {
    let mut buffer: [u8; size_of::<tcphdr>()] = [0; size_of::<tcphdr>()];
    let mut hdr_bytes_remaining: u8;
    let tcp_hdr: *mut tcphdr;
    let tcp_hdr_opt_len: u8;
    let mut err: i32 = 0;
    let mut off: u32;

    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    bpf_dynptr_from_xdp(xdp, 0, ptr.as_mut_ptr());
    let mut ptr = ptr.assume_init();

    off = (size_of::<ethhdr>() + size_of::<ipv6hdr>()) as u32;

    tcp_hdr = bpf_dynptr_slice(
        &mut ptr as *mut bpf_dynptr,
        off,
        buffer.as_mut_ptr() as *mut c_void,
        buffer.len() as u32,
    ) as *mut tcphdr;
    if tcp_hdr.is_null() {
        return XDP_DROP;
    }

    tcp_hdr_opt_len = ((*tcp_hdr).doff().wrapping_mul(4)).wrapping_sub(size_of::<tcphdr>() as u8);
    if (tcp_hdr_opt_len as u32) < tcp_hdr_opt_len_tpr {
        return XDP_DROP;
    }

    hdr_bytes_remaining = tcp_hdr_opt_len;

    off = off.wrapping_add(size_of::<tcphdr>() as u32);

    /* max number of bytes of options in tcp header is 40 bytes */
    let mut i: i32 = 0;
    while i < tcp_hdr_opt_max_opt_checks as i32 {
        err = parse_hdr_opt(
            &mut ptr as *mut bpf_dynptr,
            &mut off as *mut u32,
            &mut hdr_bytes_remaining as *mut u8,
            &mut server_id as *mut u32,
        );

        if err != 0 || hdr_bytes_remaining == 0 {
            break;
        }

        i += 1;
    }

    if server_id == 0 {
        return XDP_DROP;
    }

    XDP_PASS
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
