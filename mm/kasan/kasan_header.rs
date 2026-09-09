/* SPDX-License-Identifier: GPL-2.0 */
// Translated from kasan.h. Kernel-provided types and functions are external dependencies.

#[cfg(any(CONFIG_KASAN_SW_TAGS, CONFIG_KASAN_HW_TAGS))]
extern "C" {
    pub static mut kasan_flag_stacktrace: static_key_true;
}

#[cfg(any(CONFIG_KASAN_SW_TAGS, CONFIG_KASAN_HW_TAGS))]
#[inline]
pub unsafe fn kasan_stack_collection_enabled() -> bool {
    static_branch_unlikely(&kasan_flag_stacktrace)
}

#[cfg(not(any(CONFIG_KASAN_SW_TAGS, CONFIG_KASAN_HW_TAGS)))]
#[inline]
pub unsafe fn kasan_stack_collection_enabled() -> bool { true }

#[cfg(CONFIG_KASAN_HW_TAGS)]
extern "C" {
    pub static mut kasan_flag_vmalloc: static_key_true;
    pub static mut kasan_mode: kasan_mode;
    pub static mut kasan_page_alloc_sample: c_ulong;
    pub static mut kasan_page_alloc_sample_order: c_uint;
}

#[cfg(CONFIG_KASAN_HW_TAGS)]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kasan_mode { KASAN_MODE_SYNC, KASAN_MODE_ASYNC, KASAN_MODE_ASYMM }

#[cfg(CONFIG_KASAN_HW_TAGS)]
#[inline]
pub unsafe fn kasan_vmalloc_enabled() -> bool { static_branch_likely(&kasan_flag_vmalloc) }

#[cfg(not(CONFIG_KASAN_HW_TAGS))]
#[inline]
pub unsafe fn kasan_vmalloc_enabled() -> bool { IS_ENABLED(CONFIG_KASAN_VMALLOC) }

#[cfg(CONFIG_KASAN_HW_TAGS)]
#[inline]
pub unsafe fn kasan_async_fault_possible() -> bool {
    kasan_mode == kasan_mode::KASAN_MODE_ASYNC || kasan_mode == kasan_mode::KASAN_MODE_ASYMM
}
#[cfg(not(CONFIG_KASAN_HW_TAGS))]
#[inline]
pub unsafe fn kasan_async_fault_possible() -> bool { false }

#[cfg(CONFIG_KASAN_HW_TAGS)]
#[inline]
pub unsafe fn kasan_sync_fault_possible() -> bool {
    kasan_mode == kasan_mode::KASAN_MODE_SYNC || kasan_mode == kasan_mode::KASAN_MODE_ASYMM
}
#[cfg(not(CONFIG_KASAN_HW_TAGS))]
#[inline]
pub unsafe fn kasan_sync_fault_possible() -> bool { true }

#[cfg(CONFIG_KASAN_HW_TAGS)]
#[inline]
pub unsafe fn kasan_sample_page_alloc(order: c_uint) -> bool {
    if kasan_page_alloc_sample == 1 || order < kasan_page_alloc_sample_order { return true; }
    if this_cpu_dec_return(&mut kasan_page_alloc_skip) < 0 {
        this_cpu_write(&mut kasan_page_alloc_skip, kasan_page_alloc_sample - 1);
        return true;
    }
    false
}
#[cfg(not(CONFIG_KASAN_HW_TAGS))]
#[inline]
pub unsafe fn kasan_sample_page_alloc(_order: c_uint) -> bool { true }

#[cfg(CONFIG_KASAN_GENERIC)]
#[inline] pub const fn kasan_requires_meta() -> bool { true }
#[cfg(not(CONFIG_KASAN_GENERIC))]
#[inline] pub const fn kasan_requires_meta() -> bool { false }

