// SPDX-License-Identifier: GPL-2.0-or-later
/* System trusted keyring for trusted public keys
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Declarations supplied by the corresponding kernel headers are external dependencies.

static mut builtin_trusted_keys: *mut key = core::ptr::null_mut();
#[cfg(CONFIG_SECONDARY_TRUSTED_KEYRING)]
static mut secondary_trusted_keys: *mut key = core::ptr::null_mut();
#[cfg(CONFIG_INTEGRITY_MACHINE_KEYRING)]
static mut machine_trusted_keys: *mut key = core::ptr::null_mut();
#[cfg(CONFIG_INTEGRITY_PLATFORM_KEYRING)]
static mut platform_trusted_keys: *mut key = core::ptr::null_mut();

extern "C" {
    static system_certificate_list: *const u8;
    static system_certificate_list_size: usize;
    static module_cert_size: usize;
}

pub unsafe fn restrict_link_by_builtin_trusted(
    dest_keyring: *mut key, type_: *const key_type,
    payload: *const key_payload, _restriction_key: *mut key,
) -> c_int {
    restrict_link_by_signature(dest_keyring, type_, payload, builtin_trusted_keys)
}

pub unsafe fn restrict_link_by_digsig_builtin(
    dest_keyring: *mut key, type_: *const key_type,
    payload: *const key_payload, _restriction_key: *mut key,
) -> c_int {
    restrict_link_by_digsig(dest_keyring, type_, payload, builtin_trusted_keys)
}

#[cfg(CONFIG_SECONDARY_TRUSTED_KEYRING)]
pub unsafe fn restrict_link_by_builtin_and_secondary_trusted(
    dest_keyring: *mut key, type_: *const key_type,
    payload: *const key_payload, _restrict_key: *mut key,
) -> c_int {
    if type_ == &key_type_keyring as *const _ && dest_keyring == secondary_trusted_keys
        && payload == &(*builtin_trusted_keys).payload as *const _
    {
        return 0;
    }
    restrict_link_by_signature(dest_keyring, type_, payload, secondary_trusted_keys)
}

#[cfg(CONFIG_SECONDARY_TRUSTED_KEYRING)]
pub unsafe fn restrict_link_by_digsig_builtin_and_secondary(
    dest_keyring: *mut key, type_: *const key_type,
    payload: *const key_payload, _restrict_key: *mut key,
) -> c_int {
    if type_ == &key_type_keyring as *const _ && dest_keyring == secondary_trusted_keys
        && payload == &(*builtin_trusted_keys).payload as *const _
    {
        return 0;
    }
    restrict_link_by_digsig(dest_keyring, type_, payload, secondary_trusted_keys)
}

#[cfg(CONFIG_SECONDARY_TRUSTED_KEYRING)]
unsafe fn get_builtin_and_secondary_restriction() -> *mut key_restriction {
    let restriction = kzalloc_obj::<key_restriction>();
    if restriction.is_null() {
        panic!("Can't allocate secondary trusted keyring restriction\n");
    }
    if cfg!(CONFIG_INTEGRITY_MACHINE_KEYRING) {
        (*restriction).check = Some(restrict_link_by_builtin_secondary_and_machine);
    } else {
        (*restriction).check = Some(restrict_link_by_builtin_and_secondary_trusted);
    }
    restriction
}

#[cfg(CONFIG_SECONDARY_TRUSTED_KEYRING)]
pub unsafe fn add_to_secondary_keyring(source: *const c_char, data: *const c_void, len: usize) {
    let perm = (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW;
    let key_ref = key_create_or_update(
        make_key_ref(secondary_trusted_keys, 1), b"asymmetric\0".as_ptr() as *const c_char,
        core::ptr::null(), data, len, perm, KEY_ALLOC_NOT_IN_QUOTA,
    );
    if IS_ERR(key_ref) {
        pr_err!("Problem loading X.509 certificate from %s to secondary keyring %ld\n", source, PTR_ERR(key_ref));
        return;
    }
    pr_notice!("Loaded X.509 cert '%s'\n", (*key_ref_to_ptr(key_ref)).description);
    key_ref_put(key_ref);
}

#[cfg(CONFIG_INTEGRITY_MACHINE_KEYRING)]
pub unsafe fn set_machine_trusted_keys(keyring: *mut key) {
    machine_trusted_keys = keyring;
    if key_link(secondary_trusted_keys, machine_trusted_keys) < 0 {
        panic!("Can't link (machine) trusted keyrings\n");
    }
}

#[cfg(CONFIG_INTEGRITY_MACHINE_KEYRING)]
pub unsafe fn restrict_link_by_builtin_secondary_and_machine(
    dest_keyring: *mut key, type_: *const key_type,
    payload: *const key_payload, restrict_key: *mut key,
) -> c_int {
    if !machine_trusted_keys.is_null() && type_ == &key_type_keyring as *const _
        && dest_keyring == secondary_trusted_keys
        && payload == &(*machine_trusted_keys).payload as *const _
    { return 0; }
    restrict_link_by_builtin_and_secondary_trusted(dest_keyring, type_, payload, restrict_key)
}

unsafe fn system_trusted_keyring_init() -> c_int {
    pr_notice!("Initialise system trusted keyrings\n");
    builtin_trusted_keys = keyring_alloc(
        b".builtin_trusted_keys\0".as_ptr() as *const c_char,
        GLOBAL_ROOT_UID, GLOBAL_ROOT_GID, current_cred(),
        (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ | KEY_USR_SEARCH,
        KEY_ALLOC_NOT_IN_QUOTA, core::ptr::null_mut(), core::ptr::null_mut(),
    );
    if IS_ERR(builtin_trusted_keys) { panic!("Can't allocate builtin trusted keyring\n"); }
    #[cfg(CONFIG_SECONDARY_TRUSTED_KEYRING)]
    {
        secondary_trusted_keys = keyring_alloc(
            b".secondary_trusted_keys\0".as_ptr() as *const c_char,
            GLOBAL_ROOT_UID, GLOBAL_ROOT_GID, current_cred(),
            (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ | KEY_USR_SEARCH | KEY_USR_WRITE,
            KEY_ALLOC_NOT_IN_QUOTA, get_builtin_and_secondary_restriction(), core::ptr::null_mut(),
        );
        if IS_ERR(secondary_trusted_keys) { panic!("Can't allocate secondary trusted keyring\n"); }
        if key_link(secondary_trusted_keys, builtin_trusted_keys) < 0 { panic!("Can't link trusted keyrings\n"); }
    }
    0
}

// device_initcall(system_trusted_keyring_init)

pub unsafe fn load_module_cert(keyring: *mut key) -> c_int {
    if !cfg!(CONFIG_IMA_APPRAISE_MODSIG) { return 0; }
    pr_notice!("Loading compiled-in module X.509 certificates\n");
    x509_load_certificate_list(system_certificate_list, module_cert_size, keyring)
}

unsafe fn load_system_certificate_list() -> c_int {
    pr_notice!("Loading compiled-in X.509 certificates\n");
    #[cfg(CONFIG_MODULE_SIG)]
    let (p, size) = (system_certificate_list, system_certificate_list_size);
    #[cfg(not(CONFIG_MODULE_SIG))]
    let (p, size) = (system_certificate_list.add(module_cert_size), system_certificate_list_size - module_cert_size);
    x509_load_certificate_list(p, size, builtin_trusted_keys)
}

#[cfg(CONFIG_SYSTEM_DATA_VERIFICATION)]
pub unsafe fn verify_pkcs7_message_sig(
    mut data: *const c_void, mut len: usize, pkcs7: *mut pkcs7_message,
    mut trusted_keys: *mut key, usage: key_being_used_for,
    view_content: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize, usize) -> c_int>,
    ctx: *mut c_void,
) -> c_int {
    let mut ret: c_int;
    if !data.is_null() && pkcs7_supply_detached_data(pkcs7, data, len) < 0 { pr_err!("PKCS#7 signature with non-detached data\n"); return -EBADMSG; }
    ret = pkcs7_verify(pkcs7, usage); if ret < 0 { return ret; }
    ret = is_key_on_revocation_list(pkcs7); if ret != -ENOKEY { return ret; }
    if trusted_keys.is_null() { trusted_keys = builtin_trusted_keys; }
    else if trusted_keys == VERIFY_USE_SECONDARY_KEYRING { trusted_keys = builtin_trusted_keys; }
    else if trusted_keys == VERIFY_USE_PLATFORM_KEYRING { trusted_keys = platform_trusted_keys; if trusted_keys.is_null() { return -ENOKEY; } }
    ret = pkcs7_validate_trust(pkcs7, trusted_keys); if ret < 0 { return ret; }
    if let Some(view) = view_content {
        let mut asn1hdrlen = 0; ret = pkcs7_get_content_data(pkcs7, &mut data, &mut len, &mut asn1hdrlen);
        if ret < 0 { return ret; }
        ret = view(ctx, data, len, asn1hdrlen);
    }
    ret
}

#[cfg(CONFIG_SYSTEM_DATA_VERIFICATION)]
pub unsafe fn verify_pkcs7_signature(
    data: *const c_void, len: usize, raw_pkcs7: *const c_void, pkcs7_len: usize,
    trusted_keys: *mut key, usage: key_being_used_for,
    view_content: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize, usize) -> c_int>, ctx: *mut c_void,
) -> c_int {
    let pkcs7 = pkcs7_parse_message(raw_pkcs7, pkcs7_len);
    if IS_ERR(pkcs7) { return PTR_ERR(pkcs7); }
    let ret = verify_pkcs7_message_sig(data, len, pkcs7, trusted_keys, usage, view_content, ctx);
    pkcs7_free_message(pkcs7); ret
}

#[cfg(CONFIG_INTEGRITY_PLATFORM_KEYRING)]
pub unsafe fn set_platform_trusted_keys(keyring: *mut key) { platform_trusted_keys = keyring; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
