// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/* Copyright (C) 2016-2022 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * SipHash: a fast short-input PRF
 * https://131002.net/siphash/
 *
 * This implementation is specifically for SipHash2-4 for a secure PRF
 * and HalfSipHash1-3/SipHash1-3 for an insecure PRF only suitable for
 * hashtables.
 */

// The following types, constants, permutations, and unaligned accessors are
// supplied by the corresponding kernel headers.

macro_rules! sipround { ($v0:ident, $v1:ident, $v2:ident, $v3:ident) => { SIPHASH_PERMUTATION!($v0, $v1, $v2, $v3); }; }

#[cfg(not(feature = "config_have_efficient_unaligned_access"))]
pub unsafe fn __siphash_aligned(data: *const u8, len: usize, key: *const siphash_key_t) -> u64 {
    let end = data.add(len - (len % core::mem::size_of::<u64>()));
    let left = (len & (core::mem::size_of::<u64>() - 1)) as u8;
    let mut v0 = SIPHASH_CONST_0;
    let mut v1 = SIPHASH_CONST_1;
    let mut v2 = SIPHASH_CONST_2;
    let mut v3 = SIPHASH_CONST_3;
    let mut b = (len as u64) << 56;
    v3 ^= (*key).key[1]; v2 ^= (*key).key[0]; v1 ^= (*key).key[1]; v0 ^= (*key).key[0];
    let mut p = data;
    while p != end {
        let m = le64_to_cpup(p);
        v3 ^= m; sipround!(v0, v1, v2, v3); sipround!(v0, v1, v2, v3); v0 ^= m;
        p = p.add(8);
    }
    match left {
        7 => { b |= (*end.add(6) as u64) << 48; b |= (*end.add(5) as u64) << 40; b |= (*end.add(4) as u64) << 32; b |= le32_to_cpup(p) as u64; }
        6 => { b |= (*end.add(5) as u64) << 40; b |= (*end.add(4) as u64) << 32; b |= le32_to_cpup(p) as u64; }
        5 => { b |= (*end.add(4) as u64) << 32; b |= le32_to_cpup(p) as u64; }
        4 => { b |= le32_to_cpup(p) as u64; }
        3 => { b |= (*end.add(2) as u64) << 16; b |= le16_to_cpup(p) as u64; }
        2 => { b |= le16_to_cpup(p) as u64; }
        1 => { b |= *end as u64; }
        _ => {}
    }
    v3 ^= b; sipround!(v0,v1,v2,v3); sipround!(v0,v1,v2,v3); v0 ^= b; v2 ^= 0xff;
    sipround!(v0,v1,v2,v3); sipround!(v0,v1,v2,v3); sipround!(v0,v1,v2,v3); sipround!(v0,v1,v2,v3);
    (v0 ^ v1) ^ (v2 ^ v3)
}

pub unsafe fn __siphash_unaligned(data: *const u8, len: usize, key: *const siphash_key_t) -> u64 {
    let end = data.add(len - (len % 8)); let left = (len & 7) as u8;
    let mut v0 = SIPHASH_CONST_0; let mut v1 = SIPHASH_CONST_1; let mut v2 = SIPHASH_CONST_2; let mut v3 = SIPHASH_CONST_3;
    let mut b = (len as u64) << 56; v3 ^= (*key).key[1]; v2 ^= (*key).key[0]; v1 ^= (*key).key[1]; v0 ^= (*key).key[0];
    let mut p = data;
    while p != end { let m = get_unaligned_le64(p); v3 ^= m; sipround!(v0,v1,v2,v3); sipround!(v0,v1,v2,v3); v0 ^= m; p = p.add(8); }
    match left { 7=>{b|=(*end.add(6) as u64)<<48;b|=(*end.add(5) as u64)<<40;b|=(*end.add(4) as u64)<<32;b|=get_unaligned_le32(end) as u64;},6=>{b|=(*end.add(5) as u64)<<40;b|=(*end.add(4) as u64)<<32;b|=get_unaligned_le32(end) as u64;},5=>{b|=(*end.add(4) as u64)<<32;b|=get_unaligned_le32(end) as u64;},4=>b|=get_unaligned_le32(end) as u64,3=>{b|=(*end.add(2) as u64)<<16;b|=get_unaligned_le16(end) as u64;},2=>b|=get_unaligned_le16(end) as u64,1=>b|=*end as u64,_=>{}}
    v3^=b;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=b;v2^=0xff;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);(v0^v1)^(v2^v3)
}

