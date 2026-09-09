// SPDX-License-Identifier: GPL-2.0
/*
 * Cryptographic API.
 *
 * MD4 Message Digest Algorithm (RFC1320).
 *
 * Implementation derived from Andrew Tridgell and Steve French's
 * CIFS MD4 implementation, and the cryptoapi implementation
 * originally based on the public domain implementation written
 * by Colin Plumb in 1993.
 *
 * Copyright (c) Andrew Tridgell 1997-1998.
 * Modified by Steve French (sfrench@us.ibm.com) 2002
 * Copyright (c) Cryptoapi developers.
 * Copyright (c) 2002 David S. Miller (davem@redhat.com)
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 */

use core::ptr;

use crate::md4_ctx;

#[inline]
fn lshift(mut x: u32, s: u32) -> u32 {
    x &= 0xffff_ffff;
    x.rotate_left(s)
}

#[inline]
fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | ((!x) & z)
}

#[inline]
fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

#[inline]
fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[inline]
fn round1(a: &mut u32, b: u32, c: u32, d: u32, k: u32, s: u32) {
    *a = lshift(a.wrapping_add(f(b, c, d)).wrapping_add(k), s);
}

#[inline]
fn round2(a: &mut u32, b: u32, c: u32, d: u32, k: u32, s: u32) {
    *a = lshift(a.wrapping_add(g(b, c, d)).wrapping_add(k).wrapping_add(0x5a82_7999), s);
}

#[inline]
fn round3(a: &mut u32, b: u32, c: u32, d: u32, k: u32, s: u32) {
    *a = lshift(a.wrapping_add(h(b, c, d)).wrapping_add(k).wrapping_add(0x6ed9_eba1), s);
}

unsafe fn md4_transform(hash: *mut u32, input: *const u32) {
    let mut a = *hash.add(0);
    let mut b = *hash.add(1);
    let mut c = *hash.add(2);
    let mut d = *hash.add(3);

    macro_rules! r1 { ($a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr) => { round1(&mut $a, $b, $c, $d, *input.add($k), $s); }; }
    macro_rules! r2 { ($a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr) => { round2(&mut $a, $b, $c, $d, *input.add($k), $s); }; }
    macro_rules! r3 { ($a:ident, $b:ident, $c:ident, $d:ident, $k:expr, $s:expr) => { round3(&mut $a, $b, $c, $d, *input.add($k), $s); }; }

    r1!(a,b,c,d,0,3); r1!(d,a,b,c,1,7); r1!(c,d,a,b,2,11); r1!(b,c,d,a,3,19);
    r1!(a,b,c,d,4,3); r1!(d,a,b,c,5,7); r1!(c,d,a,b,6,11); r1!(b,c,d,a,7,19);
    r1!(a,b,c,d,8,3); r1!(d,a,b,c,9,7); r1!(c,d,a,b,10,11); r1!(b,c,d,a,11,19);
    r1!(a,b,c,d,12,3); r1!(d,a,b,c,13,7); r1!(c,d,a,b,14,11); r1!(b,c,d,a,15,19);
    r2!(a,b,c,d,0,3); r2!(d,a,b,c,4,5); r2!(c,d,a,b,8,9); r2!(b,c,d,a,12,13);
    r2!(a,b,c,d,1,3); r2!(d,a,b,c,5,5); r2!(c,d,a,b,9,9); r2!(b,c,d,a,13,13);
    r2!(a,b,c,d,2,3); r2!(d,a,b,c,6,5); r2!(c,d,a,b,10,9); r2!(b,c,d,a,14,13);
    r2!(a,b,c,d,3,3); r2!(d,a,b,c,7,5); r2!(c,d,a,b,11,9); r2!(b,c,d,a,15,13);
    r3!(a,b,c,d,0,3); r3!(d,a,b,c,8,9); r3!(c,d,a,b,4,11); r3!(b,c,d,a,12,15);
    r3!(a,b,c,d,2,3); r3!(d,a,b,c,10,9); r3!(c,d,a,b,6,11); r3!(b,c,d,a,14,15);
    r3!(a,b,c,d,1,3); r3!(d,a,b,c,9,9); r3!(c,d,a,b,5,11); r3!(b,c,d,a,13,15);
    r3!(a,b,c,d,3,3); r3!(d,a,b,c,11,9); r3!(c,d,a,b,7,11); r3!(b,c,d,a,15,15);

    *hash.add(0) = (*hash.add(0)).wrapping_add(a);
    *hash.add(1) = (*hash.add(1)).wrapping_add(b);
    *hash.add(2) = (*hash.add(2)).wrapping_add(c);
    *hash.add(3) = (*hash.add(3)).wrapping_add(d);
}

