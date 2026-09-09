// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2021 IBM Corporation
 */

// External Linux kernel, ECC, signature, ECDH, SHA-2, and crypto definitions
// supplied by the surrounding kernel build are intentionally not redefined here.

#[repr(C)]
struct EccCtx {
    curve_id: c_uint,
    curve: *const EccCurve,
    pub_key_set: bool,
    x: [u64; ECC_MAX_DIGITS],
    y: [u64; ECC_MAX_DIGITS],
    pub_key: EccPoint,
}

unsafe fn _ecdsa_verify(
    ctx: *mut EccCtx,
    hash: *const u64,
    r: *const u64,
    s: *const u64,
) -> c_int {
    let curve = (*ctx).curve;
    let ndigits = (*(*curve).g).ndigits;
    let mut s1 = [0u64; ECC_MAX_DIGITS];
    let mut u1 = [0u64; ECC_MAX_DIGITS];
    let mut u2 = [0u64; ECC_MAX_DIGITS];
    let mut x1 = [0u64; ECC_MAX_DIGITS];
    let mut y1 = [0u64; ECC_MAX_DIGITS];
    let mut res = ecc_point_init(x1.as_mut_ptr(), y1.as_mut_ptr(), ndigits);

    // 0 < r < n and 0 < s < n
    if vli_is_zero(r, ndigits) || vli_cmp(r, (*curve).n, ndigits) >= 0
        || vli_is_zero(s, ndigits) || vli_cmp(s, (*curve).n, ndigits) >= 0
    {
        return -EBADMSG;
    }

    // hash is given
    pr_devel!("hash : {:016x} {:016x} ... {:016x}\n", *hash.add(ndigits - 1), *hash.add(ndigits - 2), *hash);

    // s1 = (s^-1) mod n
    vli_mod_inv(s1.as_mut_ptr(), s, (*curve).n, ndigits);
    // u1 = (hash * s1) mod n
    vli_mod_mult_slow(u1.as_mut_ptr(), hash, s1.as_ptr(), (*curve).n, ndigits);
    // u2 = (r * s1) mod n
    vli_mod_mult_slow(u2.as_mut_ptr(), r, s1.as_ptr(), (*curve).n, ndigits);
    // res = u1*G + u2 * pub_key
    ecc_point_mult_shamir(&mut res, u1.as_ptr(), &(*curve).g, u2.as_ptr(), &(*ctx).pub_key, curve);

    // res.x = res.x mod n (if res.x > order)
    if vli_cmp(res.x, (*curve).n, ndigits) == 1 {
        // faster alternative for NIST p521, p384, p256 & p192
        vli_sub(res.x, res.x, (*curve).n, ndigits);
    }

    if vli_cmp(res.x, r, ndigits) == 0 { 0 } else { -EKEYREJECTED }
}

/*
 * Verify an ECDSA signature.
 */
unsafe fn ecdsa_verify(
    tfm: *mut CryptoSig,
    src: *const c_void,
    slen: c_uint,
    digest: *const c_void,
    dlen: c_uint,
) -> c_int {
    let ctx = crypto_sig_ctx(tfm);
    let mut bufsize = (*(*ctx).curve).g.ndigits * core::mem::size_of::<u64>();
    let sig = src as *const EcdsaRawSig;
    let mut hash = [0u64; ECC_MAX_DIGITS];

    if !(*ctx).pub_key_set || slen as usize != core::mem::size_of::<EcdsaRawSig>() {
        return -EINVAL;
    }
    if bufsize > dlen as usize { bufsize = dlen as usize; }
    ecc_digits_from_bytes(digest, bufsize, hash.as_mut_ptr(), (*(*ctx).curve).g.ndigits);
    _ecdsa_verify(ctx, hash.as_ptr(), (*sig).r.as_ptr(), (*sig).s.as_ptr())
}

unsafe fn ecdsa_ecc_ctx_init(ctx: *mut EccCtx, curve_id: c_uint) -> c_int {
    (*ctx).curve_id = curve_id;
    (*ctx).curve = ecc_get_curve(curve_id);
    if (*ctx).curve.is_null() { -EINVAL } else { 0 }
}

unsafe fn ecdsa_ecc_ctx_deinit(ctx: *mut EccCtx) { (*ctx).pub_key_set = false; }

unsafe fn ecdsa_ecc_ctx_reset(ctx: *mut EccCtx) -> c_int {
    let curve_id = (*ctx).curve_id;
    ecdsa_ecc_ctx_deinit(ctx);
    let ret = ecdsa_ecc_ctx_init(ctx, curve_id);
    if ret == 0 { (*ctx).pub_key = ecc_point_init((*ctx).x.as_mut_ptr(), (*ctx).y.as_mut_ptr(), (*(*ctx).curve).g.ndigits); }
    ret
}

/* Set the public ECC key as defined by RFC5480 section 2.2. Only uncompressed format is supported. */
unsafe fn ecdsa_set_pub_key(tfm: *mut CryptoSig, key: *const c_void, keylen: c_uint) -> c_int {
    let ctx = crypto_sig_ctx(tfm);
    let mut ret = ecdsa_ecc_ctx_reset(ctx);
    if ret < 0 { return ret; }
    if keylen < 1 || ((keylen - 1) & 1) != 0 || *(key as *const u8) != 4 { return -EINVAL; }
    let keylen = keylen - 1;
    let digitlen = keylen >> 1;
    let ndigits = (digitlen as usize + core::mem::size_of::<u64>() - 1) / core::mem::size_of::<u64>();
    if ndigits != (*(*ctx).curve).g.ndigits { return -EINVAL; }
    let d = (key as *const u8).add(1);
    ecc_digits_from_bytes(d as *const c_void, digitlen as usize, (*ctx).pub_key.x, ndigits);
    ecc_digits_from_bytes(d.add(digitlen as usize) as *const c_void, digitlen as usize, (*ctx).pub_key.y, ndigits);
    ret = ecc_is_pubkey_valid_full((*ctx).curve, &(*ctx).pub_key);
    (*ctx).pub_key_set = ret == 0;
    ret
}

