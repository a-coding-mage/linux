// SPDX-License-Identifier: GPL-2.0-or-later
/* System hash blacklist.
 *
 * Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_int, c_void};

// Kernel headers and symbols used by this implementation are supplied by the
// surrounding kernel translation.
const MAX_HASH_LEN: usize = 128;
const BLACKLIST_KEY_PERM: u32 = KEY_POS_SEARCH | KEY_POS_VIEW | KEY_USR_SEARCH | KEY_USR_VIEW;

static TBS_PREFIX: &[u8] = b"tbs\0";
static BIN_PREFIX: &[u8] = b"bin\0";

static mut blacklist_keyring: *mut key = core::ptr::null_mut();

#[cfg(CONFIG_SYSTEM_REVOCATION_LIST)]
extern "C" {
    static revocation_certificate_list: *const u8;
    static revocation_certificate_list_size: c_ulong;
}

unsafe fn blacklist_vet_description(mut desc: *const c_char) -> c_int {
    let mut i: usize = 0;
    let prefix_len = TBS_PREFIX.len() - 1;
    let mut tbs_step = 0;
    let mut bin_step = 0;

    // The following algorithm only works if prefix lengths match.
    debug_assert_eq!(TBS_PREFIX.len(), BIN_PREFIX.len());
    while *desc != 0 {
        if *desc == b':' as c_char {
            if tbs_step == prefix_len || bin_step == prefix_len {
                desc = desc.add(1);
                break;
            }
            return -EINVAL;
        }
        if i >= prefix_len { return -EINVAL; }
        if *desc == TBS_PREFIX[i] as c_char { tbs_step += 1; }
        if *desc == BIN_PREFIX[i] as c_char { bin_step += 1; }
        desc = desc.add(1);
        i += 1;
    }
    if *desc == 0 && i == 0 { return -EINVAL; }
    if i != prefix_len && *desc == 0 { return -EINVAL; }

    i = 0;
    while *desc != 0 && i < MAX_HASH_LEN {
        if !isxdigit(*desc as u8) || isupper(*desc as u8) { return -EINVAL; }
        desc = desc.add(1);
        i += 1;
    }
    if *desc != 0 { return -ENOPKG; }
    if i == 0 || (i & 1) != 0 { return -EINVAL; }
    0
}

unsafe fn blacklist_key_instantiate(key: *mut key, prep: *mut key_preparsed_payload) -> c_int {
    (*key).perm = BLACKLIST_KEY_PERM;
    if (*key).flags & (1 << KEY_FLAG_BUILTIN) == 0 {
        #[cfg(CONFIG_SYSTEM_BLACKLIST_AUTH_UPDATE)]
        {
            let err = verify_pkcs7_signature((*key).description, strlen((*key).description),
                (*prep).data, (*prep).datalen, core::ptr::null_mut(),
                VERIFYING_UNSPECIFIED_SIGNATURE, core::ptr::null_mut(), core::ptr::null_mut());
            if err != 0 { return err; }
        }
        #[cfg(not(CONFIG_SYSTEM_BLACKLIST_AUTH_UPDATE))]
        { WARN_ON_ONCE(1); return -EPERM; }
    }
    generic_key_instantiate(key, prep)
}

unsafe fn blacklist_key_update(_key: *mut key, _prep: *mut key_preparsed_payload) -> c_int { -EPERM }

unsafe fn blacklist_describe(key: *const key, m: *mut seq_file) {
    seq_puts(m, (*key).description);
}

static mut key_type_blacklist: key_type = key_type {
    name: b"blacklist\0".as_ptr() as *const c_char,
    vet_description: Some(blacklist_vet_description),
    instantiate: Some(blacklist_key_instantiate),
    update: Some(blacklist_key_update),
    describe: Some(blacklist_describe),
};

unsafe fn get_raw_hash(hash: *const u8, hash_len: usize, hash_type: blacklist_hash_type) -> *mut c_char {
    let (type_prefix, type_len) = match hash_type {
        BLACKLIST_HASH_X509_TBS => (TBS_PREFIX.as_ptr(), TBS_PREFIX.len() - 1),
        BLACKLIST_HASH_BINARY => (BIN_PREFIX.as_ptr(), BIN_PREFIX.len() - 1),
        _ => { WARN_ON_ONCE(1); return ERR_PTR(-EINVAL); }
    };
    let buffer = kmalloc(type_len + 1 + hash_len * 2 + 1, GFP_KERNEL) as *mut c_char;
    if buffer.is_null() { return ERR_PTR(-ENOMEM); }
    memcpy(buffer as *mut c_void, type_prefix as *const c_void, type_len);
    *buffer.add(type_len) = b':' as c_char;
    bin2hex(buffer.add(type_len + 1), hash, hash_len);
    *buffer.add(type_len + 1 + hash_len * 2) = 0;
    buffer
}

unsafe fn mark_raw_hash_blacklisted(hash: *const c_char) -> c_int {
    let key = key_create(make_key_ref(blacklist_keyring, true), b"blacklist\0".as_ptr() as _, hash,
        core::ptr::null_mut(), 0, BLACKLIST_KEY_PERM,
        KEY_ALLOC_NOT_IN_QUOTA | KEY_ALLOC_BUILT_IN);
    if IS_ERR(key) {
        if PTR_ERR(key) == -EEXIST { pr_warn!("Duplicate blacklisted hash %s\n", hash); }
        else { pr_err!("Problem blacklisting hash %s: %pe\n", hash, key); }
        return PTR_ERR(key);
    }
    0
}

pub unsafe fn mark_hash_blacklisted(hash: *const u8, hash_len: usize, hash_type: blacklist_hash_type) -> c_int {
    let buffer = get_raw_hash(hash, hash_len, hash_type);
    if IS_ERR(buffer) { return PTR_ERR(buffer); }
    let err = mark_raw_hash_blacklisted(buffer);
    kfree(buffer as *mut c_void);
    err
}

pub unsafe fn is_hash_blacklisted(hash: *const u8, hash_len: usize, hash_type: blacklist_hash_type) -> c_int {
    let buffer = get_raw_hash(hash, hash_len, hash_type);
    if IS_ERR(buffer) { return PTR_ERR(buffer); }
    let kref = keyring_search(make_key_ref(blacklist_keyring, true), &key_type_blacklist, buffer, false);
    let mut ret = 0;
    if !IS_ERR(kref) { key_ref_put(kref); ret = -EKEYREJECTED; }
    kfree(buffer as *mut c_void);
    ret
}

pub unsafe fn is_binary_blacklisted(hash: *const u8, hash_len: usize) -> c_int {
    if is_hash_blacklisted(hash, hash_len, BLACKLIST_HASH_BINARY) == -EKEYREJECTED { -EPERM } else { 0 }
}

#[cfg(CONFIG_SYSTEM_REVOCATION_LIST)]
pub unsafe fn add_key_to_revocation_list(data: *const c_char, size: usize) -> c_int {
    let key = key_create_or_update(make_key_ref(blacklist_keyring, true), b"asymmetric\0".as_ptr() as _,
        core::ptr::null(), data, size,
        KEY_POS_VIEW | KEY_POS_READ | KEY_POS_SEARCH | KEY_USR_VIEW,
        KEY_ALLOC_NOT_IN_QUOTA | KEY_ALLOC_BUILT_IN | KEY_ALLOC_BYPASS_RESTRICTION);
    if IS_ERR(key) { pr_err!("Problem with revocation key (%ld)\n", PTR_ERR(key)); return PTR_ERR(key); }
    0
}

#[cfg(CONFIG_SYSTEM_REVOCATION_LIST)]
pub unsafe fn is_key_on_revocation_list(pkcs7: *mut pkcs7_message) -> c_int {
    let ret = pkcs7_validate_trust(pkcs7, blacklist_keyring);
    if ret == 0 { -EKEYREJECTED } else { -ENOKEY }
}

unsafe fn restrict_link_for_blacklist(_dest_keyring: *mut key, type_: *const key_type,
    _payload: *const union_key_payload, _restrict_key: *mut key) -> c_int {
    if type_ == &key_type_blacklist { 0 } else { -EOPNOTSUPP }
}

unsafe fn blacklist_init() -> c_int {
    if register_key_type(&mut key_type_blacklist) < 0 { panic!("Can't allocate system blacklist key type\n"); }
    let restriction = kzalloc_obj::<key_restriction>();
    if restriction.is_null() { panic!("Can't allocate blacklist keyring restriction\n"); }
    (*restriction).check = Some(restrict_link_for_blacklist);
    blacklist_keyring = keyring_alloc(b".blacklist\0".as_ptr() as _, GLOBAL_ROOT_UID, GLOBAL_ROOT_GID,
        current_cred(), KEY_POS_VIEW | KEY_POS_READ | KEY_POS_SEARCH | KEY_POS_WRITE |
        KEY_USR_VIEW | KEY_USR_READ | KEY_USR_SEARCH
        #[cfg(CONFIG_SYSTEM_BLACKLIST_AUTH_UPDATE)]
        | KEY_USR_WRITE,
        KEY_ALLOC_NOT_IN_QUOTA | KEY_ALLOC_SET_KEEP, restriction, core::ptr::null_mut());
    if IS_ERR(blacklist_keyring) { panic!("Can't allocate system blacklist keyring\n"); }
    let mut bl = blacklist_hashes;
    while !(*bl).is_null() {
        if mark_raw_hash_blacklisted(*bl) < 0 { pr_err!("- blacklisting failed\n"); }
        bl = bl.add(1);
    }
    0
}

// Registered through device_initcall(blacklist_init).

#[cfg(CONFIG_SYSTEM_REVOCATION_LIST)]
unsafe fn load_revocation_certificate_list() -> c_int {
    if revocation_certificate_list_size != 0 {
        pr_notice!("Loading compiled-in revocation X.509 certificates\n");
    }
    x509_load_certificate_list(revocation_certificate_list, revocation_certificate_list_size, blacklist_keyring)
}

// Registered through late_initcall(load_revocation_certificate_list).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
