// SPDX-License-Identifier: GPL-2.0-or-later
/* procfs files for key database enumeration
 *
 * Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies:
// <linux/init.h>, <linux/sched.h>, <linux/fs.h>, <linux/proc_fs.h>,
// <linux/seq_file.h>, <asm/errno.h>, "internal.h"
use crate::*;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

unsafe extern "C" {
    fn proc_create_seq(
        name: *const c_char,
        mode: c_int,
        parent: *mut proc_dir_entry,
        ops: *const seq_operations,
    ) -> *mut proc_dir_entry;
    fn panic(fmt: *const c_char, ...) -> !;
    fn seq_user_ns(p: *mut seq_file) -> *mut user_namespace;
    fn rb_next(n: *mut rb_node) -> *mut rb_node;
    fn rb_first(r: *mut rb_root) -> *mut rb_node;
    fn kuid_has_mapping(user_ns: *mut user_namespace, uid: kuid_t) -> bool;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn make_key_ref(key: *mut key, possessed: c_int) -> key_ref_t;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn search_cred_keyrings_rcu(ctx: *mut keyring_search_context) -> key_ref_t;
    fn IS_ERR(ptr: key_ref_t) -> bool;
    fn key_ref_put(key_ref: key_ref_t);
    fn key_task_permission(key_ref: key_ref_t, cred: *const cred, perm: key_perm_t) -> c_int;
    fn ktime_get_real_seconds() -> time64_t;
    fn key_read_state(key: *const key) -> i16;
    fn refcount_read(r: *const refcount_t) -> c_int;
    fn from_kuid_munged(user_ns: *mut user_namespace, uid: kuid_t) -> u32;
    fn from_kgid_munged(user_ns: *mut user_namespace, gid: kgid_t) -> u32;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn seq_putc(m: *mut seq_file, c: c_char);
    fn div_u64(dividend: u64, divisor: u32) -> u64;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn uid_eq(left: kuid_t, right: kuid_t) -> bool;
    fn atomic_read(v: *const atomic_t) -> c_int;
}

// `static` C prototypes translated as private Rust functions below.

static proc_keys_ops: seq_operations = seq_operations {
    start: Some(proc_keys_start),
    next: Some(proc_keys_next),
    stop: Some(proc_keys_stop),
    show: Some(proc_keys_show),
};

static proc_key_users_ops: seq_operations = seq_operations {
    start: Some(proc_key_users_start),
    next: Some(proc_key_users_next),
    stop: Some(proc_key_users_stop),
    show: Some(proc_key_users_show),
};

/*
 * Declare the /proc files.
 */
unsafe extern "C" fn key_proc_init() -> c_int {
    let mut p: *mut proc_dir_entry;

    p = proc_create_seq(c"keys".as_ptr(), 0, ptr::null_mut(), &proc_keys_ops);
    if p.is_null() {
        panic(c"Cannot create /proc/keys\n".as_ptr());
    }

    p = proc_create_seq(c"key-users".as_ptr(), 0, ptr::null_mut(), &proc_key_users_ops);
    if p.is_null() {
        panic(c"Cannot create /proc/key-users\n".as_ptr());
    }

    0
}

// C `__initcall(key_proc_init);`

/*
 * Implement "/proc/keys" to provide a list of the keys on the system that
 * grant View permission to the caller.
 */
unsafe extern "C" fn key_serial_next(p: *mut seq_file, mut n: *mut rb_node) -> *mut rb_node {
    let user_ns: *mut user_namespace = seq_user_ns(p);

    n = rb_next(n);
    while !n.is_null() {
        let key: *mut key = rb_entry!(n, key, serial_node);
        if kuid_has_mapping(user_ns, (*(*key).user).uid) {
            break;
        }
        n = rb_next(n);
    }
    n
}

unsafe extern "C" fn find_ge_key(p: *mut seq_file, id: key_serial_t) -> *mut key {
    let user_ns: *mut user_namespace = seq_user_ns(p);
    let mut n: *mut rb_node = key_serial_tree.rb_node;
    let mut minkey: *mut key = ptr::null_mut();

    while !n.is_null() {
        let mut key: *mut key = rb_entry!(n, key, serial_node);
        if id < (*key).serial {
            if minkey.is_null() || (*minkey).serial > (*key).serial {
                minkey = key;
            }
            n = (*n).rb_left;
        } else if id > (*key).serial {
            n = (*n).rb_right;
        } else {
            minkey = key;
            break;
        }
        key = ptr::null_mut();
        let _ = key;
    }

    if minkey.is_null() {
        return ptr::null_mut();
    }

    loop {
        if kuid_has_mapping(user_ns, (*(*minkey).user).uid) {
            return minkey;
        }
        n = rb_next(&mut (*minkey).serial_node);
        if n.is_null() {
            return ptr::null_mut();
        }
        minkey = rb_entry!(n, key, serial_node);
    }
}

