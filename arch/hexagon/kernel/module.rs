// SPDX-License-Identifier: GPL-2.0-only
/*
 * Kernel module loader for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the kernel headers are intentionally left external.

#[allow(unused_variables)]
pub unsafe fn module_frob_arch_sections(
    hdr: *mut Elf_Ehdr,
    sechdrs: *mut Elf_Shdr,
    secstrings: *mut core::ffi::c_char,
    mod_: *mut module,
) -> core::ffi::c_int {
    let mut found: core::ffi::c_int = 0;

    // Look for .plt and/or .got.plt and/or .init.plt sections
    for i in 0..(*hdr).e_shnum as usize {
        let name = secstrings.add((*sechdrs.add(i)).sh_name as usize);
        if strcmp(name, b".plt\0".as_ptr() as *const core::ffi::c_char) == 0 {
            found = i as core::ffi::c_int + 1;
        }
        if strcmp(name, b".got.plt\0".as_ptr() as *const core::ffi::c_char) == 0 {
            found = i as core::ffi::c_int + 1;
        }
        if strcmp(name, b".rela.plt\0".as_ptr() as *const core::ffi::c_char) == 0 {
            found = i as core::ffi::c_int + 1;
        }
    }

    // At this time, we don't support modules compiled with -shared
    if found != 0 {
        printk(
            KERN_WARNING,
            b"Module '%s' contains unexpected .plt/.got sections.\0".as_ptr() as *const core::ffi::c_char,
            (*mod_).name,
        );
        // return -ENOEXEC;
    }

    0
}

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf_Shdr,
    strtab: *const core::ffi::c_char,
    symindex: u32,
    relsec: u32,
    module: *mut module,
) -> core::ffi::c_int {
    let nrelocs = (*sechdrs.add(relsec as usize)).sh_size as usize / core::mem::size_of::<Elf32_Rela>();
    let rela = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf32_Rela;
    let sym_info = (*sechdrs.add(relsec as usize)).sh_info;
    let sym_base = (*sechdrs.add(symindex as usize)).sh_addr as *mut Elf32_Sym;
    let loc_base = (*sechdrs.add(sym_info as usize)).sh_addr as *mut u32;

    for i in 0..nrelocs {
        let current = rela.add(i);
        let sym = sym_base.add(ELF32_R_SYM((*current).r_info) as usize);
        let location = (loc_base as *mut u8).add((*current).r_offset as usize) as *mut u32;
        let mut value = (*sym).st_value.wrapping_add((*current).r_addend as u32);

        match ELF32_R_TYPE((*current).r_info) {
            R_HEXAGON_B22_PCREL => {
                let dist = value.wrapping_sub(location as u32) as i32;
                if dist < -0x00800000 || dist >= 0x00800000 {
                    printk(KERN_ERR, (*module).name, b"R_HEXAGON_B22_PCREL reloc out of range\0".as_ptr(), dist, value, location as u32, core::ptr::null());
                    return -ENOEXEC;
                }
                *location &= !0x01ff3fff;
                *location |= 0x00003fff & dist as u32;
                *location |= 0x01ff0000 & ((dist.wrapping_shl(2)) as u32);
            }
            R_HEXAGON_HI16 => {
                value = (value >> 16) & 0xffff;
                *location &= !0x00c03fff;
                *location |= value & 0x3fff;
                *location |= (value & 0xc000) << 8;
            }
            R_HEXAGON_LO16 => {
                *location &= !0x00c03fff;
                *location |= value & 0x3fff;
                *location |= (value & 0xc000) << 8;
            }
            R_HEXAGON_32 => *location = value,
            R_HEXAGON_32_PCREL => *location = value.wrapping_sub(location as u32),
            R_HEXAGON_PLT_B22_PCREL | R_HEXAGON_GOTOFF_LO16 | R_HEXAGON_GOTOFF_HI16 => {
                printk(KERN_ERR, (*module).name, b"GOT/PLT relocations unsupported\0".as_ptr());
                return -ENOEXEC;
            }
            _ => {
                printk(KERN_ERR, (*module).name, b"unknown relocation: %u\0".as_ptr(), ELF32_R_TYPE((*current).r_info));
                return -ENOEXEC;
            }
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
