// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright IBM Corp. 2024
 *
 * s390 specific HMAC support.
 */

// Kernel headers and build-time configuration are supplied by the surrounding
// Rust kernel environment.

const MAX_DIGEST_SIZE: usize = SHA512_DIGEST_SIZE;
const MAX_IMBL_SIZE: usize = core::mem::size_of::<u128>();
const MAX_BLOCK_SIZE: usize = SHA512_BLOCK_SIZE;

#[inline]
const fn sha2_cv_size(bs: usize) -> usize { bs >> 1 }
#[inline]
const fn sha2_imbl_size(bs: usize) -> usize { bs >> 3 }
#[inline]
const fn sha2_imbl_offset(bs: usize) -> usize { sha2_cv_size(bs) }
#[inline]
const fn sha2_key_offset(bs: usize) -> usize { sha2_cv_size(bs) + sha2_imbl_size(bs) }

#[repr(C)]
pub struct S390HmacCtx { pub key: [u8; MAX_BLOCK_SIZE] }

#[repr(C)]
pub union S390KmacGr0 {
    pub reg: libc::c_ulong,
    pub bits: S390KmacGr0Bits,
}
#[repr(C)]
pub struct S390KmacGr0Bits {
    pub _reserved0: libc::c_ulong,
    pub ikp: libc::c_ulong,
    pub iimp: libc::c_ulong,
    pub ccup: libc::c_ulong,
    pub _reserved1: libc::c_ulong,
    pub fc: libc::c_ulong,
}

#[repr(C)]
pub struct S390KmacSha2Ctx {
    pub param: [u8; MAX_DIGEST_SIZE + MAX_IMBL_SIZE + MAX_BLOCK_SIZE],
    pub gr0: S390KmacGr0,
    pub buflen: [u64; 2],
}

#[inline]
unsafe fn kmac_sha2_set_imbl(param: *mut u8, buflen_lo: u64, buflen_hi: u64, blocksize: u32) {
    let imbl = param.add(sha2_imbl_offset(blocksize as usize));
    match blocksize as usize {
        SHA256_BLOCK_SIZE => (imbl as *mut u64).write_unaligned(buflen_lo.wrapping_mul(BITS_PER_BYTE as u64)),
        SHA512_BLOCK_SIZE => (imbl as *mut u128).write_unaligned((((buflen_hi as u128) << 64) + buflen_lo as u128) << 3),
        _ => (),
    }
}

unsafe fn hash_data(in_: *const u8, inlen: u32, digest: *mut u8, mut digestsize: u32, final_: bool) -> i32 {
    let mut param = [0u8; 144];
    let func: u32;
    match digestsize as usize {
        SHA224_DIGEST_SIZE => { func = if final_ { CPACF_KLMD_SHA_256 } else { CPACF_KIMD_SHA_256 }; if !final_ { digestsize = SHA256_DIGEST_SIZE as u32; } }
        SHA256_DIGEST_SIZE => { func = if final_ { CPACF_KLMD_SHA_256 } else { CPACF_KIMD_SHA_256 }; }
        SHA384_DIGEST_SIZE => { func = if final_ { CPACF_KLMD_SHA_512 } else { CPACF_KIMD_SHA_512 }; if !final_ { digestsize = SHA512_DIGEST_SIZE as u32; } }
        SHA512_DIGEST_SIZE => { func = if final_ { CPACF_KLMD_SHA_512 } else { CPACF_KIMD_SHA_512 }; }
        _ => return -EINVAL,
    }
    cpacf_klmd(func, param.as_mut_ptr(), in_, inlen);
    core::ptr::copy_nonoverlapping(param.as_ptr(), digest, digestsize as usize);
    0
}

unsafe fn hash_key(in_: *const u8, inlen: u32, digest: *mut u8, digestsize: u32) -> i32 {
    hash_data(in_, inlen, digest, digestsize, true)
}

