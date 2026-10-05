// SPDX-License-Identifier: GPL-2.0-only

//! User and resource-limit accounting over native kernel primitives.

// Continue the existing ucount.rs at baseline 126a30fae3bb; retain its bodies
// and function identities. Native headers own storage, layouts and primitives.
// Public/callback/init C entry points forward directly to these Rust bodies.
// No original ucount.c algorithm is included, renamed or used as a fallback.
compile_error!("SOURCE ONLY HOLD: user-accounting Rust owner is not admitted");

use kernel::bindings::ucount_native::*;
use kernel::ffi::{c_int, c_long, c_ulong};
#[cfg(CONFIG_SYSCTL)]
use kernel::ffi::c_uint;

// Preserve the original root object identity with its native initializer.
pub use kernel::bindings::ucount_native::init_ucounts;
/// Native hash width retained from the original user-accounting owner.
pub const UCOUNTS_HASHTABLE_BITS: usize = LUPOS_UCOUNT_HASHTABLE_BITS as usize;
/// Native number of user-accounting hash buckets.
pub const UCOUNTS_HASHTABLE_ENTRIES: usize = LUPOS_UCOUNT_HASHTABLE_ENTRIES as usize;

#[cfg(CONFIG_SYSCTL)]
#[export_name = "lupos_ucount_set_lookup"]
unsafe extern "C" fn set_lookup(_root: *mut ctl_table_root) -> *mut ctl_table_set {
    // SAFETY: Native sysctl callback context pins current's credentials and ns.
    unsafe { lupos_ucount_current_set() }
}

#[cfg(CONFIG_SYSCTL)]
#[export_name = "lupos_ucount_set_is_seen"]
unsafe extern "C" fn set_is_seen(set: *mut ctl_table_set) -> c_int {
    // SAFETY: The native callback supplies a live set. Compare its address,
    // correcting the old Rust prefix's attempted aggregate-to-pointer cast.
    unsafe { (lupos_ucount_current_set() == set) as c_int }
}

#[cfg(CONFIG_SYSCTL)]
#[export_name = "lupos_ucount_set_permissions"]
unsafe extern "C" fn set_permissions(
    head: *mut ctl_table_header,
    table: *const ctl_table,
) -> c_int {
    // SAFETY: Sysctl pins head, its namespace set and table for this callback.
    // All native field projections, including randomized layouts, stay in C.
    unsafe {
        let user_ns = lupos_ucount_head_namespace(head);
        let mode;
        // Allow users with CAP_SYS_RESOURCE unrestrained access.
        if lupos_ucount_ns_capable_resource(user_ns) {
            mode = (lupos_ucount_table_mode(table) & LUPOS_UCOUNT_S_IRWXU) >> 6;
        } else {
            // Allow all others at most read-only access.
            mode = lupos_ucount_table_mode(table) & LUPOS_UCOUNT_S_IROTH;
        }
        (mode << 6) | (mode << 3) | mode
    }
}

/// # Safety
/// ns is live and its unpublished sysctl state is owned by the caller; caller
/// may sleep. On failure this retires the set but does not put the namespace.
#[export_name = "lupos_ucount_setup_userns_sysctls"]
pub unsafe extern "C" fn setup_userns_sysctls(ns: *mut user_namespace) -> bool {
    #[cfg(CONFIG_SYSCTL)]
    // SAFETY: Caller owns setup; native leaves size and address the table and
    // namespace. No Rust reference or invented layout crosses this boundary.
    unsafe {
        lupos_ucount_check_user_table();
        lupos_ucount_setup_set(ns);
        let tbl = lupos_ucount_dup_user_table();
        if !tbl.is_null() {
            let mut i: c_uint = 0;
            while i < LUPOS_UCOUNT_COUNTS {
                lupos_ucount_set_table_data(tbl, i, ns);
                i += 1;
            }
            let header = lupos_ucount_register_table(ns, tbl);
            lupos_ucount_set_sysctls(ns, header);
        }
        if lupos_ucount_sysctls(ns).is_null() {
            lupos_ucount_free_table(tbl);
            lupos_ucount_retire_set(ns);
            return false;
        }
    }
    // In SYSCTL=n the original C function does not inspect ns.
    #[cfg(not(CONFIG_SYSCTL))]
    let _ = ns;
    true
}

