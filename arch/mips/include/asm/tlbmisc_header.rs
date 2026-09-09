/* SPDX-License-Identifier: GPL-2.0 */

/*
 * - add_wired_entry() add a fixed TLB entry, and move wired register
 */
extern "C" {
    pub fn add_wired_entry(
        entrylo0: core::ffi::c_ulong,
        entrylo1: core::ffi::c_ulong,
        entryhi: core::ffi::c_ulong,
        pagemask: core::ffi::c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
