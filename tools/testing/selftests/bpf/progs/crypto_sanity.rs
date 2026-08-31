// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/*
 * Translated from C source that included:
 * "vmlinux.h", "bpf_tracing_net.h", <bpf/bpf_helpers.h>,
 * <bpf/bpf_endian.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
 * "bpf_kfuncs.h", and "crypto_common.h".
 *
 * Types, constants, section attributes, and helpers from those dependencies
 * are referenced here in Rust form and are expected to be supplied externally.
 */

use core::ffi::c_void;
use core::mem::{size_of, zeroed};
use core::ptr;

type u16 = u16;
type u32 = u32;

const ETH_P_IPV6: u16 = 0x86DD;
const ETH_HLEN: u32 = 14;
const IPPROTO_UDP: u8 = 17;
const TC_ACT_SHOT: i32 = 2;
const EINVAL: i32 = 22;
const EEXIST: i32 = 17;
const ENOENT: i32 = 2;
const EIO: i32 = 5;

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub pkt_type: u32,
    pub mark: u32,
    pub queue_mapping: u32,
    pub protocol: u32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: u8,
    pub flow_lbl: [u8; 3],
    pub payload_len: u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
}

#[repr(C)]
pub struct udphdr {
    pub source: u16,
    pub dest: u16,
    pub len: u16,
    pub check: u16,
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_crypto_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_crypto_params {
    pub type_: [u8; 128],
    pub algo: [u8; 128],
    pub key: [u8; 256],
    pub key_len: u32,
    pub authsize: u32,
}

#[repr(C)]
pub struct __crypto_ctx_value {
    pub ctx: *mut bpf_crypto_ctx,
}

extern "C" {
    fn bpf_skb_load_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        to: *mut c_void,
        len: u32,
    ) -> i32;
    fn bpf_skb_pull_data(skb: *mut __sk_buff, len: u32) -> i32;
    fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_adjust(ptr: *mut bpf_dynptr, start: u32, end: u32) -> i32;
    fn bpf_dynptr_from_mem(
        data: *mut c_void,
        size: u32,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_crypto_ctx_create(
        params: *mut bpf_crypto_params,
        params__sz: u32,
        err: *mut i32,
    ) -> *mut bpf_crypto_ctx;
    fn bpf_crypto_decrypt(
        ctx: *mut bpf_crypto_ctx,
        src: *mut bpf_dynptr,
        dst: *mut bpf_dynptr,
        iv: *mut c_void,
    ) -> i32;
    fn bpf_crypto_encrypt(
        ctx: *mut bpf_crypto_ctx,
        src: *mut bpf_dynptr,
        dst: *mut bpf_dynptr,
        iv: *mut c_void,
    ) -> i32;
    fn crypto_ctx_insert(cctx: *mut bpf_crypto_ctx) -> i32;
    fn crypto_ctx_value_lookup() -> *mut __crypto_ctx_value;
}

#[inline]
fn __bpf_constant_htons(x: u16) -> u16 {
    x.to_be()
}

#[inline]
fn __bpf_htons(x: u16) -> u16 {
    x.to_be()
}

/*
 * key[] and algo[] are 8-byte aligned and 'params' is kept off the stack to
 * work around an LLVM code generation bug. clang lowers the memcpy() of these
 * byte-aligned globals into a per-byte load/store sequence staged on the stack,
 * and additionally materializes the on-stack 'struct bpf_crypto_params' twice.
 * Both blow the 512-byte BPF stack limit. Aligning the sources lets clang copy
 * word-wise, and a global 'params' removes the large object from the stack.
 */
#[repr(align(8))]
pub struct AlignedKey(pub [u8; 256]);

#[repr(align(8))]
pub struct AlignedAlgo(pub [u8; 128]);

#[no_mangle]
pub static mut key: AlignedKey = AlignedKey([0; 256]);
#[no_mangle]
pub static mut udp_test_port: u16 = 7777;
#[no_mangle]
pub static mut authsize: u32 = 0;
#[no_mangle]
pub static mut key_len: u32 = 0;
#[no_mangle]
pub static mut algo: AlignedAlgo = AlignedAlgo([0; 128]);
#[no_mangle]
pub static mut dst: [u8; 16] = [0; 16];
#[no_mangle]
pub static mut dst_bad: [u8; 8] = [0; 8];
static mut params: bpf_crypto_params = bpf_crypto_params {
    type_: [0; 128],
    algo: [0; 128],
    key: [0; 256],
    key_len: 0,
    authsize: 0,
};
#[no_mangle]
pub static mut status: i32 = 0;

unsafe fn skb_dynptr_validate(skb: *mut __sk_buff, psrc: *mut bpf_dynptr) -> i32 {
    let mut ip6h: ipv6hdr = zeroed();
    let mut udph: udphdr = zeroed();
    let offset: u32;

    if (*skb).protocol != __bpf_constant_htons(ETH_P_IPV6) as u32 {
        return -1;
    }

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN,
        &mut ip6h as *mut ipv6hdr as *mut c_void,
        size_of::<ipv6hdr>() as u32,
    ) != 0
    {
        return -1;
    }

