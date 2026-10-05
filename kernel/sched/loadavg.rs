// SPDX-License-Identifier: GPL-2.0
/*
 * kernel/sched/loadavg.c
 *
 * This file contains the magic bits required to compute the global loadavg
 * figure. Its a silly number but people think its important. We go through
 * great pains to make it work on big machines and tickless kernels.
 */

// Existing Rust continuation.
// Native source baseline: 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
// Native headers own every layout and constant. Shared native state stays behind
// primitive accessors so this code never creates Rust references to that state.
#[cfg(CONFIG_RUST_SCHED_LOADAVG)]
compile_error!("SOURCE ONLY HOLD: scheduler loadavg is not admitted");

use kernel::ffi::{c_int, c_long, c_uint, c_ulong};
use kernel::bindings::sched_loadavg_native::*;

/// Read the three native load estimates.
///
/// # Safety
/// `loads` must point to three writable unsigned longs, and `shift` must be a
/// valid native unsigned-long shift count. Estimates have no snapshot lock.
#[no_mangle]
pub unsafe extern "C" fn get_avenrun(loads: *mut c_ulong, offset: c_ulong, shift: c_int) {
    // SAFETY: The caller supplies the output range and native shift precondition.
    unsafe {
        *loads.add(0) = lupos_loadavg_avenrun_read(0).wrapping_add(offset) << shift;
        *loads.add(1) = lupos_loadavg_avenrun_read(1).wrapping_add(offset) << shift;
        *loads.add(2) = lupos_loadavg_avenrun_read(2).wrapping_add(offset) << shift;
    }
}

/// Fold the runqueue's active-count change.
///
/// # Safety
/// The caller must supply a live native runqueue and the native scheduler
/// serialization required for its runnable and uninterruptible counts.
#[no_mangle]
pub unsafe extern "C" fn calc_load_fold_active(this_rq: *mut rq, adjust: c_long) -> c_long {
    // SAFETY: All runqueue access is through configured native field accessors;
    // the caller supplies the lifetime and scheduler synchronization contract.
    unsafe {
        let mut nr_active: c_long;
        let mut delta: c_long = 0;

        nr_active = (lupos_loadavg_rq_nr_running(this_rq) as c_long).wrapping_sub(adjust);
        nr_active = nr_active.wrapping_add(lupos_loadavg_rq_nr_uninterruptible(this_rq) as c_long);

        if nr_active != lupos_loadavg_rq_active_read(this_rq) {
            delta = nr_active.wrapping_sub(lupos_loadavg_rq_active_read(this_rq));
            lupos_loadavg_rq_active_write(this_rq, nr_active);
        }

        delta
    }
}

fn fixed_power_int(mut x: c_ulong, frac_bits: c_uint, mut n: c_uint) -> c_ulong {
    let mut result = (1 as c_ulong) << frac_bits;

    if n != 0 {
        loop {
            if n & 1 != 0 {
                result = result.wrapping_mul(x);
                result = result.wrapping_add((1 as c_ulong) << (frac_bits - 1));
                result >>= frac_bits;
            }
            n >>= 1;
            if n == 0 {
                break;
            }
            x = x.wrapping_mul(x);
            x = x.wrapping_add((1 as c_ulong) << (frac_bits - 1));
            x >>= frac_bits;
        }
    }

    result
}

/// Apply the native load-decay factor for `n` intervals.
///
/// # Safety
/// Arguments must satisfy the native fixed-point load-average API contract.
#[no_mangle]
pub unsafe extern "C" fn calc_load_n(
    load: c_ulong,
    exp: c_ulong,
    active: c_ulong,
    n: c_uint,
) -> c_ulong {
    // SAFETY: The header-derived precision is the same as native calc_load_n;
    // the leaf invokes the native header's inline fixed-point rounding primitive.
    unsafe {
        lupos_loadavg_calc_load(
            load,
            fixed_power_int(exp, LUPOS_LOADAVG_FSHIFT as c_uint, n),
            active,
        )
    }
}

#[cfg(CONFIG_NO_HZ_COMMON)]
mod no_hz {
    use super::*;

    unsafe fn calc_load_write_idx() -> c_int {
        // SAFETY: Native accessors preserve the index-read/barrier/update-read
        // sequence paired with calc_global_nohz's write barrier and index flip.
        unsafe {
            let mut idx = lupos_loadavg_nohz_idx_read();
            lupos_loadavg_read_barrier();
            if !lupos_loadavg_time_before(
                lupos_loadavg_jiffies(),
                lupos_loadavg_update_read_once(),
            ) {
                idx = idx.wrapping_add(1);
            }
            idx & 1
        }
    }

