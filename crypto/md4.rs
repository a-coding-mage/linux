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

pub const MD4_DIGEST_SIZE: usize = 16;
pub const MD4_HMAC_BLOCK_SIZE: usize = 64;
pub const MD4_BLOCK_WORDS: usize = 16;
pub const MD4_HASH_WORDS: usize = 4;

#[repr(C)]
pub struct md4_ctx {
    pub hash: [u32; MD4_HASH_WORDS],
    pub block: [u32; MD4_BLOCK_WORDS],
    pub byte_count: u64,
}

#[inline]
fn lshift(mut x: u32, s: u32) -> u32 {
    x &= 0xffff_ffff;
    (x.wrapping_shl(s) & 0xffff_ffff) | x.wrapping_shr(32 - s)
}

#[inline]
fn f(x: u32, y: u32, z: u32) -> u32 { (x & y) | ((!x) & z) }

#[inline]
fn g(x: u32, y: u32, z: u32) -> u32 { (x & y) | (x & z) | (y & z) }

#[inline]
fn h(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }

#[inline]
fn round1(a: &mut u32, b: u32, c: u32, d: u32, k: u32, s: u32) {
    *a = lshift(a.wrapping_add(f(b, c, d)).wrapping_add(k), s);
}
#[inline]
fn round2(a: &mut u32, b: u32, c: u32, d: u32, k: u32, s: u32) {
    *a = lshift(a.wrapping_add(g(b, c, d)).wrapping_add(k).wrapping_add(0x5A827999), s);
}
#[inline]
fn round3(a: &mut u32, b: u32, c: u32, d: u32, k: u32, s: u32) {
    *a = lshift(a.wrapping_add(h(b, c, d)).wrapping_add(k).wrapping_add(0x6ED9EBA1), s);
}

unsafe fn md4_transform(hash: *mut u32, input: *const u32) {
    let mut a = *hash.add(0); let mut b = *hash.add(1);
    let mut c = *hash.add(2); let mut d = *hash.add(3);
    macro_rules! r1 { ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr) => { round1(&mut $a,$b,$c,$d,*input.add($k),$s) }; }
    macro_rules! r2 { ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr) => { round2(&mut $a,$b,$c,$d,*input.add($k),$s) }; }
    macro_rules! r3 { ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr) => { round3(&mut $a,$b,$c,$d,*input.add($k),$s) }; }
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
    *hash.add(0) = (*hash.add(0)).wrapping_add(a); *hash.add(1) = (*hash.add(1)).wrapping_add(b);
    *hash.add(2) = (*hash.add(2)).wrapping_add(c); *hash.add(3) = (*hash.add(3)).wrapping_add(d);
}

// Kernel crypto framework declarations and module registration are external dependencies.
#[repr(C)] pub struct shash_desc { _private: [u8; 0] }
#[repr(C)] pub struct shash_alg { pub digestsize: usize, pub init: Option<unsafe extern "C" fn(*mut shash_desc)->i32>, pub update: Option<unsafe extern "C" fn(*mut shash_desc,*const u8,u32)->i32>, pub final_: Option<unsafe extern "C" fn(*mut shash_desc,*mut u8)->i32>, pub descsize: usize, pub blocksize: usize }
extern "C" { fn shash_desc_ctx(desc: *mut shash_desc) -> *mut md4_ctx; fn crypto_register_shash(alg: *mut shash_alg) -> i32; fn crypto_unregister_shash(alg: *mut shash_alg); }

unsafe fn md4_transform_helper(ctx: *mut md4_ctx) {
    for x in (*ctx).block.iter_mut() { *x = u32::from_le(*x); }
    md4_transform((*ctx).hash.as_mut_ptr(), (*ctx).block.as_ptr());
}

pub unsafe extern "C" fn md4_init(desc: *mut shash_desc) -> i32 {
    let mctx = shash_desc_ctx(desc);
    (*mctx).hash = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
    (*mctx).byte_count = 0; 0
}

pub unsafe extern "C" fn md4_update(desc: *mut shash_desc, mut data: *const u8, mut len: u32) -> i32 {
    let mctx = shash_desc_ctx(desc);
    let avail = 64 - ((*mctx).byte_count & 0x3f) as usize;
    (*mctx).byte_count = (*mctx).byte_count.wrapping_add(len as u64);
    let offset = 64 - avail;
    if avail > len as usize {
        core::ptr::copy_nonoverlapping(data, (*mctx).block.as_mut_ptr() as *mut u8 .add(offset), len as usize); return 0;
    }
    core::ptr::copy_nonoverlapping(data, (*mctx).block.as_mut_ptr() as *mut u8 .add(offset), avail);
    md4_transform_helper(mctx); data = data.add(avail); len -= avail as u32;
    while len >= 64 { core::ptr::copy_nonoverlapping(data, (*mctx).block.as_mut_ptr() as *mut u8, 64); md4_transform_helper(mctx); data=data.add(64); len-=64; }
    core::ptr::copy_nonoverlapping(data, (*mctx).block.as_mut_ptr() as *mut u8, len as usize); 0
}

pub unsafe extern "C" fn md4_final(desc: *mut shash_desc, out: *mut u8) -> i32 {
    let mctx = shash_desc_ctx(desc); let offset = ((*mctx).byte_count & 0x3f) as usize;
    let p = (*mctx).block.as_mut_ptr() as *mut u8; *p.add(offset) = 0x80;
    let mut padding = 56i32 - (offset as i32 + 1);
    if padding < 0 { core::ptr::write_bytes(p.add(offset+1), 0, (padding + 8) as usize); md4_transform_helper(mctx); padding=56; }
    core::ptr::write_bytes(p.add(offset+1), 0, padding as usize);
    (*mctx).block[14] = ((*mctx).byte_count << 3) as u32; (*mctx).block[15] = ((*mctx).byte_count >> 29) as u32;
    for x in (*mctx).block[..14].iter_mut() { *x = u32::from_le(*x); }
    md4_transform((*mctx).hash.as_mut_ptr(), (*mctx).block.as_ptr());
    for x in (*mctx).hash.iter_mut() { *x = x.to_le(); }
    core::ptr::copy_nonoverlapping((*mctx).hash.as_ptr() as *const u8, out, 16); core::ptr::write_bytes(mctx as *mut u8, 0, core::mem::size_of::<md4_ctx>()); 0
}

#[no_mangle] pub static mut alg: shash_alg = shash_alg { digestsize: MD4_DIGEST_SIZE, init: Some(md4_init), update: Some(md4_update), final_: Some(md4_final), descsize: core::mem::size_of::<md4_ctx>(), blocksize: MD4_HMAC_BLOCK_SIZE };
pub unsafe extern "C" fn md4_mod_init() -> i32 { crypto_register_shash(&mut alg) }
pub unsafe extern "C" fn md4_mod_fini() { crypto_unregister_shash(&mut alg); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
