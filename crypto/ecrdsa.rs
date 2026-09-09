// SPDX-License-Identifier: GPL-2.0+
/*
 * Elliptic Curve (Russian) Digital Signature Algorithm for Cryptographic API
 *
 * Copyright (c) 2019 Vitaly Chikunov <vt@altlinux.org>
 *
 * References:
 * GOST 34.10-2018, GOST R 34.10-2012, RFC 7091, ISO/IEC 14888-3:2018.
 *
 * Historical references:
 * GOST R 34.10-2001, RFC 4357, ISO/IEC 14888-3:2006/Amd 1:2010.
 */

// C dependencies supplied by the surrounding kernel/crypto implementation.

const ECRDSA_MAX_SIG_SIZE: usize = 2 * 512 / 8;
const ECRDSA_MAX_DIGITS: usize = 512 / 64;

#[repr(C)]
pub struct EcrdsaCtx {
    algo_oid: Oid,
    curve_oid: Oid,
    digest_oid: Oid,
    curve: *const EccCurve,
    digest_len: u32,
    digest: *const i8,
    key_len: u32,
    key: *const i8,
    pub_key: EccPoint,
    _pubp: [[u64; ECRDSA_MAX_DIGITS]; 2],
}

extern "C" {
    fn crypto_sig_ctx(tfm: *mut CryptoSig) -> *mut EcrdsaCtx;
    fn vli_from_be64(dst: *mut u64, src: *const core::ffi::c_void, ndigits: usize);
    fn vli_from_le64(dst: *mut u64, src: *const core::ffi::c_void, ndigits: usize);
    fn vli_is_zero(vli: *const u64, ndigits: usize) -> bool;
    fn vli_cmp(left: *const u64, right: *const u64, ndigits: usize) -> i32;
    fn vli_sub(result: *mut u64, left: *const u64, right: *const u64, ndigits: usize);
    fn vli_mod_inv(result: *mut u64, input: *const u64, mod_: *const u64, ndigits: usize);
    fn vli_mod_mult_slow(result: *mut u64, left: *const u64, right: *const u64,
                         mod_: *const u64, ndigits: usize);
    fn ecc_point_mult_shamir(result: *mut EccPoint, z1: *const u64, p: *const EccPoint,
                             z2: *const u64, q: *const EccPoint, curve: *const EccCurve);
    fn ecc_is_pubkey_valid_partial(curve: *const EccCurve, point: *const EccPoint) -> i32;
    fn look_up_OID(value: *const core::ffi::c_void, vlen: usize) -> Oid;
    fn asn1_ber_decoder(decoder: *const core::ffi::c_void, context: *mut EcrdsaCtx,
                        data: *const core::ffi::c_void, len: u32) -> i32;
}

#[repr(C)] pub struct CryptoSig { _private: [u8; 0] }
#[repr(C)] pub struct EccPoint { x: *mut u64, y: *mut u64, ndigits: usize }
#[repr(C)] pub struct EccCurve { g: EccPoint, n: *const u64 }
#[repr(C)] pub struct SigAlg { _private: [u8; 0] }
#[repr(C)] pub struct Oid(u32);

const OID_NR: Oid = Oid(0);
const STREEBOG512_DIGEST_SIZE: u32 = 64;
const EINVAL: i32 = 22;
const EBADMSG: i32 = 74;
const EKEYREJECTED: i32 = 129;
const ENOPKG: i32 = 65;

extern "C" {
    static gost_cp256a: EccCurve;
    static gost_cp256b: EccCurve;
    static gost_cp256c: EccCurve;
    static gost_tc512a: EccCurve;
    static gost_tc512b: EccCurve;
    static ecrdsa_pub_key_decoder: core::ffi::c_void;
    static ecrdsa_params_decoder: core::ffi::c_void;
}

unsafe fn get_curve_by_oid(oid: Oid) -> *const EccCurve {
    match oid.0 {
        1 | 2 => &gost_cp256a,
        3 | 4 => &gost_cp256b,
        5 | 6 => &gost_cp256c,
        7 => &gost_tc512a,
        8 => &gost_tc512b,
        _ => core::ptr::null(),
    }
}

