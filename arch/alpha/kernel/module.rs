// SPDX-License-Identifier: GPL-2.0-or-later
/*  Kernel module help for Alpha.
    Copyright (C) 2002 Richard Henderson.
*/

// Dependencies supplied by the kernel headers are intentionally not redefined here.

#[repr(C)]
struct got_entry {
    next: *mut got_entry,
    r_addend: Elf64_Sxword,
    got_offset: i32,
}

unsafe fn process_reloc_for_got(
    rela: *mut Elf64_Rela,
    chains: *mut got_entry,
    poffset: *mut Elf64_Xword,
) {
    let r_sym = ELF64_R_SYM((*rela).r_info);
    let r_type = ELF64_R_TYPE((*rela).r_info);
    let r_addend = (*rela).r_addend;
    let mut g = chains.add(r_sym as usize);

    if r_type != R_ALPHA_LITERAL {
        return;
    }

    while !g.is_null() {
        if (*g).r_addend == r_addend {
            if (*g).got_offset == 0 {
                (*g).got_offset = *poffset as i32;
                *poffset = (*poffset).wrapping_add(8);
            }
            break;
        }
        g = (*g).next;
    }

    if g.is_null() {
        g = kmalloc_obj::<got_entry>();
        (*g).next = (*chains.add(r_sym as usize)).next;
        (*g).r_addend = r_addend;
        (*g).got_offset = *poffset as i32;
        *poffset = (*poffset).wrapping_add(8);
        (*chains.add(r_sym as usize)).next = g;
    }

    // The unused bits above bit 8 in r_info store the GOT offset.
    (*rela).r_info |= ((*g).got_offset as Elf64_Xword) << 8;
}

pub unsafe fn module_frob_arch_sections(
    hdr: *mut Elf64_Ehdr,
    sechdrs: *mut Elf64_Shdr,
    secstrings: *mut i8,
    me: *mut module,
) -> i32 {
    let mut chains: *mut got_entry;
    let mut rela: *mut Elf64_Rela;
    let esechdrs = sechdrs.add((*hdr).e_shnum as usize);
    let mut symtab: *mut Elf64_Shdr = core::ptr::null_mut();
    let mut got: *mut Elf64_Shdr = core::ptr::null_mut();
    let mut s = sechdrs;

    while s < esechdrs {
        if (*s).sh_type == SHT_SYMTAB {
            symtab = s;
        } else if !strcmp(b".got\0".as_ptr() as *const i8, secstrings.add((*s).sh_name as usize)) {
            got = s;
            (*me).arch.gotsecindex = s.offset_from(sechdrs) as _;
        }
        s = s.add(1);
    }

    if symtab.is_null() {
        printk(KERN_ERR, b"module %s: no symbol table\n\0".as_ptr(), (*me).name);
        return -ENOEXEC;
    }
    if got.is_null() {
        printk(KERN_ERR, b"module %s: no got section\n\0".as_ptr(), (*me).name);
        return -ENOEXEC;
    }

    let nsyms = ((*symtab).sh_size as usize) / core::mem::size_of::<Elf64_Sym>();
    chains = kzalloc_objs::<got_entry>(nsyms);
    if chains.is_null() {
        printk(KERN_ERR, b"module %s: no memory for symbol chain buffer\n\0".as_ptr(), (*me).name);
        return -ENOMEM;
    }

    (*got).sh_size = 0;
    (*got).sh_addralign = 8;
    (*got).sh_type = SHT_NOBITS;

    s = sechdrs;
    while s < esechdrs {
        if (*s).sh_type == SHT_RELA {
            let nrela = (*s).sh_size as usize / core::mem::size_of::<Elf64_Rela>();
            rela = ((*hdr as *mut u8).add((*s).sh_offset as usize)) as *mut Elf64_Rela;
            for i in 0..nrela {
                process_reloc_for_got(rela.add(i), chains, &mut (*got).sh_size);
            }
        }
        s = s.add(1);
    }

    for i in 0..nsyms {
        let mut g = (*chains.add(i)).next;
        while !g.is_null() {
            let n = (*g).next;
            kfree(g as *mut core::ffi::c_void);
            g = n;
        }
    }
    kfree(chains as *mut core::ffi::c_void);
    0
}

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf64_Shdr,
    strtab: *const i8,
    symindex: u32,
    relsec: u32,
    me: *mut module,
) -> i32 {
    let mut rela = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf64_Rela;
    let n = (*sechdrs.add(relsec as usize)).sh_size as usize / core::mem::size_of::<Elf64_Rela>();
    let symtab = (*sechdrs.add(symindex as usize)).sh_addr as *mut Elf64_Sym;
    let base = (*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr as *mut u8;
    let got = (*sechdrs.add((*me).arch.gotsecindex as usize)).sh_addr as usize;
    let gp = got.wrapping_add(0x8000);

    for i in 0..n {
        let r = rela.add(i);
        let r_sym = ELF64_R_SYM((*r).r_info) as usize;
        let mut r_type = ELF64_R_TYPE((*r).r_info) as usize;
        let r_got_offset = r_type >> 8;
        r_type &= 0xff;
        let location = base.add((*r).r_offset as usize);
        let sym = symtab.add(r_sym);
        let mut value = (*sym).st_value.wrapping_add((*r).r_addend as u64);

        match r_type {
            R_ALPHA_NONE | R_ALPHA_LITUSE | R_ALPHA_HINT => {}
            R_ALPHA_REFLONG => *(location as *mut u32) = value as u32,
            R_ALPHA_REFQUAD => { *(location as *mut u32) = value as u32; *(location.add(4) as *mut u32) = (value >> 32) as u32; }
            R_ALPHA_GPREL32 | R_ALPHA_SREL32 => { value = value.wrapping_sub(if r_type == R_ALPHA_GPREL32 { gp as u64 } else { location as u64 }); *(location as *mut u32) = value as u32; }
            R_ALPHA_LITERAL => { let hi = got.wrapping_add(r_got_offset); let lo = hi.wrapping_sub(gp); *(location as *mut u16) = lo as u16; *(hi as *mut u64) = value; }
            R_ALPHA_SREL64 => { value = value.wrapping_sub(location as u64); *(location as *mut u64) = value; }
            R_ALPHA_GPRELLOW | R_ALPHA_GPREL16 => { value = value.wrapping_sub(gp as u64); *(location as *mut u16) = value as u16; }
            R_ALPHA_GPRELHIGH => { value = value.wrapping_sub(gp as u64).wrapping_add(0x8000) >> 16; *(location as *mut u16) = value as u16; }
            R_ALPHA_GPDISP => { value = (gp as u64).wrapping_sub(location as u64); let lo = value as i16; let hi = value.wrapping_sub(lo as u64) as u32; *(location as *mut u16) = (hi >> 16) as u16; *(location.add((*r).r_addend as usize) as *mut u16) = lo as u16; }
            R_ALPHA_BRSGP | R_ALPHA_BRADDR => { value = value.wrapping_sub(location as u64).wrapping_sub(4) >> 2; *(location as *mut u32) = value as u32; }
            _ => { printk(KERN_ERR, b"module %s: Unknown relocation: %lu\n\0".as_ptr(), (*me).name, r_type); return -ENOEXEC; }
        }
        rela = r.add(0);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
