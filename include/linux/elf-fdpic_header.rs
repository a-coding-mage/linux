/* SPDX-License-Identifier: GPL-2.0-or-later */
/* FDPIC ELF load map
 *
 * Copyright (C) 2003 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependency: declarations from <uapi/linux/elf-fdpic.h>. */

/*
 * The C header selects these aliases according to ELF_CLASS:
 * ELFCLASS32 selects the 32-bit declarations; otherwise the 64-bit ones.
 * Preserve that build-time selection here for the corresponding Rust
 * dependency declarations.
 */
#[cfg(feature = "elf_class_32")]
pub type Elf_Sword = Elf32_Sword;
#[cfg(feature = "elf_class_32")]
pub type elf_fdpic_loadseg = elf32_fdpic_loadseg;
#[cfg(feature = "elf_class_32")]
pub type elf_fdpic_loadmap = elf32_fdpic_loadmap;
#[cfg(feature = "elf_class_32")]
pub const ELF_FDPIC_LOADMAP_VERSION: _ = ELF32_FDPIC_LOADMAP_VERSION;

#[cfg(not(feature = "elf_class_32"))]
pub type Elf_Sword = Elf64_Sxword;
#[cfg(not(feature = "elf_class_32"))]
pub type elf_fdpic_loadmap = elf64_fdpic_loadmap;
#[cfg(not(feature = "elf_class_32"))]
pub type elf_fdpic_loadseg = elf64_fdpic_loadseg;
#[cfg(not(feature = "elf_class_32"))]
pub const ELF_FDPIC_LOADMAP_VERSION: _ = ELF64_FDPIC_LOADMAP_VERSION;

/*
 * binfmt binary parameters structure
 */
#[repr(C)]
pub struct elf_fdpic_params {
    pub hdr: elfhdr,                         /* ref copy of ELF header */
    pub phdrs: *mut elf_phdr,                /* ref copy of PT_PHDR table */
    pub loadmap: *mut elf_fdpic_loadmap,     /* loadmap to be passed to userspace */
    pub elfhdr_addr: ::core::ffi::c_ulong,   /* mapped ELF header user address */
    pub ph_addr: ::core::ffi::c_ulong,       /* mapped PT_PHDR user address */
    pub map_addr: ::core::ffi::c_ulong,      /* mapped loadmap user address */
    pub entry_addr: ::core::ffi::c_ulong,    /* mapped entry user address */
    pub stack_size: ::core::ffi::c_ulong,    /* stack size requested (PT_GNU_STACK) */
    pub dynamic_addr: ::core::ffi::c_ulong,  /* mapped PT_DYNAMIC user address */
    pub load_addr: ::core::ffi::c_ulong,     /* user address at which to map binary */
    pub flags: ::core::ffi::c_ulong,
}

pub const ELF_FDPIC_FLAG_ARRANGEMENT: ::core::ffi::c_ulong = 0x0000000f; /* PT_LOAD arrangement flags */
pub const ELF_FDPIC_FLAG_INDEPENDENT: ::core::ffi::c_ulong = 0x00000000; /* PT_LOADs can be put anywhere */
pub const ELF_FDPIC_FLAG_HONOURVADDR: ::core::ffi::c_ulong = 0x00000001; /* PT_LOAD.vaddr must be honoured */
pub const ELF_FDPIC_FLAG_CONSTDISP: ::core::ffi::c_ulong = 0x00000002; /* PT_LOADs require constant displacement */
pub const ELF_FDPIC_FLAG_CONTIGUOUS: ::core::ffi::c_ulong = 0x00000003; /* PT_LOADs should be contiguous */
pub const ELF_FDPIC_FLAG_EXEC_STACK: ::core::ffi::c_ulong = 0x00000010; /* T if stack to be executable */
pub const ELF_FDPIC_FLAG_NOEXEC_STACK: ::core::ffi::c_ulong = 0x00000020; /* T if stack not to be executable */
pub const ELF_FDPIC_FLAG_EXECUTABLE: ::core::ffi::c_ulong = 0x00000040; /* T if this object is the executable */
pub const ELF_FDPIC_FLAG_PRESENT: ::core::ffi::c_ulong = 0x80000000; /* T if this object is present */

#[cfg(feature = "mmu")]
unsafe extern "C" {
    pub fn elf_fdpic_arch_lay_out_mm(
        exec_params: *mut elf_fdpic_params,
        interp_params: *mut elf_fdpic_params,
        start_stack: *mut ::core::ffi::c_ulong,
        start_brk: *mut ::core::ffi::c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
