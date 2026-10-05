// SPDX-License-Identifier: GPL-2.0-only
/*
 * The "user cache".
 *
 * (C) Copyright 1991-2000 Linus Torvalds
 *
 * We have a per-user structure to keep track of how many
 * processes, files etc the user has claimed, in order to be
 * able to have per-user limits for system resources.
 */

// Continued from Rust blob 6868dd0cee275e6b84fd8e94b34674cc816dca70.
// Native oracle: user.c blob 21bafdc1137956c8996bb41ea24c63ce9a2a5baa.
// Configured native headers own every layout, lock and static initializer.
#![no_std]
compile_error!("SOURCE ONLY HOLD: user-accounting Rust owner is not admitted");

use kernel::bindings::user_accounting_native::*;
use kernel::ffi::{c_int, c_ulong};

// init_binfmt_misc, init_user_ns and root_user have native storage in
// user_accounting_native_storage.c. In particular, IS_ENABLED(CONFIG_BINFMT_MISC)
// includes both y and m; n has neither the object nor the namespace field.
const UIDHASH_BITS: u32 = LUPOS_USER_ACCOUNTING_UIDHASH_BITS;
const UIDHASH_SZ: usize = LUPOS_USER_ACCOUNTING_UIDHASH_SZ as usize;
const UIDHASH_MASK: usize = UIDHASH_SZ - 1;

#[inline]
fn __uidhashfn(uid: uid_t) -> usize {
    // Native uid_t arithmetic is unsigned, including the addition's wrap.
    ((uid >> UIDHASH_BITS).wrapping_add(uid) as usize) & UIDHASH_MASK
}

static mut uid_cachep: *mut kmem_cache = core::ptr::null_mut();

/* These routines must be called with the uidhash spinlock held! */
// SAFETY: up is initialized and pinned, and hashent is an owner hash bucket.
unsafe fn uid_hash_insert(up: *mut user_struct, hashent: *mut hlist_head) {
    // SAFETY: The caller owns the native hash lock and the node insertion.
    unsafe { lupos_user_accounting_hash_insert(up, hashent) };
}

// SAFETY: up is the final-reference object, still linked under uidhash_lock.
unsafe fn uid_hash_remove(up: *mut user_struct) {
    // SAFETY: The held native hash lock excludes lookup and list mutation.
    unsafe { lupos_user_accounting_hash_remove(up) };
}

// SAFETY: The caller holds uidhash_lock throughout traversal and reference get.
unsafe fn uid_hash_find(uid: kuid_t, hashent: *mut hlist_head) -> *mut user_struct {
    // SAFETY: The lock keeps each node live; only a matching user gains a ref.
    unsafe {
        let mut user = lupos_user_accounting_hash_first(hashent);
        while !user.is_null() {
            if lupos_user_accounting_uid_eq(lupos_user_accounting_uid(user), uid) {
                lupos_user_accounting_ref_inc(user);
                return user;
            }
            user = lupos_user_accounting_hash_next(user);
        }
        core::ptr::null_mut()
    }
}

// SAFETY: up is unpublished zero-allocated storage, or the initial root_user.
unsafe fn user_epoll_alloc(up: *mut user_struct) -> c_int {
    #[cfg(CONFIG_EPOLL)]
    {
        // SAFETY: No other actor uses this counter during its initialization.
        unsafe { lupos_user_accounting_epoll_init(up) }
    }
    #[cfg(not(CONFIG_EPOLL))]
    { let _ = up; 0 }
}

// SAFETY: up has a successfully initialized counter and no remaining users.
unsafe fn user_epoll_free(up: *mut user_struct) {
    #[cfg(CONFIG_EPOLL)]
    {
        // SAFETY: This is final cleanup of the initialized native counter.
        unsafe { lupos_user_accounting_epoll_destroy(up) };
    }
    #[cfg(not(CONFIG_EPOLL))]
    { let _ = up; }
}

// SAFETY: uidhash_lock is held and the final ref is gone. flags is the output
// of the configured native irqsave operation. Non-RT restores saved IRQ state;
// PREEMPT_RT retains its native lock contract without an added IRQ-state change.
unsafe fn free_user(up: *mut user_struct, flags: c_ulong) {
    // SAFETY: Remove before unlock; after unlock only this owner may free up.
    unsafe {
        uid_hash_remove(up);
        lupos_user_accounting_unlock_irqrestore(flags);
        user_epoll_free(up);
        lupos_user_accounting_cache_free(uid_cachep, up);
    }
}

