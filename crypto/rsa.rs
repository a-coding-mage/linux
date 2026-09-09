// SPDX-License-Identifier: GPL-2.0-or-later
/* RSA asymmetric public-key algorithm [RFC3447]
 *
 * Copyright (c) 2015, Intel Corporation
 * Authors: Tadeusz Struk <tadeusz.struk@intel.com>
 */

// Kernel and crypto declarations are supplied by the surrounding build.

use core::ffi::c_void;

pub type MPI = *mut c_void;

#[repr(C)]
pub struct rsa_mpi_key {
    pub n: MPI,
    pub e: MPI,
    pub d: MPI,
    pub p: MPI,
    pub q: MPI,
    pub dp: MPI,
    pub dq: MPI,
    pub qinv: MPI,
}

extern "C" {
    fn mpi_cmp_ui(a: MPI, b: u64) -> i32;
    fn mpi_alloc(n: u32) -> MPI;
    fn mpi_sub_ui(dst: MPI, src: MPI, value: u64) -> i32;
    fn mpi_cmp(a: MPI, b: MPI) -> i32;
    fn mpi_free(a: MPI);
    fn mpi_powm(dst: MPI, base: MPI, exp: MPI, modulus: MPI) -> i32;
    fn mpi_sub(dst: MPI, a: MPI, b: MPI) -> i32;
    fn mpi_mulm(dst: MPI, a: MPI, b: MPI, modulus: MPI) -> i32;
    fn mpi_mul(dst: MPI, a: MPI, b: MPI) -> i32;
    fn mpi_addm(dst: MPI, a: MPI, b: MPI, modulus: MPI) -> i32;
    fn mpi_read_raw_from_sgl(src: *mut c_void, len: usize) -> MPI;
    fn mpi_write_to_sgl(src: MPI, dst: *mut c_void, len: usize, sign: *mut i32) -> i32;
    fn akcipher_tfm_ctx(tfm: *mut crypto_akcipher) -> *mut rsa_mpi_key;
    fn crypto_akcipher_reqtfm(req: *mut akcipher_request) -> *mut crypto_akcipher;
    fn mpi_test_bit(a: MPI, bit: u32) -> i32;
    fn mpi_set_bit(a: MPI, bit: u32) -> i32;
    fn mpi_read_raw_data(data: *const u8, len: usize) -> MPI;
    fn mpi_get_size(a: MPI) -> u32;
    fn rsa_parse_pub_key(raw: *mut rsa_key, key: *const c_void, len: u32) -> i32;
    fn rsa_parse_priv_key(raw: *mut rsa_key, key: *const c_void, len: u32) -> i32;
    fn crypto_register_akcipher(alg: *mut akcipher_alg) -> i32;
    fn crypto_unregister_akcipher(alg: *mut akcipher_alg);
    fn crypto_register_template(tmpl: *mut c_void) -> i32;
    fn crypto_unregister_template(tmpl: *mut c_void);
    static mut fips_enabled: bool;
    static mut rsa_pkcs1pad_tmpl: c_void;
    static mut rsassa_pkcs1_tmpl: c_void;
}

#[repr(C)] pub struct crypto_akcipher { _private: [u8; 0] }
#[repr(C)] pub struct akcipher_request { pub src: *mut c_void, pub src_len: usize, pub dst: *mut c_void, pub dst_len: usize }
#[repr(C)] pub struct rsa_key {
    pub n: *const u8, pub n_sz: usize, pub e: *const u8, pub e_sz: usize,
    pub d: *const u8, pub d_sz: usize, pub p: *const u8, pub p_sz: usize,
    pub q: *const u8, pub q_sz: usize, pub dp: *const u8, pub dp_sz: usize,
    pub dq: *const u8, pub dq_sz: usize, pub qinv: *const u8, pub qinv_sz: usize,
}
#[repr(C)] pub struct akcipher_alg_base { pub cra_name: *const u8, pub cra_driver_name: *const u8, pub cra_priority: i32, pub cra_module: *mut c_void, pub cra_ctxsize: usize }
#[repr(C)] pub struct akcipher_alg {
    pub encrypt: Option<unsafe extern "C" fn(*mut akcipher_request) -> i32>,
    pub decrypt: Option<unsafe extern "C" fn(*mut akcipher_request) -> i32>,
    pub set_priv_key: Option<unsafe extern "C" fn(*mut crypto_akcipher, *const c_void, u32) -> i32>,
    pub set_pub_key: Option<unsafe extern "C" fn(*mut crypto_akcipher, *const c_void, u32) -> i32>,
    pub max_size: Option<unsafe extern "C" fn(*mut crypto_akcipher) -> u32>,
    pub exit: Option<unsafe extern "C" fn(*mut crypto_akcipher)>,
    pub base: akcipher_alg_base,
}

