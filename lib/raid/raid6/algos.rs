// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2002 H. Peter Anvin - All Rights Reserved
 *
 * Algorithm list and algorithm selection for RAID-6
 */

const RAID6_MAX_ALGOS: usize = 16;
static mut RAID6_ALGOS: [*const raid6_calls; RAID6_MAX_ALGOS] = [core::ptr::null(); RAID6_MAX_ALGOS];
static mut RAID6_NR_ALGOS: u32 = 0;
static mut RAID6_RECOV_ALGO: *const raid6_recov_calls = core::ptr::null();

/* Selected algorithm */
static mut RAID6_GEN_SYNDROME_IMPL: Option<unsafe extern "C" fn(i32, usize, *mut *mut core::ffi::c_void)> = None;
static mut RAID6_XOR_SYNDROME_IMPL: Option<unsafe extern "C" fn(i32, i32, i32, usize, *mut *mut core::ffi::c_void)> = None;
static mut RAID6_RECOV_2DATA_IMPL: Option<unsafe extern "C" fn(i32, usize, i32, i32, *mut *mut core::ffi::c_void)> = None;
static mut RAID6_RECOV_DATAP_IMPL: Option<unsafe extern "C" fn(i32, usize, i32, *mut *mut core::ffi::c_void)> = None;

pub unsafe extern "C" fn raid6_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) {
    WARN_ON_ONCE(!in_task() || irqs_disabled() || softirq_count());
    WARN_ON_ONCE(bytes & 511 != 0);
    WARN_ON_ONCE(disks < RAID6_MIN_DISKS);
    if let Some(f) = RAID6_GEN_SYNDROME_IMPL { f(disks, bytes, ptrs); }
}

pub unsafe extern "C" fn raid6_xor_syndrome(disks: i32, start: i32, stop: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) {
    WARN_ON_ONCE(!in_task() || irqs_disabled() || softirq_count());
    WARN_ON_ONCE(bytes & 511 != 0);
    WARN_ON_ONCE(disks < RAID6_MIN_DISKS);
    WARN_ON_ONCE(stop < start);
    if let Some(f) = RAID6_XOR_SYNDROME_IMPL { f(disks, start, stop, bytes, ptrs); }
}

pub unsafe extern "C" fn raid6_can_xor_syndrome() -> bool { RAID6_XOR_SYNDROME_IMPL.is_some() }

pub unsafe extern "C" fn raid6_recov_2data(disks: i32, bytes: usize, faila: i32, failb: i32, ptrs: *mut *mut core::ffi::c_void) {
    WARN_ON_ONCE(!in_task() || irqs_disabled() || softirq_count());
    WARN_ON_ONCE(bytes & 511 != 0);
    WARN_ON_ONCE(bytes > PAGE_SIZE);
    WARN_ON_ONCE(failb <= faila);
    if let Some(f) = RAID6_RECOV_2DATA_IMPL { f(disks, bytes, faila, failb, ptrs); }
}

pub unsafe extern "C" fn raid6_recov_datap(disks: i32, bytes: usize, faila: i32, ptrs: *mut *mut core::ffi::c_void) {
    WARN_ON_ONCE(!in_task() || irqs_disabled() || softirq_count());
    WARN_ON_ONCE(bytes & 511 != 0);
    WARN_ON_ONCE(bytes > PAGE_SIZE);
    if let Some(f) = RAID6_RECOV_DATAP_IMPL { f(disks, bytes, faila, ptrs); }
}

const BENCH_SIZE: usize = SZ_4K;
const NR_SRCS: i32 = 8;
const NR_DISKS: i32 = NR_SRCS + 2;
const REPS: u32 = 800;

unsafe fn raid6_choose_gen(dptrs: *mut *mut core::ffi::c_void, disks: i32) -> i32 {
    let mut best: *const raid6_calls = core::ptr::null();
    let mut bestgenperf: u64 = 0;
    let mut i: u32 = 0;
    while i < RAID6_NR_ALGOS {
        let algo = RAID6_ALGOS[i as usize];
        let mut t;
        preempt_disable(); t = ktime_get_ns();
        for _ in 0..REPS { ((*algo).gen_syndrome)(disks, BENCH_SIZE, dptrs); }
        t = core::cmp::max(ktime_get_ns().wrapping_sub(t), 1); preempt_enable();
        let perf = div64_u64((BENCH_SIZE as u64) * (REPS as u64) * (NR_SRCS as u64) * 1000, t);
        if perf > bestgenperf { bestgenperf = perf; best = algo; }
        pr_info!("raid6: %-8s gen() %5lu MB/s\n", (*algo).name, perf);
        i += 1;
    }
    if best.is_null() { pr_err!("raid6: Yikes! No algorithm found!\n"); return -EINVAL; }
    RAID6_GEN_SYNDROME_IMPL = Some((*best).gen_syndrome);
    RAID6_XOR_SYNDROME_IMPL = (*best).xor_syndrome;
    pr_info!("raid6: using algorithm %s gen() %ld MB/s\n", (*best).name, bestgenperf);
    if let Some(xor) = (*best).xor_syndrome {
        let start = disks / 2 - 1; let stop = disks - 3;
        preempt_disable(); t = ktime_get_ns();
        for _ in 0..REPS { xor(disks, start, stop, BENCH_SIZE, dptrs); }
        t = core::cmp::max(ktime_get_ns().wrapping_sub(t), 1); preempt_enable();
        pr_info!("raid6: .... xor() %llu MB/s, rmw enabled\n", div64_u64((BENCH_SIZE as u64) * (REPS as u64) * (NR_SRCS as u64) / 2 * 1000, t));
    }
    0
}

