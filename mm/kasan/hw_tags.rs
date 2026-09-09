// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains core hardware tag-based KASAN code.
 *
 * Copyright (c) 2020 Google, Inc.
 * Author: Andrey Konovalov <andreyknvl@google.com>
 */

// C dependencies supplied by the surrounding kernel translation unit are intentionally omitted.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum KasanArg {
    KasanArgDefault,
    KasanArgOff,
    KasanArgOn,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum KasanArgMode {
    KasanArgModeDefault,
    KasanArgModeSync,
    KasanArgModeAsync,
    KasanArgModeAsymm,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum KasanArgVmalloc {
    KasanArgVmallocDefault,
    KasanArgVmallocOff,
    KasanArgVmallocOn,
}

static mut KASAN_ARG: KasanArg = KasanArg::KasanArgDefault;
static mut KASAN_ARG_MODE: KasanArgMode = KasanArgMode::KasanArgModeDefault;
static mut KASAN_ARG_VMALLOC: KasanArgVmalloc = KasanArgVmalloc::KasanArgVmallocDefault;

// Whether the selected mode is synchronous, asynchronous, or asymmetric.
// Defaults to KASAN_MODE_SYNC.
pub static mut kasan_mode: KasanMode = KASAN_MODE_SYNC;

// Whether to enable vmalloc tagging.
// CONFIG_KASAN_VMALLOC selects DEFINE_STATIC_KEY_TRUE; otherwise FALSE.
pub static mut kasan_flag_vmalloc: StaticKey = StaticKey::default();

// Whether to check write accesses only.
static mut KASAN_FLAG_WRITE_ONLY: bool = false;

const PAGE_ALLOC_SAMPLE_DEFAULT: u64 = 1;
const PAGE_ALLOC_SAMPLE_ORDER_DEFAULT: u32 = 3;

// Sampling interval of page_alloc allocation (un)poisoning. Defaults to no sampling.
pub static mut kasan_page_alloc_sample: u64 = PAGE_ALLOC_SAMPLE_DEFAULT;

// Minimum order of page_alloc allocations to be affected by sampling.
pub static mut kasan_page_alloc_sample_order: u32 = PAGE_ALLOC_SAMPLE_ORDER_DEFAULT;

// DEFINE_PER_CPU(long, kasan_page_alloc_skip);
extern "C" {
    static mut kasan_page_alloc_skip: isize;
}

unsafe fn early_kasan_flag(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() { return -EINVAL; }
    if c_str_eq(arg, b"off\0") { KASAN_ARG = KasanArg::KasanArgOff; }
    else if c_str_eq(arg, b"on\0") { KASAN_ARG = KasanArg::KasanArgOn; }
    else { return -EINVAL; }
    0
}

unsafe fn early_kasan_mode(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() { return -EINVAL; }
    if c_str_eq(arg, b"sync\0") { KASAN_ARG_MODE = KasanArgMode::KasanArgModeSync; }
    else if c_str_eq(arg, b"async\0") { KASAN_ARG_MODE = KasanArgMode::KasanArgModeAsync; }
    else if c_str_eq(arg, b"asymm\0") { KASAN_ARG_MODE = KasanArgMode::KasanArgModeAsymm; }
    else { return -EINVAL; }
    0
}

unsafe fn early_kasan_flag_vmalloc(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() { return -EINVAL; }
    // IS_ENABLED(CONFIG_KASAN_VMALLOC): return 0 when the configuration is disabled.
    if !CONFIG_KASAN_VMALLOC { return 0; }
    if c_str_eq(arg, b"off\0") { KASAN_ARG_VMALLOC = KasanArgVmalloc::KasanArgVmallocOff; }
    else if c_str_eq(arg, b"on\0") { KASAN_ARG_VMALLOC = KasanArgVmalloc::KasanArgVmallocOn; }
    else { return -EINVAL; }
    0
}

unsafe fn early_kasan_flag_write_only(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() { return -EINVAL; }
    if c_str_eq(arg, b"off\0") { KASAN_FLAG_WRITE_ONLY = false; }
    else if c_str_eq(arg, b"on\0") { KASAN_FLAG_WRITE_ONLY = true; }
    else { return -EINVAL; }
    0
}

unsafe fn kasan_mode_info() -> *const core::ffi::c_char {
    if kasan_mode == KASAN_MODE_ASYNC { b"async\0".as_ptr() as *const _ }
    else if kasan_mode == KASAN_MODE_ASYMM { b"asymm\0".as_ptr() as *const _ }
    else { b"sync\0".as_ptr() as *const _ }
}

unsafe fn early_kasan_flag_page_alloc_sample(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() { return -EINVAL; }
    let rv = kstrtoul(arg, 0, &mut kasan_page_alloc_sample);
    if rv != 0 { return rv; }
    if kasan_page_alloc_sample == 0 || kasan_page_alloc_sample > LONG_MAX as u64 {
        kasan_page_alloc_sample = PAGE_ALLOC_SAMPLE_DEFAULT;
        return -EINVAL;
    }
    0
}

unsafe fn early_kasan_flag_page_alloc_sample_order(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() { return -EINVAL; }
    let rv = kstrtouint(arg, 0, &mut kasan_page_alloc_sample_order);
    if rv != 0 { return rv; }
    if kasan_page_alloc_sample_order > INT_MAX as u32 {
        kasan_page_alloc_sample_order = PAGE_ALLOC_SAMPLE_ORDER_DEFAULT;
        return -EINVAL;
    }
    0
}

pub unsafe fn kasan_init_hw_tags_cpu() {
    if KASAN_ARG == KasanArg::KasanArgOff { return; }
    kasan_enable_hw_tags();
}

pub unsafe fn kasan_init_hw_tags() {
    if !system_supports_mte() || KASAN_ARG == KasanArg::KasanArgOff { return; }
    match KASAN_ARG_MODE {
        KasanArgMode::KasanArgModeDefault => (),
        KasanArgMode::KasanArgModeSync => kasan_mode = KASAN_MODE_SYNC,
        KasanArgMode::KasanArgModeAsync => kasan_mode = KASAN_MODE_ASYNC,
        KasanArgMode::KasanArgModeAsymm => kasan_mode = KASAN_MODE_ASYMM,
    }
    match KASAN_ARG_VMALLOC {
        KasanArgVmalloc::KasanArgVmallocDefault => (),
        KasanArgVmalloc::KasanArgVmallocOff => static_branch_disable(&mut kasan_flag_vmalloc),
        KasanArgVmalloc::KasanArgVmallocOn => static_branch_enable(&mut kasan_flag_vmalloc),
    }
    kasan_init_tags();
    kasan_enable();
    pr_info!("KernelAddressSanitizer initialized (hw-tags, mode={}, vmalloc={}, stacktrace={}, write_only={})\n", kasan_mode_info(), str_on_off(kasan_vmalloc_enabled()), str_on_off(kasan_stack_collection_enabled()), str_on_off(KASAN_FLAG_WRITE_ONLY));
}

// CONFIG_KASAN_VMALLOC-gated definitions are preserved below.
#[cfg(CONFIG_KASAN_VMALLOC)]
unsafe fn unpoison_vmalloc_pages(addr: *const core::ffi::c_void, tag: u8) {
    let area = find_vm_area(addr as *mut core::ffi::c_void);
    if area.is_null() { WARN_ON(true); return; }
    for i in 0..(*area).nr_pages {
        let page = *(*area).pages.add(i as usize);
        page_kasan_tag_set(page, tag);
    }
}

#[cfg(CONFIG_KASAN_VMALLOC)]
unsafe fn init_vmalloc_pages(start: *const core::ffi::c_void, size: u64) {
    let mut addr = start as usize;
    let end = addr.wrapping_add(size as usize);
    while addr < end {
        clear_highpage_kasan_tagged(vmalloc_to_page(addr as *const _));
        addr = addr.wrapping_add(PAGE_SIZE as usize);
    }
}

#[cfg(CONFIG_KASAN_VMALLOC)]
pub unsafe fn __kasan_unpoison_vmalloc(start: *const core::ffi::c_void, size: u64, flags: KasanVmallocFlags) -> *mut core::ffi::c_void {
    if !kasan_vmalloc_enabled() {
        if flags & KASAN_VMALLOC_INIT != 0 { init_vmalloc_pages(start, size); }
        return start as *mut _;
    }
    if flags & KASAN_VMALLOC_VM_ALLOC == 0 || flags & KASAN_VMALLOC_PROT_NORMAL == 0 {
        if flags & KASAN_VMALLOC_INIT != 0 { WARN_ON(true); }
        return start as *mut _;
    }
    let tag = if flags & KASAN_VMALLOC_KEEP_TAG != 0 { get_tag(start) } else { kasan_random_tag() };
    let start = set_tag(start, tag);
    kasan_unpoison(start, size, flags & KASAN_VMALLOC_INIT);
    let redzone_start = round_up((start as usize).wrapping_add(size as usize), KASAN_GRANULE_SIZE as usize);
    let redzone_size = round_up(redzone_start, PAGE_SIZE as usize).wrapping_sub(redzone_start);
    kasan_poison(redzone_start as *mut _, redzone_size as u64, KASAN_TAG_INVALID, flags & KASAN_VMALLOC_INIT);
    unpoison_vmalloc_pages(start, tag);
    start as *mut _
}

#[cfg(CONFIG_KASAN_VMALLOC)]
pub unsafe fn __kasan_poison_vmalloc(_start: *const core::ffi::c_void, _size: u64) {}

pub unsafe fn kasan_enable_hw_tags() {
    if KASAN_ARG_MODE == KasanArgMode::KasanArgModeAsync { hw_enable_tag_checks_async(); }
    else if KASAN_ARG_MODE == KasanArgMode::KasanArgModeAsymm { hw_enable_tag_checks_asymm(); }
    else { hw_enable_tag_checks_sync(); }
    if KASAN_FLAG_WRITE_ONLY && hw_enable_tag_checks_write_only() {
        KASAN_FLAG_WRITE_ONLY = false;
        pr_err_once!("write-only mode is not supported and thus not enabled\n");
    }
}

#[cfg(CONFIG_KASAN_KUNIT_TEST)]
pub unsafe fn kasan_force_async_fault() { hw_force_async_tag_fault(); }

#[cfg(CONFIG_KASAN_KUNIT_TEST)]
pub unsafe fn kasan_write_only_enabled() -> bool { KASAN_FLAG_WRITE_ONLY }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
