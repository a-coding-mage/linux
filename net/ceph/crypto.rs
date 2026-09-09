// SPDX-License-Identifier: GPL-2.0

// Kernel headers and symbols referenced below are supplied by other translation units.

unsafe fn set_aes_tfm(key: *mut ceph_crypto_key) -> c_int {
    let noio_flag = memalloc_noio_save();
    (*key).aes_tfm = crypto_alloc_sync_skcipher(c"cbc(aes)".as_ptr(), 0, 0);
    memalloc_noio_restore(noio_flag);
    if is_err((*key).aes_tfm as *const _) {
        let ret = ptr_err((*key).aes_tfm as *const _);
        (*key).aes_tfm = core::ptr::null_mut();
        return ret;
    }
    let ret = crypto_sync_skcipher_setkey((*key).aes_tfm, (*key).key, (*key).len);
    if ret != 0 { return ret; }
    0
}

unsafe fn set_krb5_tfms(key: *mut ceph_crypto_key, key_usages: *const u32,
                        key_usage_cnt: c_int) -> c_int {
    let tk = krb5_buffer { len: (*key).len, data: (*key).key };
    let mut ret = 0;
    if key_usage_cnt > (*key).krb5_tfms.len() as c_int { return -EINVAL; }
    (*key).krb5_type = crypto_krb5_find_enctype(KRB5_ENCTYPE_AES256_CTS_HMAC_SHA384_192);
    if (*key).krb5_type.is_null() { return -ENOPKG; }
    let noio_flag = memalloc_noio_save();
    for i in 0..key_usage_cnt as usize {
        (*key).krb5_tfms[i] = crypto_krb5_prepare_encryption(
            (*key).krb5_type, &tk, *key_usages.add(i), GFP_NOIO);
        if is_err((*key).krb5_tfms[i] as *const _) {
            ret = ptr_err((*key).krb5_tfms[i] as *const _);
            (*key).krb5_tfms[i] = core::ptr::null_mut();
            break;
        }
    }
    memalloc_noio_restore(noio_flag);
    ret
}

pub unsafe fn ceph_crypto_key_prepare(key: *mut ceph_crypto_key, key_usages: *const u32,
                                      key_usage_cnt: c_int) -> c_int {
    match (*key).type_ {
        CEPH_CRYPTO_NONE => 0,
        CEPH_CRYPTO_AES => set_aes_tfm(key),
        CEPH_CRYPTO_AES256KRB5 => {
            hmac_sha256_preparekey(&mut (*key).hmac_key, (*key).key, (*key).len);
            set_krb5_tfms(key, key_usages, key_usage_cnt)
        },
        _ => -ENOTSUPP,
    }
}

pub unsafe fn ceph_crypto_key_clone(dst: *mut ceph_crypto_key, src: *const ceph_crypto_key) -> c_int {
    (*dst).type_ = (*src).type_;
    (*dst).created = (*src).created;
    (*dst).len = (*src).len;
    (*dst).key = kmemdup((*src).key as *const _, (*src).len, GFP_NOIO);
    if (*dst).key.is_null() { return -ENOMEM; }
    0
}

pub unsafe fn ceph_crypto_key_decode(key: *mut ceph_crypto_key, p: *mut *mut c_void,
                                     end: *mut c_void) -> c_int {
    if !ceph_decode_need(p, end, 2 * core::mem::size_of::<u16>() + core::mem::size_of_val(&(*key).created)) {
        dout(c"failed to decode crypto key\n".as_ptr()); return -EINVAL;
    }
    (*key).type_ = ceph_decode_16(p);
    ceph_decode_copy(p, &mut (*key).created as *mut _ as *mut c_void, core::mem::size_of_val(&(*key).created));
    (*key).len = ceph_decode_16(p);
    if !ceph_decode_need(p, end, (*key).len as usize) { dout(c"failed to decode crypto key\n".as_ptr()); return -EINVAL; }
    if (*key).len > CEPH_MAX_KEY_LEN { pr_err(c"secret too big %d\n".as_ptr(), (*key).len); return -EINVAL; }
    (*key).key = kmemdup(*p, (*key).len as usize, GFP_NOIO);
    if (*key).key.is_null() { return -ENOMEM; }
    memzero_explicit(*p, (*key).len as usize);
    *p = (*p).add((*key).len as usize);
    0
}

