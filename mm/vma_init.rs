// SPDX-License-Identifier: GPL-2.0-or-later

/*
 * Functions for initializing, allocating, freeing and duplicating VMAs. Shared
 * between CONFIG_MMU and non-CONFIG_MMU kernel configurations.
 */

// Internal and external VMA API dependencies are supplied by other files.

/* SLAB cache for vm_area_struct structures */
static mut vm_area_cachep: *mut kmem_cache = core::ptr::null_mut();

pub unsafe fn vma_state_init() {
    let args = kmem_cache_args {
        use_freeptr_offset: true,
        freeptr_offset: core::mem::offset_of!(vm_area_struct, vm_freeptr),
        sheaf_capacity: 32,
    };

    vm_area_cachep = kmem_cache_create(
        "vm_area_struct\0".as_ptr() as *const i8,
        core::mem::size_of::<vm_area_struct>(),
        &args,
        SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_TYPESAFE_BY_RCU | SLAB_ACCOUNT,
    );
}

pub unsafe fn vm_area_alloc(mm: *mut mm_struct) -> *mut vm_area_struct {
    let vma = kmem_cache_alloc(vm_area_cachep, GFP_KERNEL);
    if vma.is_null() {
        return core::ptr::null_mut();
    }

    vma_init(vma, mm);
    vma
}

unsafe fn vm_area_init_from(src: *const vm_area_struct, dest: *mut vm_area_struct) {
    (*dest).vm_mm = (*src).vm_mm;
    (*dest).vm_ops = (*src).vm_ops;
    (*dest).vm_start = (*src).vm_start;
    (*dest).vm_end = (*src).vm_end;
    (*dest).anon_vma = (*src).anon_vma;
    (*dest).vm_pgoff = vma_start_pgoff(src);
    __vma_set_anon_pgoff(dest, vma_start_anon_pgoff(src));
    (*dest).vm_file = (*src).vm_file;
    (*dest).vm_private_data = (*src).vm_private_data;
    vm_flags_init(dest, (*src).vm_flags);
    core::ptr::copy_nonoverlapping(
        &(*src).vm_page_prot,
        &mut (*dest).vm_page_prot,
        1,
    );
    /*
     * src->shared.rb may be modified concurrently when called from
     * dup_mmap(), but the clone will reinitialize it.
     */
    data_race(core::ptr::copy_nonoverlapping(
        &(*src).shared,
        &mut (*dest).shared,
        1,
    ));
    core::ptr::copy_nonoverlapping(
        &(*src).vm_userfaultfd_ctx,
        &mut (*dest).vm_userfaultfd_ctx,
        1,
    );
    #[cfg(CONFIG_ANON_VMA_NAME)]
    {
        (*dest).anon_name = (*src).anon_name;
    }
    #[cfg(CONFIG_SWAP)]
    {
        core::ptr::copy_nonoverlapping(
            &(*src).swap_readahead_info,
            &mut (*dest).swap_readahead_info,
            1,
        );
    }
    #[cfg(not(CONFIG_MMU))]
    {
        (*dest).vm_region = (*src).vm_region;
    }
    #[cfg(CONFIG_NUMA)]
    {
        (*dest).vm_policy = (*src).vm_policy;
    }
    #[cfg(__HAVE_PFNMAP_TRACKING)]
    {
        (*dest).pfnmap_track_ctx = core::ptr::null_mut();
    }
}

#[cfg(__HAVE_PFNMAP_TRACKING)]
unsafe fn vma_pfnmap_track_ctx_dup(
    orig: *mut vm_area_struct,
    new: *mut vm_area_struct,
) -> i32 {
    let ctx = (*orig).pfnmap_track_ctx;

    if ctx.is_null() {
        return 0;
    }
    if kref_read(&(*ctx).kref) >= REFCOUNT_MAX {
        return -ENOMEM;
    }
    kref_get(&mut (*ctx).kref);
    (*new).pfnmap_track_ctx = ctx;
    0
}

#[cfg(__HAVE_PFNMAP_TRACKING)]
unsafe fn vma_pfnmap_track_ctx_release(vma: *mut vm_area_struct) {
    let ctx = (*vma).pfnmap_track_ctx;
    if ctx.is_null() {
        return;
    }

    kref_put(&mut (*ctx).kref, pfnmap_track_ctx_release);
    (*vma).pfnmap_track_ctx = core::ptr::null_mut();
}

#[cfg(not(__HAVE_PFNMAP_TRACKING))]
unsafe fn vma_pfnmap_track_ctx_dup(
    _orig: *mut vm_area_struct,
    _new: *mut vm_area_struct,
) -> i32 {
    0
}

#[cfg(not(__HAVE_PFNMAP_TRACKING))]
unsafe fn vma_pfnmap_track_ctx_release(_vma: *mut vm_area_struct) {}

pub unsafe fn vm_area_dup(orig: *mut vm_area_struct) -> *mut vm_area_struct {
    let new = kmem_cache_alloc(vm_area_cachep, GFP_KERNEL);
    if new.is_null() {
        return core::ptr::null_mut();
    }

    ASSERT_EXCLUSIVE_WRITER((*orig).vm_flags);
    ASSERT_EXCLUSIVE_WRITER((*orig).vm_file);
    vm_area_init_from(orig, new);

    if vma_pfnmap_track_ctx_dup(orig, new) != 0 {
        kmem_cache_free(vm_area_cachep, new);
        return core::ptr::null_mut();
    }
    vma_lock_init(new, true);
    INIT_LIST_HEAD(&mut (*new).anon_vma_chain);
    vma_numab_state_init(new);
    dup_anon_vma_name(orig, new);

    new
}

pub unsafe fn vm_area_free(vma: *mut vm_area_struct) {
    /* The vma should be detached while being destroyed. */
    vma_assert_detached(vma);
    vma_numab_state_free(vma);
    free_anon_vma_name(vma);
    vma_pfnmap_track_ctx_release(vma);
    kmem_cache_free(vm_area_cachep, vma);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