// KASAN_GRANULE_SIZE uses KASAN_SHADOW_SCALE_SHIFT for generic/SW-tags and MTE_GRANULE_SIZE otherwise.
#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
pub const KASAN_GRANULE_SIZE: c_ulong = 1u64 << KASAN_SHADOW_SCALE_SHIFT;
#[cfg(not(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS)))]
pub const KASAN_GRANULE_SIZE: c_ulong = MTE_GRANULE_SIZE;
pub const KASAN_GRANULE_MASK: c_ulong = KASAN_GRANULE_SIZE - 1;
pub const KASAN_MEMORY_PER_SHADOW_PAGE: c_ulong = KASAN_GRANULE_SIZE << PAGE_SHIFT;

#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_PAGE_FREE: u8 = 0xff;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_PAGE_REDZONE: u8 = 0xfe;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_SLAB_REDZONE: u8 = 0xfc;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_SLAB_FREE: u8 = 0xfb;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_VMALLOC_INVALID: u8 = 0xf8;
#[cfg(not(CONFIG_KASAN_GENERIC))]
pub const KASAN_PAGE_FREE: u8 = KASAN_TAG_INVALID;
#[cfg(not(CONFIG_KASAN_GENERIC))]
pub const KASAN_PAGE_REDZONE: u8 = KASAN_TAG_INVALID;
#[cfg(not(CONFIG_KASAN_GENERIC))]
pub const KASAN_SLAB_REDZONE: u8 = KASAN_TAG_INVALID;
#[cfg(not(CONFIG_KASAN_GENERIC))]
pub const KASAN_SLAB_FREE: u8 = KASAN_TAG_INVALID;
#[cfg(not(CONFIG_KASAN_GENERIC))]
pub const KASAN_VMALLOC_INVALID: u8 = KASAN_TAG_INVALID;

#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_SLAB_FREE_META: u8 = 0xfa;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_GLOBAL_REDZONE: u8 = 0xf9;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_STACK_LEFT: u8 = 0xf1;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_STACK_MID: u8 = 0xf2;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_STACK_RIGHT: u8 = 0xf3;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_STACK_PARTIAL: u8 = 0xf4;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_ALLOCA_LEFT: u8 = 0xca;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_ALLOCA_RIGHT: u8 = 0xcb;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_ALLOCA_REDZONE_SIZE: usize = 32;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_CURRENT_STACK_FRAME_MAGIC: u32 = 0x41b58ab3;
#[cfg(CONFIG_KASAN_GENERIC)]
pub const KASAN_ABI_VERSION: u32 = 1;

pub const META_BYTES_PER_BLOCK: usize = 1;
pub const META_BLOCKS_PER_ROW: usize = 16;
pub const META_BYTES_PER_ROW: usize = META_BLOCKS_PER_ROW * META_BYTES_PER_BLOCK;
pub const META_MEM_BYTES_PER_ROW: usize = META_BYTES_PER_ROW * KASAN_GRANULE_SIZE as usize;
pub const META_ROWS_AROUND_ADDR: usize = 2;
pub const KASAN_STACK_DEPTH: usize = 64;

#[repr(C)]
pub struct kasan_track { pub pid: u32, pub stack: depot_stack_handle_t,
    #[cfg(CONFIG_KASAN_EXTRA_INFO)] pub cpu: u64, #[cfg(CONFIG_KASAN_EXTRA_INFO)] pub timestamp: u64 }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kasan_report_type { KASAN_REPORT_ACCESS, KASAN_REPORT_INVALID_FREE, KASAN_REPORT_DOUBLE_FREE }

#[repr(C)]
pub struct kasan_report_info {
    pub type_: kasan_report_type, pub access_addr: *const c_void, pub access_size: usize,
    pub is_write: bool, pub ip: c_ulong, pub first_bad_addr: *const c_void,
    pub cache: *mut kmem_cache, pub object: *mut c_void, pub alloc_size: usize,
    pub bug_type: *const c_char, pub alloc_track: kasan_track, pub free_track: kasan_track,
}

#[repr(C)] pub struct kasan_source_location { pub filename: *const c_char, pub line_no: c_int, pub column_no: c_int }
#[repr(C)] pub struct kasan_global {
    pub beg: *const c_void, pub size: usize, pub size_with_redzone: usize, pub name: *const c_void,
    pub module_name: *const c_void, pub has_dynamic_init: c_ulong,
    #[cfg(any())] pub location: *mut kasan_source_location,
    #[cfg(any())] pub odr_indicator: *mut c_char,
}