unsafe fn ecdsa_exit_tfm(tfm: *mut CryptoSig) { ecdsa_ecc_ctx_deinit(crypto_sig_ctx(tfm)); }
unsafe fn ecdsa_key_size(tfm: *mut CryptoSig) -> c_uint { (*(*crypto_sig_ctx(tfm)).curve).nbits }
unsafe fn ecdsa_digest_size(_tfm: *mut CryptoSig) -> c_uint { SHA512_DIGEST_SIZE }

unsafe fn ecdsa_nist_p521_init_tfm(tfm: *mut CryptoSig) -> c_int { ecdsa_ecc_ctx_init(crypto_sig_ctx(tfm), ECC_CURVE_NIST_P521) }
unsafe fn ecdsa_nist_p384_init_tfm(tfm: *mut CryptoSig) -> c_int { ecdsa_ecc_ctx_init(crypto_sig_ctx(tfm), ECC_CURVE_NIST_P384) }
unsafe fn ecdsa_nist_p256_init_tfm(tfm: *mut CryptoSig) -> c_int { ecdsa_ecc_ctx_init(crypto_sig_ctx(tfm), ECC_CURVE_NIST_P256) }
unsafe fn ecdsa_nist_p192_init_tfm(tfm: *mut CryptoSig) -> c_int { ecdsa_ecc_ctx_init(crypto_sig_ctx(tfm), ECC_CURVE_NIST_P192) }

// The following algorithm objects and registration routines are external kernel interfaces.
extern "C" {
    static mut ecdsa_nist_p521: SigAlg;
    static mut ecdsa_nist_p384: SigAlg;
    static mut ecdsa_nist_p256: SigAlg;
    static mut ecdsa_nist_p192: SigAlg;
    static mut ecdsa_x962_tmpl: CryptoTemplate;
    static mut ecdsa_p1363_tmpl: CryptoTemplate;
    fn crypto_register_sig(alg: *mut SigAlg) -> c_int;
    fn crypto_unregister_sig(alg: *mut SigAlg);
    fn crypto_register_template(tmpl: *mut CryptoTemplate) -> c_int;
    fn crypto_unregister_template(tmpl: *mut CryptoTemplate);
}

static mut ecdsa_nist_p192_registered: bool = false;

unsafe fn ecdsa_init() -> c_int {
    let mut ret = crypto_register_sig(&mut ecdsa_nist_p192);
    ecdsa_nist_p192_registered = ret == 0;
    ret = crypto_register_sig(&mut ecdsa_nist_p256); if ret != 0 { goto_nist_p256_error(ret); }
    ret = crypto_register_sig(&mut ecdsa_nist_p384); if ret != 0 { goto_nist_p384_error(ret); }
    ret = crypto_register_sig(&mut ecdsa_nist_p521); if ret != 0 { goto_nist_p521_error(ret); }
    ret = crypto_register_template(&mut ecdsa_x962_tmpl); if ret != 0 { goto_x962_tmpl_error(ret); }
    ret = crypto_register_template(&mut ecdsa_p1363_tmpl); if ret != 0 { crypto_unregister_template(&mut ecdsa_x962_tmpl); goto_x962_tmpl_error(ret); }
    0
}

unsafe fn goto_x962_tmpl_error(ret: c_int) -> c_int { crypto_unregister_sig(&mut ecdsa_nist_p521); goto_nist_p521_error(ret) }
unsafe fn goto_nist_p521_error(ret: c_int) -> c_int { crypto_unregister_sig(&mut ecdsa_nist_p384); goto_nist_p384_error(ret) }
unsafe fn goto_nist_p384_error(ret: c_int) -> c_int { crypto_unregister_sig(&mut ecdsa_nist_p256); goto_nist_p256_error(ret) }
unsafe fn goto_nist_p256_error(ret: c_int) -> c_int { if ecdsa_nist_p192_registered { crypto_unregister_sig(&mut ecdsa_nist_p192); } ret }

unsafe fn ecdsa_exit() {
    crypto_unregister_template(&mut ecdsa_x962_tmpl); crypto_unregister_template(&mut ecdsa_p1363_tmpl);
    if ecdsa_nist_p192_registered { crypto_unregister_sig(&mut ecdsa_nist_p192); }
    crypto_unregister_sig(&mut ecdsa_nist_p256); crypto_unregister_sig(&mut ecdsa_nist_p384); crypto_unregister_sig(&mut ecdsa_nist_p521);
}

// module_init!(ecdsa_init); module_exit!(ecdsa_exit);
// MODULE_LICENSE!("GPL"); MODULE_AUTHOR!("Stefan Berger <stefanb@linux.ibm.com>");
// MODULE_DESCRIPTION!("ECDSA generic algorithm");
// MODULE_ALIAS_CRYPTO!("ecdsa-nist-p192"); MODULE_ALIAS_CRYPTO!("ecdsa-nist-p256");
// MODULE_ALIAS_CRYPTO!("ecdsa-nist-p384"); MODULE_ALIAS_CRYPTO!("ecdsa-nist-p521");
// MODULE_ALIAS_CRYPTO!("ecdsa-generic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
