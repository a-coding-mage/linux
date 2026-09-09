// SPDX-License-Identifier: GPL-2.0

/*
 * Test module for stress and analyze performance of vmalloc allocator.
 * (C) 2018 Uladzislau Rezki (Sony) <urezki@gmail.com>
 */
// Linux kernel headers and module-parameter declarations are supplied by the
// surrounding kernel translation unit.

// __param(type, name, init, msg)
// static type name = init; module_param(name, type, 0444);
// MODULE_PARM_DESC(name, msg)
static mut nr_threads: i32 = 0;
static mut sequential_test_order: bool = false;
static mut test_repeat_count: i32 = 1;
static mut test_loop_count: i32 = 1_000_000;
static mut nr_pages: i32 = 0;
static mut use_huge: bool = false;
static mut run_test_mask: i32 = 7;
static mut nr_pcpu_objects: i32 = 35000;

/* This is for synchronization of setup phase. */
static mut prepare_for_test_srcu: srcu_struct = unsafe { core::mem::zeroed() };

/* Completion tracking for worker threads. */
static mut test_all_done_comp: completion = unsafe { core::mem::zeroed() };
static mut test_n_undone: atomic_t = unsafe { core::mem::zeroed() };

#[inline]
unsafe fn test_report_one_done() {
    if atomic_dec_and_test(&mut test_n_undone) {
        complete(&mut test_all_done_comp);
    }
}

unsafe fn random_size_align_alloc_test() -> i32 {
    for _i in 0..test_loop_count {
        let rnd: u32 = get_random_u8() as u32;
        let align: usize = 1usize << (rnd % 23);
        let size: usize = ((rnd % 10 + 1) as usize) * PAGE_SIZE;
        let ptr = __vmalloc_node(size, align, GFP_KERNEL | __GFP_ZERO, 0,
                                 __builtin_return_address(0));
        if ptr.is_null() { return -1; }
        vfree(ptr);
    }
    0
}

unsafe fn align_shift_alloc_test() -> i32 {
    for i in 0..BITS_PER_LONG {
        let align = 1usize << i;
        let ptr = __vmalloc_node(PAGE_SIZE, align, GFP_KERNEL | __GFP_ZERO, 0,
                                 __builtin_return_address(0));
        if ptr.is_null() { return -1; }
        vfree(ptr);
    }
    0
}

unsafe fn fix_align_alloc_test() -> i32 {
    for _i in 0..test_loop_count {
        let ptr = __vmalloc_node(5 * PAGE_SIZE, THREAD_ALIGN << 1,
                                 GFP_KERNEL | __GFP_ZERO, 0,
                                 __builtin_return_address(0));
        if ptr.is_null() { return -1; }
        vfree(ptr);
    }
    0
}

unsafe fn random_size_alloc_test() -> i32 {
    for _i in 0..test_loop_count {
        let n = get_random_u32_inclusive(1, 100);
        let p = vmalloc((n as usize) * PAGE_SIZE);
        if p.is_null() { return -1; }
        *(p as *mut u8) = 1;
        vfree(p);
    }
    0
}

unsafe fn long_busy_list_alloc_test() -> i32 {
    let mut rv = -1;
    let ptr = vmalloc(core::mem::size_of::<*mut core::ffi::c_void>() * 15000) as *mut *mut core::ffi::c_void;
    if ptr.is_null() { return rv; }
    for i in 0..15000 { *ptr.add(i) = vmalloc(PAGE_SIZE); }
    for _i in 0..test_loop_count {
        let ptr_1 = vmalloc(100 * PAGE_SIZE);
        if ptr_1.is_null() { break; }
        let ptr_2 = vmalloc(PAGE_SIZE);
        if ptr_2.is_null() { vfree(ptr_1); break; }
        *(ptr_1 as *mut u8) = 0;
        *(ptr_2 as *mut u8) = 1;
        vfree(ptr_1); vfree(ptr_2);
    }
    if !ptr.is_null() { rv = 0; }
    for i in 0..15000 { vfree(*ptr.add(i)); }
    vfree(ptr as *mut core::ffi::c_void);
    rv
}