#[cfg(CONFIG_KASAN_GENERIC)]
#[repr(C)] pub struct kasan_alloc_meta { pub alloc_track: kasan_track, pub aux_stack: [depot_stack_handle_t; 2] }
#[cfg(CONFIG_KASAN_GENERIC)]
#[repr(C)] pub struct qlist_node { pub next: *mut qlist_node }
#[cfg(CONFIG_KASAN_GENERIC)] pub const KASAN_NO_FREE_META: c_int = INT_MAX;
#[cfg(CONFIG_KASAN_GENERIC)]
#[repr(C)] pub struct kasan_free_meta { pub quarantine_link: qlist_node, pub free_track: kasan_track }

#[cfg(any(CONFIG_KASAN_SW_TAGS, CONFIG_KASAN_HW_TAGS))]
#[repr(C)] pub struct kasan_stack_ring_entry { pub ptr: *mut c_void, pub size: usize, pub track: kasan_track, pub is_free: bool }
#[cfg(any(CONFIG_KASAN_SW_TAGS, CONFIG_KASAN_HW_TAGS))]
#[repr(C)] pub struct kasan_stack_ring { pub lock: rwlock_t, pub size: usize, pub pos: atomic64_t, pub entries: *mut kasan_stack_ring_entry }

#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
#[inline(always)] pub unsafe fn addr_in_shadow(addr: *const c_void) -> bool {
    addr as usize >= KASAN_SHADOW_START && addr as usize < KASAN_SHADOW_END
}
#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
#[inline] pub unsafe fn kasan_shadow_to_mem(shadow_addr: *const c_void) -> *const c_void {
    (((shadow_addr as usize - KASAN_SHADOW_OFFSET) << KASAN_SHADOW_SCALE_SHIFT) as *const c_void)
}
#[cfg(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS))]
#[inline(always)] pub unsafe fn addr_has_metadata(addr: *const c_void) -> bool {
    kasan_reset_tag(addr) as usize >= kasan_shadow_to_mem(KASAN_SHADOW_START as *const c_void) as usize
}

extern "C" {
    pub fn kasan_check_range(addr: *const c_void, size: usize, write: bool, ret_ip: c_ulong) -> bool;
    pub fn kasan_find_first_bad_addr(addr: *const c_void, size: usize) -> *const c_void;
    pub fn kasan_get_alloc_size(object: *mut c_void, cache: *mut kmem_cache) -> usize;
    pub fn kasan_complete_mode_report_info(info: *mut kasan_report_info);
    pub fn kasan_metadata_fetch_row(buffer: *mut c_char, row: *mut c_void);
    pub fn kasan_report(addr: *const c_void, size: usize, is_write: bool, ip: c_ulong) -> bool;
    pub fn kasan_report_invalid_free(object: *mut c_void, ip: c_ulong, type_: kasan_report_type);
    pub fn kasan_addr_to_slab(addr: *const c_void) -> *mut slab;
    pub fn kasan_save_stack(flags: gfp_t, depot_flags: depot_flags_t) -> depot_stack_handle_t;
    pub fn kasan_set_track(track: *mut kasan_track, stack: depot_stack_handle_t);
    pub fn kasan_save_track(track: *mut kasan_track, flags: gfp_t);
    pub fn kasan_save_alloc_info(cache: *mut kmem_cache, object: *mut c_void, flags: gfp_t);
    pub fn kasan_save_free_info(cache: *mut kmem_cache, object: *mut c_void);
    pub fn kasan_poison(addr: *const c_void, size: usize, value: u8, init: bool);
    pub fn kasan_unpoison(addr: *const c_void, size: usize, init: bool);
    pub fn kasan_byte_accessible(addr: *const c_void) -> bool;
    pub fn kasan_poison_last_granule(address: *const c_void, size: usize);
}

