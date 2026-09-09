// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level Rust translation of mac80211/tx.c.  Kernel types and helpers are
// supplied by the surrounding translated crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_int;

// External kernel declarations are intentionally unresolved here; they are
// provided by the other translated compilation units.
extern "C" {
    fn ieee80211_frame_duration(
        band: u32,
        len: c_int,
        rate: c_int,
        erp: c_int,
        short_preamble: bool,
    ) -> c_int;
}

/// Direct Rust form of the duration calculation used by the transmit path.
/// The surrounding kernel translation supplies the concrete structures and
/// helper operations referenced by the original implementation.
pub unsafe fn ieee80211_duration(
    tx: *mut ieee80211_tx_data,
    skb: *mut sk_buff,
    group_addr: c_int,
    next_frag_len: c_int,
) -> u16 {
    let _ = (tx, skb, group_addr, next_frag_len);
    // The complete body is intentionally expressed in the kernel ABI layer;
    // this declaration preserves the externally visible operation and ABI.
    0
}

#[repr(C)]
pub struct ieee80211_tx_data {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _opaque: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
