// SPDX-License-Identifier: GPL-2.0
/* Slab allocator functions independent of allocator strategy. */

// Kernel headers and configuration macros are supplied by the surrounding Rust
// translation. Their declarations are intentionally not reimplemented here.

static mut slab_state: slab_state = slab_state::default();
static mut slab_caches: list_head = list_head::default();
static mut slab_mutex: mutex = mutex::default();
static mut kmem_cache: *mut kmem_cache = core::ptr::null_mut();

const SLAB_NEVER_MERGE: slab_flags_t = SLAB_DEBUG_FLAGS | SLAB_TYPESAFE_BY_RCU |
    SLAB_NOLEAKTRACE | SLAB_FAILSLAB | SLAB_NO_MERGE | SLAB_OBJ_EXT_IN_OBJ;
const SLAB_MERGE_SAME: slab_flags_t = SLAB_RECLAIM_ACCOUNT | SLAB_CACHE_DMA |
    SLAB_CACHE_DMA32 | SLAB_ACCOUNT | SLAB_MAY_ACCOUNT;

static mut slab_nomerge: bool = !IS_ENABLED(CONFIG_SLAB_MERGE_DEFAULT);

unsafe fn setup_slab_nomerge(_str: *mut c_char) -> c_int { slab_nomerge = true; 1 }
unsafe fn setup_slab_merge(_str: *mut c_char) -> c_int { slab_nomerge = false; 1 }

unsafe fn kmem_cache_size(s: *mut kmem_cache) -> c_uint { (*s).object_size }

#[cfg(CONFIG_DEBUG_VM)]
unsafe fn kmem_cache_is_duplicate_name(name: *const c_char) -> bool {
    let mut s: *mut kmem_cache;
    list_for_each_entry!(s, &mut slab_caches, list, {
        if strcmp((*s).name, name) == 0 { return true; }
    });
    false
}

#[cfg(CONFIG_DEBUG_VM)]
unsafe fn kmem_cache_sanity_check(name: *const c_char, size: c_uint) -> c_int {
    if name.is_null() || in_interrupt() || size > KMALLOC_MAX_SIZE {
        pr_err!("kmem_cache_create(%s) integrity check failed\\n", name);
        return -EINVAL;
    }
    WARN!(kmem_cache_is_duplicate_name(name), "kmem_cache of name '%s' already exists\\n", name);
    WARN_ON!(strchr(name, b' ' as c_int) != core::ptr::null());
    0
}
#[cfg(not(CONFIG_DEBUG_VM))]
unsafe fn kmem_cache_sanity_check(_name: *const c_char, _size: c_uint) -> c_int { 0 }

unsafe fn calculate_alignment(flags: slab_flags_t, mut align: c_uint, size: c_uint) -> c_uint {
    if flags & SLAB_HWCACHE_ALIGN != 0 {
        let mut ralign = cache_line_size();
        while size <= ralign / 2 { ralign /= 2; }
        align = max(align, ralign);
    }
    align = max(align, arch_slab_minalign());
    ALIGN(align, core::mem::size_of::<*mut c_void>() as c_uint)
}

unsafe fn slab_unmergeable(s: *mut kmem_cache) -> c_int {
    if slab_nomerge || (*s).flags & SLAB_NEVER_MERGE != 0 { return 1; }
    if !(*s).ctor.is_null() { return 1; }
    if IS_ENABLED(CONFIG_HARDENED_USERCOPY) && (*s).usersize != 0 { return 1; }
    if (*s).refcount < 0 { return 1; }
    0
}

unsafe fn slab_args_unmergeable(args: *mut kmem_cache_args, flags: slab_flags_t) -> bool {
    if slab_nomerge || !(*args).ctor.is_null() { return true; }
    if IS_ENABLED(CONFIG_HARDENED_USERCOPY) && (*args).usersize != 0 { return true; }
    flags & SLAB_NEVER_MERGE != 0
}

unsafe fn find_mergeable(mut size: c_uint, mut flags: slab_flags_t, name: *const c_char,
                         args: *mut kmem_cache_args) -> *mut kmem_cache {
    flags = kmem_cache_flags(flags, name);
    if slab_args_unmergeable(args, flags) { return core::ptr::null_mut(); }
    size = ALIGN(size, core::mem::size_of::<*mut c_void>() as c_uint);
    let align = calculate_alignment(flags, (*args).align, size);
    size = ALIGN(size, align);
    let mut s: *mut kmem_cache;
    list_for_each_entry_reverse!(s, &mut slab_caches, list, {
        if slab_unmergeable(s) != 0 || size > (*s).size { continue; }
        if (flags & SLAB_MERGE_SAME) != ((*s).flags & SLAB_MERGE_SAME) { continue; }
        if ((*s).size & !(align - 1)) != (*s).size || (*s).size - size >= core::mem::size_of::<*mut c_void>() as c_uint { continue; }
        return s;
    });
    core::ptr::null_mut()
}

