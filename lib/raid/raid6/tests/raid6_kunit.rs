// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2002-2007 H. Peter Anvin - All Rights Reserved
 *
 * Test RAID-6 recovery algorithms.
 */

// Kernel dependencies supplied by the surrounding build are intentionally not
// reimplemented here.

const RAID6_KUNIT_SEED: u32 = 42;
const RAID6_KUNIT_NUM_TEST_ITERS: u32 = 10;
const RAID6_KUNIT_MAX_BUFFERS: usize = 64; // Including P and Q
const RAID6_KUNIT_MAX_FAILURES: usize = 2;
const RAID6_KUNIT_MAX_BYTES: usize = PAGE_SIZE;
const RAID6_KUNIT_ALLOC_BYTES: usize = SZ_16K;

static mut rng: rnd_state = rnd_state { _private: [] };
static mut test_buffers: [*mut core::ffi::c_void; RAID6_KUNIT_MAX_BUFFERS] =
    [core::ptr::null_mut(); RAID6_KUNIT_MAX_BUFFERS];
static mut aligned_buffers: [*mut core::ffi::c_void; RAID6_KUNIT_MAX_BUFFERS] =
    [core::ptr::null_mut(); RAID6_KUNIT_MAX_BUFFERS];
static mut test_recov_buffers: [*mut core::ffi::c_void; RAID6_KUNIT_MAX_FAILURES] =
    [core::ptr::null_mut(); RAID6_KUNIT_MAX_FAILURES];
static mut test_buflen: usize = 0;

#[repr(C)]
struct test_args {
    recov_idx: u32,
    recov: *const raid6_recov_calls,
    gen_idx: u32,
    gen: *const raid6_calls,
}

static mut args: test_args = test_args {
    recov_idx: 0,
    recov: core::ptr::null(),
    gen_idx: 0,
    gen: core::ptr::null(),
};

unsafe fn rand32() -> u32 {
    prandom_u32_state(&raw mut rng)
}

/* Generate a random length that is a multiple of 512. */
unsafe fn random_length(max_length: u32) -> u32 {
    round_up((rand32() % max_length) + 1, 512)
}

unsafe fn random_nr_buffers() -> u32 {
    (rand32() % (RAID6_KUNIT_MAX_BUFFERS as u32 - (RAID6_MIN_DISKS - 1))) + RAID6_MIN_DISKS
}

/* Generate a random alignment that is a multiple of 64. */
unsafe fn random_alignment(max_alignment: u32) -> u32 {
    if max_alignment == 0 { return 0; }
    (rand32() % (max_alignment + 1)) & !63
}

unsafe fn makedata(start: i32, stop: i32) {
    for i in start..=stop {
        prandom_bytes_state(&raw mut rng, test_buffers[i as usize], test_buflen);
    }
}

unsafe fn member_type(nr_buffers: u32, d: i32) -> u8 {
    if d == nr_buffers as i32 - 2 { b'P' }
    else if d == nr_buffers as i32 - 1 { b'Q' }
    else { b'D' }
}

unsafe fn test_recover_one(test: *mut kunit, nr_buffers: u32, len: u32, mut faila: i32, mut failb: i32) {
    let ta = (*test).param_value as *const test_args;
    let mut dataptrs = [core::ptr::null_mut(); RAID6_KUNIT_MAX_BUFFERS];
    if faila > failb { core::mem::swap(&mut faila, &mut failb); }
    for i in 0..RAID6_KUNIT_MAX_FAILURES { memset(test_recov_buffers[i], 0xf0, test_buflen); }
    core::ptr::copy_nonoverlapping(aligned_buffers.as_ptr(), dataptrs.as_mut_ptr(), RAID6_KUNIT_MAX_BUFFERS);
    dataptrs[faila as usize] = test_recov_buffers[0];
    dataptrs[failb as usize] = test_recov_buffers[1];
    if failb == nr_buffers as i32 - 1 {
        /* We don't implement the data+Q failure scenario, since it is
         * equivalent to a RAID-5 failure (XOR, then recompute Q). */
        if WARN_ON_ONCE(faila != nr_buffers as i32 - 2) { return; }
        /* P+Q failure. Just rebuild the syndrome. */
        ((*(*ta).gen).gen_syndrome)(nr_buffers, len, dataptrs.as_mut_ptr());
    } else if failb == nr_buffers as i32 - 2 {
        /* data+P failure. */
        ((*(*ta).recov).datap)(nr_buffers, len, faila as u32, dataptrs.as_mut_ptr());
    } else {
        /* data+data failure. */
        ((*(*ta).recov).data2)(nr_buffers, len, faila as u32, failb as u32, dataptrs.as_mut_ptr());
    }
    KUNIT_EXPECT_MEMEQ_MSG(test, aligned_buffers[faila as usize], dataptrs[faila as usize], len,
        "faila miscompared: %3d[%c] buffers %u len %u (failb=%3d[%c])\n", faila,
        member_type(nr_buffers, faila), nr_buffers, len, failb, member_type(nr_buffers, failb));
    KUNIT_EXPECT_MEMEQ_MSG(test, aligned_buffers[failb as usize], dataptrs[failb as usize], len,
        "failb miscompared: %3d[%c] buffers %u len %u (faila=%3d[%c])\n", failb,
        member_type(nr_buffers, failb), nr_buffers, len, faila, member_type(nr_buffers, faila));
}

