#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Translated from grant_table.h. C includes and header guards are omitted. */

pub const INVALID_GRANT_REF: grant_ref_t = !0 as grant_ref_t;
pub const INVALID_GRANT_HANDLE: grant_handle_t = !0 as grant_handle_t;
pub const NR_GRANT_FRAMES: u32 = 4;

#[repr(C)]
pub struct gnttab_free_callback {
    pub next: *mut gnttab_free_callback,
    pub fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub arg: *mut core::ffi::c_void,
    pub count: u16,
}

pub type gnttab_unmap_refs_done = Option<unsafe extern "C" fn(i32, *mut gntab_unmap_queue_data)>;

#[repr(C)]
pub struct gntab_unmap_queue_data {
    pub gnttab_work: delayed_work,
    pub data: *mut core::ffi::c_void,
    pub done: gnttab_unmap_refs_done,
    pub unmap_ops: *mut gnttab_unmap_grant_ref,
    pub kunmap_ops: *mut gnttab_unmap_grant_ref,
    pub pages: *mut *mut page,
    pub count: core::ffi::c_uint,
    pub age: core::ffi::c_uint,
}

extern "C" {
    pub fn gnttab_init() -> i32;
    #[cfg(CONFIG_HIBERNATE_CALLBACKS)]
    pub fn gnttab_suspend() -> i32;
    #[cfg(CONFIG_HIBERNATE_CALLBACKS)]
    pub fn gnttab_resume() -> i32;
    pub fn gnttab_grant_foreign_access(domid: domid_t, frame: c_ulong, readonly: i32) -> i32;
    pub fn gnttab_end_foreign_access_ref(reference: grant_ref_t) -> i32;
    pub fn gnttab_end_foreign_access(reference: grant_ref_t, page: *mut page);
    pub fn gnttab_try_end_foreign_access(reference: grant_ref_t) -> i32;
    pub fn gnttab_alloc_grant_references(count: u16, pprivate_head: *mut grant_ref_t) -> i32;
    pub fn gnttab_alloc_grant_reference_seq(count: c_uint, first: *mut grant_ref_t) -> i32;
    pub fn gnttab_free_grant_reference(reference: grant_ref_t);
    pub fn gnttab_free_grant_references(head: grant_ref_t);
    pub fn gnttab_free_grant_reference_seq(head: grant_ref_t, count: c_uint);
    pub fn gnttab_empty_grant_references(pprivate_head: *const grant_ref_t) -> i32;
    pub fn gnttab_claim_grant_reference(pprivate_head: *mut grant_ref_t) -> i32;
    pub fn gnttab_release_grant_reference(private_head: *mut grant_ref_t, release: grant_ref_t);
    pub fn gnttab_request_free_callback(callback: *mut gnttab_free_callback,
        fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, arg: *mut core::ffi::c_void, count: u16);
    pub fn gnttab_cancel_free_callback(callback: *mut gnttab_free_callback);
    pub fn gnttab_grant_foreign_access_ref(reference: grant_ref_t, domid: domid_t, frame: c_ulong, readonly: i32);
    pub fn xen_page_to_gfn(page: *mut page) -> xen_pfn_t;
    pub fn xen_pv_domain() -> bool;
    pub fn __pa(addr: phys_addr_t) -> phys_addr_t;
    pub fn arch_gnttab_init(nr_shared: c_ulong, nr_status: c_ulong) -> i32;
    pub fn arch_gnttab_map_shared(frames: *mut xen_pfn_t, nr_gframes: c_ulong, max_nr_gframes: c_ulong, shared: *mut *mut core::ffi::c_void) -> i32;
    pub fn arch_gnttab_map_status(frames: *mut u64, nr_gframes: c_ulong, max_nr_gframes: c_ulong, shared: *mut *mut grant_status_t) -> i32;
    pub fn arch_gnttab_unmap(shared: *mut core::ffi::c_void, nr_gframes: c_ulong);
    pub fn gnttab_max_grant_frames() -> c_uint;
    pub fn gnttab_setup_auto_xlat_frames(addr: phys_addr_t) -> i32;
    pub fn gnttab_free_auto_xlat_frames();
    pub fn gnttab_alloc_pages(nr_pages: i32, pages: *mut *mut page) -> i32;
    pub fn gnttab_free_pages(nr_pages: i32, pages: *mut *mut page);
    pub fn gnttab_page_cache_init(cache: *mut gnttab_page_cache);
    pub fn gnttab_page_cache_get(cache: *mut gnttab_page_cache, page: *mut *mut page) -> i32;
    pub fn gnttab_page_cache_put(cache: *mut gnttab_page_cache, page: *mut *mut page, num: c_uint);
    pub fn gnttab_page_cache_shrink(cache: *mut gnttab_page_cache, num: c_uint);
    pub fn gnttab_pages_set_private(nr_pages: i32, pages: *mut *mut page) -> i32;
    pub fn gnttab_pages_clear_private(nr_pages: i32, pages: *mut *mut page);
    pub fn gnttab_map_refs(map_ops: *mut gnttab_map_grant_ref, kmap_ops: *mut gnttab_map_grant_ref, pages: *mut *mut page, count: c_uint) -> i32;
    pub fn gnttab_unmap_refs(unmap_ops: *mut gnttab_unmap_grant_ref, kunmap_ops: *mut gnttab_unmap_grant_ref, pages: *mut *mut page, count: c_uint) -> i32;
    pub fn gnttab_unmap_refs_async(item: *mut gntab_unmap_queue_data);
    pub fn gnttab_unmap_refs_sync(item: *mut gntab_unmap_queue_data) -> i32;
    pub fn gnttab_batch_map(batch: *mut gnttab_map_grant_ref, count: c_uint);
    pub fn gnttab_batch_copy(batch: *mut gnttab_copy, count: c_uint);
    pub fn gnttab_foreach_grant_in_range(page: *mut page, offset: c_uint, len: c_uint, fn_: xen_grant_fn_t, data: *mut core::ffi::c_void);
    pub fn gnttab_foreach_grant(pages: *mut *mut page, nr_grefs: c_uint, fn_: xen_grant_fn_t, data: *mut core::ffi::c_void);
}

