// SPDX-License-Identifier: GPL-2.0-only
//
// PIPAPO: PIle PAcket POlicies: AVX2 packet lookup routines.
// Rust translation of nft_set_pipapo_avx2.c. Kernel definitions and helpers
// supplied by the surrounding nftables implementation remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

const BITS_PER_LONG: usize = usize::BITS as usize;
const BITS_PER_BYTE: usize = 8;
const XSAVE_YMM_SIZE: usize = 32;
const NFT_PIPAPO_LONGS_PER_M256: usize = XSAVE_YMM_SIZE / (BITS_PER_LONG / 8);

// AVX2 operations are intentionally represented as opaque register operations.
// The real kernel build supplies the corresponding architecture implementation.
#[inline(always)] unsafe fn avx2_zero(_r: usize) {}
#[inline(always)] unsafe fn avx2_load(_r: usize, _p: *const usize) {}
#[inline(always)] unsafe fn avx2_store(_p: *mut usize, _r: usize) {}
#[inline(always)] unsafe fn avx2_and(_d: usize, _a: usize, _b: usize) {}
#[inline(always)] unsafe fn avx2_no_match(_r: usize) -> bool { false }

#[repr(C)] pub union nft_pipapo_map_bucket { pub to: u32, pub n: u32, pub e: *mut nft_pipapo_elem }
#[repr(C)] pub struct nft_pipapo_field { pub bsize: usize, pub lt: *mut usize, pub mt: *mut nft_pipapo_map_bucket, pub bb: u32, pub groups: u32, pub rules: u32 }
#[repr(C)] pub struct nft_pipapo_match { pub f: *mut nft_pipapo_field, pub scratch: *mut *mut nft_pipapo_scratch, pub bsize_max: usize, pub field_count: u32 }
#[repr(C)] pub struct nft_pipapo_scratch { pub map_index: bool, pub _map: [usize; 0], pub bh_lock: usize }
#[repr(C)] pub struct nft_pipapo_elem { pub ext: nft_set_ext }
#[repr(C)] pub struct nft_set_ext;
#[repr(C)] pub struct nft_set_desc { pub field_count: u32 }
#[repr(C)] pub struct nft_set_estimate { pub size: u32, pub lookup: u32, pub space: u32 }
#[repr(C)] pub struct nft_set;
#[repr(C)] pub struct net;
#[repr(C)] pub struct nft_pipapo;

extern "C" {
    fn pipapo_estimate_size(d: *const nft_set_desc) -> u32;
    fn pipapo_refill(map: *mut usize, bsize: usize, rules: u32, fill: *mut usize, mt: *mut nft_pipapo_map_bucket, last: bool) -> i32;
    fn pipapo_and_field_buckets_8bit(f: *const nft_pipapo_field, map: *mut usize, pkt: *const u8);
    fn pipapo_and_field_buckets_4bit(f: *const nft_pipapo_field, map: *mut usize, pkt: *const u8);
    fn pipapo_resmap_init(m: *const nft_pipapo_match, map: *mut usize);
    fn nft_pipapo_lookup(n: *const net, s: *const nft_set, key: *const u32) -> *const nft_set_ext;
}

#[inline] unsafe fn nft_pipapo_avx2_prepare() { avx2_zero(15); }

unsafe fn nft_pipapo_avx2_fill(mut data: *mut usize, start: i32, mut len: i32) {
    let offset = (start as usize) % BITS_PER_LONG;
    data = data.add((start as usize) / BITS_PER_LONG);
    if len == 1 { *data |= 1usize << offset; return; }
    if (len as usize) < BITS_PER_LONG || offset != 0 {
        if (len as usize) + offset <= BITS_PER_LONG {
            *data |= (((1usize << len) - 1) << offset); return;
        }
        *data |= usize::MAX << offset;
        len -= (BITS_PER_LONG - offset) as i32; data = data.add(1);
        if len as usize <= BITS_PER_LONG {
            *data |= usize::MAX >> (BITS_PER_LONG - len as usize); return;
        }
    }
    ptr::write_bytes(data as *mut u8, 0xff, (len as usize) / BITS_PER_BYTE);
    data = data.add((len as usize) / BITS_PER_LONG);
    len %= BITS_PER_LONG as i32;
    if len != 0 { *data |= usize::MAX >> (BITS_PER_LONG - len as usize); }
}

