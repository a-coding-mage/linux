/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Authentication token and access key management internal defs
 *
 * Copyright (C) 2003-5, 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* C dependencies omitted from executable Rust:
 * linux/sched.h, linux/wait_bit.h, linux/cred.h, linux/key-type.h,
 * linux/task_work.h, linux/keyctl.h, linux/refcount.h,
 * linux/watch_queue.h, linux/compat.h, linux/mm.h, linux/vmalloc.h.
 */

#[cfg(__KDEBUG)]
macro_rules! kenter {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        printk!(KERN_DEBUG, concat!("==> %s(", $fmt, ")\n"), __func__ $(, $arg)*)
    };
}

#[cfg(__KDEBUG)]
macro_rules! kleave {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        printk!(KERN_DEBUG, concat!("<== %s()", $fmt, "\n"), __func__ $(, $arg)*)
    };
}

#[cfg(__KDEBUG)]
macro_rules! kdebug {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        printk!(KERN_DEBUG, concat!("   ", $fmt, "\n") $(, $arg)*)
    };
}

#[cfg(not(__KDEBUG))]
macro_rules! kenter {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        no_printk!(KERN_DEBUG, concat!("==> %s(", $fmt, ")\n"), __func__ $(, $arg)*)
    };
}

#[cfg(not(__KDEBUG))]
macro_rules! kleave {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        no_printk!(KERN_DEBUG, concat!("<== %s()", $fmt, "\n"), __func__ $(, $arg)*)
    };
}

#[cfg(not(__KDEBUG))]
macro_rules! kdebug {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        no_printk!(KERN_DEBUG, concat!($fmt, "\n") $(, $arg)*)
    };
}

unsafe extern "C" {
    pub static mut key_type_dead: key_type;
    pub static mut key_type_user: key_type;
    pub static mut key_type_logon: key_type;
}

/*****************************************************************************/
/*
 * Keep track of keys for a user.
 *
 * This needs to be separate to user_struct to avoid a refcount-loop
 * (user_struct pins some keyrings which pin this struct).
 *
 * We also keep track of keys under request from userspace for this UID here.
 */
#[repr(C)]
pub struct key_user {
    pub node: rb_node,
    pub cons_lock: mutex,     /* construction initiation lock */
    pub lock: spinlock_t,
    pub usage: refcount_t,    /* for accessing qnkeys & qnbytes */
    pub nkeys: atomic_t,      /* number of keys */
    pub nikeys: atomic_t,     /* number of instantiated keys */
    pub uid: kuid_t,
    pub qnkeys: core::ffi::c_int,  /* number of keys allocated to this user */
    pub qnbytes: core::ffi::c_int, /* number of bytes allocated to this user */
}

unsafe extern "C" {
    pub static mut key_user_tree: rb_root;
    pub static mut key_user_lock: spinlock_t;
    pub static mut root_key_user: key_user;

    pub fn key_user_lookup(uid: kuid_t) -> *mut key_user;
    pub fn key_user_put(user: *mut key_user);
}

/*
 * Key quota limits.
 * - root has its own separate limits to everyone else
 */
unsafe extern "C" {
    pub static mut key_quota_root_maxkeys: core::ffi::c_uint;
    pub static mut key_quota_root_maxbytes: core::ffi::c_uint;
    pub static mut key_quota_maxkeys: core::ffi::c_uint;
    pub static mut key_quota_maxbytes: core::ffi::c_uint;
}

pub const KEYQUOTA_LINK_BYTES: core::ffi::c_int = 4; /* a link in a keyring is worth 4 bytes */

unsafe extern "C" {
    pub static mut key_jar: *mut kmem_cache;
    pub static mut key_serial_tree: rb_root;
    pub static mut key_serial_lock: spinlock_t;
    pub static mut key_construction_mutex: mutex;
    pub static mut request_key_conswq: wait_queue_head_t;

    pub fn key_set_index_key(index_key: *mut keyring_index_key);
    pub fn key_type_lookup(type_: *const core::ffi::c_char) -> *mut key_type;
    pub fn key_type_put(ktype: *mut key_type);

    pub fn __key_link_lock(
        keyring: *mut key,
        index_key: *const keyring_index_key,
    ) -> core::ffi::c_int;
    pub fn __key_move_lock(
        l_keyring: *mut key,
        u_keyring: *mut key,
        index_key: *const keyring_index_key,
    ) -> core::ffi::c_int;
    pub fn __key_link_begin(
        keyring: *mut key,
        index_key: *const keyring_index_key,
        _edit: *mut *mut assoc_array_edit,
    ) -> core::ffi::c_int;
    pub fn __key_link_check_live_key(keyring: *mut key, key: *mut key) -> core::ffi::c_int;
    pub fn __key_link(keyring: *mut key, key: *mut key, _edit: *mut *mut assoc_array_edit);
    pub fn __key_link_end(
        keyring: *mut key,
        index_key: *const keyring_index_key,
        edit: *mut assoc_array_edit,
    );

    pub fn find_key_to_update(
        keyring_ref: key_ref_t,
        index_key: *const keyring_index_key,
    ) -> key_ref_t;
}

