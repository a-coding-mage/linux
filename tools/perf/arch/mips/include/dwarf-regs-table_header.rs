// SPDX-License-Identifier: GPL-2.0
/*
 * dwarf-regs-table.h : Mapping of DWARF debug register numbers into
 * register names.
 *
 * Copyright (C) 2013 Cavium, Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 */

// C conditional preserved: this table is defined only when
// DEFINE_DWARF_REGSTR_TABLE is set.
// REG_DWARFNUM_NAME(reg, idx) expanded to: [idx] = "$" #reg
pub const mips_regstr_tbl: [*const core::ffi::c_char; 66] = [
    b"$0\0".as_ptr() as *const core::ffi::c_char,
    b"$1\0".as_ptr() as *const core::ffi::c_char,
    b"$2\0".as_ptr() as *const core::ffi::c_char,
    b"$3\0".as_ptr() as *const core::ffi::c_char,
    b"$4\0".as_ptr() as *const core::ffi::c_char,
    b"$5\0".as_ptr() as *const core::ffi::c_char,
    b"$6\0".as_ptr() as *const core::ffi::c_char,
    b"$7\0".as_ptr() as *const core::ffi::c_char,
    b"$8\0".as_ptr() as *const core::ffi::c_char,
    b"$9\0".as_ptr() as *const core::ffi::c_char,
    b"$10\0".as_ptr() as *const core::ffi::c_char,
    b"$11\0".as_ptr() as *const core::ffi::c_char,
    b"$12\0".as_ptr() as *const core::ffi::c_char,
    b"$13\0".as_ptr() as *const core::ffi::c_char,
    b"$14\0".as_ptr() as *const core::ffi::c_char,
    b"$15\0".as_ptr() as *const core::ffi::c_char,
    b"$16\0".as_ptr() as *const core::ffi::c_char,
    b"$17\0".as_ptr() as *const core::ffi::c_char,
    b"$18\0".as_ptr() as *const core::ffi::c_char,
    b"$19\0".as_ptr() as *const core::ffi::c_char,
    b"$20\0".as_ptr() as *const core::ffi::c_char,
    b"$21\0".as_ptr() as *const core::ffi::c_char,
    b"$22\0".as_ptr() as *const core::ffi::c_char,
    b"$23\0".as_ptr() as *const core::ffi::c_char,
    b"$24\0".as_ptr() as *const core::ffi::c_char,
    b"$25\0".as_ptr() as *const core::ffi::c_char,
    b"$26\0".as_ptr() as *const core::ffi::c_char,
    b"$27\0".as_ptr() as *const core::ffi::c_char,
    b"$28\0".as_ptr() as *const core::ffi::c_char,
    b"%29\0".as_ptr() as *const core::ffi::c_char,
    b"$30\0".as_ptr() as *const core::ffi::c_char,
    b"$31\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    b"$hi\0".as_ptr() as *const core::ffi::c_char,
    b"$lo\0".as_ptr() as *const core::ffi::c_char,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