/// # Safety
/// ns has successfully installed sysctls; caller owns the teardown lifetime.
#[export_name = "lupos_ucount_retire_userns_sysctls"]
pub unsafe extern "C" fn retire_userns_sysctls(ns: *mut user_namespace) {
    #[cfg(CONFIG_SYSCTL)]
    // SAFETY: Save ctl_table_arg before unregister can release its header.
    unsafe {
        let header = lupos_ucount_sysctls(ns);
        let tbl = lupos_ucount_header_table(header);
        lupos_ucount_unregister_table(header);
        lupos_ucount_retire_set(ns);
        lupos_ucount_free_table(tbl);
    }
    #[cfg(not(CONFIG_SYSCTL))]
    let _ = ns;
}

unsafe fn find_ucounts(
    ns: *mut user_namespace,
    uid: kuid_t,
    hashent: *mut hlist_nulls_head,
) -> *mut ucounts {
    // SAFETY: Caller pins ns; hashent is in native static storage. RCU brackets
    // every node access and rcuref_get. Both return paths release the guard.
    unsafe {
        lupos_ucount_rcu_read_lock();
        let mut pos = lupos_ucount_first_rcu(hashent);
        while !lupos_ucount_is_nulls(pos) {
            let ucount = lupos_ucount_from_node(pos);
            if lupos_ucount_uid_eq(lupos_ucount_uid(ucount), uid)
                && lupos_ucount_namespace(ucount) == ns
                && lupos_ucount_ref_get(ucount)
            {
                lupos_ucount_rcu_read_unlock();
                return ucount;
            }
            pos = lupos_ucount_next_rcu(pos);
        }
        lupos_ucount_rcu_read_unlock();
        core::ptr::null_mut()
    }
}

unsafe fn hlist_add_ucounts(ucount: *mut ucounts) {
    // SAFETY: Boot owns this live, not-yet-linked object. The native IRQ lock
    // serializes publication and the native RCU list macro orders its stores.
    unsafe {
        let hashent = lupos_ucount_hashentry(
            lupos_ucount_namespace(ucount),
            lupos_ucount_uid(ucount),
        );
        lupos_ucount_lock_irq();
        lupos_ucount_add_head_rcu(ucount, hashent);
        lupos_ucount_unlock_irq();
    }
}

/// # Safety
/// Caller pins ns and may sleep. Success transfers one ucounts reference.
#[export_name = "lupos_ucount_alloc_ucounts"]
pub unsafe extern "C" fn alloc_ucounts(ns: *mut user_namespace, uid: kuid_t) -> *mut ucounts {
    // SAFETY: Fresh allocation is private until list publication. The duplicate
    // lookup runs under the same native lock, and returns its own rcuref.
    unsafe {
        let hashent = lupos_ucount_hashentry(ns, uid);
        let ucount = find_ucounts(ns, uid, hashent);
        if !ucount.is_null() { return ucount; }
        let new = lupos_ucount_zalloc();
        if new.is_null() { return core::ptr::null_mut(); }
        lupos_ucount_set_namespace(new, ns);
        lupos_ucount_set_uid(new, uid);
        lupos_ucount_ref_init(new);
        lupos_ucount_lock_irq();
        let existing = find_ucounts(ns, uid, hashent);
        if !existing.is_null() {
            lupos_ucount_unlock_irq();
            lupos_ucount_free(new);
            return existing;
        }
        lupos_ucount_add_head_rcu(new, hashent);
        lupos_ucount_get_user_ns(lupos_ucount_namespace(new));
        lupos_ucount_unlock_irq();
        new
    }
}

/// # Safety
/// Caller surrenders one live ucounts reference, with no later unpinned access.
#[export_name = "lupos_ucount_put_ucounts"]
pub unsafe extern "C" fn put_ucounts(ucount: *mut ucounts) {
    // SAFETY: Only native rcuref_put success permits unlink, namespace put and
    // RCU-delayed free. IRQ flags retain the configured unsigned-long type.
    unsafe {
        if lupos_ucount_ref_put(ucount) {
            let flags = lupos_ucount_lock_irqsave();
            lupos_ucount_del_rcu(ucount);
            lupos_ucount_unlock_irqrestore(flags);
            lupos_ucount_put_user_ns(lupos_ucount_namespace(ucount));
            lupos_ucount_free_rcu(ucount);
        }
    }
}

unsafe fn atomic_long_inc_below(v: *mut atomic_long_t, u: c_long) -> bool {
    // SAFETY: v is a live native counter. Only its configured native atomic
    // operations touch storage; failed cmpxchg refreshes c for this retry loop.
    unsafe {
        let mut c = lupos_ucount_atomic_read(v);
        loop {
            if c >= u { return false; }
            let new = c.wrapping_add(1);
            if lupos_ucount_atomic_try_cmpxchg(v, &mut c, new) { break; }
        }
        true
    }
}

