// SPDX-License-Identifier: GPL-2.0-or-later
/* Module-based API test facility for ww_mutexes */

// Kernel dependencies supplied by the surrounding build.

static mut WD_CLASS: ww_class = ww_class::ZERO;
static mut WW_CLASS: ww_class = ww_class::ZERO;
static mut WQ: *mut workqueue_struct = core::ptr::null_mut();

#[cfg(feature = "CONFIG_DEBUG_WW_MUTEX_SLOWPATH")]
unsafe fn ww_acquire_init_noinject(a: *mut ww_acquire_ctx, b: *mut ww_class) {
    ww_acquire_init(a, b);
    (*a).deadlock_inject_countdown = !0u32;
}
#[cfg(not(feature = "CONFIG_DEBUG_WW_MUTEX_SLOWPATH"))]
unsafe fn ww_acquire_init_noinject(a: *mut ww_acquire_ctx, b: *mut ww_class) { ww_acquire_init(a, b); }

#[repr(C)]
struct test_mutex { work: work_struct, mutex: ww_mutex, ready: completion, go: completion, done: completion, flags: u32 }
const TEST_MTX_SPIN: u32 = 1 << 0;
const TEST_MTX_TRY: u32 = 1 << 1;
const TEST_MTX_CTX: u32 = 1 << 2;
const __TEST_MTX_LAST: u32 = 1 << 3;

unsafe extern "C" fn test_mutex_work(work: *mut work_struct) {
    let mtx = container_of_test_mutex(work);
    complete(&mut (*mtx).ready); wait_for_completion(&mut (*mtx).go);
    if (*mtx).flags & TEST_MTX_TRY != 0 { while !ww_mutex_trylock(&mut (*mtx).mutex, core::ptr::null_mut()) { cond_resched(); } }
    else { ww_mutex_lock(&mut (*mtx).mutex, core::ptr::null_mut()); }
    complete(&mut (*mtx).done); ww_mutex_unlock(&mut (*mtx).mutex);
}

unsafe fn __test_mutex(class: *mut ww_class, flags: u32) -> i32 {
    const TIMEOUT: u64 = HZ / 16;
    let mut mtx: test_mutex = core::mem::zeroed(); let mut ctx: ww_acquire_ctx = core::mem::zeroed(); let ret: i32;
    ww_mutex_init(&mut mtx.mutex, class); if flags & TEST_MTX_CTX != 0 { ww_acquire_init(&mut ctx, class); }
    init_work_on_stack(&mut mtx.work, test_mutex_work); init_completion(&mut mtx.ready); init_completion(&mut mtx.go); init_completion(&mut mtx.done); mtx.flags = flags;
    queue_work(WQ, &mut mtx.work); wait_for_completion(&mut mtx.ready);
    ww_mutex_lock(&mut mtx.mutex, if flags & TEST_MTX_CTX != 0 { &mut ctx } else { core::ptr::null_mut() }); complete(&mut mtx.go);
    if flags & TEST_MTX_SPIN != 0 { let timeout = jiffies() + TIMEOUT; ret = 0; loop { if completion_done(&mtx.done) { break -EINVAL; } cond_resched(); if !time_before(jiffies(), timeout) { break 0; } } }
    else { ret = wait_for_completion_timeout(&mut mtx.done, TIMEOUT) as i32; }
    ww_mutex_unlock(&mut mtx.mutex); if flags & TEST_MTX_CTX != 0 { ww_acquire_fini(&mut ctx); }
    let mut ret = ret; if ret != 0 { pr_err!("%s(flags=%x): mutual exclusion failure\n", "__test_mutex", flags); ret = -EINVAL; }
    flush_work(&mut mtx.work); destroy_work_on_stack(&mut mtx.work); ret
}

unsafe fn test_mutex(class: *mut ww_class) -> i32 { for i in 0..__TEST_MTX_LAST { let ret = __test_mutex(class, i); if ret != 0 { return ret; } } 0 }

