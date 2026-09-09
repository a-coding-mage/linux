// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/export.h, linux/moduleloader.h, linux/kernel.h, linux/elf.h,
// linux/vmalloc.h, linux/fs.h, linux/string.h, linux/pgtable.h,
// asm/cacheflush.h

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf32_Shdr,
    _strtab: *const core::ffi::c_char,
    symindex: u32,
    relsec: u32,
    module: *mut module,
) -> i32 {
    let mut i: u32;
    let rela = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf32_Rela;
    let mut sym: *mut Elf32_Sym;
    let mut location: *mut u64;
    let mut value: u64;

    pr_debug!(
        "Applying add relocation section {} to {}\n",
        relsec,
        (*sechdrs.add(relsec as usize)).sh_info
    );

    i = 0;
    while i < (*sechdrs.add(relsec as usize)).sh_size / core::mem::size_of::<Elf32_Rela>() as u32 {
        let current_rela = &*rela.add(i as usize);

        location = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr
            + current_rela.r_offset as u64) as *mut u64;
        sym = ((*sechdrs.add(symindex as usize)).sh_addr as *mut Elf32_Sym)
            .add(ELF32_R_SYM(current_rela.r_info) as usize);
        value = (*sym).st_value as u64 + current_rela.r_addend as u64;

        match ELF32_R_TYPE(current_rela.r_info) {
            R_MICROBLAZE_32 => {
                *location = value;
            }
            R_MICROBLAZE_64 => {
                *location = (*location & 0xFFFF0000) | (value >> 16);
                *location.add(1) = (*location.add(1) & 0xFFFF0000) | (value & 0xFFFF);
            }
            R_MICROBLAZE_64_PCREL => {
                value = value.wrapping_sub(location as u64).wrapping_sub(4);
                *location = (*location & 0xFFFF0000) | (value >> 16);
                *location.add(1) = (*location.add(1) & 0xFFFF0000) | (value & 0xFFFF);
                pr_debug!("R_MICROBLAZE_64_PCREL ({:08lx})\n", value);
            }
            R_MICROBLAZE_32_PCREL_LO => {
                pr_debug!("R_MICROBLAZE_32_PCREL_LO\n");
            }
            R_MICROBLAZE_64_NONE => {
                pr_debug!("R_MICROBLAZE_64_NONE\n");
            }
            R_MICROBLAZE_NONE => {
                pr_debug!("R_MICROBLAZE_NONE\n");
            }
            _ => {
                pr_err!(
                    "module {}: Unknown relocation: {}\n",
                    (*module).name,
                    ELF32_R_TYPE(current_rela.r_info)
                );
                return -ENOEXEC;
            }
        }
        i += 1;
    }
    0
}

pub unsafe fn module_finalize(
    _hdr: *const Elf32_Ehdr,
    _sechdrs: *const Elf_Shdr,
    _module: *mut module,
) -> i32 {
    flush_dcache();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
