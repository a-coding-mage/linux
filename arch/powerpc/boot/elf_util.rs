// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) Paul Mackerras 1997.
 *
 * Updates for PPC64 by Todd Inglett, Dave Engebretsen & Peter Bergner.
 */

// Declarations and constants below are supplied by the corresponding ELF,
// page, string, and stdio dependencies.

pub unsafe fn parse_elf64(hdr: *mut core::ffi::c_void, info: *mut elf_info) -> i32 {
    let elf64 = hdr as *mut Elf64_Ehdr;
    let mut elf64ph: *mut Elf64_Phdr;
    let mut i: u32;

    if !((*elf64).e_ident[EI_MAG0] == ELFMAG0
        && (*elf64).e_ident[EI_MAG1] == ELFMAG1
        && (*elf64).e_ident[EI_MAG2] == ELFMAG2
        && (*elf64).e_ident[EI_MAG3] == ELFMAG3
        && (*elf64).e_ident[EI_CLASS] == ELFCLASS64
        // The C source selects ELFDATA2LSB when __LITTLE_ENDIAN__ is set.
        && (*elf64).e_ident[EI_DATA]
            == if cfg!(target_endian = "little") { ELFDATA2LSB } else { ELFDATA2MSB }
        && ((*elf64).e_type == ET_EXEC || (*elf64).e_type == ET_DYN)
        && (*elf64).e_machine == EM_PPC64)
    {
        return 0;
    }

    elf64ph = (elf64 as usize).wrapping_add((*elf64).e_phoff as usize) as *mut Elf64_Phdr;
    i = 0;
    while i < (*elf64).e_phnum as u32 {
        if (*elf64ph).p_type == PT_LOAD {
            break;
        }
        i = i.wrapping_add(1);
        elf64ph = elf64ph.add(1);
    }
    if i >= (*elf64).e_phnum as u32 {
        return 0;
    }

    (*info).loadsize = (*elf64ph).p_filesz as _;
    (*info).memsize = (*elf64ph).p_memsz as _;
    (*info).elfoffset = (*elf64ph).p_offset as _;

    1
}

pub unsafe fn parse_elf32(hdr: *mut core::ffi::c_void, info: *mut elf_info) -> i32 {
    let elf32 = hdr as *mut Elf32_Ehdr;
    let mut elf32ph: *mut Elf32_Phdr;
    let mut i: u32;

    if !((*elf32).e_ident[EI_MAG0] == ELFMAG0
        && (*elf32).e_ident[EI_MAG1] == ELFMAG1
        && (*elf32).e_ident[EI_MAG2] == ELFMAG2
        && (*elf32).e_ident[EI_MAG3] == ELFMAG3
        && (*elf32).e_ident[EI_CLASS] == ELFCLASS32
        && (*elf32).e_ident[EI_DATA] == ELFDATA2MSB
        && ((*elf32).e_type == ET_EXEC || (*elf32).e_type == ET_DYN)
        && (*elf32).e_machine == EM_PPC)
    {
        return 0;
    }

    elf32ph = (elf32 as usize).wrapping_add((*elf32).e_phoff as usize) as *mut Elf32_Phdr;
    i = 0;
    while i < (*elf32).e_phnum as u32 {
        if (*elf32ph).p_type == PT_LOAD {
            break;
        }
        i = i.wrapping_add(1);
        elf32ph = elf32ph.add(1);
    }
    if i >= (*elf32).e_phnum as u32 {
        return 0;
    }

    (*info).loadsize = (*elf32ph).p_filesz as _;
    (*info).memsize = (*elf32ph).p_memsz as _;
    (*info).elfoffset = (*elf32ph).p_offset as _;
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
