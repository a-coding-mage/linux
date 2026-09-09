// SPDX-License-Identifier: GPL-2.0-or-later
/* Instantiate a public key crypto key from an X.509 Certificate
 *
 * Copyright (C) 2012, 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

static mut USE_BUILTIN_KEYS: bool = false;
static mut CA_KEYID: *mut asymmetric_key_id = core::ptr::null_mut();

#[cfg(not(feature = "module"))]
#[repr(C)]
struct CaKey {
    id: asymmetric_key_id,
    data: [u8; 10],
}

#[cfg(not(feature = "module"))]
static mut CAKEY: CaKey = CaKey {
    id: asymmetric_key_id {},
    data: [0; 10],
};

#[cfg(not(feature = "module"))]
unsafe fn ca_keys_setup(str_: *mut core::ffi::c_char) -> i32 {
    if str_.is_null() {
        // default system keyring
        return 1;
    }

    if libc::strncmp(str_, b"id:\0".as_ptr() as *const _, 3) == 0 {
        let p: *mut asymmetric_key_id = &raw mut CAKEY.id;
        let hexlen = (libc::strlen(str_) - 3) / 2;
        let ret: i32;

        if hexlen == 0 || hexlen > core::mem::size_of_val(&CAKEY.data) {
            pr_err!("Missing or invalid ca_keys id\n");
            return 1;
        }

        ret = __asymmetric_key_hex_to_key_id(str_.add(3), p, hexlen);
        if ret < 0 {
            pr_err!("Unparsable ca_keys id hex string\n");
        } else {
            CA_KEYID = p; // owner key 'id:xxxxxx'
        }
    } else if libc::strcmp(str_, b"builtin\0".as_ptr() as *const _) == 0 {
        USE_BUILTIN_KEYS = true;
    }

    1
}

/**
 * restrict_link_by_signature - Restrict additions to a ring of public keys
 * @dest_keyring: Keyring being linked to.
 * @type: The type of key being added.
 * @payload: The payload of the new key.
 * @trust_keyring: A ring of keys that can be used to vouch for the new cert.
 */
pub unsafe fn restrict_link_by_signature(
    dest_keyring: *mut key,
    type_: *const key_type,
    payload: *const key_payload,
    trust_keyring: *mut key,
) -> i32 {
    let sig: *const public_key_signature;
    let key: *mut key;
    let ret: i32;

    pr_devel!("==>{}()\n", "restrict_link_by_signature");

    if trust_keyring.is_null() { return -ENOKEY; }
    if type_ != &raw const key_type_asymmetric { return -EOPNOTSUPP; }

    sig = (*payload).data[asym_auth];
    if sig.is_null() { return -ENOPKG; }
    if (*sig).auth_ids[0].is_null() && (*sig).auth_ids[1].is_null() && (*sig).auth_ids[2].is_null() { return -ENOKEY; }

    if !CA_KEYID.is_null() && !asymmetric_key_id_partial((*sig).auth_ids[1], CA_KEYID) { return -EPERM; }

    key = find_asymmetric_key(trust_keyring, (*sig).auth_ids[0], (*sig).auth_ids[1], (*sig).auth_ids[2], false);
    if is_err(key) { return -ENOKEY; }

    if USE_BUILTIN_KEYS && !test_bit(KEY_FLAG_BUILTIN, &(*key).flags) {
        ret = -ENOKEY;
    } else if is_builtin(CONFIG_SECONDARY_TRUSTED_KEYRING_SIGNED_BY_BUILTIN) &&
              libc::strcmp((*dest_keyring).description, b".secondary_trusted_keys\0".as_ptr() as *const _) == 0 &&
              !test_bit(KEY_FLAG_BUILTIN, &(*key).flags) {
        ret = -ENOKEY;
    } else {
        ret = verify_signature(key, sig);
    }
    key_put(key);
    ret
}

pub unsafe fn restrict_link_by_ca(dest_keyring: *mut key, type_: *const key_type, payload: *const key_payload, trust_keyring: *mut key) -> i32 {
    let pkey: *const public_key;
    if type_ != &raw const key_type_asymmetric { return -EOPNOTSUPP; }
    pkey = (*payload).data[asym_crypto];
    if pkey.is_null() { return -ENOPKG; }
    if !test_bit(KEY_EFLAG_CA, &(*pkey).key_eflags) { return -ENOKEY; }
    if !test_bit(KEY_EFLAG_KEYCERTSIGN, &(*pkey).key_eflags) { return -ENOKEY; }
    if !is_enabled(CONFIG_INTEGRITY_CA_MACHINE_KEYRING_MAX) { return 0; }
    if test_bit(KEY_EFLAG_DIGITALSIG, &(*pkey).key_eflags) { return -ENOKEY; }
    0
}