    unsafe fn calc_load_read_idx() -> c_int {
        // SAFETY: The native global timer context serializes index updates.
        unsafe { lupos_loadavg_nohz_idx_read() & 1 }
    }

    unsafe fn calc_load_nohz_fold(rq: *mut rq) {
        // SAFETY: The nohz caller supplies a live, appropriately serialized rq;
        // the masked index addresses one of the two native atomic accumulators.
        unsafe {
            let delta = calc_load_fold_active(rq, 0);
            if delta != 0 {
                let idx = calc_load_write_idx();
                lupos_loadavg_nohz_add(idx as c_uint, delta);
            }
        }
    }

    /// Fold the current CPU before entering NO_HZ accounting.
    ///
    /// # Safety
    /// The caller must satisfy the native nohz entry and per-CPU context rules.
    #[no_mangle]
    pub unsafe extern "C" fn calc_load_nohz_start() {
        // SAFETY: The caller pins the current native runqueue as required.
        unsafe { calc_load_nohz_fold(lupos_loadavg_this_rq()) }
    }

    /// Fold a remote runqueue between its nohz start and stop calls.
    ///
    /// # Safety
    /// `rq` must be live and held under the native remote nohz serialization.
    #[no_mangle]
    pub unsafe extern "C" fn calc_load_nohz_remote(rq: *mut rq) {
        // SAFETY: The caller satisfies the native remote runqueue contract.
        unsafe { calc_load_nohz_fold(rq) }
    }

    /// Synchronize the current CPU's sampling window after NO_HZ.
    ///
    /// # Safety
    /// The caller must satisfy the native nohz exit and per-CPU context rules.
    #[no_mangle]
    pub unsafe extern "C" fn calc_load_nohz_stop() {
        // SAFETY: Native nohz exit pins and serializes this runqueue; the global
        // update read retains native READ_ONCE semantics through the leaf.
        unsafe {
            let this_rq = lupos_loadavg_this_rq();
            lupos_loadavg_rq_update_write(this_rq, lupos_loadavg_update_read_once());
            if lupos_loadavg_time_before(
                lupos_loadavg_jiffies(),
                lupos_loadavg_rq_update_read(this_rq),
            ) {
                return;
            }
            if lupos_loadavg_time_before(
                lupos_loadavg_jiffies(),
                lupos_loadavg_rq_update_read(this_rq).wrapping_add(10),
            ) {
                lupos_loadavg_rq_update_write(
                    this_rq,
                    lupos_loadavg_rq_update_read(this_rq)
                        .wrapping_add(LUPOS_LOADAVG_LOAD_FREQ as c_ulong),
                );
            }
        }
    }

    unsafe fn calc_load_nohz_read() -> c_long {
        // SAFETY: The masked read index selects a native atomic accumulator;
        // atomic exchange retains the original native ordering.
        unsafe {
            let idx = calc_load_read_idx() as c_uint;
            if lupos_loadavg_nohz_read(idx) != 0 {
                lupos_loadavg_nohz_xchg(idx, 0)
            } else {
                0
            }
        }
    }

    pub(super) unsafe fn calc_global_nohz() {
        // SAFETY: Global timer serialization supplies the single writer;
        // native leaves preserve READ_ONCE/WRITE_ONCE and the publication barrier.
        unsafe {
            let sample_window = lupos_loadavg_update_read_once();
            if !lupos_loadavg_time_before(
                lupos_loadavg_jiffies(), sample_window.wrapping_add(10),
            ) {
                let delta = lupos_loadavg_jiffies()
                    .wrapping_sub(sample_window).wrapping_sub(10) as c_long;
                let n = (1 as c_long).wrapping_add(delta / (LUPOS_LOADAVG_LOAD_FREQ as c_long));
                let mut active = lupos_loadavg_tasks_read();
                active = if active > 0 {
                    active.wrapping_mul(LUPOS_LOADAVG_FIXED_1 as c_long)
                } else {
                    0
                };
                lupos_loadavg_avenrun_write(0, calc_load_n(
                    lupos_loadavg_avenrun_read(0), LUPOS_LOADAVG_EXP_1 as c_ulong,
                    active as c_ulong, n as c_uint,
                ));
                lupos_loadavg_avenrun_write(1, calc_load_n(
                    lupos_loadavg_avenrun_read(1), LUPOS_LOADAVG_EXP_5 as c_ulong,
                    active as c_ulong, n as c_uint,
                ));
                lupos_loadavg_avenrun_write(2, calc_load_n(
                    lupos_loadavg_avenrun_read(2), LUPOS_LOADAVG_EXP_15 as c_ulong,
                    active as c_ulong, n as c_uint,
                ));
                lupos_loadavg_update_write_once(sample_window.wrapping_add(
                    (n as c_ulong).wrapping_mul(LUPOS_LOADAVG_LOAD_FREQ as c_ulong),
                ));
            }
            lupos_loadavg_write_barrier();
            lupos_loadavg_nohz_idx_write(lupos_loadavg_nohz_idx_read().wrapping_add(1));
        }
    }

