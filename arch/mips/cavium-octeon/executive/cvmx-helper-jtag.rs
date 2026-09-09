/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 2.
 ***********************license end**************************************/

/**
 * Helper utilities for qlm_jtag.
 *
 */

// C dependencies supplied by the surrounding OCTEON headers are intentionally
// left as external Rust declarations/references.

#[repr(C)]
pub union cvmx_ciu_qlm_jtgc {
    pub u64: u64,
    pub s: cvmx_ciu_qlm_jtgc_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_ciu_qlm_jtgc_s {
    pub clk_div: u64,
    pub mux_sel: u64,
    pub bypass: u64,
}

#[repr(C)]
pub union cvmx_ciu_qlm_jtgd {
    pub u64: u64,
    pub s: cvmx_ciu_qlm_jtgd_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_ciu_qlm_jtgd_s {
    pub shift: u64,
    pub shft_cnt: u64,
    pub shft_reg: u64,
    pub select: u64,
    pub update: u64,
}

extern "C" {
    fn cvmx_sysinfo_get() -> *const cvmx_sysinfo;
    fn cvmx_write_csr(address: u64, value: u64);
    fn cvmx_read_csr(address: u64) -> u64;
    fn OCTEON_IS_MODEL(model: u32) -> bool;
}

#[repr(C)]
pub struct cvmx_sysinfo {
    pub cpu_clock_hz: u32,
}

// Constants and model identifiers are provided by the OCTEON headers.
extern "C" {
    static CVMX_CIU_QLM_JTGC: u64;
    static CVMX_CIU_QLM_JTGD: u64;
    static OCTEON_CN52XX: u32;
    static OCTEON_CN56XX_PASS1_X: u32;
}

/**
 * Initialize the internal QLM JTAG logic to allow programming
 * of the JTAG chain by the cvmx_helper_qlm_jtag_*() functions.
 * These functions should only be used at the direction of Cavium
 * Networks. Programming incorrect values into the JTAG chain
 * can cause chip damage.
 */
pub unsafe fn cvmx_helper_qlm_jtag_init() {
    let mut jtgc = cvmx_ciu_qlm_jtgc { u64: 0 };
    let mut clock_div: u32 = 0;
    let mut divisor: u32 = ((*cvmx_sysinfo_get()).cpu_clock_hz / (25 * 1000000));
    divisor = (divisor - 1) >> 2;
    /* Convert the divisor into a power of 2 shift */
    while divisor != 0 {
        clock_div += 1;
        divisor >>= 1;
    }

    /*
     * Clock divider for QLM JTAG operations. eclk is divided by
     * 2^(CLK_DIV + 2)
     */
    jtgc.u64 = 0;
    (*(&mut jtgc.s)).clk_div = clock_div as u64;
    (*(&mut jtgc.s)).mux_sel = 0;
    if OCTEON_IS_MODEL(OCTEON_CN52XX) {
        (*(&mut jtgc.s)).bypass = 0x3;
    } else {
        (*(&mut jtgc.s)).bypass = 0xf;
    }
    cvmx_write_csr(CVMX_CIU_QLM_JTGC, jtgc.u64);
    cvmx_read_csr(CVMX_CIU_QLM_JTGC);
}

/** Write up to 32 bits into the QLM JTAG chain. */
pub unsafe fn cvmx_helper_qlm_jtag_shift(qlm: i32, bits: i32, data: u32) -> u32 {
    let mut jtgd = cvmx_ciu_qlm_jtgd { u64: 0 };
    jtgd.u64 = 0;
    (*(&mut jtgd.s)).shift = 1;
    (*(&mut jtgd.s)).shft_cnt = (bits - 1) as u64;
    (*(&mut jtgd.s)).shft_reg = data as u64;
    if !OCTEON_IS_MODEL(OCTEON_CN56XX_PASS1_X) {
        (*(&mut jtgd.s)).select = (1i32 << qlm) as u64;
    }
    cvmx_write_csr(CVMX_CIU_QLM_JTGD, jtgd.u64);
    loop {
        jtgd.u64 = cvmx_read_csr(CVMX_CIU_QLM_JTGD);
        if (*(&jtgd.s)).shift == 0 { break; }
    }
    ((*(&jtgd.s)).shft_reg >> (32 - bits)) as u32
}

/** Shift long sequences of zeros into the QLM JTAG chain. */
pub unsafe fn cvmx_helper_qlm_jtag_shift_zeros(qlm: i32, mut bits: i32) {
    while bits > 0 {
        let mut n = bits;
        if n > 32 { n = 32; }
        cvmx_helper_qlm_jtag_shift(qlm, n, 0);
        bits -= n;
    }
}

/** Program the QLM JTAG chain into all lanes of the QLM. */
pub unsafe fn cvmx_helper_qlm_jtag_update(qlm: i32) {
    let mut jtgd = cvmx_ciu_qlm_jtgd { u64: 0 };
    /* Update the new data */
    jtgd.u64 = 0;
    (*(&mut jtgd.s)).update = 1;
    if !OCTEON_IS_MODEL(OCTEON_CN56XX_PASS1_X) {
        (*(&mut jtgd.s)).select = (1i32 << qlm) as u64;
    }
    cvmx_write_csr(CVMX_CIU_QLM_JTGD, jtgd.u64);
    loop {
        jtgd.u64 = cvmx_read_csr(CVMX_CIU_QLM_JTGD);
        if (*(&jtgd.s)).update == 0 { break; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