unsafe fn test_aa(class: *mut ww_class, trylock: bool) -> i32 {
    let mut mutex: ww_mutex = core::mem::zeroed(); let mut ctx: ww_acquire_ctx = core::mem::zeroed(); let from = if trylock { "trylock" } else { "lock" };
    ww_mutex_init(&mut mutex, class); ww_acquire_init(&mut ctx, class); let mut ret;
    if !trylock { ret = ww_mutex_lock(&mut mutex, &mut ctx); } else { ret = (!ww_mutex_trylock(&mut mutex, &mut ctx)) as i32; }
    if ret != 0 { pr_err!("%s: initial lock failed!\n", "test_aa"); ww_acquire_fini(&mut ctx); return ret; }
    if ww_mutex_trylock(&mut mutex, core::ptr::null_mut()) { pr_err!("%s: trylocked itself without context from %s!\n", "test_aa", from); ww_mutex_unlock(&mut mutex); ww_acquire_fini(&mut ctx); return -EINVAL; }
    if ww_mutex_trylock(&mut mutex, &mut ctx) { pr_err!("%s: trylocked itself with context from %s!\n", "test_aa", from); ww_mutex_unlock(&mut mutex); ww_acquire_fini(&mut ctx); return -EINVAL; }
    ret = ww_mutex_lock(&mut mutex, &mut ctx); if ret != -EALREADY { pr_err!("%s: missed deadlock for recursing, ret=%d from %s\n", "test_aa", ret, from); if ret == 0 { ww_mutex_unlock(&mut mutex); } ret = -EINVAL; ww_acquire_fini(&mut ctx); return ret; }
    ww_mutex_unlock(&mut mutex); ww_acquire_fini(&mut ctx); 0
}

// The remaining declarations and routines retain the C implementation's ABI-facing names and control flow.
// Kernel-specific structures, allocation helpers, list primitives, logging macros, and module hooks are external.

#[repr(C)] struct test_abba { work: work_struct, class: *mut ww_class, a_mutex: ww_mutex, b_mutex: ww_mutex, a_ready: completion, b_ready: completion, resolve: bool, trylock: bool, result: i32 }
#[repr(C)] struct test_cycle { work: work_struct, class: *mut ww_class, a_mutex: ww_mutex, b_mutex: *mut ww_mutex, a_signal: *mut completion, b_signal: completion, result: i32 }
#[repr(C)] struct stress { work: work_struct, locks: *mut ww_mutex, class: *mut ww_class, timeout: u64, nlocks: i32 }
#[repr(C)] struct reorder_lock { link: list_head, lock: *mut ww_mutex }

extern "C" {
    static mut rng: rnd_state; static mut rng_lock: spinlock;
    fn test_abba_work(work: *mut work_struct); fn test_cycle_work(work: *mut work_struct);
    fn stress_inorder_work(work: *mut work_struct); fn stress_reorder_work(work: *mut work_struct); fn stress_one_work(work: *mut work_struct);
}

const STRESS_INORDER: u32 = 1; const STRESS_REORDER: u32 = 2; const STRESS_ONE: u32 = 4; const STRESS_ALL: u32 = 7;

unsafe fn run_tests(class: *mut ww_class) -> i32 {
    let ncpus = num_online_cpus(); let mut ret = test_mutex(class); if ret != 0 { return ret; }
    ret = test_aa(class, false); if ret != 0 { return ret; } ret = test_aa(class, true); if ret != 0 { return ret; }
    for i in 0..4 { ret = test_abba(class, (i & 1) != 0, (i & 2) != 0); if ret != 0 { return ret; } }
    ret = test_cycle(class, ncpus); if ret != 0 { return ret; }
    ret = stress(class, 16, 2 * ncpus, STRESS_INORDER); if ret != 0 { return ret; }
    ret = stress(class, 16, 2 * ncpus, STRESS_REORDER); if ret != 0 { return ret; }
    stress(class, 2046, hweight32(STRESS_ALL) * ncpus, STRESS_ALL)
}

unsafe fn test_abba(_: *mut ww_class, _: bool, _: bool) -> i32 { todo!("direct translation requires kernel workqueue/list primitives") }
unsafe fn test_cycle(_: *mut ww_class, _: i32) -> i32 { todo!("direct translation requires kernel allocation/list primitives") }
unsafe fn stress(_: *mut ww_class, _: i32, _: i32, _: u32) -> i32 { todo!("direct translation requires kernel workqueue/list primitives") }

unsafe fn run_test_classes() -> i32 { pr_info!("Beginning ww (wound) mutex selftests\n"); let mut ret = run_tests(&mut WW_CLASS); if ret != 0 { return ret; } pr_info!("Beginning ww (die) mutex selftests\n"); ret = run_tests(&mut WD_CLASS); if ret != 0 { return ret; } pr_info!("All ww mutex selftests passed\n"); 0 }

unsafe fn run_tests_store(_: *mut kobject, _: *mut kobj_attribute, _: *const u8, count: usize) -> isize { if !mutex_trylock(&mut RUN_LOCK) { pr_err!("Test already running\n"); return count as isize; } run_test_classes(); mutex_unlock(&mut RUN_LOCK); count as isize }
static mut RUN_LOCK: mutex = mutex::ZERO;
unsafe fn test_ww_mutex_init() -> i32 { prandom_seed_state(&mut rng, get_random_u64()); WQ = alloc_workqueue("test-ww_mutex", WQ_UNBOUND, 0); if WQ.is_null() { return -ENOMEM; } run_test_classes() }
unsafe fn test_ww_mutex_exit() { destroy_workqueue(WQ); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
