/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// Dependencies supplied by the corresponding Linux/OpenRISC headers:
// `elf_greg_t`, `pt_regs`, and the ELF header type used by `elf_check_arch`.

/*
 * This is used to ensure we don't load something for the wrong architecture.
 */
#[macro_export]
macro_rules! elf_check_arch {
    ($x:expr) => {
        (($x).e_machine == EM_OR32) || (($x).e_machine == EM_OPENRISC)
    };
}

/* This is the location that an ET_DYN program is loaded if exec'ed.  Typical
   use of this is to invoke "./ld.so someprog" to test out a new version of
   the loader.  We need to make sure that it is out of the way of the program
   that it will "exec", and that there is sufficient room for the brk.  */
pub const ELF_ET_DYN_BASE: usize = 0x0800_0000;

/*
 * Enable dump using regset.
 * This covers all of general/DSP/FPU regs.
 */
pub const CORE_DUMP_USE_REGSET: bool = true;

pub const ELF_EXEC_PAGESIZE: usize = 8192;

unsafe extern "C" {
    pub fn dump_elf_thread(dest: *mut elf_greg_t, pt: *mut pt_regs);
}

#[macro_export]
macro_rules! ELF_CORE_COPY_REGS {
    ($dest:expr, $regs:expr) => {
        unsafe { dump_elf_thread($dest, $regs) }
    };
}

/* This yields a mask that user programs can use to figure out what
   instruction set this cpu supports.  This could be done in userspace,
   but it's not easy, and we've already done it here.  */
pub const ELF_HWCAP: usize = 0;

/* This yields a string that ld.so will use to load implementation
   specific libraries for optimization.  This is more specific in
   intent than poking at uname or /proc/cpuinfo.

   For the moment, we have only optimizations for the Intel generations,
   but that could change... */
pub const ELF_PLATFORM: *const core::ffi::c_char = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
