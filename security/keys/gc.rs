// SPDX-License-Identifier: GPL-2.0-or-later
/* Key garbage collector
 *
 * Copyright (C) 2009-2011 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_ulong, c_void};

pub type time64_t = i64;
pub type u8 = u8;

pub const TIME64_MAX: time64_t = i64::MAX;
pub const HZ: c_ulong = 100;
pub const TASK_UNINTERRUPTIBLE: c_uint = 2;
pub const KEY_TYPE_INSTANT_REAP: c_uint = 0x00000001;
pub const KEY_IS_POSITIVE: i16 = 1;
pub const KEY_IS_UNINSTANTIATED: i16 = 0;
pub const KEY_FLAG_USER_ALIVE: c_uint = 0;
pub const KEY_FLAG_DEAD: c_uint = 1;
pub const KEY_DESTROY: c_int = 0xbd;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_user {
    pub nkeys: atomic_t,
    pub nikeys: atomic_t,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

#[repr(C)]
pub struct key_tag {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_payload {
    _private: [u8; 0],
}

#[repr(C)]
pub struct key_type {
    pub name: *const c_char,
    pub flags: c_uint,
    pub destroy: Option<unsafe extern "C" fn(*mut key)>,
}

#[repr(C)]
pub struct key {
    pub serial_node: rb_node,
    pub graveyard_link: list_head,
    pub state: i16,
    pub serial: c_uint,
    pub flags: c_ulong,
    pub perm: c_uint,
    pub expiry: time64_t,
    pub type_: *mut key_type,
    pub restrict_link: *mut c_void,
    pub user: *mut key_user,
    pub domain_tag: *mut key_tag,
    pub description: *mut c_char,
    pub sem: rw_semaphore,
    pub payload: key_payload,
    #[cfg(CONFIG_KEY_NOTIFICATIONS)]
    pub watchers: *mut c_void,
}

static KEY_TYPE_DEAD_NAME: &[u8] = b".dead\0";

/*
 * Delay between key revocation/expiry in seconds
 */
#[no_mangle]
pub static mut key_gc_delay: c_uint = 5 * 60;

/*
 * Reaper for unused keys.
 */
unsafe extern "C" fn key_garbage_collector(work: *mut work_struct);

/* DECLARE_WORK(key_gc_work, key_garbage_collector); */
#[no_mangle]
pub static mut key_gc_work: work_struct = work_struct { _private: [] };

/*
 * Reaper for links from keyrings to dead keys.
 */
unsafe extern "C" fn key_gc_timer_func(unused: *mut timer_list);

/* DEFINE_TIMER(key_gc_timer, key_gc_timer_func); */
static mut key_gc_timer: timer_list = timer_list { _private: [] };

static mut key_gc_next_run: time64_t = TIME64_MAX;
static mut key_gc_dead_keytype: *mut key_type = core::ptr::null_mut();

static mut key_gc_flags: c_ulong = 0;
const KEY_GC_KEY_EXPIRED: c_uint = 0; /* A key expired and needs unlinking */
const KEY_GC_REAP_KEYTYPE: c_uint = 1; /* A keytype is being unregistered */
const KEY_GC_REAPING_KEYTYPE: c_uint = 2; /* Cleared when keytype reaped */

/*
 * Any key whose type gets unregistered will be re-typed to this if it can't be
 * immediately unlinked.
 */
#[no_mangle]
pub static mut key_type_dead: key_type = key_type {
    name: KEY_TYPE_DEAD_NAME.as_ptr() as *const c_char,
    flags: 0,
    destroy: None,
};