/// # Safety
/// Caller pins ns, supplies a valid configured ucount_type and may sleep.
/// Success holds one leaf reference and charges its complete hierarchy.
#[export_name = "lupos_ucount_inc_ucount"]
pub unsafe extern "C" fn inc_ucount(
    ns: *mut user_namespace,
    uid: kuid_t,
    type_: ucount_type,
) -> *mut ucounts {
    // SAFETY: Allocation pins the leaf and its namespace hierarchy. Failed
    // ancestors were not incremented; rollback only touches the charged prefix.
    unsafe {
        let ucounts = alloc_ucounts(ns, uid);
        let mut iter = ucounts;
        while !iter.is_null() {
            let tns = lupos_ucount_namespace(iter);
            let max = lupos_ucount_namespace_max(tns, type_);
            if !atomic_long_inc_below(lupos_ucount_counter(iter, type_), max) {
                let bad = iter;
                let mut rollback = ucounts;
                while rollback != bad {
                    lupos_ucount_atomic_dec(lupos_ucount_counter(rollback, type_));
                    rollback = lupos_ucount_namespace_ucounts(lupos_ucount_namespace(rollback));
                }
                put_ucounts(ucounts);
                return core::ptr::null_mut();
            }
            iter = lupos_ucount_namespace_ucounts(tns);
        }
        ucounts
    }
}

// EXPORT_SYMBOL_FOR_MODULES(inc_ucount, "binfmt_misc") lives in native metadata.

/// # Safety
/// Caller surrenders the reference and matching hierarchical charge returned
/// by inc_ucount. type_ must be the same valid counter used for that charge.
#[export_name = "lupos_ucount_dec_ucount"]
pub unsafe extern "C" fn dec_ucount(ucounts: *mut ucounts, type_: ucount_type) {
    // SAFETY: The held leaf reference pins traversal until its single final put.
    unsafe {
        let mut iter = ucounts;
        while !iter.is_null() {
            let dec = lupos_ucount_atomic_dec_if_positive(lupos_ucount_counter(iter, type_));
            lupos_ucount_warn_dec_ucount(dec);
            iter = lupos_ucount_namespace_ucounts(lupos_ucount_namespace(iter));
        }
        put_ucounts(ucounts);
    }
}

/// # Safety
/// Caller pins ucounts and its hierarchy and supplies a valid rlimit_type.
/// This charges every ancestor even when the returned value reports overflow.
#[export_name = "lupos_ucount_inc_rlimit_ucounts"]
pub unsafe extern "C" fn inc_rlimit_ucounts(
    ucounts: *mut ucounts,
    type_: rlimit_type,
    v: c_long,
) -> c_long {
    // SAFETY: Native atomics and READ_ONCE limits preserve C ordering/width.
    unsafe {
        let mut iter = ucounts;
        let mut max = LUPOS_UCOUNT_LONG_MAX;
        let mut ret = 0;
        while !iter.is_null() {
            let new = lupos_ucount_atomic_add_return(v, lupos_ucount_rlimit_counter(iter, type_));
            if new < 0 || new > max {
                ret = LUPOS_UCOUNT_LONG_MAX;
            } else if iter == ucounts {
                ret = new;
            }
            max = lupos_ucount_rlimit_max(lupos_ucount_namespace(iter), type_);
            iter = lupos_ucount_namespace_ucounts(lupos_ucount_namespace(iter));
        }
        ret
    }
}

/// # Safety
/// Caller pins the hierarchy and supplies the valid type and matching charge.
#[export_name = "lupos_ucount_dec_rlimit_ucounts"]
pub unsafe extern "C" fn dec_rlimit_ucounts(
    ucounts: *mut ucounts,
    type_: rlimit_type,
    v: c_long,
) -> bool {
    // SAFETY: No references are released here; the leaf result alone is returned.
    unsafe {
        let mut iter = ucounts;
        let mut new = -1;
        while !iter.is_null() {
            let dec = lupos_ucount_atomic_sub_return(v, lupos_ucount_rlimit_counter(iter, type_));
            lupos_ucount_warn_dec_rlimit(dec);
            if iter == ucounts { new = dec; }
            iter = lupos_ucount_namespace_ucounts(lupos_ucount_namespace(iter));
        }
        new == 0
    }
}

unsafe fn do_dec_rlimit_put_ucounts(
    ucounts: *mut ucounts,
    last: *mut ucounts,
    type_: rlimit_type,
) {
    // SAFETY: [ucounts, last) is the charged prefix; zero transitions own an
    // extra reference. Fetch next before put can destroy iter or its namespace.
    unsafe {
        let mut iter = ucounts;
        while iter != last {
            let dec = lupos_ucount_atomic_sub_return(1, lupos_ucount_rlimit_counter(iter, type_));
            lupos_ucount_warn_do_dec_rlimit(dec);
            let next = lupos_ucount_namespace_ucounts(lupos_ucount_namespace(iter));
            if dec == 0 { put_ucounts(iter); }
            iter = next;
        }
    }
}

