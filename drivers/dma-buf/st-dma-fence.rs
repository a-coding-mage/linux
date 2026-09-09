/* SPDX-License-Identifier: MIT */

/*
 * Copyright © 2019 Intel Corporation
 */

// Kernel dependencies supplied by the surrounding Linux/Rust environment.

unsafe extern "C" fn mock_name(_f: *mut dma_fence) -> *const c_char {
    c"mock".as_ptr()
}

static MOCK_OPS: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(mock_name),
    get_timeline_name: Some(mock_name),
};

unsafe fn mock_fence() -> *mut dma_fence {
    let f: *mut dma_fence = kmalloc(core::mem::size_of::<dma_fence>(), GFP_KERNEL);
    if f.is_null() {
        return core::ptr::null_mut();
    }

    dma_fence_init(f, &MOCK_OPS, core::ptr::null_mut(), 0, 0);
    f
}

unsafe fn test_sanitycheck(test: *mut kunit) {
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, f);

    dma_fence_enable_signaling(f);
    dma_fence_signal(f);
    dma_fence_put(f);
}

unsafe fn test_signaling(test: *mut kunit) {
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, f);

    dma_fence_enable_signaling(f);

    if dma_fence_is_signaled(f) {
        KUNIT_FAIL(test, "Fence unexpectedly signaled on creation");
        dma_fence_put(f);
        return;
    }
    if dma_fence_check_and_signal(f) {
        KUNIT_FAIL(test, "Fence reported being already signaled");
        dma_fence_put(f);
        return;
    }
    if !dma_fence_is_signaled(f) {
        KUNIT_FAIL(test, "Fence not reporting signaled");
        dma_fence_put(f);
        return;
    }
    if !dma_fence_test_signaled_flag(f) {
        KUNIT_FAIL(test, "Fence reported not being already signaled");
        dma_fence_put(f);
        return;
    }
    if !rcu_dereference_protected((*f).ops, true) .is_null() {
        KUNIT_FAIL(test, "Fence ops not cleared on signal");
    }
    dma_fence_put(f);
}

#[repr(C)]
struct simple_cb {
    cb: dma_fence_cb,
    seen: bool,
}

unsafe extern "C" fn simple_callback(_f: *mut dma_fence, cb: *mut dma_fence_cb) {
    let p = container_of!(cb, simple_cb, cb);
    smp_store_mb((*p).seen, true);
}

unsafe fn test_add_callback(test: *mut kunit) {
    let mut cb: simple_cb = core::mem::zeroed();
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, f);
    if dma_fence_add_callback(f, &mut cb.cb, Some(simple_callback)) != 0 {
        KUNIT_FAIL(test, "Failed to add callback, fence already signaled!");
        dma_fence_put(f);
        return;
    }
    dma_fence_signal(f);
    if !cb.seen {
        KUNIT_FAIL(test, "Callback failed!");
    }
    dma_fence_put(f);
}

unsafe fn test_late_add_callback(test: *mut kunit) {
    let mut cb: simple_cb = core::mem::zeroed();
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, f);
    dma_fence_enable_signaling(f);
    dma_fence_signal(f);
    if dma_fence_add_callback(f, &mut cb.cb, Some(simple_callback)) == 0 {
        KUNIT_FAIL(test, "Added callback, but fence was already signaled!");
        dma_fence_put(f);
        return;
    }
    dma_fence_signal(f);
    if cb.seen {
        KUNIT_FAIL(test, "Callback called after failed attachment!");
    }
    dma_fence_put(f);
}

unsafe fn test_rm_callback(test: *mut kunit) {
    let mut cb: simple_cb = core::mem::zeroed();
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, f);
    if dma_fence_add_callback(f, &mut cb.cb, Some(simple_callback)) != 0 {
        KUNIT_FAIL(test, "Failed to add callback, fence already signaled!");
        dma_fence_put(f);
        return;
    }
    if !dma_fence_remove_callback(f, &mut cb.cb) {
        KUNIT_FAIL(test, "Failed to remove callback!");
        dma_fence_put(f);
        return;
    }
    dma_fence_signal(f);
    if cb.seen {
        KUNIT_FAIL(test, "Callback still signaled after removal!");
    }
    dma_fence_put(f);
}