unsafe fn test_recover(test: *mut kunit, nr_buffers: u32, len: u32) {
    let nr_data = nr_buffers - 2;
    test_recover_one(test, nr_buffers, len, nr_data as i32, nr_buffers as i32 - 1);
    for i in 0..nr_buffers - 2 { test_recover_one(test, nr_buffers, len, i as i32, nr_data as i32); }
    if nr_data == 1 { return; }
    let iterations = nr_buffers * 2;
    for _ in 0..iterations {
        let faila = (rand32() % nr_data) as i32;
        let mut failb;
        loop { failb = (rand32() % nr_data) as i32; if failb != faila { break; } }
        test_recover_one(test, nr_buffers, len, faila, failb);
    }
}

/* Simulate rmw run */
unsafe fn test_rmw_one(test: *mut kunit, nr_buffers: u32, len: u32, p1: i32, p2: i32) {
    let ta = (*test).param_value as *const test_args;
    ((*(*ta).gen).xor_syndrome)(nr_buffers, p1 as u32, p2 as u32, len, aligned_buffers.as_mut_ptr());
    makedata(p1, p2);
    ((*(*ta).gen).xor_syndrome)(nr_buffers, p1 as u32, p2 as u32, len, aligned_buffers.as_mut_ptr());
    test_recover(test, nr_buffers, len);
}

unsafe fn test_rmw(test: *mut kunit, nr_buffers: u32, len: u32) {
    for _ in 0..nr_buffers / 2 {
        let mut p1 = (rand32() % (nr_buffers - 2)) as i32;
        let mut p2 = (rand32() % (nr_buffers - 2)) as i32;
        if p2 < p1 { core::mem::swap(&mut p1, &mut p2); }
        test_rmw_one(test, nr_buffers, len, p1, p2);
    }
}

unsafe fn raid6_test_one(test: *mut kunit) {
    let ta = (*test).param_value as *const test_args;
    let nr_buffers = random_nr_buffers();
    let len = random_length(RAID6_KUNIT_MAX_BYTES as u32);
    let max_alignment = RAID6_KUNIT_MAX_BYTES as u32 - len;
    memset(test_buffers[(nr_buffers - 2) as usize], 0xee, test_buflen);
    memset(test_buffers[(nr_buffers - 1) as usize], 0xee, test_buflen);
    if rand32() % 2 == 0 {
        for i in 0..nr_buffers as usize { aligned_buffers[i] = (test_buffers[i] as *mut u8).add(random_alignment(max_alignment) as usize) as *mut _; }
    } else {
        let align = test_buflen - len as usize;
        for i in 0..nr_buffers as usize { aligned_buffers[i] = (test_buffers[i] as *mut u8).add(align) as *mut _; }
    }
    ((*(*ta).gen).gen_syndrome)(nr_buffers, len, aligned_buffers.as_mut_ptr());
    test_recover(test, nr_buffers, len);
    if !(*ta).gen.is_null() && (*(*ta).gen).xor_syndrome.is_some() { test_rmw(test, nr_buffers, len); }
}

unsafe fn raid6_test(test: *mut kunit) {
    for _ in 0..RAID6_KUNIT_NUM_TEST_ITERS { raid6_test_one(test); }
}

