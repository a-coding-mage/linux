// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause

// Translated from C. Dependencies originally supplied by:
// <bpf_atomic.h>, <libarena/common.h>, <libarena/asan.h>, <libarena/spmc.h>

type u64 = u64;

const TEST_SPMC_THREADS: u64 = 3;
const TEST_SPMC_STEALERS: u64 = TEST_SPMC_THREADS - 1;

/*
 * The test requires the stealers/owners to sometimes quiesce
 * before continuing the benchmark. Normally we'd use something
 * like a condition variable, but since the benchmark is short-lived
 * and operations are wait-free we just spin around the quiescence
 * point instead. If we time out, we just fail the benchmark.
 */
const TEST_SPMC_SYNC_SPINS: u64 = BPF_MAX_LOOPS as u64;

/*
 * We track all the values we retrieve from the queue
 * to get some guarantee we're, not corrupting data,
 * e.g., accidentally reusing a past value from a slot.
 */
const TEST_SPMC_MAX_VALUES: u64 = 1024;
static mut seen: [u64; TEST_SPMC_MAX_VALUES as usize] = [0; TEST_SPMC_MAX_VALUES as usize];

/* The single spmc queue for the benchmark. */
static mut spmc: *mut spmc = core::ptr::null_mut();

/* Owner and stealer epochs. We define the , */
static mut owner_epoch: u64 = 0;
static mut stealer_epoch: u64 = 0;

/* Map owner epochs to stealer epochs (simply scale by # of stealers). */
#[inline(always)]
fn STEALER_EPOCH(owner_epoch: u64) -> u64 {
    owner_epoch.wrapping_mul(TEST_SPMC_STEALERS)
}

/* Global abort switch. If any thread fails, all others exit ASAP. */
static mut test_abort: bool = false;

/*
 * Counters useful for ensuring conservation of pushes/pops of unique values
 * (we're not stealing/popping more/fewer items than were pushed).
 */
static mut expected_total: u64 = 0;
static mut total_seen: u64 = 0;

/* Measure how many pops and steals we've made (irrespective of retrieved value). */
static mut pops: u64 = 0;
static mut steals: u64 = 0;

/* Used for the resize selftest, see below. */
static mut stealers_started: u64 = 0;

/* Used for the mixed selftest, see below. */
static mut round_steals: u64 = 0;

/*
 * We have multiple stealers and a single owner. We sometimes want the owner
 * to successfully outproduce the stealers, we add a busy loop in them.
 */
const TEST_SPMC_WASTE_ROUNDS: u64 = 1u64 << 12;

/*
 * The spmc data structure depends on the runtime fully
 * supporting acquire/release semantics, which is not
 * the case for all architectures.
 *
 * Original C enables this only when ENABLE_ATOMICS_TESTS is defined and the
 * target is arm64, x86, or 64-bit riscv.
 */
fn spmc_tests_enabled() -> bool {
    cfg!(all(
        feature = "ENABLE_ATOMICS_TESTS",
        any(
            target_arch = "aarch64",
            target_arch = "x86",
            target_arch = "x86_64",
            all(target_arch = "riscv64", target_pointer_width = "64")
        )
    ))
}

unsafe fn spmc_common_init(total: u64) -> i32 {
    let mut i: u64;

    if total > TEST_SPMC_MAX_VALUES {
        return -E2BIG;
    }

    owner_epoch = 0;
    stealer_epoch = 0;
    test_abort = false;
    expected_total = total;
    total_seen = 0;
    pops = 0;
    steals = 0;
    stealers_started = 0;
    round_steals = 0;

    i = zero as u64;
    while i < TEST_SPMC_MAX_VALUES && can_loop {
        seen[i as usize] = 0;
        i = i.wrapping_add(1);
    }

    spmc = spmc_create();
    if spmc.is_null() {
        return -ENOMEM;
    }

    0
}

unsafe fn spmc_common_fini() -> i32 {
    let ret: i32;

    ret = spmc_destroy(spmc);
    spmc = core::ptr::null_mut();

    ret
}

