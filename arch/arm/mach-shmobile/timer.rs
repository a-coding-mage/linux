// SPDX-License-Identifier: GPL-2.0
/*
 * SH-Mobile Timer
 *
 * Copyright (C) 2010  Magnus Damm
 * Copyright (C) 2002 - 2009  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    static mut preset_lpj: u32;
    static HZ: u32;

    fn of_property_read_u32(
        np: *const device_node,
        property: *const core::ffi::c_char,
        value: *mut u32,
    ) -> i32;

    // Rust-side equivalent of the kernel's for_each_of_cpu_node(np) iterator.
    fn of_cpu_nodes() -> *const [*mut device_node];
}

#[inline]
pub unsafe fn shmobile_init_delay() {
    let mut max_freq: u32 = 0;

    // The C source uses for_each_of_cpu_node(np); its iterator is supplied by
    // the surrounding translation as of_cpu_nodes().
    for &np in (&*of_cpu_nodes()).iter() {
        let mut freq: u32 = 0;

        if of_property_read_u32(
            np as *const device_node,
            b"clock-frequency\0".as_ptr() as *const core::ffi::c_char,
            &mut freq,
        ) == 0
        {
            max_freq = core::cmp::max(max_freq, freq);
        }
    }

    if max_freq == 0 {
        return;
    }

    /*
     * Calculate a worst-case loops-per-jiffy value
     * based on maximum cpu core hz setting and the
     * __delay() implementation in arch/arm/lib/delay.S.
     *
     * This will result in a longer delay than expected
     * when the cpu core runs on lower frequencies.
     */

    if preset_lpj == 0 {
        preset_lpj = max_freq / HZ;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
