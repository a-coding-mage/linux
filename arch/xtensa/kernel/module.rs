/*
 * arch/xtensa/kernel/module.c
 *
 * Module support.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2006 Tensilica Inc.
 *
 * Chris Zankel <chris@zankel.net>
 *
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn decode_calln_opcode(location: *mut u8) -> i32 {
    #[cfg(__XTENSA_EB__)]
    {
        return if unsafe { *location } & 0xf0 == 0x50 { 1 } else { 0 };
    }
    #[cfg(__XTENSA_EL__)]
    {
        return if unsafe { *location } & 0xf == 0x5 { 1 } else { 0 };
    }
    0
}

unsafe fn decode_l32r_opcode(location: *mut u8) -> i32 {
    #[cfg(__XTENSA_EB__)]
    {
        return if unsafe { *location } & 0xf0 == 0x10 { 1 } else { 0 };
    }
    #[cfg(__XTENSA_EL__)]
    {
        return if unsafe { *location } & 0xf == 0x1 { 1 } else { 0 };
    }
    0
}

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf32_Shdr,
    strtab: *const i8,
    symindex: u32,
    relsec: u32,
    mod_: *mut module,
) -> i32 {
    let rela = (*(sechdrs.add(relsec as usize))).sh_addr as *mut Elf32_Rela;
    let count = (*(sechdrs.add(relsec as usize))).sh_size
        / core::mem::size_of::<Elf32_Rela>();

    pr_debug!("Applying relocate section {} to {}\n", relsec,
        (*(sechdrs.add(relsec as usize))).sh_info);

    for i in 0..count {
        let rela_i = &*rela.add(i);
        let location = ((*(sechdrs.add((*(sechdrs.add(relsec as usize))).sh_info as usize))).sh_addr
            .wrapping_add(rela_i.r_offset)) as *mut u8;
        let sym = ( (*(sechdrs.add(symindex as usize))).sh_addr as *mut Elf32_Sym )
            .add(ELF32_R_SYM(rela_i.r_info) as usize);
        let mut value = (*sym).st_value.wrapping_add(rela_i.r_addend as u32);

        match ELF32_R_TYPE(rela_i.r_info) {
            R_XTENSA_NONE | R_XTENSA_DIFF8 | R_XTENSA_DIFF16 |
            R_XTENSA_DIFF32 | R_XTENSA_ASM_EXPAND => {}

            R_XTENSA_32 | R_XTENSA_PLT => {
                let p = location as *mut u32;
                *p = (*p).wrapping_add(value);
            }

            R_XTENSA_SLOT0_OP => {
                if decode_calln_opcode(location) != 0 {
                    value = value.wrapping_sub((location as usize as u32 & !3).wrapping_add(4));
                    if value & 3 != 0 || ((value.wrapping_add(1 << 19)) >> 20) != 0 {
                        pr_err!("{}: relocation out of range, section {} reloc {} sym '{}'\n",
                            (*mod_).name, relsec, i, strtab.add((*sym).st_name as usize));
                        return -ENOEXEC;
                    }
                    value = (value as i32 >> 2) as u32;
                    #[cfg(__XTENSA_EB__)] {
                        *location = (*location & !0x3) | ((value >> 16) as u8 & 0x3);
                        *location.add(1) = (value >> 8) as u8;
                        *location.add(2) = value as u8;
                    }
                    #[cfg(__XTENSA_EL__)] {
                        *location = (*location & !0xc0) | ((value << 6) as u8 & 0xc0);
                        *location.add(1) = (value >> 2) as u8;
                        *location.add(2) = (value >> 10) as u8;
                    }
                } else if decode_l32r_opcode(location) != 0 {
                    value = value.wrapping_sub(((location as usize as u32).wrapping_add(3)) & !3);
                    if value & 3 != 0 || (value as i32 >> 18) != -1 {
                        pr_err!("{}: relocation out of range, section {} reloc {} sym '{}'\n",
                            (*mod_).name, relsec, i, strtab.add((*sym).st_name as usize));
                        return -ENOEXEC;
                    }
                    value = (value as i32 >> 2) as u32;
                    #[cfg(__XTENSA_EB__)] {
                        *location.add(1) = (value >> 8) as u8;
                        *location.add(2) = value as u8;
                    }
                    #[cfg(__XTENSA_EL__)] {
                        *location.add(1) = value as u8;
                        *location.add(2) = (value >> 8) as u8;
                    }
                }
                /* FIXME: Ignore any other opcodes. */
            }

            R_XTENSA_SLOT1_OP | R_XTENSA_SLOT2_OP | R_XTENSA_SLOT3_OP |
            R_XTENSA_SLOT4_OP | R_XTENSA_SLOT5_OP | R_XTENSA_SLOT6_OP |
            R_XTENSA_SLOT7_OP | R_XTENSA_SLOT8_OP | R_XTENSA_SLOT9_OP |
            R_XTENSA_SLOT10_OP | R_XTENSA_SLOT11_OP | R_XTENSA_SLOT12_OP |
            R_XTENSA_SLOT13_OP | R_XTENSA_SLOT14_OP => {
                pr_err!("{}: unexpected FLIX relocation: {}\n", (*mod_).name,
                    ELF32_R_TYPE(rela_i.r_info));
                return -ENOEXEC;
            }

            R_XTENSA_SLOT0_ALT | R_XTENSA_SLOT1_ALT | R_XTENSA_SLOT2_ALT |
            R_XTENSA_SLOT3_ALT | R_XTENSA_SLOT4_ALT | R_XTENSA_SLOT5_ALT |
            R_XTENSA_SLOT6_ALT | R_XTENSA_SLOT7_ALT | R_XTENSA_SLOT8_ALT |
            R_XTENSA_SLOT9_ALT | R_XTENSA_SLOT10_ALT | R_XTENSA_SLOT11_ALT |
            R_XTENSA_SLOT12_ALT | R_XTENSA_SLOT13_ALT | R_XTENSA_SLOT14_ALT => {
                pr_err!("{}: unexpected ALT relocation: {}\n", (*mod_).name,
                    ELF32_R_TYPE(rela_i.r_info));
                return -ENOEXEC;
            }

            _ => {
                pr_err!("{}: unexpected relocation: {}\n", (*mod_).name,
                    ELF32_R_TYPE(rela_i.r_info));
                return -ENOEXEC;
            }
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
