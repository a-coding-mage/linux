/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	include/asm-mips/dec/system.h
 *
 *	Generic DECstation/DECsystem bits.
 *
 *	Copyright (C) 2005, 2006  Maciej W. Rozycki
 */

extern "C" {
    pub static mut dec_kn_slot_base: ::kernel::ffi::c_ulong;
    pub static mut dec_kn_slot_size: ::kernel::ffi::c_ulong;
    pub static mut dec_tc_bus: ::kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