macro_rules! sip_u { ($name:ident, ($($arg:ident),*), $len:expr, $body:block) => { pub unsafe fn $name($($arg: u64,)* key: *const siphash_key_t) -> u64 { let mut v0=SIPHASH_CONST_0;let mut v1=SIPHASH_CONST_1;let mut v2=SIPHASH_CONST_2;let mut v3=SIPHASH_CONST_3;let mut b=($len as u64)<<56;v3^=(*key).key[1];v2^=(*key).key[0];v1^=(*key).key[1];v0^=(*key).key[0]; $body v3^=b;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=b;v2^=0xff;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);(v0^v1)^(v2^v3) } }; }
}
sip_u!(siphash_1u64, (first), 8, { v3^=first;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=first; });
sip_u!(siphash_2u64, (first,second), 16, { v3^=first;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=first;v3^=second;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=second; });
sip_u!(siphash_3u64, (first,second,third), 24, { v3^=first;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=first;v3^=second;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=second;v3^=third;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=third; });
sip_u!(siphash_4u64, (first,second,third,forth), 32, { v3^=first;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=first;v3^=second;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=second;v3^=third;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=third;v3^=forth;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=forth; });

pub unsafe fn siphash_1u32(first:u32,key:*const siphash_key_t)->u64 { let mut v0=SIPHASH_CONST_0;let mut v1=SIPHASH_CONST_1;let mut v2=SIPHASH_CONST_2;let mut v3=SIPHASH_CONST_3;let mut b=4u64<<56;v3^=(*key).key[1];v2^=(*key).key[0];v1^=(*key).key[1];v0^=(*key).key[0];b|=first as u64;v3^=b;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=b;v2^=0xff;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);(v0^v1)^(v2^v3) }

pub unsafe fn siphash_3u32(first:u32,second:u32,third:u32,key:*const siphash_key_t)->u64 { let combined=(second as u64)<<32|first as u64;let mut v0=SIPHASH_CONST_0;let mut v1=SIPHASH_CONST_1;let mut v2=SIPHASH_CONST_2;let mut v3=SIPHASH_CONST_3;let mut b=12u64<<56;v3^=(*key).key[1];v2^=(*key).key[0];v1^=(*key).key[1];v0^=(*key).key[0];v3^=combined;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=combined;b|=third as u64;v3^=b;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);v0^=b;v2^=0xff;sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);sipround!(v0,v1,v2,v3);(v0^v1)^(v2^v3) }

// HalfSipHash implementations retain the source architecture split. The
// 64-bit branch uses the SipHash permutation and 64-bit state.
#[cfg(target_pointer_width = "64")]
pub unsafe fn hsiphash_1u32(first:u32,key:*const hsiphash_key_t)->u32 { (siphash_1u32(first,key as *const siphash_key_t)) as u32 }
#[cfg(target_pointer_width = "64")]
pub unsafe fn hsiphash_2u32(first:u32,second:u32,key:*const hsiphash_key_t)->u32 { let c=((second as u64)<<32)|first as u64; (siphash_1u64(c,key as *const siphash_key_t)) as u32 }
#[cfg(target_pointer_width = "64")]
pub unsafe fn hsiphash_3u32(first:u32,second:u32,third:u32,key:*const hsiphash_key_t)->u32 { let c=((second as u64)<<32)|first as u64; (siphash_3u32(first,second,third,key as *const siphash_key_t)) as u32 }
#[cfg(target_pointer_width = "64")]
pub unsafe fn hsiphash_4u32(first:u32,second:u32,third:u32,forth:u32,key:*const hsiphash_key_t)->u32 { let a=((second as u64)<<32)|first as u64;let b=((forth as u64)<<32)|third as u64;(siphash_2u64(a,b,key as *const siphash_key_t)) as u32 }