#[inline]
unsafe fn md4_transform_helper(ctx: *mut md4_ctx) {
    for i in 0..16 {
        (*ctx).block[i] = u32::from_le((*ctx).block[i]);
    }
    md4_transform((*ctx).hash.as_mut_ptr(), (*ctx).block.as_ptr());
}

pub unsafe fn cifs_md4_init(mctx: *mut md4_ctx) -> i32 {
    ptr::write_bytes(mctx as *mut u8, 0, core::mem::size_of::<md4_ctx>());
    (*mctx).hash[0] = 0x6745_2301;
    (*mctx).hash[1] = 0xefcd_ab89;
    (*mctx).hash[2] = 0x98ba_dcfe;
    (*mctx).hash[3] = 0x1032_5476;
    (*mctx).byte_count = 0;
    0
}

pub unsafe fn cifs_md4_update(mctx: *mut md4_ctx, mut data: *const u8, mut len: u32) -> i32 {
    let avail = (core::mem::size_of_val(&(*mctx).block) as u64
        - ((*mctx).byte_count & 0x3f)) as usize;
    (*mctx).byte_count = (*mctx).byte_count.wrapping_add(len as _);
    if avail > len as usize {
        ptr::copy_nonoverlapping(data, (*mctx).block.as_mut_ptr() as *mut u8
            .add(core::mem::size_of_val(&(*mctx).block) - avail), len as usize);
        return 0;
    }
    ptr::copy_nonoverlapping(data, (*mctx).block.as_mut_ptr() as *mut u8
        .add(core::mem::size_of_val(&(*mctx).block) - avail), avail);
    md4_transform_helper(mctx);
    data = data.add(avail);
    len -= avail as u32;
    while (len as usize) >= core::mem::size_of_val(&(*mctx).block) {
        ptr::copy_nonoverlapping(data, (*mctx).block.as_mut_ptr() as *mut u8,
            core::mem::size_of_val(&(*mctx).block));
        md4_transform_helper(mctx);
        data = data.add(core::mem::size_of_val(&(*mctx).block));
        len -= core::mem::size_of_val(&(*mctx).block) as u32;
    }
    ptr::copy_nonoverlapping(data, (*mctx).block.as_mut_ptr() as *mut u8, len as usize);
    0
}

pub unsafe fn cifs_md4_final(mctx: *mut md4_ctx, out: *mut u8) -> i32 {
    let offset = ((*mctx).byte_count & 0x3f) as usize;
    let block = (*mctx).block.as_mut_ptr() as *mut u8;
    let mut p = block.add(offset);
    let mut padding = 56isize - (offset as isize + 1);
    *p = 0x80;
    p = p.add(1);
    if padding < 0 {
        ptr::write_bytes(p, 0, (padding + core::mem::size_of::<u64>() as isize) as usize);
        md4_transform_helper(mctx);
        p = block;
        padding = 56;
    }
    ptr::write_bytes(p, 0, padding as usize);
    (*mctx).block[14] = ((*mctx).byte_count as u32).wrapping_shl(3);
    (*mctx).block[15] = ((*mctx).byte_count as u32) >> 29;
    for i in 0..14 {
        (*mctx).block[i] = u32::from_le((*mctx).block[i]);
    }
    md4_transform((*mctx).hash.as_mut_ptr(), (*mctx).block.as_ptr());
    for i in 0..4 {
        (*mctx).hash[i] = (*mctx).hash[i].to_le();
    }
    ptr::copy_nonoverlapping((*mctx).hash.as_ptr() as *const u8, out,
        core::mem::size_of_val(&(*mctx).hash));
    ptr::write_bytes(mctx as *mut u8, 0, core::mem::size_of::<md4_ctx>());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
