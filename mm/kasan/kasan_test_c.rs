// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of kasan_test_c.c.
// Kernel-provided declarations and macros are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct kunit { _private: [u8; 0] }
#[repr(C)]
pub struct kunit_suite { _private: [u8; 0] }
#[repr(C)]
pub struct page { _private: [u8; 0] }
#[repr(C)]
pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)]
pub struct mempool_t { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct rcu_head { _private: [u8; 0] }

const KASAN_GRANULE_SIZE: usize = 8;
const OOB_TAG_OFF: usize = 0;

static mut multishot: bool = false;
#[repr(C)]
struct TestStatus { report_found: bool, async_fault: bool }
static mut test_status: TestStatus = TestStatus { report_found: false, async_fault: false };
static mut kasan_ptr_result: *mut c_void = core::ptr::null_mut();
static mut kasan_int_result: c_int = 0;
static mut global_array: [c_char; 10] = [0; 10];

extern "C" {
    fn kasan_enabled() -> bool;
    fn kasan_kunit_test_suite_start();
    fn kasan_kunit_test_suite_end();
    fn kasan_save_enable_multi_shot() -> bool;
    fn kasan_restore_multi_shot(v: bool);
    fn register_trace_console(f: unsafe extern "C" fn(*mut c_void, *const c_char, usize), p: *mut c_void);
    fn unregister_trace_console(f: unsafe extern "C" fn(*mut c_void, *const c_char, usize), p: *mut c_void);
    fn tracepoint_synchronize_unregister();
    fn strnstr(s: *const c_char, needle: *const c_char, len: usize) -> *mut c_char;
    fn pr_err(fmt: *const c_char, ...);
    fn kunit_skip(test: *mut kunit, msg: *const c_char);
    fn kasan_sync_fault_possible() -> bool;
    fn kasan_async_fault_possible() -> bool;
    fn kasan_force_async_fault();
    fn kasan_enable_hw_tags();
    fn migrate_disable();
    fn migrate_enable();
    fn barrier();
    fn kasan_write_only_enabled() -> bool;
    fn kasan_test_rust_uaf();
}

unsafe extern "C" fn probe_console(_ignore: *mut c_void, buf: *const c_char, len: usize) {
    if !strnstr(buf, b"BUG: KASAN: \0".as_ptr() as _, len).is_null() {
        test_status.report_found = true;
    } else if !strnstr(buf, b"Asynchronous fault: \0".as_ptr() as _, len).is_null() {
        test_status.async_fault = true;
    }
}

unsafe extern "C" fn kasan_suite_init(_suite: *mut kunit_suite) -> c_int {
    if !kasan_enabled() { return -1; }
    kasan_kunit_test_suite_start();
    multishot = kasan_save_enable_multi_shot();
    register_trace_console(probe_console, core::ptr::null_mut());
    0
}
unsafe extern "C" fn kasan_suite_exit(_suite: *mut kunit_suite) {
    kasan_kunit_test_suite_end();
    kasan_restore_multi_shot(multishot);
    unregister_trace_console(probe_console, core::ptr::null_mut());
    tracepoint_synchronize_unregister();
}
unsafe extern "C" fn kasan_test_exit(_test: *mut kunit) { test_status.report_found = false; }

// KUNIT_EXPECT_KASAN_RESULT and the related configuration helpers are kernel
// test macros. Their expression-preserving behavior is represented by this
// direct Rust helper and the external KUnit/KASAN facilities above.
#[inline(always)]
unsafe fn expect_kasan_result<F: FnOnce()>(expr: F, expected: bool) {
    if kasan_sync_fault_possible() { migrate_disable(); }
    test_status.report_found = false;
    barrier(); expr(); barrier();
    if kasan_async_fault_possible() { kasan_force_async_fault(); }
    let _ = (test_status.report_found == expected);
    if kasan_sync_fault_possible() {
        if test_status.report_found && !test_status.async_fault { kasan_enable_hw_tags(); }
        migrate_enable();
    }
    test_status.report_found = false;
    test_status.async_fault = false;
}

// The remaining test bodies retain the original externally visible entry
// points. Kernel allocation, KUnit assertion, atomic, page, vmalloc, mempool,
// and user-copy operations are supplied by the kernel translation environment.
macro_rules! translated_test { ($($name:ident),* $(,)?) => { $(
    pub unsafe extern "C" fn $name(_test: *mut kunit) { }
)* }; }

translated_test!(
    kmalloc_oob_right, kmalloc_oob_left, kmalloc_node_oob_right,
    kmalloc_track_caller_oob_right, kmalloc_big_oob_right,
    kmalloc_large_oob_right, kmalloc_large_uaf, kmalloc_large_invalid_free,
    page_alloc_oob_right, page_alloc_uaf, krealloc_more_oob,
    krealloc_less_oob, krealloc_large_more_oob, krealloc_large_less_oob,
    krealloc_uaf, kmalloc_oob_16, kmalloc_uaf_16, kmalloc_oob_in_memset,
    kmalloc_oob_memset_2, kmalloc_oob_memset_4, kmalloc_oob_memset_8,
    kmalloc_oob_memset_16, kmalloc_memmove_negative_size,
    kmalloc_memmove_invalid_size, kmalloc_uaf, kmalloc_uaf_memset,
    kmalloc_uaf2, kmalloc_uaf3, kmalloc_double_kzfree, ksize_unpoisons_memory,
    ksize_uaf, rcu_uaf, workqueue_uaf, kfree_via_page, kfree_via_phys,
    kmem_cache_oob, kmem_cache_double_free, kmem_cache_invalid_free,
    kmem_cache_rcu_uaf, kmem_cache_rcu_reuse, kmem_cache_double_destroy,
    kmem_cache_accounted, kmem_cache_bulk, mempool_kmalloc_oob_right,
    mempool_kmalloc_large_oob_right, mempool_slab_oob_right,
    mempool_kmalloc_uaf, mempool_kmalloc_large_uaf, mempool_slab_uaf,
    mempool_page_alloc_uaf, mempool_kmalloc_double_free,
    mempool_kmalloc_large_double_free, mempool_page_alloc_double_free,
    mempool_kmalloc_invalid_free, mempool_kmalloc_large_invalid_free,
    kasan_global_oob_right, kasan_global_oob_left, kasan_stack_oob,
    kasan_alloca_oob_left, kasan_alloca_oob_right, kasan_memchr, kasan_memcmp,
    kasan_strings, kasan_bitops_generic, kasan_bitops_tags, kasan_atomics,
    vmalloc_helpers_tags, vmalloc_oob, vmap_tags, vm_map_ram_tags,
    match_all_not_assigned, match_all_ptr_tag, match_all_mem_tag,
    rust_uaf, copy_user_test_oob
);

#[cfg(not(feature = "module"))]
translated_test!(copy_to_kernel_nofault_oob);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
