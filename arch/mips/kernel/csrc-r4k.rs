/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007 by Ralf Baechle
 */

// Dependencies supplied by the surrounding kernel translation unit.

unsafe extern "C" {
    fn read_c0_count() -> u32;
    fn num_possible_cpus() -> u32;
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn clocksource_mark_unstable(cs: *mut clocksource);
    fn cpufreq_register_notifier(nb: *mut notifier_block, notifier: u32) -> i32;
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> i32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, hz: u32);
}

#[repr(C)]
struct clocksource {
    name: *const core::ffi::c_char,
    read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    mask: u64,
    flags: u32,
    rating: i32,
    vdso_clock_mode: u32,
}

#[repr(C)]
struct notifier_block {
    notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, u64, *mut core::ffi::c_void) -> i32>,
}

unsafe extern "C" {
    static mut cpu_has_counter: bool;
    static mut mips_hpt_frequency: u32;
}

const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 0;
const VDSO_CLOCKMODE_R4K: u32 = 1;
const CPUFREQ_POSTCHANGE: u64 = 0x0002;
const CPUFREQ_TRANSITION_NOTIFIER: u32 = 0;

unsafe extern "C" fn c0_hpt_read(_cs: *mut clocksource) -> u64 {
    read_c0_count() as u64
}

static mut clocksource_mips: clocksource = clocksource {
    name: b"MIPS\0".as_ptr() as *const core::ffi::c_char,
    read: Some(c0_hpt_read),
    mask: (1u64 << 32) - 1,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    rating: 0,
    vdso_clock_mode: 0,
};

#[allow(dead_code)]
unsafe extern "C" fn r4k_read_sched_clock() -> u64 {
    read_c0_count() as u64
}

#[inline]
unsafe fn rdhwr_count() -> u32 {
    let count: u32;
    core::arch::asm!(
        ".set push",
        ".set mips32r2",
        "rdhwr {count}, $2",
        ".set pop",
        count = out(reg) count,
    );
    count
}

unsafe fn rdhwr_count_usable() -> bool {
    let mut prev: u32 = rdhwr_count();

    /*
     * Older QEMUs have a broken implementation of RDHWR for the CP0 count
     * which always returns a constant value. Try to identify this and don't
     * use it in the VDSO if it is broken. This workaround can be removed
     * once the fix has been in QEMU stable for a reasonable amount of time.
     */
    for _i in 0..100 {
        let curr = rdhwr_count();
        if curr != prev {
            return true;
        }
        prev = curr;
    }

    pr_warn(b"Not using R4K clocksource in VDSO due to broken RDHWR\n\0".as_ptr() as *const core::ffi::c_char);
    false
}

#[inline]
unsafe fn count_can_be_sched_clock() -> bool {
    // CONFIG_CPU_FREQ is a build-time condition; preserve its source intent.
    if cfg!(feature = "CONFIG_CPU_FREQ") {
        return false;
    }

    if num_possible_cpus() > 1 && !cfg!(feature = "CONFIG_HAVE_UNSTABLE_SCHED_CLOCK") {
        return false;
    }

    true
}

// CONFIG_CPU_FREQ conditional section.
#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut r4k_clock_unstable: bool = false;

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn r4k_clocksource_unstable(reason: *mut core::ffi::c_char) {
    if r4k_clock_unstable {
        return;
    }
    r4k_clock_unstable = true;
    pr_info(b"R4K timer is unstable due to %s\n\0".as_ptr() as *const core::ffi::c_char, reason);
    clocksource_mark_unstable(&raw mut clocksource_mips);
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe extern "C" fn r4k_cpufreq_callback(
    _nb: *mut notifier_block,
    val: u64,
    _data: *mut core::ffi::c_void,
) -> i32 {
    if val == CPUFREQ_POSTCHANGE {
        r4k_clocksource_unstable(b"CPU frequency change\0".as_ptr() as *mut core::ffi::c_char);
    }
    0
}

#[cfg(feature = "CONFIG_CPU_FREQ")]
static mut r4k_cpufreq_notifier: notifier_block = notifier_block {
    notifier_call: Some(r4k_cpufreq_callback),
};

#[cfg(feature = "CONFIG_CPU_FREQ")]
unsafe fn r4k_register_cpufreq_notifier() -> i32 {
    cpufreq_register_notifier(&raw mut r4k_cpufreq_notifier, CPUFREQ_TRANSITION_NOTIFIER)
}

pub unsafe extern "C" fn init_r4k_clocksource() -> i32 {
    if !cpu_has_counter || mips_hpt_frequency == 0 {
        return -6; // -ENXIO
    }

    /* Calculate a somewhat reasonable rating value */
    clocksource_mips.rating = 200;
    let increment = (mips_hpt_frequency / 10_000_000).min(99);
    clocksource_mips.rating += increment as i32;

    /*
     * R2 onwards makes the count accessible to user mode so it can be used
     * by the VDSO (HWREna is configured by configure_hwrena()).
     */
    if cfg!(feature = "MIPS_R2_R6") && rdhwr_count_usable() {
        clocksource_mips.vdso_clock_mode = VDSO_CLOCKMODE_R4K;
    }

    clocksource_register_hz(&raw mut clocksource_mips, mips_hpt_frequency);

    if count_can_be_sched_clock() {
        sched_clock_register(r4k_read_sched_clock, 32, mips_hpt_frequency);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
