// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) 2012 by Pablo Neira Ayuso <pablo@netfilter.org>
 * (C) 2012 by Vyatta Inc. <http://www.vyatta.com>
 */

// Kernel dependencies supplied by the surrounding repository.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct net;
#[repr(C)]
pub struct nf_conn;
#[repr(C)]
pub struct nf_ct_timeout_hooks {
    pub timeout_put: Option<unsafe extern "C" fn(*mut nf_ct_timeout)>,
    pub timeout_find_get:
        Option<unsafe extern "C" fn(*mut net, *const c_char) -> *mut nf_ct_timeout>,
}
#[repr(C)]
pub struct nf_ct_timeout {
    pub l3num: u8,
    pub l4proto: *mut nf_conntrack_l4proto,
    pub refcnt: refcount_t,
    pub rcu: rcu_head,
}
#[repr(C)]
pub struct nf_conn_timeout {
    pub timeout: *mut nf_ct_timeout,
}
#[repr(C)]
pub struct nf_conntrack_l4proto {
    pub l4proto: u8,
}
#[repr(C)]
pub struct nf_ct_iter_data {
    pub net: *mut net,
    pub data: *mut c_void,
}
#[repr(C)]
pub struct refcount_t;
#[repr(C)]
pub struct rcu_head;

extern "C" {
    pub static mut nf_ct_timeout_hook: *const nf_ct_timeout_hooks;

    fn nf_ct_timeout_find(ct: *mut nf_conn) -> *mut nf_conn_timeout;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference<T>(ptr: *mut T) -> *mut T;
    fn rcu_init_pointer<T>(ptr: *mut T, value: *mut T);
    fn refcount_dec_and_test(refcnt: *mut refcount_t) -> bool;
    fn kfree_rcu<T>(ptr: *mut T, field: *mut rcu_head);
    fn nf_ct_iterate_cleanup_net(
        fnptr: unsafe extern "C" fn(*mut nf_conn, *mut c_void) -> c_int,
        data: *mut nf_ct_iter_data,
    );
    fn nf_ct_is_template(ct: *mut nf_conn) -> bool;
    fn nf_ct_timeout_ext_add(
        ct: *mut nf_conn,
        timeout: *mut nf_ct_timeout,
        gfp: c_int,
    ) -> *mut nf_conn_timeout;
}

// nf_ct_iterate_cleanup() holds the conntrack lock.
unsafe extern "C" fn untimeout(ct: *mut nf_conn, timeout: *mut c_void) -> c_int {
    let timeout_ext = nf_ct_timeout_find(ct);

    if !timeout_ext.is_null() {
        rcu_read_lock();
        let t = rcu_dereference(&mut (*timeout_ext).timeout);
        if t.is_null() {
            rcu_read_unlock();
            return 0;
        }

        if timeout.is_null() || t as *mut c_void == timeout {
            rcu_init_pointer(&mut (*timeout_ext).timeout, core::ptr::null_mut());

            // No race with nf_conntrack_free(), called only after removal
            // of the conntrack from the hashes.
            if refcount_dec_and_test(&mut (*t).refcnt) {
                kfree_rcu(t, &mut (*t).rcu);
            }
        }
        rcu_read_unlock();
    }

    // We are not intended to delete this conntrack.
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_untimeout(net: *mut net, timeout: *mut nf_ct_timeout) {
    let mut iter_data = nf_ct_iter_data {
        net,
        data: timeout as *mut c_void,
    };

    nf_ct_iterate_cleanup_net(untimeout, &mut iter_data);
}

unsafe fn __nf_ct_timeout_put(timeout: *mut nf_ct_timeout) {
    let h = rcu_dereference(&mut nf_ct_timeout_hook);

    if !h.is_null() {
        if let Some(timeout_put) = (*h).timeout_put {
            timeout_put(timeout);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_set_timeout(
    net: *mut net,
    ct: *mut nf_conn,
    l3num: u8,
    l4num: u8,
    timeout_name: *const c_char,
) -> c_int {
    let mut timeout: *mut nf_ct_timeout;
    let mut timeout_ext: *mut nf_conn_timeout;
    let mut errmsg: *const c_char = core::ptr::null();
    let mut ret: c_int = 0;

    // WARN_ON_ONCE(!nf_ct_is_template(ct));
    let _ = nf_ct_is_template(ct);

    rcu_read_lock();
    let h = rcu_dereference(&mut nf_ct_timeout_hook);
    if h.is_null() {
        ret = -2; // -ENOENT
        errmsg = b"Timeout policy base is empty\0".as_ptr() as *const c_char;
        rcu_read_unlock();
        let _ = errmsg;
        return ret;
    }

    timeout = match (*h).timeout_find_get {
        Some(timeout_find_get) => timeout_find_get(net, timeout_name),
        None => core::ptr::null_mut(),
    };
    if timeout.is_null() {
        ret = -2; // -ENOENT
        rcu_read_unlock();
        return ret;
    }

    if (*timeout).l3num != l3num {
        ret = -22; // -EINVAL
        __nf_ct_timeout_put(timeout);
        rcu_read_unlock();
        return ret;
    }
    // Make sure the timeout policy matches any existing protocol tracker.
    if (*(*timeout).l4proto).l4proto != l4num {
        ret = -22; // -EINVAL
        __nf_ct_timeout_put(timeout);
        rcu_read_unlock();
        return ret;
    }
    timeout_ext = nf_ct_timeout_ext_add(ct, timeout, 0 /* GFP_ATOMIC */);
    if timeout_ext.is_null() {
        ret = -12; // -ENOMEM
        __nf_ct_timeout_put(timeout);
        rcu_read_unlock();
        return ret;
    }

    rcu_read_unlock();
    ret
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_destroy_timeout(ct: *mut nf_conn) {
    // WARN_ON_ONCE(!nf_ct_is_template(ct));
    let _ = nf_ct_is_template(ct);

    rcu_read_lock();
    let h = rcu_dereference(&mut nf_ct_timeout_hook);

    if !h.is_null() {
        let timeout_ext = nf_ct_timeout_find(ct);
        if !timeout_ext.is_null() {
            let t = rcu_dereference(&mut (*timeout_ext).timeout);
            if !t.is_null() {
                if let Some(timeout_put) = (*h).timeout_put {
                    timeout_put(t);
                }
            }
            rcu_init_pointer(&mut (*timeout_ext).timeout, core::ptr::null_mut());
            if !t.is_null() && refcount_dec_and_test(&mut (*t).refcnt) {
                kfree_rcu(t, &mut (*t).rcu);
            }
        }
    }
    rcu_read_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
