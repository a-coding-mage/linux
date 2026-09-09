// SPDX-License-Identifier: GPL-2.0-or-later
/* LRW: as defined by Cyril Guyot in
 * http://grouper.ieee.org/groups/1619/email/pdf00017.pdf
 *
 * Copyright (c) 2006 Rik Snel <rsnel@cube.dyndns.org>
 *
 * Based on ecb.c
 * Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
 */
/* This implementation is checked against the test vectors in the above
 * document and by a test vector provided by Ken Buchanan at
 * https://www.mail-archive.com/stds-p1619@listserv.ieee.org/msg00173.html
 *
 * The test vectors are included in the testing module tcrypt.[ch]
 */

// External kernel crypto headers and symbols are supplied by the surrounding build.

const LRW_BLOCK_SIZE: usize = 16;

#[repr(C)]
struct lrw_tfm_ctx {
    child: *mut crypto_skcipher,
    table: *mut gf128mul_64k,
    mulinc: [be128; 128],
}

#[repr(C)]
struct lrw_request_ctx {
    t: be128,
    subreq: skcipher_request,
}

#[inline]
unsafe fn lrw_setbit128_bbe(b: *mut core::ffi::c_void, bit: i32) {
    __set_bit(bit ^ (0x80 - BITS_PER_BYTE), b);
}

unsafe fn lrw_setkey(parent: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(parent) as *mut lrw_tfm_ctx;
    let child = (*ctx).child;
    let bsize: i32 = LRW_BLOCK_SIZE as i32;
    let tweak = key.add(keylen as usize - bsize as usize);
    let mut tmp: be128 = core::mem::zeroed();

    crypto_skcipher_clear_flags(child, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(child, crypto_skcipher_get_flags(parent) & CRYPTO_TFM_REQ_MASK);
    let err = crypto_skcipher_setkey(child, key, keylen - bsize as u32);
    if err != 0 { return err; }

    if !(*ctx).table.is_null() { gf128mul_free_64k((*ctx).table); }
    (*ctx).table = gf128mul_init_64k_bbe(tweak as *const be128);
    if (*ctx).table.is_null() { return -ENOMEM; }

    for i in 0..128 {
        lrw_setbit128_bbe(&mut tmp as *mut be128 as *mut core::ffi::c_void, i);
        (*ctx).mulinc[i] = tmp;
        gf128mul_64k_bbe(&mut (*ctx).mulinc[i], (*ctx).table);
    }
    0
}

unsafe fn lrw_next_index(counter: *mut u32) -> i32 {
    let mut res = 0;
    for i in 0..4 {
        let value = *counter.add(i);
        if value.wrapping_add(1) != 0 {
            *counter.add(i) = value.wrapping_add(1);
            return res + ffz(value);
        }
        *counter.add(i) = 0;
        res += 32;
    }
    127
}

unsafe fn lrw_xor_tweak(req: *mut skcipher_request, second_pass: bool) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *const lrw_tfm_ctx;
    let rctx = skcipher_request_ctx(req) as *mut lrw_request_ctx;
    let mut t = (*rctx).t;
    let mut w: skcipher_walk = core::mem::zeroed();
    let mut req2 = req;

    if second_pass {
        req2 = &mut (*rctx).subreq;
        skcipher_request_set_tfm(req2, tfm);
    }
    let mut err = skcipher_walk_virt(&mut w, req2, false);
    if err != 0 { return err; }

    let iv = w.iv as *mut __be32;
    let mut counter = [
        be32_to_cpu(*iv.add(3)), be32_to_cpu(*iv.add(2)),
        be32_to_cpu(*iv.add(1)), be32_to_cpu(*iv.add(0)),
    ];

    while w.nbytes != 0 {
        let mut avail = w.nbytes;
        let mut wsrc = w.src.virt.addr as *const be128;
        let mut wdst = w.dst.virt.addr as *mut be128;
        loop {
            be128_xor(wdst, &t, wsrc);
            wdst = wdst.add(1); wsrc = wsrc.add(1);
            be128_xor(&mut t, &t, &(*ctx).mulinc[lrw_next_index(counter.as_mut_ptr()) as usize]);
            avail -= LRW_BLOCK_SIZE;
            if avail < LRW_BLOCK_SIZE { break; }
        }
        if second_pass && w.nbytes == w.total {
            *iv.add(0) = cpu_to_be32(counter[3]); *iv.add(1) = cpu_to_be32(counter[2]);
            *iv.add(2) = cpu_to_be32(counter[1]); *iv.add(3) = cpu_to_be32(counter[0]);
        }
        err = skcipher_walk_done(&mut w, avail);
    }
    err
}

unsafe fn lrw_xor_tweak_pre(req: *mut skcipher_request) -> i32 { lrw_xor_tweak(req, false) }
unsafe fn lrw_xor_tweak_post(req: *mut skcipher_request) -> i32 { lrw_xor_tweak(req, true) }

unsafe extern "C" fn lrw_crypt_done(data: *mut core::ffi::c_void, mut err: i32) {
    let req = data as *mut skcipher_request;
    if err == 0 {
        let rctx = skcipher_request_ctx(req) as *mut lrw_request_ctx;
        (*rctx).subreq.base.flags &= !CRYPTO_TFM_REQ_MAY_SLEEP;
        err = lrw_xor_tweak_post(req);
    }
    skcipher_request_complete(req, err);
}

