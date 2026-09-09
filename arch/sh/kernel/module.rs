// SPDX-License-Identifier: GPL-2.0+
/*  Kernel module help for SH.

    SHcompact version by Kaz Kojima and Paul Mundt.

    SHmedia bits:

	Copyright 2004 SuperH (UK) Ltd
	Author: Richard Curnow

	Based on the sh version, and on code from the sh64-specific parts of
	modutils, originally written by Richard Curnow and Ben Gaster.
*/

// Dependencies supplied by the surrounding kernel environment:
// linux/moduleloader.h, linux/elf.h, linux/vmalloc.h, linux/bug.h,
// linux/fs.h, linux/string.h, linux/kernel.h, linux/unaligned.h,
// and asm/dwarf.h.

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf32_Shdr,
    strtab: *const ::std::os::raw::c_char,
    symindex: u32,
    relsec: u32,
    me: *mut module,
) -> i32 {
    let mut i: u32;
    let mut rel: *mut Elf32_Rela = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf32_Rela;
    let mut sym: *mut Elf32_Sym;
    let mut relocation: Elf32_Addr;
    let mut location: *mut u32;
    let mut value: u32;

    pr_debug(
        "Applying relocate section %u to %u\n\0".as_ptr() as *const ::std::os::raw::c_char,
        relsec,
        (*sechdrs.add(relsec as usize)).sh_info,
    );
    i = 0;
    while i < (*sechdrs.add(relsec as usize)).sh_size / ::std::mem::size_of::<Elf32_Rela>() as u32 {
        // This is where to make the change
        location = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr
            + (*rel.add(i as usize)).r_offset) as *mut u32;
        // This is the symbol it is referring to.  Note that all
        // undefined symbols have been resolved.
        sym = ((*sechdrs.add(symindex as usize)).sh_addr as *mut Elf32_Sym)
            .add(ELF32_R_SYM((*rel.add(i as usize)).r_info) as usize);
        relocation = (*sym).st_value.wrapping_add((*rel.add(i as usize)).r_addend as Elf32_Addr);

        match ELF32_R_TYPE((*rel.add(i as usize)).r_info) {
            R_SH_NONE => {}
            R_SH_DIR32 => {
                value = get_unaligned(location);
                value = value.wrapping_add(relocation);
                put_unaligned(value, location);
            }
            R_SH_REL32 => {
                relocation = relocation.wrapping_sub(location as Elf32_Addr);
                value = get_unaligned(location);
                value = value.wrapping_add(relocation);
                put_unaligned(value, location);
            }
            R_SH_IMM_LOW16 => {
                *location = (*location & !0x03ff_fc00) | ((relocation & 0xffff) << 10);
            }
            R_SH_IMM_MEDLOW16 => {
                *location = (*location & !0x03ff_fc00)
                    | (((relocation >> 16) & 0xffff) << 10);
            }
            R_SH_IMM_LOW16_PCREL => {
                relocation = relocation.wrapping_sub(location as Elf32_Addr);
                *location = (*location & !0x03ff_fc00) | ((relocation & 0xffff) << 10);
            }
            R_SH_IMM_MEDLOW16_PCREL => {
                relocation = relocation.wrapping_sub(location as Elf32_Addr);
                *location = (*location & !0x03ff_fc00)
                    | (((relocation >> 16) & 0xffff) << 10);
            }
            _ => {
                printk(
                    KERN_ERR,
                    "module %s: Unknown relocation: %u\n\0".as_ptr()
                        as *const ::std::os::raw::c_char,
                    (*me).name,
                    ELF32_R_TYPE((*rel.add(i as usize)).r_info),
                );
                return -ENOEXEC;
            }
        }
        i += 1;
    }
    0
}

pub unsafe fn module_finalize(
    hdr: *const Elf_Ehdr,
    sechdrs: *const Elf_Shdr,
    me: *mut module,
) -> i32 {
    let mut ret: i32 = 0;

    ret |= module_dwarf_finalize(hdr, sechdrs, me);

    ret
}

pub unsafe fn module_arch_cleanup(mod_: *mut module) {
    module_dwarf_cleanup(mod_);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
