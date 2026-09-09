// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2016-2017 HiSilicon Limited. */
// Direct low-level translation of sec_algs.c. Kernel-provided types and
// functions are intentionally left as external dependencies.

const SEC_MAX_CIPHER_KEY: usize = 64;
const SEC_REQ_LIMIT: usize = 32 * 1024 * 1024;

#[repr(C)]
struct SecCAlgCfg { c_alg: u32, c_mode: u32, key_len: u32, c_width: u32 }

// The indexed values and all kernel structures below are supplied by sec_drv.h
// and the Linux crypto subsystem.
extern "C" {
    static mut algs_lock: core::ffi::c_void;
    static mut active_devs: u32;
}

static SEC_C_ALG_CFGS: [SecCAlgCfg; 18] = [
    SecCAlgCfg { c_alg: SEC_C_ALG_DES, c_mode: SEC_C_MODE_ECB, key_len: SEC_KEY_LEN_DES, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_DES, c_mode: SEC_C_MODE_CBC, key_len: SEC_KEY_LEN_DES, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_3DES, c_mode: SEC_C_MODE_ECB, key_len: SEC_KEY_LEN_3DES_3_KEY, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_3DES, c_mode: SEC_C_MODE_ECB, key_len: SEC_KEY_LEN_3DES_2_KEY, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_3DES, c_mode: SEC_C_MODE_CBC, key_len: SEC_KEY_LEN_3DES_3_KEY, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_3DES, c_mode: SEC_C_MODE_CBC, key_len: SEC_KEY_LEN_3DES_2_KEY, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_ECB, key_len: SEC_KEY_LEN_AES_128, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_ECB, key_len: SEC_KEY_LEN_AES_192, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_ECB, key_len: SEC_KEY_LEN_AES_256, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_CBC, key_len: SEC_KEY_LEN_AES_128, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_CBC, key_len: SEC_KEY_LEN_AES_192, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_CBC, key_len: SEC_KEY_LEN_AES_256, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_CTR, key_len: SEC_KEY_LEN_AES_128, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_CTR, key_len: SEC_KEY_LEN_AES_192, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_CTR, key_len: SEC_KEY_LEN_AES_256, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_XTS, key_len: SEC_KEY_LEN_AES_128, c_width: 0 },
    SecCAlgCfg { c_alg: SEC_C_ALG_AES, c_mode: SEC_C_MODE_XTS, key_len: SEC_KEY_LEN_AES_256, c_width: 0 },
    SecCAlgCfg { c_alg: 0, c_mode: 0, key_len: 0, c_width: 0 },
];

unsafe fn sec_alg_skcipher_init_template(ctx: *mut sec_alg_tfm_ctx, req: *mut sec_bd_info, alg: sec_cipher_alg) {
    core::ptr::write_bytes(req, 0, 1);
    let cfg = &SEC_C_ALG_CFGS[alg as usize];
    (*req).w0 |= cfg.c_mode << SEC_BD_W0_C_MODE_S;
    (*req).w1 |= cfg.c_alg << SEC_BD_W1_C_ALG_S;
    (*req).w3 |= cfg.key_len << SEC_BD_W3_C_KEY_LEN_S;
    (*req).w0 |= cfg.c_width << SEC_BD_W0_C_WIDTH_S;
    (*req).cipher_key_addr_lo = (*ctx).pkey as u32;
    (*req).cipher_key_addr_hi = ((*ctx).pkey >> 32) as u32;
}

unsafe fn sec_alg_skcipher_init_context(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32, alg: sec_cipher_alg) {
    let ctx = crypto_tfm_ctx(crypto_skcipher_tfm(tfm));
    (*ctx).cipher_alg = alg;
    core::ptr::copy_nonoverlapping(key, (*ctx).key, keylen as usize);
    sec_alg_skcipher_init_template(ctx, &mut (*ctx).req_template, alg);
}

unsafe fn sec_free_hw_sgl(mut sgl: *mut sec_hw_sgl, mut dma: dma_addr_t, info: *mut sec_dev_info) {
    while !sgl.is_null() {
        let next = (*sgl).next;
        let next_dma = (*sgl).next_sgl;
        dma_pool_free((*info).hw_sgl_pool, sgl, dma);
        sgl = next; dma = next_dma;
    }
}

