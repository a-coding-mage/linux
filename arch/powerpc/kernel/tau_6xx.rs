// SPDX-License-Identifier: GPL-2.0
/*
 * temp.c	Thermal management for cpu's with Thermal Assist Units
 *
 * Written by Troy Benjegerdes <hozer@drgw.net>
 *
 * TODO:
 * dynamic power management to limit peak CPU temp (using ICTC)
 * calibration???
 *
 * Silly, crazy ideas: use cpu load (from scheduler) and ICTC to extend battery
 * life in portables, and add a 'performance/watt' metric somewhere in /proc
 */

// Linux and PowerPC definitions supplied by the surrounding kernel translation.

#[repr(C)]
struct TauTemp {
    interrupts: i32,
    low: u8,
    high: u8,
    grew: u8,
}

static mut TAU: [TauTemp; NR_CPUS as usize] = [TauTemp {
    interrupts: 0,
    low: 0,
    high: 0,
    grew: 0,
}; NR_CPUS as usize];

static mut tau_int_enable: bool = false;

// Configurable values for the threshold step and window expansion.
const STEP_SIZE: u8 = 2;
const WINDOW_EXPAND: u8 = 1;
// Configurable values for shrinking the window.
const SHRINK_TIMER: u32 = 2000;
const MIN_WINDOW: u8 = 2;

unsafe fn set_thresholds(cpu: usize) {
    let maybe_tie: u32 = if tau_int_enable { THRM1_TIE } else { 0 };

    // setup THRM1, threshold, valid bit, interrupt when below threshold
    mtspr(SPRN_THRM1, THRM1_THRES(TAU[cpu].low) | THRM1_V | maybe_tie | THRM1_TID);

    // setup THRM2, threshold, valid bit, interrupt when above threshold
    mtspr(SPRN_THRM2, THRM1_THRES(TAU[cpu].high) | THRM1_V | maybe_tie);
}

unsafe fn TAUupdate(cpu: usize) {
    let bits: u32 = THRM1_TIV | THRM1_TIN | THRM1_V;

    // if both thresholds are crossed, the step_sizes cancel out
    // and the window winds up getting expanded twice.
    let mut thrm = mfspr(SPRN_THRM1);
    if (thrm & bits) == bits {
        mtspr(SPRN_THRM1, 0);

        if TAU[cpu].low >= STEP_SIZE {
            TAU[cpu].low -= STEP_SIZE;
            TAU[cpu].high -= STEP_SIZE - WINDOW_EXPAND;
        }
        TAU[cpu].grew = 1;
        pr_debug!("{}: low threshold crossed\n", "TAUupdate");
    }
    thrm = mfspr(SPRN_THRM2);
    if (thrm & bits) == bits {
        mtspr(SPRN_THRM2, 0);

        if TAU[cpu].high <= 127 - STEP_SIZE {
            TAU[cpu].low += STEP_SIZE - WINDOW_EXPAND;
            TAU[cpu].high += STEP_SIZE;
        }
        TAU[cpu].grew = 1;
        pr_debug!("{}: high threshold crossed\n", "TAUupdate");
    }
}

#[cfg(CONFIG_TAU_INT)]
unsafe extern "C" fn TAUException() {
    let cpu = smp_processor_id() as usize;

    TAU[cpu].interrupts += 1;
    TAUupdate(cpu);
}

unsafe extern "C" fn tau_timeout(_info: *mut core::ffi::c_void) {
    let cpu = smp_processor_id() as usize;
    let size: u8;
    let mut shrink: u8;

    if !tau_int_enable {
        TAUupdate(cpu);
    }

    // Stop thermal sensor comparisons and interrupts
    mtspr(SPRN_THRM3, 0);

    size = TAU[cpu].high - TAU[cpu].low;
    if size > MIN_WINDOW && TAU[cpu].grew == 0 {
        // do an exponential shrink of half the amount currently over size
        shrink = (2 + size - MIN_WINDOW) / 4;
        if shrink != 0 {
            TAU[cpu].low += shrink;
            TAU[cpu].high -= shrink;
        } else {
            // size must have been min_window + 1
            TAU[cpu].low += 1;
            if TAU[cpu].high - TAU[cpu].low != MIN_WINDOW {
                printk!(KERN_ERR, "temp.c: line %d, logic error\n", 125);
            }
        }
    }

    TAU[cpu].grew = 0;
    set_thresholds(cpu);

    // Restart thermal sensor comparisons and interrupts.
    // The PowerPC 740 and PowerPC 750 Microprocessor Datasheet recommends
    // that the maximum value be set in THRM3 under all conditions.
    mtspr(SPRN_THRM3, THRM3_SITV(0x1fff) | THRM3_E);
}

static mut tau_workq: *mut workqueue_struct = core::ptr::null_mut();

unsafe extern "C" fn tau_work_func(work: *mut work_struct) {
    msleep(SHRINK_TIMER);
    on_each_cpu(tau_timeout, core::ptr::null_mut(), 0);
    // schedule ourselves to be run again
    queue_work(tau_workq, work);
}

static mut tau_work: work_struct = DECLARE_WORK!(tau_work_func);

// setup the TAU: THRM1 is the lower bound and THRM2 is the upper bound.
// Start off at zero.
pub static mut tau_initialized: i32 = 0;

unsafe extern "C" fn TAU_init_smp(_info: *mut core::ffi::c_void) {
    let cpu = smp_processor_id() as usize;

    // set these to a reasonable value and let the timer shrink the window
    TAU[cpu].low = 5;
    TAU[cpu].high = 120;
    set_thresholds(cpu);
}

unsafe extern "C" fn TAU_init() -> i32 {
    // We assume in SMP that if one CPU has TAU support, they all have it.
    if !cpu_has_feature(CPU_FTR_TAU) {
        printk!("Thermal assist unit not available\n");
        tau_initialized = 0;
        return 1;
    }

    tau_int_enable = IS_ENABLED!(CONFIG_TAU_INT)
        && strcmp(cur_cpu_spec.platform, "ppc750") == 0;

    tau_workq = alloc_ordered_workqueue!("tau", 0);
    if tau_workq.is_null() {
        return -ENOMEM;
    }

    on_each_cpu(TAU_init_smp, core::ptr::null_mut(), 0);
    queue_work(tau_workq, &mut tau_work);

    pr_info!(
        "Thermal assist unit using {}, shrink_timer: {} ms\n",
        if tau_int_enable { "interrupts" } else { "workqueue" },
        SHRINK_TIMER
    );
    tau_initialized = 1;
    0
}

__initcall!(TAU_init);

// return current temp
pub unsafe fn cpu_temp_both(cpu: usize) -> u32 {
    ((TAU[cpu].high as u32) << 16) | TAU[cpu].low as u32
}

pub unsafe fn cpu_temp(cpu: usize) -> u32 {
    ((TAU[cpu].high as u32 + TAU[cpu].low as u32) / 2)
}

pub unsafe fn tau_interrupts(cpu: usize) -> u32 {
    TAU[cpu].interrupts as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