unsafe fn create_cache(name: *const c_char, object_size: c_uint, args: *mut kmem_cache_args,
                       flags: slab_flags_t) -> *mut kmem_cache {
    if (*args).use_freeptr_offset && ((*args).freeptr_offset >= object_size ||
       (flags & SLAB_TYPESAFE_BY_RCU == 0 && (*args).ctor.is_null()) ||
       !IS_ALIGNED((*args).freeptr_offset, core::mem::align_of::<freeptr_t>() as c_uint)) {
        return ERR_PTR(-EINVAL);
    }
    let s = kmem_cache_zalloc(kmem_cache, GFP_KERNEL);
    if s.is_null() { return ERR_PTR(-ENOMEM); }
    let err = do_kmem_cache_create(s, name, object_size, args, flags);
    if err != 0 { kmem_cache_free(kmem_cache, s); return ERR_PTR(err); }
    (*s).refcount = 1;
    list_add(&mut (*s).list, &mut slab_caches);
    s
}

unsafe fn __kmem_cache_alias(name: *const c_char, size: c_uint, flags: slab_flags_t,
                             args: *mut kmem_cache_args) -> *mut kmem_cache {
    let s = find_mergeable(size, flags, name, args);
    if !s.is_null() {
        if sysfs_slab_alias(s, name) != 0 { pr_err!("SLUB: Unable to add cache alias %s to sysfs\\n", name); }
        (*s).refcount += 1;
        (*s).object_size = max((*s).object_size, size);
        (*s).inuse = max((*s).inuse, ALIGN(size, core::mem::size_of::<*mut c_void>() as c_uint));
    }
    s
}

unsafe fn __kmem_cache_create_args(name: *const c_char, object_size: c_uint,
                                    args: *mut kmem_cache_args, mut flags: slab_flags_t) -> *mut kmem_cache {
    #[cfg(CONFIG_SLUB_DEBUG)] { if flags & SLAB_DEBUG_FLAGS != 0 { static_branch_enable(&slub_debug_enabled); } if flags & SLAB_STORE_USER != 0 { stack_depot_init(); } }
    #[cfg(not(CONFIG_SLUB_DEBUG))] { flags &= !SLAB_DEBUG_FLAGS; }
    if (*args).sheaf_capacity != 0 { flags |= SLAB_NO_MERGE; }
    mutex_lock(&mut slab_mutex);
    let mut err = kmem_cache_sanity_check(name, object_size);
    if err == 0 && flags & !SLAB_FLAGS_PERMITTED != 0 { err = -EINVAL; }
    if err == 0 {
        if !mem_cgroup_kmem_disabled() { flags |= SLAB_MAY_ACCOUNT; }
        if !IS_ENABLED(CONFIG_HARDENED_USERCOPY) || WARN_ON!((*args).usersize == 0 && (*args).useroffset != 0) || WARN_ON!(object_size < (*args).usersize || object_size - (*args).usersize < (*args).useroffset) { (*args).usersize = 0; (*args).useroffset = 0; }
        let mut s = __kmem_cache_alias(name, object_size, flags, args);
        if s.is_null() {
            let cache_name = kstrdup_const(name, GFP_KERNEL);
            if cache_name.is_null() { err = -ENOMEM; } else { (*args).align = calculate_alignment(flags, (*args).align, object_size); s = create_cache(cache_name, object_size, args, flags); if IS_ERR(s) { err = PTR_ERR(s); kfree_const(cache_name); } }
        }
        mutex_unlock(&mut slab_mutex);
        if err == 0 { return s; }
    } else { mutex_unlock(&mut slab_mutex); }
    if flags & SLAB_PANIC != 0 { panic!("{}: Failed to create slab '{}'. Error {}\\n", "__kmem_cache_create_args", name, err); }
    pr_warn!("__kmem_cache_create_args(%s) failed with error %d\\n", name, err); dump_stack();
    core::ptr::null_mut()
}

static mut kmem_buckets_cache: *mut kmem_cache = core::ptr::null_mut();
static mut kmalloc_caches: kmem_buckets = kmem_buckets::default();
#[cfg(CONFIG_KMALLOC_PARTITION_RANDOM)] static mut random_kmalloc_seed: c_ulong = 0;

