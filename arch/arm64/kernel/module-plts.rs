// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014-2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

unsafe fn __get_adrp_add_pair(dst: u64, pc: u64, reg: enum_aarch64_insn_register) -> plt_entry {
    let adrp = aarch64_insn_gen_adr(pc, dst, reg, AARCH64_INSN_ADR_TYPE_ADRP);
    let add = aarch64_insn_gen_add_sub_imm(
        reg, reg, dst % SZ_4K, AARCH64_INSN_VARIANT_64BIT, AARCH64_INSN_ADSB_ADD,
    );

    plt_entry { adrp: cpu_to_le32(adrp), add: cpu_to_le32(add), br: 0 }
}

pub unsafe fn get_plt_entry(dst: u64, pc: *mut core::ffi::c_void) -> plt_entry {
    static mut BR: u32 = 0;
    let mut plt: plt_entry;

    if BR == 0 {
        BR = aarch64_insn_gen_branch_reg(AARCH64_INSN_REG_16, AARCH64_INSN_BRANCH_NOLINK);
    }

    plt = __get_adrp_add_pair(dst, pc as u64, AARCH64_INSN_REG_16);
    plt.br = cpu_to_le32(BR);
    plt
}

unsafe fn plt_entries_equal(a: *const plt_entry, b: *const plt_entry) -> bool {
    let p: u64;
    let q: u64;

    if (*a).add != (*b).add || (*a).br != (*b).br {
        return false;
    }

    p = ((*a as u64) & !(SZ_4K - 1));
    q = ((*b as u64) & !(SZ_4K - 1));

    if (*a).adrp == (*b).adrp && p == q {
        return true;
    }

    (p + aarch64_insn_adrp_get_offset(le32_to_cpu((*a).adrp))) ==
        (q + aarch64_insn_adrp_get_offset(le32_to_cpu((*b).adrp)))
}

pub unsafe fn module_emit_plt_entry(
    mod_: *mut module,
    sechdrs: *mut Elf64_Shdr,
    loc: *mut core::ffi::c_void,
    rela: *const Elf64_Rela,
    sym: *mut Elf64_Sym,
) -> u64 {
    let pltsec = if !within_module_init(loc as usize, mod_) {
        &mut (*mod_).arch.core
    } else {
        &mut (*mod_).arch.init
    };
    let plt = (*sechdrs.add(pltsec.plt_shndx as usize)).sh_addr as *mut plt_entry;
    let mut i = pltsec.plt_num_entries as isize;
    let j = i - 1;
    let val = (*sym).st_value + (*rela).r_addend as u64;

    if is_forbidden_offset_for_adrp(&(*plt.offset(i)).adrp) {
        i += 1;
    }

    *plt.offset(i) = get_plt_entry(val, plt.offset(i) as *mut core::ffi::c_void);

    if j >= 0 && plt_entries_equal(plt.offset(i), plt.offset(j)) {
        return plt.offset(j) as u64;
    }

    pltsec.plt_num_entries += (i - j) as _;
    if WARN_ON(pltsec.plt_num_entries > pltsec.plt_max_entries) {
        return 0;
    }
    plt.offset(i) as u64
}

// CONFIG_ARM64_ERRATUM_843419
#[cfg(CONFIG_ARM64_ERRATUM_843419)]
pub unsafe fn module_emit_veneer_for_adrp(
    mod_: *mut module, sechdrs: *mut Elf64_Shdr, loc: *mut core::ffi::c_void, val: u64,
) -> u64 {
    let pltsec = if !within_module_init(loc as usize, mod_) { &mut (*mod_).arch.core } else { &mut (*mod_).arch.init };
    let plt = (*sechdrs.add(pltsec.plt_shndx as usize)).sh_addr as *mut plt_entry;
    let mut i = pltsec.plt_num_entries as isize;
    pltsec.plt_num_entries += 1;
    if WARN_ON(pltsec.plt_num_entries > pltsec.plt_max_entries) { return 0; }
    if is_forbidden_offset_for_adrp(&(*plt.offset(i)).adrp) { i = pltsec.plt_num_entries as isize; pltsec.plt_num_entries += 1; }
    let rd = aarch64_insn_decode_register(AARCH64_INSN_REGTYPE_RD, le32_to_cpup(loc as *const __le32));
    let br = aarch64_insn_gen_branch_imm(&mut (*plt.offset(i)).br as *mut _ as u64, loc as u64 + 4, AARCH64_INSN_BRANCH_NOLINK);
    *plt.offset(i) = __get_adrp_add_pair(val, &*plt.offset(i) as *const _ as u64, rd);
    (*plt.offset(i)).br = cpu_to_le32(br);
    plt.offset(i) as u64
}