pub type c_ulong = core::ffi::c_ulong;
pub type c_uint = core::ffi::c_uint;

#[cfg(not(CONFIG_HIBERNATE_CALLBACKS))]
pub unsafe fn gnttab_suspend() -> i32 { 0 }
#[cfg(not(CONFIG_HIBERNATE_CALLBACKS))]
pub unsafe fn gnttab_resume() -> i32 { 0 }

#[inline]
pub unsafe fn gnttab_page_grant_foreign_access_ref_one(reference: grant_ref_t, domid: domid_t, p: *mut page, readonly: i32) {
    gnttab_grant_foreign_access_ref(reference, domid, xen_page_to_gfn(p) as c_ulong, readonly);
}

#[inline]
pub unsafe fn gnttab_set_map_op(map: *mut gnttab_map_grant_ref, addr: phys_addr_t, flags: u32, reference: grant_ref_t, domid: domid_t) {
    (*map).host_addr = if flags & GNTMAP_contains_pte != 0 { addr } else if !xen_pv_domain() { __pa(addr) } else { addr };
    (*map).flags = flags;
    (*map).ref_ = reference;
    (*map).dom = domid;
    (*map).status = 1;
}

#[inline]
pub unsafe fn gnttab_set_unmap_op(unmap: *mut gnttab_unmap_grant_ref, addr: phys_addr_t, flags: u32, handle: grant_handle_t) {
    (*unmap).host_addr = if flags & GNTMAP_contains_pte != 0 { addr } else if !xen_pv_domain() { __pa(addr) } else { addr };
    (*unmap).handle = handle;
    (*unmap).dev_bus_addr = 0;
}

#[repr(C)]
pub struct grant_frames { pub pfn: *mut xen_pfn_t, pub count: c_uint, pub vaddr: *mut core::ffi::c_void }
extern "C" { pub static mut xen_auto_xlat_grant_frames: grant_frames; }

#[inline]
pub unsafe fn gnttab_map_vaddr(map: &gnttab_map_grant_ref) -> *mut core::ffi::c_void { map.host_virt_addr as *mut core::ffi::c_void }

#[repr(C)]
pub struct gnttab_page_cache {
    pub lock: spinlock_t,
    #[cfg(CONFIG_XEN_UNPOPULATED_ALLOC)] pub pages: *mut page,
    #[cfg(not(CONFIG_XEN_UNPOPULATED_ALLOC))] pub pages: list_head,
    pub num_pages: c_uint,
}

#[cfg(CONFIG_XEN_GRANT_DMA_ALLOC)]
#[repr(C)]
pub struct gnttab_dma_alloc_args {
    pub dev: *mut device,
    pub coherent: bool,
    pub nr_pages: i32,
    pub pages: *mut *mut page,
    pub frames: *mut xen_pfn_t,
    pub vaddr: *mut core::ffi::c_void,
    pub dev_bus_addr: dma_addr_t,
}

#[cfg(CONFIG_XEN_GRANT_DMA_ALLOC)]
extern "C" { pub fn gnttab_dma_alloc_pages(args: *mut gnttab_dma_alloc_args) -> i32; pub fn gnttab_dma_free_pages(args: *mut gnttab_dma_alloc_args) -> i32; }

#[repr(C)]
pub struct xen_page_foreign { pub domid: domid_t, pub gref: grant_ref_t }

extern "C" { pub fn PageForeign(page: *mut page) -> bool; }

#[inline]
pub unsafe fn xen_page_foreign(page: *mut page) -> *mut xen_page_foreign {
    if !PageForeign(page) { return core::ptr::null_mut(); }
    (*page).private as *mut xen_page_foreign
}

pub type xen_grant_fn_t = Option<unsafe extern "C" fn(c_ulong, c_uint, c_uint, *mut core::ffi::c_void)>;

#[inline]
pub unsafe fn gnttab_for_one_grant(page: *mut page, offset: c_uint, mut len: c_uint, fn_: xen_grant_fn_t, data: *mut core::ffi::c_void) {
    len = core::cmp::min(XEN_PAGE_SIZE - (offset & !XEN_PAGE_MASK), len);
    gnttab_foreach_grant_in_range(page, offset, len, fn_, data);
}

#[inline]
pub unsafe fn gnttab_count_grant(start: c_uint, len: c_uint) -> c_uint {
    XEN_PFN_UP(xen_offset_in_page(start) + len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
