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

// C dependencies supplied by the surrounding kernel translation.

#[cfg(any())]
pub static mut init_binfmt_misc: struct binfmt_misc = struct binfmt_misc {
    entries: HLIST_HEAD_INIT,
    enabled: true,
    entries_lock: __SPIN_LOCK_UNLOCKED,
};

/*
 * userns count is 1 for root user, 1 for init_uts_ns,
 * and 1 for... ?
 */
#[no_mangle]
pub static mut init_user_ns: struct user_namespace = struct user_namespace {
    ns: NS_COMMON_INIT,
    uid_map: {
        let mut value = Default::default();
        value.extent[0] = uid_gid_extent { first: 0, lower_first: 0, count: 4_294_967_295u32 };
        value.nr_extents = 1;
        value
    },
    gid_map: {
        let mut value = Default::default();
        value.extent[0] = uid_gid_extent { first: 0, lower_first: 0, count: 4_294_967_295u32 };
        value.nr_extents = 1;
        value
    },
    projid_map: {
        let mut value = Default::default();
        value.extent[0] = uid_gid_extent { first: 0, lower_first: 0, count: 4_294_967_295u32 };
        value.nr_extents = 1;
        value
    },
    owner: GLOBAL_ROOT_UID,
    group: GLOBAL_ROOT_GID,
    flags: USERNS_INIT_FLAGS,
    #[cfg(feature = "CONFIG_KEYS")]
    keyring_name_list: LIST_HEAD_INIT,
    #[cfg(feature = "CONFIG_KEYS")]
    keyring_sem: __RWSEM_INITIALIZER,
    #[cfg(any())]
    binfmt_misc: unsafe { &mut init_binfmt_misc },
};

#[cfg(feature = "CONFIG_BASE_SMALL")]
const UIDHASH_BITS: usize = 3;
#[cfg(not(feature = "CONFIG_BASE_SMALL"))]
const UIDHASH_BITS: usize = 7;
const UIDHASH_SZ: usize = 1usize << UIDHASH_BITS;
const UIDHASH_MASK: usize = UIDHASH_SZ - 1;

#[inline]
fn __uidhashfn(uid: u32) -> usize {
    (((uid >> UIDHASH_BITS) + uid) as usize) & UIDHASH_MASK
}

static mut uid_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut uidhash_table: [hlist_head; UIDHASH_SZ] = [/* HLIST_HEAD_INIT */ unsafe { core::mem::zeroed() }; UIDHASH_SZ];
static mut uidhash_lock: spinlock_t = unsafe { core::mem::zeroed() };

/* root_user.__count is 1, for init task cred */
#[no_mangle]
pub static mut root_user: user_struct = user_struct {
    __count: REFCOUNT_INIT(1),
    uid: GLOBAL_ROOT_UID,
    ratelimit: RATELIMIT_STATE_INIT,
};

/* These routines must be called with the uidhash spinlock held! */
unsafe fn uid_hash_insert(up: *mut user_struct, hashent: *mut hlist_head) {
    hlist_add_head(&mut (*up).uidhash_node, hashent);
}

unsafe fn uid_hash_remove(up: *mut user_struct) {
    hlist_del_init(&mut (*up).uidhash_node);
}

unsafe fn uid_hash_find(uid: kuid_t, hashent: *mut hlist_head) -> *mut user_struct {
    let mut user: *mut user_struct;
    hlist_for_each_entry!(user, hashent, uidhash_node, {
        if uid_eq((*user).uid, uid) {
            refcount_inc(&mut (*user).__count);
            return user;
        }
    });
    core::ptr::null_mut()
}

unsafe fn user_epoll_alloc(up: *mut user_struct) -> i32 {
    #[cfg(feature = "CONFIG_EPOLL")]
    { return percpu_counter_init(&mut (*up).epoll_watches, 0, GFP_KERNEL); }
    #[cfg(not(feature = "CONFIG_EPOLL"))]
    { let _ = up; 0 }
}

unsafe fn user_epoll_free(up: *mut user_struct) {
    #[cfg(feature = "CONFIG_EPOLL")]
    { percpu_counter_destroy(&mut (*up).epoll_watches); }
    #[cfg(not(feature = "CONFIG_EPOLL"))]
    { let _ = up; }
}

unsafe fn free_user(up: *mut user_struct, flags: c_ulong) {
    uid_hash_remove(up);
    spin_unlock_irqrestore(&mut uidhash_lock, flags);
    user_epoll_free(up);
    kmem_cache_free(uid_cachep, up as *mut core::ffi::c_void);
}

#[no_mangle]
pub unsafe fn find_user(uid: kuid_t) -> *mut user_struct {
    let flags: c_ulong = 0;
    spin_lock_irqsave(&mut uidhash_lock, &flags);
    let ret = uid_hash_find(uid, uidhash_table.as_mut_ptr().add(__uidhashfn(uid.val))); // uidhashentry(uid)
    spin_unlock_irqrestore(&mut uidhash_lock, flags);
    ret
}

#[no_mangle]
pub unsafe fn free_uid(up: *mut user_struct) {
    if up.is_null() { return; }
    let flags: c_ulong = 0;
    if refcount_dec_and_lock_irqsave(&mut (*up).__count, &mut uidhash_lock, &flags) {
        free_user(up, flags);
    }
}

#[no_mangle]
pub unsafe fn alloc_uid(uid: kuid_t) -> *mut user_struct {
    let hashent = uidhash_table.as_mut_ptr().add(__uidhashfn(uid.val));
    spin_lock_irq(&mut uidhash_lock);
    let mut up = uid_hash_find(uid, hashent);
    spin_unlock_irq(&mut uidhash_lock);
    if up.is_null() {
        let new = kmem_cache_zalloc(uid_cachep, GFP_KERNEL) as *mut user_struct;
        if new.is_null() { return core::ptr::null_mut(); }
        (*new).uid = uid;
        refcount_set(&mut (*new).__count, 1);
        if user_epoll_alloc(new) != 0 {
            kmem_cache_free(uid_cachep, new as *mut core::ffi::c_void);
            return core::ptr::null_mut();
        }
        ratelimit_state_init(&mut (*new).ratelimit, HZ, 100);
        ratelimit_set_flags(&mut (*new).ratelimit, RATELIMIT_MSG_ON_RELEASE);
        spin_lock_irq(&mut uidhash_lock);
        up = uid_hash_find(uid, hashent);
        if !up.is_null() {
            user_epoll_free(new);
            kmem_cache_free(uid_cachep, new as *mut core::ffi::c_void);
        } else {
            uid_hash_insert(new, hashent);
            up = new;
        }
        spin_unlock_irq(&mut uidhash_lock);
    }
    up
}

unsafe fn uid_cache_init() -> i32 {
    uid_cachep = kmem_cache_create(c"uid_cache", core::mem::size_of::<user_struct>(), 0, SLAB_HWCACHE_ALIGN | SLAB_PANIC, core::ptr::null_mut());
    let mut n = 0;
    while n < UIDHASH_SZ {
        INIT_HLIST_HEAD(uidhash_table.as_mut_ptr().add(n));
        n += 1;
    }
    if user_epoll_alloc(&mut root_user) != 0 { panic!("root_user epoll percpu counter alloc failed"); }
    spin_lock_irq(&mut uidhash_lock);
    uid_hash_insert(&mut root_user, uidhash_table.as_mut_ptr().add(__uidhashfn(GLOBAL_ROOT_UID.val)));
    spin_unlock_irq(&mut uidhash_lock);
    0
}

subsys_initcall!(uid_cache_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
