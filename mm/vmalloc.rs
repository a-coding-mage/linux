// SPDX-License-Identifier: GPL-2.0-only
/*
 * Low-level Rust translation of vmalloc.c.
 *
 * The Linux kernel types and helper functions referenced by this translation
 * are supplied by the surrounding kernel environment.
 */

#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
static mut IOREMAP_MAX_PAGE_SHIFT: libc::c_uint = (BITS_PER_LONG - 1) as libc::c_uint;
#[cfg(not(CONFIG_HAVE_ARCH_HUGE_VMAP))]
static IOREMAP_MAX_PAGE_SHIFT: libc::c_uint = PAGE_SHIFT;

#[cfg(CONFIG_HAVE_ARCH_HUGE_VMALLOC)]
static mut VMAP_ALLOW_HUGE: bool = true;
#[cfg(not(CONFIG_HAVE_ARCH_HUGE_VMALLOC))]
static VMAP_ALLOW_HUGE: bool = false;

pub unsafe extern "C" fn is_vmalloc_addr(x: *const core::ffi::c_void) -> bool {
    let addr = kasan_reset_tag(x) as usize;
    addr >= VMALLOC_START && addr < VMALLOC_END
}

pub unsafe extern "C" fn vmap_page_range(
    addr: libc::c_ulong, end: libc::c_ulong, phys_addr: phys_addr_t, prot: pgprot_t,
) -> libc::c_int {
    let err = vmap_range_noflush(addr, end, pgprot_nx(prot), IOREMAP_MAX_PAGE_SHIFT);
    flush_cache_vmap(addr, end);
    if err == 0 {
        kmsan_ioremap_page_range(addr, end, phys_addr, prot, IOREMAP_MAX_PAGE_SHIFT)
    } else { err }
}

pub unsafe extern "C" fn ioremap_page_range(
    addr: libc::c_ulong, end: libc::c_ulong, phys_addr: phys_addr_t, prot: pgprot_t,
) -> libc::c_int {
    let area = find_vm_area(addr as *mut core::ffi::c_void);
    if area.is_null() || ((*area).flags & VM_IOREMAP) == 0 { return -EINVAL; }
    if addr != (*area).addr as libc::c_ulong ||
       end as *mut core::ffi::c_void != (*area).addr.add(get_vm_area_size(area)) {
        return -ERANGE;
    }
    vmap_page_range(addr, end, phys_addr, prot)
}

pub unsafe extern "C" fn vunmap_range_noflush(start: libc::c_ulong, end: libc::c_ulong) {
    kmsan_vunmap_range_noflush(start, end);
    __vunmap_range_noflush(start, end);
}

pub unsafe extern "C" fn vunmap_range(addr: libc::c_ulong, end: libc::c_ulong) {
    flush_cache_vunmap(addr, end);
    vunmap_range_noflush(addr, end);
    flush_tlb_kernel_range(addr, end);
}

pub unsafe extern "C" fn is_vmalloc_or_module_addr(x: *const core::ffi::c_void) -> libc::c_int {
    #[cfg(all(CONFIG_EXECMEM, MODULES_VADDR))]
    {
        let addr = kasan_reset_tag(x) as usize;
        if addr >= MODULES_VADDR && addr < MODULES_END { return 1; }
    }
    if is_vmalloc_addr(x) { 1 } else { 0 }
}

// The remaining page-table and global KVA allocator routines retain the
// declarations and layouts supplied by the kernel's vmalloc implementation.
extern "C" {
    fn vmap_range_noflush(addr: libc::c_ulong, end: libc::c_ulong, phys_addr: phys_addr_t,
                          prot: pgprot_t, max_page_shift: libc::c_uint) -> libc::c_int;
    fn __vunmap_range_noflush(start: libc::c_ulong, end: libc::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
