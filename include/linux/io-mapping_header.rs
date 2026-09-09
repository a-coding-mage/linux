/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright © 2008 Keith Packard <keithp@keithp.com>
 */

/* Dependencies supplied by the surrounding kernel translation are intentionally
 * referenced here rather than reimplemented. */

/*
 * The io_mapping mechanism provides an abstraction for mapping
 * individual pages from an io device to the CPU in an efficient fashion.
 *
 * See Documentation/driver-api/io-mapping.rst
 */

#[repr(C)]
pub struct io_mapping {
    pub base: resource_size_t,
    pub size: c_ulong,
    pub prot: pgprot_t,
    pub iomem: *mut core::ffi::c_void,
}

#[cfg(CONFIG_HAVE_ATOMIC_IOMAP)]
pub unsafe extern "C" fn io_mapping_init_wc(
    iomap: *mut io_mapping,
    base: resource_size_t,
    size: c_ulong,
) -> *mut io_mapping {
    let mut prot: pgprot_t = core::mem::zeroed();
    if iomap_create_wc(base, size, &mut prot) != 0 {
        return core::ptr::null_mut();
    }
    (*iomap).base = base;
    (*iomap).size = size;
    (*iomap).prot = prot;
    iomap
}

#[cfg(CONFIG_HAVE_ATOMIC_IOMAP)]
pub unsafe extern "C" fn io_mapping_fini(mapping: *mut io_mapping) {
    iomap_free((*mapping).base, (*mapping).size);
}

#[cfg(CONFIG_HAVE_ATOMIC_IOMAP)]
pub unsafe extern "C" fn io_mapping_map_atomic_wc(
    mapping: *mut io_mapping,
    offset: c_ulong,
) -> *mut core::ffi::c_void {
    BUG_ON(offset >= (*mapping).size);
    let phys_addr: resource_size_t = (*mapping).base + offset;
    if !IS_ENABLED(CONFIG_PREEMPT_RT) { preempt_disable(); } else { migrate_disable(); }
    pagefault_disable();
    __iomap_local_pfn_prot(PHYS_PFN(phys_addr), (*mapping).prot)
}

#[cfg(CONFIG_HAVE_ATOMIC_IOMAP)]
pub unsafe extern "C" fn io_mapping_unmap_atomic(vaddr: *mut core::ffi::c_void) {
    kunmap_local_indexed(vaddr);
    pagefault_enable();
    if !IS_ENABLED(CONFIG_PREEMPT_RT) { preempt_enable(); } else { migrate_enable(); }
}

#[cfg(CONFIG_HAVE_ATOMIC_IOMAP)]
pub unsafe extern "C" fn io_mapping_map_local_wc(mapping: *mut io_mapping, offset: c_ulong) -> *mut core::ffi::c_void {
    BUG_ON(offset >= (*mapping).size);
    let phys_addr: resource_size_t = (*mapping).base + offset;
    __iomap_local_pfn_prot(PHYS_PFN(phys_addr), (*mapping).prot)
}

#[cfg(CONFIG_HAVE_ATOMIC_IOMAP)]
pub unsafe extern "C" fn io_mapping_unmap_local(vaddr: *mut core::ffi::c_void) { kunmap_local_indexed(vaddr); }

#[cfg(CONFIG_HAVE_ATOMIC_IOMAP)]
pub unsafe extern "C" fn io_mapping_map_wc(mapping: *mut io_mapping, offset: c_ulong, size: c_ulong) -> *mut core::ffi::c_void {
    BUG_ON(offset >= (*mapping).size);
    ioremap_wc((*mapping).base + offset, size)
}

#[cfg(CONFIG_HAVE_ATOMIC_IOMAP)]
pub unsafe extern "C" fn io_mapping_unmap(vaddr: *mut core::ffi::c_void) { iounmap(vaddr); }

#[cfg(not(CONFIG_HAVE_ATOMIC_IOMAP))]
pub unsafe extern "C" fn io_mapping_init_wc(iomap: *mut io_mapping, base: resource_size_t, size: c_ulong) -> *mut io_mapping {
    (*iomap).iomem = ioremap_wc(base, size);
    if (*iomap).iomem.is_null() { return core::ptr::null_mut(); }
    (*iomap).base = base;
    (*iomap).size = size;
    (*iomap).prot = pgprot_writecombine(PAGE_KERNEL);
    iomap
}

#[cfg(not(CONFIG_HAVE_ATOMIC_IOMAP))]
pub unsafe extern "C" fn io_mapping_fini(mapping: *mut io_mapping) { iounmap((*mapping).iomem); }

#[cfg(not(CONFIG_HAVE_ATOMIC_IOMAP))]
pub unsafe extern "C" fn io_mapping_map_wc(mapping: *mut io_mapping, offset: c_ulong, _size: c_ulong) -> *mut core::ffi::c_void { (*mapping).iomem.add(offset as usize) }

#[cfg(not(CONFIG_HAVE_ATOMIC_IOMAP))]
pub unsafe extern "C" fn io_mapping_unmap(_vaddr: *mut core::ffi::c_void) {}

#[cfg(not(CONFIG_HAVE_ATOMIC_IOMAP))]
pub unsafe extern "C" fn io_mapping_map_atomic_wc(mapping: *mut io_mapping, offset: c_ulong) -> *mut core::ffi::c_void {
    if !IS_ENABLED(CONFIG_PREEMPT_RT) { preempt_disable(); } else { migrate_disable(); }
    pagefault_disable();
    io_mapping_map_wc(mapping, offset, PAGE_SIZE)
}

#[cfg(not(CONFIG_HAVE_ATOMIC_IOMAP))]
pub unsafe extern "C" fn io_mapping_unmap_atomic(vaddr: *mut core::ffi::c_void) {
    io_mapping_unmap(vaddr); pagefault_enable();
    if !IS_ENABLED(CONFIG_PREEMPT_RT) { preempt_enable(); } else { migrate_enable(); }
}

#[cfg(not(CONFIG_HAVE_ATOMIC_IOMAP))]
pub unsafe extern "C" fn io_mapping_map_local_wc(mapping: *mut io_mapping, offset: c_ulong) -> *mut core::ffi::c_void { io_mapping_map_wc(mapping, offset, PAGE_SIZE) }

#[cfg(not(CONFIG_HAVE_ATOMIC_IOMAP))]
pub unsafe extern "C" fn io_mapping_unmap_local(vaddr: *mut core::ffi::c_void) { io_mapping_unmap(vaddr); }

pub unsafe extern "C" fn io_mapping_create_wc(base: resource_size_t, size: c_ulong) -> *mut io_mapping {
    let iomap = kmalloc_obj::<io_mapping>();
    if iomap.is_null() { return core::ptr::null_mut(); }
    if io_mapping_init_wc(iomap, base, size).is_null() { kfree(iomap); return core::ptr::null_mut(); }
    iomap
}

pub unsafe extern "C" fn io_mapping_free(iomap: *mut io_mapping) {
    io_mapping_fini(iomap);
    kfree(iomap);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
