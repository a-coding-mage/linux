// SPDX-License-Identifier: GPL-2.0
/*
 * sparse memory mappings.
 */

// Linux kernel dependencies supplied by other translation units.

#[cfg(feature = "config_sparsemem_extreme")]
pub static mut mem_section: *mut *mut mem_section = core::ptr::null_mut();
#[cfg(not(feature = "config_sparsemem_extreme"))]
pub static mut mem_section: [[mem_section; SECTIONS_PER_ROOT]; NR_SECTION_ROOTS] =
    [[mem_section { section_mem_map: 0 }; SECTIONS_PER_ROOT]; NR_SECTION_ROOTS];

#[cfg(feature = "node_not_in_page_flags")]
#[cfg(feature = "max_num_nodes_le_256")]
static mut section_to_node_table: [u8; NR_MEM_SECTIONS] = [0; NR_MEM_SECTIONS];
#[cfg(feature = "node_not_in_page_flags")]
#[cfg(not(feature = "max_num_nodes_le_256"))]
static mut section_to_node_table: [u16; NR_MEM_SECTIONS] = [0; NR_MEM_SECTIONS];

#[cfg(feature = "node_not_in_page_flags")]
pub unsafe fn memdesc_nid(mdf: *const memdesc_flags_t) -> i32 {
    section_to_node_table[memdesc_section(mdf) as usize] as i32
}

#[cfg(feature = "node_not_in_page_flags")]
unsafe fn set_section_nid(section_nr: usize, nid: i32) {
    section_to_node_table[section_nr] = nid as _;
}

#[cfg(not(feature = "node_not_in_page_flags"))]
#[inline]
unsafe fn set_section_nid(_section_nr: usize, _nid: i32) {}

#[cfg(feature = "config_sparsemem_extreme")]
unsafe fn sparse_index_alloc(nid: i32) -> *mut mem_section {
    let array_size = SECTIONS_PER_ROOT * core::mem::size_of::<mem_section>();
    let mut section: *mut mem_section = core::ptr::null_mut();
    if slab_is_available() {
        section = kzalloc_node(array_size, GFP_KERNEL, nid);
    } else {
        section = memblock_alloc_node(array_size, SMP_CACHE_BYTES, nid);
        if section.is_null() {
            panic!("{}: Failed to allocate {} bytes nid={}\n", "sparse_index_alloc", array_size, nid);
        }
    }
    section
}

#[cfg(feature = "config_sparsemem_extreme")]
pub unsafe fn sparse_index_init(section_nr: usize, nid: i32) -> i32 {
    let root = SECTION_NR_TO_ROOT(section_nr);
    if !(*mem_section.add(root)).is_null() {
        return 0;
    }
    let section = sparse_index_alloc(nid);
    if section.is_null() { return -12; }
    *mem_section.add(root) = section;
    0
}

#[cfg(not(feature = "config_sparsemem_extreme"))]
pub unsafe fn sparse_index_init(_section_nr: usize, _nid: i32) -> i32 { 0 }

#[inline]
unsafe fn sparse_encode_early_nid(nid: i32) -> usize { (nid as usize) << SECTION_NID_SHIFT }

#[inline]
unsafe fn sparse_early_nid(section: *mut mem_section) -> i32 {
    ((*section).section_mem_map >> SECTION_NID_SHIFT) as i32
}

unsafe fn mminit_validate_memmodel_limits(start_pfn: *mut usize, end_pfn: *mut usize) {
    let max_sparsemem_pfn = (DIRECT_MAP_PHYSMEM_END + 1) >> PAGE_SHIFT;
    if *start_pfn > max_sparsemem_pfn {
        mminit_dprintk(MMINIT_WARNING, "pfnvalidation", "Start of range %lu -> %lu exceeds SPARSEMEM max %lu\n", *start_pfn, *end_pfn, max_sparsemem_pfn);
        WARN_ON_ONCE(1);
        *start_pfn = max_sparsemem_pfn;
        *end_pfn = max_sparsemem_pfn;
    } else if *end_pfn > max_sparsemem_pfn {
        mminit_dprintk(MMINIT_WARNING, "pfnvalidation", "End of range %lu -> %lu exceeds SPARSEMEM max %lu\n", *start_pfn, *end_pfn, max_sparsemem_pfn);
        WARN_ON_ONCE(1);
        *end_pfn = max_sparsemem_pfn;
    }
}

pub static mut __highest_present_section_nr: usize = 0;

#[inline]
unsafe fn first_present_section_nr() -> usize { next_present_section_nr(usize::MAX) }

unsafe fn memory_present(nid: i32, mut start: usize, mut end: usize) {
    start &= PAGE_SECTION_MASK;
    mminit_validate_memmodel_limits(&mut start, &mut end);
    let mut pfn = start;
    while pfn < end {
        let section_nr = pfn_to_section_nr(pfn);
        sparse_index_init(section_nr, nid);
        set_section_nid(section_nr, nid);
        let ms = __nr_to_section(section_nr);
        if (*ms).section_mem_map == 0 {
            (*ms).section_mem_map = sparse_encode_early_nid(nid) | SECTION_IS_ONLINE;
            __section_mark_present(ms, section_nr);
        }
        pfn += PAGES_PER_SECTION;
    }
}