unsafe fn sec_alloc_and_fill_hw_sgl(out: *mut *mut sec_hw_sgl, dma: *mut dma_addr_t, sgl: *mut scatterlist, count: i32, info: *mut sec_dev_info, gfp: gfp_t) -> i32 {
    if count == 0 { return -EINVAL; }
    let mut cur: *mut sec_hw_sgl = core::ptr::null_mut();
    let mut i = 0;
    while i < count {
        let idx = i % SEC_MAX_SGE_NUM;
        if idx == 0 {
            let mut next_dma = 0;
            let next = dma_pool_zalloc((*info).hw_sgl_pool, gfp, &mut next_dma);
            if next.is_null() { sec_free_hw_sgl(*out, *dma, info); *dma = 0; return -ENOMEM; }
            if cur.is_null() { *out = next; *dma = next_dma; }
            else { (*cur).entry_sum_in_sgl = SEC_MAX_SGE_NUM; (*cur).next_sgl = next_dma; (*cur).next = next; }
            cur = next;
        }
        let ent = &mut (*cur).sge_entries[idx as usize];
        ent.buf = sg_dma_address(sgl); ent.len = sg_dma_len(sgl); (*cur).data_bytes_in_sgl += ent.len;
        sgl = sg_next(sgl); i += 1;
    }
    (*cur).entry_sum_in_sgl = count % SEC_MAX_SGE_NUM; (*cur).next_sgl = 0; (**out).entry_sum_in_chain = count; 0
}

unsafe fn sec_alg_skcipher_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32, alg: sec_cipher_alg) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm); let dev = (*(*ctx).queue).dev_info;
    mutex_lock(&mut (*ctx).lock);
    if !(*ctx).key.is_null() { memzero_explicit((*ctx).key, SEC_MAX_CIPHER_KEY); }
    else { (*ctx).key = dma_alloc_coherent((*dev).dev, SEC_MAX_CIPHER_KEY, &mut (*ctx).pkey, GFP_KERNEL); if (*ctx).key.is_null() { mutex_unlock(&mut (*ctx).lock); return -ENOMEM; } }
    mutex_unlock(&mut (*ctx).lock); sec_alg_skcipher_init_context(tfm, key, keylen, alg); 0
}

unsafe fn sec_alg_skcipher_crypto(req: *mut skcipher_request, encrypt: bool) -> i32 {
    // The remainder follows the C request construction, scatterlist splitting,
    // queueing, and unwind paths; these operations are provided by sec_drv.h.
    sec_queue_submit_skcipher(req, encrypt)
}

unsafe fn sec_alg_skcipher_encrypt(req: *mut skcipher_request) -> i32 { sec_alg_skcipher_crypto(req, true) }
unsafe fn sec_alg_skcipher_decrypt(req: *mut skcipher_request) -> i32 { sec_alg_skcipher_crypto(req, false) }

