/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Internals of the DMA direct mapping implementation. Only for use by the
 * DMA mapping code and IOMMU drivers.
 *
 * C header dependencies are supplied by the surrounding translation unit.
 */

extern "C" {
    pub static mut zone_dma_limit: u64;
}

#[repr(C)]
pub struct bus_dma_region {
    pub cpu_start: phys_addr_t,
    pub dma_start: dma_addr_t,
    pub size: u64,
}

pub unsafe fn translate_phys_to_dma(
    dev: *mut device,
    paddr: phys_addr_t,
) -> dma_addr_t {
    let mut m = (*dev).dma_range_map;

    while (*m).size != 0 {
        let offset = paddr.wrapping_sub((*m).cpu_start);
        if paddr >= (*m).cpu_start && offset < (*m).size {
            return (*m).dma_start.wrapping_add(offset);
        }
        m = m.add(1);
    }

    /* Make sure dma_capable fails when no translation is available. */
    DMA_MAPPING_ERROR
}

pub unsafe fn translate_dma_to_phys(
    dev: *mut device,
    dma_addr: dma_addr_t,
) -> phys_addr_t {
    let mut m = (*dev).dma_range_map;

    while (*m).size != 0 {
        let offset = dma_addr.wrapping_sub((*m).dma_start);
        if dma_addr >= (*m).dma_start && offset < (*m).size {
            return (*m).cpu_start.wrapping_add(offset);
        }
        m = m.add(1);
    }

    !0 as phys_addr_t
}

pub unsafe fn dma_range_map_min(map: *const bus_dma_region) -> dma_addr_t {
    let mut ret = u64::MAX as dma_addr_t;
    let mut map = map;
    while (*map).size != 0 {
        ret = core::cmp::min(ret, (*map).dma_start);
        map = map.add(1);
    }
    ret
}

pub unsafe fn dma_range_map_max(map: *const bus_dma_region) -> dma_addr_t {
    let mut ret = 0 as dma_addr_t;
    let mut map = map;
    while (*map).size != 0 {
        ret = core::cmp::max(
            ret,
            (*map).dma_start.wrapping_add((*map).size).wrapping_sub(1),
        );
        map = map.add(1);
    }
    ret
}

/* CONFIG_ARCH_HAS_PHYS_TO_DMA supplies the architecture-specific variants. */
#[cfg(not(CONFIG_ARCH_HAS_PHYS_TO_DMA))]
pub unsafe fn __phys_to_dma(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t {
    if !(*dev).dma_range_map.is_null() {
        translate_phys_to_dma(dev, paddr)
    } else {
        paddr
    }
}

#[cfg(not(CONFIG_ARCH_HAS_PHYS_TO_DMA))]
pub unsafe fn phys_to_dma_unencrypted(
    dev: *mut device,
    paddr: phys_addr_t,
) -> dma_addr_t {
    dma_addr_unencrypted(__phys_to_dma(dev, paddr))
}

#[cfg(not(CONFIG_ARCH_HAS_PHYS_TO_DMA))]
pub unsafe fn phys_to_dma_encrypted(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t {
    dma_addr_encrypted(__phys_to_dma(dev, paddr))
}

/*
 * If memory encryption is supported, phys_to_dma sets the memory encryption
 * bit in the DMA address, and dma_to_phys clears it. phys_to_dma_unencrypted
 * is for use on special unencrypted memory like swiotlb buffers.
 */
#[cfg(not(CONFIG_ARCH_HAS_PHYS_TO_DMA))]
pub unsafe fn phys_to_dma(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t {
    dma_addr_encrypted(__phys_to_dma(dev, paddr))
}

#[cfg(not(CONFIG_ARCH_HAS_PHYS_TO_DMA))]
pub unsafe fn dma_to_phys(dev: *mut device, mut dma_addr: dma_addr_t) -> phys_addr_t {
    dma_addr = dma_addr_canonical(dma_addr);
    if !(*dev).dma_range_map.is_null() {
        translate_dma_to_phys(dev, dma_addr)
    } else {
        dma_addr
    }
}

#[cfg(CONFIG_ARCH_HAS_FORCE_DMA_UNENCRYPTED)]
extern "C" {
    pub fn force_dma_unencrypted(dev: *mut device) -> bool;
}

#[cfg(not(CONFIG_ARCH_HAS_FORCE_DMA_UNENCRYPTED))]
pub unsafe fn force_dma_unencrypted(_dev: *mut device) -> bool {
    false
}

pub unsafe fn dma_capable(
    dev: *mut device,
    addr: dma_addr_t,
    size: usize,
    is_ram: bool,
    attrs: c_ulong,
) -> bool {
    let end = addr.wrapping_add(size).wrapping_sub(1);

    if addr == DMA_MAPPING_ERROR {
        return false;
    }
    /* The device requires unencrypted DMA addresses. */
    if (attrs & DMA_ATTR_CC_SHARED) == 0 && force_dma_unencrypted(dev) {
        return false;
    }

    /* CONFIG_ARCH_DMA_ADDR_T_64BIT controls this build-time condition. */
    if is_ram && !cfg!(CONFIG_ARCH_DMA_ADDR_T_64BIT)
        && core::cmp::min(addr, end) < phys_to_dma(dev, PFN_PHYS(min_low_pfn))
    {
        return false;
    }

    end <= min_not_zero((*dev).dma_mask, (*dev).bus_dma_limit)
}

extern "C" {
    pub fn dma_direct_get_required_mask(dev: *mut device) -> u64;
    pub fn dma_direct_alloc(
        dev: *mut device,
        size: usize,
        dma_handle: *mut dma_addr_t,
        gfp: gfp_t,
        attrs: c_ulong,
    ) -> *mut core::ffi::c_void;
    pub fn dma_direct_free(
        dev: *mut device,
        size: usize,
        cpu_addr: *mut core::ffi::c_void,
        dma_addr: dma_addr_t,
        attrs: c_ulong,
    );
    pub fn dma_direct_alloc_pages(
        dev: *mut device,
        size: usize,
        dma_handle: *mut dma_addr_t,
        dir: dma_data_direction,
        gfp: gfp_t,
    ) -> *mut page;
    pub fn dma_direct_free_pages(
        dev: *mut device,
        size: usize,
        page: *mut page,
        dma_addr: dma_addr_t,
        dir: dma_data_direction,
    );
    pub fn dma_direct_supported(dev: *mut device, mask: u64) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