#[repr(C)]
pub struct keyring_search_context {
    pub index_key: keyring_index_key,
    pub cred: *const cred,
    pub match_data: key_match_data,
    pub flags: core::ffi::c_uint,

    pub iterator: Option<
        unsafe extern "C" fn(
            object: *const core::ffi::c_void,
            iterator_data: *mut core::ffi::c_void,
        ) -> core::ffi::c_int,
    >,

    /* Internal stuff */
    pub skipped_ret: core::ffi::c_int,
    pub possessed: bool,
    pub result: key_ref_t,
    pub now: time64_t,
}

pub const KEYRING_SEARCH_NO_STATE_CHECK: core::ffi::c_uint = 0x0001; /* Skip state checks */
pub const KEYRING_SEARCH_DO_STATE_CHECK: core::ffi::c_uint = 0x0002; /* Override NO_STATE_CHECK */
pub const KEYRING_SEARCH_NO_UPDATE_TIME: core::ffi::c_uint = 0x0004; /* Don't update times */
pub const KEYRING_SEARCH_NO_CHECK_PERM: core::ffi::c_uint = 0x0008; /* Don't check permissions */
pub const KEYRING_SEARCH_DETECT_TOO_DEEP: core::ffi::c_uint = 0x0010; /* Give an error on excessive depth */
pub const KEYRING_SEARCH_SKIP_EXPIRED: core::ffi::c_uint = 0x0020; /* Ignore expired keys (intention to replace) */
pub const KEYRING_SEARCH_RECURSE: core::ffi::c_uint = 0x0040; /* Search child keyrings also */

