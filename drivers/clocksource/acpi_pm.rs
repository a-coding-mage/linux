// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/drivers/clocksource/acpi_pm.c
 *
 * This file contains the ACPI PM based clocksource.
 *
 * This code was largely moved from the i386 timer_pm.c file
 * which was (C) Dominik Brodowski <linux@brodo.de> 2003
 * and contained the following comments:
 *
 * Driver to use the Power Management Timer (PMTMR) available in some
 * southbridges as primary timing source for the Linux kernel.
 *
 * Based on parts of linux/drivers/acpi/hardware/hwtimer.c, timer_pit.c,
 * timer_hpet.c, and on Arjan van de Ven's implementation for 2.4.
 */

// Kernel dependencies supplied by other translation units.
extern "C" {
    fn inl(port: u32) -> u32;
    fn udelay(usecs: u32);
    fn clocksource_register_hz(cs: *mut clocksource, hz: u64) -> i32;
    fn mach_prepare_counter();
    fn mach_countup(count: *mut usize);
    fn kstrtouint(arg: *mut i8, base: u32, res: *mut u32) -> i32;
}

#[repr(C)]
pub struct clocksource {
    pub name: *const i8,
    pub rating: i32,
    pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    pub mask: u64,
    pub flags: u32,
    pub suspend: Option<unsafe extern "C" fn(*mut clocksource)>,
    pub resume: Option<unsafe extern "C" fn(*mut clocksource)>,
}

#[repr(C)]
pub struct pci_dev {
    pub revision: u8,
}

const ACPI_PM_MASK: u32 = 0x00ff_ffff;
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 0;
const CLOCK_SOURCE_CALIBRATED: u32 = 1 << 1;
const ENODEV: i32 = 19;
const EINVAL: i32 = 22;
const PMTMR_TICKS_PER_SEC: u64 = 3_579_545;

static mut suspend_resume_cb_data: *mut core::ffi::c_void = core::ptr::null_mut();
static mut suspend_resume_callback:
    Option<unsafe extern "C" fn(*mut core::ffi::c_void, bool)> = None;

/*
 * The I/O port the PMTMR resides at.
 * The location is detected during setup_arch(),
 * in arch/i386/kernel/acpi/boot.c
 */
#[no_mangle]
pub static mut pmtmr_ioport: u32 = 0;

#[inline]
unsafe fn read_pmtmr() -> u32 {
    /* mask the output to 24 bits */
    inl(pmtmr_ioport) & ACPI_PM_MASK
}

#[no_mangle]
pub unsafe extern "C" fn acpi_pm_read_verified() -> u32 {
    let mut v1: u32;
    let mut v2: u32;
    let mut v3: u32;

    /*
     * It has been reported that because of various broken
     * chipsets (ICH4, PIIX4 and PIIX4E) where the ACPI PM clock
     * source is not latched, you must read it multiple
     * times to ensure a safe value is read:
     */
    loop {
        v1 = read_pmtmr();
        v2 = read_pmtmr();
        v3 = read_pmtmr();
        if !((v1 > v2 && v1 < v3) || (v2 > v3 && v2 < v1)
            || (v3 > v1 && v3 < v2))
        {
            break;
        }
    }

    v2
}

#[no_mangle]
pub unsafe extern "C" fn acpi_pmtmr_register_suspend_resume_callback(
    cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, bool)>,
    data: *mut core::ffi::c_void,
) {
    suspend_resume_callback = cb;
    suspend_resume_cb_data = data;
}

#[no_mangle]
pub unsafe extern "C" fn acpi_pmtmr_unregister_suspend_resume_callback() {
    suspend_resume_callback = None;
    suspend_resume_cb_data = core::ptr::null_mut();
}

unsafe extern "C" fn acpi_pm_suspend(_cs: *mut clocksource) {
    if let Some(callback) = suspend_resume_callback {
        callback(suspend_resume_cb_data, true);
    }
}

unsafe extern "C" fn acpi_pm_resume(_cs: *mut clocksource) {
    if let Some(callback) = suspend_resume_callback {
        callback(suspend_resume_cb_data, false);
    }
}

unsafe extern "C" fn acpi_pm_read(_cs: *mut clocksource) -> u64 {
    read_pmtmr() as u64
}

static mut clocksource_acpi_pm: clocksource = clocksource {
    name: b"acpi_pm\0".as_ptr() as *const i8,
    rating: 200,
    read: Some(acpi_pm_read),
    mask: ACPI_PM_MASK as u64,
    flags: CLOCK_SOURCE_IS_CONTINUOUS | CLOCK_SOURCE_CALIBRATED,
    suspend: Some(acpi_pm_suspend),
    resume: Some(acpi_pm_resume),
};

