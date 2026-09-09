/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Imagination Technologies
 * Author: Alex Smith <alex.smith@imgtec.com>
 */

// External ELF types, constants, helpers, globals, and C I/O/string symbols
// referenced below are supplied by the surrounding translation unit.

pub unsafe fn patch_vdso(path: *const std::os::raw::c_char, vdso: *mut u8) -> bool {
    let ehdr = vdso as *const Ehdr;
    let shdrs: *mut u8;
    let mut shdr: *mut Shdr;
    let shstrtab: *mut std::os::raw::c_char;
    let mut name: *mut std::os::raw::c_char;
    let sh_count: u16;
    let sh_entsize: u16;

    shdrs = vdso.add(swap_uint((*ehdr).e_shoff) as usize);
    sh_count = swap_uint16((*ehdr).e_shnum);
    sh_entsize = swap_uint16((*ehdr).e_shentsize);

    shdr = shdrs
        .add((sh_entsize as usize) * (swap_uint16((*ehdr).e_shstrndx) as usize))
        as *mut Shdr;
    shstrtab = vdso.add(swap_uint((*shdr).sh_offset) as usize)
        as *mut std::os::raw::c_char;

    for i in 0..sh_count {
        shdr = shdrs.add((i as usize) * (sh_entsize as usize)) as *mut Shdr;
        name = shstrtab.add(swap_uint32((*shdr).sh_name) as usize);

        /*
         * Ensure there are no relocation sections - ld.so does not
         * relocate the VDSO so if there are relocations things will
         * break.
         */
        match swap_uint32((*shdr).sh_type) {
            SHT_REL | SHT_RELA => {
                fprintf(
                    stderr,
                    b"%s: '%s' contains relocation sections\n\0".as_ptr() as _,
                    program_name,
                    path,
                );
                return false;
            }
            _ => {}
        }

        /* Check for existing sections. */
        if strcmp(name, b".MIPS.abiflags\0".as_ptr() as _) == 0 {
            fprintf(
                stderr,
                b"%s: '%s' already contains a '.MIPS.abiflags' section\n\0".as_ptr()
                    as _,
                program_name,
                path,
            );
            return false;
        }

        if strcmp(name, b".mips_abiflags\0".as_ptr() as _) == 0 {
            strcpy(name, b".MIPS.abiflags\0".as_ptr() as _);
            (*shdr).sh_type = swap_uint32(SHT_MIPS_ABIFLAGS);
            (*shdr).sh_entsize = (*shdr).sh_size;
        }
    }

    true
}

pub unsafe fn get_symbols(path: *const std::os::raw::c_char, vdso: *mut u8) -> bool {
    let ehdr = vdso as *const Ehdr;
    let shdrs: *mut u8;
    let mut symtab: *mut u8;
    let mut shdr: *mut Shdr;
    let mut sym: *const Sym;
    let strtab: *mut std::os::raw::c_char;
    let mut name: *mut std::os::raw::c_char;
    let sh_count: u16;
    let sh_entsize: u16;
    let st_count: u16;
    let st_entsize: u16;
    let mut offset: u64;
    let flags: u32;

    shdrs = vdso.add(swap_uint((*ehdr).e_shoff) as usize);
    sh_count = swap_uint16((*ehdr).e_shnum);
    sh_entsize = swap_uint16((*ehdr).e_shentsize);

    let mut i = 0u16;
    while i < sh_count {
        shdr = shdrs.add((i as usize) * (sh_entsize as usize)) as *mut Shdr;
        if swap_uint32((*shdr).sh_type) == SHT_SYMTAB {
            break;
        }
        i += 1;
    }

    if i == sh_count {
        fprintf(stderr, b"%s: '%s' has no symbol table\n\0".as_ptr() as _, program_name, path);
        return false;
    }

    /* Get flags */
    flags = swap_uint32((*ehdr).e_flags);
    if elf_class == ELFCLASS64 {
        elf_abi = ABI_N64;
    } else if flags & EF_MIPS_ABI2 != 0 {
        elf_abi = ABI_N32;
    } else {
        elf_abi = ABI_O32;
    }

    /* Get symbol table. */
    symtab = vdso.add(swap_uint((*shdr).sh_offset) as usize);
    st_entsize = swap_uint((*shdr).sh_entsize) as u16;
    st_count = (swap_uint((*shdr).sh_size) / st_entsize as u32) as u16;

    /* Get string table. */
    shdr = shdrs.add((swap_uint32((*shdr).sh_link) as usize) * sh_entsize as usize)
        as *mut Shdr;
    strtab = vdso.add(swap_uint((*shdr).sh_offset) as usize) as *mut _;

    /* Write offsets for symbols needed by the kernel. */
    let mut si = 0usize;
    while !(*vdso_symbols.add(si)).name.is_null() {
        if (*vdso_symbols.add(si)).abis & elf_abi == 0 {
            si += 1;
            continue;
        }

        let mut j = 0u16;
        while j < st_count {
            sym = symtab.add((j as usize) * st_entsize as usize) as *const Sym;
            name = strtab.add(swap_uint32((*sym).st_name) as usize);

            if strcmp(name, (*vdso_symbols.add(si)).name) == 0 {
                offset = swap_uint((*sym).st_value);
                fprintf(out_file, b"\t.%s = 0x%lx,\n\0".as_ptr() as _, (*vdso_symbols.add(si)).offset_name, offset);
                break;
            }
            j += 1;
        }

        if j == st_count {
            fprintf(stderr, b"%s: '%s' is missing required symbol '%s'\n\0".as_ptr() as _, program_name, path, (*vdso_symbols.add(si)).name);
            return false;
        }
        si += 1;
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