unsafe fn test_late_rm_callback(test: *mut kunit) {
    let mut cb: simple_cb = core::mem::zeroed();
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, f);
    if dma_fence_add_callback(f, &mut cb.cb, Some(simple_callback)) != 0 {
        KUNIT_FAIL(test, "Failed to add callback, fence already signaled!");
        dma_fence_put(f);
        return;
    }
    dma_fence_signal(f);
    if !cb.seen {
        KUNIT_FAIL(test, "Callback failed!");
        dma_fence_put(f);
        return;
    }
    if dma_fence_remove_callback(f, &mut cb.cb) {
        KUNIT_FAIL(test, "Callback removal succeeded after being executed!");
    }
    dma_fence_put(f);
}

unsafe fn test_status(test: *mut kunit) {
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, f);
    dma_fence_enable_signaling(f);
    if dma_fence_get_status(f) != 0 {
        KUNIT_FAIL(test, "Fence unexpectedly has signaled status on creation");
        dma_fence_put(f);
        return;
    }
    dma_fence_signal(f);
    if dma_fence_get_status(f) == 0 {
        KUNIT_FAIL(test, "Fence not reporting signaled status");
    }
    dma_fence_put(f);
}

unsafe fn test_error(test: *mut kunit) {
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, f);
    dma_fence_enable_signaling(f);
    dma_fence_set_error(f, -EIO);
    if dma_fence_get_status(f) != 0 {
        KUNIT_FAIL(test, "Fence unexpectedly has error status before signal");
        dma_fence_put(f);
        return;
    }
    dma_fence_signal(f);
    if dma_fence_get_status(f) != -EIO {
        KUNIT_FAIL(test, "Fence not reporting error status, got %d", dma_fence_get_status(f));
    }
    dma_fence_put(f);
}

unsafe fn test_wait(test: *mut kunit) {
    let f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, f);
    dma_fence_enable_signaling(f);
    if dma_fence_wait_timeout(f, false, 0) != 0 {
        KUNIT_FAIL(test, "Wait reported complete before being signaled");
        dma_fence_signal(f);
        dma_fence_put(f);
        return;
    }
    dma_fence_signal(f);
    if dma_fence_wait_timeout(f, false, 0) != 1 {
        KUNIT_FAIL(test, "Wait reported incomplete after being signaled");
    }
    dma_fence_signal(f);
    dma_fence_put(f);
}

#[repr(C)]
struct wait_timer {
    timer: timer_list,
    f: *mut dma_fence,
}

unsafe extern "C" fn wait_timer(timer: *mut timer_list) {
    let wt = timer_container_of!(timer, wait_timer, timer);
    dma_fence_signal((*wt).f);
}

unsafe fn test_wait_timeout(test: *mut kunit) {
    let mut wt: wait_timer = core::mem::zeroed();
    timer_setup_on_stack(&mut wt.timer, Some(wait_timer), 0);
    wt.f = mock_fence();
    KUNIT_ASSERT_NOT_NULL(test, wt.f);
    dma_fence_enable_signaling(wt.f);
    if dma_fence_wait_timeout(wt.f, false, 1) != 0 {
        KUNIT_FAIL(test, "Wait reported complete before being signaled");
        timer_delete_sync(&mut wt.timer);
        timer_destroy_on_stack(&mut wt.timer);
        dma_fence_signal(wt.f);
        dma_fence_put(wt.f);
        return;
    }
    mod_timer(&mut wt.timer, jiffies + 1);
    if dma_fence_wait_timeout(wt.f, false, HZ) == 0 {
        if timer_pending(&wt.timer) {
            kunit_mark_skipped(test, "Timer did not fire within on HZ!\n");
        } else {
            KUNIT_FAIL(test, "Wait reported incomplete after timeout");
        }
    }
    timer_delete_sync(&mut wt.timer);
    timer_destroy_on_stack(&mut wt.timer);
    dma_fence_signal(wt.f);
    dma_fence_put(wt.f);
}

unsafe fn test_stub(test: *mut kunit) {
    let mut f: [*mut dma_fence; 64] = [core::ptr::null_mut(); 64];
    let mut i = 0usize;
    while i < f.len() {
        f[i] = dma_fence_get_stub();
        if !dma_fence_is_signaled(f[i]) {
            KUNIT_FAIL(test, "Obtained unsignaled stub fence!");
            break;
        }
        i += 1;
    }
    while i != 0 {
        i -= 1;
        dma_fence_put(f[i]);
    }
}

/* Now off to the races! */

#[repr(C)]
struct race_thread {
    fences: *mut *mut dma_fence,
    task: *mut task_struct,
    before: bool,
    id: i32,
}

unsafe fn __wait_for_callbacks(f: *mut dma_fence) {
    let mut flags: ulong = 0;
    dma_fence_lock_irqsave(f, &mut flags);
    dma_fence_unlock_irqrestore(f, flags);
}

