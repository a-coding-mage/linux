// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Synchronous Cryptographic Hash operations.
 *
 * Copyright (c) 2008 Herbert Xu <herbert@gondor.apana.org.au>
 */

// External kernel declarations and constants are supplied by the surrounding
// translation unit and corresponding Rust bindings.

#[inline]
unsafe fn crypto_shash_block_only(tfm: *mut crypto_shash) -> bool {
    ((*crypto_shash_alg(tfm)).base.cra_flags & CRYPTO_AHASH_ALG_BLOCK_ONLY) != 0
}

#[inline]
unsafe fn crypto_shash_final_nonzero(tfm: *mut crypto_shash) -> bool {
    ((*crypto_shash_alg(tfm)).base.cra_flags & CRYPTO_AHASH_ALG_FINAL_NONZERO) != 0
}

#[inline]
unsafe fn crypto_shash_finup_max(tfm: *mut crypto_shash) -> bool {
    ((*crypto_shash_alg(tfm)).base.cra_flags & CRYPTO_AHASH_ALG_FINUP_MAX) != 0
}

pub unsafe extern "C" fn shash_no_setkey(
    _tfm: *mut crypto_shash,
    _key: *const u8,
    _keylen: c_uint,
) -> c_int {
    -ENOSYS
}

unsafe fn shash_set_needkey(tfm: *mut crypto_shash, alg: *mut shash_alg) {
    if crypto_shash_alg_needs_key(alg) {
        crypto_shash_set_flags(tfm, CRYPTO_TFM_NEED_KEY);
    }
}

pub unsafe extern "C" fn crypto_shash_setkey(
    tfm: *mut crypto_shash,
    key: *const u8,
    keylen: c_uint,
) -> c_int {
    let shash = crypto_shash_alg(tfm);
    let err = ((*shash).setkey)(tfm, key, keylen);
    if err != 0 {
        shash_set_needkey(tfm, shash);
        return err;
    }
    crypto_shash_clear_flags(tfm, CRYPTO_TFM_NEED_KEY);
    0
}

unsafe fn __crypto_shash_init(desc: *mut shash_desc) -> c_int {
    let tfm = (*desc).tfm;
    if crypto_shash_block_only(tfm) {
        let mut buf = shash_desc_ctx(desc);
        buf = buf.add(crypto_shash_descsize(tfm) as usize - 1);
        *buf = 0;
    }
    ((*crypto_shash_alg(tfm)).init)(desc)
}

pub unsafe extern "C" fn crypto_shash_init(desc: *mut shash_desc) -> c_int {
    if crypto_shash_get_flags((*desc).tfm) & CRYPTO_TFM_NEED_KEY != 0 {
        return -ENOKEY;
    }
    __crypto_shash_init(desc)
}

unsafe fn shash_default_finup(desc: *mut shash_desc, data: *const u8, len: c_uint, out: *mut u8) -> c_int {
    let shash = crypto_shash_alg((*desc).tfm);
    let err = ((*shash).update)(desc, data, len);
    if err != 0 { err } else { ((*shash).final_)(desc, out) }
}

unsafe fn crypto_shash_op_and_zero(
    op: unsafe extern "C" fn(*mut shash_desc, *const u8, c_uint, *mut u8) -> c_int,
    desc: *mut shash_desc, data: *const u8, len: c_uint, out: *mut u8,
) -> c_int {
    let err = op(desc, data, len, out);
    memset(shash_desc_ctx(desc), 0, crypto_shash_descsize((*desc).tfm) as usize);
    err
}

