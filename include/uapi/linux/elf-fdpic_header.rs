/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* elf-fdpic.h: FDPIC ELF load map
 *
 * Copyright (C) 2003 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// Dependency supplied by the Linux ELF definitions: PT_LOOS.
pub const PT_GNU_STACK: u32 = PT_LOOS + 0x474e551;

/* segment mappings for ELF FDPIC libraries/executables/interpreters */
#[repr(C)]
pub struct elf32_fdpic_loadseg {
	pub addr: Elf32_Addr,   /* core address to which mapped */
	pub p_vaddr: Elf32_Addr, /* VMA recorded in file */
	pub p_memsz: Elf32_Word, /* allocation size recorded in file */
}

#[repr(C)]
pub struct elf32_fdpic_loadmap {
	pub version: Elf32_Half, /* version of these structures, just in case... */
	pub nsegs: Elf32_Half,   /* number of segments */
	// Flexible array member: struct elf32_fdpic_loadseg segs[];
	pub segs: [elf32_fdpic_loadseg; 0],
}

pub const ELF32_FDPIC_LOADMAP_VERSION: u16 = 0x0000;

/* segment mappings for ELF FDPIC libraries/executables/interpreters */
#[repr(C)]
pub struct elf64_fdpic_loadseg {
	pub addr: Elf64_Addr,   /* core address to which mapped */
	pub p_vaddr: Elf64_Addr, /* VMA recorded in file */
	pub p_memsz: Elf64_Word, /* allocation size recorded in file */
}

#[repr(C)]
pub struct elf64_fdpic_loadmap {
	pub version: Elf64_Half, /* version of these structures, just in case... */
	pub nsegs: Elf64_Half,   /* number of segments */
	// Flexible array member: struct elf64_fdpic_loadseg segs[];
	pub segs: [elf64_fdpic_loadseg; 0],
}

pub const ELF64_FDPIC_LOADMAP_VERSION: u16 = 0x0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
