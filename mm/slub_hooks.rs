// SPDX-License-Identifier: GPL-2.0
// mm/slub.c:2084-2805: slab extensions, profiling, accounting and safety hooks.
#[inline]
unsafe fn mark_obj_codetag_empty(obj: *const Void) {
    #[cfg(all(CONFIG_SLAB_OBJ_EXT, CONFIG_MEM_ALLOC_PROFILING_DEBUG))]
    {
        if !rust_slub_obj_ext_has_codetag() {
            return;
        }
        let obj_slab = rust_slub_virt_to_slab(obj);
        let exts = rust_slub_slab_obj_exts(obj_slab);
        if exts != 0 {
            rust_slub_get_slab_obj_exts(exts);
            let ext = rust_slub_slab_obj_ext((*obj_slab).slab_cache, obj_slab, exts, obj);
            let r = rust_slub_obj_ext_codetag_ref(obj_slab, ext);
            if rust_slub_is_codetag_empty(r) {
                rust_slub_put_slab_obj_exts(exts);
                return;
            }
            rust_slub_warn_codetag_present(!(*r).ct.is_null());
            rust_slub_set_codetag_empty(r);
            rust_slub_put_slab_obj_exts(exts);
        }
    }
}
#[cfg(CONFIG_SLAB_OBJ_EXT)]
#[inline]
unsafe fn mark_failed_objexts_alloc(slab: *mut slab) -> bool {
    #[cfg(CONFIG_MEM_ALLOC_PROFILING_DEBUG)]
    {
        rust_slub_cmpxchg_ulong(addr_of_mut!((*slab).obj_exts), 0, RSL_OBJEXTS_ALLOC_FAIL) == 0
    }
    #[cfg(not(CONFIG_MEM_ALLOC_PROFILING_DEBUG))]
    {
        false
    }
}
#[cfg(CONFIG_SLAB_OBJ_EXT)]
#[inline]
unsafe fn handle_failed_objexts_alloc(slab: *mut slab, obj_exts: ULong, mut vec: *mut slabobj_ext) {
    #[cfg(CONFIG_MEM_ALLOC_PROFILING_DEBUG)]
    {
        if !rust_slub_obj_ext_has_codetag() || obj_exts != RSL_OBJEXTS_ALLOC_FAIL {
            return;
        }
        let stride = rust_slub_slab_obj_ext_size(slab) as usize / size_of::<slabobj_ext>();
        for _ in 0..rust_slub_slab_objects(slab) {
            rust_slub_set_codetag_empty(rust_slub_obj_ext_codetag_ref(slab, vec));
            vec = vec.add(stride);
        }
    }
}
#[inline]
unsafe fn init_slab_obj_exts(slab: *mut slab) {
    #[cfg(CONFIG_SLAB_OBJ_EXT)]
    {
        (*slab).obj_exts = 0;
    }
}
#[cfg(CONFIG_SLAB_OBJ_EXT)]
#[no_mangle]
pub unsafe extern "C" fn alloc_slab_obj_exts(
    slab: *mut slab,
    s: *mut kmem_cache,
    mut gfp: gfp_t,
    mut alloc_flags: u32,
) -> i32 {
    let allow_spin = alloc_flags & RSL_SLAB_ALLOC_NOLOCK == 0;
    let new_slab = alloc_flags & RSL_SLAB_ALLOC_NEW_SLAB != 0;
    let sz = rust_slub_slab_obj_ext_size(slab) as usize * rust_slub_slab_objects(slab) as usize;
    gfp &= !RSL_OBJCGS_CLEAR_MASK;
    if rust_slub_is_kmalloc_normal(s) {
        alloc_flags |= RSL_SLAB_ALLOC_NO_OBJ_EXT;
    }
    alloc_flags &= !RSL_SLAB_ALLOC_NEW_SLAB;
    let vec = rust_slub_kmalloc_flags_obj_ext(
        sz,
        gfp | RSL___GFP_ZERO,
        alloc_flags,
        rust_slub_slab_nid(slab),
    )
    .cast::<slabobj_ext>();
    if vec.is_null() {
        if !mark_failed_objexts_alloc(slab) && rust_slub_slab_obj_exts(slab) != 0 {
            return 0;
        }
        return -(ENOMEM as i32);
    }
    #[cfg(CONFIG_DEBUG_VM)]
    {
        let exts_slab = rust_slub_virt_to_slab(vec.cast());
        if !exts_slab.is_null() {
            let exts_cache = (*exts_slab).slab_cache;
            rust_slub_warn_ext_recursion(
                !rust_slub_is_kmalloc_normal(exts_cache)
                    && (*exts_cache).flags & RSL_SLAB_NO_OBJ_EXT == 0,
            );
        }
    }
    let mut new_exts = vec as ULong;
    #[cfg(CONFIG_MEMCG)]
    {
        new_exts |= RSL_MEMCG_DATA_OBJEXTS;
    }
    loop {
        let old_exts = rust_slub_read_ulong(addr_of!((*slab).obj_exts));
        handle_failed_objexts_alloc(slab, old_exts, vec);
        if new_slab {
            (*slab).obj_exts = new_exts;
            break;
        }
        if old_exts & !RSL_OBJEXTS_FLAGS_MASK != 0 {
            if !allow_spin {
                rust_slub_kfree_nolock(vec.cast());
            } else {
                kfree(vec.cast());
            }
            return 0;
        }
        if rust_slub_cmpxchg_ulong(addr_of_mut!((*slab).obj_exts), old_exts, new_exts) == old_exts {
            break;
        }
    }
    if allow_spin {
        rust_slub_kmemleak_not_leak(vec.cast());
    }
    0
}
#[cfg(not(CONFIG_SLAB_OBJ_EXT))]
unsafe fn alloc_slab_obj_exts(
    slab: *mut slab,
    s: *mut kmem_cache,
    gfp: gfp_t,
    alloc_flags: u32,
) -> i32 {
    0
}
#[inline]
unsafe fn free_slab_obj_exts(slab: *mut slab, allow_spin: bool) {
    #[cfg(CONFIG_SLAB_OBJ_EXT)]
    {
        let exts = rust_slub_slab_obj_exts(slab) as *mut slabobj_ext;
        if exts.is_null() || obj_exts_in_slab((*slab).slab_cache, slab) {
            (*slab).obj_exts = 0;
            return;
        }
        if allow_spin {
            kfree(exts.cast());
        } else {
            rust_slub_kfree_nolock(exts.cast());
        }
        (*slab).obj_exts = 0;
    }
}
unsafe fn alloc_slab_obj_exts_early(s: *mut kmem_cache, slab: *mut slab) {
    #[cfg(CONFIG_SLAB_OBJ_EXT)]
    {
        if !need_slab_obj_exts(s) {
            return;
        }
        if obj_exts_fit_within_slab_leftover(s, slab) {
            let addr = rust_slub_kasan_reset_tag(
                rust_slub_slab_address(slab).byte_add(obj_exts_offset_in_slab(s, slab) as usize),
            );
            let mut exts = addr as ULong;
            rust_slub_get_slab_obj_exts(exts);
            core::ptr::write_bytes(addr.cast::<u8>(), 0, obj_exts_size_in_slab(slab) as usize);
            rust_slub_put_slab_obj_exts(exts);
            #[cfg(CONFIG_MEMCG)]
            {
                exts |= RSL_MEMCG_DATA_OBJEXTS;
            }
            (*slab).obj_exts = exts;
        } else if (*s).flags & RSL_SLAB_OBJ_EXT_IN_OBJ != 0 {
            let offset = obj_exts_offset_in_object(s);
            let base = rust_slub_slab_address(slab);
            let mut exts = (base as ULong) + (*s).red_left_pad as ULong + offset as ULong;
            rust_slub_get_slab_obj_exts(exts);
            let end = base.byte_add(((*s).size * rust_slub_slab_objects(slab)) as usize);
            let mut addr = fixup_red_left(s, base);
            while addr < end {
                core::ptr::write_bytes(
                    rust_slub_kasan_reset_tag(addr)
                        .byte_add(offset as usize)
                        .cast::<u8>(),
                    0,
                    rust_slub_slab_obj_ext_size(slab) as usize,
                );
                addr = addr.byte_add((*s).size as usize);
            }
            rust_slub_put_slab_obj_exts(exts);
            #[cfg(CONFIG_MEMCG)]
            {
                exts |= RSL_MEMCG_DATA_OBJEXTS;
            }
            (*slab).obj_exts = exts;
            slab_set_obj_exts_in_object(slab);
        }
    }
}
#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
unsafe fn prepare_slab_obj_exts_hook(
    s: *mut kmem_cache,
    slab: *mut slab,
    flags: gfp_t,
    alloc_flags: u32,
    p: *mut Void,
) -> ULong {
    if rust_slub_slab_obj_exts(slab) == 0 {
        if rust_slub_is_kfence_address(p) {
            return 0;
        }
        if alloc_slab_obj_exts(slab, s, flags, alloc_flags) != 0 {
            rust_slub_warn_ext_alloc_failed(s);
            return 0;
        }
    }
    rust_slub_slab_obj_exts(slab)
}
#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline(never)]
unsafe fn __alloc_tagging_slab_alloc_hook(
    s: *mut kmem_cache,
    object: *mut Void,
    flags: gfp_t,
    alloc_flags: u32,
) {
    if object.is_null()
        || (*s).flags & (RSL_SLAB_NO_OBJ_EXT | RSL_SLAB_NOLEAKTRACE) != 0
        || alloc_flags & RSL_SLAB_ALLOC_NO_RECURSE != 0
    {
        return;
    }
    let slab = rust_slub_virt_to_slab(object);
    let exts = prepare_slab_obj_exts_hook(s, slab, flags, alloc_flags, object);
    if exts != 0 {
        rust_slub_get_slab_obj_exts(exts);
        let ext = rust_slub_slab_obj_ext(s, slab, exts, object);
        let r = rust_slub_obj_ext_codetag_ref(slab, ext);
        rust_slub_alloc_tag_add(r, rust_slub_current_alloc_tag(), (*s).size as usize);
        rust_slub_put_slab_obj_exts(exts);
    } else if !rust_slub_is_kfence_address(object) {
        rust_slub_alloc_tag_set_inaccurate(rust_slub_current_alloc_tag());
    }
}
#[inline]
unsafe fn alloc_tagging_slab_alloc_hook(
    s: *mut kmem_cache,
    object: *mut Void,
    flags: gfp_t,
    alloc_flags: u32,
) {
    #[cfg(CONFIG_MEM_ALLOC_PROFILING)]
    if rust_slub_mem_alloc_profiling_enabled() {
        __alloc_tagging_slab_alloc_hook(s, object, flags, alloc_flags);
    }
}
#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline(never)]
unsafe fn __alloc_tagging_slab_free_hook(
    s: *mut kmem_cache,
    slab: *mut slab,
    p: *mut *mut Void,
    objects: i32,
) {
    if (*s).flags & (RSL_SLAB_NO_OBJ_EXT | RSL_SLAB_NOLEAKTRACE) != 0 {
        return;
    }
    let exts = rust_slub_slab_obj_exts(slab);
    if exts == 0 {
        return;
    }
    rust_slub_get_slab_obj_exts(exts);
    for i in 0..objects {
        let ext = rust_slub_slab_obj_ext(s, slab, exts, *p.add(i as usize));
        rust_slub_alloc_tag_sub(rust_slub_obj_ext_codetag_ref(slab, ext), (*s).size as usize);
    }
    rust_slub_put_slab_obj_exts(exts);
}
#[inline]
unsafe fn alloc_tagging_slab_free_hook(
    s: *mut kmem_cache,
    slab: *mut slab,
    p: *mut *mut Void,
    objects: i32,
) {
    #[cfg(CONFIG_MEM_ALLOC_PROFILING)]
    if rust_slub_mem_alloc_profiling_enabled() {
        __alloc_tagging_slab_free_hook(s, slab, p, objects);
    }
}
#[cfg_attr(CONFIG_MEM_ALLOC_PROFILING, link_section = ".init.text")]
unsafe fn slab_obj_ext_has_codetag_init() {
    #[cfg(CONFIG_MEM_ALLOC_PROFILING)]
    {
        let need = !rust_slub_mem_alloc_profiling_permanently_disabled();
        if need != rust_slub_obj_ext_codetag_key_enabled() {
            if need {
                rust_slub_obj_ext_codetag_key_enable();
            } else {
                rust_slub_obj_ext_codetag_key_disable();
            }
        }
    }
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn memcg_slab_post_alloc_hook(
    s: *mut kmem_cache,
    flags: gfp_t,
    size: usize,
    p: *mut *mut Void,
    ac: *const slab_alloc_context,
) -> bool {
    #[cfg(CONFIG_MEMCG)]
    {
        if !rust_slub_memcg_kmem_online()
            || flags & RSL___GFP_ACCOUNT == 0 && (*s).flags & RSL_SLAB_ACCOUNT == 0
        {
            return true;
        }
        if __memcg_slab_post_alloc_hook(s, (*ac).lru, flags, (*ac).alloc_flags, size, p) {
            return true;
        }
        if size == 1 {
            memcg_alloc_abort_single(s, *p);
            *p = null_mut();
        } else {
            kmem_cache_free_bulk(s, size, p);
        }
        return false;
    }
    #[cfg(not(CONFIG_MEMCG))]
    {
        true
    }
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn memcg_slab_free_hook(
    s: *mut kmem_cache,
    slab: *mut slab,
    p: *mut *mut Void,
    objects: i32,
) {
    #[cfg(CONFIG_MEMCG)]
    {
        if !rust_slub_memcg_kmem_online() {
            return;
        }
        let exts = rust_slub_slab_obj_exts(slab);
        if exts == 0 || !rust_slub_slab_needs_objcg(slab) {
            return;
        }
        rust_slub_get_slab_obj_exts(exts);
        __memcg_slab_free_hook(s, slab, p, objects, exts);
        rust_slub_put_slab_obj_exts(exts);
    }
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn memcg_slab_post_charge(mut p: *mut Void, flags: gfp_t) -> bool {
    #[cfg(CONFIG_MEMCG)]
    {
        let page = rust_slub_virt_to_page(p);
        if rust_slub_page_large_kmalloc(page) {
            if rust_slub_page_memcg_kmem(page) {
                return true;
            }
            let order = rust_slub_large_kmalloc_order(page);
            if __memcg_kmem_charge_page(page, flags, order) != 0 {
                return false;
            }
            let size = ((RSL_PAGE_SIZE as u32) << order) as i32;
            rust_slub_mod_node_page_state(
                rust_slub_page_pgdat(page),
                NR_SLAB_UNRECLAIMABLE_B,
                -(size as Long),
            );
            rust_slub_mod_lruvec_page_state(page, NR_SLAB_UNRECLAIMABLE_B, size as i32);
            return true;
        }
        let slab = rust_slub_page_slab(page);
        let s = (*slab).slab_cache;
        if !rust_slub_cache_needs_objcg(s) {
            return true;
        }
        let exts = rust_slub_slab_obj_exts(slab);
        if exts != 0 {
            rust_slub_get_slab_obj_exts(exts);
            let ext = rust_slub_slab_obj_ext(s, slab, exts, p);
            if !rust_slub_obj_ext_objcg(slab, ext).is_null() {
                rust_slub_put_slab_obj_exts(exts);
                return true;
            }
            rust_slub_put_slab_obj_exts(exts);
        }
        return __memcg_slab_post_alloc_hook(
            s,
            null_mut(),
            flags,
            RSL_SLAB_ALLOC_DEFAULT,
            1,
            &mut p,
        );
    }
    #[cfg(not(CONFIG_MEMCG))]
    {
        true
    }
}
#[inline(always)]
unsafe fn slab_free_hook(
    s: *mut kmem_cache,
    x: *mut Void,
    init: bool,
    after_rcu_delay: bool,
) -> bool {
    let still_accessible = (*s).flags & RSL_SLAB_TYPESAFE_BY_RCU != 0 && !after_rcu_delay;
    rust_slub_kmemleak_free_recursive(x, (*s).flags);
    rust_slub_kmsan_slab_free(s, x);
    rust_slub_debug_check_no_locks_freed(x, (*s).object_size as ULong);
    if (*s).flags & RSL_SLAB_DEBUG_OBJECTS == 0 {
        rust_slub_debug_check_no_obj_freed(x, (*s).object_size as ULong);
    }
    if !still_accessible {
        rust_slub_kcsan_check_free(x, (*s).object_size as usize);
    }
    if rust_slub_kfence_free(x) {
        return false;
    }
    if rust_slub_kasan_slab_pre_free(s, x) {
        return false;
    }
    #[cfg(CONFIG_SLUB_RCU_DEBUG)]
    if still_accessible {
        let delayed = rust_slub_kmalloc_rcu_delayed().cast::<rcu_delayed_free>();
        if !delayed.is_null() {
            rust_slub_kasan_record_aux_stack(x);
            (*delayed).object = x;
            call_rcu(
                addr_of_mut!((*delayed).head),
                Some(slab_free_after_rcu_debug),
            );
            return false;
        }
    }
    if init {
        let inuse = get_info_end(s);
        let orig_size = get_orig_size(s, x) as u32;
        if !rust_slub_kasan_has_integrated_init() {
            core::ptr::write_bytes(
                rust_slub_kasan_reset_tag(x).cast::<u8>(),
                0,
                orig_size as usize,
            );
        }
        let rsize = if (*s).flags & RSL_SLAB_RED_ZONE != 0 {
            (*s).red_left_pad
        } else {
            0
        };
        core::ptr::write_bytes(
            rust_slub_kasan_reset_tag(x)
                .byte_add(inuse as usize)
                .cast::<u8>(),
            0,
            ((*s).size - inuse - rsize) as usize,
        );
        set_orig_size(s, x, orig_size as ULong);
    }
    !rust_slub_kasan_slab_free(s, x, init, still_accessible, false)
}
#[cfg_attr(not(CONFIG_SLUB_TINY), inline(always))]
unsafe fn slab_free_freelist_hook(
    s: *mut kmem_cache,
    head: *mut *mut Void,
    tail: *mut *mut Void,
    cnt: *mut i32,
) -> bool {
    let mut next = *head;
    let old_tail = *tail;
    if rust_slub_is_kfence_address(next) {
        slab_free_hook(s, next, false, false);
        return false;
    }
    *head = null_mut();
    *tail = null_mut();
    let init = rust_slub_slab_want_init_on_free(s);
    loop {
        let object = next;
        next = get_freepointer(s, object);
        if slab_free_hook(s, object, init, false) {
            set_freepointer(s, object, *head);
            *head = object;
            if (*tail).is_null() {
                *tail = object;
            }
        } else {
            *cnt -= 1;
        }
        if object == old_tail {
            break;
        }
    }
    !(*head).is_null()
}
#[inline]
unsafe fn setup_object(s: *mut kmem_cache, object: *mut Void) -> *mut Void {
    setup_object_debug(s, object);
    let object = rust_slub_kasan_init_slab_obj(s, object);
    if let Some(ctor) = (*s).ctor {
        rust_slub_kasan_unpoison_new_object(s, object);
        ctor(object);
        rust_slub_kasan_poison_new_object(s, object);
    }
    object
}