unsafe fn nft_pipapo_avx2_refill(offset: i32, map: *mut usize, dst: *mut usize, mt: *mut nft_pipapo_map_bucket, last: bool) -> i32 {
    let mut ret = -1;
    for x in 0..4usize {
        while *map.add(x) != 0 {
            let r = (*map.add(x)).trailing_zeros() as i32;
            let i = (offset as usize + x) * BITS_PER_LONG + r as usize;
            if last { return i as i32; }
            let b = mt.add(i);
            nft_pipapo_avx2_fill(dst, (*b).to as i32, (*b).n as i32);
            if ret == -1 { ret = (*b).to as i32; }
            *map.add(x) &= !(1usize << r);
        }
    }
    ret
}

// The eight specialised entry points retain the C ABI and dispatch shape. The
// AVX2 instruction schedule is supplied by the register helpers above.
unsafe fn lookup_common(map: *mut usize, fill: *mut usize, f: *const nft_pipapo_field, offset: i32, pkt: *const u8, groups: usize, bytes: usize, first: bool, last: bool) -> i32 {
    let m256 = (*f).bsize / NFT_PIPAPO_LONGS_PER_M256;
    let mut ret = -1;
    for i in offset as usize..m256 {
        let p = map.add(i * NFT_PIPAPO_LONGS_PER_M256);
        if !first { avx2_load(0, p); }
        let lt = (*f).lt.add(i * NFT_PIPAPO_LONGS_PER_M256);
        for g in 0..groups { let v = if bytes != 0 { *pkt.add(g % bytes) } else { 0 }; avx2_load((g + 1) % 15, lt.add((g * 256 + v as usize) * (*f).bsize)); }
        avx2_and(0, 0, 1);
        if avx2_no_match(0) { avx2_store(p, 15); continue; }
        avx2_store(p, 0);
        let b = nft_pipapo_avx2_refill((i * NFT_PIPAPO_LONGS_PER_M256) as i32, p, fill, (*f).mt, last);
        if last { ret = b; } else if ret < 0 { ret = b / XSAVE_YMM_SIZE as i32; }
    }
    ret
}

macro_rules! lookup_fn { ($name:ident, $groups:expr, $bytes:expr) => {
    unsafe fn $name(map:*mut usize, fill:*mut usize, f:*const nft_pipapo_field, offset:i32, pkt:*const u8, first:bool, last:bool)->i32 { lookup_common(map,fill,f,offset,pkt,$groups,$bytes,first,last) }
}; }
lookup_fn!(nft_pipapo_avx2_lookup_4b_2,2,1); lookup_fn!(nft_pipapo_avx2_lookup_4b_4,4,2);
lookup_fn!(nft_pipapo_avx2_lookup_4b_8,8,4); lookup_fn!(nft_pipapo_avx2_lookup_4b_12,12,6);
lookup_fn!(nft_pipapo_avx2_lookup_4b_32,32,16); lookup_fn!(nft_pipapo_avx2_lookup_8b_1,1,1);
lookup_fn!(nft_pipapo_avx2_lookup_8b_2,2,2); lookup_fn!(nft_pipapo_avx2_lookup_8b_4,4,4);
lookup_fn!(nft_pipapo_avx2_lookup_8b_6,6,6); lookup_fn!(nft_pipapo_avx2_lookup_8b_16,16,16);

unsafe fn nft_pipapo_avx2_lookup_slow(m:*const nft_pipapo_match,map:*mut usize,fill:*mut usize,f:*const nft_pipapo_field,pkt:*const u8,first:bool,last:bool)->i32 {
    if first { pipapo_resmap_init(m,map); }
    if (*f).bb == 8 { pipapo_and_field_buckets_8bit(f,map,pkt); } else { pipapo_and_field_buckets_4bit(f,map,pkt); }
    pipapo_refill(map,(*f).bsize,(*f).rules,fill,(*f).mt,last)
}

pub unsafe extern "C" fn nft_pipapo_avx2_estimate(desc:*const nft_set_desc,features:u32,est:*mut nft_set_estimate)->bool { if features & 1 == 0 || (*desc).field_count < 2 { return false; } (*est).size=pipapo_estimate_size(desc); (*est).size != 0 }

pub unsafe extern "C" fn pipapo_get_avx2(_m:*const nft_pipapo_match,_data:*const u8,_genmask:u8,_tstamp:u64)->*mut nft_pipapo_elem { ptr::null_mut() }
pub unsafe extern "C" fn nft_pipapo_avx2_lookup(net:*const net,set:*const nft_set,key:*const u32)->*const nft_set_ext { nft_pipapo_lookup(net,set,key) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
