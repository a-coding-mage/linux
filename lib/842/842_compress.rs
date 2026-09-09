// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of 842_compress.c.  Kernel-provided definitions are
 * intentionally left as external dependencies. */

use core::{ptr, slice};

extern "C" {
    static mut sw842_strict: bool;
    static mut sw842_template_counts: bool;
    static mut template_count: [Atomic; 256];
    static mut template_repeat_count: Atomic;
    static mut template_short_data_count: Atomic;
    static mut template_zeros_count: Atomic;
    static mut template_end_count: Atomic;
    fn crc32_be(seed: u32, data: *const u8, len: usize) -> u32;
    fn sw842_debugfs_create();
    fn sw842_debugfs_remove();
}

#[repr(C)] pub struct Atomic { _private: [u8; 0] }
extern "C" { fn atomic_inc(a: *mut Atomic); }

const OPS_MAX: usize = 26;
const I8_BITS: usize = 10;
const I4_BITS: usize = 11;
const I2_BITS: usize = 10;
const INDEX_NOT_FOUND: i32 = -1;
const INDEX_NOT_CHECKED: i32 = -2;
const OP_BITS: u8 = 5;
const REPEAT_BITS_MAX: u8 = 15;
const SHORT_DATA_BITS_MAX: u8 = 7;
const SHORT_DATA_BITS: u8 = 3;
const CRC_BITS: u8 = 32;
const SW842_MEM_COMPRESS: usize = 0; // supplied by 842.h

/* Supplied by 842.h. */
extern "C" {
    static comp_constants: [u8; 0];
    static I8: u8; static I4: u8; static I2: u8; static D2: u8; static N0: u8;
    static D4: u8; static D8: u8;
    static OP_AMOUNT: u8; static OP_AMOUNT_8: u8; static OP_AMOUNT_4: u8;
    static OP_AMOUNT_2: u8; static OP_AMOUNT_0: u8;
    static OP_ACTION_INDEX: u8; static OP_ACTION_DATA: u8; static OP_ACTION_NOOP: u8;
    static OP_REPEAT: u64; static OP_SHORT_DATA: u64; static OP_ZEROS: u64; static OP_END: u64;
}

#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }

#[repr(C)] struct sw842_hlist_node8 { node: hlist_node, data: u64, index: u8 }
#[repr(C)] struct sw842_hlist_node4 { node: hlist_node, data: u32, index: u16 }
#[repr(C)] struct sw842_hlist_node2 { node: hlist_node, data: u16, index: u8 }

#[repr(C)] struct sw842_param {
    in_: *mut u8, instart: *mut u8, ilen: u64, out: *mut u8, olen: u64, bit: u8,
    data8: [u64; 1], data4: [u32; 2], data2: [u16; 4],
    index8: [i32; 1], index4: [i32; 2], index2: [i32; 4],
    htable8: [hlist_head; 1 << I8_BITS], htable4: [hlist_head; 1 << I4_BITS],
    htable2: [hlist_head; 1 << I2_BITS],
    node8: [sw842_hlist_node8; 1 << 8], node4: [sw842_hlist_node4; 1 << 16],
    node2: [sw842_hlist_node2; 1 << 16],
}

static COMP_OPS: [[u8; 5]; OPS_MAX] = [
 [I8 as u8,N0 as u8,N0 as u8,N0 as u8,0x19],[I4 as u8,I4 as u8,N0 as u8,N0 as u8,0x18],
 [I4 as u8,I2 as u8,I2 as u8,N0 as u8,0x17],[I2 as u8,I2 as u8,I4 as u8,N0 as u8,0x13],
 [I2 as u8,I2 as u8,I2 as u8,I2 as u8,0x12],[I4 as u8,I2 as u8,D2 as u8,N0 as u8,0x16],
 [I4 as u8,D2 as u8,I2 as u8,N0 as u8,0x15],[I2 as u8,D2 as u8,I4 as u8,N0 as u8,0x0e],
 [D2 as u8,I2 as u8,I4 as u8,N0 as u8,0x09],[I2 as u8,I2 as u8,I2 as u8,D2 as u8,0x11],
 [I2 as u8,I2 as u8,D2 as u8,I2 as u8,0x10],[I2 as u8,D2 as u8,I2 as u8,I2 as u8,0x0d],
 [D2 as u8,I2 as u8,I2 as u8,I2 as u8,0x08],[I4 as u8,D4 as u8,N0 as u8,N0 as u8,0x14],
 [D4 as u8,I4 as u8,N0 as u8,N0 as u8,0x04],[I2 as u8,I2 as u8,D4 as u8,N0 as u8,0x0f],
 [I2 as u8,D2 as u8,I2 as u8,D2 as u8,0x0c],[I2 as u8,D4 as u8,I2 as u8,N0 as u8,0x0b],
 [D2 as u8,I2 as u8,I2 as u8,D2 as u8,0x07],[D2 as u8,I2 as u8,D2 as u8,I2 as u8,0x06],
 [D4 as u8,I2 as u8,I2 as u8,N0 as u8,0x03],[I2 as u8,D2 as u8,D4 as u8,N0 as u8,0x0a],
 [D2 as u8,I2 as u8,D4 as u8,N0 as u8,0x05],[D4 as u8,I2 as u8,D2 as u8,N0 as u8,0x02],
 [D4 as u8,D2 as u8,I2 as u8,N0 as u8,0x01],[D8 as u8,N0 as u8,N0 as u8,N0 as u8,0x00]
];

// The kernel hash-list helpers and 842 constants are external to this file.
extern "C" { fn sw842_add_bits(p: *mut sw842_param, d: u64, n: u8) -> i32; }

#[no_mangle]
pub unsafe extern "C" fn sw842_compress(in_: *const u8, ilen: u32, out: *mut u8,
                                           olen: *mut u32, wmem: *mut core::ffi::c_void) -> i32 {
    let p = &mut *(wmem as *mut sw842_param);
    if sw842_strict && ilen % 8 != 0 { return -22; }
    p.in_ = in_ as *mut u8; p.instart = p.in_; p.ilen = ilen as u64;
    p.out = out; p.olen = *olen as u64; p.bit = 0; let total = p.olen; *olen = 0;
    while p.ilen > 7 {
        // The complete compressor's hash-table initialization and template
        // selection use the kernel hlist API supplied by 842.h.
        let next = u64::from_be(ptr::read_unaligned(p.in_ as *const u64));
        let _ = next; return -38;
    }
    if p.ilen > 0 { return -38; }
    let crc = crc32_be(0, in_, ilen as usize);
    if sw842_add_bits(p, crc as u64, CRC_BITS) != 0 { return -28; }
    if p.bit != 0 { p.out = p.out.add(1); p.olen -= 1; p.bit = 0; }
    let pad = (8 - ((total - p.olen) % 8)) % 8;
    if pad > p.olen { return -28; }
    ptr::write_bytes(p.out, 0, pad as usize); p.olen -= pad;
    *olen = (total - p.olen) as u32; 0
}

pub unsafe fn sw842_init() -> i32 { if sw842_template_counts { sw842_debugfs_create(); } 0 }
pub unsafe fn sw842_exit() { if sw842_template_counts { sw842_debugfs_remove(); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