unsafe extern "C" fn thread_signal_callback(arg: *mut c_void) -> i32 {
    let t = arg as *const race_thread;
    let mut pass: ulong = 0;
    let mut miss: ulong = 0;
    let mut err = 0;
    while err == 0 && !kthread_should_stop() {
        let mut cb: simple_cb = core::mem::zeroed();
        let f1 = mock_fence();
        if f1.is_null() { err = -ENOMEM; break; }
        dma_fence_enable_signaling(f1);
        rcu_assign_pointer((*t).fences.add((*t).id as usize), f1);
        smp_wmb();
        rcu_read_lock();
        let mut f2;
        loop {
            f2 = dma_fence_get_rcu_safe((*t).fences.add((!(*t).id) as usize));
            if !f2.is_null() || kthread_should_stop() { break; }
        }
        rcu_read_unlock();
        if (*t).before { dma_fence_signal(f1); }
        smp_store_mb(cb.seen, false);
        if f2.is_null() || dma_fence_add_callback(f2, &mut cb.cb, Some(simple_callback)) != 0 {
            miss += 1; cb.seen = true;
        }
        if !(*t).before { dma_fence_signal(f1); }
        if !cb.seen { dma_fence_wait(f2, false); __wait_for_callbacks(f2); }
        if !READ_ONCE(cb.seen) {
            pr_err!("Callback not seen on thread %d, pass %lu (%lu misses), signaling %s add_callback; fence signaled? %s\n", (*t).id, pass, miss, if (*t).before { "before" } else { "after" }, if dma_fence_is_signaled(f2) { "yes" } else { "no" });
            err = -EINVAL;
        }
        dma_fence_put(f2);
        rcu_assign_pointer((*t).fences.add((*t).id as usize), core::ptr::null_mut());
        smp_wmb();
        dma_fence_put(f1);
        pass += 1;
    }
    pr_info!("thread_signal_callback[%d] completed %lu passes, %lu misses\n", (*t).id, pass, miss);
    err
}

unsafe fn test_race_signal_callback(test: *mut kunit) {
    let mut f: [*mut dma_fence; 2] = [core::ptr::null_mut(); 2];
    let mut ret = 0;
    if num_online_cpus() < 2 { kunit_skip(test, "requires at least 2 CPUs"); }
    let mut pass = 0;
    while ret == 0 && pass <= 1 {
        let mut t: [race_thread; 2] = core::mem::zeroed();
        let mut i = 0;
        while i < t.len() {
            t[i].fences = f.as_mut_ptr(); t[i].id = i as i32; t[i].before = pass != 0;
            t[i].task = kthread_run(Some(thread_signal_callback), &mut t[i] as *mut _ as *mut c_void, "dma-fence:%d", i);
            if IS_ERR(t[i].task) { KUNIT_FAIL(test, "Failed to create kthread"); while i != 0 { i -= 1; kthread_stop_put(t[i].task); } return; }
            get_task_struct(t[i].task); i += 1;
        }
        msleep(50);
        i = 0;
        while i < t.len() { let e = kthread_stop_put(t[i].task); if e != 0 && ret == 0 { ret = e; } i += 1; }
        pass += 1;
    }
    KUNIT_EXPECT_EQ(test, ret, 0);
}

unsafe fn dma_fence_suite_init(_suite: *mut kunit_suite) -> i32 {
    pr_info!("sizeof(dma_fence)=%zu\n", core::mem::size_of::<dma_fence>());
    0
}

static mut DMA_FENCE_CASES: [kunit_case; 13] = [
    KUNIT_CASE!(test_sanitycheck),
    KUNIT_CASE!(test_signaling),
    KUNIT_CASE!(test_add_callback),
    KUNIT_CASE!(test_late_add_callback),
    KUNIT_CASE!(test_rm_callback),
    KUNIT_CASE!(test_late_rm_callback),
    KUNIT_CASE!(test_status),
    KUNIT_CASE!(test_error),
    KUNIT_CASE!(test_wait),
    KUNIT_CASE!(test_wait_timeout),
    KUNIT_CASE!(test_stub),
    KUNIT_CASE!(test_race_signal_callback),
    KUNIT_CASE_NONE!(),
];

static mut DMA_FENCE_TEST_SUITE: kunit_suite = kunit_suite {
    name: c"dma-buf-fence".as_ptr(),
    suite_init: Some(dma_fence_suite_init),
    test_cases: unsafe { DMA_FENCE_CASES.as_mut_ptr() },
};

// kunit_test_suite(dma_fence_test_suite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
