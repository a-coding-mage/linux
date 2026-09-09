/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// C dependency: <uapi/asm/setup.h> supplies COMMAND_LINE_SIZE.

use core::ffi::c_char;

unsafe extern "C" {
    pub static mut cmd_line: [c_char; COMMAND_LINE_SIZE];

    pub static mut klimit: *mut c_char;

    pub fn mmu_reset();

    pub fn machine_early_init(
        cmdline: *const c_char,
        ram: u32,
        fdt: u32,
        msr: u32,
        tlb0: u32,
        tlb1: u32,
    );

    pub fn machine_restart(cmd: *mut c_char);
    pub fn machine_shutdown();
    pub fn machine_halt();
    pub fn machine_power_off();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