unsafe fn rsa_check_payload(x: MPI, n: MPI) -> i32 {
    if mpi_cmp_ui(x, 1) <= 0 { return -22; }
    let n1 = mpi_alloc(0);
    if n1.is_null() { return -12; }
    if mpi_sub_ui(n1, n, 1) != 0 || mpi_cmp(x, n1) >= 0 { mpi_free(n1); return -22; }
    mpi_free(n1); 0
}

unsafe fn _rsa_enc(key: *const rsa_mpi_key, c: MPI, m: MPI) -> i32 {
    if rsa_check_payload(m, (*key).n) != 0 { return -22; }
    mpi_powm(c, m, (*key).e, (*key).n)
}

unsafe fn _rsa_dec_crt(key: *const rsa_mpi_key, m: MPI, c: MPI) -> i32 {
    if rsa_check_payload(c, (*key).n) != 0 { return -22; }
    let m2 = mpi_alloc(0); let qh = mpi_alloc(0); let mut ret = -12;
    if m2.is_null() || qh.is_null() { mpi_free(qh); mpi_free(m2); return ret; }
    ret = mpi_powm(m, c, (*key).dp, (*key).p);
    if ret == 0 { ret = mpi_powm(m2, c, (*key).dq, (*key).q); }
    if ret == 0 { ret = mpi_sub(qh, m, m2); }
    if ret == 0 { ret = mpi_mulm(m, qh, (*key).qinv, (*key).p); }
    if ret == 0 { ret = mpi_mul(qh, (*key).q, m); }
    if ret == 0 { ret = mpi_addm(m, m2, qh, (*key).n); }
    mpi_free(qh); mpi_free(m2); ret
}

unsafe fn rsa_get_key(tfm: *mut crypto_akcipher) -> *mut rsa_mpi_key { akcipher_tfm_ctx(tfm) }

unsafe extern "C" fn rsa_enc(req: *mut akcipher_request) -> i32 {
    let tfm = crypto_akcipher_reqtfm(req); let key = rsa_get_key(tfm); let c = mpi_alloc(0); let mut sign = 0;
    if c.is_null() { return -12; } if (*key).n.is_null() || (*key).e.is_null() { mpi_free(c); return -22; }
    let m = mpi_read_raw_from_sgl((*req).src, (*req).src_len); if m.is_null() { mpi_free(c); return -12; }
    let mut ret = _rsa_enc(key, c, m); if ret == 0 { ret = mpi_write_to_sgl(c, (*req).dst, (*req).dst_len, &mut sign); } if ret == 0 && sign < 0 { ret = -74; }
    mpi_free(m); mpi_free(c); ret
}

unsafe extern "C" fn rsa_dec(req: *mut akcipher_request) -> i32 {
    let tfm = crypto_akcipher_reqtfm(req); let key = rsa_get_key(tfm); let m = mpi_alloc(0); let mut sign = 0;
    if m.is_null() { return -12; } if (*key).n.is_null() || (*key).d.is_null() { mpi_free(m); return -22; }
    let c = mpi_read_raw_from_sgl((*req).src, (*req).src_len); if c.is_null() { mpi_free(m); return -12; }
    let mut ret = _rsa_dec_crt(key, m, c); if ret == 0 { ret = mpi_write_to_sgl(m, (*req).dst, (*req).dst_len, &mut sign); } if ret == 0 && sign < 0 { ret = -74; }
    mpi_free(c); mpi_free(m); ret
}

unsafe fn rsa_free_mpi_key(key: *mut rsa_mpi_key) { mpi_free((*key).d); mpi_free((*key).e); mpi_free((*key).n); mpi_free((*key).p); mpi_free((*key).q); mpi_free((*key).dp); mpi_free((*key).dq); mpi_free((*key).qinv); *key = core::mem::zeroed(); }

unsafe fn rsa_check_key_length(len: u32) -> i32 { match len { 512 | 1024 | 1536 if fips_enabled => -22, 512 | 1024 | 1536 | 2048 | 3072 | 4096 => 0, _ => -22 } }

