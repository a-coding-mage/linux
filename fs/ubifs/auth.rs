// SPDX-License-Identifier: GPL-2.0
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2018 Pengutronix, Sascha Hauer <s.hauer@pengutronix.de>
 */

/*
 * This file implements various helper functions for UBIFS authentication support
 */

// Kernel crypto, verification, key, and UBIFS declarations are supplied by
// other translation units.

pub unsafe fn __ubifs_node_calc_hash(
    c: *const ubifs_info,
    node: *const core::ffi::c_void,
    hash: *mut u8,
) -> i32 {
    let ch = node as *const ubifs_ch;
    crypto_shash_tfm_digest((*c).hash_tfm, node, le32_to_cpu((*ch).len), hash)
}

unsafe fn ubifs_hash_calc_hmac(
    c: *const ubifs_info,
    hash: *const u8,
    hmac: *mut u8,
) -> i32 {
    crypto_shash_tfm_digest((*c).hmac_tfm, hash, (*c).hash_len, hmac)
}

pub unsafe fn ubifs_prepare_auth_node(
    c: *mut ubifs_info,
    node: *mut core::ffi::c_void,
    inhash: *mut shash_desc,
) -> i32 {
    let auth = node as *mut ubifs_auth_node;
    let mut hash = [0u8; UBIFS_HASH_ARR_SZ as usize];
    let mut hash_desc: shash_desc = core::mem::zeroed();

    hash_desc.tfm = (*c).hash_tfm;
    ubifs_shash_copy_state(c, inhash, &mut hash_desc);

    let mut err = crypto_shash_final(&mut hash_desc, hash.as_mut_ptr());
    if err != 0 {
        return err;
    }

    err = ubifs_hash_calc_hmac(c, hash.as_ptr(), (*auth).hmac.as_mut_ptr());
    if err != 0 {
        return err;
    }

    (*auth).ch.node_type = UBIFS_AUTH_NODE;
    ubifs_prepare_node(c, auth, ubifs_auth_node_sz(c), 0);
    0
}

unsafe fn ubifs_get_desc(
    c: *const ubifs_info,
    tfm: *mut crypto_shash,
) -> *mut shash_desc {
    if !ubifs_authenticated(c) {
        return core::ptr::null_mut();
    }

    let desc = kmalloc(
        core::mem::size_of::<shash_desc>() + crypto_shash_descsize(tfm),
        GFP_KERNEL,
    ) as *mut shash_desc;
    if desc.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*desc).tfm = tfm;
    let err = crypto_shash_init(desc);
    if err != 0 {
        kfree(desc as *mut core::ffi::c_void);
        return ERR_PTR(err);
    }
    desc
}

pub unsafe fn __ubifs_hash_get_desc(c: *const ubifs_info) -> *mut shash_desc {
    ubifs_get_desc(c, (*c).hash_tfm)
}

pub unsafe fn ubifs_bad_hash(
    c: *const ubifs_info,
    node: *const core::ffi::c_void,
    hash: *const u8,
    lnum: i32,
    offs: i32,
) {
    let len = core::cmp::min((*c).hash_len, 20);
    let cropped = len != (*c).hash_len;
    let cont = if cropped { "..." } else { "" };
    let mut calc = [0u8; UBIFS_HASH_ARR_SZ as usize];

    __ubifs_node_calc_hash(c, node, calc.as_mut_ptr());
    ubifs_err(c, "hash mismatch on node at LEB %d:%d", lnum, offs);
    ubifs_err(c, "hash expected:   %*ph%s", len, hash, cont);
    ubifs_err(c, "hash calculated: %*ph%s", len, calc.as_ptr(), cont);
}

pub unsafe fn __ubifs_node_check_hash(
    c: *const ubifs_info,
    node: *const core::ffi::c_void,
    expected: *const u8,
) -> i32 {
    let mut calc = [0u8; UBIFS_HASH_ARR_SZ as usize];
    let err = __ubifs_node_calc_hash(c, node, calc.as_mut_ptr());
    if err != 0 {
        return err;
    }
    if ubifs_check_hash(c, expected, calc.as_ptr()) != 0 {
        return -EPERM;
    }
    0
}

