// SPDX-License-Identifier: GPL-2.0
// Authoritative C: mm/slub.c:249-934. Layouts are generated from native headers.
static mut slab_nodes: nodemask_t = unsafe { zeroed() };
static mut slab_barn_nodes: nodemask_t = unsafe { zeroed() };
static mut flushwq: *mut workqueue_struct = null_mut();

#[inline]
unsafe fn kmem_cache_debug(s: *mut kmem_cache) -> bool {
    rust_slub_kmem_cache_debug_flags(s, RSL_SLAB_DEBUG_FLAGS)
}
#[no_mangle]
pub unsafe extern "C" fn fixup_red_left(s: *mut kmem_cache, p: *mut Void) -> *mut Void {
    if rust_slub_kmem_cache_debug_flags(s, RSL_SLAB_RED_ZONE) {
        p.byte_add((*s).red_left_pad as usize)
    } else {
        p
    }
}
#[inline]
unsafe fn stat(s: *const kmem_cache, si: stat_item) {
    rust_slub_stat(s, si);
}
#[inline]
unsafe fn stat_add(s: *const kmem_cache, si: stat_item, v: i32) {
    rust_slub_stat_add(s, si, v);
}
#[inline]
unsafe fn get_node(s: *mut kmem_cache, node: i32) -> *mut kmem_cache_node {
    (*s).per_node[node as usize].node
}
#[inline]
unsafe fn get_barn_node(s: *mut kmem_cache, node: i32) -> *mut node_barn {
    (*s).per_node[node as usize].barn
}
#[inline]
unsafe fn get_barn(s: *mut kmem_cache) -> *mut node_barn {
    get_barn_node(s, rust_slub_numa_node_id())
}
#[inline]
unsafe fn freelist_ptr_encode(s: *const kmem_cache, ptr: *mut Void, ptr_addr: ULong) -> freeptr_t {
    #[cfg(CONFIG_SLAB_FREELIST_HARDENED)]
    let encoded = ptr as ULong ^ (*s).random ^ ptr_addr.swap_bytes();
    #[cfg(not(CONFIG_SLAB_FREELIST_HARDENED))]
    let encoded = ptr as ULong;
    freeptr_t { v: encoded }
}
#[inline]
unsafe fn freelist_ptr_decode(s: *const kmem_cache, ptr: freeptr_t, ptr_addr: ULong) -> *mut Void {
    #[cfg(CONFIG_SLAB_FREELIST_HARDENED)]
    {
        (ptr.v ^ (*s).random ^ ptr_addr.swap_bytes()) as *mut Void
    }
    #[cfg(not(CONFIG_SLAB_FREELIST_HARDENED))]
    {
        ptr.v as *mut Void
    }
}
#[inline]
unsafe fn get_freepointer(s: *mut kmem_cache, object: *mut Void) -> *mut Void {
    let object = rust_slub_kasan_reset_tag(object);
    let ptr_addr = (object as ULong).wrapping_add((*s).offset as ULong);
    freelist_ptr_decode(s, *(ptr_addr as *const freeptr_t), ptr_addr)
}
#[inline]
unsafe fn set_freepointer(s: *mut kmem_cache, object: *mut Void, fp: *mut Void) {
    let freeptr_addr = (object as ULong).wrapping_add((*s).offset as ULong);
    #[cfg(CONFIG_SLAB_FREELIST_HARDENED)]
    rust_slub_bug_on(object == fp);
    let freeptr_addr = rust_slub_kasan_reset_tag(freeptr_addr as *mut Void) as ULong;
    *(freeptr_addr as *mut freeptr_t) = freelist_ptr_encode(s, fp, freeptr_addr);
}
#[inline]
unsafe fn freeptr_outside_object(s: *mut kmem_cache) -> bool {
    (*s).offset >= (*s).inuse
}
#[inline]
unsafe fn get_info_end(s: *mut kmem_cache) -> u32 {
    (*s).inuse
        + if freeptr_outside_object(s) {
            size_of::<*mut Void>() as u32
        } else {
            0
        }
}
#[inline]
fn order_objects(order: u32, size: u32) -> u32 {
    (RSL_PAGE_SIZE << order) / size
}
#[inline]
fn oo_make(order: u32, size: u32) -> kmem_cache_order_objects {
    kmem_cache_order_objects {
        x: (order << RSL_OO_SHIFT) + order_objects(order, size),
    }
}
#[inline]
fn oo_order(x: kmem_cache_order_objects) -> u32 {
    x.x >> RSL_OO_SHIFT
}
#[inline]
fn oo_objects(x: kmem_cache_order_objects) -> u32 {
    x.x & RSL_OO_MASK
}
#[inline]
unsafe fn slab_test_pfmemalloc(slab: *const slab) -> bool {
    rust_slub_test_slab_flag(slab, SL_pfmemalloc)
}
#[inline]
unsafe fn slab_set_pfmemalloc(slab: *mut slab) {
    rust_slub_set_slab_flag(slab, SL_pfmemalloc);
}
#[inline]
unsafe fn __slab_clear_pfmemalloc(slab: *mut slab) {
    rust_slub_clear_slab_flag_nonatomic(slab, SL_pfmemalloc);
}
#[inline(always)]
unsafe fn slab_lock(slab: *mut slab) {
    rust_slub_slab_lock(slab);
}
#[inline(always)]
unsafe fn slab_unlock(slab: *mut slab) {
    rust_slub_slab_unlock(slab);
}
#[inline]
unsafe fn __update_freelist_fast(
    slab: *mut slab,
    old: *mut freelist_counters,
    new: *mut freelist_counters,
) -> bool {
    rust_slub_try_update_freelist(slab, old, new)
}
#[inline]
unsafe fn __update_freelist_slow(
    slab: *mut slab,
    old: *mut freelist_counters,
    new: *mut freelist_counters,
) -> bool {
    let mut ret = false;
    slab_lock(slab);
    if rust_slub_slab_freelist(slab) == rust_slub_fc_freelist(old)
        && rust_slub_slab_counters(slab) == rust_slub_fc_counters(old)
    {
        rust_slub_slab_set_freelist(slab, rust_slub_fc_freelist(new));
        // Native WRITE_ONCE prevents tearing against get_partial_node_bulk.
        rust_slub_slab_set_counters(slab, rust_slub_fc_counters(new));
        ret = true;
    }
    slab_unlock(slab);
    ret
}
#[inline]
unsafe fn __slab_update_freelist(
    s: *mut kmem_cache,
    slab: *mut slab,
    old: *mut freelist_counters,
    new: *mut freelist_counters,
    _n: *const CChar,
) -> bool {
    #[cfg(not(CONFIG_PREEMPT_RT))]
    rust_slub_assert_irqs_disabled();
    let ret = if (*s).flags & RSL___CMPXCHG_DOUBLE != 0 {
        __update_freelist_fast(slab, old, new)
    } else {
        __update_freelist_slow(slab, old, new)
    };
    if ret {
        return true;
    }
    rust_slub_cpu_relax();
    stat(s, CMPXCHG_DOUBLE_FAIL);
    false
}
#[inline]
unsafe fn slab_update_freelist(
    s: *mut kmem_cache,
    slab: *mut slab,
    old: *mut freelist_counters,
    new: *mut freelist_counters,
    _n: *const CChar,
) -> bool {
    let ret = if (*s).flags & RSL___CMPXCHG_DOUBLE != 0 {
        __update_freelist_fast(slab, old, new)
    } else {
        let flags = rust_slub_local_irq_save();
        let ret = __update_freelist_slow(slab, old, new);
        rust_slub_local_irq_restore(flags);
        ret
    };
    if ret {
        return true;
    }
    rust_slub_cpu_relax();
    stat(s, CMPXCHG_DOUBLE_FAIL);
    false
}
#[inline]
unsafe fn set_orig_size(s: *mut kmem_cache, object: *mut Void, orig_size: ULong) {
    if !rust_slub_slub_debug_orig_size(s) {
        return;
    }
    let p = rust_slub_kasan_reset_tag(object)
        .byte_add(get_info_end(s) as usize + 2 * size_of::<track>());
    *p.cast::<ULong>() = orig_size;
}
#[inline]
unsafe fn get_orig_size(s: *mut kmem_cache, object: *mut Void) -> ULong {
    let p = rust_slub_kasan_reset_tag(object);
    if rust_slub_is_kfence_address(object) {
        return rust_slub_kfence_ksize(object) as ULong;
    }
    if !rust_slub_slub_debug_orig_size(s) {
        return (*s).object_size as ULong;
    }
    *p.byte_add(get_info_end(s) as usize + 2 * size_of::<track>())
        .cast::<ULong>()
}
#[inline]
unsafe fn need_slab_obj_exts(s: *mut kmem_cache) -> bool {
    #[cfg(CONFIG_SLAB_OBJ_EXT)]
    {
        if (*s).flags & RSL_SLAB_NO_OBJ_EXT != 0 {
            return false;
        }
        if rust_slub_memcg_kmem_online() && (*s).flags & RSL_SLAB_ACCOUNT != 0 {
            return true;
        }
        return rust_slub_mem_alloc_profiling_enabled();
    }
    #[cfg(not(CONFIG_SLAB_OBJ_EXT))]
    {
        false
    }
}
#[inline]
unsafe fn obj_exts_size_in_slab(slab: *mut slab) -> u32 {
    #[cfg(CONFIG_SLAB_OBJ_EXT)]
    {
        rust_slub_slab_obj_ext_size(slab) * rust_slub_slab_objects(slab)
    }
    #[cfg(not(CONFIG_SLAB_OBJ_EXT))]
    {
        0
    }
}
#[inline]
unsafe fn obj_exts_offset_in_slab(s: *mut kmem_cache, slab: *mut slab) -> ULong {
    #[cfg(CONFIG_SLAB_OBJ_EXT)]
    {
        let offset = ((*s).size * rust_slub_slab_objects(slab)) as ULong;
        let align = size_of::<slabobj_ext>() as ULong;
        (offset + align - 1) & !(align - 1)
    }
    #[cfg(not(CONFIG_SLAB_OBJ_EXT))]
    {
        0
    }
}
#[inline]
unsafe fn obj_exts_fit_within_slab_leftover(s: *mut kmem_cache, slab: *mut slab) -> bool {
    #[cfg(CONFIG_SLAB_OBJ_EXT)]
    {
        obj_exts_offset_in_slab(s, slab) + obj_exts_size_in_slab(slab) as ULong
            <= rust_slub_slab_size(slab)
    }
    #[cfg(not(CONFIG_SLAB_OBJ_EXT))]
    {
        false
    }
}
#[inline]
unsafe fn obj_exts_in_slab(s: *mut kmem_cache, slab: *mut slab) -> bool {
    #[cfg(CONFIG_SLAB_OBJ_EXT)]
    {
        let exts = rust_slub_slab_obj_exts(slab);
        if exts == 0 {
            return false;
        }
        let start = rust_slub_slab_address(slab) as ULong;
        exts >= start && exts < start + rust_slub_slab_size(slab)
    }
    #[cfg(not(CONFIG_SLAB_OBJ_EXT))]
    {
        false
    }
}
unsafe fn obj_exts_offset_in_object(s: *mut kmem_cache) -> u32 {
    #[cfg(all(CONFIG_SLAB_OBJ_EXT, CONFIG_64BIT))]
    {
        let mut offset = get_info_end(s);
        if rust_slub_kmem_cache_debug_flags(s, RSL_SLAB_STORE_USER) {
            offset += (2 * size_of::<track>()) as u32;
        }
        if rust_slub_slub_debug_orig_size(s) {
            offset += size_of::<ULong>() as u32;
        }
        offset + rust_slub_kasan_metadata_size(s, false) as u32
    }
    #[cfg(not(all(CONFIG_SLAB_OBJ_EXT, CONFIG_64BIT)))]
    {
        0
    }
}
#[inline]
unsafe fn slab_set_obj_exts_in_object(slab: *mut slab) {
    #[cfg(all(CONFIG_SLAB_OBJ_EXT, CONFIG_64BIT))]
    rust_slub_slab_set_obj_exts_in_object(slab);
}
#[cfg(any(CONFIG_DEBUG_VM, CONFIG_PROVE_LOCKING))]
#[inline(never)]
#[no_mangle]
unsafe extern "C" fn slab_attach_kprobe_locked() {
    rust_slub_barrier();
}
#[cfg(not(any(CONFIG_DEBUG_VM, CONFIG_PROVE_LOCKING)))]
#[inline]
unsafe fn slab_attach_kprobe_locked() {}
// These no-op alternatives are present in the original C for disabled features.
#[cfg(not(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY))))]
unsafe fn sysfs_slab_add(s: *mut kmem_cache) -> i32 {
    0
}
#[cfg(not(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY))))]
unsafe fn slab_kset_init() -> i32 {
    0
}
#[cfg(not(all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY))))]
unsafe fn slab_sysfs_process_aliases() {}
#[cfg(not(all(CONFIG_DEBUG_FS, CONFIG_SLUB_DEBUG)))]
unsafe fn debugfs_slab_add(s: *mut kmem_cache) {}
#[cfg(not(all(CONFIG_DEBUG_FS, CONFIG_SLUB_DEBUG)))]
unsafe fn slab_debugfs_root_init() {}
