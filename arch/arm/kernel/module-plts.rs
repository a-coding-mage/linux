// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014-2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

// Dependencies supplied by the kernel's ARM and module-loader interfaces are
// intentionally referenced here rather than redefined.

#[cfg(feature = "CONFIG_THUMB2_KERNEL")]
const PLT_ENT_LDR: u32 = __opcode_to_mem_thumb32(0xf8dff000 | (PLT_ENT_STRIDE - 4));
#[cfg(not(feature = "CONFIG_THUMB2_KERNEL"))]
const PLT_ENT_LDR: u32 = __opcode_to_mem_arm(0xe59ff000 | (PLT_ENT_STRIDE - 8));

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
static FIXED_PLTS: [u32; 2] = [FTRACE_ADDR, MCOUNT_ADDR];
#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
static FIXED_PLTS: [u32; 0] = [];

unsafe fn prealloc_fixed(pltsec: *mut mod_plt_sec, plt: *mut plt_entries) {
    if FIXED_PLTS.is_empty() || (*pltsec).plt_count != 0 { return; }
    (*pltsec).plt_count = FIXED_PLTS.len();
    for i in 0..(*plt).ldr.len() { (*plt).ldr[i] = PLT_ENT_LDR; }
    // BUILD_BUG_ON(sizeof(fixed_plts) > sizeof(plt->lit));
    core::ptr::copy_nonoverlapping(FIXED_PLTS.as_ptr(), (*plt).lit.as_mut_ptr(), FIXED_PLTS.len());
}

unsafe fn get_module_plt(mod_: *mut module, loc: usize, val: Elf32_Addr) -> u32 {
    let pltsec = if !within_module_init(loc, mod_) { &mut (*mod_).arch.core } else { &mut (*mod_).arch.init };
    if pltsec.plt_ent.is_null() { pltsec.plt_ent = pltsec.plt.sh_addr as *mut plt_entries; }
    let mut plt = pltsec.plt_ent;
    prealloc_fixed(pltsec, plt);
    for idx in 0..FIXED_PLTS.len() { if (*plt).lit[idx] == val { return (&(*plt).ldr[idx] as *const _ as usize) as u32; } }
    let mut idx = 0usize;
    if pltsec.plt_count > 0 {
        plt = plt.add((pltsec.plt_count - 1) / PLT_ENT_COUNT);
        idx = (pltsec.plt_count - 1) % PLT_ENT_COUNT;
        if (*plt).lit[idx] == val { return (&(*plt).ldr[idx] as *const _ as usize) as u32; }
        idx = (idx + 1) % PLT_ENT_COUNT;
        if idx == 0 { plt = plt.add(1); }
    }
    pltsec.plt_count += 1;
    BUG_ON(pltsec.plt_count * PLT_ENT_SIZE > pltsec.plt.sh_size);
    if idx == 0 {
        (*plt).ldr = [PLT_ENT_LDR; PLT_ENT_COUNT];
        (*plt).lit[0] = val;
    } else { (*plt).lit[idx] = val; }
    (&(*plt).ldr[idx] as *const _ as usize) as u32
}

#[inline]
fn cmp_3way<T: Ord>(a: T, b: T) -> i32 { if a < b { -1 } else if a > b { 1 } else { 0 } }

unsafe fn cmp_rel(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let x = a as *const Elf32_Rel; let y = b as *const Elf32_Rel;
    let mut i = cmp_3way(ELF32_R_TYPE((*x).r_info), ELF32_R_TYPE((*y).r_info));
    if i == 0 { i = cmp_3way(ELF32_R_SYM((*x).r_info), ELF32_R_SYM((*y).r_info)); } i
}

unsafe fn is_zero_addend_relocation(base: Elf32_Addr, rel: *const Elf32_Rel) -> bool {
    let tval = (base + (*rel).r_offset) as *mut u32;
    match ELF32_R_TYPE((*rel).r_info) {
        R_ARM_THM_CALL | R_ARM_THM_JUMP24 => {
            let upper = __mem_to_opcode_thumb16(*(tval as *const u16));
            let lower = __mem_to_opcode_thumb16(*((tval as *const u16).add(1)));
            (upper & 0x7ff) == 0x7ff && (lower & 0x2fff) == 0x2ffe
        }
        R_ARM_CALL | R_ARM_PC24 | R_ARM_JUMP24 => (__mem_to_opcode_arm(*tval) & 0xffffff) == 0xfffffe,
        _ => { BUG(); false }
    }
}

