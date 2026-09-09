// SPDX-License-Identifier: GPL-2.0
/* Key setup facility for FS encryption support. */

// External kernel/fscrypt declarations are supplied by other translation units.

#[repr(C)]
pub static mut fscrypt_modes: [fscrypt_mode; 8] = [
    fscrypt_mode { friendly_name: b"AES-256-XTS\0".as_ptr() as _, cipher_str: b"xts(aes)\0".as_ptr() as _, keysize: 64, security_strength: 32, ivsize: 16, blk_crypto_mode: BLK_ENCRYPTION_MODE_AES_256_XTS, logged_cryptoapi_impl: 0 },
    fscrypt_mode { friendly_name: b"AES-256-CBC-CTS\0".as_ptr() as _, cipher_str: b"cts(cbc(aes))\0".as_ptr() as _, keysize: 32, security_strength: 32, ivsize: 16, blk_crypto_mode: 0, logged_cryptoapi_impl: 0 },
    fscrypt_mode { friendly_name: b"AES-128-CBC-ESSIV\0".as_ptr() as _, cipher_str: b"essiv(cbc(aes),sha256)\0".as_ptr() as _, keysize: 16, security_strength: 16, ivsize: 16, blk_crypto_mode: BLK_ENCRYPTION_MODE_AES_128_CBC_ESSIV, logged_cryptoapi_impl: 0 },
    fscrypt_mode { friendly_name: b"AES-128-CBC-CTS\0".as_ptr() as _, cipher_str: b"cts(cbc(aes))\0".as_ptr() as _, keysize: 16, security_strength: 16, ivsize: 16, blk_crypto_mode: 0, logged_cryptoapi_impl: 0 },
    fscrypt_mode { friendly_name: b"SM4-XTS\0".as_ptr() as _, cipher_str: b"xts(sm4)\0".as_ptr() as _, keysize: 32, security_strength: 16, ivsize: 16, blk_crypto_mode: BLK_ENCRYPTION_MODE_SM4_XTS, logged_cryptoapi_impl: 0 },
    fscrypt_mode { friendly_name: b"SM4-CBC-CTS\0".as_ptr() as _, cipher_str: b"cts(cbc(sm4))\0".as_ptr() as _, keysize: 16, security_strength: 16, ivsize: 16, blk_crypto_mode: 0, logged_cryptoapi_impl: 0 },
    fscrypt_mode { friendly_name: b"Adiantum\0".as_ptr() as _, cipher_str: b"adiantum(xchacha12,aes)\0".as_ptr() as _, keysize: 32, security_strength: 32, ivsize: 32, blk_crypto_mode: BLK_ENCRYPTION_MODE_ADIANTUM, logged_cryptoapi_impl: 0 },
    fscrypt_mode { friendly_name: b"AES-256-HCTR2\0".as_ptr() as _, cipher_str: b"hctr2(aes)\0".as_ptr() as _, keysize: 32, security_strength: 32, ivsize: 32, blk_crypto_mode: 0, logged_cryptoapi_impl: 0 },
];

unsafe fn select_encryption_mode(policy: *const fscrypt_policy, inode: *const inode) -> *mut fscrypt_mode {
    if S_ISREG((*inode).i_mode) { return &mut fscrypt_modes[fscrypt_policy_contents_mode(policy) as usize]; }
    if S_ISDIR((*inode).i_mode) || S_ISLNK((*inode).i_mode) { return &mut fscrypt_modes[fscrypt_policy_fnames_mode(policy) as usize]; }
    WARN_ONCE(true, "fscrypt: filesystem tried to load encryption info for inode %llu, which is not encryptable (file type %d)\n", (*inode).i_ino, (*inode).i_mode & S_IFMT);
    ERR_PTR(-EINVAL)
}