unsafe fn full_fit_alloc_test() -> i32 {
    let junk_length = fls(num_online_cpus()) * (32 * 1024 * 1024 / PAGE_SIZE as i32);
    let ptr = vmalloc(core::mem::size_of::<*mut core::ffi::c_void>() * junk_length as usize) as *mut *mut core::ffi::c_void;
    if ptr.is_null() { return -1; }
    let junk_ptr = vmalloc(core::mem::size_of::<*mut core::ffi::c_void>() * junk_length as usize) as *mut *mut core::ffi::c_void;
    if junk_ptr.is_null() { vfree(ptr as *mut _); return -1; }
    for i in 0..junk_length as usize { *ptr.add(i) = vmalloc(PAGE_SIZE); *junk_ptr.add(i) = vmalloc(PAGE_SIZE); }
    for i in 0..junk_length as usize { vfree(*junk_ptr.add(i)); }
    let mut rv = 0;
    for _i in 0..test_loop_count {
        let tmp = vmalloc(PAGE_SIZE);
        if tmp.is_null() { rv = -1; break; }
        *(tmp as *mut u8) = 1; vfree(tmp);
    }
    for i in 0..junk_length as usize { vfree(*ptr.add(i)); }
    vfree(ptr as *mut _); vfree(junk_ptr as *mut _); rv
}

unsafe fn fix_size_alloc_test() -> i32 {
    for _i in 0..test_loop_count {
        let size = ((if nr_pages > 0 { nr_pages } else { 1 }) as usize) * PAGE_SIZE;
        let ptr = if use_huge { vmalloc_huge(size, GFP_KERNEL) } else { vmalloc(size) };
        if ptr.is_null() { return -1; }
        *(ptr as *mut u8) = 0; vfree(ptr);
    }
    0
}

unsafe fn no_block_alloc_test() -> i32 {
    for _i in 0..test_loop_count {
        let use_atomic = get_random_u8() % 2 != 0;
        let gfp = if use_atomic { GFP_ATOMIC } else { GFP_NOWAIT };
        let size = ((if nr_pages > 0 { nr_pages } else { 1 }) as usize) * PAGE_SIZE;
        preempt_disable(); let ptr = __vmalloc(size, gfp); preempt_enable();
        if ptr.is_null() { return -1; }
        *(ptr as *mut u8) = 0; vfree(ptr);
    }
    0
}

unsafe fn pcpu_alloc_test() -> i32 {
    let mut rv = 0;
    // CONFIG_NEED_PER_CPU_KM excludes this body when enabled.
    let pcpu = vmalloc(core::mem::size_of::<*mut core::ffi::c_void>() * nr_pcpu_objects as usize) as *mut *mut core::ffi::c_void;
    if pcpu.is_null() { return -1; }
    for i in 0..nr_pcpu_objects as usize {
        let size = get_random_u32_inclusive(1, (PAGE_SIZE / 4) as u32) as usize;
        let align = 1usize << get_random_u32_inclusive(1, PAGE_SHIFT - 1);
        *pcpu.add(i) = __alloc_percpu(size, align);
        if (*pcpu.add(i)).is_null() { rv = -1; }
    }
    for i in 0..nr_pcpu_objects as usize { free_percpu(*pcpu.add(i)); }
    vfree(pcpu as *mut _); rv
}

#[repr(C)]
struct test_kvfree_rcu { rcu: rcu_head, array: [u8; 20] }

unsafe fn kvfree_rcu_1_arg_vmalloc_test() -> i32 {
    for _i in 0..test_loop_count { let p = vmalloc(PAGE_SIZE) as *mut test_kvfree_rcu; if p.is_null() { return -1; } (*p).array[0] = b'a'; kvfree_rcu_mightsleep(p); }
    0
}
unsafe fn kvfree_rcu_2_arg_vmalloc_test() -> i32 {
    for _i in 0..test_loop_count { let p = vmalloc(PAGE_SIZE) as *mut test_kvfree_rcu; if p.is_null() { return -1; } (*p).array[0] = b'a'; kvfree_rcu(p, rcu); }
    0
}