// 32-bit HalfSipHash branch: external HSIPHASH_PERMUTATION and constants are
// provided by the included kernel definitions.
#[cfg(target_pointer_width = "32")]
macro_rules! hsipround { ($a:ident,$b:ident,$c:ident,$d:ident) => { HSIPHASH_PERMUTATION!($a,$b,$c,$d); }; }
#[cfg(target_pointer_width = "32")]
macro_rules! hsip_finish { ($v0:ident,$v1:ident,$v2:ident,$v3:ident,$b:ident) => {{ $v3^=$b;hsipround!($v0,$v1,$v2,$v3);$v0^=$b;$v2^=0xff;hsipround!($v0,$v1,$v2,$v3);hsipround!($v0,$v1,$v2,$v3);hsipround!($v0,$v1,$v2,$v3);$v1^$v3 }}; }
#[cfg(target_pointer_width = "32")]
pub unsafe fn hsiphash_1u32(first:u32,key:*const hsiphash_key_t)->u32 { let mut v0=HSIPHASH_CONST_0;let mut v1=HSIPHASH_CONST_1;let mut v2=HSIPHASH_CONST_2;let mut v3=HSIPHASH_CONST_3;let mut b=4u32<<24;v3^=(*key).key[1];v2^=(*key).key[0];v1^=(*key).key[1];v0^=(*key).key[0];v3^=first;hsipround!(v0,v1,v2,v3);v0^=first;hsip_finish!(v0,v1,v2,v3,b) }
#[cfg(target_pointer_width = "32")]
pub unsafe fn hsiphash_2u32(first:u32,second:u32,key:*const hsiphash_key_t)->u32 { let mut v0=HSIPHASH_CONST_0;let mut v1=HSIPHASH_CONST_1;let mut v2=HSIPHASH_CONST_2;let mut v3=HSIPHASH_CONST_3;let mut b=8u32<<24;v3^=(*key).key[1];v2^=(*key).key[0];v1^=(*key).key[1];v0^=(*key).key[0];v3^=first;hsipround!(v0,v1,v2,v3);v0^=first;v3^=second;hsipround!(v0,v1,v2,v3);v0^=second;hsip_finish!(v0,v1,v2,v3,b) }
#[cfg(target_pointer_width = "32")]
pub unsafe fn hsiphash_3u32(first:u32,second:u32,third:u32,key:*const hsiphash_key_t)->u32 { let mut v0=HSIPHASH_CONST_0;let mut v1=HSIPHASH_CONST_1;let mut v2=HSIPHASH_CONST_2;let mut v3=HSIPHASH_CONST_3;let mut b=12u32<<24;v3^=(*key).key[1];v2^=(*key).key[0];v1^=(*key).key[1];v0^=(*key).key[0];v3^=first;hsipround!(v0,v1,v2,v3);v0^=first;v3^=second;hsipround!(v0,v1,v2,v3);v0^=second;v3^=third;hsipround!(v0,v1,v2,v3);v0^=third;hsip_finish!(v0,v1,v2,v3,b) }
#[cfg(target_pointer_width = "32")]
pub unsafe fn hsiphash_4u32(first:u32,second:u32,third:u32,forth:u32,key:*const hsiphash_key_t)->u32 { let mut v0=HSIPHASH_CONST_0;let mut v1=HSIPHASH_CONST_1;let mut v2=HSIPHASH_CONST_2;let mut v3=HSIPHASH_CONST_3;let mut b=16u32<<24;v3^=(*key).key[1];v2^=(*key).key[0];v1^=(*key).key[1];v0^=(*key).key[0];v3^=first;hsipround!(v0,v1,v2,v3);v0^=first;v3^=second;hsipround!(v0,v1,v2,v3);v0^=second;v3^=third;hsipround!(v0,v1,v2,v3);v0^=third;v3^=forth;hsipround!(v0,v1,v2,v3);v0^=forth;hsip_finish!(v0,v1,v2,v3,b) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
