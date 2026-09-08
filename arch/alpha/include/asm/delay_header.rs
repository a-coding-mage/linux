/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ALPHA_DELAY_H

unsafe extern "C" {
    pub fn __delay(loops: ::core::ffi::c_int);
    pub fn udelay(usecs: ::core::ffi::c_ulong);

    pub fn ndelay(nsecs: ::core::ffi::c_ulong);
}

// C macro: #define ndelay ndelay

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