unsafe fn vm_map_ram_test() -> i32 {
    let map_nr_pages = if nr_pages > 0 { nr_pages } else { 1 };
    let pages = kzalloc_objs::<*mut page>(map_nr_pages);
    if pages.is_null() { return -1; }
    let nr_allocated = alloc_pages_bulk(GFP_KERNEL, map_nr_pages, pages);
    if nr_allocated == map_nr_pages { for _i in 0..test_loop_count { let v_ptr = vm_map_ram(pages, map_nr_pages, NUMA_NO_NODE); *v_ptr = b'a'; vm_unmap_ram(v_ptr, map_nr_pages); } }
    for i in 0..nr_allocated { __free_page(*pages.add(i as usize)); }
    kfree(pages); (nr_allocated != map_nr_pages) as i32
}

unsafe fn vrealloc_test() -> i32 {
    for _i in 0..test_loop_count {
        let mut ptr = vrealloc(core::ptr::null_mut(), PAGE_SIZE, GFP_KERNEL); if ptr.is_null() { return -1; }
        *(ptr as *mut u8) = b'a';
        let mut err = 0;
        for size in [4 * PAGE_SIZE, PAGE_SIZE, PAGE_SIZE / 2, PAGE_SIZE] { let tmp = vrealloc(ptr, size, GFP_KERNEL); if tmp.is_null() || *(tmp as *mut u8) != b'a' { err = -1; break; } ptr = tmp; }
        vfree(ptr); if err != 0 { return err; }
    }
    0
}

#[repr(C)]
struct test_case_desc { test_name: *const i8, test_func: unsafe fn() -> i32, xfail: bool }

static mut test_case_array: [test_case_desc; 13] = [
    test_case_desc { test_name: b"fix_size_alloc_test\0".as_ptr() as *const i8, test_func: fix_size_alloc_test, xfail: false },
    test_case_desc { test_name: b"full_fit_alloc_test\0".as_ptr() as *const i8, test_func: full_fit_alloc_test, xfail: false },
    test_case_desc { test_name: b"long_busy_list_alloc_test\0".as_ptr() as *const i8, test_func: long_busy_list_alloc_test, xfail: false },
    test_case_desc { test_name: b"random_size_alloc_test\0".as_ptr() as *const i8, test_func: random_size_alloc_test, xfail: false },
    test_case_desc { test_name: b"fix_align_alloc_test\0".as_ptr() as *const i8, test_func: fix_align_alloc_test, xfail: false },
    test_case_desc { test_name: b"random_size_align_alloc_test\0".as_ptr() as *const i8, test_func: random_size_align_alloc_test, xfail: false },
    test_case_desc { test_name: b"align_shift_alloc_test\0".as_ptr() as *const i8, test_func: align_shift_alloc_test, xfail: true },
    test_case_desc { test_name: b"pcpu_alloc_test\0".as_ptr() as *const i8, test_func: pcpu_alloc_test, xfail: false },
    test_case_desc { test_name: b"kvfree_rcu_1_arg_vmalloc_test\0".as_ptr() as *const i8, test_func: kvfree_rcu_1_arg_vmalloc_test, xfail: false },
    test_case_desc { test_name: b"kvfree_rcu_2_arg_vmalloc_test\0".as_ptr() as *const i8, test_func: kvfree_rcu_2_arg_vmalloc_test, xfail: false },
    test_case_desc { test_name: b"vm_map_ram_test\0".as_ptr() as *const i8, test_func: vm_map_ram_test, xfail: false },
    test_case_desc { test_name: b"no_block_alloc_test\0".as_ptr() as *const i8, test_func: no_block_alloc_test, xfail: true },
    test_case_desc { test_name: b"vrealloc_test\0".as_ptr() as *const i8, test_func: vrealloc_test, xfail: false },
];

