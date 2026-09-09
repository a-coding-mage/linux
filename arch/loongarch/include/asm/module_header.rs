/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by asm/inst.h, asm/orc_types.h, and asm-generic/module.h

pub const RELA_STACK_DEPTH: i32 = 16;

#[repr(C)]
pub struct mod_section {
    pub shndx: i32,
    pub num_entries: i32,
    pub max_entries: i32,
}

#[repr(C)]
pub struct mod_arch_specific {
    pub got: mod_section,
    pub plt: mod_section,
    pub plt_idx: mod_section,

    #[cfg(CONFIG_UNWINDER_ORC)]
    pub num_orcs: u32,
    #[cfg(CONFIG_UNWINDER_ORC)]
    pub orc_unwind_ip: *mut i32,
    #[cfg(CONFIG_UNWINDER_ORC)]
    pub orc_unwind: *mut orc_entry,

    /* For CONFIG_DYNAMIC_FTRACE */
    pub ftrace_trampolines: *mut plt_entry,
}

#[repr(C)]
pub struct got_entry {
    pub symbol_addr: Elf_Addr,
}

#[repr(C)]
pub struct plt_entry {
    pub inst_lu12iw: u32,
    #[cfg(CONFIG_64BIT)]
    pub inst_lu32id: u32,
    #[cfg(CONFIG_64BIT)]
    pub inst_lu52id: u32,
    pub inst_jirl: u32,
}

#[repr(C)]
pub struct plt_idx_entry {
    pub symbol_addr: Elf_Addr,
}

extern "C" {
    pub fn module_emit_got_entry(mod_: *mut r#module, sechdrs: *mut Elf_Shdr, val: Elf_Addr) -> Elf_Addr;
    pub fn module_emit_plt_entry(mod_: *mut r#module, sechdrs: *mut Elf_Shdr, val: Elf_Addr) -> Elf_Addr;
}

#[inline]
pub fn emit_got_entry(val: Elf_Addr) -> got_entry {
    got_entry { symbol_addr: val }
}

#[inline]
pub unsafe fn emit_plt_entry(val: usize) -> plt_entry {
    #[cfg(CONFIG_32BIT)]
    {
        let lu12iw = larch_insn_gen_lu12iw(LOONGARCH_GPR_T1, ADDR_IMM(val, LU12IW));
        let jirl = larch_insn_gen_jirl(0, LOONGARCH_GPR_T1, ADDR_IMM(val, ORI));
        plt_entry { inst_lu12iw: lu12iw, inst_jirl: jirl }
    }
    #[cfg(not(CONFIG_32BIT))]
    {
        let lu12iw = larch_insn_gen_lu12iw(LOONGARCH_GPR_T1, ADDR_IMM(val, LU12IW));
        let lu32id = larch_insn_gen_lu32id(LOONGARCH_GPR_T1, ADDR_IMM(val, LU32ID));
        let lu52id = larch_insn_gen_lu52id(LOONGARCH_GPR_T1, LOONGARCH_GPR_T1, ADDR_IMM(val, LU52ID));
        let jirl = larch_insn_gen_jirl(0, LOONGARCH_GPR_T1, ADDR_IMM(val, ORI));
        plt_entry { inst_lu12iw: lu12iw, inst_lu32id: lu32id, inst_lu52id: lu52id, inst_jirl: jirl }
    }
}

#[inline]
pub fn emit_plt_idx_entry(val: usize) -> plt_idx_entry {
    plt_idx_entry { symbol_addr: val }
}

#[inline]
pub unsafe fn get_plt_idx(val: usize, sechdrs: *mut Elf_Shdr, sec: *const mod_section) -> i32 {
    let sec = &*sec;
    let plt_idx = (*sechdrs.add(sec.shndx as usize)).sh_addr as *mut plt_idx_entry;
    let mut i = 0;
    while i < sec.num_entries {
        if (*plt_idx.add(i as usize)).symbol_addr == val {
            return i;
        }
        i += 1;
    }
    -1
}

#[inline]
pub unsafe fn get_plt_entry(
    val: usize,
    sechdrs: *mut Elf_Shdr,
    sec_plt: *const mod_section,
    sec_plt_idx: *const mod_section,
) -> *mut plt_entry {
    let plt_idx = get_plt_idx(val, sechdrs, sec_plt_idx);
    let plt = (*sechdrs.add((*sec_plt).shndx as usize)).sh_addr as *mut plt_entry;
    if plt_idx < 0 { core::ptr::null_mut() } else { plt.add(plt_idx as usize) }
}

#[inline]
pub unsafe fn get_got_entry(val: Elf_Addr, sechdrs: *mut Elf_Shdr, sec: *const mod_section) -> *mut got_entry {
    let sec = &*sec;
    let got = (*sechdrs.add(sec.shndx as usize)).sh_addr as *mut got_entry;
    let mut i = 0;
    while i < sec.num_entries {
        if (*got.add(i as usize)).symbol_addr == val {
            return got.add(i as usize);
        }
        i += 1;
    }
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
