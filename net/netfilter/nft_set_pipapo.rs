// SPDX-License-Identifier: GPL-2.0-only
//
// PIPAPO: PIle PAcket POlicies: set for arbitrary concatenations of ranges.
//
// This is a low-level Rust translation of nft_set_pipapo.c.  Kernel types,
// constants, macros, and helpers supplied by the surrounding nftables code are
// intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

// External kernel/nftables declarations (provided by the surrounding crate).
extern "C" {
    fn bitmap_clear(map: *mut c_ulong, start: c_uint, nbits: c_uint);
    fn bitmap_set(map: *mut c_ulong, start: c_uint, nbits: c_uint);
}

pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = usize;
pub type u8_ = u8;
pub type u64_ = u64;

#[repr(C)]
pub union nft_pipapo_map_bucket {
    pub map: MapRange,
    pub e: *mut nft_pipapo_elem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MapRange { pub to: c_uint, pub n: c_uint }

#[repr(C)]
pub struct nft_pipapo_elem { pub priv_: nft_elem_priv, pub ext: nft_set_ext }
#[repr(C)] pub struct nft_elem_priv { _private: [u8; 0] }
#[repr(C)] pub struct nft_set_ext { _private: [u8; 0] }
#[repr(C)] pub struct nft_pipapo_field {
    pub lt: *mut c_ulong, pub mt: *mut nft_pipapo_map_bucket,
    pub groups: u8, pub bb: u8, pub bsize: c_uint,
    pub rules: c_uint, pub rules_alloc: c_uint,
}
#[repr(C)] pub struct nft_pipapo_match {
    pub field_count: c_uint, pub bsize_max: c_ulong,
    pub scratch: *mut *mut nft_pipapo_scratch,
    pub f: nft_pipapo_field,
}
#[repr(C)] pub struct nft_pipapo_scratch {
    pub map_index: bool, pub __map: [c_ulong; 0],
}

extern "C" {
    fn pipapo_resmap_init(m: *const nft_pipapo_match, map: *mut c_ulong);
    fn pipapo_and_field_buckets_8bit(f: *const nft_pipapo_field, map: *mut c_ulong, data: *const u8);
    fn pipapo_and_field_buckets_4bit(f: *const nft_pipapo_field, map: *mut c_ulong, data: *const u8);
}

/// For each set bit in `map`, fill the corresponding range in `dst`.
#[no_mangle]
pub unsafe extern "C" fn pipapo_refill(
    map: *mut c_ulong, len: c_uint, rules: c_uint, dst: *mut c_ulong,
    mt: *const nft_pipapo_map_bucket, match_only: bool,
) -> c_int {
    let mut ret: c_int = -1;
    for k in 0..len {
        let mut bitset = *map.add(k as usize);
        while bitset != 0 {
            let t = bitset & bitset.wrapping_neg();
            let r = bitset.trailing_zeros();
            let i = k.wrapping_mul((usize::BITS) as c_uint).wrapping_add(r);
            if i >= rules { *map.add(k as usize) = 0; return -1; }
            if match_only {
                bitmap_clear(map, i, 1);
                return i as c_int;
            }
            ret = 0;
            let bucket = &*mt.add(i as usize);
            let range = bucket.map;
            bitmap_set(dst, range.to, range.n);
            bitset ^= t;
        }
        *map.add(k as usize) = 0;
    }
    ret
}

// The remaining kernel-facing entry points retain the C ABI and are supplied
// by the translation unit's nftables integration.  Their bodies are declared
// here so the translated implementation can refer to the same interfaces.
extern "C" {
    fn nft_pipapo_abort(set: *const nft_set);
}
#[repr(C)] pub struct nft_set { _private: [u8; 0] }

// C conditional compilation is preserved for the AVX2 implementation by the
// integration build; the generic implementation is selected there.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
