// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding translation unit.

#[cfg(CONFIG_SPARSEMEM)]
const PAGE_EXT_INVALID: usize = 0x1;

#[cfg(all(CONFIG_PAGE_IDLE_FLAG, not(CONFIG_64BIT)))]
unsafe extern "C" {
    fn need_page_idle() -> bool;
    static mut page_idle_ops: page_ext_operations;
}

#[cfg(CONFIG_SPARSEMEM)]
unsafe extern "C" {
    static mut page_owner_ops: page_ext_operations;
}
#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
unsafe extern "C" {
    static mut page_alloc_tagging_ops: page_ext_operations;
}
#[cfg(CONFIG_PAGE_TABLE_CHECK)]
unsafe extern "C" {
    static mut page_table_check_ops: page_ext_operations;
}
#[cfg(CONFIG_IOMMU_DEBUG_PAGEALLOC)]
unsafe extern "C" {
    static mut page_iommu_debug_ops: page_ext_operations;
}

#[repr(C)]
pub struct page_ext_operations {
    pub need: Option<unsafe extern "C" fn() -> bool>,
    pub init: Option<unsafe extern "C" fn()>,
    pub size: usize,
    pub offset: usize,
    pub need_shared_flags: bool,
}

extern "C" {
    static mut page_ext_size: usize;
}
static mut total_usage: usize = 0;

#[cfg(CONFIG_MEM_ALLOC_PROFILING_DEBUG)]
pub static mut early_page_ext: bool = true;
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING_DEBUG))]
pub static mut early_page_ext: bool = false;

unsafe extern "C" fn setup_early_page_ext(_str: *mut core::ffi::c_char) -> i32 {
    early_page_ext = true;
    0
}

#[cfg(all(CONFIG_PAGE_IDLE_FLAG, not(CONFIG_64BIT)))]
static mut page_idle_ops_local: page_ext_operations = page_ext_operations {
    need: Some(need_page_idle), init: None, size: 0, offset: 0,
    need_shared_flags: true,
};

static mut page_ext_ops: [*mut page_ext_operations; 5] = [
    #[cfg(CONFIG_PAGE_OWNER)] unsafe { &raw mut page_owner_ops },
    #[cfg(all(CONFIG_PAGE_IDLE_FLAG, not(CONFIG_64BIT)))] unsafe { &raw mut page_idle_ops_local },
    #[cfg(CONFIG_MEM_ALLOC_PROFILING)] unsafe { &raw mut page_alloc_tagging_ops },
    #[cfg(CONFIG_PAGE_TABLE_CHECK)] unsafe { &raw mut page_table_check_ops },
    #[cfg(CONFIG_IOMMU_DEBUG_PAGEALLOC)] unsafe { &raw mut page_iommu_debug_ops },
];

unsafe fn invoke_need_callbacks() -> bool {
    let mut need = false;
    for op in page_ext_ops.iter().copied() {
        if let Some(f) = (*op).need {
            if f() && (*op).need_shared_flags {
                page_ext_size = core::mem::size_of::<page_ext>();
                break;
            }
        }
    }
    for op in page_ext_ops.iter().copied() {
        if let Some(f) = (*op).need {
            if f() {
                (*op).offset = page_ext_size;
                page_ext_size += (*op).size;
                need = true;
            }
        }
    }
    need
}

unsafe fn invoke_init_callbacks() {
    for op in page_ext_ops.iter().copied() {
        if let Some(f) = (*op).init { f(); }
    }
}

unsafe fn get_entry(base: *mut page_ext, index: usize) -> *mut page_ext {
    (base as *mut u8).add(page_ext_size.wrapping_mul(index)) as *mut page_ext
}

#[cfg(not(CONFIG_SPARSEMEM))]
pub unsafe extern "C" fn page_ext_init_flatmem_late() { invoke_init_callbacks(); }

#[cfg(not(CONFIG_SPARSEMEM))]
unsafe fn lookup_page_ext(page: *const page) -> *mut page_ext {
    let pfn = page_to_pfn(page);
    let base = (*NODE_DATA(page_to_nid(page))).node_page_ext;
    if base.is_null() { return core::ptr::null_mut(); }
    get_entry(base, pfn.wrapping_sub(round_down(node_start_pfn(page_to_nid(page)), MAX_ORDER_NR_PAGES)))
}

#[cfg(not(CONFIG_SPARSEMEM))]
unsafe fn alloc_node_page_ext(nid: i32) -> i32 {
    let nr_pages = (*NODE_DATA(nid)).node_spanned_pages;
    if nr_pages == 0 { return 0; }
    let mut pages = nr_pages;
    if !is_aligned(node_start_pfn(nid), MAX_ORDER_NR_PAGES) || !is_aligned(node_end_pfn(nid), MAX_ORDER_NR_PAGES) { pages += MAX_ORDER_NR_PAGES; }
    let table_size = page_ext_size * pages;
    let base = memblock_alloc_try_nid(table_size, PAGE_SIZE, __pa(MAX_DMA_ADDRESS), MEMBLOCK_ALLOC_ACCESSIBLE, nid);
    if base.is_null() { return -ENOMEM; }
    (*NODE_DATA(nid)).node_page_ext = base;
    total_usage += table_size;
    memmap_boot_pages_add(div_round_up(table_size, PAGE_SIZE));
    0
}

#[cfg(not(CONFIG_SPARSEMEM))]
pub unsafe extern "C" fn page_ext_init_flatmem() {
    if !invoke_need_callbacks() { return; }
    for_each_online_node!(nid => { if alloc_node_page_ext(nid) != 0 { panic!("Out of memory"); } });
    pr_info!("allocated %ld bytes of page_ext\n", total_usage);
}

