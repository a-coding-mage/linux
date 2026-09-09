/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: declarations supplied by linux/mm.h and related architecture headers.

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    pub static mut local_flush_cache_all: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;
    pub static mut local_flush_cache_mm: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;
    pub static mut local_flush_cache_dup_mm: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;
    pub static mut local_flush_cache_page: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;
    pub static mut local_flush_cache_range: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;
    pub static mut local_flush_dcache_folio: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;
    pub static mut local_flush_icache_range: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;
    pub static mut local_flush_icache_folio: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;
    pub static mut local_flush_cache_sigtramp: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;

    pub static mut __flush_wback_region: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>;
    pub static mut __flush_purge_region: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>;
    pub static mut __flush_invalidate_region: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>;

    pub fn flush_cache_all();
    pub fn flush_cache_mm(mm: *mut mm_struct);
    pub fn flush_cache_dup_mm(mm: *mut mm_struct);
    pub fn flush_cache_page(vma: *mut vm_area_struct, addr: usize, pfn: usize);
    pub fn flush_cache_range(vma: *mut vm_area_struct, start: usize, end: usize);
    pub fn flush_dcache_folio(folio: *mut folio);
    pub fn flush_icache_range(start: usize, end: usize);
    pub fn flush_icache_pages(vma: *mut vm_area_struct, page: *mut page, nr: u32);
    pub fn flush_cache_sigtramp(address: usize);
    pub fn __flush_anon_page(page: *mut page, vmaddr: usize);
    pub fn copy_to_user_page(vma: *mut vm_area_struct, page: *mut page, vaddr: usize,
                             dst: *mut core::ffi::c_void, src: *const core::ffi::c_void,
                             len: usize);
    pub fn copy_from_user_page(vma: *mut vm_area_struct, page: *mut page, vaddr: usize,
                               dst: *mut core::ffi::c_void, src: *const core::ffi::c_void,
                               len: usize);
    pub fn kmap_coherent_init();
    pub fn kmap_coherent(page: *mut page, addr: usize) -> *mut core::ffi::c_void;
    pub fn kunmap_coherent(kvaddr: *mut core::ffi::c_void);
    pub fn cpu_cache_init();
    pub fn l2_cache_init();
    pub fn j2_cache_init();
    pub fn sh2_cache_init();
    pub fn sh2a_cache_init();
    pub fn sh3_cache_init();
    pub fn shx3_cache_init();
    pub fn sh4_cache_init();
    pub fn sh7705_cache_init();
    pub fn sh4__flush_region_init();
}

#[inline]
pub unsafe fn cache_noop(_args: *mut core::ffi::c_void) {}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

#[inline]
pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

pub const ARCH_HAS_FLUSH_ANON_PAGE: bool = true;

#[inline]
pub unsafe fn flush_anon_page(vma: *mut vm_area_struct, page: *mut page, vmaddr: usize) {
    if boot_cpu_data_dcache_n_aliases() != 0 && PageAnon(page) {
        __flush_anon_page(page, vmaddr);
    }
}

pub const ARCH_IMPLEMENTS_FLUSH_KERNEL_VMAP_RANGE: i32 = 1;

#[inline]
pub unsafe fn flush_kernel_vmap_range(addr: *mut core::ffi::c_void, size: i32) {
    if let Some(f) = __flush_wback_region { f(addr, size); }
}

#[inline]
pub unsafe fn invalidate_kernel_vmap_range(addr: *mut core::ffi::c_void, size: i32) {
    if let Some(f) = __flush_invalidate_region { f(addr, size); }
}

#[repr(C)]
pub struct flusher_data {
    pub vma: *mut vm_area_struct,
    pub addr1: usize,
    pub addr2: usize,
}

#[inline]
pub unsafe fn sh_cacheop_vaddr(mut vaddr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    if __in_29bit_mode() {
        vaddr = CAC_ADDR(vaddr as usize) as *mut core::ffi::c_void;
    }
    vaddr
}

// C macros retained as Rust functions/constants; build-time architecture symbols are external.
pub const PG_dcache_clean: usize = PG_arch_1;
pub unsafe fn flush_cache_vmap(_start: usize, _end: usize) { if let Some(f) = local_flush_cache_all { f(core::ptr::null_mut()); } }
pub unsafe fn flush_cache_vmap_early(_start: usize, _end: usize) {}
pub unsafe fn flush_cache_vunmap(_start: usize, _end: usize) { if let Some(f) = local_flush_cache_all { f(core::ptr::null_mut()); } }
pub unsafe fn flush_dcache_mmap_lock(_mapping: *mut core::ffi::c_void) {}
pub unsafe fn flush_dcache_mmap_unlock(_mapping: *mut core::ffi::c_void) {}

extern "C" {
    fn page_folio(page: *mut page) -> *mut folio;
    fn PageAnon(page: *mut page) -> bool;
    fn boot_cpu_data_dcache_n_aliases() -> usize;
    fn __in_29bit_mode() -> bool;
    fn CAC_ADDR(addr: usize) -> usize;
}

extern "C" {
    static PG_arch_1: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
