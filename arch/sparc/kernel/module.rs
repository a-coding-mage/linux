// SPDX-License-Identifier: GPL-2.0
/* Kernel module help for sparc64.
 *
 * Copyright (C) 2001 Rusty Russell.
 * Copyright (C) 2002 David S. Miller.
 */

// C dependencies supplied by the kernel and architecture headers are expected
// to provide the types, constants, macros, and functions referenced below.

/* Make generic code ignore STT_REGISTER dummy undefined symbols. */
pub unsafe fn module_frob_arch_sections(
    hdr: *mut Elf_Ehdr,
    sechdrs: *mut Elf_Shdr,
    _secstrings: *mut core::ffi::c_char,
    mod_: *mut module,
) -> i32 {
    let mut symidx: u32 = 0;
    let sym: *mut Elf_Sym;
    let mut i: i32;

    loop {
        if (*sechdrs.add(symidx as usize)).sh_type == SHT_SYMTAB {
            break;
        }
        if symidx == (*hdr).e_shnum - 1 {
            printk(b"%s: no symtab found.\n\0".as_ptr(), (*mod_).name);
            return -ENOEXEC;
        }
        symidx += 1;
    }
    sym = (*sechdrs.add(symidx as usize)).sh_addr as *mut Elf_Sym;

    i = 1;
    while i < ((*sechdrs.add(symidx as usize)).sh_size / core::mem::size_of::<Elf_Sym>()) as i32 {
        if (*sym.add(i as usize)).st_shndx == SHN_UNDEF {
            if ELF_ST_TYPE((*sym.add(i as usize)).st_info) == STT_REGISTER {
                (*sym.add(i as usize)).st_shndx = SHN_ABS;
            }
        }
        i += 1;
    }
    0
}

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf_Shdr,
    _strtab: *const core::ffi::c_char,
    symindex: u32,
    relsec: u32,
    me: *mut module,
) -> i32 {
    let mut i: u32 = 0;
    let rel = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf_Rela;

    while i < ((*sechdrs.add(relsec as usize)).sh_size / core::mem::size_of::<Elf_Rela>()) as u32 {
        let rela = &*rel.add(i as usize);
        let location = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr
            + rela.r_offset) as *mut u8;
        let loc32 = location as *mut u32;

        // CONFIG_SPARC64 conditional: preserve the original address assertion.
        #[cfg(CONFIG_SPARC64)]
        BUG_ON(((location as u64) >> 32) != 0);

        let sym = ((*sechdrs.add(symindex as usize)).sh_addr as *mut Elf_Sym)
            .add(ELF_R_SYM(rela.r_info) as usize);
        let mut v: Elf_Addr = (*sym).st_value + rela.r_addend;

        match ELF_R_TYPE(rela.r_info) & 0xff {
            R_SPARC_DISP32 => {
                v -= location as Elf_Addr;
                *loc32 = v as u32;
            }
            #[cfg(CONFIG_SPARC64)]
            R_SPARC_64 | R_SPARC_UA64 => {
                *location.add(0) = (v >> 56) as u8;
                *location.add(1) = (v >> 48) as u8;
                *location.add(2) = (v >> 40) as u8;
                *location.add(3) = (v >> 32) as u8;
                *location.add(4) = (v >> 24) as u8;
                *location.add(5) = (v >> 16) as u8;
                *location.add(6) = (v >> 8) as u8;
                *location.add(7) = v as u8;
            }
            #[cfg(CONFIG_SPARC64)]
            R_SPARC_WDISP19 => {
                v -= location as Elf_Addr;
                *loc32 = (*loc32 & !0x7ffff) | (((v >> 2) as u32) & 0x7ffff);
            }
            #[cfg(CONFIG_SPARC64)]
            R_SPARC_OLO10 => {
                *loc32 = (*loc32 & !0x1fff)
                    | ((((v & 0x3ff) + ((ELF_R_TYPE(rela.r_info) >> 8) as Elf_Addr)) as u32) & 0x1fff);
            }
            R_SPARC_32 | R_SPARC_UA32 => {
                *location.add(0) = (v >> 24) as u8;
                *location.add(1) = (v >> 16) as u8;
                *location.add(2) = (v >> 8) as u8;
                *location.add(3) = v as u8;
            }
            R_SPARC_WDISP30 => {
                v -= location as Elf_Addr;
                *loc32 = (*loc32 & !0x3fffffff) | (((v >> 2) as u32) & 0x3fffffff);
            }
            R_SPARC_WDISP22 => {
                v -= location as Elf_Addr;
                *loc32 = (*loc32 & !0x3fffff) | (((v >> 2) as u32) & 0x3fffff);
            }
            R_SPARC_LO10 => *loc32 = (*loc32 & !0x3ff) | ((v as u32) & 0x3ff),
            R_SPARC_HI22 => *loc32 = (*loc32 & !0x3fffff) | (((v >> 10) as u32) & 0x3fffff),
            _ => {
                printk(b"module %s: Unknown relocation: 0x%x\n\0".as_ptr(), (*me).name, (ELF_R_TYPE(rela.r_info) & 0xff) as i32);
                return -ENOEXEC;
            }
        }
        i += 1;
    }
    0
}

#[cfg(CONFIG_SPARC64)]
unsafe fn do_patch_sections(hdr: *const Elf_Ehdr, sechdrs: *const Elf_Shdr) {
    let mut sun4v_1insn: *const Elf_Shdr = core::ptr::null();
    let mut sun4v_2insn: *const Elf_Shdr = core::ptr::null();
    let secstrings = (hdr as *const u8).add((*sechdrs.add((*hdr).e_shstrndx as usize)).sh_offset as usize);

    for n in 0..(*hdr).e_shnum {
        let s = sechdrs.add(n as usize);
        let name = secstrings.add((*s).sh_name as usize) as *const i8;
        if strcmp(b".sun4v_1insn_patch\0".as_ptr() as *const i8, name) == 0 { sun4v_1insn = s; }
        if strcmp(b".sun4v_2insn_patch\0".as_ptr() as *const i8, name) == 0 { sun4v_2insn = s; }
    }
    if !sun4v_1insn.is_null() && tlb_type == hypervisor {
        let p = (*sun4v_1insn).sh_addr as *mut core::ffi::c_void;
        sun4v_patch_1insn_range(p, (p as *mut u8).add((*sun4v_1insn).sh_size as usize) as *mut core::ffi::c_void);
    }
    if !sun4v_2insn.is_null() && tlb_type == hypervisor {
        let p = (*sun4v_2insn).sh_addr as *mut core::ffi::c_void;
        sun4v_patch_2insn_range(p, (p as *mut u8).add((*sun4v_2insn).sh_size as usize) as *mut core::ffi::c_void);
    }
}

#[cfg(CONFIG_SPARC64)]
pub unsafe fn module_finalize(hdr: *const Elf_Ehdr, sechdrs: *const Elf_Shdr, _me: *mut module) -> i32 {
    do_patch_sections(hdr, sechdrs);
    /* Cheetah's I-cache is fully coherent. */
    if tlb_type == spitfire {
        flushw_all();
        let mut va: usize = 0;
        while va < (PAGE_SIZE << 1) {
            spitfire_put_icache_tag(va, 0x0);
            va += 32;
        }
        core::arch::asm!("flush %g6");
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
