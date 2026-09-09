// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: hwtimer.c - ACPI Power Management Timer Interface
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// The declarations referenced below are supplied by the surrounding ACPI
// implementation. This module is omitted when ACPI_REDUCED_HARDWARE is set.

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_get_timer_resolution(resolution: *mut u32) -> acpi_status {
    if resolution.is_null() {
        return AE_BAD_PARAMETER;
    }

    if (acpi_gbl_FADT.flags & ACPI_FADT_32BIT_TIMER) == 0 {
        *resolution = 24;
    } else {
        *resolution = 32;
    }

    AE_OK
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_get_timer(ticks: *mut u32) -> acpi_status {
    let mut status: acpi_status;
    let mut timer_value: u64 = 0;

    if ticks.is_null() {
        return AE_BAD_PARAMETER;
    }

    // ACPI 5.0A: PM Timer is optional
    if acpi_gbl_FADT.xpm_timer_block.address == 0 {
        return AE_SUPPORT;
    }

    status = acpi_hw_read(&mut timer_value, &acpi_gbl_FADT.xpm_timer_block);
    if ACPI_SUCCESS(status) {
        // ACPI PM Timer is defined to be 32 bits (PM_TMR_LEN)
        *ticks = timer_value as u32;
    }

    status
}

#[cfg(not(ACPI_REDUCED_HARDWARE))]
pub unsafe fn acpi_get_timer_duration(
    start_ticks: u32,
    end_ticks: u32,
    time_elapsed: *mut u32,
) -> acpi_status {
    let status: acpi_status;
    let mut delta_ticks: u64;
    let mut quotient: u64 = 0;

    if time_elapsed.is_null() {
        return AE_BAD_PARAMETER;
    }

    // ACPI 5.0A: PM Timer is optional
    if acpi_gbl_FADT.xpm_timer_block.address == 0 {
        return AE_SUPPORT;
    }

    if start_ticks == end_ticks {
        *time_elapsed = 0;
        return AE_OK;
    }

    //
    // Compute Tick Delta:
    // Handle (max one) timer rollovers on 24-bit versus 32-bit timers.
    //
    delta_ticks = end_ticks as u64;
    if start_ticks > end_ticks {
        if (acpi_gbl_FADT.flags & ACPI_FADT_32BIT_TIMER) == 0 {
            // 24-bit Timer
            delta_ticks |= (1u64) << 24;
        } else {
            // 32-bit Timer
            delta_ticks |= (1u64) << 32;
        }
    }
    delta_ticks = delta_ticks.wrapping_sub(start_ticks as u64);

    //
    // Compute Duration (Requires a 64-bit multiply and divide):
    //
    // time_elapsed (microseconds) =
    //  (delta_ticks * ACPI_USEC_PER_SEC) / ACPI_PM_TIMER_FREQUENCY;
    //
    status = acpi_ut_short_divide(
        delta_ticks.wrapping_mul(ACPI_USEC_PER_SEC),
        ACPI_PM_TIMER_FREQUENCY,
        &mut quotient,
        core::ptr::null_mut(),
    );

    *time_elapsed = quotient as u32;
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