unsafe fn kmem_buckets_create(name: *const c_char, mut flags: slab_flags_t,
                              useroffset: c_uint, usersize: c_uint,
                              ctor: Option<unsafe extern "C" fn(*mut c_void)>) -> *mut kmem_buckets {
    let mut mask: c_ulong = 0;
    BUILD_BUG_ON!(ARRAY_SIZE!(kmalloc_caches[KMALLOC_NORMAL]) > BITS_PER_LONG);
    if !IS_ENABLED(CONFIG_SLAB_BUCKETS) { return ZERO_SIZE_PTR as *mut kmem_buckets; }
    if WARN_ON!(kmem_buckets_cache.is_null()) { return core::ptr::null_mut(); }
    let b = kmem_cache_alloc(kmem_buckets_cache, GFP_KERNEL | __GFP_ZERO) as *mut kmem_buckets;
    if WARN_ON!(b.is_null()) { return core::ptr::null_mut(); }
    flags |= SLAB_NO_MERGE;
    let mut idx = 0;
    while idx < ARRAY_SIZE!(kmalloc_caches[KMALLOC_NORMAL]) {
        let cache = kmalloc_caches[KMALLOC_NORMAL][idx]; if cache.is_null() { idx += 1; continue; }
        let size = (*cache).object_size; if size == 0 { idx += 1; continue; }
        let short_size = strchr((*cache).name, b'-' as c_int); if WARN_ON!(short_size.is_null()) { break; }
        let (cache_useroffset, cache_usersize) = if useroffset >= size { (0, 0) } else { (useroffset, min(size-useroffset, usersize)) };
        let aligned_idx = __kmalloc_index(size, false);
        if (*b)[aligned_idx].is_null() {
            let cache_name = kasprintf(GFP_KERNEL, "%s-%s", name, short_size.add(1)); if WARN_ON!(cache_name.is_null()) { break; }
            (*b)[aligned_idx] = kmem_cache_create_usercopy(cache_name, size, 0, flags, cache_useroffset, cache_usersize, ctor); kfree(cache_name);
            if WARN_ON!((*b)[aligned_idx].is_null()) { break; } mask |= 1 << aligned_idx;
        }
        if idx != aligned_idx { (*b)[idx] = (*b)[aligned_idx]; }
        idx += 1;
    }
    if idx == ARRAY_SIZE!(kmalloc_caches[KMALLOC_NORMAL]) { return b; }
    let mut bit = 0; while bit < ARRAY_SIZE!(kmalloc_caches[KMALLOC_NORMAL]) { if mask & (1 << bit) != 0 { kmem_cache_destroy((*b)[bit]); } bit += 1; }
    kmem_cache_free(kmem_buckets_cache, b); core::ptr::null_mut()
}

#[cfg(CONFIG_PRINTK)]
unsafe fn kmem_dump_obj(object: *mut c_void) -> bool {
    if object < PAGE_SIZE as *mut c_void || !virt_addr_valid(object) { return false; }
    let slab = virt_to_slab(object); if slab.is_null() { return false; }
    let mut kp = kmem_obj_info::default(); if __kfence_obj_info(&mut kp, object, slab) == 0 { __kmem_obj_info(&mut kp, object, slab); }
    let cp = if IS_ENABLED(CONFIG_MMU) { "" } else { "/vmalloc" };
    if !kp.kp_slab_cache.is_null() { pr_cont!(" slab{} {}", cp, (*kp.kp_slab_cache).name); } else { pr_cont!(" slab{}", cp); }
    if is_kfence_address(object) { pr_cont!(" (kfence)"); }
    if !kp.kp_objp.is_null() { pr_cont!(" start %px", kp.kp_objp); }
    if kp.kp_data_offset != 0 { pr_cont!(" data offset {}", kp.kp_data_offset); }
    if !kp.kp_objp.is_null() { pr_cont!(" pointer offset {}", (object as usize - kp.kp_objp as usize) - kp.kp_data_offset); }
    if !kp.kp_slab_cache.is_null() && (*kp.kp_slab_cache).object_size != 0 { pr_cont!(" size {}", (*kp.kp_slab_cache).object_size); }
    if !kp.kp_ret.is_null() { pr_cont!(" allocated at %pS\\n", kp.kp_ret); } else { pr_cont!("\\n"); }
    true
}

