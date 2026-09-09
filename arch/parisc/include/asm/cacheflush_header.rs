/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mm.h, linux/uaccess.h, asm/tlbflush.h, and linux/jump_label.h.

/* The usual comment is "Caches aren't brain-dead on the <architecture>".
 * Unfortunately, that doesn't apply to PA-RISC. */

// DECLARE_STATIC_KEY_TRUE(parisc_has_cache);
// DECLARE_STATIC_KEY_TRUE(parisc_has_dcache);
// DECLARE_STATIC_KEY_TRUE(parisc_has_icache);
// The static-key declarations above retain their external C linkage and type.

#[inline(always)]
pub unsafe fn flush_cache_dup_mm(mm: *mut crate::mm_struct) {
    flush_cache_mm(mm);
}

unsafe extern "C" {
    pub fn flush_user_icache_range_asm(start: libc::c_ulong, end: libc::c_ulong);
    pub fn flush_kernel_icache_range_asm(start: libc::c_ulong, end: libc::c_ulong);
    pub fn flush_user_dcache_range_asm(start: libc::c_ulong, end: libc::c_ulong);
    pub fn flush_kernel_dcache_range_asm(start: libc::c_ulong, end: libc::c_ulong);
    pub fn purge_kernel_dcache_range_asm(start: libc::c_ulong, end: libc::c_ulong);
    pub fn flush_kernel_dcache_page_asm(addr: *const core::ffi::c_void);
    pub fn flush_kernel_icache_page(addr: *mut core::ffi::c_void);

    /* Cache flush operations */
    pub fn flush_cache_all_local();
    pub fn flush_cache_all();
    pub fn flush_cache_mm(mm: *mut crate::mm_struct);

    pub fn flush_kernel_vmap_range(vaddr: *mut core::ffi::c_void, size: libc::c_int);
    pub fn invalidate_kernel_vmap_range(vaddr: *mut core::ffi::c_void, size: libc::c_int);

    pub fn flush_cache_vmap(start: libc::c_ulong, end: libc::c_ulong);
    pub fn flush_cache_vunmap(start: libc::c_ulong, end: libc::c_ulong);

    pub fn flush_dcache_folio(folio: *mut crate::folio);

    pub fn flush_icache_pages(
        vma: *mut crate::vm_area_struct,
        page: *mut crate::page,
        nr: libc::c_uint,
    );

    pub fn copy_to_user_page(
        vma: *mut crate::vm_area_struct,
        page: *mut crate::page,
        user_vaddr: libc::c_ulong,
        dst: *mut core::ffi::c_void,
        src: *mut core::ffi::c_void,
        len: libc::c_int,
    );
    pub fn copy_from_user_page(
        vma: *mut crate::vm_area_struct,
        page: *mut crate::page,
        user_vaddr: libc::c_ulong,
        dst: *mut core::ffi::c_void,
        src: *mut core::ffi::c_void,
        len: libc::c_int,
    );
    pub fn flush_cache_page(
        vma: *mut crate::vm_area_struct,
        vmaddr: libc::c_ulong,
        pfn: libc::c_ulong,
    );
    pub fn flush_cache_range(
        vma: *mut crate::vm_area_struct,
        start: libc::c_ulong,
        end: libc::c_ulong,
    );

    pub fn flush_anon_page(
        vma: *mut crate::vm_area_struct,
        page: *mut crate::page,
        vmaddr: libc::c_ulong,
    );
    pub fn kunmap_flush_on_unmap(addr: *const core::ffi::c_void);
}

#[inline(always)]
pub unsafe fn flush_kernel_dcache_range(start: libc::c_ulong, size: libc::c_ulong) {
    flush_kernel_dcache_range_asm(start, start.wrapping_add(size));
}

// The only way to flush a vmap range is to flush whole cache.
pub const ARCH_IMPLEMENTS_FLUSH_KERNEL_VMAP_RANGE: libc::c_int = 1;

pub unsafe fn flush_cache_vmap_early(_start: libc::c_ulong, _end: libc::c_ulong) {}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: libc::c_int = 1;

#[inline(always)]
pub unsafe fn flush_dcache_page(page: *mut crate::page) {
    flush_dcache_folio(crate::page_folio(page));
}

// flush_dcache_folio is intentionally a self-referential macro in the C header.

pub unsafe fn flush_dcache_mmap_lock(mapping: *mut crate::address_space) {
    crate::xa_lock_irq(unsafe { &mut (*mapping).i_pages });
}

pub unsafe fn flush_dcache_mmap_unlock(mapping: *mut crate::address_space) {
    crate::xa_unlock_irq(unsafe { &mut (*mapping).i_pages });
}

pub unsafe fn flush_dcache_mmap_lock_irqsave(
    mapping: *mut crate::address_space,
    flags: *mut libc::c_ulong,
) {
    crate::xa_lock_irqsave(unsafe { &mut (*mapping).i_pages }, flags);
}

pub unsafe fn flush_dcache_mmap_unlock_irqrestore(
    mapping: *mut crate::address_space,
    flags: libc::c_ulong,
) {
    crate::xa_unlock_irqrestore(unsafe { &mut (*mapping).i_pages }, flags);
}

// flush_icache_pages is intentionally a self-referential macro in the C header.

#[inline(always)]
pub unsafe fn flush_icache_range(s: libc::c_ulong, e: libc::c_ulong) {
    flush_kernel_dcache_range_asm(s, e);
    flush_kernel_icache_range_asm(s, e);
}

pub const ARCH_HAS_FLUSH_ANON_PAGE: bool = true;
pub const ARCH_HAS_FLUSH_ON_KUNMAP: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
