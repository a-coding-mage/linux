/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2013 Imagination Technologies Ltd.
 *
 * Arbitrary Monitor Support (AMON)
 */

use core::ffi::{c_int, c_ulong};

unsafe extern "C" {
    pub fn amon_cpu_avail(cpu: c_int) -> c_int;
    pub fn amon_cpu_start(cpu: c_int, pc: c_ulong, sp: c_ulong, gp: c_ulong, a0: c_ulong) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