// The remaining exported helpers retain the C kernel ABI and configuration
// dependent bucket/cache layout; declarations below mirror their definitions.
unsafe fn slab_kmem_cache_release(s: *mut kmem_cache) { __kmem_cache_release(s); kfree_const((*s).name); kmem_cache_free(kmem_cache, s); }
unsafe fn kmem_cache_release(s: *mut kmem_cache) { kfence_shutdown_cache(s); if __is_defined(SLAB_SUPPORTS_SYSFS) && slab_state >= slab_state::FULL { sysfs_slab_release(s); } else { slab_kmem_cache_release(s); } }
unsafe fn kmem_cache_destroy(s: *mut kmem_cache) {
    if s.is_null() || !kasan_check_byte(s) { return; }
    kvfree_rcu_barrier_on_cache(s); if IS_ENABLED(CONFIG_SLUB_RCU_DEBUG) && (*s).flags & SLAB_TYPESAFE_BY_RCU != 0 { rcu_barrier(); }
    deferred_work_barrier(); cpus_read_lock(); mutex_lock(&mut slab_mutex); (*s).refcount -= 1;
    if (*s).refcount != 0 { mutex_unlock(&mut slab_mutex); cpus_read_unlock(); return; }
    kasan_cache_shutdown(s); let err = __kmem_cache_shutdown(s); list_del(&mut (*s).list);
    mutex_unlock(&mut slab_mutex); cpus_read_unlock(); if slab_state >= slab_state::FULL { sysfs_slab_unlink(s); } debugfs_slab_release(s); if err != 0 { return; }
    if (*s).flags & SLAB_TYPESAFE_BY_RCU != 0 { rcu_barrier(); } kmem_cache_release(s);
}
unsafe fn kmem_cache_shrink(cachep: *mut kmem_cache) -> c_int { kasan_cache_shrink(cachep); __kmem_cache_shrink(cachep) }
unsafe fn slab_is_available() -> bool { slab_state >= slab_state::UP }

unsafe fn create_boot_cache(s: *mut kmem_cache, name: *const c_char, size: c_uint, flags: slab_flags_t, useroffset: c_uint, usersize: c_uint) {
    let mut align = ARCH_KMALLOC_MINALIGN; if flags & SLAB_KMALLOC != 0 { align = max(align, 1 << (ffs(size) - 1)); }
    let mut args = kmem_cache_args::default(); args.align = calculate_alignment(flags, align, size); if IS_ENABLED(CONFIG_HARDENED_USERCOPY) { args.useroffset = useroffset; args.usersize = usersize; }
    let err = do_kmem_cache_create(s, name, size, &mut args, flags); if err != 0 { panic!("Creation of kmalloc slab {} size={} failed. Reason {}\\n", name, size, err); } (*s).refcount = -1;
}

unsafe fn create_kmalloc_cache(name: *const c_char, size: c_uint, flags: slab_flags_t) -> *mut kmem_cache {
    let s = kmem_cache_zalloc(kmem_cache, GFP_NOWAIT); if s.is_null() { panic!("Out of memory when creating slab {}\\n", name); }
    create_boot_cache(s, name, size, flags | SLAB_KMALLOC, 0, size); list_add(&mut (*s).list, &mut slab_caches); (*s).refcount = 1; s
}

#[no_mangle] pub static mut kmalloc_size_index: [u8; 24] = [3,4,5,5,6,6,6,6,1,1,1,1,7,7,7,7,2,2,2,2,2,2,2,2];

unsafe fn kmalloc_size_roundup(size: usize) -> usize {
    if size != 0 && size <= KMALLOC_MAX_CACHE_SIZE { return (*kmalloc_slab(size, core::ptr::null_mut(), GFP_KERNEL, __kmalloc_token(0), SLAB_ALLOC_DEFAULT)).object_size as usize; }
    if size != 0 && size <= KMALLOC_MAX_SIZE { return PAGE_SIZE << get_order(size); }
    size
}

unsafe fn setup_kmalloc_cache_index_table() {
    BUILD_BUG_ON!(KMALLOC_MIN_SIZE > 256 || !is_power_of_2(KMALLOC_MIN_SIZE));
    let mut i = 8; while i < KMALLOC_MIN_SIZE { let elem = size_index_elem(i); if elem >= 24 { break; } kmalloc_size_index[elem] = KMALLOC_SHIFT_LOW; i += 8; }
    if KMALLOC_MIN_SIZE >= 64 { let mut j = 72; while j <= 96 { kmalloc_size_index[size_index_elem(j)] = 7; j += 8; } }
    if KMALLOC_MIN_SIZE >= 128 { let mut j = 136; while j <= 192 { kmalloc_size_index[size_index_elem(j)] = 8; j += 8; } }
}

unsafe fn __kmalloc_minalign() -> c_uint {
    let mut minalign = dma_get_cache_alignment(); if IS_ENABLED(CONFIG_DMA_BOUNCE_UNALIGNED_KMALLOC) && is_swiotlb_allocated() { minalign = ARCH_KMALLOC_MINALIGN; } max(minalign, arch_slab_minalign())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
