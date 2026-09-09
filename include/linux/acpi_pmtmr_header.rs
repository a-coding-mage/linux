/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation: clocksource mask.

/* Number of PMTMR ticks expected during calibration run */
pub const PMTMR_TICKS_PER_SEC: u32 = 3_579_545;

/* limit it to 24 bits */
pub const ACPI_PM_MASK: u32 = (1u32 << 24) - 1;

/* Overrun value */
pub const ACPI_PM_OVRRUN: u32 = 1u32 << 24;

// Corresponds to CONFIG_X86_PM_TIMER.
#[cfg(CONFIG_X86_PM_TIMER)]
extern "C" {
    pub fn acpi_pm_read_verified() -> u32;
    pub static mut pmtmr_ioport: u32;
}

#[cfg(CONFIG_X86_PM_TIMER)]
#[inline]
pub unsafe fn acpi_pm_read_early() -> u32 {
    if pmtmr_ioport == 0 {
        return 0;
    }
    /* mask the output to 24 bits */
    acpi_pm_read_verified() & ACPI_PM_MASK
}

/**
 * acpi_pmtmr_register_suspend_resume_callback - Register callback for
 * suspend and resume event
 *
 * @cb: Callback triggered on suspend and resume
 * @data: Data passed with the callback
 */
#[cfg(CONFIG_X86_PM_TIMER)]
extern "C" {
    pub fn acpi_pmtmr_register_suspend_resume_callback(
        cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, bool)>,
        data: *mut core::ffi::c_void,
    );
}

/**
 * acpi_pmtmr_unregister_suspend_resume_callback - Remove registered callback
 * for suspend and resume event
 */
#[cfg(CONFIG_X86_PM_TIMER)]
extern "C" {
    pub fn acpi_pmtmr_unregister_suspend_resume_callback();
}

#[cfg(not(CONFIG_X86_PM_TIMER))]
#[inline]
pub fn acpi_pm_read_early() -> u32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