#[cfg(CONFIG_SPARSEMEM)]
unsafe fn page_ext_invalid(p: *mut page_ext) -> bool { p.is_null() || ((p as usize & PAGE_EXT_INVALID) == PAGE_EXT_INVALID) }

#[cfg(CONFIG_SPARSEMEM)]
unsafe fn lookup_page_ext(page: *const page) -> *mut page_ext {
    let section = __pfn_to_section(page_to_pfn(page));
    let ext = read_once((*section).page_ext);
    if page_ext_invalid(ext) { return core::ptr::null_mut(); }
    get_entry(ext, page_to_pfn(page))
}

#[cfg(CONFIG_SPARSEMEM)]
unsafe fn alloc_page_ext(size: usize, nid: i32) -> *mut core::ffi::c_void {
    let flags = GFP_KERNEL | __GFP_ZERO | __GFP_NOWARN;
    let mut addr = alloc_pages_exact_nid(nid, size, flags);
    if !addr.is_null() { kmemleak_alloc(addr, size, 1, flags); } else { addr = vzalloc_node(size, nid); }
    if !addr.is_null() { memmap_pages_add(div_round_up(size, PAGE_SIZE)); }
    addr
}

#[cfg(CONFIG_SPARSEMEM)]
unsafe fn init_section_page_ext(mut pfn: usize, nid: i32) -> i32 {
    let section = __pfn_to_section(pfn);
    if !(*section).page_ext.is_null() { return 0; }
    let table_size = page_ext_size * PAGES_PER_SECTION;
    let base = alloc_page_ext(table_size, nid) as *mut page_ext;
    kmemleak_not_leak(base as *mut _);
    if base.is_null() { pr_err!("page ext allocation failure\n"); return -ENOMEM; }
    pfn &= PAGE_SECTION_MASK;
    (*section).page_ext = (base as *mut u8).sub(page_ext_size * pfn) as *mut _;
    total_usage += table_size;
    0
}

#[cfg(CONFIG_SPARSEMEM)]
unsafe fn free_page_ext(addr: *mut core::ffi::c_void) {
    let table_size = page_ext_size * PAGES_PER_SECTION;
    memmap_pages_add(-(div_round_up(table_size, PAGE_SIZE) as isize));
    if is_vmalloc_addr(addr) { vfree(addr); } else { let p = virt_to_page(addr); BUG_ON!(PageReserved(p)); kmemleak_free(addr); free_pages_exact(addr, table_size); }
}

#[cfg(CONFIG_SPARSEMEM)]
unsafe fn __free_page_ext(pfn: usize) {
    let ms = __pfn_to_section(pfn); if ms.is_null() || (*ms).page_ext.is_null() { return; }
    let mut base = read_once((*ms).page_ext); if page_ext_invalid(base) { base = (base as *mut u8).sub(PAGE_EXT_INVALID) as *mut _; }
    write_once(&mut (*ms).page_ext, core::ptr::null_mut());
    free_page_ext(get_entry(base, pfn) as *mut _);
}

#[cfg(CONFIG_SPARSEMEM)]
unsafe fn __invalidate_page_ext(pfn: usize) { let ms = __pfn_to_section(pfn); if !ms.is_null() && !(*ms).page_ext.is_null() { write_once(&mut (*ms).page_ext, ((*ms).page_ext as *mut u8).add(PAGE_EXT_INVALID) as *mut _); } }

#[cfg(CONFIG_SPARSEMEM)]
unsafe fn online_page_ext(start_pfn: usize, nr_pages: usize) -> i32 {
    let nid = pfn_to_nid(start_pfn); let start = section_align_down(start_pfn); let mut pfn = start; let end = section_align_up(start_pfn + nr_pages);
    while pfn < end { if init_section_page_ext(pfn, nid) != 0 { let rollback_end = pfn - PAGES_PER_SECTION; let mut q = start; while q < rollback_end { __free_page_ext(q); q += PAGES_PER_SECTION; } return -ENOMEM; } pfn += PAGES_PER_SECTION; } 0
}

#[cfg(CONFIG_SPARSEMEM)]
unsafe fn offline_page_ext(start_pfn: usize, nr_pages: usize) { let start = section_align_down(start_pfn); let end = section_align_up(start_pfn + nr_pages); let mut p = start; while p < end { __invalidate_page_ext(p); p += PAGES_PER_SECTION; } synchronize_rcu(); p = start; while p < end { __free_page_ext(p); p += PAGES_PER_SECTION; } }

#[cfg(CONFIG_SPARSEMEM)]
pub unsafe extern "C" fn page_ext_init() { if !invoke_need_callbacks() { return; } /* node/section scan and hotplug notifier registration are supplied by kernel macros. */ invoke_init_callbacks(); }

pub unsafe extern "C" fn page_ext_lookup(pfn: usize) -> *mut page_ext { lookup_page_ext(pfn_to_page(pfn)) }
pub unsafe extern "C" fn page_ext_get(page: *const page) -> *mut page_ext { rcu_read_lock(); let p = lookup_page_ext(page); if p.is_null() { rcu_read_unlock(); } p }
pub unsafe extern "C" fn page_ext_from_phys(phys: phys_addr_t) -> *mut page_ext { let p = pfn_to_online_page(__phys_to_pfn(phys)); if p.is_null() { core::ptr::null_mut() } else { page_ext_get(p) } }
pub unsafe extern "C" fn page_ext_put(page_ext: *mut page_ext) { if !page_ext.is_null() { rcu_read_unlock(); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