#[cfg(CONFIG_KASAN_GENERIC)]
extern "C" { pub fn kasan_get_alloc_meta(cache: *mut kmem_cache, object: *const c_void) -> *mut kasan_alloc_meta; pub fn kasan_get_free_meta(cache: *mut kmem_cache, object: *const c_void) -> *mut kasan_free_meta; pub fn kasan_init_object_meta(cache: *mut kmem_cache, object: *const c_void); pub fn kasan_quarantine_put(cache: *mut kmem_cache, object: *mut c_void) -> bool; pub fn kasan_quarantine_reduce(); pub fn kasan_quarantine_remove_cache(cache: *mut kmem_cache); }

#[cfg(not(CONFIG_KASAN_GENERIC))]
#[inline] pub unsafe fn kasan_init_object_meta(_cache: *mut kmem_cache, _object: *const c_void) {}
#[cfg(not(CONFIG_KASAN_GENERIC))]
#[inline] pub unsafe fn kasan_quarantine_put(_cache: *mut kmem_cache, _object: *mut c_void) -> bool { false }
#[cfg(not(CONFIG_KASAN_GENERIC))]
#[inline] pub unsafe fn kasan_quarantine_reduce() {}
#[cfg(not(CONFIG_KASAN_GENERIC))]
#[inline] pub unsafe fn kasan_quarantine_remove_cache(_cache: *mut kmem_cache) {}