unsafe fn fscrypt_allocate_skcipher(mode: *mut fscrypt_mode, raw_key: *const u8, inode: *const inode) -> *mut crypto_sync_skcipher {
    let tfm = crypto_alloc_sync_skcipher((*mode).cipher_str, 0, FSCRYPT_CRYPTOAPI_MASK);
    if IS_ERR(tfm) {
        if PTR_ERR(tfm) == -ENOENT { fscrypt_warn(inode, "Missing crypto API support for %s (API name: \"%s\")", (*mode).friendly_name, (*mode).cipher_str); return ERR_PTR(-ENOPKG); }
        fscrypt_err(inode, "Error allocating '%s' transform: %ld", (*mode).cipher_str, PTR_ERR(tfm)); return tfm;
    }
    if xchg(&mut_ref((*mode).logged_cryptoapi_impl), 1) == 0 { pr_info("fscrypt: %s using implementation \"%s\"\n", (*mode).friendly_name, crypto_skcipher_driver_name(&(*tfm).base)); }
    if WARN_ON_ONCE(crypto_sync_skcipher_ivsize(tfm) != (*mode).ivsize) { crypto_free_sync_skcipher(tfm); return ERR_PTR(-EINVAL); }
    crypto_sync_skcipher_set_flags(tfm, CRYPTO_TFM_REQ_FORBID_WEAK_KEYS);
    let err = crypto_sync_skcipher_setkey(tfm, raw_key, (*mode).keysize);
    if err != 0 { crypto_free_sync_skcipher(tfm); return ERR_PTR(err); }
    tfm
}

pub unsafe fn fscrypt_prepare_key(prep_key: *mut fscrypt_prepared_key, raw_key: *const u8, ci: *const fscrypt_inode_info) -> i32 {
    if fscrypt_using_inline_encryption(ci) { return fscrypt_prepare_inline_crypt_key(prep_key, raw_key, (*(*ci).ci_mode).keysize, false, ci); }
    let tfm = fscrypt_allocate_skcipher((*ci).ci_mode, raw_key, (*ci).ci_inode);
    if IS_ERR(tfm) { return PTR_ERR(tfm); }
    (*prep_key).tfm = tfm; 0
}

pub unsafe fn fscrypt_destroy_prepared_key(sb: *mut super_block, prep_key: *mut fscrypt_prepared_key) {
    crypto_free_sync_skcipher((*prep_key).tfm); fscrypt_destroy_inline_crypt_key(sb, prep_key); memzero_explicit(prep_key as *mut _, core::mem::size_of::<fscrypt_prepared_key>());
}

pub unsafe fn fscrypt_set_per_file_enc_key(ci: *mut fscrypt_inode_info, raw_key: *const u8) -> i32 { (*ci).ci_owns_key = true; fscrypt_prepare_key(&mut (*ci).ci_enc_key, raw_key, ci) }

unsafe fn fscrypt_derive_siphash_key(mk: *const fscrypt_master_key, context: u8, info: *const u8, infolen: u32, key: *mut siphash_key_t) {
    fscrypt_hkdf_expand(&(*mk).mk_secret.hkdf, context, info, infolen, key as *mut u8, core::mem::size_of::<siphash_key_t>() as u32);
    le64_to_cpus(&mut (*key).key[0]); le64_to_cpus(&mut (*key).key[1]);
}

pub unsafe fn fscrypt_derive_dirhash_key(ci: *mut fscrypt_inode_info, mk: *const fscrypt_master_key) { fscrypt_derive_siphash_key(mk, HKDF_CONTEXT_DIRHASH_KEY, (*ci).ci_nonce.as_ptr(), FSCRYPT_FILE_NONCE_SIZE, &mut (*ci).ci_dirhash_key); (*ci).ci_dirhash_key_initialized = true; }

pub unsafe fn fscrypt_hash_inode_number(ci: *mut fscrypt_inode_info, mk: *const fscrypt_master_key) { WARN_ON_ONCE((*(*ci).ci_inode).i_ino == 0); WARN_ON_ONCE(!(*mk).mk_ino_hash_key_initialized); (*ci).ci_hashed_ino = siphash_1u64((*(*ci).ci_inode).i_ino, &(*mk).mk_ino_hash_key) as u32; }

