// SPDX-License-Identifier: GPL-2.0
/*
 * KMSAN shadow implementation.
 *
 * Copyright (C) 2017-2022 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

#[repr(C)]
pub struct page {
    pub kmsan_shadow: *mut page,
    pub kmsan_origin: *mut page,
}

pub type u64 = u64;
pub type gfp_t = usize;
pub type pgprot_t = usize;
pub type depot_stack_handle_t = usize;

extern "C" {
    static mut kmsan_enabled: bool;
    static mut dummy_load_page: [i8; PAGE_SIZE];
    static mut dummy_store_page: [i8; PAGE_SIZE];

    fn page_address(page: *mut page) -> *mut core::ffi::c_void;
    fn kmsan_internal_is_vmalloc_addr(addr: *mut core::ffi::c_void) -> bool;
    fn kmsan_internal_is_module_addr(addr: *mut core::ffi::c_void) -> bool;
    fn kmsan_virt_addr_valid(addr: *mut core::ffi::c_void) -> bool;
    fn virt_to_page(addr: *mut core::ffi::c_void) -> *mut page;
    fn arch_kmsan_get_meta_or_null(addr: *mut core::ffi::c_void, is_origin: bool) -> *mut core::ffi::c_void;
    fn kmsan_metadata_is_contiguous(addr: *mut core::ffi::c_void, size: u64) -> bool;
    fn kmsan_in_runtime() -> bool;
    fn kmsan_enter_runtime();
    fn kmsan_leave_runtime();
    fn kmsan_internal_unpoison_memory(addr: *mut core::ffi::c_void, size: usize, checked: bool);
    fn kmsan_internal_poison_memory(addr: *mut core::ffi::c_void, size: usize, flags: gfp_t, poison: u32);
    fn kmsan_save_stack_with_flags(flags: gfp_t, extra_bits: u32) -> depot_stack_handle_t;
    fn __memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize);
    fn __memset(dst: *mut core::ffi::c_void, value: i32, size: usize);
    fn kzalloc_objs<T>(obj: T, nr: i32, flags: gfp_t) -> *mut *mut page;
    fn __vmap_pages_range_noflush(start: usize, end: usize, prot: pgprot_t, pages: *mut *mut page, page_shift: u32) -> i32;
    fn flush_tlb_kernel_range(start: usize, end: usize);
    fn flush_cache_vmap(start: usize, end: usize);
    fn kfree(ptr: *mut *mut page);
    fn memblock_alloc_or_panic(size: u64, align: usize) -> *mut core::ffi::c_void;
    fn KMSAN_WARN_ON(condition: bool);
}

const PAGE_SIZE: usize = 4096;
const KMSAN_ORIGIN_SIZE: u64 = 4;
const KMSAN_META_SHADOW: bool = false;
const KMSAN_META_ORIGIN: bool = true;
const KMSAN_VMALLOC_ORIGIN_START: usize = 0;
const KMSAN_VMALLOC_SHADOW_START: usize = 0;
const KMSAN_MODULES_ORIGIN_START: usize = 0;
const KMSAN_MODULES_SHADOW_START: usize = 0;
const VMALLOC_START: usize = 0;
const MODULES_VADDR: usize = 0;
const PAGE_KERNEL: pgprot_t = 0;
const __GFP_ZERO: gfp_t = 0;
const GFP_KERNEL: gfp_t = 0;
const __GFP_RECLAIM: gfp_t = 0;
const KMSAN_POISON_CHECK: u32 = 0;
const KMSAN_POISON_FREE: u32 = 0;

#[repr(C)]
pub struct shadow_origin_ptr {
    pub shadow: *mut core::ffi::c_void,
    pub origin: *mut core::ffi::c_void,
}

unsafe fn shadow_ptr_for(page: *mut page) -> *mut core::ffi::c_void { page_address((*page).kmsan_shadow) }
unsafe fn origin_ptr_for(page: *mut page) -> *mut core::ffi::c_void { page_address((*page).kmsan_origin) }
unsafe fn page_has_metadata(page: *mut page) -> bool { !(*page).kmsan_shadow.is_null() && !(*page).kmsan_origin.is_null() }
unsafe fn set_no_shadow_origin_page(page: *mut page) { (*page).kmsan_shadow = core::ptr::null_mut(); (*page).kmsan_origin = core::ptr::null_mut(); }

unsafe fn vmalloc_meta(addr: *mut core::ffi::c_void, is_origin: bool) -> usize {
    let addr64 = addr as usize;
    KMSAN_WARN_ON(is_origin && addr64 % KMSAN_ORIGIN_SIZE as usize != 0);
    if kmsan_internal_is_vmalloc_addr(addr) { return addr64 - VMALLOC_START + if is_origin { KMSAN_VMALLOC_ORIGIN_START } else { KMSAN_VMALLOC_SHADOW_START }; }
    if kmsan_internal_is_module_addr(addr) { return addr64 - MODULES_VADDR + if is_origin { KMSAN_MODULES_ORIGIN_START } else { KMSAN_MODULES_SHADOW_START }; }
    0
}

unsafe fn virt_to_page_or_null(vaddr: *mut core::ffi::c_void) -> *mut page { if kmsan_virt_addr_valid(vaddr) { virt_to_page(vaddr) } else { core::ptr::null_mut() } }

pub unsafe fn kmsan_get_shadow_origin_ptr(address: *mut core::ffi::c_void, size: u64, store: bool) -> shadow_origin_ptr {
    let mut ret: shadow_origin_ptr;
    KMSAN_WARN_ON(size > PAGE_SIZE as u64);
    if !kmsan_enabled { return dummy_ptr(store); }
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(address, size));
    let shadow = kmsan_get_metadata(address, KMSAN_META_SHADOW);
    if shadow.is_null() { return dummy_ptr(store); }
    ret = shadow_origin_ptr { shadow, origin: kmsan_get_metadata(address, KMSAN_META_ORIGIN) };
    ret
}

unsafe fn dummy_ptr(store: bool) -> shadow_origin_ptr {
    let p = if store { dummy_store_page.as_mut_ptr() as *mut core::ffi::c_void } else { dummy_load_page.as_mut_ptr() as *mut core::ffi::c_void };
    shadow_origin_ptr { shadow: p, origin: p }
}

pub unsafe fn kmsan_get_metadata(mut address: *mut core::ffi::c_void, is_origin: bool) -> *mut core::ffi::c_void {
    let mut addr = address as u64;
    if is_origin { addr &= !(KMSAN_ORIGIN_SIZE - 1); }
    address = addr as usize as *mut core::ffi::c_void;
    if kmsan_internal_is_vmalloc_addr(address) || kmsan_internal_is_module_addr(address) { return vmalloc_meta(address, is_origin) as *mut core::ffi::c_void; }
    let ret = arch_kmsan_get_meta_or_null(address, is_origin);
    if !ret.is_null() { return ret; }
    let page = virt_to_page_or_null(address);
    if page.is_null() || !page_has_metadata(page) { return core::ptr::null_mut(); }
    let off = (addr as usize) & (PAGE_SIZE - 1);
    (if is_origin { origin_ptr_for(page) } else { shadow_ptr_for(page) }).add(off)
}

pub unsafe fn kmsan_copy_page_meta(dst: *mut page, src: *mut page) {
    if !kmsan_enabled || kmsan_in_runtime() || dst.is_null() || !page_has_metadata(dst) { return; }
    if src.is_null() || !page_has_metadata(src) { kmsan_internal_unpoison_memory(page_address(dst), PAGE_SIZE, false); return; }
    kmsan_enter_runtime(); __memcpy(shadow_ptr_for(dst), shadow_ptr_for(src), PAGE_SIZE); __memcpy(origin_ptr_for(dst), origin_ptr_for(src), PAGE_SIZE); kmsan_leave_runtime();
}

pub unsafe fn kmsan_alloc_page(page: *mut page, order: u32, flags: gfp_t) {
    let initialized = (flags & __GFP_ZERO) != 0 || !kmsan_enabled;
    if page.is_null() { return; }
    let shadow = (*page).kmsan_shadow; let origin = (*page).kmsan_origin; let pages = 1usize << order;
    if initialized { __memset(page_address(shadow), 0, PAGE_SIZE * pages); __memset(page_address(origin), 0, PAGE_SIZE * pages); return; }
    if kmsan_in_runtime() { return; }
    __memset(page_address(shadow), -1, PAGE_SIZE * pages); kmsan_enter_runtime(); let handle = kmsan_save_stack_with_flags(flags, 0); kmsan_leave_runtime();
    for i in 0..(PAGE_SIZE * pages / core::mem::size_of::<depot_stack_handle_t>()) { *(page_address(origin) as *mut depot_stack_handle_t).add(i) = handle; }
}

pub unsafe fn kmsan_free_page(page: *mut page, order: u32) {
    if !kmsan_enabled || kmsan_in_runtime() { return; }
    kmsan_enter_runtime(); kmsan_internal_poison_memory(page_address(page), PAGE_SIZE << order, GFP_KERNEL & !__GFP_RECLAIM, KMSAN_POISON_CHECK | KMSAN_POISON_FREE); kmsan_leave_runtime();
}

pub unsafe fn kmsan_vmap_pages_range_noflush(start: usize, end: usize, mut prot: pgprot_t, pages: *mut *mut page, page_shift: u32, gfp_mask: gfp_t) -> i32 {
    if !kmsan_enabled { return 0; }
    let shadow_start = vmalloc_meta(start as *mut _, KMSAN_META_SHADOW); let shadow_end = vmalloc_meta(end as *mut _, KMSAN_META_SHADOW); if shadow_start == 0 { return 0; }
    let nr = ((end - start) / PAGE_SIZE) as i32; let s_pages = kzalloc_objs(core::ptr::null_mut::<page>(), nr, gfp_mask); let o_pages = kzalloc_objs(core::ptr::null_mut::<page>(), nr, gfp_mask); let mut err = 0;
    if s_pages.is_null() || o_pages.is_null() { err = -12; } else {
        for i in 0..nr as isize { *s_pages.offset(i) = (*pages.offset(i)).kmsan_shadow; *o_pages.offset(i) = (*pages.offset(i)).kmsan_origin; }
        prot = PAGE_KERNEL; let origin_start = vmalloc_meta(start as *mut _, KMSAN_META_ORIGIN); let origin_end = vmalloc_meta(end as *mut _, KMSAN_META_ORIGIN);
        kmsan_enter_runtime(); let mapped = __vmap_pages_range_noflush(shadow_start, shadow_end, prot, s_pages, page_shift); kmsan_leave_runtime();
        if mapped != 0 { err = mapped; } else { kmsan_enter_runtime(); let mapped = __vmap_pages_range_noflush(origin_start, origin_end, prot, o_pages, page_shift); kmsan_leave_runtime(); if mapped != 0 { err = mapped; } else { flush_tlb_kernel_range(shadow_start, shadow_end); flush_tlb_kernel_range(origin_start, origin_end); flush_cache_vmap(shadow_start, shadow_end); flush_cache_vmap(origin_start, origin_end); } }
    }
    if !s_pages.is_null() { kfree(s_pages); } if !o_pages.is_null() { kfree(o_pages); } err
}

pub unsafe fn kmsan_init_alloc_meta_for_range(mut start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) {
    let start_addr = (start as u64) & !(PAGE_SIZE as u64 - 1); let size = ((end as u64 - start_addr) + PAGE_SIZE as u64 - 1) & !(PAGE_SIZE as u64 - 1); start = start_addr as usize as *mut _;
    let shadow = memblock_alloc_or_panic(size, PAGE_SIZE); let origin = memblock_alloc_or_panic(size, PAGE_SIZE);
    let mut addr = 0; while addr < size { let page = virt_to_page_or_null((start as usize + addr as usize) as *mut _); let shadow_p = virt_to_page((shadow as usize + addr as usize) as *mut _); set_no_shadow_origin_page(shadow_p); (*page).kmsan_shadow = shadow_p; let origin_p = virt_to_page((origin as usize + addr as usize) as *mut _); set_no_shadow_origin_page(origin_p); (*page).kmsan_origin = origin_p; addr += PAGE_SIZE as u64; }
}

pub unsafe fn kmsan_setup_meta(page: *mut page, shadow: *mut page, origin: *mut page, order: u32) {
    for i in 0..(1usize << order) { let p = page.add(i); let s = shadow.add(i); let o = origin.add(i); set_no_shadow_origin_page(s); set_no_shadow_origin_page(o); (*p).kmsan_shadow = s; (*p).kmsan_origin = o; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