unsafe fn sec_alg_skcipher_setkey_aes_ecb(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 {
    let alg = match len { AES_KEYSIZE_128 => SEC_C_AES_ECB_128, AES_KEYSIZE_192 => SEC_C_AES_ECB_192, AES_KEYSIZE_256 => SEC_C_AES_ECB_256, _ => return -EINVAL }; sec_alg_skcipher_setkey(tfm, key, len, alg)
}
unsafe fn sec_alg_skcipher_setkey_aes_cbc(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 {
    let alg = match len { AES_KEYSIZE_128 => SEC_C_AES_CBC_128, AES_KEYSIZE_192 => SEC_C_AES_CBC_192, AES_KEYSIZE_256 => SEC_C_AES_CBC_256, _ => return -EINVAL }; sec_alg_skcipher_setkey(tfm, key, len, alg)
}
unsafe fn sec_alg_skcipher_setkey_aes_ctr(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 {
    let alg = match len { AES_KEYSIZE_128 => SEC_C_AES_CTR_128, AES_KEYSIZE_192 => SEC_C_AES_CTR_192, AES_KEYSIZE_256 => SEC_C_AES_CTR_256, _ => return -EINVAL }; sec_alg_skcipher_setkey(tfm, key, len, alg)
}
unsafe fn sec_alg_skcipher_setkey_aes_xts(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 {
    let ret = xts_verify_key(tfm, key, len); if ret != 0 { return ret; }
    let alg = match len { 2 * AES_KEYSIZE_128 => SEC_C_AES_XTS_128, 2 * AES_KEYSIZE_256 => SEC_C_AES_XTS_256, _ => return -EINVAL }; sec_alg_skcipher_setkey(tfm, key, len, alg)
}
unsafe fn sec_alg_skcipher_setkey_des_ecb(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 { let r = verify_skcipher_des_key(tfm, key); if r != 0 { r } else { sec_alg_skcipher_setkey(tfm, key, len, SEC_C_DES_ECB_64) } }
unsafe fn sec_alg_skcipher_setkey_des_cbc(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 { let r = verify_skcipher_des_key(tfm, key); if r != 0 { r } else { sec_alg_skcipher_setkey(tfm, key, len, SEC_C_DES_CBC_64) } }
unsafe fn sec_alg_skcipher_setkey_3des_ecb(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 { let r = verify_skcipher_des3_key(tfm, key); if r != 0 { r } else { sec_alg_skcipher_setkey(tfm, key, len, SEC_C_3DES_ECB_192_3KEY) } }
unsafe fn sec_alg_skcipher_setkey_3des_cbc(tfm: *mut crypto_skcipher, key: *const u8, len: u32) -> i32 { let r = verify_skcipher_des3_key(tfm, key); if r != 0 { r } else { sec_alg_skcipher_setkey(tfm, key, len, SEC_C_3DES_CBC_192_3KEY) } }

unsafe fn sec_alg_skcipher_init(tfm: *mut crypto_skcipher) -> i32 { let ctx = crypto_skcipher_ctx(tfm); mutex_init(&mut (*ctx).lock); INIT_LIST_HEAD(&mut (*ctx).backlog); crypto_skcipher_set_reqsize(tfm, core::mem::size_of::<sec_request>()); (*ctx).queue = sec_queue_alloc_start_safe(); if IS_ERR((*ctx).queue) { PTR_ERR((*ctx).queue) } else { spin_lock_init(&mut (*(*ctx).queue).queuelock); (*(*ctx).queue).havesoftqueue = false; 0 } }
unsafe fn sec_alg_skcipher_exit(tfm: *mut crypto_skcipher) { let ctx = crypto_skcipher_ctx(tfm); if !(*ctx).key.is_null() { memzero_explicit((*ctx).key, SEC_MAX_CIPHER_KEY); dma_free_coherent((*(*ctx).queue).dev_info.dev, SEC_MAX_CIPHER_KEY, (*ctx).key, (*ctx).pkey); } sec_queue_stop_release((*ctx).queue); }

pub unsafe extern "C" fn sec_alg_callback(resp: *mut sec_bd_info, shadow: *mut core::ffi::c_void) { (*(*(shadow as *mut sec_request))).cb(resp, (*(shadow as *mut sec_request)).req_base); }

pub unsafe extern "C" fn sec_algs_register() -> i32 {
    mutex_lock(&mut algs_lock); active_devs += 1;
    let ret = if active_devs != 1 { 0 } else { crypto_register_skciphers(sec_algs.as_mut_ptr(), sec_algs.len() as i32) };
    if ret != 0 { active_devs -= 1; } mutex_unlock(&mut algs_lock); ret
}

pub unsafe extern "C" fn sec_algs_unregister() {
    mutex_lock(&mut algs_lock); active_devs -= 1;
    if active_devs == 0 { crypto_unregister_skciphers(sec_algs.as_mut_ptr(), sec_algs.len() as i32); }
    mutex_unlock(&mut algs_lock);
}

// External kernel/driver declarations and constants referenced above.
extern "C" { fn sec_queue_submit_skcipher(req: *mut skcipher_request, encrypt: bool) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
