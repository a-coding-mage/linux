// SPDX-License-Identifier: GPL-2.0-only
/*
 * test module to check whether the TSC-based delay routine continues
 * to work properly after cpufreq transitions. Needs ACPI to work
 * properly.
 *
 * Based partly on the Power Management Timer (PMTMR) code to be found
 * in arch/i386/kernel/timers/timer_pm.c on recent 2.6. kernels, especially
 * code written by John Stultz. The read_pmtmr function was copied verbatim
 * from that file.
 *
 * (C) 2004 Dominik Brodowski
 *
 * To use:
 * 1.) pass clock=tsc to the kernel on your bootloader
 * 2.) modprobe this module (it'll fail)
 * 3.) change CPU frequency
 * 4.) modprobe this module again
 * 5.) if the third value, "diff_pmtmr", changes between 2. and 4., the
 *     TSC-based delay routine on the Linux kernel does not correctly
 *     handle the cpufreq transition. Please report this to
 *     linux-pm@vger.kernel.org
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong};

type u32 = u32;
type u64 = u64;

const FADT2_REVISION_ID: u8 = 3;
const ACPI_ADR_SPACE_SYSTEM_IO: u8 = 1;
const ENODEV: c_int = 19;

#[repr(C)]
pub struct acpi_table_header {
    pub signature: [c_char; 4],
    pub length: u32,
    pub revision: u8,
}

#[repr(C)]
pub struct acpi_generic_address {
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_width: u8,
    pub address: u64,
}

#[repr(C)]
pub struct acpi_table_fadt {
    pub header: acpi_table_header,
    /*
     * The complete Linux ACPI FADT layout is supplied by external headers.
     * This source-level translation names only fields used by this file.
     */
    pub pm_timer_block: c_uint,
    pub xpm_timer_block: acpi_generic_address,
}

unsafe extern "C" {
    static mut acpi_gbl_FADT: acpi_table_fadt;

    fn inl(port: c_uint) -> u32;
    fn rdtsc() -> u64;
    fn mdelay(msecs: c_ulonglong);
    fn printk(fmt: *const c_char, ...) -> c_int;
}

static mut pm_tmr_ioport: c_int = 0;

/*helper function to safely read acpi pm timesource*/
unsafe fn read_pmtmr() -> u32 {
    let mut v1: u32 = 0;
    let mut v2: u32 = 0;
    let mut v3: u32 = 0;

    /* It has been reported that because of various broken
     * chipsets (ICH4, PIIX4 and PIIX4E) where the ACPI PM time
     * source is not latched, so you must read it multiple
     * times to insure a safe value is read.
     */
    loop {
        v1 = inl(pm_tmr_ioport as c_uint);
        v2 = inl(pm_tmr_ioport as c_uint);
        v3 = inl(pm_tmr_ioport as c_uint);

        if !((v1 > v2 && v1 < v3) || (v2 > v3 && v2 < v1) || (v3 > v1 && v3 < v2)) {
            break;
        }
    }

    /* mask the output to 24 bits */
    v2 & 0xFFFFFF
}

unsafe fn cpufreq_test_tsc() -> c_int {
    let mut now: u32;
    let mut then: u32;
    let mut diff: u32;
    let mut now_tsc: u64;
    let mut then_tsc: u64;
    let mut diff_tsc: u64;
    let mut i: c_int;

    /* the following code snipped is copied from arch/x86/kernel/acpi/boot.c
       of Linux v2.6.25. */

    /* detect the location of the ACPI PM Timer */
    if acpi_gbl_FADT.header.revision >= FADT2_REVISION_ID {
        /* FADT rev. 2 */
        if acpi_gbl_FADT.xpm_timer_block.space_id != ACPI_ADR_SPACE_SYSTEM_IO {
            return 0;
        }

        pm_tmr_ioport = acpi_gbl_FADT.xpm_timer_block.address as c_int;
        /*
         * "X" fields are optional extensions to the original V1.0
         * fields, so we must selectively expand V1.0 fields if the
         * corresponding X field is zero.
         */
        if pm_tmr_ioport == 0 {
            pm_tmr_ioport = acpi_gbl_FADT.pm_timer_block as c_int;
        }
    } else {
        /* FADT rev. 1 */
        pm_tmr_ioport = acpi_gbl_FADT.pm_timer_block as c_int;
    }

    printk(c"\x017start--> \n".as_ptr());
    then = read_pmtmr();
    then_tsc = rdtsc();
    i = 0;
    while i < 20 {
        mdelay(100);
        now = read_pmtmr();
        now_tsc = rdtsc();
        diff = now.wrapping_sub(then) & 0xFFFFFF;
        diff_tsc = now_tsc.wrapping_sub(then_tsc);
        printk(
            c"\x017t1: %08u t2: %08u diff_pmtmr: %08u diff_tsc: %016llu\n".as_ptr(),
            then,
            now,
            diff,
            diff_tsc as c_ulonglong,
        );
        then = now;
        then_tsc = now_tsc;
        i += 1;
    }
    printk(c"\x017<-- end \n".as_ptr());
    -ENODEV
}

unsafe fn cpufreq_none() {
    return;
}

/*
 * module_init(cpufreq_test_tsc)
 * module_exit(cpufreq_none)
 *
 * MODULE_AUTHOR("Dominik Brodowski");
 * MODULE_DESCRIPTION("Verify the TSC cpufreq notifier working correctly -- needs ACPI-enabled system");
 * MODULE_LICENSE ("GPL");
 */
