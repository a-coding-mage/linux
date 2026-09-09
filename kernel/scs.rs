// SPDX-License-Identifier: GPL-2.0
/*
 * Shadow Call Stack support.
 *
 * Copyright (C) 2019 Google LLC
 */

use core::ffi::c_void;

// Kernel declarations and macros supplied by the surrounding tree.
extern "C" {
    fn vmalloc_to_page(addr: *mut c_void) -> *mut page;
    fn page_pgdat(page: *mut page) -> *mut c_void;
    fn mod_node_page_state(pgdat: *mut c_void, item: i32, delta: i64);
    fn kasan_unpoison_vmalloc(addr: *mut c_void, size: usize, prot: i32) -> *mut c_void;
    fn kasan_reset_tag(addr: *mut c_void) -> *mut c_void;
    fn kasan_poison_vmalloc(addr: *mut c_void, size: usize);
    fn __vmalloc_node_range(
        size: usize,
        align: usize,
        start: usize,
        end: usize,
        gfp_mask: u32,
        prot: usize,
        vm_flags: usize,
        node: i32,
        caller: *mut c_void,
    ) -> *mut c_void;
    fn vfree_atomic(addr: *mut c_void);
    fn vfree(addr: *mut c_void);
    fn cpuhp_setup_state(
        state: i32,
        name: *const u8,
        startup: *mut c_void,
        teardown: Option<unsafe extern "C" fn(cpu: u32) -> i32>,
    ) -> i32;
    fn scs_is_enabled() -> bool;
    fn task_scs(tsk: *mut task_struct) -> *mut c_void;
    fn task_scs_sp(tsk: *mut task_struct) -> *mut c_void;
    fn task_scs_end_corrupted(tsk: *mut task_struct) -> bool;
    fn task_pid_nr(tsk: *mut task_struct) -> i32;
    fn pr_info(fmt: *const u8, ...);
    fn warn(condition: bool, fmt: *const u8, ...);
    fn cmpxchg_relaxed(ptr: *mut usize, old: usize, new: usize) -> usize;
    fn __scs_magic(s: *mut c_void) -> *mut usize;
    fn this_cpu_xchg(cache: *mut *mut c_void, value: *mut c_void) -> *mut c_void;
    fn this_cpu_cmpxchg(
        cache: *mut *mut c_void,
        old: *mut c_void,
        new: *mut c_void,
    ) -> *mut c_void;
    fn per_cpu_ptr(cache: *mut *mut c_void, cpu: u32) -> *mut *mut c_void;
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[cfg(feature = "config_dynamic_scs")]
#[no_mangle]
pub static mut dynamic_scs_enabled: bool = false;

const NR_CACHED_SCS: usize = 2;

// DEFINE_PER_CPU(void *, scs_cache[NR_CACHED_SCS]);
#[no_mangle]
pub static mut scs_cache: [*mut c_void; NR_CACHED_SCS] = [core::ptr::null_mut(); NR_CACHED_SCS];

unsafe fn __scs_account(s: *mut c_void, account: i32) {
    let scs_page = vmalloc_to_page(s);
    mod_node_page_state(page_pgdat(scs_page), 0 /* NR_KERNEL_SCS_KB */, (account as i64) * 16);
}

unsafe fn __scs_alloc(node: i32) -> *mut c_void {
    let mut i = 0;
    let mut s: *mut c_void;

    while i < NR_CACHED_SCS {
        s = this_cpu_xchg(&mut scs_cache[i], core::ptr::null_mut());
        if !s.is_null() {
            s = kasan_unpoison_vmalloc(s, 0 /* SCS_SIZE */, 0 /* KASAN_VMALLOC_PROT_NORMAL */);
            core::ptr::write_bytes(s, 0, 0 /* SCS_SIZE */);
            return kasan_reset_tag(s);
        }
        i += 1;
    }

    s = __vmalloc_node_range(
        0 /* SCS_SIZE */,
        1,
        0 /* VMALLOC_START */,
        0 /* VMALLOC_END */,
        0 /* GFP_SCS */,
        0 /* PAGE_KERNEL */,
        0,
        node,
        __builtin_return_address(0),
    );

    kasan_reset_tag(s)
}

#[no_mangle]
pub unsafe extern "C" fn scs_alloc(node: i32) -> *mut c_void {
    let s = __scs_alloc(node);
    if s.is_null() {
        return core::ptr::null_mut();
    }

    *__scs_magic(s) = 0 /* SCS_END_MAGIC */;
    /*
     * Poison the allocation to catch unintentional accesses to
     * the shadow stack when KASAN is enabled.
     */
    kasan_poison_vmalloc(s, 0 /* SCS_SIZE */);
    __scs_account(s, 1);
    s
}

#[no_mangle]
pub unsafe extern "C" fn scs_free(s: *mut c_void) {
    __scs_account(s, -1);

    /*
     * We cannot sleep as this can be called in interrupt context,
     * so use this_cpu_cmpxchg to update the cache, and vfree_atomic
     * to free the stack.
     */
    let mut i = 0;
    while i < NR_CACHED_SCS {
        if this_cpu_cmpxchg(&mut scs_cache[i], core::ptr::null_mut(), s).is_null() {
            return;
        }
        i += 1;
    }

    kasan_unpoison_vmalloc(s, 0 /* SCS_SIZE */, 0 /* KASAN_VMALLOC_PROT_NORMAL */);
    vfree_atomic(s);
}

unsafe extern "C" fn scs_cleanup(cpu: u32) -> i32 {
    let cache = per_cpu_ptr(scs_cache.as_mut_ptr(), cpu);
    let mut i = 0;
    while i < NR_CACHED_SCS {
        vfree(*cache.add(i));
        *cache.add(i) = core::ptr::null_mut();
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn scs_init() {
    if !scs_is_enabled() {
        return;
    }
    cpuhp_setup_state(0 /* CPUHP_BP_PREPARE_DYN */, b"scs:scs_cache\0".as_ptr(), core::ptr::null_mut(), Some(scs_cleanup));
}

#[no_mangle]
pub unsafe extern "C" fn scs_prepare(tsk: *mut task_struct, node: i32) -> i32 {
    if !scs_is_enabled() {
        return 0;
    }
    let s = scs_alloc(node);
    if s.is_null() {
        return -12 /* -ENOMEM */;
    }
    // task_scs(tsk) = task_scs_sp(tsk) = s;
    let _ = task_scs_sp(tsk);
    let _ = task_scs(tsk);
    0
}

unsafe fn scs_check_usage(tsk: *mut task_struct) {
    static mut highest: usize = 0;
    let mut p: *mut usize;
    let mut prev: usize;
    let mut curr = highest;
    let mut used: usize = 0;

    // if (!IS_ENABLED(CONFIG_DEBUG_STACK_USAGE)) return;
    p = task_scs(tsk) as *mut usize;
    while p < __scs_magic(task_scs(tsk)) {
        if core::ptr::read_volatile(p) == 0 {
            break;
        }
        used += core::mem::size_of::<usize>();
        p = p.add(1);
    }

    while used > curr {
        prev = cmpxchg_relaxed(&mut highest, curr, used);
        if prev == curr {
            pr_info(b"%s (%d): highest shadow stack usage: %lu bytes\n\0".as_ptr(), task_pid_nr(tsk), used);
            break;
        }
        curr = prev;
    }
}

#[no_mangle]
pub unsafe extern "C" fn scs_release(tsk: *mut task_struct) {
    let s = task_scs(tsk);
    if !scs_is_enabled() || s.is_null() {
        return;
    }
    warn(task_scs_end_corrupted(tsk), b"corrupted shadow stack detected when freeing task\n\0".as_ptr());
    scs_check_usage(tsk);
    scs_free(s);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
