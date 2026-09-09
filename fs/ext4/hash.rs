// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ext4/hash.c
 *
 * Copyright (C) 2002 by Theodore Ts'o
 */

// Dependencies supplied by the surrounding kernel/ext4 translation.

const DELTA: u32 = 0x9E3779B9;

unsafe fn TEA_transform(buf: *mut u32, input: *const u32) {
    let mut sum: u32 = 0;
    let mut b0 = *buf.add(0);
    let mut b1 = *buf.add(1);
    let a = *input.add(0);
    let b = *input.add(1);
    let c = *input.add(2);
    let d = *input.add(3);
    let mut n: i32 = 16;

    loop {
        sum = sum.wrapping_add(DELTA);
        b0 = b0.wrapping_add(((b1 << 4).wrapping_add(a)) ^ b1.wrapping_add(sum) ^ ((b1 >> 5).wrapping_add(b)));
        b1 = b1.wrapping_add(((b0 << 4).wrapping_add(c)) ^ b0.wrapping_add(sum) ^ ((b0 >> 5).wrapping_add(d)));
        n -= 1;
        if n == 0 { break; }
    }

    *buf.add(0) = (*buf.add(0)).wrapping_add(b0);
    *buf.add(1) = (*buf.add(1)).wrapping_add(b1);
}

#[inline]
fn rol32(x: u32, s: u32) -> u32 { x.rotate_left(s) }

#[inline]
fn f(x: u32, y: u32, z: u32) -> u32 { z ^ (x & (y ^ z)) }
#[inline]
fn g(x: u32, y: u32, z: u32) -> u32 { (x & y).wrapping_add((x ^ y) & z) }
#[inline]
fn h(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }

unsafe fn half_md4_transform(buf: *mut u32, input: *const u32) -> u32 {
    let mut a = *buf.add(0); let mut b = *buf.add(1);
    let mut c = *buf.add(2); let mut d = *buf.add(3);
    macro_rules! round { ($fun:ident, $aa:ident, $bb:ident, $cc:ident, $dd:ident, $x:expr, $s:expr) => {
        $aa = $aa.wrapping_add($fun($bb, $cc, $dd)).wrapping_add($x); $aa = rol32($aa, $s);
    }; }
    round!(f, a,b,c,d, *input.add(0), 3); round!(f, d,a,b,c, *input.add(1), 7);
    round!(f, c,d,a,b, *input.add(2), 11); round!(f, b,c,d,a, *input.add(3), 19);
    round!(f, a,b,c,d, *input.add(4), 3); round!(f, d,a,b,c, *input.add(5), 7);
    round!(f, c,d,a,b, *input.add(6), 11); round!(f, b,c,d,a, *input.add(7), 19);
    const K2: u32 = 0o13240474631; const K3: u32 = 0o15666365641;
    round!(g, a,b,c,d, (*input.add(1)).wrapping_add(K2), 3); round!(g, d,a,b,c, (*input.add(3)).wrapping_add(K2), 5);
    round!(g, c,d,a,b, (*input.add(5)).wrapping_add(K2), 9); round!(g, b,c,d,a, (*input.add(7)).wrapping_add(K2), 13);
    round!(g, a,b,c,d, (*input.add(0)).wrapping_add(K2), 3); round!(g, d,a,b,c, (*input.add(2)).wrapping_add(K2), 5);
    round!(g, c,d,a,b, (*input.add(4)).wrapping_add(K2), 9); round!(g, b,c,d,a, (*input.add(6)).wrapping_add(K2), 13);
    round!(h, a,b,c,d, (*input.add(3)).wrapping_add(K3), 3); round!(h, d,a,b,c, (*input.add(7)).wrapping_add(K3), 9);
    round!(h, c,d,a,b, (*input.add(2)).wrapping_add(K3), 11); round!(h, b,c,d,a, (*input.add(6)).wrapping_add(K3), 15);
    round!(h, a,b,c,d, (*input.add(1)).wrapping_add(K3), 3); round!(h, d,a,b,c, (*input.add(5)).wrapping_add(K3), 9);
    round!(h, c,d,a,b, (*input.add(0)).wrapping_add(K3), 11); round!(h, b,c,d,a, (*input.add(4)).wrapping_add(K3), 15);
    *buf.add(0)=(*buf.add(0)).wrapping_add(a); *buf.add(1)=(*buf.add(1)).wrapping_add(b);
    *buf.add(2)=(*buf.add(2)).wrapping_add(c); *buf.add(3)=(*buf.add(3)).wrapping_add(d); *buf.add(1)
}

