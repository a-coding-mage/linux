/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KMSAN API for subsystems.
 *
 * Copyright (C) 2017-2022 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_KMSAN)]
extern "C" {
    pub fn kmsan_task_create(task: *mut task_struct);
    pub fn kmsan_task_exit(task: *mut task_struct);
    pub fn kmsan_init_shadow();
    pub fn kmsan_init_runtime();
    pub fn kmsan_memblock_free_pages(page: *mut page, order: ::core::ffi::c_uint) -> bool;
    pub fn kmsan_alloc_page(page: *mut page, order: ::core::ffi::c_uint, flags: gfp_t);
    pub fn kmsan_free_page(page: *mut page, order: ::core::ffi::c_uint);
    pub fn kmsan_copy_page_meta(dst: *mut page, src: *mut page);
    pub fn kmsan_slab_alloc(s: *mut kmem_cache, object: *mut ::core::ffi::c_void, flags: gfp_t);
    pub fn kmsan_slab_free(s: *mut kmem_cache, object: *mut ::core::ffi::c_void);
    pub fn kmsan_kmalloc_large(ptr: *const ::core::ffi::c_void, size: usize, flags: gfp_t);
    pub fn kmsan_kfree_large(ptr: *const ::core::ffi::c_void);
    pub fn kmsan_vmap_pages_range_noflush(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
        prot: pgprot_t,
        pages: *mut *mut page,
        page_shift: ::core::ffi::c_uint,
        gfp_mask: gfp_t,
    ) -> ::core::ffi::c_int;
    pub fn kmsan_vunmap_range_noflush(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn kmsan_ioremap_page_range(
        addr: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
        phys_addr: phys_addr_t,
        prot: pgprot_t,
        page_shift: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn kmsan_iounmap_page_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn kmsan_handle_dma(phys: phys_addr_t, size: usize, dir: dma_data_direction);
    pub fn kmsan_handle_dma_sg(
        sg: *mut scatterlist,
        nents: ::core::ffi::c_int,
        dir: dma_data_direction,
    );
    pub fn kmsan_handle_urb(urb: *const urb, is_out: bool);
    pub fn kmsan_unpoison_entry_regs(regs: *const pt_regs);
    pub fn kmsan_get_metadata(addr: *mut ::core::ffi::c_void, is_origin: bool)
        -> *mut ::core::ffi::c_void;
    pub fn kmsan_enable_current();
    pub fn kmsan_disable_current();
    pub static mut kmsan_enabled: bool;
    pub static mut panic_on_kmsan: ::core::ffi::c_int;
}

#[cfg(CONFIG_KMSAN)]
#[inline]
pub unsafe fn memset_no_sanitize_memory(
    s: *mut ::core::ffi::c_void,
    c: ::core::ffi::c_int,
    n: usize,
) -> *mut ::core::ffi::c_void {
    __memset(s, c, n)
}

// KMSAN_WARN_ON(cond) evaluates WARN_ON(cond), disables KMSAN on warning,
// and invokes BUG when panic_on_kmsan is set. The kernel WARN_ON/BUG behavior
// is supplied by the surrounding translation.
#[cfg(CONFIG_KMSAN)]
#[inline]
pub unsafe fn kmsan_warn_on(cond: bool) -> bool {
    let __cond = WARN_ON(cond);
    if unlikely(__cond) {
        WRITE_ONCE(&mut kmsan_enabled, false);
        if panic_on_kmsan != 0 {
            BUG();
        }
    }
    __cond
}

#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_init_shadow() {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_init_runtime() {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_memblock_free_pages(_page: *mut page, _order: ::core::ffi::c_uint) -> bool { true }
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_task_create(_task: *mut task_struct) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_task_exit(_task: *mut task_struct) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_alloc_page(_page: *mut page, _order: ::core::ffi::c_uint, _flags: gfp_t) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_free_page(_page: *mut page, _order: ::core::ffi::c_uint) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_copy_page_meta(_dst: *mut page, _src: *mut page) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_slab_alloc(_s: *mut kmem_cache, _object: *mut ::core::ffi::c_void, _flags: gfp_t) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_slab_free(_s: *mut kmem_cache, _object: *mut ::core::ffi::c_void) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_kmalloc_large(_ptr: *const ::core::ffi::c_void, _size: usize, _flags: gfp_t) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_kfree_large(_ptr: *const ::core::ffi::c_void) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_vmap_pages_range_noflush(
    _start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong, _prot: pgprot_t,
    _pages: *mut *mut page, _page_shift: ::core::ffi::c_uint, _gfp_mask: gfp_t,
) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_vunmap_range_noflush(_start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_ioremap_page_range(
    _start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong, _phys_addr: phys_addr_t,
    _prot: pgprot_t, _page_shift: ::core::ffi::c_uint,
) -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_iounmap_page_range(_start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_handle_dma(_phys: phys_addr_t, _size: usize, _dir: dma_data_direction) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_handle_dma_sg(_sg: *mut scatterlist, _nents: ::core::ffi::c_int, _dir: dma_data_direction) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_handle_urb(_urb: *const urb, _is_out: bool) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_unpoison_entry_regs(_regs: *const pt_regs) {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_enable_current() {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn kmsan_disable_current() {}
#[cfg(not(CONFIG_KMSAN))]
#[inline]
pub unsafe fn memset_no_sanitize_memory(
    s: *mut ::core::ffi::c_void, c: ::core::ffi::c_int, n: usize,
) -> *mut ::core::ffi::c_void { memset(s, c, n) }

// In the disabled configuration, KMSAN_WARN_ON is WARN_ON.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
