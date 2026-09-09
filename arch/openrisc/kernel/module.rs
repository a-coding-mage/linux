// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC module.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// Dependencies supplied by the Linux module loader and ELF headers.

unsafe extern "C" {
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

const R_OR1K_32: u32 = 1;
const R_OR1K_LO_16_IN_INSN: u32 = 2;
const R_OR1K_HI_16_IN_INSN: u32 = 3;
const R_OR1K_INSN_REL_26: u32 = 4;
const R_OR1K_32_PCREL: u32 = 5;
const R_OR1K_AHI16: u32 = 6;
const R_OR1K_SLO16: u32 = 7;

#[repr(C)]
pub struct Elf32_Shdr {
    pub sh_addr: u32,
    pub sh_info: u32,
    pub sh_size: u32,
}

#[repr(C)]
pub struct Elf32_Rela {
    pub r_offset: u32,
    pub r_info: u32,
    pub r_addend: i32,
}

#[repr(C)]
pub struct Elf32_Sym {
    pub st_value: u32,
}

#[repr(C)]
pub struct module {
    pub name: *const core::ffi::c_char,
}

#[inline]
unsafe fn elf32_r_sym(info: u32) -> usize {
    (info >> 8) as usize
}

#[inline]
unsafe fn elf32_r_type(info: u32) -> u32 {
    info & 0xff
}

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf32_Shdr,
    _strtab: *const core::ffi::c_char,
    symindex: u32,
    relsec: u32,
    me: *mut module,
) -> i32 {
    let rel = (*sechdrs.add(relsec as usize)).sh_addr as usize as *mut Elf32_Rela;

    pr_debug(
        b"Applying relocate section %u to %u\n\0".as_ptr() as *const core::ffi::c_char,
        relsec,
        (*sechdrs.add(relsec as usize)).sh_info,
    );
    for i in 0..((*sechdrs.add(relsec as usize)).sh_size as usize / core::mem::size_of::<Elf32_Rela>()) {
        // This is where to make the change
        let location = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr
            as usize
            + (*rel.add(i)).r_offset as usize) as *mut u32;

        // This is the symbol it is referring to.  Note that all
        // undefined symbols have been resolved.
        let sym = ((*sechdrs.add(symindex as usize)).sh_addr as usize
            as *mut Elf32_Sym)
            .add(elf32_r_sym((*rel.add(i)).r_info));
        let mut value = (*sym).st_value.wrapping_add((*rel.add(i)).r_addend as u32);

        match elf32_r_type((*rel.add(i)).r_info) {
            R_OR1K_32 => {
                *location = value;
            }
            R_OR1K_LO_16_IN_INSN => {
                *((location as *mut u16).add(1)) = value as u16;
            }
            R_OR1K_HI_16_IN_INSN => {
                *((location as *mut u16).add(1)) = (value >> 16) as u16;
            }
            R_OR1K_INSN_REL_26 => {
                value = value.wrapping_sub(location as u32);
                value >>= 2;
                value &= 0x03ffffff;
                value |= *location & 0xfc000000;
                *location = value;
            }
            R_OR1K_32_PCREL => {
                value = value.wrapping_sub(location as u32);
                *location = value;
            }
            R_OR1K_AHI16 => {
                // Adjust the operand to match with a signed LO16.
                value = value.wrapping_add(0x8000);
                *((location as *mut u16).add(1)) = (value >> 16) as u16;
            }
            R_OR1K_SLO16 => {
                // Split value lower 16-bits.
                value = ((value & 0xf800) << 10) | (value & 0x7ff);
                *location = (*location & !0x3e007ff) | value;
            }
            _ => {
                pr_err(
                    b"module %s: Unknown relocation: %u\n\0".as_ptr()
                        as *const core::ffi::c_char,
                    (*me).name,
                    elf32_r_type((*rel.add(i)).r_info),
                );
            }
        }
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