unsafe fn dx_hack_hash_unsigned(name: *const i8, mut len: i32) -> u32 {
    let (mut hash0, mut hash1) = (0x12a3fe2d_u32, 0x37abe8f9_u32); let mut p=name as *const u8;
    while len > 0 { let hash=hash1.wrapping_add(hash0 ^ ((*p as i32 * 7152373) as u32)); let hash=if hash&0x80000000!=0 {hash.wrapping_sub(0x7fffffff)} else {hash}; hash1=hash0; hash0=hash; p=p.add(1); len-=1; } hash0<<1
}
unsafe fn dx_hack_hash_signed(name: *const i8, mut len: i32) -> u32 {
    let (mut hash0, mut hash1) = (0x12a3fe2d_u32, 0x37abe8f9_u32); let mut p=name as *const i8;
    while len > 0 { let hash=hash1.wrapping_add(hash0 ^ ((*p as i32 * 7152373) as u32)); let hash=if hash&0x80000000!=0 {hash.wrapping_sub(0x7fffffff)} else {hash}; hash1=hash0; hash0=hash; p=p.add(1); len-=1; } hash0<<1
}

// The remaining ext4-facing implementation is kept in direct pointer-oriented form.
// External kernel types and helpers are supplied by the surrounding translation.
unsafe fn str2hashbuf_signed(msg: *const i8, mut len: i32, mut buf: *mut u32, mut num: i32) {
    let mut pad = len as u32 | ((len as u32) << 8); pad |= pad << 16;
    if len > num * 4 { len = num * 4; }
    let mut p = msg; while len >= 4 { let v=((*p.add(0) as i32 as u32)<<24)|((*p.add(1) as i32 as u32)<<16)|((*p.add(2) as i32 as u32)<<8)|(*p.add(3) as i32 as u32); *buf=v; buf=buf.add(1); p=p.add(4); len-=4; num-=1; }
    let mut val=pad; for i in 0..len { val=(*p.add(i as usize) as i32 as u32).wrapping_add(val<<8); } num-=1; if num>=0 { *buf=val; buf=buf.add(1); } while {num-=1; num>=0} { *buf=pad; buf=buf.add(1); }
}
unsafe fn str2hashbuf_unsigned(msg: *const i8, mut len: i32, mut buf: *mut u32, mut num: i32) {
    let mut pad = len as u32 | ((len as u32)<<8); pad |= pad<<16; if len>num*4 {len=num*4;} let mut p=msg as *const u8;
    while len>=4 { *buf=u32::from_be_bytes([*p,*p.add(1),*p.add(2),*p.add(3)]); buf=buf.add(1); p=p.add(4); len-=4; num-=1; } let mut val=pad; for i in 0..len {val=(*p.add(i as usize) as u32).wrapping_add(val<<8);} num-=1; if num>=0 {*buf=val;buf=buf.add(1);} while {num-=1;num>=0}{*buf=pad;buf=buf.add(1);}
}

// Constants and helpers below are supplied by ext4.h and the kernel headers.
unsafe fn __ext4fs_dirhash(dir: *const inode, name: *const i8, mut len: i32, hinfo: *mut dx_hash_info) -> i32 {
    let mut hash: u32; let mut minor_hash=0u32; let mut input=[0u32;8]; let mut buf=[0u32;4]; let mut unsigned=false;
    buf=[0x67452301,0xefcdab89,0x98badcfe,0x10325476];
    if !(*hinfo).seed.is_null() { for i in 0..4 { if *(*hinfo).seed.add(i)!=0 { core::ptr::copy_nonoverlapping((*hinfo).seed,buf.as_mut_ptr(),4); break; } } }
    match (*hinfo).hash_version { DX_HASH_LEGACY_UNSIGNED=>hash=dx_hack_hash_unsigned(name,len), DX_HASH_LEGACY=>hash=dx_hack_hash_signed(name,len), DX_HASH_HALF_MD4_UNSIGNED|DX_HASH_HALF_MD4=>{unsigned=(*hinfo).hash_version==DX_HASH_HALF_MD4_UNSIGNED; while len>0 {if unsigned {str2hashbuf_unsigned(name,len,input.as_mut_ptr(),8)} else {str2hashbuf_signed(name,len,input.as_mut_ptr(),8)} half_md4_transform(buf.as_mut_ptr(),input.as_ptr());len-=32;} minor_hash=buf[2];hash=buf[1];}, DX_HASH_TEA_UNSIGNED|DX_HASH_TEA=>{unsigned=(*hinfo).hash_version==DX_HASH_TEA_UNSIGNED;while len>0{if unsigned{str2hashbuf_unsigned(name,len,input.as_mut_ptr(),4)}else{str2hashbuf_signed(name,len,input.as_mut_ptr(),4)}TEA_transform(buf.as_mut_ptr(),input.as_ptr());len-=16;}hash=buf[0];minor_hash=buf[1];}, _=>{(*hinfo).hash=0;(*hinfo).minor_hash=0;return -22;} }
    hash &= !1; if hash == (EXT4_HTREE_EOF_32BIT<<1) {hash=(EXT4_HTREE_EOF_32BIT-1)<<1;} (*hinfo).hash=hash;(*hinfo).minor_hash=minor_hash;0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