#[no_mangle]
pub unsafe extern "C" fn spmc_quiesce_on_owner(epoch: u64) -> i32 {
    let mut i: u64;

    i = zero as u64;
    while i < TEST_SPMC_SYNC_SPINS && can_loop {
        if test_abort {
            return -EINTR;
        }
        if smp_load_acquire(&raw const owner_epoch) >= epoch {
            return 0;
        }
        i = i.wrapping_add(1);
    }

    test_abort = true;

    -ETIMEDOUT
}

#[no_mangle]
pub unsafe extern "C" fn spmc_quiesce_on_stealer(epoch: u64) -> i32 {
    let target: u64;
    let mut cur: u64;
    let mut i: u32;
    let mut err: i32 = -ETIMEDOUT;

    target = STEALER_EPOCH(epoch);
    i = zero as u32;
    while (i as u64) < TEST_SPMC_SYNC_SPINS && can_loop {
        if test_abort {
            err = -EINTR;
            break;
        }

        cur = smp_load_acquire(&raw const stealer_epoch);
        if cur > target {
            err = -EINVAL;
            test_abort = true;
            break;
        }

        if cur == target {
            return 0;
        }
        i = i.wrapping_add(1);
    }

    test_abort = true;

    err
}

unsafe fn spmc_update_stats(val: u64, owner: bool) -> i32 {
    let total: u64;

    total = expected_total;
    if val >= total || val >= TEST_SPMC_MAX_VALUES {
        test_abort = true;
        return -EINVAL;
    }

    if __sync_fetch_and_add(&raw mut seen[val as usize], 1) != 0 {
        test_abort = true;
        return -EINVAL;
    }

    __sync_fetch_and_add(&raw mut total_seen, 1);
    if owner {
        __sync_fetch_and_add(&raw mut pops, 1);
    } else {
        __sync_fetch_and_add(&raw mut steals, 1);
    }

    0
}

