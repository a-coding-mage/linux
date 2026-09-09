/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header dependencies and the __ASSEMBLER__ guard are represented by the
 * Rust items below; names supplied by the surrounding kernel translation are
 * intentionally left as external dependencies.
 */

/* supports 3 memory models. */

#[cfg(CONFIG_FLATMEM)]
pub const ARCH_PFN_OFFSET: ::core::ffi::c_ulong = 0;

#[cfg(CONFIG_FLATMEM)]
extern "C" {
    pub static mut max_mapnr: ::core::ffi::c_ulong;
    pub static mut mem_map: *mut page;
}

#[cfg(CONFIG_FLATMEM)]
#[inline]
pub unsafe fn __pfn_to_page(pfn: ::core::ffi::c_ulong) -> *mut page {
    mem_map.add(pfn.wrapping_sub(ARCH_PFN_OFFSET) as usize)
}

#[cfg(CONFIG_FLATMEM)]
#[inline]
pub unsafe fn __page_to_pfn(page: *const page) -> ::core::ffi::c_ulong {
    (page.offset_from(mem_map as *const page) as ::core::ffi::c_ulong)
        .wrapping_add(ARCH_PFN_OFFSET)
}

#[cfg(CONFIG_FLATMEM)]
#[inline]
pub unsafe fn pfn_valid(pfn: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let pfn_offset = ARCH_PFN_OFFSET;
    (pfn >= pfn_offset && pfn.wrapping_sub(pfn_offset) < max_mapnr) as ::core::ffi::c_int
}

#[cfg(CONFIG_FLATMEM)]
#[inline]
pub unsafe fn for_each_valid_pfn<F: FnMut(::core::ffi::c_ulong)>(
    mut body: F,
    start_pfn: ::core::ffi::c_ulong,
    end_pfn: ::core::ffi::c_ulong,
) {
    let mut pfn = core::cmp::max(start_pfn, ARCH_PFN_OFFSET);
    let end = core::cmp::min(end_pfn, ARCH_PFN_OFFSET.wrapping_add(max_mapnr));
    while pfn < end {
        body(pfn);
        pfn = pfn.wrapping_add(1);
    }
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
extern "C" {
    pub static mut vmemmap: *mut page;
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
#[inline]
pub unsafe fn __pfn_to_page(pfn: ::core::ffi::c_ulong) -> *mut page {
    vmemmap.add(pfn as usize)
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
#[inline]
pub unsafe fn __page_to_pfn(page: *const page) -> ::core::ffi::c_ulong {
    page.offset_from(vmemmap as *const page) as ::core::ffi::c_ulong
}

#[cfg(CONFIG_SPARSEMEM)]
#[inline]
pub unsafe fn __page_to_pfn(pg: *const page) -> ::core::ffi::c_ulong {
    let sec = memdesc_section(&(*pg).flags);
    pg.offset_from(__section_mem_map_addr(__nr_to_section(sec)) as *const page)
        as ::core::ffi::c_ulong
}

#[cfg(CONFIG_SPARSEMEM)]
#[inline]
pub unsafe fn __pfn_to_page(pfn: ::core::ffi::c_ulong) -> *mut page {
    let sec = __pfn_to_section(pfn);
    (__section_mem_map_addr(sec) as *mut page).add(pfn as usize)
}

/* Convert a physical address to a Page Frame Number and back. */
#[inline]
pub fn __phys_to_pfn(paddr: usize) -> usize {
    PHYS_PFN(paddr)
}

#[inline]
pub fn __pfn_to_phys(pfn: usize) -> usize {
    PFN_PHYS(pfn)
}

#[inline]
pub unsafe fn page_to_pfn(page: *const page) -> ::core::ffi::c_ulong {
    __page_to_pfn(page)
}

#[inline]
pub unsafe fn pfn_to_page(pfn: ::core::ffi::c_ulong) -> *mut page {
    __pfn_to_page(pfn)
}

#[cfg(CONFIG_DEBUG_VIRTUAL)]
#[inline]
pub unsafe fn page_to_phys(page: *const page) -> usize {
    let pfn = page_to_pfn(page);
    WARN_ON_ONCE(!pfn_valid(pfn));
    PFN_PHYS(pfn)
}

#[cfg(not(CONFIG_DEBUG_VIRTUAL))]
#[inline]
pub unsafe fn page_to_phys(page: *const page) -> usize {
    PFN_PHYS(page_to_pfn(page))
}

#[inline]
pub unsafe fn phys_to_page(phys: usize) -> *mut page {
    pfn_to_page(PHYS_PFN(phys))
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
