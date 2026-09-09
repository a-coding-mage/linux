/* SPDX-License-Identifier: GPL-2.0 */

/* Outside of CONFIG_HIGHMEM to support X86 32bit iomap_atomic() cruft. */

#[cfg(feature = "CONFIG_KMAP_LOCAL")]
extern "C" {
    pub fn __kmap_local_pfn_prot(pfn: ::core::ffi::c_ulong, prot: pgprot_t) -> *mut ::core::ffi::c_void;
    pub fn __kmap_local_page_prot(page: *const page, prot: pgprot_t) -> *mut ::core::ffi::c_void;
    pub fn kunmap_local_indexed(vaddr: *const ::core::ffi::c_void);
    pub fn kmap_local_fork(tsk: *mut task_struct);
    pub fn __kmap_local_sched_out();
    pub fn __kmap_local_sched_in();
}

#[cfg(not(feature = "CONFIG_KMAP_LOCAL"))]
#[inline]
pub unsafe fn kmap_local_fork(_tsk: *mut task_struct) {}

#[cfg(feature = "CONFIG_KMAP_LOCAL")]
#[inline]
pub unsafe fn kmap_assert_nomap() { DEBUG_LOCKS_WARN_ON((*current).kmap_ctrl.idx); }
#[cfg(not(feature = "CONFIG_KMAP_LOCAL"))]
#[inline]
pub unsafe fn kmap_assert_nomap() {}

#[cfg(feature = "CONFIG_HIGHMEM")]
extern "C" {
    pub fn kmap_high(page: *mut page) -> *mut ::core::ffi::c_void;
    pub fn kunmap_high(page: *const page);
    pub fn __kmap_flush_unused();
    pub fn __kmap_to_page(addr: *mut ::core::ffi::c_void) -> *mut page;
    pub fn might_sleep();
    pub fn PageHighMem(page: *const page) -> bool;
    pub fn page_address(page: *const page) -> *mut ::core::ffi::c_void;
    pub fn kmap_flush_tlb(addr: ::core::ffi::c_ulong);
    pub fn folio_page(folio: *const folio, index: usize) -> *const page;
    pub fn migrate_disable();
    pub fn preempt_disable();
    pub fn pagefault_disable();
    pub fn migrate_enable();
    pub fn preempt_enable();
    pub fn pagefault_enable();
    pub fn PKMAP_ADDR(index: usize) -> ::core::ffi::c_ulong;
    pub fn __fix_to_virt(x: usize) -> ::core::ffi::c_ulong;
}

#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline]
pub unsafe fn kmap(page_: *mut page) -> *mut ::core::ffi::c_void {
    might_sleep();
    let addr = if !PageHighMem(page_) { page_address(page_) } else { kmap_high(page_) };
    kmap_flush_tlb(addr as ::core::ffi::c_ulong);
    addr
}

#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline]
pub unsafe fn kunmap(page_: *const page) {
    might_sleep();
    if !PageHighMem(page_) { return; }
    kunmap_high(page_);
}

#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_to_page(addr: *mut ::core::ffi::c_void) -> *mut page { __kmap_to_page(addr) }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_flush_unused() { __kmap_flush_unused(); }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_local_page(page_: *const page) -> *mut ::core::ffi::c_void { __kmap_local_page_prot(page_, kmap_prot()) }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_local_page_try_from_panic(page_: *const page) -> *mut ::core::ffi::c_void { if !PageHighMem(page_) { page_address(page_) } else { ::core::ptr::null_mut() } }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_local_folio(folio_: *const folio, offset: usize) -> *mut ::core::ffi::c_void { (__kmap_local_page_prot(folio_page(folio_, offset / PAGE_SIZE), kmap_prot()) as *mut u8).add(offset % PAGE_SIZE) as *mut _ }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_local_page_prot(page_: *const page, prot: pgprot_t) -> *mut ::core::ffi::c_void { __kmap_local_page_prot(page_, prot) }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_local_pfn(pfn: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void { __kmap_local_pfn_prot(pfn, kmap_prot()) }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn __kunmap_local(addr: *const ::core::ffi::c_void) { kunmap_local_indexed(addr); }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_atomic_prot(page_: *const page, prot: pgprot_t) -> *mut ::core::ffi::c_void { if IS_ENABLED_PREEMPT_RT() { migrate_disable(); } else { preempt_disable(); } pagefault_disable(); __kmap_local_page_prot(page_, prot) }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_atomic(page_: *const page) -> *mut ::core::ffi::c_void { kmap_atomic_prot(page_, kmap_prot()) }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn kmap_atomic_pfn(pfn: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void { if IS_ENABLED_PREEMPT_RT() { migrate_disable(); } else { preempt_disable(); } pagefault_disable(); __kmap_local_pfn_prot(pfn, kmap_prot()) }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn __kunmap_atomic(addr: *const ::core::ffi::c_void) { kunmap_local_indexed(addr); pagefault_enable(); if IS_ENABLED_PREEMPT_RT() { migrate_enable(); } else { preempt_enable(); } }