pub unsafe extern "C" fn crypto_shash_finup(desc: *mut shash_desc, mut data: *const u8, mut len: c_uint, out: *mut u8) -> c_int {
    let tfm = (*desc).tfm;
    let mut blenp = shash_desc_ctx(desc);
    if !crypto_shash_block_only(tfm) {
        if !out.is_null() { return crypto_shash_op_and_zero((*crypto_shash_alg(tfm)).finup, desc, data, len, out); }
        return ((*crypto_shash_alg(tfm)).update)(desc, data, len);
    }
    let finup_max = !out.is_null() && crypto_shash_finup_max(tfm);
    let nonzero = crypto_shash_final_nonzero(tfm) as usize;
    blenp = blenp.add(crypto_shash_descsize(tfm) as usize - 1);
    let bs = crypto_shash_blocksize(tfm) as usize;
    let buf = blenp.sub(bs);
    if *blenp == 0 && finup_max { return crypto_shash_op_and_zero((*crypto_shash_alg(tfm)).finup, desc, data, len, out); }
    while (*blenp as usize + len as usize) >= bs + nonzero {
        let mut nbytes = len as usize - nonzero;
        let mut src = data;
        if *blenp != 0 {
            memcpy(buf.add(*blenp as usize), data, bs - *blenp as usize);
            nbytes = bs;
            src = buf;
        }
        let err = ((*crypto_shash_alg(tfm)).update)(desc, src, nbytes as c_uint);
        if err < 0 { return err; }
        let consumed = nbytes as c_int - err - *blenp as c_int;
        data = data.add(consumed as usize);
        len -= consumed as c_uint;
        *blenp = 0;
    }
    if *blenp != 0 || out.is_null() {
        memcpy(buf.add(*blenp as usize), data, len as usize);
        *blenp = (*blenp).wrapping_add(len as u8);
        if out.is_null() { return 0; }
        data = buf;
        len = *blenp as c_uint;
    }
    crypto_shash_op_and_zero((*crypto_shash_alg(tfm)).finup, desc, data, len, out)
}

unsafe fn shash_default_digest(desc: *mut shash_desc, data: *const u8, len: c_uint, out: *mut u8) -> c_int {
    let err = __crypto_shash_init(desc);
    if err != 0 { err } else { crypto_shash_finup(desc, data, len, out) }
}

pub unsafe extern "C" fn crypto_shash_digest(desc: *mut shash_desc, data: *const u8, len: c_uint, out: *mut u8) -> c_int {
    let tfm = (*desc).tfm;
    if crypto_shash_get_flags(tfm) & CRYPTO_TFM_NEED_KEY != 0 { return -ENOKEY; }
    crypto_shash_op_and_zero((*crypto_shash_alg(tfm)).digest, desc, data, len, out)
}

pub unsafe extern "C" fn crypto_shash_tfm_digest(tfm: *mut crypto_shash, data: *const u8, len: c_uint, out: *mut u8) -> c_int {
    let mut desc = shash_desc_on_stack(tfm);
    (*desc).tfm = tfm;
    crypto_shash_digest(desc, data, len, out)
}

unsafe fn __crypto_shash_export(desc: *mut shash_desc, out: *mut c_void, export: Option<unsafe extern "C" fn(*mut shash_desc, *mut c_void) -> c_int>) -> c_int {
    let tfm = (*desc).tfm;
    let buf = shash_desc_ctx(desc);
    let plen = crypto_shash_blocksize(tfm) + 1;
    let mut ss = crypto_shash_statesize(tfm);
    if crypto_shash_block_only(tfm) { ss -= plen; }
    match export { None => { memcpy(out, buf as *const c_void, ss as usize); 0 }, Some(f) => f(desc, out) }
}

pub unsafe extern "C" fn crypto_shash_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    __crypto_shash_export(desc, out, (*crypto_shash_alg((*desc).tfm)).export_core)
}

pub unsafe extern "C" fn crypto_shash_export(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    let tfm = (*desc).tfm;
    if crypto_shash_block_only(tfm) {
        let plen = crypto_shash_blocksize(tfm) + 1;
        let descsize = crypto_shash_descsize(tfm);
        let ss = crypto_shash_statesize(tfm);
        let buf = shash_desc_ctx(desc);
        memcpy(out.add((ss - plen) as usize), buf.add((descsize - plen) as usize), plen as usize);
    }
    __crypto_shash_export(desc, out, (*crypto_shash_alg(tfm)).export)
}

unsafe fn __crypto_shash_import(desc: *mut shash_desc, input: *const c_void, import: Option<unsafe extern "C" fn(*mut shash_desc, *const c_void) -> c_int>) -> c_int {
    let tfm = (*desc).tfm;
    let buf = shash_desc_ctx(desc);
    if crypto_shash_get_flags(tfm) & CRYPTO_TFM_NEED_KEY != 0 { return -ENOKEY; }
    let mut ss = crypto_shash_statesize(tfm);
    if crypto_shash_block_only(tfm) {
        let plen = crypto_shash_blocksize(tfm) + 1;
        ss -= plen;
        *buf.add(crypto_shash_descsize(tfm) as usize - 1) = 0;
    }
    match import { None => { memcpy(buf as *mut c_void, input, ss as usize); 0 }, Some(f) => f(desc, input) }
}