// The CONFIG_PCI section is retained conditionally by the original build.
#[cfg(feature = "CONFIG_PCI")]
mod config_pci {
    use super::*;

    static mut acpi_pm_good: i32 = 0;

    unsafe extern "C" fn acpi_pm_read_slow(_cs: *mut clocksource) -> u64 {
        acpi_pm_read_verified() as u64
    }

    #[inline]
    unsafe fn acpi_pm_need_workaround() {
        clocksource_acpi_pm.read = Some(acpi_pm_read_slow);
        clocksource_acpi_pm.rating = 120;
    }

    /*
     * PIIX4 Errata:
     *
     * The power management timer may return improper results when read.
     * Although the timer value settles properly after incrementing,
     * while incrementing there is a 3 ns window every 69.8 ns where the
     * timer value is indeterminate (a 4.2% chance that the data will be
     * incorrect when read). As a result, the ACPI free running count up
     * timer specification is violated due to erroneous reads.
     */
    unsafe extern "C" fn acpi_pm_check_blacklist(dev: *mut pci_dev) {
        if acpi_pm_good != 0 {
            return;
        }

        /* the bug has been fixed in PIIX4M */
        if (*dev).revision < 3 {
            acpi_pm_need_workaround();
        }
    }

    unsafe extern "C" fn acpi_pm_check_graylist(_dev: *mut pci_dev) {
        if acpi_pm_good != 0 {
            return;
        }
        acpi_pm_need_workaround();
    }
}

#[cfg(not(target_arch = "x86_64"))]
const PMTMR_EXPECTED_RATE: u64 = 0; // CALIBRATE_LATCH and PIT_TICK_RATE are external build-time constants.

#[cfg(not(target_arch = "x86_64"))]
unsafe fn verify_pmtmr_rate() -> i32 {
    let mut value1: u64;
    let mut value2: u64;
    let mut count: usize = 0;
    mach_prepare_counter();
    value1 = acpi_pm_read(&raw mut clocksource_acpi_pm);
    mach_countup(&mut count);
    value2 = acpi_pm_read(&raw mut clocksource_acpi_pm);
    let delta = (value2.wrapping_sub(value1)) & ACPI_PM_MASK as u64;
    if delta < (PMTMR_EXPECTED_RATE * 19) / 20
        || delta > (PMTMR_EXPECTED_RATE * 21) / 20
    {
        return -1;
    }
    0
}

#[cfg(target_arch = "x86_64")]
unsafe fn verify_pmtmr_rate() -> i32 {
    0
}

/* Number of monotonicity checks to perform during initialization */
const ACPI_PM_MONOTONICITY_CHECKS: u32 = 10;
/* Number of reads we try to get two different values */
const ACPI_PM_READ_CHECKS: u32 = 10000;

unsafe extern "C" fn init_acpi_pm_clocksource() -> i32 {
    let mut value1: u64;
    let mut value2: u64;

    if pmtmr_ioport == 0 {
        return -ENODEV;
    }

    /* "verify" this timing source: */
    let mut j = 0;
    while j < ACPI_PM_MONOTONICITY_CHECKS {
        udelay(100 * j);
        value1 = (clocksource_acpi_pm.read.unwrap())(&raw mut clocksource_acpi_pm);
        let mut i = 0;
        while i < ACPI_PM_READ_CHECKS {
            value2 = (clocksource_acpi_pm.read.unwrap())(&raw mut clocksource_acpi_pm);
            if value2 == value1 {
                i += 1;
                continue;
            }
            if value2 > value1 {
                break;
            }
            if value2 < value1 && value2 < 0xFFF {
                break;
            }
            pmtmr_ioport = 0;
            return -EINVAL;
        }
        if i == ACPI_PM_READ_CHECKS {
            pmtmr_ioport = 0;
            return -ENODEV;
        }
        j += 1;
    }

    if verify_pmtmr_rate() != 0 {
        pmtmr_ioport = 0;
        return -ENODEV;
    }

    clocksource_register_hz(&raw mut clocksource_acpi_pm, PMTMR_TICKS_PER_SEC)
}

// fs_initcall(init_acpi_pm_clocksource);

/*
 * Allow an override of the IOPort. Stupid BIOSes do not tell us about
 * the PMTimer, but we might know where it is.
 */
unsafe extern "C" fn parse_pmtmr(arg: *mut i8) -> i32 {
    let mut base: u32 = 0;
    let ret = kstrtouint(arg, 16, &mut base);
    if ret != 0 {
        return 1;
    }

    pmtmr_ioport = base;
    1
}

// __setup("pmtmr=", parse_pmtmr);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