// The remaining implementation follows the C control flow and relies on the external
// kernel/fscrypt types, constants, helpers, list primitives, and memory-ordering APIs.
// Functions whose bodies require those declarations are kept as direct unsafe wrappers.

pub unsafe fn fscrypt_get_encryption_info(inode: *mut inode, allow_unsupported: bool) -> i32 {
    let mut ctx = core::mem::MaybeUninit::<fscrypt_context>::uninit(); let mut policy = core::mem::MaybeUninit::<fscrypt_policy>::uninit();
    if fscrypt_has_encryption_key(inode) { return 0; }
    let mut res = (*(*inode).i_sb).s_cop.get_context(inode, ctx.as_mut_ptr(), core::mem::size_of::<fscrypt_context>());
    if res < 0 { if res == -ERANGE && allow_unsupported { return 0; } fscrypt_warn(inode, "Error %d getting encryption context", res); return res; }
    res = fscrypt_policy_from_context(policy.as_mut_ptr(), ctx.as_ptr(), res);
    if res != 0 { if allow_unsupported { return 0; } fscrypt_warn(inode, "Unrecognized or corrupt encryption context"); return res; }
    if !fscrypt_supported_policy(policy.as_ptr(), inode) { if allow_unsupported { return 0; } return -EINVAL; }
    res = fscrypt_setup_encryption_info(inode, policy.as_ptr(), fscrypt_context_nonce(ctx.as_ptr()), IS_CASEFOLDED(inode) && S_ISDIR((*inode).i_mode));
    if res == -ENOPKG && allow_unsupported { res = 0; } if res == -ENOKEY { res = 0; } res
}

pub unsafe fn fscrypt_put_encryption_info(inode: *mut inode) { let p = fscrypt_inode_info_addr(inode); put_crypt_info(*p); *p = core::ptr::null_mut(); }

pub unsafe fn fscrypt_free_inode(inode: *mut inode) { if IS_ENCRYPTED(inode) && S_ISLNK((*inode).i_mode) { kfree((*inode).i_link); (*inode).i_link = core::ptr::null_mut(); } }

pub unsafe fn fscrypt_drop_inode(inode: *mut inode) -> i32 { let ci = fscrypt_get_inode_info(inode); if ci.is_null() || (*ci).ci_master_key.is_null() { return 0; } if inode_state_read(inode) & I_DIRTY_ALL != 0 { return 0; } (!READ_ONCE((*(*ci).ci_master_key).mk_present)) as i32 }

pub unsafe fn fscrypt_prepare_new_inode(dir: *mut inode, inode: *mut inode, encrypt_ret: *mut bool) -> i32 {
    let policy = fscrypt_policy_to_inherit(dir);
    if policy.is_null() { return 0; }
    if IS_ERR(policy) { return PTR_ERR(policy); }
    if WARN_ON_ONCE((*inode).i_blkbits == 0) || WARN_ON_ONCE((*inode).i_mode == 0) { return -EINVAL; }
    if !S_ISREG((*inode).i_mode) && !S_ISDIR((*inode).i_mode) && !S_ISLNK((*inode).i_mode) { return 0; }
    *encrypt_ret = true;
    let mut nonce = [0u8; FSCRYPT_FILE_NONCE_SIZE as usize];
    get_random_bytes(nonce.as_mut_ptr(), FSCRYPT_FILE_NONCE_SIZE);
    fscrypt_setup_encryption_info(inode, policy, nonce.as_ptr(), IS_CASEFOLDED(dir) && S_ISDIR((*inode).i_mode))
}

pub unsafe fn fscrypt_set_per_file_enc_key_export(ci: *mut fscrypt_inode_info, raw_key: *const u8) -> i32 {
    fscrypt_set_per_file_enc_key(ci, raw_key)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