/// # Safety
/// The native user cache has been initialized. A nonnull return transfers a
/// reference that the caller must release through free_uid.
#[no_mangle]
pub unsafe extern "C" fn find_user(uid: kuid_t) -> *mut user_struct {
    // SAFETY: The native irqsave operation writes this exclusive local output
    // before lookup; the matching native restore consumes that value on both exits.
    unsafe {
        let mut flags: c_ulong = 0;
        lupos_user_accounting_lock_irqsave(&mut flags);
        let ret = uid_hash_find(uid, lupos_user_accounting_hash_bucket(
            __uidhashfn(lupos_user_accounting_kuid_val(uid))));
        lupos_user_accounting_unlock_irqrestore(flags);
        ret
    }
}

/// # Safety
/// A nonnull up transfers one live native user reference to this call.
#[no_mangle]
pub unsafe extern "C" fn free_uid(up: *mut user_struct) {
    if up.is_null() { return; }
    // SAFETY: The native conditional decrement owns the final-ref race. Only
    // true transfers the held lock and native flags output to free_user.
    unsafe {
        let mut flags: c_ulong = 0;
        if lupos_user_accounting_ref_dec_and_lock_irqsave(up, &mut flags) {
            free_user(up, flags);
        }
    }
}

/// # Safety
/// The native user cache is initialized and the caller may allocate with
/// GFP_KERNEL. The original spin_lock_irq/unlock_irq caller contract applies.
#[no_mangle]
pub unsafe extern "C" fn alloc_uid(uid: kuid_t) -> *mut user_struct {
    // SAFETY: Hash lookup and insertion hold uidhash_lock. Fresh storage stays
    // private until insertion; a race loser destroys only its own allocation.
    unsafe {
        let hashent = lupos_user_accounting_hash_bucket(
            __uidhashfn(lupos_user_accounting_kuid_val(uid)));
        lupos_user_accounting_lock_irq();
        let mut up = uid_hash_find(uid, hashent);
        lupos_user_accounting_unlock_irq();
        if up.is_null() {
            let new = lupos_user_accounting_cache_zalloc(uid_cachep);
            if new.is_null() { return core::ptr::null_mut(); }
            lupos_user_accounting_set_uid(new, uid);
            lupos_user_accounting_ref_set(new, 1);
            if user_epoll_alloc(new) != 0 {
                lupos_user_accounting_cache_free(uid_cachep, new);
                return core::ptr::null_mut();
            }
            lupos_user_accounting_ratelimit_init(new);
            lupos_user_accounting_ratelimit_set_flags(new);
            lupos_user_accounting_lock_irq();
            up = uid_hash_find(uid, hashent);
            if !up.is_null() {
                user_epoll_free(new);
                lupos_user_accounting_cache_free(uid_cachep, new);
            } else {
                uid_hash_insert(new, hashent);
                up = new;
            }
            lupos_user_accounting_unlock_irq();
        }
        up
    }
}

// SAFETY: Called once during the native subsys init phase, before allocation.
#[cold]
#[link_section = ".init.text"]
unsafe fn uid_cache_init() -> c_int {
    // SAFETY: This phase exclusively initializes the cache pointer, hash heads
    // and root counter. Root publication still uses the native hash lock.
    unsafe {
        uid_cachep = lupos_user_accounting_cache_create();
        let mut n = 0;
        while n < UIDHASH_SZ {
            lupos_user_accounting_hash_init(lupos_user_accounting_hash_bucket(n));
            n += 1;
        }
        if user_epoll_alloc(lupos_user_accounting_root_user()) != 0 {
            lupos_user_accounting_root_epoll_panic();
        }
        lupos_user_accounting_lock_irq();
        uid_hash_insert(lupos_user_accounting_root_user(),
            lupos_user_accounting_hash_bucket(__uidhashfn(
                lupos_user_accounting_kuid_val(lupos_user_accounting_root_uid()))));
        lupos_user_accounting_unlock_irq();
        0
    }
}

/// # Safety
/// Only the native __init uid_cache_init(void) trampoline calls this body,
/// exactly once at subsys_initcall time while .init.text remains live.
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_user_accounting_uid_cache_init() -> c_int {
    // SAFETY: The native initcall provides the phase and one-shot contract.
    unsafe { uid_cache_init() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
