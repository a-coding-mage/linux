// SPDX-License-Identifier: GPL-2.0-or-later
/* 32-bit compatibility syscall for 64-bit systems
 *
 * Copyright (C) 2004-5 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies from:
// <linux/syscalls.h>, <linux/keyctl.h>, <linux/compat.h>,
// <linux/slab.h>, and "internal.h".

use core::ffi::c_void;

pub type u32 = core::ffi::c_uint;
pub type c_long = core::ffi::c_long;

unsafe extern "C" {
    static KEYCTL_GET_KEYRING_ID: u32;
    static KEYCTL_JOIN_SESSION_KEYRING: u32;
    static KEYCTL_UPDATE: u32;
    static KEYCTL_REVOKE: u32;
    static KEYCTL_DESCRIBE: u32;
    static KEYCTL_CLEAR: u32;
    static KEYCTL_LINK: u32;
    static KEYCTL_UNLINK: u32;
    static KEYCTL_SEARCH: u32;
    static KEYCTL_READ: u32;
    static KEYCTL_CHOWN: u32;
    static KEYCTL_SETPERM: u32;
    static KEYCTL_INSTANTIATE: u32;
    static KEYCTL_NEGATE: u32;
    static KEYCTL_SET_REQKEY_KEYRING: u32;
    static KEYCTL_SET_TIMEOUT: u32;
    static KEYCTL_ASSUME_AUTHORITY: u32;
    static KEYCTL_GET_SECURITY: u32;
    static KEYCTL_SESSION_TO_PARENT: u32;
    static KEYCTL_REJECT: u32;
    static KEYCTL_INSTANTIATE_IOV: u32;
    static KEYCTL_INVALIDATE: u32;
    static KEYCTL_GET_PERSISTENT: u32;
    static KEYCTL_DH_COMPUTE: u32;
    static KEYCTL_RESTRICT_KEYRING: u32;
    static KEYCTL_PKEY_QUERY: u32;
    static KEYCTL_PKEY_ENCRYPT: u32;
    static KEYCTL_PKEY_DECRYPT: u32;
    static KEYCTL_PKEY_SIGN: u32;
    static KEYCTL_PKEY_VERIFY: u32;
    static KEYCTL_MOVE: u32;
    static KEYCTL_CAPABILITIES: u32;
    static KEYCTL_WATCH_KEY: u32;

    static EINVAL: core::ffi::c_int;
    static EOPNOTSUPP: core::ffi::c_int;

    fn compat_ptr(arg: u32) -> *mut c_void;

    fn keyctl_get_keyring_ID(arg2: u32, arg3: u32) -> c_long;
    fn keyctl_join_session_keyring(arg2: *mut c_void) -> c_long;
    fn keyctl_update_key(arg2: u32, arg3: *mut c_void, arg4: u32) -> c_long;
    fn keyctl_revoke_key(arg2: u32) -> c_long;
    fn keyctl_describe_key(arg2: u32, arg3: *mut c_void, arg4: u32) -> c_long;
    fn keyctl_keyring_clear(arg2: u32) -> c_long;
    fn keyctl_keyring_link(arg2: u32, arg3: u32) -> c_long;
    fn keyctl_keyring_unlink(arg2: u32, arg3: u32) -> c_long;
    fn keyctl_keyring_search(
        arg2: u32,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: u32,
    ) -> c_long;
    fn keyctl_read_key(arg2: u32, arg3: *mut c_void, arg4: u32) -> c_long;
    fn keyctl_chown_key(arg2: u32, arg3: u32, arg4: u32) -> c_long;
    fn keyctl_setperm_key(arg2: u32, arg3: u32) -> c_long;
    fn keyctl_instantiate_key(arg2: u32, arg3: *mut c_void, arg4: u32, arg5: u32) -> c_long;
    fn keyctl_negate_key(arg2: u32, arg3: u32, arg4: u32) -> c_long;
    fn keyctl_set_reqkey_keyring(arg2: u32) -> c_long;
    fn keyctl_set_timeout(arg2: u32, arg3: u32) -> c_long;
    fn keyctl_assume_authority(arg2: u32) -> c_long;
    fn keyctl_get_security(arg2: u32, arg3: *mut c_void, arg4: u32) -> c_long;
    fn keyctl_session_to_parent() -> c_long;
    fn keyctl_reject_key(arg2: u32, arg3: u32, arg4: u32, arg5: u32) -> c_long;
    fn keyctl_instantiate_key_iov(
        arg2: u32,
        arg3: *mut c_void,
        arg4: u32,
        arg5: u32,
    ) -> c_long;
    fn keyctl_invalidate_key(arg2: u32) -> c_long;
    fn keyctl_get_persistent(arg2: u32, arg3: u32) -> c_long;
    fn compat_keyctl_dh_compute(
        arg2: *mut c_void,
        arg3: *mut c_void,
        arg4: u32,
        arg5: *mut c_void,
    ) -> c_long;
    fn keyctl_restrict_keyring(arg2: u32, arg3: *mut c_void, arg4: *mut c_void) -> c_long;
    fn keyctl_pkey_query(arg2: u32, arg4: *mut c_void, arg5: *mut c_void) -> c_long;
    fn keyctl_pkey_e_d_s(
        option: u32,
        arg2: *mut c_void,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: *mut c_void,
    ) -> c_long;
    fn keyctl_pkey_verify(
        arg2: *mut c_void,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: *mut c_void,
    ) -> c_long;
    fn keyctl_keyring_move(arg2: u32, arg3: u32, arg4: u32, arg5: u32) -> c_long;
    fn keyctl_capabilities(arg2: *mut c_void, arg3: u32) -> c_long;
    fn keyctl_watch_key(arg2: u32, arg3: u32, arg4: u32) -> c_long;
}

/*
 * The key control system call, 32-bit compatibility version for 64-bit archs
 */