    if ip6h.nexthdr != IPPROTO_UDP {
        return -1;
    }

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN + size_of::<ipv6hdr>() as u32,
        &mut udph as *mut udphdr as *mut c_void,
        size_of::<udphdr>() as u32,
    ) != 0
    {
        return -1;
    }

    if udph.dest != __bpf_htons(udp_test_port) {
        return -1;
    }

    offset = ETH_HLEN + size_of::<ipv6hdr>() as u32 + size_of::<udphdr>() as u32;
    if (*skb).len < offset + 16 {
        return -1;
    }

    /* let's make sure that 16 bytes of payload are in the linear part of skb */
    bpf_skb_pull_data(skb, offset + 16);
    bpf_dynptr_from_skb(skb, 0, psrc);
    bpf_dynptr_adjust(psrc, offset, offset + 16);

    0
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn skb_crypto_setup(ctx: *mut c_void) -> i32 {
    let cctx: *mut bpf_crypto_ctx;
    let mut err: i32 = 0;

    let _ = ctx;
    status = 0;
    if key_len > 256 {
        status = -EINVAL;
        return 0;
    }

    ptr::copy_nonoverlapping(
        b"skcipher\0".as_ptr(),
        params.type_.as_mut_ptr(),
        size_of::<[u8; 9]>(),
    );
    params.key_len = key_len;
    params.authsize = authsize;
    ptr::copy_nonoverlapping(algo.0.as_ptr(), params.algo.as_mut_ptr(), algo.0.len());
    ptr::copy_nonoverlapping(key.0.as_ptr(), params.key.as_mut_ptr(), key.0.len());

    cctx = bpf_crypto_ctx_create(
        &mut params as *mut bpf_crypto_params,
        size_of::<bpf_crypto_params>() as u32,
        &mut err as *mut i32,
    );
    if cctx.is_null() {
        status = err;
        return 0;
    }

    err = crypto_ctx_insert(cctx);
    if err != 0 && err != -EEXIST {
        status = err;
    }
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn decrypt_sanity(skb: *mut __sk_buff) -> i32 {
    let v: *mut __crypto_ctx_value;
    let ctx: *mut bpf_crypto_ctx;
    let mut psrc: bpf_dynptr = zeroed();
    let mut pdst: bpf_dynptr = zeroed();
    let mut err: i32;

    status = 0;
    err = skb_dynptr_validate(skb, &mut psrc as *mut bpf_dynptr);
    if err < 0 {
        status = err;
        return TC_ACT_SHOT;
    }

    v = crypto_ctx_value_lookup();
    if v.is_null() {
        status = -ENOENT;
        return TC_ACT_SHOT;
    }

    ctx = (*v).ctx;
    if ctx.is_null() {
        status = -ENOENT;
        return TC_ACT_SHOT;
    }

    /* Check also bad case where the dst buffer is smaller than the
     * skb's linear section.
     */
    bpf_dynptr_from_mem(
        dst_bad.as_mut_ptr() as *mut c_void,
        size_of::<[u8; 8]>() as u32,
        0,
        &mut pdst as *mut bpf_dynptr,
    );
    status = bpf_crypto_decrypt(
        ctx,
        &mut psrc as *mut bpf_dynptr,
        &mut pdst as *mut bpf_dynptr,
        ptr::null_mut(),
    );
    if status == 0 {
        status = -EIO;
    }
    if status != -EINVAL {
        return TC_ACT_SHOT;
    }

    /* dst is a global variable to make testing part easier to check.
     * In real production code, a percpu map should be used to store
     * the result.
     */
    bpf_dynptr_from_mem(
        dst.as_mut_ptr() as *mut c_void,
        size_of::<[u8; 16]>() as u32,
        0,
        &mut pdst as *mut bpf_dynptr,
    );
    status = bpf_crypto_decrypt(
        ctx,
        &mut psrc as *mut bpf_dynptr,
        &mut pdst as *mut bpf_dynptr,
        ptr::null_mut(),
    );

    TC_ACT_SHOT
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn encrypt_sanity(skb: *mut __sk_buff) -> i32 {
    let v: *mut __crypto_ctx_value;
    let ctx: *mut bpf_crypto_ctx;
    let mut psrc: bpf_dynptr = zeroed();
    let mut pdst: bpf_dynptr = zeroed();
    let mut err: i32;

    status = 0;
    err = skb_dynptr_validate(skb, &mut psrc as *mut bpf_dynptr);
    if err < 0 {
        status = err;
        return TC_ACT_SHOT;
    }

    v = crypto_ctx_value_lookup();
    if v.is_null() {
        status = -ENOENT;
        return TC_ACT_SHOT;
    }

    ctx = (*v).ctx;
    if ctx.is_null() {
        status = -ENOENT;
        return TC_ACT_SHOT;
    }

    /* Check also bad case where the dst buffer is smaller than the
     * skb's linear section.
     */
    bpf_dynptr_from_mem(
        dst_bad.as_mut_ptr() as *mut c_void,
        size_of::<[u8; 8]>() as u32,
        0,
        &mut pdst as *mut bpf_dynptr,
    );
    status = bpf_crypto_encrypt(
        ctx,
        &mut psrc as *mut bpf_dynptr,
        &mut pdst as *mut bpf_dynptr,
        ptr::null_mut(),
    );
    if status == 0 {
        status = -EIO;
    }
    if status != -EINVAL {
        return TC_ACT_SHOT;
    }

    /* dst is a global variable to make testing part easier to check.
     * In real production code, a percpu map should be used to store
     * the result.
     */
    bpf_dynptr_from_mem(
        dst.as_mut_ptr() as *mut c_void,
        size_of::<[u8; 16]>() as u32,
        0,
        &mut pdst as *mut bpf_dynptr,
    );
    status = bpf_crypto_encrypt(
        ctx,
        &mut psrc as *mut bpf_dynptr,
        &mut pdst as *mut bpf_dynptr,
        ptr::null_mut(),
    );

    TC_ACT_SHOT
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";
