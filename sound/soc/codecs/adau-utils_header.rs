/* SPDX-License-Identifier: GPL-2.0 */

unsafe extern "C" {
    pub fn adau_calc_pll_cfg(freq_in: u32, freq_out: u32, regs: *mut u8) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