#[no_mangle]
pub unsafe extern "C" fn compat_sys_keyctl(
    option: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
) -> c_long {
    if option == KEYCTL_GET_KEYRING_ID {
        return keyctl_get_keyring_ID(arg2, arg3);
    }
    if option == KEYCTL_JOIN_SESSION_KEYRING {
        return keyctl_join_session_keyring(compat_ptr(arg2));
    }
    if option == KEYCTL_UPDATE {
        return keyctl_update_key(arg2, compat_ptr(arg3), arg4);
    }
    if option == KEYCTL_REVOKE {
        return keyctl_revoke_key(arg2);
    }
    if option == KEYCTL_DESCRIBE {
        return keyctl_describe_key(arg2, compat_ptr(arg3), arg4);
    }
    if option == KEYCTL_CLEAR {
        return keyctl_keyring_clear(arg2);
    }
    if option == KEYCTL_LINK {
        return keyctl_keyring_link(arg2, arg3);
    }
    if option == KEYCTL_UNLINK {
        return keyctl_keyring_unlink(arg2, arg3);
    }
    if option == KEYCTL_SEARCH {
        return keyctl_keyring_search(arg2, compat_ptr(arg3), compat_ptr(arg4), arg5);
    }
    if option == KEYCTL_READ {
        return keyctl_read_key(arg2, compat_ptr(arg3), arg4);
    }
    if option == KEYCTL_CHOWN {
        return keyctl_chown_key(arg2, arg3, arg4);
    }
    if option == KEYCTL_SETPERM {
        return keyctl_setperm_key(arg2, arg3);
    }
    if option == KEYCTL_INSTANTIATE {
        return keyctl_instantiate_key(arg2, compat_ptr(arg3), arg4, arg5);
    }
    if option == KEYCTL_NEGATE {
        return keyctl_negate_key(arg2, arg3, arg4);
    }
    if option == KEYCTL_SET_REQKEY_KEYRING {
        return keyctl_set_reqkey_keyring(arg2);
    }
    if option == KEYCTL_SET_TIMEOUT {
        return keyctl_set_timeout(arg2, arg3);
    }
    if option == KEYCTL_ASSUME_AUTHORITY {
        return keyctl_assume_authority(arg2);
    }
    if option == KEYCTL_GET_SECURITY {
        return keyctl_get_security(arg2, compat_ptr(arg3), arg4);
    }
    if option == KEYCTL_SESSION_TO_PARENT {
        return keyctl_session_to_parent();
    }
    if option == KEYCTL_REJECT {
        return keyctl_reject_key(arg2, arg3, arg4, arg5);
    }
    if option == KEYCTL_INSTANTIATE_IOV {
        return keyctl_instantiate_key_iov(arg2, compat_ptr(arg3), arg4, arg5);
    }
    if option == KEYCTL_INVALIDATE {
        return keyctl_invalidate_key(arg2);
    }
    if option == KEYCTL_GET_PERSISTENT {
        return keyctl_get_persistent(arg2, arg3);
    }
    if option == KEYCTL_DH_COMPUTE {
        return compat_keyctl_dh_compute(
            compat_ptr(arg2),
            compat_ptr(arg3),
            arg4,
            compat_ptr(arg5),
        );
    }
    if option == KEYCTL_RESTRICT_KEYRING {
        return keyctl_restrict_keyring(arg2, compat_ptr(arg3), compat_ptr(arg4));
    }
    if option == KEYCTL_PKEY_QUERY {
        if arg3 != 0 {
            return -(EINVAL as c_long);
        }
        return keyctl_pkey_query(arg2, compat_ptr(arg4), compat_ptr(arg5));
    }
    if option == KEYCTL_PKEY_ENCRYPT || option == KEYCTL_PKEY_DECRYPT || option == KEYCTL_PKEY_SIGN {
        return keyctl_pkey_e_d_s(
            option,
            compat_ptr(arg2),
            compat_ptr(arg3),
            compat_ptr(arg4),
            compat_ptr(arg5),
        );
    }
    if option == KEYCTL_PKEY_VERIFY {
        return keyctl_pkey_verify(
            compat_ptr(arg2),
            compat_ptr(arg3),
            compat_ptr(arg4),
            compat_ptr(arg5),
        );
    }
    if option == KEYCTL_MOVE {
        return keyctl_keyring_move(arg2, arg3, arg4, arg5);
    }
    if option == KEYCTL_CAPABILITIES {
        return keyctl_capabilities(compat_ptr(arg2), arg3);
    }
    if option == KEYCTL_WATCH_KEY {
        return keyctl_watch_key(arg2, arg3, arg4);
    }

    -(EOPNOTSUPP as c_long)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
