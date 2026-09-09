/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/mmzone.h, and linux/stacktrace.h

pub struct pglist_data;

#[cfg(CONFIG_PAGE_EXTENSION)]
/// Per-page_ext client operations.
#[repr(C)]
pub struct page_ext_operations {
    pub offset: usize,
    pub size: usize,
    pub need: Option<unsafe extern "C" fn() -> bool>,
    pub init: Option<unsafe extern "C" fn()>,
    pub need_shared_flags: bool,
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[repr(C)]
pub enum page_ext_flags {
    PAGE_EXT_OWNER,
    PAGE_EXT_OWNER_ALLOCATED,
    #[cfg(all(CONFIG_PAGE_IDLE_FLAG, not(CONFIG_64BIT)))]
    PAGE_EXT_YOUNG,
    #[cfg(all(CONFIG_PAGE_IDLE_FLAG, not(CONFIG_64BIT)))]
    PAGE_EXT_IDLE,
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[repr(C)]
pub struct page_ext {
    pub flags: c_ulong,
}

#[cfg(CONFIG_PAGE_EXTENSION)]
extern "C" {
    pub static mut early_page_ext: bool;
    pub static mut page_ext_size: c_ulong;
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[inline]
pub unsafe fn early_page_ext_enabled() -> bool {
    early_page_ext
}

#[cfg(all(CONFIG_PAGE_EXTENSION, CONFIG_SPARSEMEM))]
#[inline]
pub unsafe fn page_ext_init_flatmem() {}

#[cfg(all(CONFIG_PAGE_EXTENSION, CONFIG_SPARSEMEM))]
extern "C" {
    pub fn page_ext_init();
}

#[cfg(all(CONFIG_PAGE_EXTENSION, CONFIG_SPARSEMEM))]
#[inline]
pub unsafe fn page_ext_init_flatmem_late() {}

#[cfg(all(CONFIG_PAGE_EXTENSION, CONFIG_SPARSEMEM))]
#[inline]
pub unsafe fn page_ext_iter_next_fast_possible(next_pfn: c_ulong) -> bool {
    next_pfn % PAGES_PER_SECTION != 0
}

#[cfg(all(CONFIG_PAGE_EXTENSION, not(CONFIG_SPARSEMEM)))]
extern "C" {
    pub fn page_ext_init_flatmem();
    pub fn page_ext_init_flatmem_late();
}

#[cfg(all(CONFIG_PAGE_EXTENSION, not(CONFIG_SPARSEMEM)))]
#[inline]
pub unsafe fn page_ext_init() {}

#[cfg(all(CONFIG_PAGE_EXTENSION, not(CONFIG_SPARSEMEM)))]
#[inline]
pub unsafe fn page_ext_iter_next_fast_possible(_next_pfn: c_ulong) -> bool {
    true
}

#[cfg(CONFIG_PAGE_EXTENSION)]
extern "C" {
    pub fn page_ext_get(page: *const page) -> *mut page_ext;
    pub fn page_ext_from_phys(phys: phys_addr_t) -> *mut page_ext;
    pub fn page_ext_put(page_ext: *mut page_ext);
    pub fn page_ext_lookup(pfn: c_ulong) -> *mut page_ext;
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[inline]
pub unsafe fn page_ext_data(
    page_ext: *mut page_ext,
    ops: *mut page_ext_operations,
) -> *mut c_void {
    (page_ext as *mut u8).add((*ops).offset) as *mut c_void
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[inline]
pub unsafe fn page_ext_next(curr: *mut page_ext) -> *mut page_ext {
    (curr as *mut u8).add(page_ext_size as usize) as *mut page_ext
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[repr(C)]
pub struct page_ext_iter {
    pub index: c_ulong,
    pub start_pfn: c_ulong,
    pub page_ext: *mut page_ext,
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[inline]
pub unsafe fn page_ext_iter_begin(
    iter: *mut page_ext_iter,
    pfn: c_ulong,
    count: c_ulong,
) -> *mut page_ext {
    if count == 0 {
        return core::ptr::null_mut();
    }
    (*iter).index = 0;
    (*iter).start_pfn = pfn;
    (*iter).page_ext = page_ext_lookup(pfn);
    (*iter).page_ext
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[inline]
pub unsafe fn page_ext_iter_next(
    iter: *mut page_ext_iter,
    count: c_ulong,
) -> *mut page_ext {
    let pfn: c_ulong;
    if WARN_ON_ONCE((*iter).page_ext.is_null()) {
        return core::ptr::null_mut();
    }
    (*iter).index = (*iter).index.wrapping_add(1);
    if (*iter).index >= count {
        return core::ptr::null_mut();
    }
    pfn = (*iter).start_pfn.wrapping_add((*iter).index);
    if page_ext_iter_next_fast_possible(pfn) {
        (*iter).page_ext = page_ext_next((*iter).page_ext);
    } else {
        (*iter).page_ext = page_ext_lookup(pfn);
    }
    (*iter).page_ext
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[inline]
pub unsafe fn page_ext_iter_get(iter: *const page_ext_iter) -> *mut page_ext {
    (*iter).page_ext
}

#[cfg(CONFIG_PAGE_EXTENSION)]
#[macro_export]
macro_rules! for_each_page_ext {
    ($page:expr, $pgcount:expr, $page_ext:expr, $iter:expr) => {
        for $page_ext in $crate::page_ext_iter_begin(
            &mut $iter,
            page_to_pfn($page),
            $pgcount,
        ) {
            let _ = $page_ext;
        }
    };
}

#[cfg(not(CONFIG_PAGE_EXTENSION))]
pub struct page_ext;

#[cfg(not(CONFIG_PAGE_EXTENSION))]
#[inline]
pub unsafe fn early_page_ext_enabled() -> bool { false }

#[cfg(not(CONFIG_PAGE_EXTENSION))]
#[inline]
pub unsafe fn page_ext_init() {}

#[cfg(not(CONFIG_PAGE_EXTENSION))]
#[inline]
pub unsafe fn page_ext_init_flatmem_late() {}

#[cfg(not(CONFIG_PAGE_EXTENSION))]
#[inline]
pub unsafe fn page_ext_init_flatmem() {}

#[cfg(not(CONFIG_PAGE_EXTENSION))]
#[inline]
pub unsafe fn page_ext_get(_page: *const page) -> *mut page_ext { core::ptr::null_mut() }

#[cfg(not(CONFIG_PAGE_EXTENSION))]
#[inline]
pub unsafe fn page_ext_from_phys(_phys: phys_addr_t) -> *mut page_ext { core::ptr::null_mut() }

#[cfg(not(CONFIG_PAGE_EXTENSION))]
#[inline]
pub unsafe fn page_ext_put(_page_ext: *mut page_ext) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