#[repr(C)]
struct test_case_data { test_failed: i32, test_xfailed: i32, test_passed: i32, time: u64 }
#[repr(C)]
struct test_driver { task: *mut task_struct, data: [test_case_data; 13], start: usize, stop: usize }
static mut tdriver: *mut test_driver = core::ptr::null_mut();

unsafe fn shuffle_array(arr: *mut i32, n: i32) { for i in (1..n).rev() { let j = get_random_u32_below(i as u32) as usize; core::ptr::swap(arr.add(i as usize), arr.add(j)); } }

unsafe fn test_func(private: *mut core::ffi::c_void) -> i32 {
    let t = private as *mut test_driver; let mut random_array = [0i32; 13];
    for i in 0..13 { random_array[i] = i as i32; }
    if !sequential_test_order { shuffle_array(random_array.as_mut_ptr(), 13); }
    synchronize_srcu(&mut prepare_for_test_srcu); (*t).start = get_cycles();
    for i in 0..13 { let index = random_array[i] as usize; if (run_test_mask & (1 << index)) == 0 { continue; } let kt = ktime_get(); for _j in 0..test_repeat_count { let ret = (test_case_array[index].test_func)(); if ret == 0 { (*t).data[index].test_passed += 1; } else if test_case_array[index].xfail { (*t).data[index].test_xfailed += 1; } else { (*t).data[index].test_failed += 1; } } (*t).data[index].time = (ktime_us_delta(ktime_get(), kt) as u64) / test_repeat_count as u64; }
    (*t).stop = get_cycles(); test_report_one_done(); while !kthread_should_stop() { msleep(10); } 0
}

unsafe fn init_test_configuration() -> i32 {
    nr_threads = clamp(nr_threads, 1, USHRT_MAX as i32); tdriver = kvzalloc_objs::<test_driver>(nr_threads as usize); if tdriver.is_null() { return -1; } if test_repeat_count <= 0 { test_repeat_count = 1; } if test_loop_count <= 0 { test_loop_count = 1; } 0
}

unsafe fn do_concurrent_test() {
    if init_test_configuration() < 0 { return; }
    let idx = srcu_read_lock(&mut prepare_for_test_srcu);
    for i in 0..nr_threads as usize { let t = tdriver.add(i); (*t).task = kthread_run(test_func, t as *mut _, b"vmalloc_test/%d\0".as_ptr() as *const i8, i as i32); if !IS_ERR((*t).task) { atomic_inc(&mut test_n_undone); } else { pr_err(b"Failed to start %d kthread\n\0".as_ptr() as *const i8, i as i32); } }
    srcu_read_unlock(&mut prepare_for_test_srcu, idx); while wait_for_completion_timeout(&mut test_all_done_comp, HZ) == 0 {}
    for i in 0..nr_threads as usize { let t = tdriver.add(i); if !IS_ERR((*t).task) { kthread_stop((*t).task); } for j in 0..13 { if (run_test_mask & (1 << j)) == 0 { continue; } pr_info(b"Summary: %s passed: %d failed: %d xfailed: %d repeat: %d loops: %d avg: %llu usec\n\0".as_ptr() as *const i8, test_case_array[j].test_name, (*t).data[j].test_passed, (*t).data[j].test_failed, (*t).data[j].test_xfailed, test_repeat_count, test_loop_count, (*t).data[j].time); } pr_info(b"All test took worker%d=%lu cycles\n\0".as_ptr() as *const i8, i as i32, (*t).stop - (*t).start); }
    kvfree(tdriver);
}

unsafe fn vmalloc_test_init() -> i32 { do_concurrent_test(); if IS_BUILTIN(CONFIG_TEST_VMALLOC) { 0 } else { -EAGAIN } }

// MODULE_INIT / late_initcall, MODULE_LICENSE("GPL"), MODULE_AUTHOR("Uladzislau Rezki"),
// MODULE_DESCRIPTION("vmalloc test module") are supplied as kernel metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
