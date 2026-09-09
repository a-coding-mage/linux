/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[cfg(feature = "CONFIG_MODULES")]
pub unsafe fn apply_relocate(
    sechdrs: *mut Elf32_Shdr,
    _strtab: *const ::core::ffi::c_char,
    symindex: u32,
    relsec: u32,
    me: *mut module,
) -> i32 {
    let mut i: u32;
    let rel = (*(sechdrs.add(relsec as usize))).sh_addr as *mut Elf32_Rel;
    let mut sym: *mut Elf32_Sym;
    let mut location: *mut u32;

    // DEBUGP("Applying relocate section %u to %u\n", relsec,
    //         (*sechdrs.add(relsec as usize)).sh_info);
    i = 0;
    while i < (*sechdrs.add(relsec as usize)).sh_size / core::mem::size_of::<Elf32_Rel>() as u32 {
        location = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr
            + (*rel.add(i as usize)).r_offset) as *mut u32;
        sym = ((*sechdrs.add(symindex as usize)).sh_addr as *mut Elf32_Sym)
            .add(ELF32_R_SYM((*rel.add(i as usize)).r_info) as usize);

        match ELF32_R_TYPE((*rel.add(i as usize)).r_info) {
            R_68K_32 => {
                *location = (*location).wrapping_add((*sym).st_value);
            }
            R_68K_PC32 => {
                *location = (*location).wrapping_add(
                    (*sym).st_value.wrapping_sub(location as u32),
                );
            }
            _ => {
                pr_err!("module %s: Unknown relocation: %u\n", (*me).name,
                    ELF32_R_TYPE((*rel.add(i as usize)).r_info));
                return -ENOEXEC;
            }
        }
        i += 1;
    }
    0
}

#[cfg(feature = "CONFIG_MODULES")]
pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf32_Shdr,
    _strtab: *const ::core::ffi::c_char,
    symindex: u32,
    relsec: u32,
    me: *mut module,
) -> i32 {
    let mut i: u32;
    let rel = (*(sechdrs.add(relsec as usize))).sh_addr as *mut Elf32_Rela;
    let mut sym: *mut Elf32_Sym;
    let mut location: *mut u32;

    // DEBUGP("Applying relocate_add section %u to %u\n", relsec,
    //         (*sechdrs.add(relsec as usize)).sh_info);
    i = 0;
    while i < (*sechdrs.add(relsec as usize)).sh_size / core::mem::size_of::<Elf32_Rela>() as u32 {
        location = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr
            + (*rel.add(i as usize)).r_offset) as *mut u32;
        sym = ((*sechdrs.add(symindex as usize)).sh_addr as *mut Elf32_Sym)
            .add(ELF32_R_SYM((*rel.add(i as usize)).r_info) as usize);

        match ELF32_R_TYPE((*rel.add(i as usize)).r_info) {
            R_68K_32 => {
                *location = ((*rel.add(i as usize)).r_addend as u32).wrapping_add((*sym).st_value);
            }
            R_68K_PC32 => {
                *location = ((*rel.add(i as usize)).r_addend as u32)
                    .wrapping_add((*sym).st_value)
                    .wrapping_sub(location as u32);
            }
            _ => {
                pr_err!("module %s: Unknown relocation: %u\n", (*me).name,
                    ELF32_R_TYPE((*rel.add(i as usize)).r_info));
                return -ENOEXEC;
            }
        }
        i += 1;
    }
    0
}

#[cfg(feature = "CONFIG_MODULES")]
pub unsafe fn module_finalize(
    _hdr: *const Elf_Ehdr,
    _sechdrs: *const Elf_Shdr,
    mod_: *mut module,
) -> i32 {
    module_fixup(mod_, (*mod_).arch.fixup_start, (*mod_).arch.fixup_end);
    0
}

pub unsafe fn module_fixup(
    _mod: *mut module,
    start: *mut m68k_fixup_info,
    end: *mut m68k_fixup_info,
) {
    #[cfg(feature = "CONFIG_MMU")]
    {
        let mut fixup = start;
        while fixup < end {
            match (*fixup).type_ {
                m68k_fixup_memoffset => {
                    *((*fixup).addr as *mut u32) = m68k_memoffset;
                }
                m68k_fixup_vnode_shift => {
                    let addr = (*fixup).addr as *mut u16;
                    *addr = (*addr).wrapping_add(m68k_virt_to_node_shift);
                }
                _ => {}
            }
            fixup = fixup.add(1);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
