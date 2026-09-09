/*
 * Header file for using the wbflush routine
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 1998 Harald Koerfgen
 * Copyright (C) 2002 Maciej W. Rozycki
 */

// CONFIG_CPU_HAS_WB selects the implementation, matching the original
// preprocessor condition.

#[cfg(feature = "CONFIG_CPU_HAS_WB")]
unsafe extern "C" {
    pub static mut __wbflush: Option<unsafe extern "C" fn()>;
    pub fn wbflush_setup();
    fn __sync();
}

#[cfg(feature = "CONFIG_CPU_HAS_WB")]
#[inline]
pub unsafe fn wbflush() {
    __sync();
    if let Some(wbflush) = __wbflush {
        wbflush();
    }
}

#[cfg(not(feature = "CONFIG_CPU_HAS_WB"))]
#[inline]
pub fn wbflush_setup() {}

#[cfg(not(feature = "CONFIG_CPU_HAS_WB"))]
unsafe extern "C" {
    fn fast_iob();
}

#[cfg(not(feature = "CONFIG_CPU_HAS_WB"))]
#[inline]
pub unsafe fn wbflush() {
    fast_iob();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