pub unsafe extern "C" fn crypto_shash_import_core(desc: *mut shash_desc, input: *const c_void) -> c_int {
    __crypto_shash_import(desc, input, (*crypto_shash_alg((*desc).tfm)).import_core)
}

pub unsafe extern "C" fn crypto_shash_import(desc: *mut shash_desc, input: *const c_void) -> c_int {
    let tfm = (*desc).tfm;
    let mut err = __crypto_shash_import(desc, input, (*crypto_shash_alg(tfm)).import);
    if crypto_shash_block_only(tfm) {
        let plen = crypto_shash_blocksize(tfm) + 1;
        let descsize = crypto_shash_descsize(tfm);
        let ss = crypto_shash_statesize(tfm);
        let buf = shash_desc_ctx(desc);
        memcpy(buf.add((descsize - plen) as usize), (input as *const u8).add((ss - plen) as usize) as *const c_void, plen as usize);
        if *buf.add(descsize as usize - 1) >= plen as u8 { err = -EOVERFLOW; }
    }
    err
}

unsafe extern "C" fn crypto_shash_exit_tfm(tfm: *mut crypto_tfm) {
    let hash = __crypto_shash_cast(tfm);
    let alg = crypto_shash_alg(hash);
    ((*alg).exit_tfm)(hash);
}

unsafe extern "C" fn crypto_shash_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    let hash = __crypto_shash_cast(tfm);
    let alg = crypto_shash_alg(hash);
    shash_set_needkey(hash, alg);
    if let Some(_) = (*alg).exit_tfm { (*tfm).exit = Some(crypto_shash_exit_tfm); }
    match (*alg).init_tfm { None => 0, Some(f) => f(hash) }
}

unsafe extern "C" fn crypto_shash_free_instance(inst: *mut crypto_instance) {
    let shash = shash_instance(inst);
    ((*shash).free)(shash);
}

unsafe extern "C" fn crypto_shash_report(skb: *mut sk_buff, alg: *mut crypto_alg) -> c_int {
    let salg = __crypto_shash_alg(alg);
    let rhash = crypto_report_hash { type_: b"shash\0".as_ptr() as *const c_char, blocksize: (*alg).cra_blocksize, digestsize: (*salg).digestsize };
    nla_put(skb, CRYPTOCFGA_REPORT_HASH, size_of::<crypto_report_hash>() as u16, &rhash as *const _ as *const c_void)
}

unsafe extern "C" fn crypto_shash_show(m: *mut seq_file, alg: *mut crypto_alg) {
    let salg = __crypto_shash_alg(alg);
    seq_printf(m, b"type         : shash\n\0".as_ptr() as *const c_char);
    seq_printf(m, b"blocksize    : %u\n\0".as_ptr() as *const c_char, (*alg).cra_blocksize);
    seq_printf(m, b"digestsize   : %u\n\0".as_ptr() as *const c_char, (*salg).digestsize);
}

pub static mut crypto_shash_type: crypto_type = crypto_type {
    extsize: crypto_alg_extsize,
    init_tfm: Some(crypto_shash_init_tfm),
    free: Some(crypto_shash_free_instance),
    show: Some(crypto_shash_show),
    report: Some(crypto_shash_report),
    maskclear: !CRYPTO_ALG_TYPE_MASK,
    maskset: CRYPTO_ALG_TYPE_MASK,
    type_: CRYPTO_ALG_TYPE_SHASH,
    tfmsize: offset_of!(crypto_shash, base),
    algsize: offset_of!(shash_alg, base),
};

pub unsafe extern "C" fn crypto_grab_shash(spawn: *mut crypto_shash_spawn, inst: *mut crypto_instance, name: *const c_char, type_: u32, mask: u32) -> c_int {
    (*spawn).base.frontend = &mut crypto_shash_type;
    crypto_grab_spawn(&mut (*spawn).base, inst, name, type_, mask)
}

pub unsafe extern "C" fn crypto_alloc_shash(alg_name: *const c_char, type_: u32, mask: u32) -> *mut crypto_shash {
    crypto_alloc_tfm(alg_name, &mut crypto_shash_type, type_, mask)
}

