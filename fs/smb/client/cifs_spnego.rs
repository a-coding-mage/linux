// SPDX-License-Identifier: LGPL-2.1
/*
 *   SPNEGO upcall management for CIFS
 *
 *   Copyright (c) 2007 Red Hat, Inc.
 *   Author(s): Jeff Layton (jlayton@redhat.com)
 */

// Kernel headers and CIFS-local headers supply the declarations referenced below.

static mut SPNEGO_CRED: *const cred = core::ptr::null();

/* create a new cifs key */
unsafe extern "C" fn cifs_spnego_key_instantiate(
    key: *mut key,
    prep: *mut key_preparsed_payload,
) -> c_int {
    let payload = kmemdup((*prep).data, (*prep).datalen, GFP_KERNEL);

    if payload.is_null() {
        return -ENOMEM;
    }

    /* attach the data */
    (*key).payload.data[0] = payload;
    0
}

unsafe extern "C" fn cifs_spnego_key_destroy(key: *mut key) {
    kfree((*key).payload.data[0]);
}

unsafe extern "C" fn cifs_spnego_key_vet_description(description: *const c_char) -> c_int {
    /*
     * cifs.spnego descriptions are authority-bearing inputs to cifs.upcall.
     * They are only valid when produced by CIFS while using the private
     * spnego_cred installed below.  Do not let userspace create this type
     * of key through request_key(2)/add_key(2), since the helper treats
     * pid/uid/creduid/upcall_target as kernel-originating fields.
     */
    if current_cred() != SPNEGO_CRED {
        return -EPERM;
    }
    0
}

/* keytype for CIFS spnego keys */
#[no_mangle]
pub static mut cifs_spnego_key_type: key_type = key_type {
    name: c"cifs.spnego".as_ptr(),
    vet_description: Some(cifs_spnego_key_vet_description),
    instantiate: Some(cifs_spnego_key_instantiate),
    destroy: Some(cifs_spnego_key_destroy),
    describe: Some(user_describe),
};

/* length of longest version string e.g. strlen("ver=0xFF") */
const MAX_VER_STR_LEN: usize = 8;
/* length of longest security mechanism name */
const MAX_MECH_STR_LEN: usize = 13;
/* strlen of ";host=" */
const HOST_KEY_LEN: usize = 6;
/* strlen of ";ip4=" or ";ip6=" */
const IP_KEY_LEN: usize = 5;
/* strlen of ";uid=0x" */
const UID_KEY_LEN: usize = 7;
/* strlen of ";creduid=0x" */
const CREDUID_KEY_LEN: usize = 11;
/* strlen of ";user=" */
const USER_KEY_LEN: usize = 6;
/* strlen of ";pid=0x" */
const PID_KEY_LEN: usize = 7;
/* strlen of ";upcall_target=" */
const UPCALL_TARGET_KEY_LEN: usize = 15;

