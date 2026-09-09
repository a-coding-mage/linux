// SPDX-License-Identifier: GPL-2.0
/*
 * Count register synchronisation.
 *
 * Derived from arch/x86/kernel/tsc_sync.c
 * Copyright (C) 2006, Red Hat, Inc., Ingo Molnar
 */

// Linux and MIPS headers supplied by the surrounding translation unit.

const COUNTON: u32 = 100;
const NR_LOOPS: i32 = 3;
const LOOP_TIMEOUT: u32 = 20;

/*
 * Entry/exit counters that make sure that both CPUs
 * run the measurement code at once:
 */
static mut start_count: atomic_t = atomic_t::new();
static mut stop_count: atomic_t = atomic_t::new();
static mut test_runs: atomic_t = atomic_t::new();

/*
 * We use a raw spinlock in this exceptional case, because
 * we want to have the fastest, inlined, non-debug version
 * of a critical section, to be able to prove counter time-warps:
 */
static mut sync_lock: arch_spinlock_t = __ARCH_SPIN_LOCK_UNLOCKED;

static mut last_counter: u32 = 0;
static mut max_warp: u32 = 0;
static mut nr_warps: i32 = 0;
static mut random_warps: i32 = 0;

/* Counter warp measurement loop running on both CPUs. */
unsafe fn check_counter_warp() -> u32 {
    let start: u32 = read_c0_count();
    let end: u32 = start.wrapping_add(
        (mips_hpt_frequency as u32 / 1000).wrapping_mul(LOOP_TIMEOUT),
    );
    let mut now: u32 = 0;
    let mut cur_max_warp: u32 = 0;
    let mut cur_warps: i32 = 0;
    let mut i: i32 = 0;

    loop {
        /* Take the global lock and update the previous counter timestamp. */
        arch_spin_lock(&mut sync_lock);
        let prev = last_counter;
        now = read_c0_count();
        last_counter = now;
        arch_spin_unlock(&mut sync_lock);

        /* Be nice periodically and check whether measurement is done. */
        if (i & 7) == 0 {
            if now > end || i > 10_000_000 {
                break;
            }
            cpu_relax();
            touch_nmi_watchdog();
        }

        /* Check whether the counter went backwards. */
        if prev > now {
            arch_spin_lock(&mut sync_lock);
            max_warp = core::cmp::max(max_warp, prev - now);
            cur_max_warp = max_warp;
            /* Check whether this bounces back and forth. */
            if cur_warps != nr_warps {
                random_warps += 1;
            }
            nr_warps += 1;
            cur_warps = nr_warps;
            arch_spin_unlock(&mut sync_lock);
        }
        i += 1;
    }

    WARN(
        now.wrapping_sub(start) == 0,
        "Warning: zero counter calibration delta: %d [max: %d]\n",
        now.wrapping_sub(start),
        end.wrapping_sub(start),
    );
    cur_max_warp
}

/* The freshly booted CPU initiates this via an async SMP function call. */
unsafe fn check_counter_sync_source(__cpu: *mut core::ffi::c_void) {
    let cpu = __cpu as usize as u32;
    let cpus: i32 = 2;

    atomic_set(&mut test_runs, NR_LOOPS);
    'retry: loop {
        while atomic_read(&start_count) != cpus - 1 {
            cpu_relax();
        }
        atomic_inc(&mut start_count);
        check_counter_warp();
        while atomic_read(&stop_count) != cpus - 1 {
            cpu_relax();
        }

        if nr_warps == 0 {
            atomic_set(&mut test_runs, 0);
            pr_info("Counter synchronization [CPU#%d -> CPU#%u]: passed\n", smp_processor_id(), cpu);
        } else if atomic_dec_and_test(&mut test_runs) || random_warps != 0 {
            atomic_set(&mut test_runs, 0);
            pr_info("Counter synchronization [CPU#%d -> CPU#%u]:\n", smp_processor_id(), cpu);
            pr_info("Measured %d cycles counter warp between CPUs", max_warp);
            if random_warps != 0 {
                pr_warn("Counter warped randomly between CPUs\n");
            }
        }

        atomic_set(&mut start_count, 0);
        random_warps = 0;
        nr_warps = 0;
        max_warp = 0;
        last_counter = 0;
        atomic_inc(&mut stop_count);

        if atomic_read(&test_runs) > 0 {
            continue 'retry;
        }
        break;
    }
}

/* Freshly booted CPUs call into this. */
pub unsafe fn synchronise_count_slave(cpu: i32) {
    let mut cur_max_warp: u32;
    let gbl_max_warp: u32;
    let mut count: u32;
    let cpus: i32 = 2;

    if !cpu_has_counter || mips_hpt_frequency == 0 {
        return;
    }

    smp_call_function_single(
        cpumask_first(cpu_online_mask),
        check_counter_sync_source,
        cpu as usize as *mut core::ffi::c_void,
        0,
    );

    'retry: loop {
        atomic_inc(&mut start_count);
        while atomic_read(&start_count) != cpus {
            cpu_relax();
        }

        cur_max_warp = check_counter_warp();
        gbl_max_warp = max_warp;
        atomic_inc(&mut stop_count);
        while atomic_read(&stop_count) != cpus {
            cpu_relax();
        }
        atomic_set(&mut stop_count, 0);

        if atomic_read(&test_runs) == 0 {
            write_c0_compare(read_c0_count().wrapping_add(COUNTON));
            return;
        }

        if cur_max_warp == 0 {
            cur_max_warp = 0u32.wrapping_sub(gbl_max_warp);
        }
        count = read_c0_count();
        count = count.wrapping_add(cur_max_warp);
        write_c0_count(count);
        pr_debug("Counter compensate: CPU%u observed %d warp\n", cpu, cur_max_warp);
        continue 'retry;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