// Benchmark and KUnit registration retain their kernel-facing declarations;
// the surrounding kernel bindings provide the referenced types and symbols.
unsafe fn raid6_benchmark(test: *mut kunit) {
    let nr_to_test: [u32; 10] = [4, 5, 6, 7, 8, 10, 12, 15, 16, 32];
    let len_to_test: [u32; 2] = [SZ_4K as u32, SZ_16K as u32];
    if !IS_ENABLED(CONFIG_RAID6_PQ_KUNIT_BENCHMARK) { kunit_skip(test, "not enabled"); return; }
    for &nr in &nr_to_test { for &len in &len_to_test { for _ in 0..10 { raid6_gen_syndrome(nr, len, test_buffers.as_mut_ptr()); } } }
    kunit_info(test, "          \t%5u bytes\t%5u bytes\n", len_to_test[0], len_to_test[1]);
    for &nr in &nr_to_test { let mut speed = [0u64; 2]; for j in 0..2 { let len = len_to_test[j]; let n: u64 = 1000; preempt_disable(); let mut t = ktime_get_ns(); for _ in 0..n { raid6_gen_syndrome(nr, len, test_buffers.as_mut_ptr()); } t = core::cmp::max(ktime_get_ns() - t, 1); preempt_enable(); speed[j] = div64_u64(len as u64 * n * nr as u64, t); } kunit_info(test, "%3u disks:\t%5llu  GB/s\t%5llu  GB/s\n", nr, speed[0], speed[1]); }
}

unsafe fn raid6_gen_params(_test: *mut kunit, prev: *const core::ffi::c_void, desc: *mut u8) -> *const core::ffi::c_void {
    if prev.is_null() { args = core::mem::zeroed(); }
    loop { if prev.is_null() || args.recov.is_null() { args.recov_idx = 0; args.gen = raid6_algo_find(args.gen_idx); if args.gen.is_null() { return core::ptr::null(); } } if !args.recov.is_null() { args.recov_idx += 1; } args.recov = raid6_recov_algo_find(args.recov_idx); if args.recov.is_null() { args.gen_idx += 1; continue; } snprintf(desc, KUNIT_PARAM_DESC_SIZE, b"gen=%s recov=%s\0".as_ptr(), (*args.gen).name, (*args.recov).name); return &raw const args as *const _ as *const core::ffi::c_void; }
}

#[repr(C)]
struct kunit_suite {
    name: *const u8,
    test_cases: *mut kunit_case,
    suite_init: Option<unsafe extern "C" fn(*mut kunit_suite) -> i32>,
    suite_exit: Option<unsafe extern "C" fn(*mut kunit_suite)>,
}

unsafe extern "C" fn raid6_suite_init(_suite: *mut kunit_suite) -> i32 {
    prandom_seed_state(&raw mut rng, RAID6_KUNIT_SEED);
    test_buflen = round_up(RAID6_KUNIT_ALLOC_BYTES, PAGE_SIZE);
    for i in 0..RAID6_KUNIT_MAX_FAILURES {
        test_recov_buffers[i] = vmalloc(test_buflen);
        if test_recov_buffers[i].is_null() { for j in 0..RAID6_KUNIT_MAX_FAILURES { vfree(test_recov_buffers[j]); test_recov_buffers[j] = core::ptr::null_mut(); } return -ENOMEM; }
    }
    for i in 0..RAID6_KUNIT_MAX_BUFFERS {
        test_buffers[i] = vmalloc(test_buflen);
        if test_buffers[i].is_null() { for j in 0..RAID6_KUNIT_MAX_BUFFERS { vfree(test_buffers[j]); test_buffers[j] = core::ptr::null_mut(); } for j in 0..RAID6_KUNIT_MAX_FAILURES { vfree(test_recov_buffers[j]); test_recov_buffers[j] = core::ptr::null_mut(); } return -ENOMEM; }
    }
    makedata(0, RAID6_KUNIT_MAX_BUFFERS as i32 - 1);
    0
}

unsafe extern "C" fn raid6_suite_exit(_suite: *mut kunit_suite) {
    for i in 0..RAID6_KUNIT_MAX_BUFFERS { vfree(test_buffers[i]); test_buffers[i] = core::ptr::null_mut(); }
    for i in 0..RAID6_KUNIT_MAX_FAILURES { vfree(test_recov_buffers[i]); test_recov_buffers[i] = core::ptr::null_mut(); }
}

static mut raid6_test_suite: kunit_suite = kunit_suite {
    name: b"raid6\0".as_ptr(), test_cases: core::ptr::null_mut(),
    suite_init: Some(raid6_suite_init), suite_exit: Some(raid6_suite_exit),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