pub unsafe fn restrict_link_by_digsig(dest_keyring: *mut key, type_: *const key_type, payload: *const key_payload, trust_keyring: *mut key) -> i32 {
    let pkey: *const public_key;
    if type_ != &raw const key_type_asymmetric { return -EOPNOTSUPP; }
    pkey = (*payload).data[asym_crypto];
    if pkey.is_null() { return -ENOPKG; }
    if !test_bit(KEY_EFLAG_DIGITALSIG, &(*pkey).key_eflags) { return -ENOKEY; }
    if test_bit(KEY_EFLAG_CA, &(*pkey).key_eflags) { return -ENOKEY; }
    if test_bit(KEY_EFLAG_KEYCERTSIGN, &(*pkey).key_eflags) { return -ENOKEY; }
    restrict_link_by_signature(dest_keyring, type_, payload, trust_keyring)
}

unsafe fn match_either_id(pair: *const *const asymmetric_key_id, single: *const asymmetric_key_id) -> bool {
    asymmetric_key_id_same(*pair, single) || asymmetric_key_id_same(*pair.add(1), single)
}

unsafe fn key_or_keyring_common(dest_keyring: *mut key, type_: *const key_type, payload: *const key_payload, trusted: *mut key, check_dest: bool) -> i32 {
    let sig: *const public_key_signature;
    let mut key: *mut key = core::ptr::null_mut();
    let ret: i32;
    pr_devel!("==>{}()\n", "key_or_keyring_common");
    if dest_keyring.is_null() { return -ENOKEY; }
    if (*dest_keyring).type_ != &raw const key_type_keyring { return -EOPNOTSUPP; }
    if trusted.is_null() && !check_dest { return -ENOKEY; }
    if type_ != &raw const key_type_asymmetric { return -EOPNOTSUPP; }
    sig = (*payload).data[asym_auth];
    if sig.is_null() { return -ENOPKG; }
    if (*sig).auth_ids[0].is_null() && (*sig).auth_ids[1].is_null() && (*sig).auth_ids[2].is_null() { return -ENOKEY; }
    if !trusted.is_null() {
        if (*trusted).type_ == &raw const key_type_keyring {
            key = find_asymmetric_key(trusted, (*sig).auth_ids[0], (*sig).auth_ids[1], (*sig).auth_ids[2], false);
            if is_err(key) { key = core::ptr::null_mut(); }
        } else if (*trusted).type_ == &raw const key_type_asymmetric {
            let signer_ids = asymmetric_key_ids(trusted).id as *const *const asymmetric_key_id;
            if (*sig).auth_ids[0].is_null() && (*sig).auth_ids[1].is_null() {
                if asymmetric_key_id_same(*signer_ids.add(2), (*sig).auth_ids[2]) { key = __key_get(trusted); }
            } else if (*sig).auth_ids[0].is_null() || (*sig).auth_ids[1].is_null() {
                let auth_id = if !(*sig).auth_ids[0].is_null() { (*sig).auth_ids[0] } else { (*sig).auth_ids[1] };
                if match_either_id(signer_ids, auth_id) { key = __key_get(trusted); }
            } else if asymmetric_key_id_same(*signer_ids.add(1), (*sig).auth_ids[1]) && match_either_id(signer_ids, (*sig).auth_ids[0]) { key = __key_get(trusted); }
        } else { return -EOPNOTSUPP; }
    }
    if check_dest && key.is_null() {
        key = find_asymmetric_key(dest_keyring, (*sig).auth_ids[0], (*sig).auth_ids[1], (*sig).auth_ids[2], false);
        if is_err(key) { key = core::ptr::null_mut(); }
    }
    if key.is_null() { return -ENOKEY; }
    ret = key_validate(key);
    let ret = if ret == 0 { verify_signature(key, sig) } else { ret };
    key_put(key);
    ret
}

pub unsafe fn restrict_link_by_key_or_keyring(dest_keyring: *mut key, type_: *const key_type, payload: *const key_payload, trusted: *mut key) -> i32 {
    key_or_keyring_common(dest_keyring, type_, payload, trusted, false)
}

pub unsafe fn restrict_link_by_key_or_keyring_chain(dest_keyring: *mut key, type_: *const key_type, payload: *const key_payload, trusted: *mut key) -> i32 {
    key_or_keyring_common(dest_keyring, type_, payload, trusted, true)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