pub unsafe fn ceph_crypto_key_unarmor(key: *mut ceph_crypto_key, inkey: *const c_char) -> c_int {
    let inlen = strlen(inkey) as c_int;
    let mut blen = inlen * 3 / 4;
    let buf = kmalloc(blen as usize, GFP_NOFS);
    if buf.is_null() { return -ENOMEM; }
    blen = ceph_unarmor(buf, inkey, inkey.add(inlen as usize));
    if blen < 0 { kfree(buf); return blen; }
    let mut p = buf;
    let ret = ceph_crypto_key_decode(key, &mut p, p.add(blen as usize));
    kfree(buf);
    ret
}

pub unsafe fn ceph_crypto_key_destroy(key: *mut ceph_crypto_key) {
    if key.is_null() { return; }
    kfree_sensitive((*key).key); (*key).key = core::ptr::null_mut();
    if (*key).type_ == CEPH_CRYPTO_AES {
        if !(*key).aes_tfm.is_null() { crypto_free_sync_skcipher((*key).aes_tfm); (*key).aes_tfm = core::ptr::null_mut(); }
    } else if (*key).type_ == CEPH_CRYPTO_AES256KRB5 {
        memzero_explicit(&mut (*key).hmac_key as *mut _ as *mut c_void, core::mem::size_of_val(&(*key).hmac_key));
        for tfm in &mut (*key).krb5_tfms { if !tfm.is_null() { crypto_free_aead(*tfm); *tfm = core::ptr::null_mut(); } }
    }
}

static mut AES_IV: *const u8 = CEPH_AES_IV as *const u8;

unsafe fn setup_sgtable(sgt: *mut sg_table, prealloc_sg: *mut scatterlist,
                        buf: *const c_void, buf_len: u32) -> c_int {
    if buf_len == 0 { core::ptr::write_bytes(sgt as *mut u8, 0, core::mem::size_of::<sg_table>()); return -EINVAL; }
    let is_vmalloc = is_vmalloc_addr(buf);
    let mut off = offset_in_page(buf);
    let mut chunk_cnt = 1u32;
    let mut chunk_len = page_align(off + buf_len);
    if is_vmalloc { chunk_cnt = chunk_len >> PAGE_SHIFT; chunk_len = PAGE_SIZE; }
    if chunk_cnt > 1 { let ret = sg_alloc_table(sgt, chunk_cnt, GFP_NOFS); if ret != 0 { return ret; } }
    else { sg_init_table(prealloc_sg, 1); (*sgt).sgl = prealloc_sg; (*sgt).nents = 1; (*sgt).orig_nents = 1; }
    let mut sg = (*sgt).sgl; let mut left = buf_len; let mut ptr = buf as *const u8;
    for _ in 0..(*sgt).orig_nents { let page = if is_vmalloc { vmalloc_to_page(ptr as *const _) } else { virt_to_page(ptr as *const _) }; let len = core::cmp::min(chunk_len - off, left); sg_set_page(sg, page, len, off); off = 0; ptr = ptr.add(len as usize); left -= len; sg = sg_next(sg); }
    0
}

unsafe fn teardown_sgtable(sgt: *mut sg_table) { if (*sgt).orig_nents > 1 { sg_free_table(sgt); } }

// The cryptographic operation helpers retain the kernel ABI and are declared externally.
extern "C" {
    fn ceph_aes_crypt(key: *const ceph_crypto_key, encrypt: bool, buf: *mut c_void, buf_len: c_int, in_len: c_int, out: *mut c_int) -> c_int;
    fn ceph_krb5_encrypt(key: *const ceph_crypto_key, slot: c_int, buf: *mut c_void, buf_len: c_int, in_len: c_int, out: *mut c_int) -> c_int;
    fn ceph_krb5_decrypt(key: *const ceph_crypto_key, slot: c_int, buf: *mut c_void, buf_len: c_int, in_len: c_int, out: *mut c_int) -> c_int;
}