fn cmp_3way<T: PartialOrd>(a: T, b: T) -> i32 { if a < b { -1 } else if a > b { 1 } else { 0 } }

unsafe fn cmp_rela(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let x = a as *const Elf64_Rela; let y = b as *const Elf64_Rela;
    let mut i = cmp_3way(ELF64_R_TYPE((*x).r_info), ELF64_R_TYPE((*y).r_info));
    if i == 0 { i = cmp_3way(ELF64_R_SYM((*x).r_info), ELF64_R_SYM((*y).r_info)); }
    if i == 0 { i = cmp_3way((*x).r_addend, (*y).r_addend); }
    i
}

unsafe fn duplicate_rel(rela: *const Elf64_Rela, num: isize) -> bool {
    num > 0 && cmp_rela(rela.offset(num) as *const _, rela.offset(num - 1) as *const _) == 0
}

unsafe fn count_plts(syms: *mut Elf64_Sym, rela: *mut Elf64_Rela, num: i32, dstidx: Elf64_Word, dstsec: *mut Elf64_Shdr) -> u32 {
    let mut ret = 0u32;
    for i in 0..num as isize {
        let r = &*rela.offset(i);
        match ELF64_R_TYPE(r.r_info) {
            R_AARCH64_JUMP26 | R_AARCH64_CALL26 => {
                let s = &*syms.add(ELF64_R_SYM(r.r_info) as usize);
                if s.st_shndx == dstidx { break; }
                if r.r_addend != 0 || !duplicate_rel(rela, i) { ret += 1; }
            }
            R_AARCH64_ADR_PREL_PG_HI21_NC | R_AARCH64_ADR_PREL_PG_HI21 => {
                if !cpus_have_final_cap(ARM64_WORKAROUND_843419) { continue; }
                let min_align = 2u64 << ffz(r.r_offset | 0x7);
                if min_align > SZ_4K { ret += 1; } else { (*dstsec).sh_addralign = max((*dstsec).sh_addralign, min_align); }
            }
            _ => {}
        }
    }
    if cpus_have_final_cap(ARM64_WORKAROUND_843419) { ret += DIV_ROUND_UP(ret, SZ_4K / core::mem::size_of::<plt_entry>()); }
    ret
}

unsafe fn branch_rela_needs_plt(syms: *mut Elf64_Sym, rela: *mut Elf64_Rela, dstidx: Elf64_Word) -> bool {
    let s = &*syms.add(ELF64_R_SYM((*rela).r_info) as usize);
    s.st_shndx != dstidx && (ELF64_R_TYPE((*rela).r_info) == R_AARCH64_JUMP26 || ELF64_R_TYPE((*rela).r_info) == R_AARCH64_CALL26)
}

unsafe fn partition_branch_plt_relas(syms: *mut Elf64_Sym, rela: *mut Elf64_Rela, numrels: isize, dstidx: Elf64_Word) -> isize {
    let (mut i, mut j) = (0isize, numrels - 1);
    while i < j { if branch_rela_needs_plt(syms, rela.offset(i), dstidx) { i += 1; } else if branch_rela_needs_plt(syms, rela.offset(j), dstidx) { core::ptr::swap(rela.offset(i), rela.offset(j)); } else { j -= 1; } }
    i
}

