/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

/* Dependency: user_regs_struct and user_regs_arcv2 are supplied by asm/ptrace.h. */

/* Machine specific ELF Hdr flags */
pub const EF_ARC_OSABI_MSK: u32 = 0x00000f00;

pub const EF_ARC_OSABI_V3: u32 = 0x00000300; /* v3 (no legacy syscalls) */
pub const EF_ARC_OSABI_V4: u32 = 0x00000400; /* v4 (64bit data any reg align) */

/* __GNUC__ < 6 selects V3; otherwise the current ABI is V4. */
pub const EF_ARC_OSABI_CURRENT: u32 = EF_ARC_OSABI_V4;

pub type elf_greg_t = ::core::ffi::c_ulong;
pub type elf_fpregset_t = ::core::ffi::c_ulong;

pub const ELF_NGREG: usize =
    ::core::mem::size_of::<user_regs_struct>() / ::core::mem::size_of::<elf_greg_t>();
pub const ELF_ARCV2REG: usize =
    ::core::mem::size_of::<user_regs_arcv2>() / ::core::mem::size_of::<elf_greg_t>();

pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