unsafe fn memblocks_present() {
    #[cfg(feature = "config_sparsemem_extreme")]
    {
        let size = core::mem::size_of::<*mut mem_section>() * NR_SECTION_ROOTS;
        let align = 1usize << INTERNODE_CACHE_SHIFT;
        mem_section = memblock_alloc_or_panic(size, align);
    }
    let mut i = 0;
    let mut start = 0usize;
    let mut end = 0usize;
    let mut nid = 0i32;
    while for_each_mem_pfn_range(&mut i, MAX_NUMNODES, &mut start, &mut end, &mut nid) {
        memory_present(nid, start, end);
    }
}

#[cfg(feature = "config_sparsemem_vmemmap")]
pub unsafe fn section_map_size() -> usize {
    ALIGN(core::mem::size_of::<page>() * PAGES_PER_SECTION, PMD_SIZE)
}

#[cfg(not(feature = "config_sparsemem_vmemmap"))]
pub unsafe fn section_map_size() -> usize {
    PAGE_ALIGN(core::mem::size_of::<page>() * PAGES_PER_SECTION)
}

#[cfg(not(feature = "config_sparsemem_vmemmap"))]
pub unsafe fn __populate_section_memmap(pfn: usize, _nr_pages: usize, nid: i32, _altmap: *mut vmem_altmap, _pgmap: *mut dev_pagemap) -> *mut page {
    let size = section_map_size();
    memmap_alloc(size, size, __pa(MAX_DMA_ADDRESS), nid, false)
}

pub unsafe fn vmemmap_populate_print_last() {}

static mut sparse_usagebuf: *mut core::ffi::c_void = core::ptr::null_mut();
static mut sparse_usagebuf_end: *mut core::ffi::c_void = core::ptr::null_mut();

pub unsafe fn sparse_init_early_section(nid: i32, map: *mut page, pnum: usize, flags: usize) {
    BUG_ON(sparse_usagebuf.is_null() || sparse_usagebuf >= sparse_usagebuf_end);
    sparse_init_one_section(__nr_to_section(pnum), pnum, map, sparse_usagebuf, SECTION_IS_EARLY | flags);
    sparse_usagebuf = (sparse_usagebuf as *mut u8).add(mem_section_usage_size()) as *mut _;
}

unsafe fn sparse_usage_init(nid: i32, map_count: usize) -> i32 {
    let size = mem_section_usage_size() * map_count;
    sparse_usagebuf = memblock_alloc_node(size, SMP_CACHE_BYTES, nid);
    if sparse_usagebuf.is_null() { sparse_usagebuf_end = core::ptr::null_mut(); return -12; }
    sparse_usagebuf_end = (sparse_usagebuf as *mut u8).add(size) as *mut _;
    0
}

unsafe fn sparse_usage_fini() { sparse_usagebuf = core::ptr::null_mut(); sparse_usagebuf_end = core::ptr::null_mut(); }

unsafe fn sparse_init_nid(nid: i32, pnum_begin: usize, pnum_end: usize, map_count: usize) {
    if sparse_usage_init(nid, map_count) != 0 { panic!("Failed to allocate usemap for node {}\n", nid); }
    sparse_vmemmap_init_nid_early(nid);
    let mut pnum = 0usize;
    while for_each_present_section_nr(pnum_begin, &mut pnum) {
        if pnum >= pnum_end { break; }
        let ms = __nr_to_section(pnum);
        let pfn = section_nr_to_pfn(pnum);
        if !preinited_vmemmap_section(ms) {
            let map = __populate_section_memmap(pfn, PAGES_PER_SECTION, nid, core::ptr::null_mut(), core::ptr::null_mut());
            if map.is_null() { panic!("Failed to allocate memmap for section {}\n", pnum); }
            memmap_boot_pages_add(DIV_ROUND_UP(PAGES_PER_SECTION * core::mem::size_of::<page>(), PAGE_SIZE));
            sparse_init_early_section(nid, map, pnum, 0);
        }
    }
    sparse_usage_fini();
}

pub unsafe fn sparse_init() {
    BUILD_BUG_ON(!is_power_of_2(core::mem::size_of::<mem_section>()));
    memblocks_present();
    if compound_info_has_mask() { VM_WARN_ON_ONCE(!IS_ALIGNED(pfn_to_page(0) as usize, MAX_FOLIO_VMEMMAP_ALIGN)); }
    let pnum_begin = first_present_section_nr();
    let mut nid_begin = sparse_early_nid(__nr_to_section(pnum_begin));
    let mut pnum_end = 0usize;
    let mut map_count = 1usize;
    let mut pnum = 0usize;
    while for_each_present_section_nr(pnum_begin + 1, &mut pnum) {
        pnum_end = pnum;
        let nid = sparse_early_nid(__nr_to_section(pnum_end));
        if nid == nid_begin { map_count += 1; continue; }
        sparse_init_nid(nid_begin, pnum_begin, pnum_end, map_count);
        nid_begin = nid;
        map_count = 1;
    }
    sparse_init_nid(nid_begin, pnum_begin, pnum_end, map_count);
    sparse_init_subsection_map();
    vmemmap_populate_print_last();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
