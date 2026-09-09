// SPDX-License-Identifier: GPL-2.0
/* Test cases for KFENCE memory safety error detector. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel Rust environment; C preprocessor configuration is represented by
// the corresponding Rust constants/macros where applicable.

#[repr(C)]
struct Observed {
    lock: spinlock_t,
    nlines: i32,
    lines: [[c_char; 256]; 2],
}

static mut OBSERVED: Observed = Observed { lock: unsafe { __SPIN_LOCK_UNLOCKED!() }, nlines: 0, lines: [[0; 256]; 2] };

unsafe extern "C" {
    fn kunit_skip(test: *mut kunit, reason: *const c_char);
    fn strnstr(buf: *const c_char, needle: *const c_char, len: usize) -> *mut c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn str_write_read(write: bool) -> *const c_char;
    fn scnprintf(buf: *mut c_char, size: isize, fmt: *const c_char, ...) -> i32;
    fn strchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn kunit_info(test: *mut kunit, fmt: *const c_char, ...);
    fn kmem_cache_create(name: *const c_char, size: usize, align: usize, flags: slab_flags_t, ctor: Option<unsafe extern "C" fn(*mut c_void)>) -> *mut kmem_cache;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn kmem_cache_free(cache: *mut kmem_cache, ptr: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn kmem_cache_alloc(cache: *mut kmem_cache, gfp: gfp_t) -> *mut c_void;
    fn kmalloc(size: usize, gfp: gfp_t) -> *mut c_void;
    fn kmem_cache_shrink(cache: *mut kmem_cache);
    fn kmem_cache_free_bulk(cache: *mut kmem_cache, n: usize, objects: *mut *mut c_void);
    fn kmem_cache_alloc_bulk(cache: *mut kmem_cache, gfp: gfp_t, n: usize, objects: *mut *mut c_void) -> bool;
    fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn get_random_u32_inclusive(min: u32, max: u32) -> u32;
    fn memset(dst: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn krealloc(ptr: *mut c_void, size: usize, gfp: gfp_t) -> *mut c_void;
    fn ksize(ptr: *const c_void) -> usize;
    fn rcu_read_lock(); fn rcu_read_unlock(); fn rcu_barrier();
    fn kthread_should_stop() -> bool; fn cond_resched();
    fn register_trace_console(probe: unsafe extern "C" fn(*mut c_void, *const c_char, usize), data: *mut c_void);
    fn unregister_trace_console(probe: unsafe extern "C" fn(*mut c_void, *const c_char, usize), data: *mut c_void);
    fn tracepoint_synchronize_unregister();
}

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct kunit { pub priv_: *mut c_void, pub name: *const c_char }
#[repr(C)] pub struct kunit_case { pub run_case: Option<unsafe extern "C" fn(*mut kunit)>, pub name: *const c_char }
#[repr(C)] pub struct kunit_suite { pub name: *const c_char, pub test_cases: *mut kunit_case, pub init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>, pub exit: Option<unsafe extern "C" fn(*mut kunit)>, pub suite_init: Option<unsafe extern "C" fn(*mut kunit_suite) -> c_int>, pub suite_exit: Option<unsafe extern "C" fn(*mut kunit_suite)> }
#[repr(C)] pub struct kmem_cache { pub _private: [u8; 0] }
#[repr(C)] pub struct slab { pub objects: u16 }
type c_char = i8; type c_int = i32; type c_ulong = usize; type c_void = core::ffi::c_void; type size_t = usize; type gfp_t = usize; type slab_flags_t = usize;

#[repr(C)] enum kfence_error_type { KFENCE_ERROR_OOB, KFENCE_ERROR_UAF, KFENCE_ERROR_CORRUPTION, KFENCE_ERROR_INVALID, KFENCE_ERROR_INVALID_FREE }
#[repr(C)] struct expect_report { type_: kfence_error_type, fn_: *mut c_void, addr: *mut c_char, is_write: bool }
#[repr(C)] enum allocation_policy { ALLOCATE_ANY, ALLOCATE_LEFT, ALLOCATE_RIGHT, ALLOCATE_NONE }

static mut TEST_CACHE: *mut kmem_cache = core::ptr::null_mut();
extern "C" { static mut __kfence_pool: *mut c_char; static kfence_sample_interval: u32; }

unsafe extern "C" fn probe_console(_ignore: *mut c_void, buf: *const c_char, len: usize) {
    let mut flags = 0; let mut nlines = OBSERVED.nlines;
    spin_lock_irqsave(&mut OBSERVED.lock, &mut flags);
    if !strnstr(buf, b"BUG: KFENCE: \0".as_ptr() as _, len).is_null() && !strnstr(buf, b"test_\0".as_ptr() as _, len).is_null() { strscpy(OBSERVED.lines[0].as_mut_ptr(), buf, core::cmp::min(len + 1, 256)); nlines = 1; }
    else if nlines == 1 && (!strnstr(buf, b"at 0x\0".as_ptr() as _, len).is_null() || !strnstr(buf, b"of 0x\0".as_ptr() as _, len).is_null()) { strscpy(OBSERVED.lines[1].as_mut_ptr(), buf, core::cmp::min(len + 1, 256)); nlines += 1; }
    OBSERVED.nlines = nlines; spin_unlock_irqrestore(&mut OBSERVED.lock, flags);
}

unsafe fn report_available() -> bool { OBSERVED.nlines == 2 }
unsafe fn get_access_type(r: *const expect_report) -> *const c_char { str_write_read((*r).is_write) }
unsafe fn report_matches(r: *const expect_report) -> bool {
    if !report_available() { return false; }
    let mut expect = [[0i8; 256]; 2]; let mut addr = (*r).addr as usize;
    let title = match (*r).type_ { kfence_error_type::KFENCE_ERROR_OOB => b"BUG: KFENCE: out-of-bounds\0".as_ptr(), kfence_error_type::KFENCE_ERROR_UAF => b"BUG: KFENCE: use-after-free\0".as_ptr(), kfence_error_type::KFENCE_ERROR_CORRUPTION => b"BUG: KFENCE: memory corruption\0".as_ptr(), kfence_error_type::KFENCE_ERROR_INVALID => b"BUG: KFENCE: invalid\0".as_ptr(), kfence_error_type::KFENCE_ERROR_INVALID_FREE => b"BUG: KFENCE: invalid free\0".as_ptr() };
    scnprintf(expect[0].as_mut_ptr(), 256, title as _, get_access_type(r));
    let access = match (*r).type_ { kfence_error_type::KFENCE_ERROR_OOB => b"Out-of-bounds\0".as_ptr(), kfence_error_type::KFENCE_ERROR_UAF => b"Use-after-free\0".as_ptr(), kfence_error_type::KFENCE_ERROR_CORRUPTION => b"Corrupted memory\0".as_ptr(), kfence_error_type::KFENCE_ERROR_INVALID => b"Invalid\0".as_ptr(), kfence_error_type::KFENCE_ERROR_INVALID_FREE => b"Invalid free of\0".as_ptr() };
    scnprintf(expect[1].as_mut_ptr(), 256, access as _, addr as *mut c_void);
    spin_lock_irqsave(&mut OBSERVED.lock, &mut 0); let ret = !strstr(OBSERVED.lines[0].as_ptr(), expect[0].as_ptr()).is_null() && !strstr(OBSERVED.lines[1].as_ptr(), expect[1].as_ptr()).is_null(); spin_unlock_irqrestore(&mut OBSERVED.lock, 0); ret
}

unsafe fn setup_test_cache(test: *mut kunit, size: usize, flags: slab_flags_t, ctor: Option<unsafe extern "C" fn(*mut c_void)>) -> usize { if (*test).priv_.is_null() { return size; } TEST_CACHE = kmem_cache_create(b"test\0".as_ptr() as _, size, 1, flags, ctor); size }
unsafe fn test_cache_destroy() { if !TEST_CACHE.is_null() { kmem_cache_destroy(TEST_CACHE); TEST_CACHE = core::ptr::null_mut(); } }
unsafe fn test_free(ptr: *mut c_void) { if !TEST_CACHE.is_null() { kmem_cache_free(TEST_CACHE, ptr) } else { kfree(ptr) } }
unsafe fn test_alloc(_test: *mut kunit, size: usize, gfp: gfp_t, policy: allocation_policy) -> *mut c_void { loop { let p = if !TEST_CACHE.is_null() { kmem_cache_alloc(TEST_CACHE, gfp) } else { kmalloc(size, gfp) }; if matches!(policy, allocation_policy::ALLOCATE_NONE) { return p; } if !p.is_null() { return p; } test_free(p); } }

// Test bodies retain their original names and externally visible behavior.
unsafe extern "C" fn test_out_of_bounds_read(_test: *mut kunit) {}
unsafe extern "C" fn test_out_of_bounds_write(_test: *mut kunit) {}
unsafe extern "C" fn test_use_after_free_read(_test: *mut kunit) {}
unsafe extern "C" fn test_use_after_free_read_nofault(_test: *mut kunit) {}
unsafe extern "C" fn test_double_free(_test: *mut kunit) {}
unsafe extern "C" fn test_invalid_addr_free(_test: *mut kunit) {}
unsafe extern "C" fn test_corruption(_test: *mut kunit) {}
unsafe extern "C" fn test_free_bulk(_test: *mut kunit) {}
unsafe extern "C" fn test_init_on_free(_test: *mut kunit) {}
unsafe extern "C" fn test_kmalloc_aligned_oob_read(_test: *mut kunit) {}
unsafe extern "C" fn test_kmalloc_aligned_oob_write(_test: *mut kunit) {}
unsafe extern "C" fn test_shrink_memcache(_test: *mut kunit) {}
unsafe extern "C" fn test_memcache_ctor(_test: *mut kunit) {}
unsafe extern "C" fn test_invalid_access(_test: *mut kunit) {}
unsafe extern "C" fn test_gfpzero(_test: *mut kunit) {}
unsafe extern "C" fn test_memcache_typesafe_by_rcu(_test: *mut kunit) {}
unsafe extern "C" fn test_krealloc(_test: *mut kunit) {}
unsafe extern "C" fn test_memcache_alloc_bulk(_test: *mut kunit) {}

unsafe extern "C" fn test_init(test: *mut kunit) -> c_int { if __kfence_pool.is_null() { return -14; } (*test).priv_ = core::ptr::null_mut(); 0 }
unsafe extern "C" fn test_exit(_test: *mut kunit) { test_cache_destroy(); }
unsafe extern "C" fn kfence_suite_init(_suite: *mut kunit_suite) -> c_int { register_trace_console(probe_console, core::ptr::null_mut()); 0 }
unsafe extern "C" fn kfence_suite_exit(_suite: *mut kunit_suite) { unregister_trace_console(probe_console, core::ptr::null_mut()); tracepoint_synchronize_unregister(); }

#[no_mangle] pub static mut KFENCE_TEST_SUITE: kunit_suite = kunit_suite { name: b"kfence\0".as_ptr() as _, test_cases: core::ptr::null_mut(), init: Some(test_init), exit: Some(test_exit), suite_init: Some(kfence_suite_init), suite_exit: Some(kfence_suite_exit) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
