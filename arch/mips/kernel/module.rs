// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *  Copyright (C) 2001 Rusty Russell.
 *  Copyright (C) 2003, 2004 Ralf Baechle (ralf@linux-mips.org)
 *  Copyright (C) 2005 Thiemo Seufer
 */

// C headers and build configuration are supplied by the surrounding kernel.

#[repr(C)]
struct mips_hi16 {
    next: *mut mips_hi16,
    addr: *mut Elf_Addr,
    value: Elf_Addr,
}

static mut dbe_list: list_head = LIST_HEAD_INIT;
static mut dbe_lock: spinlock_t = DEFINE_SPINLOCK_INIT;

unsafe fn apply_r_mips_32(location: *mut u32, base: u32, v: Elf_Addr) {
    *location = base.wrapping_add(v as u32);
}

unsafe fn apply_r_mips_26(me: *mut module, location: *mut u32, base: u32, v: Elf_Addr) -> c_int {
    if v % 4 != 0 {
        pr_err!("module %s: dangerous R_MIPS_26 relocation\n", (*me).name);
        return -ENOEXEC;
    }
    if (v & 0xf0000000) != (((location as usize + 4) as Elf_Addr) & 0xf0000000) {
        pr_err!("module %s: relocation overflow\n", (*me).name);
        return -ENOEXEC;
    }
    *location = (*location & !0x03ffffff) | ((base.wrapping_add((v >> 2) as u32)) & 0x03ffffff);
    0
}

unsafe fn apply_r_mips_hi16(me: *mut module, location: *mut u32, v: Elf_Addr, rela: bool) -> c_int {
    if rela {
        *location = (*location & 0xffff0000) | ((((v as i64 + 0x8000) >> 16) as u32) & 0xffff);
        return 0;
    }
    let n = kmalloc_obj::<mips_hi16>();
    if n.is_null() { return -ENOMEM; }
    (*n).addr = location as *mut Elf_Addr;
    (*n).value = v;
    (*n).next = (*me).arch.r_mips_hi16_list;
    (*me).arch.r_mips_hi16_list = n;
    0
}

unsafe fn free_relocation_chain(mut l: *mut mips_hi16) {
    while !l.is_null() {
        let next = (*l).next;
        kfree(l);
        l = next;
    }
}

unsafe fn apply_r_mips_lo16(me: *mut module, location: *mut u32, base: u32, v: Elf_Addr, rela: bool) -> c_int {
    let mut insnlo = base as u64;
    if rela { *location = (*location & 0xffff0000) | (v as u32 & 0xffff); return 0; }
    let vallo = (((insnlo & 0xffff) ^ 0x8000).wrapping_sub(0x8000)) as i64;
    if !(*me).arch.r_mips_hi16_list.is_null() {
        let mut l = (*me).arch.r_mips_hi16_list;
        while !l.is_null() {
            let next = (*l).next;
            if v != (*l).value { free_relocation_chain(l); (*me).arch.r_mips_hi16_list = core::ptr::null_mut(); pr_err!("module %s: dangerous R_MIPS_LO16 relocation\n", (*me).name); return -ENOEXEC; }
            let mut insn = *(*l).addr as u64;
            let mut val = ((insn & 0xffff) << 16).wrapping_add(vallo as u64).wrapping_add(v as u64);
            val = ((val >> 16) + ((val & 0x8000) != 0) as u64) & 0xffff;
            insn = (insn & !0xffff) | val;
            *(*l).addr = insn as Elf_Addr;
            kfree(l); l = next;
        }
        (*me).arch.r_mips_hi16_list = core::ptr::null_mut();
    }
    let val = (v as i64).wrapping_add(vallo) as u64;
    insnlo = (insnlo & !0xffff) | (val & 0xffff);
    *location = insnlo as u32;
    0
}

unsafe fn apply_r_mips_pc(me: *mut module, location: *mut u32, base: u32, v: Elf_Addr, bits: u32) -> c_int {
    let mask = (1u64 << bits) - 1;
    if v % 4 != 0 { pr_err!("module %s: dangerous R_MIPS_PC%u relocation\n", (*me).name, bits); return -ENOEXEC; }
    let mut offset = (base as u64) & mask;
    if offset & (1u64 << (bits - 1)) != 0 { offset |= !mask; }
    let offset = (offset as i64).wrapping_add(((v as i64) - (location as i64)) >> 2);
    let se_bits = if offset & (1i64 << (bits - 1)) != 0 { !0u64 } else { 0 };
    if (offset as u64 & !mask) != (se_bits & !mask) { pr_err!("module %s: relocation overflow\n", (*me).name); return -ENOEXEC; }
    *location = (*location & !(mask as u32)) | (offset as u32 & mask as u32);
    0
}

unsafe fn apply_r_mips_pc16(m: *mut module, l: *mut u32, b: u32, v: Elf_Addr) -> c_int { apply_r_mips_pc(m,l,b,v,16) }
unsafe fn apply_r_mips_pc21(m: *mut module, l: *mut u32, b: u32, v: Elf_Addr) -> c_int { apply_r_mips_pc(m,l,b,v,21) }
unsafe fn apply_r_mips_pc26(m: *mut module, l: *mut u32, b: u32, v: Elf_Addr) -> c_int { apply_r_mips_pc(m,l,b,v,26) }

unsafe fn apply_r_mips_64(location: *mut u32, v: Elf_Addr, rela: bool) -> c_int { if !rela { return -EINVAL; } *(location as *mut Elf_Addr) = v; 0 }
unsafe fn apply_r_mips_higher(location: *mut u32, v: Elf_Addr, rela: bool) -> c_int { if !rela { return -EINVAL; } *location = (*location & 0xffff0000) | ((((v as i64 + 0x80008000) >> 32) as u32) & 0xffff); 0 }
unsafe fn apply_r_mips_highest(location: *mut u32, v: Elf_Addr, rela: bool) -> c_int { if !rela { return -EINVAL; } *location = (*location & 0xffff0000) | ((((v as i64 + 0x800080008000) >> 48) as u32) & 0xffff); 0 }