pub unsafe fn ubifs_sb_verify_signature(
    c: *mut ubifs_info,
    sup: *const ubifs_sb_node,
) -> i32 {
    let sleb = ubifs_scan(c, UBIFS_SB_LNUM, UBIFS_SB_NODE_SZ, (*c).sbuf, 0);
    if IS_ERR(sleb) {
        return PTR_ERR(sleb);
    }
    let mut err;
    if (*sleb).nodes_cnt == 0 {
        ubifs_err(c, "Unable to find signature node");
        err = -EINVAL;
    } else {
        let snod = list_first_entry(&mut (*sleb).nodes, ubifs_scan_node, list);
        if (*snod).type_ != UBIFS_SIG_NODE {
            ubifs_err(c, "Signature node is of wrong type");
            err = -EINVAL;
        } else {
            let signode = (*snod).node as *const ubifs_sig_node;
            if le32_to_cpu((*signode).len) > (*snod).len - core::mem::size_of::<ubifs_sig_node>() {
                ubifs_err(c, "invalid signature len %d", le32_to_cpu((*signode).len));
                err = -EINVAL;
            } else if le32_to_cpu((*signode).type_) != UBIFS_SIGNATURE_TYPE_PKCS7 {
                ubifs_err(c, "Signature type %d is not supported\n", le32_to_cpu((*signode).type_));
                err = -EINVAL;
            } else {
                err = verify_pkcs7_signature(
                    sup as *const core::ffi::c_void,
                    core::mem::size_of::<ubifs_sb_node>(),
                    (*signode).sig,
                    le32_to_cpu((*signode).len),
                    core::ptr::null_mut(),
                    VERIFYING_UNSPECIFIED_SIGNATURE,
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                );
                if err != 0 { ubifs_err(c, "Failed to verify signature"); }
                else { ubifs_msg(c, "Successfully verified super block signature"); }
            }
        }
    }
    ubifs_scan_destroy(sleb);
    err
}

pub unsafe fn ubifs_init_authentication(c: *mut ubifs_info) -> i32 {
    let mut keyring_key;
    let mut err;
    let mut hmac_name = [0i8; CRYPTO_MAX_ALG_NAME as usize];
    if (*c).auth_hash_name.is_null() { ubifs_err(c, "authentication hash name needed with authentication"); return -EINVAL; }
    (*c).auth_hash_algo = match_string(hash_algo_name, HASH_ALGO__LAST, (*c).auth_hash_name);
    if (*c).auth_hash_algo < 0 { ubifs_err(c, "Unknown hash algo %s specified", (*c).auth_hash_name); return -EINVAL; }
    snprintf(hmac_name.as_mut_ptr(), CRYPTO_MAX_ALG_NAME, "hmac(%s)", (*c).auth_hash_name);
    keyring_key = request_key(&key_type_logon, (*c).auth_key_name, core::ptr::null_mut());
    if IS_ERR(keyring_key) { ubifs_err(c, "Failed to request key: %ld", PTR_ERR(keyring_key)); return PTR_ERR(keyring_key); }
    down_read(&mut (*keyring_key).sem);
    if (*keyring_key).type_ != &key_type_logon { ubifs_err(c, "key type must be logon"); err = -ENOKEY; }
    else {
        let ukp = user_key_payload_locked(keyring_key);
        if ukp.is_null() { err = -EKEYREVOKED; }
        else {
            (*c).hash_tfm = crypto_alloc_shash((*c).auth_hash_name, 0, 0);
            if IS_ERR((*c).hash_tfm) { err = PTR_ERR((*c).hash_tfm); ubifs_err(c, "Can not allocate %s: %d", (*c).auth_hash_name, err); }
            else if crypto_shash_digestsize((*c).hash_tfm) > UBIFS_HASH_ARR_SZ { ubifs_err(c, "hash %s is bigger than maximum allowed hash size (%d > %d)", (*c).auth_hash_name, (*c).hash_len, UBIFS_HASH_ARR_SZ); err = -EINVAL; }
            else {
                (*c).hash_len = crypto_shash_digestsize((*c).hash_tfm);
                (*c).hmac_tfm = crypto_alloc_shash(hmac_name.as_ptr(), 0, 0);
                if IS_ERR((*c).hmac_tfm) { err = PTR_ERR((*c).hmac_tfm); }
                else { err = crypto_shash_setkey((*c).hmac_tfm, (*ukp).data, (*ukp).datalen); if err == 0 { (*c).authenticated = true; (*c).log_hash = ubifs_hash_get_desc(c); } }
            }
        }
    }
    up_read(&mut (*keyring_key).sem); key_put(keyring_key); err
}