pub unsafe fn module_frob_arch_sections(ehdr: *mut Elf_Ehdr, sechdrs: *mut Elf_Shdr, secstrings: *mut i8, mod_: *mut module) -> i32 {
    let (mut core_plts, mut init_plts) = (0usize, 0usize);
    let mut syms: *mut Elf64_Sym = core::ptr::null_mut();
    let (mut tramp, mut init_tramp): (*mut Elf64_Shdr, *mut Elf64_Shdr) = (core::ptr::null_mut(), core::ptr::null_mut());
    for i in 0..(*ehdr).e_shnum as usize {
        let s = sechdrs.add(i);
        if !strcmp(secstrings.add((*s).sh_name as usize), ".plt") { (*mod_).arch.core.plt_shndx = i as _; }
        else if !strcmp(secstrings.add((*s).sh_name as usize), ".init.plt") { (*mod_).arch.init.plt_shndx = i as _; }
        else if !strcmp(secstrings.add((*s).sh_name as usize), ".text.ftrace_trampoline") { tramp = s; }
        else if !strcmp(secstrings.add((*s).sh_name as usize), ".init.text.ftrace_trampoline") { init_tramp = s; }
        else if (*s).sh_type == SHT_SYMTAB { syms = (*s).sh_addr as *mut _; }
    }
    if (*mod_).arch.core.plt_shndx == 0 || (*mod_).arch.init.plt_shndx == 0 { pr_err("%s: module PLT section(s) missing\n", (*mod_).name); return -ENOEXEC; }
    if syms.is_null() { pr_err("%s: module symtab section missing\n", (*mod_).name); return -ENOEXEC; }
    for i in 0..(*ehdr).e_shnum as usize {
        let sec = sechdrs.add(i); if (*sec).sh_type != SHT_RELA { continue; }
        let dstsec = sechdrs.add((*sec).sh_info as usize); if (*dstsec).sh_flags & SHF_EXECINSTR == 0 { continue; }
        let rels = (ehdr as *mut u8).add((*sec).sh_offset as usize) as *mut Elf64_Rela;
        let numrels = ((*sec).sh_size as usize / core::mem::size_of::<Elf64_Rela>()) as isize;
        let nents = partition_branch_plt_relas(syms, rels, numrels, (*sec).sh_info);
        if nents != 0 { sort(rels, nents as usize, core::mem::size_of::<Elf64_Rela>(), cmp_rela, core::ptr::null_mut()); }
        if !module_init_layout_section(secstrings.add((*dstsec).sh_name as usize)) { core_plts += count_plts(syms, rels, numrels as i32, (*sec).sh_info, dstsec) as usize; } else { init_plts += count_plts(syms, rels, numrels as i32, (*sec).sh_info, dstsec) as usize; }
    }
    let p = sechdrs.add((*mod_).arch.core.plt_shndx as usize); (*p).sh_type = SHT_NOBITS; (*p).sh_flags = SHF_EXECINSTR | SHF_ALLOC; (*p).sh_addralign = L1_CACHE_BYTES; (*p).sh_size = (core_plts + 1) * core::mem::size_of::<plt_entry>(); (*mod_).arch.core.plt_num_entries = 0; (*mod_).arch.core.plt_max_entries = core_plts;
    let p = sechdrs.add((*mod_).arch.init.plt_shndx as usize); (*p).sh_type = SHT_NOBITS; (*p).sh_flags = SHF_EXECINSTR | SHF_ALLOC; (*p).sh_addralign = L1_CACHE_BYTES; (*p).sh_size = (init_plts + 1) * core::mem::size_of::<plt_entry>(); (*mod_).arch.init.plt_num_entries = 0; (*mod_).arch.init.plt_max_entries = init_plts;
    if !tramp.is_null() { (*tramp).sh_type = SHT_NOBITS; (*tramp).sh_flags = SHF_EXECINSTR | SHF_ALLOC; (*tramp).sh_addralign = core::mem::align_of::<plt_entry>(); (*tramp).sh_size = NR_FTRACE_PLTS * core::mem::size_of::<plt_entry>(); }
    if !init_tramp.is_null() { (*init_tramp).sh_type = SHT_NOBITS; (*init_tramp).sh_flags = SHF_EXECINSTR | SHF_ALLOC; (*init_tramp).sh_addralign = core::mem::align_of::<plt_entry>(); (*init_tramp).sh_size = NR_FTRACE_PLTS * core::mem::size_of::<plt_entry>(); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
