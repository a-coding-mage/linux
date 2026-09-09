/*
 * The Cobalt board ID information.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997 Cobalt Microserver
 * Copyright (C) 1997, 2003 Ralf Baechle
 * Copyright (C) 2001, 2002, 2003 Liam Davies (ldavies@agile.tv)
 */

// Original C header guard: __ASM_COBALT_H

unsafe extern "C" {
    pub static mut cobalt_board_id: ::core::ffi::c_int;

    pub fn cobalt_machine_halt();
    pub fn cobalt_machine_restart(command: *mut ::core::ffi::c_char);
}

pub const COBALT_BRD_ID_QUBE1: i32 = 0x3;
pub const COBALT_BRD_ID_RAQ1: i32 = 0x4;
pub const COBALT_BRD_ID_QUBE2: i32 = 0x5;
pub const COBALT_BRD_ID_RAQ2: i32 = 0x6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
