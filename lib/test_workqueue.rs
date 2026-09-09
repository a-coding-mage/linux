// SPDX-License-Identifier: GPL-2.0
//
// Test module for stress and performance analysis of workqueue.
//
// Benchmarks queue_work() throughput on an unbound workqueue to measure
// pool->lock contention under different affinity scope configurations
// (e.g., cache vs cache_shard).
//
// The affinity scope is changed between runs via the workqueue's sysfs
// affinity_scope attribute (WQ_SYSFS).

// Kernel headers and symbols are supplied by the surrounding kernel build.

const WQ_NAME: &str = "bench_wq";
const SCOPE_PATH: &str = "/sys/bus/workqueue/devices/bench_wq/affinity_scope";

static mut nr_threads: i32 = 0;
static mut wq_items: i32 = 50000;
static mut bench_wq: *mut workqueue_struct = core::ptr::null_mut();
static mut threads_done: atomic_t = atomic_t { counter: 0 };
static mut start_comp: completion = completion {};
static mut all_done_comp: completion = completion {};

#[repr(C)]
struct thread_ctx {
    work_done: completion,
    work: work_struct,
    latencies: *mut u64,
    cpu: i32,
    items: i32,
}

unsafe fn bench_work_fn(work: *mut work_struct) {
    let ctx: *mut thread_ctx = container_of!(work, thread_ctx, work);
    complete(&mut (*ctx).work_done);
}

unsafe fn bench_kthread_fn(data: *mut core::ffi::c_void) -> i32 {
    let ctx = data as *mut thread_ctx;
    let mut t_start: ktime_t;
    let mut t_end: ktime_t;

    // Wait for all threads to be ready
    wait_for_completion(&mut start_comp);

    if kthread_should_stop() {
        return 0;
    }

    for i in 0..(*ctx).items {
        reinit_completion(&mut (*ctx).work_done);
        INIT_WORK!(&mut (*ctx).work, bench_work_fn);

        t_start = ktime_get();
        queue_work(bench_wq, &mut (*ctx).work);
        t_end = ktime_get();

        *(*ctx).latencies.offset(i as isize) = ktime_to_ns(ktime_sub(t_end, t_start));
        wait_for_completion(&mut (*ctx).work_done);
    }

    if atomic_dec_and_test(&mut threads_done) {
        complete(&mut all_done_comp);
    }

    // Wait for kthread_stop() so the module text isn't freed
    // while we're still executing.
    while !kthread_should_stop() {
        schedule();
    }

    0
}

unsafe fn cmp_u64(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let va = *(a as *const u64);
    let vb = *(b as *const u64);
    if va < vb { -1 } else if va > vb { 1 } else { 0 }
}

unsafe fn set_affn_scope(scope: *const core::ffi::c_char) -> i32 {
    let mut pos: loff_t = 0;
    let f = filp_open(SCOPE_PATH.as_ptr() as *const core::ffi::c_char, O_WRONLY, 0);
    if IS_ERR!(f) {
        pr_err!("test_workqueue: open %s failed: %ld\n", SCOPE_PATH, PTR_ERR!(f));
        return PTR_ERR!(f);
    }
    let ret = kernel_write(f, scope, strlen(scope), &mut pos);
    filp_close(f, core::ptr::null_mut());
    if ret < 0 {
        pr_err!("test_workqueue: write failed: %zd\n", ret);
        return ret as i32;
    }
    0
}