unsafe fn reloc_handler(t: u32, me: *mut module, l: *mut u32, b: u32, v: Elf_Addr, r: bool) -> c_int {
    match t {
        R_MIPS_NONE => {}, R_MIPS_32 => apply_r_mips_32(l,b,v),
        R_MIPS_26 => return apply_r_mips_26(me,l,b,v), R_MIPS_HI16 => return apply_r_mips_hi16(me,l,v,r),
        R_MIPS_LO16 => return apply_r_mips_lo16(me,l,b,v,r), R_MIPS_PC16 => return apply_r_mips_pc16(me,l,b,v),
        R_MIPS_PC21_S2 => return apply_r_mips_pc21(me,l,b,v), R_MIPS_PC26_S2 => return apply_r_mips_pc26(me,l,b,v),
        R_MIPS_64 => return apply_r_mips_64(l,v,r), R_MIPS_HIGHER => return apply_r_mips_higher(l,v,r),
        R_MIPS_HIGHEST => return apply_r_mips_highest(l,v,r),
        _ => { pr_err!("%s: Unknown relocation type %u\n", (*me).name, t); return -EINVAL; }
    } 0
}

unsafe fn __apply_relocate(sechdrs: *mut Elf_Shdr, strtab: *const c_char, symindex: u32, relsec: u32, me: *mut module, rela: bool) -> c_int {
    let mut err = 0;
    let mut rel = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf_Mips_Rel;
    let size = if rela { core::mem::size_of::<Elf_Mips_Rela>() } else { core::mem::size_of::<Elf_Mips_Rel>() };
    (*me).arch.r_mips_hi16_list = core::ptr::null_mut();
    for _ in 0..((*sechdrs.add(relsec as usize)).sh_size as usize / size) {
        let location = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr + (*rel).r_offset) as *mut u32;
        let sym = ((*sechdrs.add(symindex as usize)).sh_addr as *mut Elf_Sym).add(ELF_MIPS_R_SYM(*rel) as usize);
        if (*sym).st_value >= -MAX_ERRNO as Elf_Addr { if ELF_ST_BIND((*sym).st_info) == STB_WEAK { rel = rel.add(1); continue; } pr_warn!("%s: Unknown symbol %s\n", (*me).name, strtab.add((*sym).st_name as usize)); err = -ENOENT; break; }
        let (v,b) = if rela { let x = *(rel as *mut Elf_Mips_Rela); rel = (rel as *mut Elf_Mips_Rela).add(1) as *mut Elf_Mips_Rel; ((*sym).st_value + x.r_addend as Elf_Addr, 0) } else { let x = (*sym).st_value; let b = *location; rel = rel.add(1); (x,b) };
        err = reloc_handler(ELF_MIPS_R_TYPE(*rel.sub(1)),me,location,b,v,rela); if err != 0 { break; }
    }
    if !(*me).arch.r_mips_hi16_list.is_null() { free_relocation_chain((*me).arch.r_mips_hi16_list); (*me).arch.r_mips_hi16_list = core::ptr::null_mut(); if err == 0 { err = -ENOEXEC; } }
    err
}

pub unsafe fn apply_relocate(s: *mut Elf_Shdr, st: *const c_char, si: u32, rs: u32, m: *mut module) -> c_int { __apply_relocate(s,st,si,rs,m,false) }

#[cfg(CONFIG_MODULES_USE_ELF_RELA)]
pub unsafe fn apply_relocate_add(s: *mut Elf_Shdr, st: *const c_char, si: u32, rs: u32, m: *mut module) -> c_int { __apply_relocate(s,st,si,rs,m,true) }

pub unsafe fn search_module_dbetables(addr: c_ulong) -> *const exception_table_entry {
    let mut e = core::ptr::null(); let mut dbe: *mut mod_arch_specific;
    spin_lock_irqsave(&mut dbe_lock, core::ptr::null_mut());
    list_for_each_entry!(dbe, &mut dbe_list, dbe_list) { e = search_extable((*dbe).dbe_start, (*dbe).dbe_end - (*dbe).dbe_start, addr); if !e.is_null() { break; } }
    spin_unlock_irqrestore(&mut dbe_lock, core::ptr::null_mut()); e
}

pub unsafe fn module_finalize(hdr: *const Elf_Ehdr, sechdrs: *const Elf_Shdr, me: *mut module) -> c_int {
    if IS_ENABLED!(CONFIG_JUMP_LABEL) { jump_label_apply_nops(me); }
    INIT_LIST_HEAD!(&mut (*me).arch.dbe_list);
    for i in 0..(*hdr).e_shnum as usize { let s = sechdrs.add(i); if strcmp(b"__dbe_table\0".as_ptr() as *const c_char, (hdr as *const u8).add((*sechdrs.add((*hdr).e_shstrndx as usize)).sh_offset as usize).add((*s).sh_name as usize) as *const c_char) != 0 { continue; } (*me).arch.dbe_start = (*s).sh_addr as *mut _; (*me).arch.dbe_end = ((*s).sh_addr + (*s).sh_size) as *mut _; spin_lock_irq(&mut dbe_lock); list_add(&mut (*me).arch.dbe_list, &mut dbe_list); spin_unlock_irq(&mut dbe_lock); } 0
}

pub unsafe fn module_arch_cleanup(mod_: *mut module) { spin_lock_irq(&mut dbe_lock); list_del(&mut (*mod_).arch.dbe_list); spin_unlock_irq(&mut dbe_lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
