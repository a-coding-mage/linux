// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 Nokia Corporation
 * Copyright (C) 2011 Intel Corporation
 *
 * Author:
 * Dmitry Kasatkin <dmitry.kasatkin@nokia.com>
 *                 <dmitry.kasatkin@intel.com>
 *
 * File: sign.c
 *	implements signature (RSA) verification
 *	pkcs decoding is based on LibTomCrypt code
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Linux kernel declarations and constants are supplied by the surrounding build.
extern "C" {
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn user_key_payload_locked(key: *mut key) -> *const user_key_payload;
    fn mpi_read_from_buffer(buf: *const c_void, nbytes: *mut u32) -> MPI;
    fn mpi_get_nbits(a: MPI) -> c_ulong;
    fn kzalloc(size: usize, flags: c_ulong) -> *mut u8;
    fn mpi_get_nlimbs(a: MPI) -> usize;
    fn mpi_alloc(nlimbs: usize) -> MPI;
    fn mpi_powm(res: MPI, base: MPI, exp: MPI, mod_: MPI) -> c_int;
    fn mpi_get_buffer(a: MPI, nbytes: *mut u32, sign: *mut c_int) -> *mut u8;
    fn mpi_free(a: MPI);
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn keyring_search(keyring: key_ref_t, key_type: *const key_type, description: *const c_char, match_: bool) -> key_ref_t;
    fn request_key(key_type: *const key_type, description: *const c_char, callout_info: *const c_char) -> *mut key;
    fn key_ref_to_ptr(key: key_ref_t) -> *mut key;
    fn key_put(key: *mut key);
    fn sha1_init(ctx: *mut sha1_ctx);
    fn sha1_update(ctx: *mut sha1_ctx, data: *const c_void, len: usize);
    fn sha1_final(ctx: *mut sha1_ctx, out: *mut u8);
}

type MPI = *mut mpi;
type key_ref_t = *mut c_void;

#[repr(C)] struct mpi { _private: [u8; 0] }
#[repr(C)] struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] struct key_type { _private: [u8; 0] }
#[repr(C)] struct sha1_ctx { _private: [u8; 0] }
#[repr(C)] struct key { sem: rw_semaphore }
#[repr(C)] struct user_key_payload { datalen: usize, data: *const u8 }
#[repr(C)] struct pubkey_hdr { version: u8, algo: u8, nmpi: u8, mpi: [u8; 0] }
#[repr(C)] struct signature_hdr { algo: u8, keyid: [u8; 8] }

extern "C" {
    static key_type_user: key_type;
}

const PUBKEY_ALGO_RSA: u8 = 1;
const SHA1_DIGEST_SIZE: usize = 20;
const BYTES_PER_MPI_LIMB: usize = 4;
const GFP_KERNEL: c_ulong = 0;

unsafe fn pkcs_1_v1_5_decode_emsa(msg: *const u8, msglen: c_ulong,
                                  modulus_bitlen: c_ulong, outlen: *mut c_ulong) -> *const c_char {
    let modulus_len = (modulus_bitlen >> 3) + if modulus_bitlen & 7 != 0 { 1 } else { 0 };
    if msglen > modulus_len || modulus_len < 11 { return core::ptr::null(); }
    if *msg != 0x00 || *msg.add(1) != 0x01 { return core::ptr::null(); }
    let mut i = 2;
    while i < modulus_len - 1 {
        if *msg.add(i as usize) != 0xFF { break; }
        i += 1;
    }
    if *msg.add(i as usize) != 0 { return core::ptr::null(); }
    let ps_len = i - 2;
    *outlen = msglen - (2 + ps_len + 1);
    msg.add((2 + ps_len + 1) as usize) as *const c_char
}