pub unsafe fn ecrdsa_verify(tfm: *mut CryptoSig, src: *const u8, slen: u32,
                            digest: *const u8, dlen: u32) -> i32 {
    let ctx = &mut *crypto_sig_ctx(tfm);
    let ndigits = (dlen as usize) / core::mem::size_of::<u64>();
    let mut r = [0u64; ECRDSA_MAX_DIGITS];
    let mut neg_r = [0u64; ECRDSA_MAX_DIGITS];
    let mut s = [0u64; ECRDSA_MAX_DIGITS];
    let mut e = [0u64; ECRDSA_MAX_DIGITS];
    let mut z1 = [0u64; ECRDSA_MAX_DIGITS];
    let curve = ctx.curve;
    if curve.is_null() || ctx.digest.is_null() || src.is_null() || digest.is_null() ||
       ctx.pub_key.x.is_null() || dlen != ctx.digest_len ||
       dlen as usize != (*curve).g.ndigits * 8 || ctx.pub_key.ndigits != (*curve).g.ndigits ||
       dlen * 2 != slen || slen as usize > ECRDSA_MAX_SIG_SIZE ||
       dlen > STREEBOG512_DIGEST_SIZE { return -EBADMSG; }
    vli_from_be64(s.as_mut_ptr(), src as *const _, ndigits);
    vli_from_be64(r.as_mut_ptr(), src.add(ndigits * 8) as *const _, ndigits);
    if vli_is_zero(r.as_ptr(), ndigits) || vli_cmp(r.as_ptr(), (*curve).n, ndigits) >= 0 ||
       vli_is_zero(s.as_ptr(), ndigits) || vli_cmp(s.as_ptr(), (*curve).n, ndigits) >= 0 { return -EKEYREJECTED; }
    vli_from_le64(e.as_mut_ptr(), digest as *const _, ndigits);
    if vli_cmp(e.as_ptr(), (*curve).n, ndigits) >= 0 { vli_sub(e.as_mut_ptr(), e.as_ptr(), (*curve).n, ndigits); }
    if vli_is_zero(e.as_ptr(), ndigits) { e[0] = 1; }
    vli_mod_inv(e.as_mut_ptr(), e.as_ptr(), (*curve).n, ndigits);
    vli_mod_mult_slow(z1.as_mut_ptr(), s.as_ptr(), e.as_ptr(), (*curve).n, ndigits);
    vli_sub(neg_r.as_mut_ptr(), (*curve).n, r.as_ptr(), ndigits);
    vli_mod_mult_slow(neg_r.as_mut_ptr(), neg_r.as_ptr(), e.as_ptr(), (*curve).n, ndigits);
    let mut cc = EccPoint { x: s.as_mut_ptr(), y: e.as_mut_ptr(), ndigits };
    ecc_point_mult_shamir(&mut cc, z1.as_ptr(), &(*curve).g, neg_r.as_ptr(), &ctx.pub_key, curve);
    if vli_cmp(cc.x, (*curve).n, ndigits) >= 0 { vli_sub(cc.x, cc.x, (*curve).n, ndigits); }
    if vli_cmp(cc.x, r.as_ptr(), ndigits) == 0 { 0 } else { -EKEYREJECTED }
}

pub unsafe fn ecrdsa_param_curve(context: *mut EcrdsaCtx, _hdrlen: usize, _tag: u8,
                                 value: *const core::ffi::c_void, vlen: usize) -> i32 {
    (*context).curve_oid = look_up_OID(value, vlen);
    if (*context).curve_oid == OID_NR { return -EINVAL; }
    (*context).curve = get_curve_by_oid((*context).curve_oid); 0
}