unsafe fn s390_hmac_sha2_setkey(tfm: *mut crypto_shash, key: *const u8, keylen: u32) -> i32 {
    let tfm_ctx = crypto_shash_ctx(tfm) as *mut S390HmacCtx;
    let ds = crypto_shash_digestsize(tfm);
    let bs = crypto_shash_blocksize(tfm);
    core::ptr::write_bytes(tfm_ctx as *mut u8, 0, core::mem::size_of::<S390HmacCtx>());
    if keylen > bs { return hash_key(key, keylen, (*tfm_ctx).key.as_mut_ptr(), ds); }
    core::ptr::copy_nonoverlapping(key, (*tfm_ctx).key.as_mut_ptr(), keylen as usize);
    0
}

unsafe fn s390_hmac_sha2_init(desc: *mut shash_desc) -> i32 {
    let tfm = (*desc).tfm;
    let tfm_ctx = crypto_shash_ctx(tfm) as *mut S390HmacCtx;
    let ctx = shash_desc_ctx(desc) as *mut S390KmacSha2Ctx;
    let bs = crypto_shash_blocksize(tfm) as usize;
    core::ptr::copy_nonoverlapping((*tfm_ctx).key.as_ptr(), (*ctx).param.as_mut_ptr().add(sha2_key_offset(bs)), bs);
    (*ctx).buflen = [0, 0];
    (*ctx).gr0.reg = 0;
    (*ctx).gr0.bits.fc = match crypto_shash_digestsize(tfm) as usize {
        SHA224_DIGEST_SIZE => CPACF_KMAC_HMAC_SHA_224 as libc::c_ulong,
        SHA256_DIGEST_SIZE => CPACF_KMAC_HMAC_SHA_256 as libc::c_ulong,
        SHA384_DIGEST_SIZE => CPACF_KMAC_HMAC_SHA_384 as libc::c_ulong,
        SHA512_DIGEST_SIZE => CPACF_KMAC_HMAC_SHA_512 as libc::c_ulong,
        _ => return -EINVAL,
    };
    0
}

unsafe fn s390_hmac_sha2_update(desc: *mut shash_desc, data: *const u8, len: u32) -> i32 {
    let ctx = shash_desc_ctx(desc) as *mut S390KmacSha2Ctx;
    let bs = crypto_shash_blocksize((*desc).tfm);
    let n = len - (len % bs);
    (*ctx).buflen[0] = (*ctx).buflen[0].wrapping_add(n as u64);
    if (*ctx).buflen[0] < n as u64 { (*ctx).buflen[1] = (*ctx).buflen[1].wrapping_add(1); }
    (*ctx).gr0.bits.iimp = 1;
    _cpacf_kmac(&mut (*ctx).gr0.reg, (*ctx).param.as_mut_ptr(), data, n);
    (len - n) as i32
}

unsafe fn s390_hmac_sha2_finup(desc: *mut shash_desc, src: *const u8, len: u32, out: *mut u8) -> i32 {
    let ctx = shash_desc_ctx(desc) as *mut S390KmacSha2Ctx;
    let bs = crypto_shash_blocksize((*desc).tfm);
    (*ctx).buflen[0] = (*ctx).buflen[0].wrapping_add(len as u64);
    if (*ctx).buflen[0] < len as u64 { (*ctx).buflen[1] = (*ctx).buflen[1].wrapping_add(1); }
    (*ctx).gr0.bits.iimp = 0;
    kmac_sha2_set_imbl((*ctx).param.as_mut_ptr(), (*ctx).buflen[0], (*ctx).buflen[1], bs);
    _cpacf_kmac(&mut (*ctx).gr0.reg, (*ctx).param.as_mut_ptr(), src, len);
    core::ptr::copy_nonoverlapping((*ctx).param.as_ptr(), out, crypto_shash_digestsize((*desc).tfm) as usize);
    0
}

unsafe fn s390_hmac_sha2_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 {
    let ctx = shash_desc_ctx(desc) as *mut S390KmacSha2Ctx;
    let ds = crypto_shash_digestsize((*desc).tfm);
    let rc = s390_hmac_sha2_init(desc); if rc != 0 { return rc; }
    (*ctx).gr0.bits.iimp = 0;
    kmac_sha2_set_imbl((*ctx).param.as_mut_ptr(), len as u64, 0, crypto_shash_blocksize((*desc).tfm));
    _cpacf_kmac(&mut (*ctx).gr0.reg, (*ctx).param.as_mut_ptr(), data, len);
    core::ptr::copy_nonoverlapping((*ctx).param.as_ptr(), out, ds as usize);
    0
}