unsafe extern "C" fn proc_keys_start(p: *mut seq_file, _pos: *mut loff_t) -> *mut c_void {
    let pos: key_serial_t = *_pos as key_serial_t;
    let key: *mut key;

    spin_lock(&mut key_serial_lock);

    if *_pos > INT_MAX as loff_t {
        return ptr::null_mut();
    }
    key = find_ge_key(p, pos);
    if key.is_null() {
        return ptr::null_mut();
    }
    *_pos = (*key).serial as loff_t;
    &mut (*key).serial_node as *mut rb_node as *mut c_void
}

#[inline]
unsafe extern "C" fn key_node_serial(n: *mut rb_node) -> key_serial_t {
    let key: *mut key = rb_entry!(n, key, serial_node);
    (*key).serial
}

unsafe extern "C" fn proc_keys_next(
    p: *mut seq_file,
    v: *mut c_void,
    _pos: *mut loff_t,
) -> *mut c_void {
    let n: *mut rb_node;

    n = key_serial_next(p, v as *mut rb_node);
    if !n.is_null() {
        *_pos = key_node_serial(n) as loff_t;
    } else {
        *_pos += 1;
    }
    n as *mut c_void
}

unsafe extern "C" fn proc_keys_stop(_p: *mut seq_file, _v: *mut c_void) {
    spin_unlock(&mut key_serial_lock);
}

unsafe extern "C" fn proc_keys_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let _p: *mut rb_node = v as *mut rb_node;
    let key: *mut key = rb_entry!(_p, key, serial_node);
    let flags: c_ulong;
    let mut key_ref: key_ref_t;
    let skey_ref: key_ref_t;
    let now: time64_t;
    let expiry: time64_t;
    let mut xbuf: [c_char; 16] = [0; 16];
    let state: i16;
    let timo: u64;
    let rc: c_int;

    let mut ctx = keyring_search_context {
        index_key: (*key).index_key,
        cred: (*(*m).file).f_cred,
        match_data: keyring_search_match_data {
            cmp: Some(lookup_user_key_possessed),
            raw_data: key as *const c_void,
            lookup_type: KEYRING_SEARCH_LOOKUP_DIRECT,
        },
        flags: KEYRING_SEARCH_NO_STATE_CHECK | KEYRING_SEARCH_RECURSE,
    };

    key_ref = make_key_ref(key, 0);

    /* determine if the key is possessed by this process (a test we can
     * skip if the key does not indicate the possessor can view it
     */
    if (*key).perm & KEY_POS_VIEW != 0 {
        rcu_read_lock();
        skey_ref = search_cred_keyrings_rcu(&mut ctx);
        rcu_read_unlock();
        if !IS_ERR(skey_ref) {
            key_ref_put(skey_ref);
            key_ref = make_key_ref(key, 1);
        }
    }

    /* check whether the current task is allowed to view the key */
    rc = key_task_permission(key_ref, ctx.cred, KEY_NEED_VIEW);
    if rc < 0 {
        return 0;
    }

    now = ktime_get_real_seconds();

    rcu_read_lock();

    /* come up with a suitable timeout value */
    expiry = READ_ONCE!((*key).expiry);
    if expiry == TIME64_MAX {
        memcpy(xbuf.as_mut_ptr() as *mut c_void, c"perm".as_ptr() as *const c_void, 5);
    } else if now >= expiry {
        memcpy(xbuf.as_mut_ptr() as *mut c_void, c"expd".as_ptr() as *const c_void, 5);
    } else {
        timo = (expiry - now) as u64;

        if timo < 60 {
            sprintf(xbuf.as_mut_ptr(), c"%llus".as_ptr(), timo);
        } else if timo < 60 * 60 {
            sprintf(xbuf.as_mut_ptr(), c"%llum".as_ptr(), div_u64(timo, 60));
        } else if timo < 60 * 60 * 24 {
            sprintf(xbuf.as_mut_ptr(), c"%lluh".as_ptr(), div_u64(timo, 60 * 60));
        } else if timo < 60 * 60 * 24 * 7 {
            sprintf(xbuf.as_mut_ptr(), c"%llud".as_ptr(), div_u64(timo, 60 * 60 * 24));
        } else {
            sprintf(xbuf.as_mut_ptr(), c"%lluw".as_ptr(), div_u64(timo, 60 * 60 * 24 * 7));
        }
    }

    state = key_read_state(key);

    macro_rules! showflag {
        ($FLAGS:expr, $LETTER:expr, $FLAG:expr) => {
            if ($FLAGS & (1 << $FLAG)) != 0 {
                $LETTER
            } else {
                b'-' as c_char
            }
        };
    }

    flags = READ_ONCE!((*key).flags);
    seq_printf(
        m,
        c"%08x %c%c%c%c%c%c%c %5d %4s %08x %5d %5d %-9.9s ".as_ptr(),
        (*key).serial,
        if state != KEY_IS_UNINSTANTIATED { b'I' as c_char } else { b'-' as c_char },
        showflag!(flags, b'R' as c_char, KEY_FLAG_REVOKED),
        showflag!(flags, b'D' as c_char, KEY_FLAG_DEAD),
        showflag!(flags, b'Q' as c_char, KEY_FLAG_IN_QUOTA),
        showflag!(flags, b'U' as c_char, KEY_FLAG_USER_CONSTRUCT),
        if state < 0 { b'N' as c_char } else { b'-' as c_char },
        showflag!(flags, b'i' as c_char, KEY_FLAG_INVALIDATED),
        refcount_read(&(*key).usage),
        xbuf.as_ptr(),
        (*key).perm,
        from_kuid_munged(seq_user_ns(m), (*key).uid),
        from_kgid_munged(seq_user_ns(m), (*key).gid),
        (*(*key).type_).name,
    );

    if (*(*key).type_).describe.is_some() {
        ((*(*key).type_).describe.unwrap())(key, m);
    }
    seq_putc(m, b'\n' as c_char);

    rcu_read_unlock();
    0
}