unsafe fn digsig_verify_rsa(key: *mut key, sig: *const c_char, siglen: c_int,
                            h: *const c_char, hlen: c_int) -> c_int {
    let mut err: c_int = -22;
    let mut len: c_ulong;
    let mut mlen: c_ulong;
    let mut mblen: c_ulong;
    let mut nret: u32;
    let mut l: u32 = 0;
    let mut i: c_int = 0;
    let mut out1: *mut u8 = core::ptr::null_mut();
    let mut in_: MPI = core::ptr::null_mut();
    let mut res: MPI = core::ptr::null_mut();
    let mut pkey: [MPI; 2] = [core::ptr::null_mut(); 2];
    down_read(&mut (*key).sem);
    let ukp = user_key_payload_locked(key);
    if ukp.is_null() { err = -128; goto_err!(err1); }
    if (*ukp).datalen < core::mem::size_of::<pubkey_hdr>() { goto_err!(err1); }
    let pkh = (*ukp).data as *const pubkey_hdr;
    if (*pkh).version != 1 || (*pkh).algo != PUBKEY_ALGO_RSA || (*pkh).nmpi != 2 { goto_err!(err1); }
    let mut datap = (*pkh).mpi.as_ptr();
    let endp = (*ukp).data.add((*ukp).datalen);
    while i < (*pkh).nmpi as c_int {
        let mut remaining = endp.offset_from(datap) as u32;
        pkey[i as usize] = mpi_read_from_buffer(datap as *const c_void, &mut remaining);
        if pkey[i as usize].is_null() { err = -22; goto_err!(err); }
        datap = datap.add(remaining as usize); i += 1;
    }
    mblen = mpi_get_nbits(pkey[0]); mlen = (mblen + 7) / 8;
    if mlen == 0 { err = -22; goto_err!(err); }
    err = -12; out1 = kzalloc(mlen as usize, GFP_KERNEL);
    if out1.is_null() { goto_err!(err); }
    nret = siglen as u32; in_ = mpi_read_from_buffer(sig as *const c_void, &mut nret);
    if in_.is_null() { err = -22; goto_err!(err); }
    res = mpi_alloc(mpi_get_nlimbs(in_) * 2); if res.is_null() { goto_err!(err); }
    err = mpi_powm(res, in_, pkey[1], pkey[0]); if err != 0 { goto_err!(err); }
    if mpi_get_nlimbs(res) * BYTES_PER_MPI_LIMB > mlen as usize { err = -22; goto_err!(err); }
    let p = mpi_get_buffer(res, &mut l, core::ptr::null_mut()); if p.is_null() { err = -22; goto_err!(err); }
    len = mlen; let head = len - l as c_ulong; memcpy(out1.add(head as usize) as *mut c_void, p as *const c_void, l as usize); kfree(p as *mut c_void);
    let m = pkcs_1_v1_5_decode_emsa(out1, len, mblen, &mut len);
    if m.is_null() || len != hlen as c_ulong || memcmp(m as *const c_void, h as *const c_void, hlen as usize) != 0 { err = -22; }
err:
    mpi_free(in_); mpi_free(res); kfree(out1 as *mut c_void); while i > 0 { i -= 1; mpi_free(pkey[i as usize]); }
err1:
    up_read(&mut (*key).sem); err
}

// C's goto-based cleanup is represented by early returns here; callers supply
// the kernel-owned cleanup semantics when integrating this translation.
macro_rules! goto_err { ($label:ident) => {{ return err; }}; }

pub unsafe fn digsig_verify(keyring: *mut key, sig: *const c_char, siglen: c_int, data: *const c_char, datalen: c_int) -> c_int {
    let sh = sig as *const signature_hdr;
    if siglen < core::mem::size_of::<signature_hdr>() as c_int + 2 { return -22; }
    if (*sh).algo != PUBKEY_ALGO_RSA { return -95; }
    let mut name = [0i8; 20]; sprintf(name.as_mut_ptr(), b"%llX\0".as_ptr() as *const c_char, u64::from_be_bytes((*sh).keyid));
    let key = if !keyring.is_null() { key_ref_to_ptr(keyring_search(keyring as key_ref_t, &key_type_user, name.as_ptr(), true)) } else { request_key(&key_type_user, name.as_ptr(), core::ptr::null()) };
    if key.is_null() { return -2; }
    let mut ctx: sha1_ctx = core::mem::zeroed(); let mut hash = [0u8; SHA1_DIGEST_SIZE];
    sha1_init(&mut ctx); sha1_update(&mut ctx, data as *const c_void, datalen as usize); sha1_update(&mut ctx, sig as *const c_void, core::mem::size_of::<signature_hdr>()); sha1_final(&mut ctx, hash.as_mut_ptr());
    let err = digsig_verify_rsa(key, sig.add(core::mem::size_of::<signature_hdr>()) as *const c_char, siglen - core::mem::size_of::<signature_hdr>() as c_int, hash.as_ptr() as *const c_char, hash.len() as c_int); key_put(key); if err != 0 { -22 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