#[inline] pub unsafe fn arch_kasan_set_tag(addr: *const c_void, _tag: u8) -> *const c_void { addr }
#[inline] pub unsafe fn set_tag(addr: *const c_void, tag: u8) -> *mut c_void { arch_kasan_set_tag(addr, tag) as *mut c_void }
extern "C" {
    pub fn kasan_print_tags(addr_tag: u8, addr: *const c_void);
    pub fn kasan_print_address_stack_frame(addr: *const c_void);
    pub fn kasan_print_aux_stacks(cache: *mut kmem_cache, object: *const c_void);
    pub fn kasan_enable_hw_tags();
    pub fn kasan_init_tags();
    pub fn kasan_force_async_fault();
    pub fn kasan_write_only_enabled() -> bool;
    pub fn kasan_random_tag() -> u8;
    pub fn kasan_kunit_test_suite_start(); pub fn kasan_kunit_test_suite_end();
    pub fn kasan_save_enable_multi_shot() -> bool; pub fn kasan_restore_multi_shot(enabled: bool);
    pub fn __asan_register_globals(globals: *mut c_void, size: ssize_t); pub fn __asan_unregister_globals(globals: *mut c_void, size: ssize_t);
    pub fn __asan_handle_no_return(); pub fn __asan_alloca_poison(_: *mut c_void, size: ssize_t); pub fn __asan_allocas_unpoison(_: *mut c_void, _: ssize_t);
    pub fn __asan_load1(_: *mut c_void); pub fn __asan_store1(_: *mut c_void); pub fn __asan_load2(_: *mut c_void); pub fn __asan_store2(_: *mut c_void); pub fn __asan_load4(_: *mut c_void); pub fn __asan_store4(_: *mut c_void); pub fn __asan_load8(_: *mut c_void); pub fn __asan_store8(_: *mut c_void); pub fn __asan_load16(_: *mut c_void); pub fn __asan_store16(_: *mut c_void); pub fn __asan_loadN(_: *mut c_void, _: ssize_t); pub fn __asan_storeN(_: *mut c_void, _: ssize_t);
    pub fn __asan_load1_noabort(_: *mut c_void); pub fn __asan_store1_noabort(_: *mut c_void); pub fn __asan_load2_noabort(_: *mut c_void); pub fn __asan_store2_noabort(_: *mut c_void); pub fn __asan_load4_noabort(_: *mut c_void); pub fn __asan_store4_noabort(_: *mut c_void); pub fn __asan_load8_noabort(_: *mut c_void); pub fn __asan_store8_noabort(_: *mut c_void); pub fn __asan_load16_noabort(_: *mut c_void); pub fn __asan_store16_noabort(_: *mut c_void); pub fn __asan_loadN_noabort(_: *mut c_void, _: ssize_t); pub fn __asan_storeN_noabort(_: *mut c_void, _: ssize_t);
    pub fn __asan_report_load1_noabort(_: *mut c_void); pub fn __asan_report_store1_noabort(_: *mut c_void); pub fn __asan_report_load2_noabort(_: *mut c_void); pub fn __asan_report_store2_noabort(_: *mut c_void); pub fn __asan_report_load4_noabort(_: *mut c_void); pub fn __asan_report_store4_noabort(_: *mut c_void); pub fn __asan_report_load8_noabort(_: *mut c_void); pub fn __asan_report_store8_noabort(_: *mut c_void); pub fn __asan_report_load16_noabort(_: *mut c_void); pub fn __asan_report_store16_noabort(_: *mut c_void); pub fn __asan_report_load_n_noabort(_: *mut c_void, _: ssize_t); pub fn __asan_report_store_n_noabort(_: *mut c_void, _: ssize_t);
    pub fn __asan_set_shadow_00(_: *const c_void, _: ssize_t); pub fn __asan_set_shadow_f1(_: *const c_void, _: ssize_t); pub fn __asan_set_shadow_f2(_: *const c_void, _: ssize_t); pub fn __asan_set_shadow_f3(_: *const c_void, _: ssize_t); pub fn __asan_set_shadow_f5(_: *const c_void, _: ssize_t); pub fn __asan_set_shadow_f8(_: *const c_void, _: ssize_t);
    pub fn __asan_memset(_: *mut c_void, _: c_int, _: ssize_t) -> *mut c_void; pub fn __asan_memmove(_: *mut c_void, _: *const c_void, _: ssize_t) -> *mut c_void; pub fn __asan_memcpy(_: *mut c_void, _: *const c_void, _: ssize_t) -> *mut c_void;
    pub fn __hwasan_load1_noabort(_: *mut c_void); pub fn __hwasan_store1_noabort(_: *mut c_void); pub fn __hwasan_load2_noabort(_: *mut c_void); pub fn __hwasan_store2_noabort(_: *mut c_void); pub fn __hwasan_load4_noabort(_: *mut c_void); pub fn __hwasan_store4_noabort(_: *mut c_void); pub fn __hwasan_load8_noabort(_: *mut c_void); pub fn __hwasan_store8_noabort(_: *mut c_void); pub fn __hwasan_load16_noabort(_: *mut c_void); pub fn __hwasan_store16_noabort(_: *mut c_void); pub fn __hwasan_loadN_noabort(_: *mut c_void, _: ssize_t); pub fn __hwasan_storeN_noabort(_: *mut c_void, _: ssize_t);
    pub fn __hwasan_tag_memory(_: *mut c_void, _: u8, _: ssize_t); pub fn __hwasan_memset(_: *mut c_void, _: c_int, _: ssize_t) -> *mut c_void; pub fn __hwasan_memmove(_: *mut c_void, _: *const c_void, _: ssize_t) -> *mut c_void; pub fn __hwasan_memcpy(_: *mut c_void, _: *const c_void, _: ssize_t) -> *mut c_void;
    pub fn kasan_tag_mismatch(addr: *mut c_void, access_info: c_ulong, ret_ip: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
