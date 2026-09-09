/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997, 1999, 2001, 06 by Ralf Baechle
 * Copyright (C) 2001 MIPS Technologies, Inc.
 */

use core::ffi::c_char;

extern "C" {
    pub static mut _machine_restart: Option<unsafe extern "C" fn(command: *mut c_char)>;
    pub static mut _machine_halt: Option<unsafe extern "C" fn()>;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