unsafe extern "C" {
    pub fn key_default_cmp(key: *const key, match_data: *const key_match_data) -> bool;
    pub fn keyring_search_rcu(
        keyring_ref: key_ref_t,
        ctx: *mut keyring_search_context,
    ) -> key_ref_t;

    pub fn search_cred_keyrings_rcu(ctx: *mut keyring_search_context) -> key_ref_t;
    pub fn search_process_keyrings_rcu(ctx: *mut keyring_search_context) -> key_ref_t;

    pub fn find_keyring_by_name(name: *const core::ffi::c_char, uid_keyring: bool) -> *mut key;

    pub fn look_up_user_keyrings(arg1: *mut *mut key, arg2: *mut *mut key) -> core::ffi::c_int;
    pub fn get_user_session_keyring_rcu(arg1: *const cred) -> *mut key;
    pub fn install_thread_keyring_to_cred(arg1: *mut cred) -> core::ffi::c_int;
    pub fn install_process_keyring_to_cred(arg1: *mut cred) -> core::ffi::c_int;
    pub fn install_session_keyring_to_cred(arg1: *mut cred, arg2: *mut key) -> core::ffi::c_int;

    pub fn request_key_and_link(
        type_: *mut key_type,
        description: *const core::ffi::c_char,
        domain_tag: *mut key_tag,
        callout_info: *const core::ffi::c_void,
        callout_len: size_t,
        aux: *mut core::ffi::c_void,
        dest_keyring: *mut key,
        flags: core::ffi::c_ulong,
    ) -> *mut key;

    pub fn lookup_user_key_possessed(key: *const key, match_data: *const key_match_data) -> bool;

    pub fn join_session_keyring(name: *const core::ffi::c_char) -> core::ffi::c_long;
    pub fn key_change_session_keyring(twork: *mut callback_head);

    pub static mut key_gc_work: work_struct;
    pub static mut key_gc_delay: core::ffi::c_uint;
    pub fn keyring_gc(keyring: *mut key, limit: time64_t);
    pub fn keyring_restriction_gc(keyring: *mut key, dead_type: *mut key_type);
    pub fn key_set_expiry(key: *mut key, expiry: time64_t);
    pub fn key_schedule_gc(gc_at: time64_t);
    pub fn key_schedule_gc_links();
    pub fn key_gc_keytype(ktype: *mut key_type);

    pub fn key_task_permission(
        key_ref: key_ref_t,
        cred: *const cred,
        need_perm: key_need_perm,
    ) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn notify_key(key: *mut key, subtype: key_notification_subtype, aux: u32) {
    #[cfg(CONFIG_KEY_NOTIFICATIONS)]
    {
        let mut n = key_notification {
            watch: watch_notification {
                type_: WATCH_TYPE_KEY_NOTIFY,
                subtype,
                info: 0,
            },
            key_id: key_serial(key),
            aux,
        };

        n.watch.info = watch_sizeof(&n);

        post_watch_notification(
            (*key).watchers,
            &mut n.watch,
            current_cred(),
            n.key_id,
        );
    }

    #[cfg(not(CONFIG_KEY_NOTIFICATIONS))]
    {
        let _ = (key, subtype, aux);
    }
}

/*
 * Check to see whether permission is granted to use a key in the desired way.
 */
#[inline]
pub unsafe fn key_permission(key_ref: key_ref_t, need_perm: key_need_perm) -> core::ffi::c_int {
    key_task_permission(key_ref, current_cred(), need_perm)
}

unsafe extern "C" {
    pub static mut key_type_request_key_auth: key_type;
    pub fn request_key_auth_new(
        target: *mut key,
        op: *const core::ffi::c_char,
        callout_info: *const core::ffi::c_void,
        callout_len: size_t,
        dest_keyring: *mut key,
    ) -> *mut key;
    pub fn request_key_auth_get(authkey: *mut key) -> *mut request_key_auth;
    pub fn request_key_auth_put(rka: *mut request_key_auth);

    pub fn key_get_instantiation_authkey(target_id: key_serial_t) -> *mut key;
}

/*
 * Determine whether a key is dead.
 */
#[inline]
pub unsafe fn key_is_dead(key: *const key, limit: time64_t) -> bool {
    let mut expiry: time64_t = (*key).expiry;

    if expiry != TIME64_MAX {
        if ((*(*key).type_).flags & KEY_TYPE_INSTANT_REAP) == 0 {
            expiry += key_gc_delay as time64_t;
        }
        if expiry <= limit {
            return true;
        }
    }

    ((*key).flags & ((1 << KEY_FLAG_DEAD) | (1 << KEY_FLAG_INVALIDATED))) != 0
        || (*(*key).domain_tag).removed
}

/*
 * keyctl() functions
 */
unsafe extern "C" {
    pub fn keyctl_get_keyring_ID(arg1: key_serial_t, arg2: core::ffi::c_int) -> core::ffi::c_long;
    pub fn keyctl_join_session_keyring(arg1: *const core::ffi::c_char) -> core::ffi::c_long;
    pub fn keyctl_update_key(
        arg1: key_serial_t,
        arg2: *const core::ffi::c_void,
        arg3: size_t,
    ) -> core::ffi::c_long;
    pub fn keyctl_revoke_key(arg1: key_serial_t) -> core::ffi::c_long;
    pub fn keyctl_keyring_clear(arg1: key_serial_t) -> core::ffi::c_long;
    pub fn keyctl_keyring_link(arg1: key_serial_t, arg2: key_serial_t) -> core::ffi::c_long;
    pub fn keyctl_keyring_move(
        arg1: key_serial_t,
        arg2: key_serial_t,
        arg3: key_serial_t,
        arg4: core::ffi::c_uint,
    ) -> core::ffi::c_long;
    pub fn keyctl_keyring_unlink(arg1: key_serial_t, arg2: key_serial_t) -> core::ffi::c_long;
    pub fn keyctl_describe_key(
        arg1: key_serial_t,
        arg2: *mut core::ffi::c_char,
        arg3: size_t,
    ) -> core::ffi::c_long;
    pub fn keyctl_keyring_search(
        arg1: key_serial_t,
        arg2: *const core::ffi::c_char,
        arg3: *const core::ffi::c_char,
        arg4: key_serial_t,
    ) -> core::ffi::c_long;
    pub fn keyctl_read_key(
        arg1: key_serial_t,
        arg2: *mut core::ffi::c_char,
        arg3: size_t,
    ) -> core::ffi::c_long;
    pub fn keyctl_chown_key(arg1: key_serial_t, arg2: uid_t, arg3: gid_t) -> core::ffi::c_long;
    pub fn keyctl_setperm_key(arg1: key_serial_t, arg2: key_perm_t) -> core::ffi::c_long;
    pub fn keyctl_instantiate_key(
        arg1: key_serial_t,
        arg2: *const core::ffi::c_void,
        arg3: size_t,
        arg4: key_serial_t,
    ) -> core::ffi::c_long;
    pub fn keyctl_negate_key(
        arg1: key_serial_t,
        arg2: core::ffi::c_uint,
        arg3: key_serial_t,
    ) -> core::ffi::c_long;
    pub fn keyctl_set_reqkey_keyring(arg1: core::ffi::c_int) -> core::ffi::c_long;
    pub fn keyctl_set_timeout(arg1: key_serial_t, arg2: core::ffi::c_uint) -> core::ffi::c_long;
    pub fn keyctl_assume_authority(arg1: key_serial_t) -> core::ffi::c_long;
    pub fn keyctl_get_security(
        keyid: key_serial_t,
        buffer: *mut core::ffi::c_char,
        buflen: size_t,
    ) -> core::ffi::c_long;
    pub fn keyctl_session_to_parent() -> core::ffi::c_long;
    pub fn keyctl_reject_key(
        arg1: key_serial_t,
        arg2: core::ffi::c_uint,
        arg3: core::ffi::c_uint,
        arg4: key_serial_t,
    ) -> core::ffi::c_long;
    pub fn keyctl_instantiate_key_iov(
        arg1: key_serial_t,
        arg2: *const iovec,
        arg3: core::ffi::c_uint,
        arg4: key_serial_t,
    ) -> core::ffi::c_long;
    pub fn keyctl_invalidate_key(arg1: key_serial_t) -> core::ffi::c_long;
    pub fn keyctl_restrict_keyring(
        id: key_serial_t,
        _type: *const core::ffi::c_char,
        _restriction: *const core::ffi::c_char,
    ) -> core::ffi::c_long;
}

#[cfg(CONFIG_PERSISTENT_KEYRINGS)]
unsafe extern "C" {
    pub fn keyctl_get_persistent(arg1: uid_t, arg2: key_serial_t) -> core::ffi::c_long;
    pub static mut persistent_keyring_expiry: core::ffi::c_uint;
}

#[cfg(not(CONFIG_PERSISTENT_KEYRINGS))]
#[inline]
pub unsafe fn keyctl_get_persistent(uid: uid_t, destring: key_serial_t) -> core::ffi::c_long {
    let _ = (uid, destring);
    -EOPNOTSUPP as core::ffi::c_long
}

#[cfg(CONFIG_KEY_DH_OPERATIONS)]
unsafe extern "C" {
    pub fn keyctl_dh_compute(
        arg1: *mut keyctl_dh_params,
        arg2: *mut core::ffi::c_char,
        arg3: size_t,
        arg4: *mut keyctl_kdf_params,
    ) -> core::ffi::c_long;
    pub fn __keyctl_dh_compute(
        arg1: *mut keyctl_dh_params,
        arg2: *mut core::ffi::c_char,
        arg3: size_t,
        arg4: *mut keyctl_kdf_params,
    ) -> core::ffi::c_long;
}

#[cfg(all(CONFIG_KEY_DH_OPERATIONS, CONFIG_COMPAT))]
unsafe extern "C" {
    pub fn compat_keyctl_dh_compute(
        params: *mut keyctl_dh_params,
        buffer: *mut core::ffi::c_char,
        buflen: size_t,
        kdf: *mut compat_keyctl_kdf_params,
    ) -> core::ffi::c_long;
}

#[cfg(CONFIG_KEY_DH_OPERATIONS)]
pub const KEYCTL_KDF_MAX_OUTPUT_LEN: core::ffi::c_int = 1024; /* max length of KDF output */
#[cfg(CONFIG_KEY_DH_OPERATIONS)]
pub const KEYCTL_KDF_MAX_OI_LEN: core::ffi::c_int = 64; /* max length of otherinfo */

#[cfg(not(CONFIG_KEY_DH_OPERATIONS))]
#[inline]
pub unsafe fn keyctl_dh_compute(
    params: *mut keyctl_dh_params,
    buffer: *mut core::ffi::c_char,
    buflen: size_t,
    kdf: *mut keyctl_kdf_params,
) -> core::ffi::c_long {
    let _ = (params, buffer, buflen, kdf);
    -EOPNOTSUPP as core::ffi::c_long
}

#[cfg(all(not(CONFIG_KEY_DH_OPERATIONS), CONFIG_COMPAT))]
#[inline]
pub unsafe fn compat_keyctl_dh_compute(
    params: *mut keyctl_dh_params,
    buffer: *mut core::ffi::c_char,
    buflen: size_t,
    kdf: *mut keyctl_kdf_params,
) -> core::ffi::c_long {
    let _ = (params, buffer, buflen, kdf);
    -EOPNOTSUPP as core::ffi::c_long
}

#[cfg(CONFIG_ASYMMETRIC_KEY_TYPE)]
unsafe extern "C" {
    pub fn keyctl_pkey_query(
        arg1: key_serial_t,
        arg2: *const core::ffi::c_char,
        arg3: *mut keyctl_pkey_query,
    ) -> core::ffi::c_long;

    pub fn keyctl_pkey_verify(
        arg1: *const keyctl_pkey_params,
        arg2: *const core::ffi::c_char,
        arg3: *const core::ffi::c_void,
        arg4: *const core::ffi::c_void,
    ) -> core::ffi::c_long;

    pub fn keyctl_pkey_e_d_s(
        arg1: core::ffi::c_int,
        arg2: *const keyctl_pkey_params,
        arg3: *const core::ffi::c_char,
        arg4: *const core::ffi::c_void,
        arg5: *mut core::ffi::c_void,
    ) -> core::ffi::c_long;
}

#[cfg(not(CONFIG_ASYMMETRIC_KEY_TYPE))]
#[inline]
pub unsafe fn keyctl_pkey_query(
    id: key_serial_t,
    _info: *const core::ffi::c_char,
    _res: *mut keyctl_pkey_query,
) -> core::ffi::c_long {
    let _ = (id, _info, _res);
    -EOPNOTSUPP as core::ffi::c_long
}

#[cfg(not(CONFIG_ASYMMETRIC_KEY_TYPE))]
#[inline]
pub unsafe fn keyctl_pkey_verify(
    params: *const keyctl_pkey_params,
    _info: *const core::ffi::c_char,
    _in: *const core::ffi::c_void,
    _in2: *const core::ffi::c_void,
) -> core::ffi::c_long {
    let _ = (params, _info, _in, _in2);
    -EOPNOTSUPP as core::ffi::c_long
}

#[cfg(not(CONFIG_ASYMMETRIC_KEY_TYPE))]
#[inline]
pub unsafe fn keyctl_pkey_e_d_s(
    op: core::ffi::c_int,
    params: *const keyctl_pkey_params,
    _info: *const core::ffi::c_char,
    _in: *const core::ffi::c_void,
    _out: *mut core::ffi::c_void,
) -> core::ffi::c_long {
    let _ = (op, params, _info, _in, _out);
    -EOPNOTSUPP as core::ffi::c_long
}

unsafe extern "C" {
    pub fn keyctl_capabilities(
        _buffer: *mut core::ffi::c_uchar,
        buflen: size_t,
    ) -> core::ffi::c_long;
}

#[cfg(CONFIG_KEY_NOTIFICATIONS)]
unsafe extern "C" {
    pub fn keyctl_watch_key(
        arg1: key_serial_t,
        arg2: core::ffi::c_int,
        arg3: core::ffi::c_int,
    ) -> core::ffi::c_long;
}

#[cfg(not(CONFIG_KEY_NOTIFICATIONS))]
#[inline]
pub unsafe fn keyctl_watch_key(
    key_id: key_serial_t,
    watch_fd: core::ffi::c_int,
    watch_id: core::ffi::c_int,
) -> core::ffi::c_long {
    let _ = (key_id, watch_fd, watch_id);
    -EOPNOTSUPP as core::ffi::c_long
}

/*
 * Debugging key validation
 */
#[cfg(KEY_DEBUGGING)]
unsafe extern "C" {
    pub fn __key_check(arg1: *const key);
}

#[cfg(KEY_DEBUGGING)]
#[inline]
pub unsafe fn key_check(key: *const key) {
    if !key.is_null() && (IS_ERR(key) || (*key).magic != KEY_DEBUG_MAGIC) {
        __key_check(key);
    }
}

#[cfg(not(KEY_DEBUGGING))]
macro_rules! key_check {
    ($key:expr) => {
        do_while_0!({})
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