/* get a key struct with a SPNEGO security blob, suitable for session setup */
#[no_mangle]
pub unsafe extern "C" fn cifs_get_spnego_key(
    sesInfo: *mut cifs_ses,
    server: *mut TCP_Server_Info,
) -> *mut key {
    let sa = &mut (*server).dstaddr as *mut _ as *mut sockaddr_in;
    let sa6 = &mut (*server).dstaddr as *mut _ as *mut sockaddr_in6;
    let mut description: *mut c_char;
    let mut dp: *mut c_char;
    let mut desc_len: usize;
    let mut spnego_key: *mut key;
    let hostname = (*server).hostname;

    desc_len = MAX_VER_STR_LEN + HOST_KEY_LEN + strlen(hostname) + IP_KEY_LEN + INET6_ADDRSTRLEN
        + MAX_MECH_STR_LEN + UID_KEY_LEN + (core::mem::size_of::<uid_t>() * 2)
        + CREDUID_KEY_LEN + (core::mem::size_of::<uid_t>() * 2)
        + PID_KEY_LEN + (core::mem::size_of::<pid_t>() * 2) + 1;

    if !(*sesInfo).user_name.is_null() {
        desc_len += USER_KEY_LEN + strlen((*sesInfo).user_name);
    }
    if (*sesInfo).upcall_target == UPTARGET_MOUNT {
        desc_len += UPCALL_TARGET_KEY_LEN + 5;
    } else {
        desc_len += UPCALL_TARGET_KEY_LEN + 3;
    }

    spnego_key = ERR_PTR(-ENOMEM);
    description = kzalloc(desc_len, GFP_KERNEL) as *mut c_char;
    if description.is_null() {
        return spnego_key;
    }

    dp = description;
    spnego_key = ERR_PTR(-EINVAL);
    dp = dp.add(sprintf(dp, c"ver=0x%x;host=%s;", CIFS_SPNEGO_UPCALL_VERSION, hostname) as usize);

    if (*server).dstaddr.ss_family == AF_INET {
        dp = dp.add(sprintf(dp, c"ip4=%pI4", &(*sa).sin_addr) as usize);
    } else if (*server).dstaddr.ss_family == AF_INET6 {
        dp = dp.add(sprintf(dp, c"ip6=%pI6", &(*sa6).sin6_addr) as usize);
    } else {
        kfree(description);
        return spnego_key;
    }

    if (*server).sec_kerberos {
        dp = dp.add(sprintf(dp, c";sec=krb5") as usize);
    } else if (*server).sec_mskerberos {
        dp = dp.add(sprintf(dp, c";sec=mskrb5") as usize);
    } else if (*server).sec_iakerb {
        dp = dp.add(sprintf(dp, c";sec=iakerb") as usize);
    } else {
        cifs_dbg(VFS, c"unknown or missing server auth type, use krb5\n");
        dp = dp.add(sprintf(dp, c";sec=krb5") as usize);
    }

    dp = dp.add(sprintf(dp, c";uid=0x%x", from_kuid_munged(&init_user_ns, (*sesInfo).linux_uid)) as usize);
    dp = dp.add(sprintf(dp, c";creduid=0x%x", from_kuid_munged(&init_user_ns, (*sesInfo).cred_uid)) as usize);
    if !(*sesInfo).user_name.is_null() {
        dp = dp.add(sprintf(dp, c";user=%s", (*sesInfo).user_name) as usize);
    }
    dp = dp.add(sprintf(dp, c";pid=0x%x", (*current).pid) as usize);
    if (*sesInfo).upcall_target == UPTARGET_MOUNT {
        dp = dp.add(sprintf(dp, c";upcall_target=mount") as usize);
    } else {
        dp = dp.add(sprintf(dp, c";upcall_target=app") as usize);
    }

    cifs_dbg(FYI, c"key description = %s\n", description);
    scoped_with_creds(SPNEGO_CRED, {
        spnego_key = request_key(&cifs_spnego_key_type, description, c"");
    });
    trace_smb3_kerberos_auth(server, sesInfo, PTR_ERR_OR_ZERO(spnego_key));
    kfree(description);
    spnego_key
}

#[no_mangle]
pub unsafe extern "C" fn init_cifs_spnego() -> c_int {
    let cred = prepare_kernel_cred(&init_task);
    if cred.is_null() {
        return -ENOMEM;
    }
    let keyring = keyring_alloc(c".cifs_spnego", GLOBAL_ROOT_UID, GLOBAL_ROOT_GID, cred,
        (KEY_POS_ALL & !KEY_POS_SETATTR) | KEY_USR_VIEW | KEY_USR_READ,
        KEY_ALLOC_NOT_IN_QUOTA, core::ptr::null_mut(), core::ptr::null_mut());
    if IS_ERR(keyring) {
        let ret = PTR_ERR(keyring);
        put_cred(cred);
        return ret;
    }
    let ret = register_key_type(&mut cifs_spnego_key_type);
    if ret < 0 {
        key_put(keyring);
        put_cred(cred);
        return ret;
    }
    set_bit(KEY_FLAG_ROOT_CAN_CLEAR, &mut (*keyring).flags);
    (*cred).thread_keyring = keyring;
    (*cred).jit_keyring = KEY_REQKEY_DEFL_THREAD_KEYRING;
    SPNEGO_CRED = cred;
    cifs_dbg(FYI, c"cifs spnego keyring: %d\n", key_serial(keyring));
    0
}

#[no_mangle]
pub unsafe extern "C" fn exit_cifs_spnego() {
    key_revoke((*SPNEGO_CRED).thread_keyring);
    unregister_key_type(&mut cifs_spnego_key_type);
    put_cred(SPNEGO_CRED);
    cifs_dbg(FYI, c"Unregistered %s key type\n", cifs_spnego_key_type.name);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