unsafe fn duplicate_rel(base: Elf32_Addr, rel: *const Elf32_Rel, num: i32) -> bool {
    if num == 0 { return false; }
    let prev = rel.add((num - 1) as usize);
    cmp_rel(rel.add(num as usize) as _, prev as _) == 0 && is_zero_addend_relocation(base, prev)
}

unsafe fn count_plts(syms: *const Elf32_Sym, base: Elf32_Addr, rel: *const Elf32_Rel, num: i32, dstidx: Elf32_Word) -> u32 {
    let mut ret = 0; for i in 0..num {
        match ELF32_R_TYPE((*rel.add(i as usize)).r_info) {
            R_ARM_CALL | R_ARM_PC24 | R_ARM_JUMP24 | R_ARM_THM_CALL | R_ARM_THM_JUMP24 => {
                let s = syms.add(ELF32_R_SYM((*rel.add(i as usize)).r_info) as usize);
                if (*s).st_shndx == dstidx { continue; }
                if !is_zero_addend_relocation(base, rel.add(i as usize)) || !duplicate_rel(base, rel, i) { ret += 1; }
            }, _ => {}
        }
    } ret
}

// The section-layout routine and PLT-range predicate retain the kernel ABI
// structures and helpers supplied by the surrounding translation unit.
unsafe fn module_frob_arch_sections(ehdr: *mut Elf_Ehdr, sechdrs: *mut Elf_Shdr, secstrings: *mut i8, mod_: *mut module) -> i32 {
    let mut core_plts = FIXED_PLTS.len(); let mut init_plts = FIXED_PLTS.len();
    let end = sechdrs.add((*ehdr).e_shnum as usize); let mut syms: *mut Elf32_Sym = core::ptr::null_mut();
    let mut s = sechdrs;
    while s < end { if strcmp(b".plt\0".as_ptr() as _, secstrings.add((*s).sh_name as usize)) == 0 { (*mod_).arch.core.plt = s; } else if strcmp(b".init.plt\0".as_ptr() as _, secstrings.add((*s).sh_name as usize)) == 0 { (*mod_).arch.init.plt = s; } else if (*s).sh_type == SHT_SYMTAB { syms = (*s).sh_addr as *mut Elf32_Sym; } s = s.add(1); }
    if (*mod_).arch.core.plt.is_null() || (*mod_).arch.init.plt.is_null() || syms.is_null() { return -ENOEXEC; }
    s = sechdrs.add(1); while s < end { if (*s).sh_type == SHT_REL { let rels = ((ehdr as *mut u8).add((*s).sh_offset as usize)) as *mut Elf32_Rel; let n = ((*s).sh_size as usize / core::mem::size_of::<Elf32_Rel>()) as i32; let dst = sechdrs.add((*s).sh_info as usize); if (*dst).sh_flags & SHF_EXECINSTR != 0 { sort(rels as _, n as _, core::mem::size_of::<Elf32_Rel>(), cmp_rel, core::ptr::null_mut()); if !module_init_layout_section(secstrings.add((*dst).sh_name as usize)) { core_plts += count_plts(syms, (*dst).sh_addr, rels, n, (*s).sh_info) as usize; } else { init_plts += count_plts(syms, (*dst).sh_addr, rels, n, (*s).sh_info) as usize; } } } s = s.add(1); }
    for (p, n) in [(&mut (*mod_).arch.core, core_plts), (&mut (*mod_).arch.init, init_plts)] { (*p.plt).sh_type = SHT_NOBITS; (*p.plt).sh_flags = SHF_EXECINSTR | SHF_ALLOC; (*p.plt).sh_addralign = L1_CACHE_BYTES; (*p.plt).sh_size = round_up(n * PLT_ENT_SIZE, core::mem::size_of::<plt_entries>()); p.plt_count = 0; p.plt_ent = core::ptr::null_mut(); }
    0
}

unsafe fn in_module_plt(loc: usize) -> bool { let mod_ = __module_text_address(loc); !mod_.is_null() && (loc - (*mod_).arch.core.plt_ent as usize < (*mod_).arch.core.plt_count * PLT_ENT_SIZE || loc - (*mod_).arch.init.plt_ent as usize < (*mod_).arch.init.plt_count * PLT_ENT_SIZE) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
