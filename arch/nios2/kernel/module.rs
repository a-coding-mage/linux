/*
 * Kernel module support for Nios II.
 *
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *   Written by Wentao Xu <xuwentao@microtronix.com>
 * Copyright (C) 2001, 2003 Rusty Russell
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file COPYING in the main directory of this
 * archive for more details.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf32_Shdr,
    strtab: *const c_char,
    symindex: c_uint,
    relsec: c_uint,
    mod_: *mut module,
) -> c_int {
    let mut i: c_uint = 0;
    let rela = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf32_Rela;

    pr_debug!(
        "Applying relocate section %u to %u\n",
        relsec,
        (*sechdrs.add(relsec as usize)).sh_info
    );

    while i < (*sechdrs.add(relsec as usize)).sh_size / core::mem::size_of::<Elf32_Rela>() as u32 {
        /* This is where to make the change */
        let mut word: u32;
        let loc = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr
            as *mut u8)
            .add((*rela.add(i as usize)).r_offset as usize) as *mut u32;
        /* This is the symbol it is referring to.  Note that all
           undefined symbols have been resolved.  */
        let sym = ( (*sechdrs.add(symindex as usize)).sh_addr as *mut Elf32_Sym)
            .add(ELF32_R_SYM((*rela.add(i as usize)).r_info) as usize);
        let v = (*sym).st_value.wrapping_add((*rela.add(i as usize)).r_addend as u32);

        pr_debug!(
            "reltype %d 0x%x name:<%s>\n",
            ELF32_R_TYPE((*rela.add(i as usize)).r_info),
            (*rela.add(i as usize)).r_offset,
            strtab.add((*sym).st_name as usize)
        );

        match ELF32_R_TYPE((*rela.add(i as usize)).r_info) {
            R_NIOS2_NONE => {}
            R_NIOS2_BFD_RELOC_32 => {
                *loc = (*loc).wrapping_add(v);
            }
            R_NIOS2_PCREL16 => {
                let v = v.wrapping_sub((loc as usize as u32).wrapping_add(4));
                if (v as i32) > 0x7fff || (v as i32) < -(0x8000i32) {
                    pr_err!("module %s: relocation overflow\n", (*mod_).name);
                    return -ENOEXEC;
                }
                word = *loc;
                *loc = ((((word >> 22) << 16) | (v & 0xffff)) << 6) | (word & 0x3f);
            }
            R_NIOS2_CALL26 => {
                if v & 3 != 0 {
                    pr_err!("module %s: dangerous relocation\n", (*mod_).name);
                    return -ENOEXEC;
                }
                if (v >> 28) != ((loc as usize as u32) >> 28) {
                    pr_err!("module %s: relocation overflow\n", (*mod_).name);
                    return -ENOEXEC;
                }
                *loc = (*loc & 0x3f) | ((v >> 2) << 6);
            }
            R_NIOS2_HI16 => {
                word = *loc;
                *loc = ((((word >> 22) << 16) | ((v >> 16) & 0xffff)) << 6) | (word & 0x3f);
            }
            R_NIOS2_LO16 => {
                word = *loc;
                *loc = ((((word >> 22) << 16) | (v & 0xffff)) << 6) | (word & 0x3f);
            }
            R_NIOS2_HIADJ16 => {
                let word2 = ((v >> 16).wrapping_add((v >> 15) & 1)) & 0xffff;
                word = *loc;
                *loc = ((((word >> 22) << 16) | word2) << 6) | (word & 0x3f);
            }
            _ => {
                pr_err!(
                    "module %s: Unknown reloc: %u\n",
                    (*mod_).name,
                    ELF32_R_TYPE((*rela.add(i as usize)).r_info)
                );
                return -ENOEXEC;
            }
        }
        i += 1;
    }

    0
}

pub unsafe fn module_finalize(
    _hdr: *const Elf_Ehdr,
    _sechdrs: *const Elf_Shdr,
    _me: *mut module,
) -> c_int {
    flush_cache_all();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