pub unsafe fn __ubifs_exit_authentication(c: *mut ubifs_info) { if ubifs_authenticated(c) { crypto_free_shash((*c).hmac_tfm); crypto_free_shash((*c).hash_tfm); kfree((*c).log_hash as *mut core::ffi::c_void); } }

unsafe fn ubifs_node_calc_hmac(c: *const ubifs_info, node: *const core::ffi::c_void, len: i32, ofs_hmac: i32, hmac: *mut core::ffi::c_void) -> i32 {
    let mut shash: shash_desc = core::mem::zeroed(); let hmac_len = (*c).hmac_desc_len;
    ubifs_assert(c, ofs_hmac > 8); ubifs_assert(c, ofs_hmac + hmac_len < len); shash.tfm = (*c).hmac_tfm;
    let mut err = crypto_shash_init(&mut shash); if err != 0 { return err; }
    err = crypto_shash_update(&mut shash, (node as *const u8).add(8), (ofs_hmac - 8) as usize); if err < 0 { return err; }
    if len - ofs_hmac - hmac_len > 0 { err = crypto_shash_update(&mut shash, (node as *const u8).add((ofs_hmac + hmac_len) as usize), (len - ofs_hmac - hmac_len) as usize); if err < 0 { return err; } }
    crypto_shash_final(&mut shash, hmac as *mut u8)
}

pub unsafe fn __ubifs_node_insert_hmac(c: *const ubifs_info, node: *mut core::ffi::c_void, len: i32, ofs_hmac: i32) -> i32 { ubifs_node_calc_hmac(c, node, len, ofs_hmac, (node as *mut u8).add(ofs_hmac as usize) as *mut core::ffi::c_void) }

pub unsafe fn __ubifs_node_verify_hmac(c: *const ubifs_info, node: *const core::ffi::c_void, len: i32, ofs_hmac: i32) -> i32 {
    let hmac_len = (*c).hmac_desc_len; let hmac = kmalloc(hmac_len as usize, GFP_NOFS) as *mut u8; if hmac.is_null() { return -ENOMEM; }
    let mut err = ubifs_node_calc_hmac(c, node, len, ofs_hmac, hmac as *mut core::ffi::c_void); if err == 0 { err = crypto_memneq(hmac, (node as *const u8).add(ofs_hmac as usize), hmac_len as usize); } kfree(hmac as *mut core::ffi::c_void); if err == 0 { 0 } else { -EPERM }
}

pub unsafe fn __ubifs_shash_copy_state(c: *const ubifs_info, src: *mut shash_desc, target: *mut shash_desc) -> i32 {
    let state = kmalloc(crypto_shash_descsize((*src).tfm), GFP_NOFS) as *mut u8; if state.is_null() { return -ENOMEM; }
    let mut err = crypto_shash_export(src, state); if err == 0 { err = crypto_shash_import(target, state); } kfree(state as *mut core::ffi::c_void); err
}

pub unsafe fn ubifs_hmac_wkm(c: *mut ubifs_info, hmac: *mut u8) -> i32 { if !ubifs_authenticated(c) { return 0; } let msg = b"UBIFS"; crypto_shash_tfm_digest((*c).hmac_tfm, msg.as_ptr(), msg.len(), hmac) }

pub unsafe fn ubifs_hmac_zero(c: *mut ubifs_info, hmac: *const u8) -> bool { memchr_inv(hmac, 0, (*c).hmac_desc_len as usize).is_null() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