    pub(super) unsafe fn calc_load_nohz_delta() -> c_long {
        // SAFETY: The global timer caller supplies the native load-update context.
        unsafe { calc_load_nohz_read() }
    }
}

#[cfg(not(CONFIG_NO_HZ_COMMON))]
mod no_hz {
    use super::*;
    // These are the exact !CONFIG_NO_HZ_COMMON branches in native loadavg.c.
    pub(super) unsafe fn calc_global_nohz() {}
    pub(super) unsafe fn calc_load_nohz_delta() -> c_long { 0 }
}

#[cfg(CONFIG_NO_HZ_COMMON)]
pub use no_hz::{calc_load_nohz_remote, calc_load_nohz_start, calc_load_nohz_stop};

/// Update the global load-average estimates from the global timer.
///
/// # Safety
/// The caller must hold the native global timer/load-update serialization.
#[no_mangle]
pub unsafe extern "C" fn calc_global_load() {
    // SAFETY: The caller serializes global writes; native leaves retain atomic
    // folding and READ_ONCE/WRITE_ONCE accesses without Rust shared references.
    unsafe {
        let sample_window = lupos_loadavg_update_read_once();
        if lupos_loadavg_time_before(lupos_loadavg_jiffies(), sample_window.wrapping_add(10)) {
            return;
        }
        let delta = no_hz::calc_load_nohz_delta();
        if delta != 0 {
            lupos_loadavg_tasks_add(delta);
        }
        let mut active = lupos_loadavg_tasks_read();
        active = if active > 0 {
            active.wrapping_mul(LUPOS_LOADAVG_FIXED_1 as c_long)
        } else {
            0
        };
        lupos_loadavg_avenrun_write(0, lupos_loadavg_calc_load(
            lupos_loadavg_avenrun_read(0), LUPOS_LOADAVG_EXP_1 as c_ulong, active as c_ulong,
        ));
        lupos_loadavg_avenrun_write(1, lupos_loadavg_calc_load(
            lupos_loadavg_avenrun_read(1), LUPOS_LOADAVG_EXP_5 as c_ulong, active as c_ulong,
        ));
        lupos_loadavg_avenrun_write(2, lupos_loadavg_calc_load(
            lupos_loadavg_avenrun_read(2), LUPOS_LOADAVG_EXP_15 as c_ulong, active as c_ulong,
        ));
        lupos_loadavg_update_write_once(sample_window.wrapping_add(LUPOS_LOADAVG_LOAD_FREQ as c_ulong));
        no_hz::calc_global_nohz();
    }
}

/// Fold one CPU's active count from its scheduler tick.
///
/// # Safety
/// `this_rq` must be live and serialized as required by the native scheduler tick.
#[no_mangle]
pub unsafe extern "C" fn calc_global_load_tick(this_rq: *mut rq) {
    // SAFETY: The native tick caller owns this runqueue's load-accounting update.
    unsafe {
        if lupos_loadavg_time_before(
            lupos_loadavg_jiffies(), lupos_loadavg_rq_update_read(this_rq),
        ) {
            return;
        }
        let delta = calc_load_fold_active(this_rq, 0);
        if delta != 0 {
            lupos_loadavg_tasks_add(delta);
        }
        lupos_loadavg_rq_update_write(
            this_rq,
            lupos_loadavg_rq_update_read(this_rq).wrapping_add(LUPOS_LOADAVG_LOAD_FREQ as c_ulong),
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