unsafe fn spmc_validate_owner_empty() -> i32 {
    let mut val: u64 = 0;
    let ret: i32;

    ret = spmc_owned_remove(spmc, &mut val);
    if ret != -ENOENT {
        test_abort = true;
        /* Change a 0 return value into -EINVAL. */
        return if ret != 0 { ret } else { -EINVAL };
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn spmc_validate_all_seen() -> i32 {
    let mut i: u64;
    let total: u64;

    total = expected_total;
    if total_seen != total {
        test_abort = true;
        return -EINVAL;
    }

    if pops.wrapping_add(steals) != total {
        test_abort = true;
        return -EINVAL;
    }

    i = zero as u64;
    while i < total && can_loop {
        if seen[(i % TEST_SPMC_MAX_VALUES) as usize] != 1 {
            test_abort = true;
            return -EINVAL;
        }
        i = i.wrapping_add(1);
    }

    0
}

/*
 * Single value benchmark. The owner adds an item then races with
 * the stealers for it. This way directly race between owner and
 * stealers on the same slot.
 */
const TEST_SPMC_SINGLEVAL_ITERS: u64 = 64;

#[no_mangle]
pub unsafe extern "C" fn spmc_singleval_tryconsume(expected: u64, steal: bool) -> i32 {
    let mut val: u64 = 0;
    let mut ret: i32;

    while can_loop {
        if steal {
            ret = spmc_steal(spmc, &mut val);
        } else {
            ret = spmc_owned_remove(spmc, &mut val);
        }

        /* Success. Update and validate. */
        if ret == 0 {
            if val != expected {
                return -EINVAL;
            }

            ret = spmc_update_stats(val, !steal);
            if ret != 0 {
                return ret;
            }

            return 0;
        }

        /*
         * If we got -ENOENT, the queue is empty
         * and we're good to go.
         */
        if ret != -EAGAIN {
            return if ret == -ENOENT { 0 } else { ret };
        }
    }

    /* Impossible. */
    -EINVAL
}

unsafe fn spmc_singleval_owner() -> i32 {
    let mut ret: i32;
    let mut i: u64;

    i = zero as u64;
    while i < TEST_SPMC_SINGLEVAL_ITERS && can_loop {
        ret = spmc_quiesce_on_stealer(i);
        if ret != 0 {
            test_abort = true;
            return -EINVAL;
        }

        ret = spmc_owned_add(spmc, i);
        if ret != 0 {
            test_abort = true;
            return -EINVAL;
        }

        __sync_fetch_and_add(&raw mut owner_epoch, 1);

        ret = spmc_singleval_tryconsume(i, false);
        if ret != 0 {
            test_abort = true;
            return -EINVAL;
        }

        ret = spmc_quiesce_on_stealer(i.wrapping_add(1));
        if ret != 0 {
            test_abort = true;
            return -EINVAL;
        }
        i = i.wrapping_add(1);
    }

    ret = spmc_validate_owner_empty();
    if ret != 0 {
        return ret;
    }

    spmc_validate_all_seen()
}

unsafe fn spmc_singleval_stealer() -> i32 {
    let mut ret: i32;
    let mut i: u64;

    i = zero as u64;
    while i < TEST_SPMC_SINGLEVAL_ITERS && can_loop {
        ret = spmc_quiesce_on_owner(i.wrapping_add(1));
        if ret != 0 {
            test_abort = true;
            return -EINVAL;
        }

        ret = spmc_singleval_tryconsume(i, true);
        if ret != 0 {
            test_abort = true;
            return -EINVAL;
        }

        __sync_fetch_and_add(&raw mut stealer_epoch, 1);
        i = i.wrapping_add(1);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_singleval__enabled() -> i32 {
    if spmc_tests_enabled() { 0 } else { -EOPNOTSUPP }
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_singleval__init() -> i32 {
    spmc_common_init(TEST_SPMC_SINGLEVAL_ITERS)
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_singleval__fini() -> i32 {
    spmc_common_fini()
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_singleval__0() -> i32 {
    spmc_singleval_owner()
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_singleval__1() -> i32 {
    spmc_singleval_stealer()
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_singleval__2() -> i32 {
    spmc_singleval_stealer()
}

/*
 * The resize test. Force a resize from the owner even while the stealers
 * are trying to consume. Then make sure the queue is still consistent
 * after the resize.
 *
 * The owner _doesn't_ consume from the queue. The test makes sure that
 * switching the array from underneath the stealers works.
 */

/* Force 2 resizes (since the rate of resize is logarithmic). */
const TEST_SPMC_RESIZE_ORDER: u64 = 2;
const TEST_SPMC_RESIZE_PREFILL: u64 = (SPMC_ARR_BASESZ << TEST_SPMC_RESIZE_ORDER) - 1;

/* */
const TEST_SPMC_RESIZE_TAIL: u64 = SPMC_ARR_BASESZ << TEST_SPMC_RESIZE_ORDER;
const TEST_SPMC_RESIZE_TOTAL: u64 = TEST_SPMC_RESIZE_PREFILL + TEST_SPMC_RESIZE_TAIL;

#[no_mangle]
pub unsafe extern "C" fn spmc_wait_for_stealers_to_start(target: u64) -> i32 {
    let mut i: u64;

    i = zero as u64;
    while i < TEST_SPMC_SYNC_SPINS && can_loop {
        if test_abort {
            return -EINTR;
        }
        if READ_ONCE(&raw const stealers_started) >= target {
            return 0;
        }
        i = i.wrapping_add(1);
    }

    test_abort = true;

    -ETIMEDOUT
}

#[no_mangle]
pub unsafe extern "C" fn spmc_waste_time() {
    let mut i: i32;
    let mut j: i32 = 0;

    i = zero as i32;
    while (i as u64) < TEST_SPMC_WASTE_ROUNDS && can_loop {
        /* Random computation. */
        WRITE_ONCE(&mut j, i.wrapping_mul(17).wrapping_add(23));
        i = i.wrapping_add(1);
    }
}

unsafe fn spmc_resize_owner() -> i32 {
    let mut resized: bool = false;
    let mut i: u64;
    let mut ret: i32;

    /* Get a head start vs the consumers. */
    i = zero as u64;
    while i < TEST_SPMC_RESIZE_PREFILL && can_loop {
        ret = spmc_owned_add(spmc, i);
        if ret != 0 {
            test_abort = true;
            return ret;
        }
        i = i.wrapping_add(1);
    }

    __sync_fetch_and_add(&raw mut owner_epoch, 1);

    /* Wait for stealers to start then start racing. */
    ret = spmc_wait_for_stealers_to_start(TEST_SPMC_STEALERS);
    if ret != 0 {
        return ret;
    }

    i = TEST_SPMC_RESIZE_PREFILL;
    while i < TEST_SPMC_RESIZE_TOTAL && can_loop {
        ret = spmc_owned_add(spmc, i);
        if ret != 0 {
            test_abort = true;
            return ret;
        }

        if (*(*spmc).cur).order > TEST_SPMC_RESIZE_ORDER {
            resized = true;
        }
        i = i.wrapping_add(1);
    }

    /* Did we get to resize while racing? */
    if !resized {
        test_abort = true;
        return -EINVAL;
    }

    /*
     * Wait for the stealers to drain and make sure
     * we didn't lose any items along the way.
     */
    __sync_fetch_and_add(&raw mut owner_epoch, 1);

    ret = spmc_quiesce_on_stealer(1);
    if ret != 0 {
        return ret;
    }

    ret = spmc_validate_owner_empty();
    if ret != 0 {
        return ret;
    }

    spmc_validate_all_seen()
}

unsafe fn spmc_resize_stealer() -> i32 {
    let mut owner_done: bool = false;
    let mut val: u64 = 0;
    let mut ret: i32;

    arena_subprog_init();

    ret = spmc_quiesce_on_owner(1);
    if ret != 0 {
        return ret;
    }

    __sync_fetch_and_add(&raw mut stealers_started, 1);

    while can_loop {
        spmc_waste_time();
        if test_abort {
            return -EINTR;
        }

        ret = spmc_steal(spmc, &mut val);
        if ret == 0 {
            ret = spmc_update_stats(val, false);
            if ret != 0 {
                return ret;
            }
            continue;
        }

        if ret == -EAGAIN {
            continue;
        }

        if ret == -ENOENT {
            if owner_done {
                break;
            }
            owner_done = owner_epoch >= 2;
            continue;
        }

        test_abort = true;
        return ret;
    }

    __sync_fetch_and_add(&raw mut stealer_epoch, 1);

    0
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_resize__enabled() -> i32 {
    if spmc_tests_enabled() { 0 } else { -EOPNOTSUPP }
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_resize__init() -> i32 {
    spmc_common_init(TEST_SPMC_RESIZE_TOTAL)
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_resize__fini() -> i32 {
    spmc_common_fini()
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_resize__0() -> i32 {
    spmc_resize_owner()
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_resize__1() -> i32 {
    spmc_resize_stealer()
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_resize__2() -> i32 {
    spmc_resize_stealer()
}

/*
 * The burst benchmark. The owner generates data all at once,
 * then waits for the stealers to steal half then starts removing
 * items until the queue empties. The owner also makes sure the
 * item order is not jumbled.
 */

const TEST_SPMC_BURST_ROUNDS: u64 = 4;
const TEST_SPMC_BURST_BURST: u64 = 64;
const TEST_SPMC_BURST_TOTAL: u64 = TEST_SPMC_BURST_ROUNDS * TEST_SPMC_BURST_BURST;
const TEST_SPMC_BURST_STEAL_TARGET: u64 = TEST_SPMC_BURST_BURST / 2;

unsafe fn spmc_wait_for_round_steals(target: u64) -> i32 {
    let mut i: u64;

    arena_subprog_init();

    i = zero as u64;
    while i < TEST_SPMC_SYNC_SPINS && can_loop {
        if test_abort {
            return -EINTR;
        }
        if round_steals >= target {
            return 0;
        }
        i = i.wrapping_add(1);
    }

    test_abort = true;

    -ETIMEDOUT
}

#[no_mangle]
pub unsafe extern "C" fn spmc_burst_owner_round(round: u64) -> i32 {
    let mut i: u64;
    let base: u64;
    let stolen: u64;
    let mut expected: u64;
    let mut val: u64 = 0;
    let mut ret: i32;

    base = round.wrapping_mul(TEST_SPMC_BURST_BURST);
    round_steals = 0;

    i = zero as u64;
    while i < TEST_SPMC_BURST_BURST && can_loop {
        ret = spmc_owned_add(spmc, base.wrapping_add(i));
        if ret != 0 {
            return ret;
        }
        i = i.wrapping_add(1);
    }

    __sync_fetch_and_add(&raw mut owner_epoch, 1);

    ret = spmc_wait_for_round_steals(TEST_SPMC_BURST_STEAL_TARGET);
    if ret == -EINTR || ret == -ETIMEDOUT {
        return ret;
    }

    __sync_fetch_and_add(&raw mut owner_epoch, 1);

    ret = spmc_quiesce_on_stealer(round.wrapping_add(1));
    if ret != 0 {
        return ret;
    }

    stolen = round_steals;
    if stolen > TEST_SPMC_BURST_BURST {
        return -EINVAL;
    }

    i = zero as u64;
    while i < TEST_SPMC_BURST_BURST.wrapping_sub(stolen) && can_loop {
        ret = spmc_owned_remove(spmc, &mut val);
        if ret != 0 {
            return ret;
        }

        expected = base
            .wrapping_add(TEST_SPMC_BURST_BURST)
            .wrapping_sub(1)
            .wrapping_sub(i);
        if val != expected {
            return -EINVAL;
        }

        ret = spmc_update_stats(val, true);
        if ret != 0 {
            test_abort = true;
            return -EINVAL;
        }
        i = i.wrapping_add(1);
    }

    ret = spmc_validate_owner_empty();
    if ret != 0 {
        return ret;
    }

    0
}

unsafe fn spmc_burst_owner() -> i32 {
    let mut round: u64;
    let mut ret: i32;

    arena_subprog_init();

    round = zero as u64;
    while round < TEST_SPMC_BURST_ROUNDS && can_loop {
        ret = spmc_burst_owner_round(round);
        if ret != 0 {
            test_abort = true;
            return -EINVAL;
        }
        round = round.wrapping_add(1);
    }

    spmc_validate_all_seen()
}

unsafe fn spmc_burst_stealer() -> i32 {
    let mut round: u64;
    let mut val: u64 = 0;
    let active_epoch: u64;
    let mut ret: i32;

    arena_subprog_init();

    round = zero as u64;
    while round < TEST_SPMC_BURST_ROUNDS && can_loop {
        active_epoch = round.wrapping_mul(2).wrapping_add(1);

        /*
         * Wait till the owner prefills the queue then
         * start stealing.
         */
        ret = spmc_quiesce_on_owner(active_epoch);
        if ret != 0 {
            return ret;
        }

        while owner_epoch == active_epoch && can_loop {
            if test_abort {
                return -EINTR;
            }

            ret = spmc_steal(spmc, &mut val);
            if ret == 0 {
                ret = spmc_update_stats(val, false);
                if ret != 0 {
                    return ret;
                }
                __sync_fetch_and_add(&raw mut round_steals, 1);
                continue;
            }
            if ret == -EAGAIN || ret == -ENOENT {
                continue;
            }

            test_abort = true;
            return ret;
        }

        __sync_fetch_and_add(&raw mut stealer_epoch, 1);
        round = round.wrapping_add(1);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_burst__enabled() -> i32 {
    if spmc_tests_enabled() { 0 } else { -EOPNOTSUPP }
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_burst__init() -> i32 {
    spmc_common_init(TEST_SPMC_BURST_TOTAL)
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_burst__fini() -> i32 {
    spmc_common_fini()
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_burst__0() -> i32 {
    spmc_burst_owner()
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_burst__1() -> i32 {
    spmc_burst_stealer()
}

#[no_mangle]
pub unsafe extern "C" fn parallel_test_spmc_burst__2() -> i32 {
    spmc_burst_stealer()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
