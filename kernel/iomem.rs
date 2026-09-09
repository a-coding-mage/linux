/* SPDX-License-Identifier: GPL-2.0 */
// Translated from iomem.c. Linux headers and externally supplied symbols are
// intentionally left as dependencies of the surrounding repository.

use core::ffi::c_void;

#[allow(non_camel_case_types)]
type resource_size_t = u64;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(not(any()))]
unsafe fn arch_memremap_wb(offset: resource_size_t, size: u64, _flags: u64) -> *mut c_void {
    // #ifdef ioremap_cache
    // return (__force void *)ioremap_cache(offset, size);
    // #else
    ioremap(offset, size)
    // #endif
}

#[cfg(not(any()))]
unsafe fn arch_memremap_can_ram_remap(
    _offset: resource_size_t,
    _size: usize,
    _flags: u64,
) -> bool {
    true
}

unsafe fn try_ram_remap(offset: resource_size_t, size: usize, flags: u64) -> *mut c_void {
    let pfn: u64 = PHYS_PFN(offset);

    /* In the simple case just return the existing linear address */
    if pfn_valid(pfn)
        && !PageHighMem(pfn_to_page(pfn))
        && arch_memremap_can_ram_remap(offset, size, flags)
    {
        return __va(offset);
    }

    core::ptr::null_mut() /* fallback to arch_memremap_wb */
}

/**
 * memremap() - remap an iomem_resource as cacheable memory
 * @offset: iomem resource start address
 * @size: size of remap
 * @flags: any of MEMREMAP_WB, MEMREMAP_WT, MEMREMAP_WC,
 *          MEMREMAP_ENC, MEMREMAP_DEC
 *
 * memremap() is "ioremap" for cases where it is known that the resource
 * being mapped does not have i/o side effects and the __iomem
 * annotation is not applicable. In the case of multiple flags, the different
 * mapping types will be attempted in the order listed below until one of
 * them succeeds.
 *
 * MEMREMAP_WB - matches the default mapping for System RAM on
 * the architecture.  This is usually a read-allocate write-back cache.
 * Moreover, if MEMREMAP_WB is specified and the requested remap region is RAM
 * memremap() will bypass establishing a new mapping and instead return
 * a pointer into the direct map.
 *
 * MEMREMAP_WT - establish a mapping whereby writes either bypass the
 * cache or are written through to memory and never exist in a
 * cache-dirty state with respect to program visibility.  Attempts to
 * map System RAM with this mapping type will fail.
 *
 * MEMREMAP_WC - establish a writecombine mapping, whereby writes may
 * be coalesced together (e.g. in the CPU's write buffers), but is otherwise
 * uncached. Attempts to map System RAM with this mapping type will fail.
 */
pub unsafe fn memremap(offset: resource_size_t, size: usize, flags: u64) -> *mut c_void {
    let is_ram = region_intersects(offset, size, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE);
    let mut addr: *mut c_void = core::ptr::null_mut();

    if flags == 0 {
        return core::ptr::null_mut();
    }

    if is_ram == REGION_MIXED {
        WARN_ONCE(1, "memremap attempted on mixed range %pa size: %#lx\n", &offset, size as u64);
        return core::ptr::null_mut();
    }

    /* Try all mapping types requested until one returns non-NULL */
    if flags & MEMREMAP_WB != 0 {
        /* MEMREMAP_WB is special in that it can be satisfied from the direct map. */
        if is_ram == REGION_INTERSECTS {
            addr = try_ram_remap(offset, size, flags);
        }
        if addr.is_null() {
            addr = arch_memremap_wb(offset, size as u64, flags);
        }
    }

    /* Enforce that a new virtual mapping is not aliasing System RAM. */
    if addr.is_null() && is_ram == REGION_INTERSECTS && flags != MEMREMAP_WB {
        WARN_ONCE(1, "memremap attempted on ram %pa size: %#lx\n", &offset, size as u64);
        return core::ptr::null_mut();
    }

    if addr.is_null() && flags & MEMREMAP_WT != 0 {
        addr = ioremap_wt(offset, size);
    }

    if addr.is_null() && flags & MEMREMAP_WC != 0 {
        addr = ioremap_wc(offset, size);
    }

    addr
}

pub unsafe fn memunmap(addr: *mut c_void) {
    if is_ioremap_addr(addr) {
        iounmap(addr);
    }
}

unsafe fn devm_memremap_release(_dev: *mut device, res: *mut c_void) {
    memunmap(*(res as *mut *mut c_void));
}

unsafe fn devm_memremap_match(
    _dev: *mut device,
    res: *mut c_void,
    match_data: *mut c_void,
) -> i32 {
    (*(res as *mut *mut c_void) == match_data) as i32
}

pub unsafe fn devm_memremap(
    dev: *mut device,
    offset: resource_size_t,
    size: usize,
    flags: u64,
) -> *mut c_void {
    let ptr = devres_alloc_node(
        devm_memremap_release,
        core::mem::size_of::<*mut c_void>(),
        GFP_KERNEL,
        dev_to_node(dev),
    ) as *mut *mut c_void;
    if ptr.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let addr = memremap(offset, size, flags);
    if !addr.is_null() {
        *ptr = addr;
        devres_add(dev, ptr as *mut c_void);
    } else {
        devres_free(ptr as *mut c_void);
        return ERR_PTR(-ENXIO);
    }

    addr
}

pub unsafe fn devm_memunmap(dev: *mut device, addr: *mut c_void) {
    WARN_ON(devres_release(
        dev,
        devm_memremap_release,
        devm_memremap_match,
        addr,
    ));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
