/* SPDX-License-Identifier: GPL-2.0-or-later */
/* toshiba.h -- Linux driver for accessing the SMM on Toshiba laptops
 *
 * Copyright (c) 1996-2000  Jonathan A. Buzzard (jonathan@buzzard.org.uk)
 *
 * Thanks to Juergen Heinzl <juergen@monocerus.demon.co.uk> for the pointers
 * on making sure the structure is aligned and packed.
 */

// Dependency provided by uapi/linux/toshiba.h:
// SMMRegisters

unsafe extern "C" {
    pub fn tosh_smm(regs: *mut SMMRegisters) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
