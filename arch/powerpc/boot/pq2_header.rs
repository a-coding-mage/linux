/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by the translated types header: u32.

extern "C" {
    pub fn pq2_get_clocks(
        crystal: u32,
        sysfreq: *mut u32,
        corefreq: *mut u32,
        timebase: *mut u32,
        brgfreq: *mut u32,
    ) -> i32;

    pub fn pq2_set_clocks(sysfreq: u32, corefreq: u32, timebase: u32, brgfreq: u32);

    pub fn pq2_fixup_clocks(crystal: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