unsafe fn rsa_check_exponent_fips(e: MPI) -> i32 { if mpi_test_bit(e, 0) == 0 || mpi_cmp_ui(e, 65536) <= 0 { return -22; } let x = mpi_alloc(0); if x.is_null() { return -12; } let r = mpi_set_bit(x, 256); if r != 0 { mpi_free(x); return r; } let out = if mpi_cmp(e, x) >= 0 { -22 } else { 0 }; mpi_free(x); out }

unsafe extern "C" fn rsa_max_size(tfm: *mut crypto_akcipher) -> u32 { mpi_get_size((*rsa_get_key(tfm)).n) }
unsafe extern "C" fn rsa_exit_tfm(tfm: *mut crypto_akcipher) { rsa_free_mpi_key(rsa_get_key(tfm)); }

unsafe extern "C" fn rsa_set_pub_key(tfm: *mut crypto_akcipher, key: *const c_void, keylen: u32) -> i32 {
    let dst = rsa_get_key(tfm); rsa_free_mpi_key(dst); let mut raw: rsa_key = core::mem::zeroed();
    let mut ret = rsa_parse_pub_key(&mut raw, key, keylen); if ret != 0 { return ret; }
    (*dst).e = mpi_read_raw_data(raw.e, raw.e_sz); if (*dst).e.is_null() { rsa_free_mpi_key(dst); return -12; }
    (*dst).n = mpi_read_raw_data(raw.n, raw.n_sz); if (*dst).n.is_null() { rsa_free_mpi_key(dst); return -12; }
    if rsa_check_key_length(mpi_get_size((*dst).n) << 3) != 0 || (fips_enabled && rsa_check_exponent_fips((*dst).e) != 0) { rsa_free_mpi_key(dst); ret = -22; }
    ret
}

unsafe extern "C" fn rsa_set_priv_key(tfm: *mut crypto_akcipher, key: *const c_void, keylen: u32) -> i32 {
    let dst = rsa_get_key(tfm); rsa_free_mpi_key(dst); let mut raw: rsa_key = core::mem::zeroed();
    let mut ret = rsa_parse_priv_key(&mut raw, key, keylen); if ret != 0 { return ret; }
    macro_rules! read { ($field:ident, $size:ident) => {{ (*dst).$field = mpi_read_raw_data(raw.$field, raw.$size); if (*dst).$field.is_null() { rsa_free_mpi_key(dst); return -12; } }} }
    read!(d, d_sz); read!(e, e_sz); read!(n, n_sz); read!(p, p_sz); read!(q, q_sz); read!(dp, dp_sz); read!(dq, dq_sz); read!(qinv, qinv_sz);
    if rsa_check_key_length(mpi_get_size((*dst).n) << 3) != 0 || (fips_enabled && rsa_check_exponent_fips((*dst).e) != 0) { rsa_free_mpi_key(dst); ret = -22; }
    ret
}

#[no_mangle]
pub static mut rsa: akcipher_alg = akcipher_alg {
    encrypt: Some(rsa_enc), decrypt: Some(rsa_dec), set_priv_key: Some(rsa_set_priv_key), set_pub_key: Some(rsa_set_pub_key),
    max_size: Some(rsa_max_size), exit: Some(rsa_exit_tfm),
    base: akcipher_alg_base { cra_name: b"rsa\0".as_ptr(), cra_driver_name: b"rsa-generic\0".as_ptr(), cra_priority: 100, cra_module: core::ptr::null_mut(), cra_ctxsize: core::mem::size_of::<rsa_mpi_key>() },
};

unsafe extern "C" fn rsa_init() -> i32 {
    let mut err = crypto_register_akcipher(&mut rsa); if err != 0 { return err; }
    err = crypto_register_template(&mut rsa_pkcs1pad_tmpl); if err != 0 { crypto_unregister_akcipher(&mut rsa); return err; }
    err = crypto_register_template(&mut rsassa_pkcs1_tmpl); if err != 0 { crypto_unregister_template(&mut rsa_pkcs1pad_tmpl); crypto_unregister_akcipher(&mut rsa); }
    err
}

unsafe extern "C" fn rsa_exit() { crypto_unregister_template(&mut rsassa_pkcs1_tmpl); crypto_unregister_template(&mut rsa_pkcs1pad_tmpl); crypto_unregister_akcipher(&mut rsa); }

// module_init(rsa_init); module_exit(rsa_exit);
// MODULE_ALIAS_CRYPTO("rsa"); MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("RSA generic algorithm");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