unsafe fn s390_hmac_export_zero(desc: *mut shash_desc, out: *mut u8) -> i32 {
    let tfm = (*desc).tfm;
    let mut ipad = [0u8; SHA512_BLOCK_SIZE];
    let ctx = crypto_shash_ctx(tfm) as *mut S390HmacCtx;
    let bs = crypto_shash_blocksize(tfm) as usize;
    for i in 0..bs { ipad[i] = (*ctx).key[i] ^ HMAC_IPAD_VALUE; }
    let err = hash_data(ipad.as_ptr(), bs as u32, out, crypto_shash_digestsize(tfm), false);
    memzero_explicit(ipad.as_mut_ptr(), ipad.len());
    err
}

unsafe fn s390_hmac_export(desc: *mut shash_desc, out: *mut u8) -> i32 {
    let ctx = shash_desc_ctx(desc) as *mut S390KmacSha2Ctx;
    let bs = crypto_shash_blocksize((*desc).tfm) as usize;
    let ds = bs / 2;
    let mut lo = (*ctx).buflen[0];
    let err = if (*ctx).gr0.bits.ikp == 0 { s390_hmac_export_zero(desc, out) } else { core::ptr::copy_nonoverlapping((*ctx).param.as_ptr(), out, ds); 0 };
    let p = out.add(ds);
    lo = lo.wrapping_add(bs as u64);
    (p as *mut u64).write_unaligned(lo);
    if ds == SHA512_DIGEST_SIZE { p.add(8).cast::<u64>().write_unaligned((*ctx).buflen[1].wrapping_add((lo < bs as u64) as u64)); }
    err
}

unsafe fn s390_hmac_import(desc: *mut shash_desc, input: *const u8) -> i32 {
    let ctx = shash_desc_ctx(desc) as *mut S390KmacSha2Ctx;
    let bs = crypto_shash_blocksize((*desc).tfm) as usize;
    let ds = bs / 2;
    let err = s390_hmac_sha2_init(desc);
    core::ptr::copy_nonoverlapping(input, (*ctx).param.as_mut_ptr(), ds);
    let p = input.add(ds);
    let lo = (p as *const u64).read_unaligned();
    (*ctx).buflen[0] = lo.wrapping_sub(bs as u64);
    if ds == SHA512_DIGEST_SIZE { (*ctx).buflen[1] = p.add(8).cast::<u64>().read_unaligned().wrapping_sub((lo < bs as u64) as u64); }
    if (*ctx).buflen[0] | (*ctx).buflen[1] != 0 { (*ctx).gr0.bits.ikp = 1; }
    err
}

// The algorithm descriptor table and module registration are represented by
// the surrounding kernel Rust bindings; the C macro expands to four shash
// registrations for SHA-224, SHA-256, SHA-384, and SHA-512.

extern "C" {
    fn cpacf_klmd(func: u32, param: *mut u8, input: *const u8, len: u32);
    fn _cpacf_kmac(gr0: *mut libc::c_ulong, param: *mut u8, input: *const u8, len: u32);
    fn crypto_shash_ctx(tfm: *mut crypto_shash) -> *mut core::ffi::c_void;
    fn crypto_shash_digestsize(tfm: *mut crypto_shash) -> u32;
    fn crypto_shash_blocksize(tfm: *mut crypto_shash) -> u32;
    fn shash_desc_ctx(desc: *mut shash_desc) -> *mut core::ffi::c_void;
    fn memzero_explicit(ptr: *mut u8, len: usize);
}

// External kernel types/constants are intentionally unresolved dependencies.
#[repr(C)] pub struct crypto_shash { _private: [u8; 0] }
#[repr(C)] pub struct shash_desc { pub tfm: *mut crypto_shash }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