unsafe fn run_bench(n_threads: i32, scope: *const core::ffi::c_char, label: *const core::ffi::c_char) -> i32 {
    let mut ret = set_affn_scope(scope);
    if ret != 0 { return ret; }

    let ctxs = kcalloc(n_threads as usize, core::mem::size_of::<thread_ctx>(), GFP_KERNEL) as *mut thread_ctx;
    if ctxs.is_null() { return -ENOMEM; }
    let tasks = kcalloc(n_threads as usize, core::mem::size_of::<*mut task_struct>(), GFP_KERNEL) as *mut *mut task_struct;
    if tasks.is_null() { kfree(ctxs as *mut core::ffi::c_void); return -ENOMEM; }

    let total_items = (n_threads as usize).wrapping_mul(wq_items as usize);
    let all_latencies = kvmalloc_array(total_items, core::mem::size_of::<u64>(), GFP_KERNEL) as *mut u64;
    if all_latencies.is_null() { kfree(tasks as *mut core::ffi::c_void); kfree(ctxs as *mut core::ffi::c_void); return -ENOMEM; }

    for i in 0..n_threads {
        (*ctxs.add(i as usize)).latencies = kvmalloc_array(wq_items as usize, core::mem::size_of::<u64>(), GFP_KERNEL) as *mut u64;
        if (*ctxs.add(i as usize)).latencies.is_null() {
            let mut j = i - 1;
            while j >= 0 { kvfree((*ctxs.add(j as usize)).latencies as *mut core::ffi::c_void); j -= 1; }
            kvfree(all_latencies as *mut core::ffi::c_void); kfree(tasks as *mut core::ffi::c_void); kfree(ctxs as *mut core::ffi::c_void); return -ENOMEM;
        }
    }

    atomic_set(&mut threads_done, n_threads);
    reinit_completion(&mut all_done_comp); reinit_completion(&mut start_comp);

    let mut i = 0;
    for cpu in for_each_online_cpu() {
        if i >= n_threads { break; }
        (*ctxs.add(i as usize)).cpu = cpu; (*ctxs.add(i as usize)).items = wq_items;
        init_completion(&mut (*ctxs.add(i as usize)).work_done);
        *tasks.add(i as usize) = kthread_create(bench_kthread_fn, ctxs.add(i as usize) as *mut core::ffi::c_void, "wq_bench/%d", cpu);
        if IS_ERR!(*tasks.add(i as usize)) {
            ret = PTR_ERR!(*tasks.add(i as usize)); complete_all(&mut start_comp);
            let mut j = i - 1; while j >= 0 { kthread_stop(*tasks.add(j as usize)); j -= 1; }
            break;
        }
        kthread_bind(*tasks.add(i as usize), cpu); wake_up_process(*tasks.add(i as usize)); i += 1;
    }

    let start = ktime_get(); complete_all(&mut start_comp); wait_for_completion(&mut all_done_comp);
    flush_workqueue(bench_wq);
    for i in 0..n_threads { kthread_stop(*tasks.add(i as usize)); }
    let end = ktime_get();
    let elapsed_us = ktime_us_delta(end, start);
    let mut j = 0;
    for i in 0..n_threads { memcpy(all_latencies.add(j), (*ctxs.add(i as usize)).latencies as *const core::ffi::c_void, wq_items as usize * core::mem::size_of::<u64>()); j += wq_items as usize; }
    sort(all_latencies as *mut core::ffi::c_void, total_items, core::mem::size_of::<u64>(), cmp_u64, core::ptr::null_mut());
    pr_info!("test_workqueue: %s %llu items/sec p50=%llu p90=%llu p95=%llu ns\n", label, if elapsed_us != 0 { (total_items as u64 * 1000000) / elapsed_us as u64 } else { 0 }, *all_latencies.add(total_items * 50 / 100), *all_latencies.add(total_items * 90 / 100), *all_latencies.add(total_items * 95 / 100));
    for i in 0..n_threads { kvfree((*ctxs.add(i as usize)).latencies as *mut core::ffi::c_void); }
    kvfree(all_latencies as *mut core::ffi::c_void); kfree(tasks as *mut core::ffi::c_void); kfree(ctxs as *mut core::ffi::c_void); ret
}

static bench_scopes: [&[u8]; 6] = [b"cpu\0", b"smt\0", b"cache_shard\0", b"cache\0", b"numa\0", b"system\0"];

unsafe fn test_workqueue_init() -> i32 {
    let n_threads = core::cmp::min(if nr_threads != 0 { nr_threads } else { num_online_cpus() }, num_online_cpus());
    if wq_items <= 0 { pr_err!("test_workqueue: wq_items must be > 0\n"); return -EINVAL; }
    bench_wq = alloc_workqueue(WQ_NAME, WQ_UNBOUND | WQ_SYSFS, 0);
    if bench_wq.is_null() { return -ENOMEM; }
    pr_info!("test_workqueue: running %d threads, %d items/thread\n", n_threads, wq_items);
    for scope in bench_scopes.iter() { run_bench(n_threads, scope.as_ptr() as *const _, scope.as_ptr() as *const _); }
    destroy_workqueue(bench_wq); -EAGAIN
}

// module_init(test_workqueue_init);
// MODULE_AUTHOR("Breno Leitao <leitao@debian.org>");
// MODULE_DESCRIPTION("Stress/performance benchmark for workqueue subsystem");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
