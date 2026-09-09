/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependency: <linux/elf-em.h>
pub const EM_MICROBLAZE_OLD: u16 = 0xbaab;
pub const ELF_ARCH: u16 = EM_MICROBLAZE;

/* Note there is no "official" ELF designation for Microblaze. */

/* This is used to ensure we don't load something for the wrong architecture. */
#[macro_export]
macro_rules! elf_check_arch {
    ($x:expr) => {
        (($x).e_machine == $crate::EM_MICROBLAZE
            || ($x).e_machine == $crate::EM_MICROBLAZE_OLD)
    };
}

/* These are used to set parameters in the core dumps. */
pub const ELF_CLASS: u8 = ELFCLASS32;

/* ELF register definitions. */
// Dependencies: <asm/ptrace.h>, <asm/byteorder.h>

pub type elf_greg_t = core::ffi::c_ulong;
pub const ELF_NGREG: usize = core::mem::size_of::<crate::pt_regs>() / core::mem::size_of::<elf_greg_t>();
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];

/* TBD */
pub const ELF_NFPREG: usize = 33; /* includes fsr */
pub type elf_fpreg_t = core::ffi::c_ulong;
pub type elf_fpregset_t = [elf_fpreg_t; ELF_NFPREG];

/*
 * This is the location that an ET_DYN program is loaded if exec'ed. Typical
 * use of this is to invoke "./ld.so someprog" to test out a new version of
 * the loader. We need to make sure that it is out of the way of the program
 * that it will "exec", and that there is sufficient room for the brk.
 */
pub const ELF_ET_DYN_BASE: u32 = 0x08000000;

#[cfg(__MICROBLAZEEL__)]
pub const ELF_DATA: u8 = ELFDATA2LSB;
#[cfg(not(__MICROBLAZEEL__))]
pub const ELF_DATA: u8 = ELFDATA2MSB;

pub const ELF_EXEC_PAGESIZE: usize = PAGE_SIZE;

#[macro_export]
macro_rules! ELF_CORE_COPY_REGS {
    ($dest:expr, $regs:expr) => {
        unsafe {
            core::ptr::copy_nonoverlapping(
                ($regs as *const _).cast::<u8>(),
                (&mut $dest as *mut _).cast::<u8>(),
                core::mem::size_of::<$crate::pt_regs>(),
            );
        }
    };
}

/* This yields a mask that user programs can use to figure out what
 * instruction set this CPU supports. */
pub const ELF_HWCAP: u32 = 0;

/* This yields a string that ld.so will use to load implementation specific
 * libraries for optimization. */
pub const ELF_PLATFORM: *const core::ffi::c_char = core::ptr::null();

/* Added _f parameter. Is this definition correct: TBD */
#[macro_export]
macro_rules! ELF_PLAT_INIT {
    ($r:expr, $f:expr) => {{
        $r.r0 = 0;
        $r.r1 = 0;
        $r.r2 = 0;
        $r.r3 = 0;
        $r.r4 = 0;
        $r.r5 = 0;
        $r.r6 = 0;
        $r.r7 = 0;
        $r.r8 = 0;
        $r.r9 = 0;
        $r.r10 = 0;
        $r.r11 = 0;
        $r.r12 = 0;
        $r.r13 = 0;
        $r.r14 = 0;
        $r.r15 = 0;
        $r.r16 = 0;
        $r.r17 = 0;
        $r.r18 = 0;
        $r.r19 = 0;
        $r.r20 = 0;
        $r.r21 = 0;
        $r.r22 = 0;
        $r.r23 = 0;
        $r.r24 = 0;
        $r.r25 = 0;
        $r.r26 = 0;
        $r.r27 = 0;
        $r.r28 = 0;
        $r.r29 = 0;
        $r.r30 = 0;
        $r.r31 = 0;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
