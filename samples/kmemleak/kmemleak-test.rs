// SPDX-License-Identifier: GPL-2.0-only
/*
 * samples/kmemleak/kmemleak-test.c
 *
 * Copyright (C) 2008 ARM Limited
 * Written by Catalin Marinas <catalin.marinas@arm.com>
 */

// C dependencies supplied by the kernel build are intentionally left external.

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct test_node {
    pub header: [c_long; 25],
    pub list: list_head,
    pub footer: [c_long; 25],
}

type c_long = isize;
type c_int = i32;

extern "C" {
    static mut files_cachep: *mut core::ffi::c_void;

    fn kmalloc(size: usize, flags: usize) -> *mut core::ffi::c_void;
    fn kzalloc(size: usize, flags: usize) -> *mut core::ffi::c_void;
    fn vmalloc(size: usize) -> *mut core::ffi::c_void;
    fn __alloc_percpu(size: usize, align: usize) -> *mut core::ffi::c_void;
    fn kmem_cache_alloc(cache: *mut core::ffi::c_void, flags: usize)
        -> *mut core::ffi::c_void;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn pr_info(fmt: *const u8, ...);
    fn possible_cpu_count() -> c_int;
}

static mut test_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut kmemleak_test_pointer: *mut core::ffi::c_void = core::ptr::null_mut();

const GFP_KERNEL: usize = 0;

// This is a per-CPU variable in the C implementation.
// The CONFIG_MODULES conditional is preserved below as a build-time dependency.

unsafe fn kmemleak_test_init() -> c_int {
    let mut elem: *mut test_node;
    let mut i: c_int;

    pr_info(b"Kmemleak testing\0".as_ptr());

    // make some orphan objects
    pr_info(b"kmalloc(32) = 0x%px\n\0".as_ptr(), kmalloc(32, GFP_KERNEL));
    pr_info(b"kmalloc(32) = 0x%px\n\0".as_ptr(), kmalloc(32, GFP_KERNEL));
    pr_info(b"kmalloc(1024) = 0x%px\n\0".as_ptr(), kmalloc(1024, GFP_KERNEL));
    pr_info(b"kmalloc(1024) = 0x%px\n\0".as_ptr(), kmalloc(1024, GFP_KERNEL));
    pr_info(b"kmalloc(2048) = 0x%px\n\0".as_ptr(), kmalloc(2048, GFP_KERNEL));
    pr_info(b"kmalloc(2048) = 0x%px\n\0".as_ptr(), kmalloc(2048, GFP_KERNEL));
    pr_info(b"kmalloc(4096) = 0x%px\n\0".as_ptr(), kmalloc(4096, GFP_KERNEL));
    pr_info(b"kmalloc(4096) = 0x%px\n\0".as_ptr(), kmalloc(4096, GFP_KERNEL));

    // #ifndef CONFIG_MODULES
    pr_info(b"kmem_cache_alloc(files_cachep) = 0x%px\n\0".as_ptr(), kmem_cache_alloc(files_cachep, GFP_KERNEL));
    pr_info(b"kmem_cache_alloc(files_cachep) = 0x%px\n\0".as_ptr(), kmem_cache_alloc(files_cachep, GFP_KERNEL));
    // #endif

    pr_info(b"vmalloc(64) = 0x%px\n\0".as_ptr(), vmalloc(64));
    pr_info(b"vmalloc(64) = 0x%px\n\0".as_ptr(), vmalloc(64));
    pr_info(b"vmalloc(64) = 0x%px\n\0".as_ptr(), vmalloc(64));
    pr_info(b"vmalloc(64) = 0x%px\n\0".as_ptr(), vmalloc(64));
    pr_info(b"vmalloc(64) = 0x%px\n\0".as_ptr(), vmalloc(64));

    // Add elements to a list. They should only appear as orphan after the module is removed.
    i = 0;
    while i < 10 {
        elem = kzalloc(core::mem::size_of::<test_node>(), GFP_KERNEL) as *mut test_node;
        pr_info(b"kzalloc(sizeof(*elem)) = 0x%px\n\0".as_ptr(), elem);
        if elem.is_null() {
            return -12;
        }
        (*elem).list.next = &mut (*elem).list;
        (*elem).list.prev = &mut (*elem).list;
        list_add_tail(&mut (*elem).list, &mut test_list);
        i += 1;
    }

    // for_each_possible_cpu(i): per-CPU allocation and logging.
    i = 0;
    while i < possible_cpu_count() {
        kmemleak_test_pointer = kmalloc(129, GFP_KERNEL);
        pr_info(b"kmalloc(129) = 0x%px\n\0".as_ptr(), kmemleak_test_pointer);
        i += 1;
    }

    pr_info(b"__alloc_percpu(64, 4) = 0x%px\n\0".as_ptr(), __alloc_percpu(64, 4));
    0
}

unsafe fn kmemleak_test_exit() {
    let mut pos: *mut list_head = test_list.next;
    let head: *mut list_head = &mut test_list;
    // Remove the list elements without actually freeing the memory.
    while pos != head {
        let next = (*pos).next;
        list_del(pos);
        pos = next;
    }
}

// module_init(kmemleak_test_init);
// module_exit(kmemleak_test_exit);
// MODULE_DESCRIPTION("Sample module to leak memory for kmemleak testing");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
