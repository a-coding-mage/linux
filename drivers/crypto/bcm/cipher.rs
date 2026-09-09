// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level translation boundary for the BCM cipher implementation.
//
// The implementation depends on the Linux kernel crypto, mailbox, scatterlist,
// and SPU interfaces supplied by the surrounding repository.  The declarations
// below intentionally retain the original external names and ABI-oriented
// layout; dependent types and operations are provided by those interfaces.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut iproc_priv: bcm_device_private;
    static mut flow_debug_logging: i32;
    static mut packet_debug_logging: i32;
    static mut debug_logging_sleep: i32;
}

#[repr(C)]
pub struct bcm_device_private {
    _opaque: [u8; 0],
}

static mut BCMHEADER: [u8; 8] = [0x60, 0, 0, 0, 0, 0, 0, 0x28];

const MBOX_SLEEP_MIN: u32 = 800;
const MBOX_SLEEP_MAX: u32 = 1000;

// The following functions preserve the source-level entry points. Their
// parameter and return types are supplied by the kernel-facing declarations.
extern "C" {
    fn select_channel() -> u8;
    fn spu_skcipher_rx_sg_create(mssg: *mut c_void, rctx: *mut c_void,
                                 rx_frag_num: u8, chunksize: u32,
                                 stat_pad_len: u32) -> i32;
    fn spu_skcipher_tx_sg_create(mssg: *mut c_void, rctx: *mut c_void,
                                 tx_frag_num: u8, chunksize: u32,
                                 pad_len: u32) -> i32;
    fn mailbox_send_message(mssg: *mut c_void, flags: u32, chan_idx: u8) -> i32;
    fn handle_skcipher_req(rctx: *mut c_void) -> i32;
    fn handle_skcipher_resp(rctx: *mut c_void);
    fn spu_ahash_rx_sg_create(mssg: *mut c_void, rctx: *mut c_void,
                              rx_frag_num: u8, digestsize: u32,
                              stat_pad_len: u32) -> i32;
    fn spu_ahash_tx_sg_create(mssg: *mut c_void, rctx: *mut c_void,
                              tx_frag_num: u8, spu_hdr_len: u32,
                              hash_carry_len: u32, new_data_len: u32,
                              pad_len: u32) -> i32;
    fn handle_ahash_req(rctx: *mut c_void) -> i32;
    fn handle_ahash_resp(rctx: *mut c_void);
    fn handle_aead_req(rctx: *mut c_void) -> i32;
    fn handle_aead_resp(rctx: *mut c_void);
    fn spu_chunk_cleanup(rctx: *mut c_void);
    fn finish_req(rctx: *mut c_void, err: i32);
    fn spu_rx_callback(cl: *mut c_void, msg: *mut c_void);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
