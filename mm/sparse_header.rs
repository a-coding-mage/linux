/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * sparse.h:
 *
 * mm/ internal sparse and sparse-vmemmap declarations
 */

// Dependency declarations from <linux/mmzone.h> are supplied by the surrounding
// translation unit.

/*
 * mm/sparse.c
 */
#[cfg(feature = "CONFIG_SPARSEMEM")]
extern "C" {
    pub fn sparse_init();
    pub fn sparse_index_init(section_nr: ::core::ffi::c_ulong, nid: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_SPARSEMEM")]
#[inline]
pub unsafe fn sparse_init_one_section(
    ms: *mut mem_section,
    pnum: ::core::ffi::c_ulong,
    mem_map: *mut page,
    usage: *mut mem_section_usage,
    flags: ::core::ffi::c_ulong,
) {
    let coded_mem_map: ::core::ffi::c_ulong;

    // BUILD_BUG_ON(SECTION_MAP_LAST_BIT > PFN_SECTION_SHIFT);

    /*
     * We encode the start PFN of the section into the mem_map such that
     * page_to_pfn() on !CONFIG_SPARSEMEM_VMEMMAP can simply subtract it
     * from the page pointer to obtain the PFN.
     */
    coded_mem_map = mem_map.offset(-(section_nr_to_pfn(pnum) as isize)) as ::core::ffi::c_ulong;
    // VM_WARN_ON_ONCE(coded_mem_map & !SECTION_MAP_MASK);

    (*ms).section_mem_map &= !SECTION_MAP_MASK;
    (*ms).section_mem_map |= coded_mem_map;
    (*ms).section_mem_map |= flags | SECTION_HAS_MEM_MAP;
    (*ms).usage = usage;
}

#[cfg(feature = "CONFIG_SPARSEMEM")]
#[inline]
pub unsafe fn __section_mark_present(
    ms: *mut mem_section,
    section_nr: ::core::ffi::c_ulong,
) {
    if section_nr > __highest_present_section_nr {
        __highest_present_section_nr = section_nr;
    }

    (*ms).section_mem_map |= SECTION_MARKED_PRESENT;
}

#[cfg(feature = "CONFIG_SPARSEMEM")]
#[inline]
pub const fn mem_section_usage_size() -> usize {
    struct_size_t(
        ::core::mem::size_of::<mem_section_usage>(),
        BITS_TO_LONGS(SECTION_BLOCKFLAGS_BITS),
    )
}

#[cfg(not(feature = "CONFIG_SPARSEMEM"))]
#[inline]
pub fn sparse_init() {}

/*
 * mm/sparse-vmemmap.c
 */
#[cfg(feature = "CONFIG_SPARSEMEM_VMEMMAP")]
extern "C" {
    pub fn sparse_init_subsection_map();
}

#[cfg(not(feature = "CONFIG_SPARSEMEM_VMEMMAP"))]
#[inline]
pub fn sparse_init_subsection_map() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
