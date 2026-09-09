// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

#[inline]
unsafe fn arc_write_me(addr: *mut u16, value: ::core::ffi::c_ulong) {
    *addr = ((value & 0xffff0000) >> 16) as u16;
    *addr.add(1) = (value & 0xffff) as u16;
}

/*
 * This gets called before relocation loop in generic loader
 * Make a note of the section index of unwinding section
 */
pub unsafe fn module_frob_arch_sections(
    _hdr: *mut Elf_Ehdr,
    _sechdrs: *mut Elf_Shdr,
    secstr: *mut ::core::ffi::c_char,
    mod_: *mut module,
) -> i32 {
    #[cfg(CONFIG_ARC_DW2_UNWIND)]
    {
        (*mod_).arch.unw_sec_idx = 0;
        (*mod_).arch.unw_info = ::core::ptr::null_mut();
    }
    (*mod_).arch.secstr = secstr;
    0
}

pub unsafe fn module_arch_cleanup(mod_: *mut module) {
    #[cfg(CONFIG_ARC_DW2_UNWIND)]
    if !(*mod_).arch.unw_info.is_null() {
        unwind_remove_table((*mod_).arch.unw_info, 0);
    }
}

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf32_Shdr,
    strtab: *const ::core::ffi::c_char,
    symindex: u32, // sec index for sym tbl
    relsec: u32, // sec index for relo sec
    module: *mut module,
) -> i32 {
    let mut i: i32;
    let n: i32;
    let mut relo_type: i32;
    let rel_entry = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf32_Rela;
    let mut sym_entry: *mut Elf32_Sym;
    let sym_sec = (*sechdrs.add(symindex as usize)).sh_addr as *mut Elf32_Sym;
    let mut relocation: Elf32_Addr;
    let mut location: Elf32_Addr;
    let mut tgt_addr: Elf32_Addr;
    let tgtsec: u32;

    tgtsec = (*sechdrs.add(relsec as usize)).sh_info;
    tgt_addr = (*sechdrs.add(tgtsec as usize)).sh_addr;
    n = ((*sechdrs.add(relsec as usize)).sh_size / core::mem::size_of::<Elf32_Rela>()) as i32;

    pr_debug("\nSection to fixup %s @%x\n", (*module).arch.secstr.add((*sechdrs.add(tgtsec as usize)).sh_name as usize), tgt_addr);
    pr_debug("=========================================================\n");
    pr_debug("r_off\tr_add\tst_value ADDRESS  VALUE\n");
    pr_debug("=========================================================\n");

    i = 0;
    while i < n {
        let s: *const ::core::ffi::c_char;
        location = tgt_addr.wrapping_add((*rel_entry.add(i as usize)).r_offset);
        sym_entry = sym_sec.add(ELF32_R_SYM((*rel_entry.add(i as usize)).r_info) as usize);
        relocation = (*sym_entry).st_value.wrapping_add((*rel_entry.add(i as usize)).r_addend as Elf32_Addr);

        if (*sym_entry).st_name == 0 && ELF_ST_TYPE((*sym_entry).st_info) == STT_SECTION {
            s = (*module).arch.secstr.add((*sechdrs.add((*sym_entry).st_shndx as usize)).sh_name as usize);
        } else {
            s = strtab.add((*sym_entry).st_name as usize);
        }

        pr_debug("   %x\t%x\t%x %x %x [%s]\n", (*rel_entry.add(i as usize)).r_offset, (*rel_entry.add(i as usize)).r_addend, (*sym_entry).st_value, location, relocation, s);
        relo_type = ELF32_R_TYPE((*rel_entry.add(i as usize)).r_info) as i32;

        if R_ARC_32_ME == relo_type {
            arc_write_me(location as *mut u16, relocation as ::core::ffi::c_ulong);
        } else if R_ARC_32 == relo_type {
            *(location as *mut Elf32_Addr) = relocation;
        } else if R_ARC_32_PCREL == relo_type {
            *(location as *mut Elf32_Addr) = relocation.wrapping_sub(location);
        } else {
            pr_err("%s: unknown relocation: %u\n", (*module).name, ELF32_R_TYPE((*rel_entry.add(i as usize)).r_info));
            return -ENOEXEC;
        }
        i += 1;
    }

    #[cfg(CONFIG_ARC_DW2_UNWIND)]
    if strcmp((*module).arch.secstr.add((*sechdrs.add(tgtsec as usize)).sh_name as usize), ".eh_frame".as_ptr() as *const _) == 0 {
        (*module).arch.unw_sec_idx = tgtsec;
    }
    0
}

/* Just before lift off: After sections have been relocated, we add the
 * dwarf section to unwinder table pool
 * This couldn't be done in module_frob_arch_sections() because
 * relocations had not been applied by then
 */
pub unsafe fn module_finalize(_hdr: *const Elf32_Ehdr, sechdrs: *const Elf_Shdr, mod_: *mut module) -> i32 {
    #[cfg(CONFIG_ARC_DW2_UNWIND)]
    {
        let unwsec = (*mod_).arch.unw_sec_idx;
        if unwsec != 0 {
            let unw = unwind_add_table(mod_, (*sechdrs.add(unwsec as usize)).sh_addr as *mut _, (*sechdrs.add(unwsec as usize)).sh_size);
            (*mod_).arch.unw_info = unw;
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
