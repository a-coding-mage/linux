// SPDX-License-Identifier: GPL-2.0-only
/*
 * PowerQUICC II support functions
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Declarations supplied by the corresponding platform headers.
extern "C" {
    fn fsl_get_immr() -> *mut u32;
    fn in_be32(addr: *const u32) -> u32;
    fn printf(format: *const u8, ...) -> i32;
    fn dt_fixup_cpu_clocks(corefreq: u32, timebase: u32, sysfreq: u32);
    fn finddevice(path: *const u8) -> *mut core::ffi::c_void;
    fn setprop(node: *mut core::ffi::c_void, name: *const u8, value: *const core::ffi::c_void, len: u32);
}

const PQ2_SCCR: usize = 0x10c80 / 4; // System Clock Configuration Register
const PQ2_SCMR: usize = 0x10c88 / 4; // System Clock Mode Register

static PQ2_CORECNF_MAP: [i32; 32] = [
    3, 2, 2, 2, 4, 4, 5, 9, 6, 11, 8, 10, 3, 12, 7, -1,
    6, 5, 13, 2, 14, 4, 15, 9, 0, 11, 8, 10, 16, 12, 7, -1,
];

/* Get various clocks from crystal frequency.
 * Returns zero on failure and non-zero on success.
 */
#[no_mangle]
pub unsafe extern "C" fn pq2_get_clocks(
    crystal: u32,
    sysfreq: *mut u32,
    corefreq: *mut u32,
    timebase: *mut u32,
    brgfreq: *mut u32,
) -> i32 {
    let immr = fsl_get_immr();
    if immr.is_null() {
        printf(b"pq2_get_clocks: Couldn't get IMMR base.\r\n\0".as_ptr());
        return 0;
    }

    let sccr = in_be32(immr.add(PQ2_SCCR));
    let scmr = in_be32(immr.add(PQ2_SCMR));

    let dfbrg = sccr & 3;
    let corecnf = ((scmr >> 24) & 0x1f) as usize;
    let busdf = (scmr >> 20) & 0xf;
    let plldf = (scmr >> 12) & 1;
    let pllmf = scmr & 0xfff;

    let mainclk = crystal.wrapping_mul(pllmf.wrapping_add(1)) / plldf.wrapping_add(1);
    let busclk = mainclk / busdf.wrapping_add(1);

    if !sysfreq.is_null() {
        *sysfreq = mainclk / 2;
    }
    if !timebase.is_null() {
        *timebase = busclk / 4;
    }
    if !brgfreq.is_null() {
        *brgfreq = mainclk / (1u32 << ((dfbrg + 1) * 2));
    }

    if !corefreq.is_null() {
        let coremult = PQ2_CORECNF_MAP[corecnf];

        if coremult < 0 {
            *corefreq = mainclk / 2;
        } else if coremult == 0 {
            return 0;
        } else {
            *corefreq = busclk.wrapping_mul(coremult as u32) / 2;
        }
    }

    1
}

/* Set common device tree fields based on the given clock frequencies. */
#[no_mangle]
pub unsafe extern "C" fn pq2_set_clocks(sysfreq: u32, corefreq: u32, timebase: u32, brgfreq: u32) {
    dt_fixup_cpu_clocks(corefreq, timebase, sysfreq);

    let mut node = finddevice(b"/soc/cpm\0".as_ptr());
    if !node.is_null() {
        setprop(node, b"clock-frequency\0".as_ptr(), &sysfreq as *const u32 as *const core::ffi::c_void, 4);
    }

    node = finddevice(b"/soc/cpm/brg\0".as_ptr());
    if !node.is_null() {
        setprop(node, b"clock-frequency\0".as_ptr(), &brgfreq as *const u32 as *const core::ffi::c_void, 4);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pq2_fixup_clocks(crystal: u32) -> i32 {
    let mut sysfreq = 0u32;
    let mut corefreq = 0u32;
    let mut timebase = 0u32;
    let mut brgfreq = 0u32;

    if pq2_get_clocks(crystal, &mut sysfreq, &mut corefreq, &mut timebase, &mut brgfreq) == 0 {
        return 0;
    }

    pq2_set_clocks(sysfreq, corefreq, timebase, brgfreq);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