#[cfg(feature = "CONFIG_HIGHMEM")]
extern "C" { pub fn __nr_free_highpages() -> ::core::ffi::c_ulong; pub fn __totalhigh_pages() -> ::core::ffi::c_ulong; }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn nr_free_highpages() -> ::core::ffi::c_ulong { __nr_free_highpages() }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn totalhigh_pages() -> ::core::ffi::c_ulong { __totalhigh_pages() }
#[cfg(feature = "CONFIG_HIGHMEM")]
#[inline] pub unsafe fn is_kmap_addr(x: *const ::core::ffi::c_void) -> bool { let addr=x as ::core::ffi::c_ulong; (addr >= PKMAP_ADDR(0) && addr < PKMAP_ADDR(LAST_PKMAP)) || (addr >= __fix_to_virt(FIX_KMAP_END) && addr < __fix_to_virt(FIX_KMAP_BEGIN)) }

#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_to_page(addr: *mut ::core::ffi::c_void) -> *mut page { virt_to_page(addr) }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap(page_: *mut page) -> *mut ::core::ffi::c_void { might_sleep(); page_address(page_) }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kunmap_high(_page: *const page) {}
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_flush_unused() {}
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kunmap(_page: *const page) {}
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_local_page(page_: *const page) -> *mut ::core::ffi::c_void { page_address(page_) }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_local_page_try_from_panic(page_: *const page) -> *mut ::core::ffi::c_void { page_address(page_) }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_local_folio(folio_: *const folio, offset: usize) -> *mut ::core::ffi::c_void { (folio_address(folio_) as *mut u8).add(offset) as *mut _ }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_local_page_prot(page_: *const page, _prot: pgprot_t) -> *mut ::core::ffi::c_void { kmap_local_page(page_) }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_local_pfn(pfn: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void { kmap_local_page(pfn_to_page(pfn)) }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn __kunmap_local(_addr: *const ::core::ffi::c_void) {}
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_atomic(page_: *const page) -> *mut ::core::ffi::c_void { if IS_ENABLED_PREEMPT_RT() { migrate_disable(); } else { preempt_disable(); } pagefault_disable(); page_address(page_) }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_atomic_prot(page_: *const page, _prot: pgprot_t) -> *mut ::core::ffi::c_void { kmap_atomic(page_) }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn kmap_atomic_pfn(pfn: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void { kmap_atomic(pfn_to_page(pfn)) }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn __kunmap_atomic(_addr: *const ::core::ffi::c_void) { pagefault_enable(); if IS_ENABLED_PREEMPT_RT() { migrate_enable(); } else { preempt_enable(); } }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn nr_free_highpages() -> ::core::ffi::c_ulong { 0 }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn totalhigh_pages() -> ::core::ffi::c_ulong { 0 }
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
#[inline] pub unsafe fn is_kmap_addr(_x: *const ::core::ffi::c_void) -> bool { false }

/* kunmap_atomic() and kunmap_local() retain the C type-checking macro intent. */
#[inline] pub unsafe fn kunmap_atomic_macro(addr: *const ::core::ffi::c_void) { __kunmap_atomic(addr); }
#[inline] pub unsafe fn kunmap_local_macro(addr: *const ::core::ffi::c_void) { __kunmap_local(addr); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