/// # Safety
/// Caller surrenders one successful inc_rlimit_get_ucounts hierarchy charge.
#[export_name = "lupos_ucount_dec_rlimit_put_ucounts"]
pub unsafe extern "C" fn dec_rlimit_put_ucounts(ucounts: *mut ucounts, type_: rlimit_type) {
    // SAFETY: The full charged chain ends at NULL and owns its zero-transition refs.
    unsafe { do_dec_rlimit_put_ucounts(ucounts, core::ptr::null_mut(), type_); }
}

/// # Safety
/// Caller must hold a reference to ucounts throughout, as in the original C.
/// type_ is valid; override_rlimit has the same caller-side authorization as C.
#[export_name = "lupos_ucount_inc_rlimit_get_ucounts"]
pub unsafe extern "C" fn inc_rlimit_get_ucounts(
    ucounts: *mut ucounts,
    type_: rlimit_type,
    override_rlimit: bool,
) -> c_long {
    // SAFETY: Caller pins the chain. Only 0-to-1 transitions take extra refs.
    // The failed node's increment is removed without a put, then the successful
    // prefix is unwound using the same zero-transition reference protocol.
    unsafe {
        let mut iter = ucounts;
        let mut max = LUPOS_UCOUNT_LONG_MAX;
        let mut ret = 0;
        while !iter.is_null() {
            let new = lupos_ucount_atomic_add_return(1, lupos_ucount_rlimit_counter(iter, type_));
            if new < 0 || new > max { break; }
            if iter == ucounts { ret = new; }
            if !override_rlimit {
                max = lupos_ucount_rlimit_max(lupos_ucount_namespace(iter), type_);
            }
            // Grab an extra ucount reference when the rlimit was previously 0.
            if new == 1 && lupos_ucount_get_ucounts(iter).is_null() { break; }
            iter = lupos_ucount_namespace_ucounts(lupos_ucount_namespace(iter));
        }
        if iter.is_null() { return ret; }
        let dec = lupos_ucount_atomic_sub_return(1, lupos_ucount_rlimit_counter(iter, type_));
        lupos_ucount_warn_inc_rlimit_unwind(dec);
        do_dec_rlimit_put_ucounts(ucounts, iter, type_);
        0
    }
}

/// # Safety
/// Caller pins ucounts and its hierarchy and supplies a valid rlimit_type.
#[export_name = "lupos_ucount_is_rlimit_overlimit"]
pub unsafe extern "C" fn is_rlimit_overlimit(
    ucounts: *mut ucounts,
    type_: rlimit_type,
    rlimit: c_ulong,
) -> bool {
    // SAFETY: Clamp unsigned input before comparison and retain native long
    // atomic reads and parent limit propagation. No Rust field references exist.
    unsafe {
        let mut iter = ucounts;
        let mut max = rlimit as c_long;
        if rlimit > LUPOS_UCOUNT_LONG_MAX as c_ulong { max = LUPOS_UCOUNT_LONG_MAX; }
        while !iter.is_null() {
            let val = lupos_ucount_rlimit_value(iter, type_);
            if val < 0 || val > max { return true; }
            max = lupos_ucount_rlimit_max(lupos_ucount_namespace(iter), type_);
            iter = lupos_ucount_namespace_ucounts(lupos_ucount_namespace(iter));
        }
        false
    }
}

#[export_name = "lupos_ucount_user_namespace_sysctl_init"]
#[link_section = ".init.text"]
#[cold]
unsafe extern "C" fn user_namespace_sysctl_init() -> c_int {
    // SAFETY: Called once by the native __init subsys-initcall wrapper before
    // init sections are discarded. Root objects have native static lifetimes.
    unsafe {
        #[cfg(CONFIG_SYSCTL)]
        {
            let user_header = lupos_ucount_register_user_root();
            lupos_ucount_store_user_header(user_header);
            lupos_ucount_kmemleak_ignore(user_header);
            lupos_ucount_bug_user_header(user_header);
            // Native BUG_ON evaluates the exact original setup expression;
            // its typed public wrapper calls only this owner's Rust body.
            lupos_ucount_bug_setup_initial_sysctls();
        }
        let root = core::ptr::addr_of_mut!(init_ucounts);
        hlist_add_ucounts(root);
        inc_rlimit_ucounts(root, lupos_ucount_nproc_type(), 1);
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