pub unsafe fn ceph_crypt(key: *const ceph_crypto_key, usage_slot: c_int, encrypt: bool, buf: *mut c_void, buf_len: c_int, in_len: c_int, out: *mut c_int) -> c_int {
    match (*key).type_ { CEPH_CRYPTO_NONE => { *out = in_len; 0 }, CEPH_CRYPTO_AES => ceph_aes_crypt(key, encrypt, buf, buf_len, in_len, out), CEPH_CRYPTO_AES256KRB5 => if encrypt { ceph_krb5_encrypt(key, usage_slot, buf, buf_len, in_len, out) } else { ceph_krb5_decrypt(key, usage_slot, buf, buf_len, in_len, out) }, _ => -ENOTSUPP }
}

pub unsafe fn ceph_crypt_data_offset(key: *const ceph_crypto_key) -> c_int { match (*key).type_ { CEPH_CRYPTO_NONE | CEPH_CRYPTO_AES => 0, CEPH_CRYPTO_AES256KRB5 => AES_BLOCK_SIZE as c_int, _ => bug() } }
pub unsafe fn ceph_crypt_buflen(key: *const ceph_crypto_key, data_len: c_int) -> c_int { match (*key).type_ { CEPH_CRYPTO_NONE => data_len, CEPH_CRYPTO_AES => data_len + AES_BLOCK_SIZE as c_int - (data_len & (AES_BLOCK_SIZE as c_int - 1)), CEPH_CRYPTO_AES256KRB5 => AES_BLOCK_SIZE as c_int + data_len + 24, _ => bug() } }
pub unsafe fn ceph_hmac_sha256(key: *const ceph_crypto_key, buf: *const c_void, len: c_int, hmac: *mut u8) { match (*key).type_ { CEPH_CRYPTO_NONE | CEPH_CRYPTO_AES => core::ptr::write_bytes(hmac, 0, SHA256_DIGEST_SIZE), CEPH_CRYPTO_AES256KRB5 => hmac_sha256(&(*key).hmac_key, buf, len, hmac), _ => bug() } }

unsafe fn ceph_key_preparse(prep: *mut key_preparsed_payload) -> c_int {
    let datalen = (*prep).datalen;
    if datalen <= 0 || datalen > 32767 || (*prep).data.is_null() { return -EINVAL; }
    let ckey = kzalloc_obj::<ceph_crypto_key>();
    if ckey.is_null() { return -ENOMEM; }
    let mut p = (*prep).data as *mut c_void;
    let ret = ceph_crypto_key_decode(&mut *ckey, &mut p, ((*prep).data as *const u8).add(datalen).cast_mut().cast());
    if ret < 0 { kfree(ckey as *mut c_void); return ret; }
    (*prep).payload.data[0] = ckey as *mut c_void;
    (*prep).quotalen = datalen;
    0
}

unsafe fn ceph_key_free_preparse(prep: *mut key_preparsed_payload) {
    let ckey = (*prep).payload.data[0] as *mut ceph_crypto_key;
    ceph_crypto_key_destroy(ckey); kfree(ckey as *mut c_void);
}

unsafe fn ceph_key_destroy(key: *mut key) {
    let ckey = (*key).payload.data[0] as *mut ceph_crypto_key;
    ceph_crypto_key_destroy(ckey); kfree(ckey as *mut c_void);
}

static mut key_type_ceph: key_type = key_type {
    name: c"ceph".as_ptr(),
    preparse: Some(ceph_key_preparse),
    free_preparse: Some(ceph_key_free_preparse),
    instantiate: Some(generic_key_instantiate),
    destroy: Some(ceph_key_destroy),
};

pub unsafe fn ceph_crypto_init() -> c_int { register_key_type(&mut key_type_ceph) }
pub unsafe fn ceph_crypto_shutdown() { unregister_key_type(&mut key_type_ceph); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