extern "C" {
    static mut jiffies: c_ulong;
    static mut key_serial_lock: c_void;
    static mut key_serial_tree: rb_root;
    static mut key_type_keyring: key_type;
    static mut key_jar: *mut c_void;

    fn ktime_get_real_seconds() -> time64_t;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn test_bit(nr: c_uint, addr: *const c_ulong) -> bool;
    fn test_and_clear_bit(nr: c_uint, addr: *mut c_ulong) -> bool;
    fn set_bit(nr: c_uint, addr: *mut c_ulong);
    fn clear_bit(nr: c_uint, addr: *mut c_ulong);
    fn test_bit_acquire(nr: c_uint, addr: *const c_ulong) -> bool;
    fn smp_mb();
    fn wait_on_bit(word: *mut c_ulong, bit: c_uint, mode: c_uint);
    fn wake_up_bit(word: *mut c_ulong, bit: c_uint);
    fn list_empty(head: *const list_head) -> bool;
    fn list_del(entry: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn spin_is_contended(lock: *mut c_void) -> bool;
    fn need_resched() -> bool;
    fn cond_resched();
    fn synchronize_rcu();
    fn key_check(key: *mut key);
    fn security_key_free(key: *mut key);
    fn atomic_dec(v: *mut atomic_t);
    fn key_user_put(user: *mut key_user);
    fn key_put_tag(tag: *mut key_tag);
    fn kfree(ptr: *mut c_void);
    fn memzero_explicit(ptr: *mut c_void, len: usize);
    fn kmem_cache_free(cachep: *mut c_void, objp: *mut c_void);
    fn key_serial(key: *mut key) -> c_uint;
    fn keyring_restriction_gc(key: *mut key, dead_type: *mut key_type);
    fn keyring_gc(key: *mut key, limit: time64_t);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    #[cfg(CONFIG_KEY_NOTIFICATIONS)]
    fn remove_watch_list(watchers: *mut c_void, serial: c_uint);
}

unsafe fn container_of_key_from_graveyard_link(ptr: *mut list_head) -> *mut key {
    (ptr as *mut u8).sub(core::mem::offset_of!(key, graveyard_link)) as *mut key
}

unsafe fn container_of_key_from_serial_node(ptr: *mut rb_node) -> *mut key {
    (ptr as *mut u8).sub(core::mem::offset_of!(key, serial_node)) as *mut key
}

/*
 * Schedule a garbage collection run.
 * - time precision isn't particularly important
 */
#[no_mangle]
pub unsafe extern "C" fn key_schedule_gc(gc_at: time64_t) {
    let expires: c_ulong;
    let now: time64_t = ktime_get_real_seconds();

    if gc_at <= now || test_bit(KEY_GC_REAP_KEYTYPE, core::ptr::addr_of!(key_gc_flags)) {
        schedule_work(core::ptr::addr_of_mut!(key_gc_work));
    } else if gc_at < key_gc_next_run {
        key_gc_next_run = gc_at;
        expires = jiffies.wrapping_add(((gc_at - now) as c_ulong).wrapping_mul(HZ));
        mod_timer(core::ptr::addr_of_mut!(key_gc_timer), expires);
    }
}

/*
 * Set the expiration time on a key.
 */
#[no_mangle]
pub unsafe extern "C" fn key_set_expiry(key: *mut key, expiry: time64_t) {
    (*key).expiry = expiry;
    if expiry != TIME64_MAX {
        let mut expiry = expiry;
        if !((*(*key).type_).flags & KEY_TYPE_INSTANT_REAP != 0) {
            expiry += key_gc_delay as time64_t;
        }
        key_schedule_gc(expiry);
    }
}

/*
 * Schedule a dead links collection run.
 */
#[no_mangle]
pub unsafe extern "C" fn key_schedule_gc_links() {
    set_bit(KEY_GC_KEY_EXPIRED, core::ptr::addr_of_mut!(key_gc_flags));
    schedule_work(core::ptr::addr_of_mut!(key_gc_work));
}

/*
 * Some key's cleanup time was met after it expired, so we need to get the
 * reaper to go through a cycle finding expired keys.
 */
unsafe extern "C" fn key_gc_timer_func(_unused: *mut timer_list) {
    key_gc_next_run = TIME64_MAX;
    key_schedule_gc_links();
}

/*
 * Reap keys of dead type.
 *
 * We use three flags to make sure we see three complete cycles of the garbage
 * collector: the first to mark keys of that type as being dead, the second to
 * collect dead links and the third to clean up the dead keys.  We have to be
 * careful as there may already be a cycle in progress.
 *
 * The caller must be holding key_types_sem.
 */
#[no_mangle]
pub unsafe extern "C" fn key_gc_keytype(ktype: *mut key_type) {
    key_gc_dead_keytype = ktype;
    set_bit(KEY_GC_REAPING_KEYTYPE, core::ptr::addr_of_mut!(key_gc_flags));
    smp_mb();
    set_bit(KEY_GC_REAP_KEYTYPE, core::ptr::addr_of_mut!(key_gc_flags));

    schedule_work(core::ptr::addr_of_mut!(key_gc_work));

    wait_on_bit(
        core::ptr::addr_of_mut!(key_gc_flags),
        KEY_GC_REAPING_KEYTYPE,
        TASK_UNINTERRUPTIBLE,
    );

    key_gc_dead_keytype = core::ptr::null_mut();
}

/*
 * Garbage collect a list of unreferenced, detached keys
 */
unsafe extern "C" fn key_gc_unused_keys(keys: *mut list_head) {
    while !list_empty(keys) {
        let key = container_of_key_from_graveyard_link((*keys).next);
        let state = (*key).state;

        list_del(core::ptr::addr_of_mut!((*key).graveyard_link));

        key_check(key);

        #[cfg(CONFIG_KEY_NOTIFICATIONS)]
        {
            remove_watch_list((*key).watchers, (*key).serial);
            (*key).watchers = core::ptr::null_mut();
        }

        /* Throw away the key data if the key is instantiated */
        if state == KEY_IS_POSITIVE && (*(*key).type_).destroy.is_some() {
            ((*(*key).type_).destroy.unwrap())(key);
        }

        security_key_free(key);

        atomic_dec(core::ptr::addr_of_mut!((*(*key).user).nkeys));
        if state != KEY_IS_UNINSTANTIATED {
            atomic_dec(core::ptr::addr_of_mut!((*(*key).user).nikeys));
        }

        key_user_put((*key).user);
        key_put_tag((*key).domain_tag);
        kfree((*key).description as *mut c_void);

        memzero_explicit(key as *mut c_void, core::mem::size_of::<key>());
        kmem_cache_free(key_jar, key as *mut c_void);
    }
}

/*
 * Garbage collector for unused keys.
 *
 * This is done in process context so that we don't have to disable interrupts
 * all over the place.  key_put() schedules this rather than trying to do the
 * cleanup itself, which means key_put() doesn't have to sleep.
 */
unsafe extern "C" fn key_garbage_collector(_work: *mut work_struct) {
    static mut GRAVEYARD: list_head = list_head {
        next: core::ptr::null_mut(),
        prev: core::ptr::null_mut(),
    };
    static mut gc_state: u8 = 0; /* Internal persistent state */
    const KEY_GC_REAP_AGAIN: u8 = 0x01; /* - Need another cycle */
    const KEY_GC_REAPING_LINKS: u8 = 0x02; /* - We need to reap links */
    const KEY_GC_REAPING_DEAD_1: u8 = 0x10; /* - We need to mark dead keys */
    const KEY_GC_REAPING_DEAD_2: u8 = 0x20; /* - We need to reap dead key links */
    const KEY_GC_REAPING_DEAD_3: u8 = 0x40; /* - We need to reap dead keys */
    const KEY_GC_FOUND_DEAD_KEY: u8 = 0x80; /* - We found at least one dead key */

    enum Action {
        ContinueScanning,
        Contended,
        FoundUnreferencedKey,
        FoundRestrictedKeyring,
        FoundKeyring,
        DestroyDeadKey,
        MaybeResched,
        Complete,
    }

    let mut cursor: *mut rb_node;
    let mut key: *mut key = core::ptr::null_mut();
    let mut new_timer: time64_t;
    let limit: time64_t;
    let mut expiry: time64_t;

    if GRAVEYARD.next.is_null() {
        GRAVEYARD.next = core::ptr::addr_of_mut!(GRAVEYARD);
        GRAVEYARD.prev = core::ptr::addr_of_mut!(GRAVEYARD);
    }

    limit = ktime_get_real_seconds();

    /* Work out what we're going to be doing in this pass */
    gc_state &= KEY_GC_REAPING_DEAD_1 | KEY_GC_REAPING_DEAD_2;
    gc_state <<= 1;
    if test_and_clear_bit(KEY_GC_KEY_EXPIRED, core::ptr::addr_of_mut!(key_gc_flags)) {
        gc_state |= KEY_GC_REAPING_LINKS;
    }

    if test_and_clear_bit(KEY_GC_REAP_KEYTYPE, core::ptr::addr_of_mut!(key_gc_flags)) {
        gc_state |= KEY_GC_REAPING_DEAD_1;
    }

    new_timer = TIME64_MAX;

    /* As only this function is permitted to remove things from the key
     * serial tree, if cursor is non-NULL then it will always point to a
     * valid node in the tree - even if lock got dropped.
     */
    spin_lock(core::ptr::addr_of_mut!(key_serial_lock));
    cursor = rb_first(core::ptr::addr_of_mut!(key_serial_tree));

    let mut action = Action::ContinueScanning;
    loop {
        match action {
            Action::ContinueScanning => {
                while !cursor.is_null() {
                    key = container_of_key_from_serial_node(cursor);
                    cursor = rb_next(cursor);

                    if !test_bit_acquire(KEY_FLAG_USER_ALIVE, core::ptr::addr_of!((*key).flags)) {
                        /* Clobber key->user after final put seen. */
                        action = Action::FoundUnreferencedKey;
                        break;
                    }

                    if gc_state & KEY_GC_REAPING_DEAD_1 != 0 {
                        if (*key).type_ == key_gc_dead_keytype {
                            gc_state |= KEY_GC_FOUND_DEAD_KEY;
                            set_bit(KEY_FLAG_DEAD, core::ptr::addr_of_mut!((*key).flags));
                            (*key).perm = 0;
                            if spin_is_contended(core::ptr::addr_of_mut!(key_serial_lock))
                                || need_resched()
                            {
                                action = Action::Contended;
                                break;
                            }
                            continue;
                        } else if (*key).type_ == core::ptr::addr_of_mut!(key_type_keyring)
                            && !(*key).restrict_link.is_null()
                        {
                            action = Action::FoundRestrictedKeyring;
                            break;
                        }
                    }

                    expiry = (*key).expiry;
                    if expiry != TIME64_MAX {
                        if !((*(*key).type_).flags & KEY_TYPE_INSTANT_REAP != 0) {
                            expiry += key_gc_delay as time64_t;
                        }
                        if expiry > limit && expiry < new_timer {
                            let _ = key_serial(key);
                            new_timer = (*key).expiry;
                        }
                    }

                    if gc_state & KEY_GC_REAPING_DEAD_2 != 0 {
                        if (*key).type_ == key_gc_dead_keytype {
                            gc_state |= KEY_GC_FOUND_DEAD_KEY;
                        }
                    }

                    if (gc_state & KEY_GC_REAPING_LINKS != 0)
                        || (gc_state & KEY_GC_REAPING_DEAD_2 != 0)
                    {
                        if (*key).type_ == core::ptr::addr_of_mut!(key_type_keyring) {
                            action = Action::FoundKeyring;
                            break;
                        }
                    }

                    if gc_state & KEY_GC_REAPING_DEAD_3 != 0 {
                        if (*key).type_ == key_gc_dead_keytype {
                            action = Action::DestroyDeadKey;
                            break;
                        }
                    }

                    if spin_is_contended(core::ptr::addr_of_mut!(key_serial_lock)) || need_resched()
                    {
                        action = Action::Contended;
                        break;
                    }
                }
                if cursor.is_null() {
                    action = Action::Contended;
                }
            }
            Action::Contended => {
                spin_unlock(core::ptr::addr_of_mut!(key_serial_lock));
                action = Action::MaybeResched;
            }
            Action::MaybeResched => {
                if !cursor.is_null() {
                    cond_resched();
                    spin_lock(core::ptr::addr_of_mut!(key_serial_lock));
                    action = Action::ContinueScanning;
                } else {
                    action = Action::Complete;
                }
            }
            Action::Complete => {
                /* We've completed the pass.  Set the timer if we need to and queue a
                 * new cycle if necessary.  We keep executing cycles until we find one
                 * where we didn't reap any keys.
                 */
                if new_timer != TIME64_MAX {
                    new_timer += key_gc_delay as time64_t;
                    key_schedule_gc(new_timer);
                }

                if (gc_state & KEY_GC_REAPING_DEAD_2 != 0)
                    || !list_empty(core::ptr::addr_of_mut!(GRAVEYARD))
                {
                    /* Make sure that all pending keyring payload destructions are
                     * fulfilled and that people aren't now looking at dead or
                     * dying keys that they don't have a reference upon or a link
                     * to.
                     */
                    synchronize_rcu();
                }

                if !list_empty(core::ptr::addr_of_mut!(GRAVEYARD)) {
                    key_gc_unused_keys(core::ptr::addr_of_mut!(GRAVEYARD));
                }

                if gc_state & (KEY_GC_REAPING_DEAD_1 | KEY_GC_REAPING_DEAD_2) != 0 {
                    if !(gc_state & KEY_GC_FOUND_DEAD_KEY != 0) {
                        /* No remaining dead keys: short circuit the remaining
                         * keytype reap cycles.
                         */
                        gc_state &= !(KEY_GC_REAPING_DEAD_1 | KEY_GC_REAPING_DEAD_2);
                        gc_state |= KEY_GC_REAPING_DEAD_3;
                    } else {
                        gc_state |= KEY_GC_REAP_AGAIN;
                    }
                }

                if gc_state & KEY_GC_REAPING_DEAD_3 != 0 {
                    smp_mb();
                    clear_bit(KEY_GC_REAPING_KEYTYPE, core::ptr::addr_of_mut!(key_gc_flags));
                    wake_up_bit(
                        core::ptr::addr_of_mut!(key_gc_flags),
                        KEY_GC_REAPING_KEYTYPE,
                    );
                }

                if gc_state & KEY_GC_REAP_AGAIN != 0 {
                    schedule_work(core::ptr::addr_of_mut!(key_gc_work));
                }
                return;
            }
            Action::FoundUnreferencedKey => {
                /* We found an unreferenced key - once we've removed it from the tree,
                 * we can safely drop the lock.
                 */
                rb_erase(
                    core::ptr::addr_of_mut!((*key).serial_node),
                    core::ptr::addr_of_mut!(key_serial_tree),
                );
                spin_unlock(core::ptr::addr_of_mut!(key_serial_lock));

                list_add_tail(
                    core::ptr::addr_of_mut!((*key).graveyard_link),
                    core::ptr::addr_of_mut!(GRAVEYARD),
                );
                gc_state |= KEY_GC_REAP_AGAIN;
                action = Action::MaybeResched;
            }
            Action::FoundRestrictedKeyring => {
                /* We found a restricted keyring and need to update the restriction if
                 * it is associated with the dead key type.
                 */
                spin_unlock(core::ptr::addr_of_mut!(key_serial_lock));
                keyring_restriction_gc(key, key_gc_dead_keytype);
                action = Action::MaybeResched;
            }
            Action::FoundKeyring => {
                /* We found a keyring and we need to check the payload for links to
                 * dead or expired keys.  We don't flag another reap immediately as we
                 * have to wait for the old payload to be destroyed by RCU before we
                 * can reap the keys to which it refers.
                 */
                spin_unlock(core::ptr::addr_of_mut!(key_serial_lock));
                keyring_gc(key, limit);
                action = Action::MaybeResched;
            }
            Action::DestroyDeadKey => {
                /* We found a dead key that is still referenced.  Reset its type and
                 * destroy its payload with its semaphore held.
                 */
                spin_unlock(core::ptr::addr_of_mut!(key_serial_lock));
                down_write(core::ptr::addr_of_mut!((*key).sem));
                (*key).type_ = core::ptr::addr_of_mut!(key_type_dead);
                if (*key_gc_dead_keytype).destroy.is_some() {
                    ((*key_gc_dead_keytype).destroy.unwrap())(key);
                }
                memset(
                    core::ptr::addr_of_mut!((*key).payload) as *mut c_void,
                    KEY_DESTROY,
                    core::mem::size_of::<key_payload>(),
                );
                up_write(core::ptr::addr_of_mut!((*key).sem));
                action = Action::MaybeResched;
            }
        }
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