unsafe fn lrw_init_crypt(req: *mut skcipher_request) {
    let ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)) as *const lrw_tfm_ctx;
    let rctx = skcipher_request_ctx(req) as *mut lrw_request_ctx;
    let subreq = &mut (*rctx).subreq;
    skcipher_request_set_tfm(subreq, (*ctx).child);
    skcipher_request_set_callback(subreq, (*req).base.flags, Some(lrw_crypt_done), req as *mut _);
    skcipher_request_set_crypt(subreq, (*req).dst, (*req).dst, (*req).cryptlen, (*req).iv);
    core::ptr::copy_nonoverlapping((*req).iv as *const u8, &mut (*rctx).t as *mut be128 as *mut u8, core::mem::size_of::<be128>());
    gf128mul_64k_bbe(&mut (*rctx).t, (*ctx).table);
}

unsafe fn lrw_encrypt(req: *mut skcipher_request) -> i32 {
    let rctx = skcipher_request_ctx(req) as *mut lrw_request_ctx;
    lrw_init_crypt(req);
    let err = lrw_xor_tweak_pre(req); if err != 0 { return err; }
    let err = crypto_skcipher_encrypt(&mut (*rctx).subreq); if err != 0 { return err; }
    lrw_xor_tweak_post(req)
}

unsafe fn lrw_decrypt(req: *mut skcipher_request) -> i32 {
    let rctx = skcipher_request_ctx(req) as *mut lrw_request_ctx;
    lrw_init_crypt(req);
    let err = lrw_xor_tweak_pre(req); if err != 0 { return err; }
    let err = crypto_skcipher_decrypt(&mut (*rctx).subreq); if err != 0 { return err; }
    lrw_xor_tweak_post(req)
}

unsafe fn lrw_init_tfm(tfm: *mut crypto_skcipher) -> i32 {
    let inst = skcipher_alg_instance(tfm);
    let spawn = skcipher_instance_ctx(inst);
    let ctx = crypto_skcipher_ctx(tfm) as *mut lrw_tfm_ctx;
    let cipher = crypto_spawn_skcipher(spawn);
    if IS_ERR(cipher) { return PTR_ERR(cipher); }
    (*ctx).child = cipher;
    crypto_skcipher_set_reqsize(tfm, crypto_skcipher_reqsize(cipher) + core::mem::size_of::<lrw_request_ctx>());
    0
}
unsafe fn lrw_exit_tfm(tfm: *mut crypto_skcipher) {
    let ctx = crypto_skcipher_ctx(tfm) as *mut lrw_tfm_ctx;
    if !(*ctx).table.is_null() { gf128mul_free_64k((*ctx).table); }
    crypto_free_skcipher((*ctx).child);
}
unsafe fn lrw_free_instance(inst: *mut skcipher_instance) { crypto_drop_skcipher(skcipher_instance_ctx(inst)); kfree(inst as *mut core::ffi::c_void); }
unsafe fn lrw_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> i32 {
    let mut spawn: *mut crypto_skcipher_spawn = core::ptr::null_mut();
    let mut alg: *mut skcipher_alg_common;
    let inst = kzalloc(core::mem::size_of::<skcipher_instance>() + core::mem::size_of::<crypto_skcipher_spawn>(), GFP_KERNEL) as *mut skcipher_instance;
    if inst.is_null() { return -ENOMEM; }
    spawn = skcipher_instance_ctx(inst);
    let mut mask = 0u32;
    let mut err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_SKCIPHER, &mut mask);
    if err != 0 { lrw_free_instance(inst); return err; }
    let cipher_name = crypto_attr_alg_name(*tb.add(1));
    if IS_ERR(cipher_name) { lrw_free_instance(inst); return PTR_ERR(cipher_name); }
    err = crypto_grab_skcipher(spawn, skcipher_crypto_instance(inst), cipher_name, 0, mask);
    if err != 0 { lrw_free_instance(inst); return err; }
    alg = crypto_spawn_skcipher_alg_common(spawn);
    if (*alg).base.cra_blocksize != LRW_BLOCK_SIZE || (*alg).ivsize != 0 { lrw_free_instance(inst); return -EINVAL; }
    err = crypto_inst_setname(skcipher_crypto_instance(inst), "lrw", &(*alg).base);
    if err != 0 { lrw_free_instance(inst); return err; }
    (*inst).alg.base.cra_priority = (*alg).base.cra_priority;
    (*inst).alg.base.cra_blocksize = LRW_BLOCK_SIZE;
    (*inst).alg.ivsize = LRW_BLOCK_SIZE;
    (*inst).alg.min_keysize = (*alg).min_keysize + LRW_BLOCK_SIZE;
    (*inst).alg.max_keysize = (*alg).max_keysize + LRW_BLOCK_SIZE;
    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<lrw_tfm_ctx>();
    (*inst).alg.init = Some(lrw_init_tfm); (*inst).alg.exit = Some(lrw_exit_tfm);
    (*inst).alg.setkey = Some(lrw_setkey); (*inst).alg.encrypt = Some(lrw_encrypt); (*inst).alg.decrypt = Some(lrw_decrypt);
    (*inst).free = Some(lrw_free_instance);
    err = skcipher_register_instance(tmpl, inst);
    if err != 0 { lrw_free_instance(inst); }
    err
}

static mut lrw_tmpl: crypto_template = crypto_template {
    name: "lrw",
    create: Some(lrw_create),
    module: THIS_MODULE,
};

unsafe extern "C" fn lrw_module_init() -> i32 { crypto_register_template(&mut lrw_tmpl) }
unsafe extern "C" fn lrw_module_exit() { crypto_unregister_template(&mut lrw_tmpl); }

// module_init!(lrw_module_init); module_exit!(lrw_module_exit);
// MODULE_LICENSE!("GPL");
// MODULE_DESCRIPTION!("LRW block cipher mode");
// MODULE_ALIAS_CRYPTO!("lrw");
// MODULE_SOFTDEP!("pre: ecb");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