pub unsafe fn ecrdsa_param_digest(context: *mut EcrdsaCtx, _hdrlen: usize, _tag: u8,
                                  value: *const core::ffi::c_void, vlen: usize) -> i32 {
    if look_up_OID(value, vlen) != (*context).digest_oid { -EINVAL } else { 0 }
}

pub unsafe fn ecrdsa_parse_pub_key(context: *mut EcrdsaCtx, _hdrlen: usize, _tag: u8,
                                   value: *const core::ffi::c_void, vlen: usize) -> i32 {
    (*context).key = value as *const i8; (*context).key_len = vlen as u32; 0
}

unsafe fn ecrdsa_unpack_u32(dst: *mut u32, src: *mut u8) -> *mut u8 {
    core::ptr::copy_nonoverlapping(src, dst as *mut u8, 4); src.add(4)
}

pub unsafe fn ecrdsa_set_pub_key(tfm: *mut CryptoSig, key: *const u8, keylen: u32) -> i32 {
    let ctx = &mut *crypto_sig_ctx(tfm);
    let mut algo = 0u32;
    let mut paramlen = 0u32;
    let params = ecrdsa_unpack_u32(&mut paramlen, ecrdsa_unpack_u32(&mut algo, key.add(keylen as usize) as *mut u8));
    let err = asn1_ber_decoder(&ecrdsa_pub_key_decoder, ctx, key as *const _, keylen);
    if err < 0 { return err; }
    if algo == 1 { ctx.digest = b"streebog256\0".as_ptr() as *const i8; ctx.digest_oid = Oid(10); ctx.digest_len = 32; }
    else if algo == 2 { ctx.digest = b"streebog512\0".as_ptr() as *const i8; ctx.digest_oid = Oid(11); ctx.digest_len = 64; }
    else { return -ENOPKG; }
    ctx.algo_oid = Oid(algo);
    let err = asn1_ber_decoder(&ecrdsa_params_decoder, ctx, params as *const _, paramlen);
    if err < 0 { return err; }
    if ctx.curve.is_null() || (*ctx.curve).g.ndigits * 8 != ctx.digest_len as usize { return -ENOPKG; }
    if (ctx.key_len != 64 && ctx.key_len != 128) || ctx.key_len != ((*ctx.curve).g.ndigits * 8 * 2) as u32 { return -ENOPKG; }
    let ndigits = ctx.key_len as usize / 8 / 2;
    ctx.pub_key = EccPoint { x: ctx._pubp[0].as_mut_ptr(), y: ctx._pubp[1].as_mut_ptr(), ndigits };
    vli_from_le64(ctx.pub_key.x, ctx.key as *const _, ndigits);
    vli_from_le64(ctx.pub_key.y, ctx.key.add(ndigits * 8) as *const _, ndigits);
    if ecc_is_pubkey_valid_partial(ctx.curve, &ctx.pub_key) != 0 { return -EKEYREJECTED; }
    0
}

pub unsafe fn ecrdsa_key_size(tfm: *mut CryptoSig) -> u32 {
    (*crypto_sig_ctx(tfm)).pub_key.ndigits as u32 * 8 * 8
}

pub unsafe fn ecrdsa_max_size(tfm: *mut CryptoSig) -> u32 {
    (*crypto_sig_ctx(tfm)).pub_key.ndigits as u32 * 8 * 2
}

extern "C" {
    fn crypto_register_sig(alg: *mut SigAlg) -> i32;
    fn crypto_unregister_sig(alg: *mut SigAlg);
}

static mut ECRDSA_ALG: *mut SigAlg = core::ptr::null_mut();

pub unsafe fn ecrdsa_mod_init() -> i32 { crypto_register_sig(ECRDSA_ALG) }
pub unsafe fn ecrdsa_mod_fini() { crypto_unregister_sig(ECRDSA_ALG); }

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Vitaly Chikunov <vt@altlinux.org>");
// MODULE_DESCRIPTION("EC-RDSA generic algorithm");
// MODULE_ALIAS_CRYPTO("ecrdsa");
// MODULE_ALIAS_CRYPTO("ecrdsa-generic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