unsafe extern "C" fn __key_user_next(
    user_ns: *mut user_namespace,
    mut n: *mut rb_node,
) -> *mut rb_node {
    while !n.is_null() {
        let user: *mut key_user = rb_entry!(n, key_user, node);
        if kuid_has_mapping(user_ns, (*user).uid) {
            break;
        }
        n = rb_next(n);
    }
    n
}

unsafe extern "C" fn key_user_next(
    user_ns: *mut user_namespace,
    n: *mut rb_node,
) -> *mut rb_node {
    __key_user_next(user_ns, rb_next(n))
}

unsafe extern "C" fn key_user_first(
    user_ns: *mut user_namespace,
    r: *mut rb_root,
) -> *mut rb_node {
    let n: *mut rb_node = rb_first(r);
    __key_user_next(user_ns, n)
}

unsafe extern "C" fn proc_key_users_start(p: *mut seq_file, _pos: *mut loff_t) -> *mut c_void {
    let mut _p: *mut rb_node;
    let mut pos: loff_t = *_pos;

    spin_lock(&mut key_user_lock);

    _p = key_user_first(seq_user_ns(p), &mut key_user_tree);
    while pos > 0 && !_p.is_null() {
        pos -= 1;
        _p = key_user_next(seq_user_ns(p), _p);
    }

    _p as *mut c_void
}

unsafe extern "C" fn proc_key_users_next(
    p: *mut seq_file,
    v: *mut c_void,
    _pos: *mut loff_t,
) -> *mut c_void {
    *_pos += 1;
    key_user_next(seq_user_ns(p), v as *mut rb_node) as *mut c_void
}

unsafe extern "C" fn proc_key_users_stop(_p: *mut seq_file, _v: *mut c_void) {
    spin_unlock(&mut key_user_lock);
}

unsafe extern "C" fn proc_key_users_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let _p: *mut rb_node = v as *mut rb_node;
    let user: *mut key_user = rb_entry!(_p, key_user, node);
    let maxkeys: u32 = if uid_eq((*user).uid, GLOBAL_ROOT_UID) {
        key_quota_root_maxkeys
    } else {
        key_quota_maxkeys
    };
    let maxbytes: u32 = if uid_eq((*user).uid, GLOBAL_ROOT_UID) {
        key_quota_root_maxbytes
    } else {
        key_quota_maxbytes
    };

    seq_printf(
        m,
        c"%5u: %5d %d/%d %d/%d %d/%d\n".as_ptr(),
        from_kuid_munged(seq_user_ns(m), (*user).uid),
        refcount_read(&(*user).usage),
        atomic_read(&(*user).nkeys),
        atomic_read(&(*user).nikeys),
        (*user).qnkeys,
        maxkeys,
        (*user).qnbytes,
        maxbytes,
    );

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