unsafe fn raid6_select_algo() -> i32 {
    let disks = NR_DISKS; let mut dptrs: [*mut core::ffi::c_void; NR_DISKS as usize] = [core::ptr::null_mut(); NR_DISKS as usize];
    if !IS_ENABLED!(CONFIG_RAID6_PQ_BENCHMARK) || RAID6_NR_ALGOS == 1 { pr_info!("raid6: skipped pq benchmark and selected %s\n", (*RAID6_ALGOS[(RAID6_NR_ALGOS - 1) as usize]).name); return 0; }
    let disk_ptr = kmalloc_array(NR_DISKS as usize, BENCH_SIZE, GFP_KERNEL);
    if disk_ptr.is_null() { pr_err!("raid6: Yikes!  No memory available.\n"); return -ENOMEM; }
    for i in 0..disks as usize { dptrs[i] = disk_ptr.add(BENCH_SIZE * i) as *mut _; }
    let cycle = ((disks - 2) as usize * BENCH_SIZE) / 65536;
    for i in 0..cycle { core::ptr::copy_nonoverlapping(raid6_gfmul, disk_ptr.add(65536 * i), 65536); }
    let rem = ((disks - 2) as usize * BENCH_SIZE) % 65536;
    if rem != 0 { core::ptr::copy_nonoverlapping(raid6_gfmul, disk_ptr.add(65536 * cycle), rem); }
    let error = raid6_choose_gen(dptrs.as_mut_ptr(), disks); kfree(disk_ptr); error
}

pub unsafe fn raid6_algo_add(algo: *const raid6_calls) { if WARN_ON_ONCE!(RAID6_NR_ALGOS as usize == RAID6_MAX_ALGOS) { return; } RAID6_ALGOS[RAID6_NR_ALGOS as usize] = algo; RAID6_NR_ALGOS += 1; }
pub unsafe fn raid6_algo_add_default() { raid6_algo_add(&raid6_intx1); raid6_algo_add(&raid6_intx2); raid6_algo_add(&raid6_intx4); raid6_algo_add(&raid6_intx8); }
pub unsafe fn raid6_recov_algo_add(algo: *const raid6_recov_calls) { if WARN_ON_ONCE!(!RAID6_RECOV_ALGO.is_null()) { return; } RAID6_RECOV_ALGO = algo; }

unsafe fn arch_raid6_init() { raid6_algo_add_default(); }

unsafe fn raid6_init() -> i32 {
    arch_raid6_init();
    if RAID6_RECOV_ALGO.is_null() { RAID6_RECOV_ALGO = &raid6_recov_intx1; }
    RAID6_RECOV_2DATA_IMPL = Some((*RAID6_RECOV_ALGO).data2); RAID6_RECOV_DATAP_IMPL = Some((*RAID6_RECOV_ALGO).datap);
    pr_info!("raid6: using %s recovery algorithm\n", (*RAID6_RECOV_ALGO).name);
    RAID6_GEN_SYNDROME_IMPL = Some((*RAID6_ALGOS[(RAID6_NR_ALGOS - 1) as usize]).gen_syndrome);
    RAID6_XOR_SYNDROME_IMPL = (*RAID6_ALGOS[(RAID6_NR_ALGOS - 1) as usize]).xor_syndrome;
    0
}

unsafe fn raid6_exit() {}

#[cfg(CONFIG_RAID6_PQ_KUNIT_TEST)]
pub unsafe fn raid6_algo_find(idx: u32) -> *const raid6_calls {
    if idx >= RAID6_NR_ALGOS { if idx == RAID6_NR_ALGOS && RAID6_ALGOS[0] != &raid6_intx1 { return &raid6_intx1; } return core::ptr::null(); }
    RAID6_ALGOS[idx as usize]
}

#[cfg(CONFIG_RAID6_PQ_KUNIT_TEST)]
pub unsafe fn raid6_recov_algo_find(idx: u32) -> *const raid6_recov_calls {
    match idx { 0 => &raid6_recov_intx1, 1 => if RAID6_RECOV_ALGO != &raid6_recov_intx1 { RAID6_RECOV_ALGO } else { core::ptr::null() }, _ => core::ptr::null() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