pub unsafe extern "C" fn crypto_has_shash(alg_name: *const c_char, type_: u32, mask: u32) -> c_int {
    crypto_type_has_alg(alg_name, &mut crypto_shash_type, type_, mask)
}

pub unsafe extern "C" fn hash_prepare_alg(alg: *mut hash_alg_common) -> c_int {
    let base = &mut (*alg).base;
    if (*alg).digestsize > HASH_MAX_DIGESTSIZE || base.cra_alignmask != 0 { return -EINVAL; }
    base.cra_flags &= !CRYPTO_ALG_TYPE_MASK;
    0
}

unsafe extern "C" fn shash_default_export_core(_desc: *mut shash_desc, _out: *mut c_void) -> c_int { -ENOSYS }
unsafe extern "C" fn shash_default_import_core(_desc: *mut shash_desc, _in: *const c_void) -> c_int { -ENOSYS }

unsafe fn shash_prepare_alg(alg: *mut shash_alg) -> c_int {
    let base = &mut (*alg).halg.base;
    if ((*alg).export.is_some()) != ((*alg).import.is_some()) { return -EINVAL; }
    let err = hash_prepare_alg(&mut (*alg).halg);
    if err != 0 { return err; }
    base.cra_type = &mut crypto_shash_type;
    base.cra_flags |= CRYPTO_ALG_TYPE_SHASH | CRYPTO_ALG_REQ_VIRT;
    if (*alg).finup.is_none() { (*alg).finup = Some(shash_default_finup); }
    if (*alg).digest.is_none() { (*alg).digest = Some(shash_default_digest); }
    if (*alg).export.is_none() && (*alg).halg.statesize == 0 { (*alg).halg.statesize = (*alg).descsize; }
    if (*alg).setkey.is_none() { (*alg).setkey = Some(shash_no_setkey); }
    if base.cra_flags & CRYPTO_AHASH_ALG_BLOCK_ONLY != 0 {
        (*alg).descsize += base.cra_blocksize + 1;
        (*alg).statesize += base.cra_blocksize + 1;
        (*alg).export_core = (*alg).export;
        (*alg).import_core = (*alg).import;
    } else if (*alg).export_core.is_none() || (*alg).import_core.is_none() {
        (*alg).export_core = Some(shash_default_export_core);
        (*alg).import_core = Some(shash_default_import_core);
        base.cra_flags |= CRYPTO_AHASH_ALG_NO_EXPORT_CORE;
    }
    if (*alg).descsize > HASH_MAX_DESCSIZE || (*alg).statesize > HASH_MAX_STATESIZE { return -EINVAL; }
    base.cra_reqsize = size_of::<shash_desc>() as u32 + (*alg).descsize;
    0
}

pub unsafe extern "C" fn crypto_register_shash(alg: *mut shash_alg) -> c_int {
    let err = shash_prepare_alg(alg);
    if err != 0 { return err; }
    crypto_register_alg(&mut (*alg).base)
}

pub unsafe extern "C" fn crypto_unregister_shash(alg: *mut shash_alg) { crypto_unregister_alg(&mut (*alg).base); }

pub unsafe extern "C" fn crypto_register_shashes(algs: *mut shash_alg, count: c_int) -> c_int {
    for i in 0..count {
        let ret = crypto_register_shash(algs.add(i as usize));
        if ret != 0 { crypto_unregister_shashes(algs, i); return ret; }
    }
    0
}

pub unsafe extern "C" fn crypto_unregister_shashes(algs: *mut shash_alg, count: c_int) {
    let mut i = count - 1;
    while i >= 0 { crypto_unregister_shash(algs.add(i as usize)); i -= 1; }
}

pub unsafe extern "C" fn shash_register_instance(tmpl: *mut crypto_template, inst: *mut shash_instance) -> c_int {
    if (*inst).free.is_none() { return -EINVAL; }
    let err = shash_prepare_alg(&mut (*inst).alg);
    if err != 0 { return err; }
    crypto_register_instance(tmpl, shash_crypto_instance(inst))
}

pub unsafe extern "C" fn shash_free_singlespawn_instance(inst: *mut shash_instance) {
    crypto_drop_spawn(shash_instance_ctx(inst));
    kfree(inst as *mut c_void);
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Synchronous cryptographic hash type");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
