/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2017 Andes Technology Corporation */

// Translated from the C header. Names supplied by the included headers remain
// external dependencies.

use core::ffi::c_char;

extern "C" {
    pub fn module_emit_got_entry(mod_: *mut module, val: c_ulong) -> c_ulong;
    pub fn module_emit_plt_entry(mod_: *mut module, val: c_ulong) -> c_ulong;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

pub type c_ulong = usize;
pub type c_int = i32;
pub type u32 = core::primitive::u32;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Shdr {
    pub sh_name: u32,
    pub sh_addr: c_ulong,
    pub sh_offset: c_ulong,
}

#[repr(C)]
pub struct Elf_Ehdr {
    pub e_shstrndx: u16,
    pub e_shnum: u16,
}

#[cfg(CONFIG_MODULE_SECTIONS)]
#[repr(C)]
pub struct mod_section {
    pub shdr: *mut Elf_Shdr,
    pub num_entries: c_int,
    pub max_entries: c_int,
}

#[cfg(CONFIG_MODULE_SECTIONS)]
#[repr(C)]
pub struct mod_arch_specific {
    pub got: mod_section,
    pub plt: mod_section,
    pub got_plt: mod_section,
}

#[cfg(CONFIG_MODULE_SECTIONS)]
#[repr(C)]
pub struct got_entry {
    pub symbol_addr: c_ulong,
}

#[cfg(CONFIG_MODULE_SECTIONS)]
#[inline]
pub fn emit_got_entry(val: c_ulong) -> got_entry {
    got_entry { symbol_addr: val }
}

#[cfg(CONFIG_MODULE_SECTIONS)]
#[inline]
pub unsafe fn get_got_entry(val: c_ulong, sec: *const mod_section) -> *mut got_entry {
    let got = (*(*sec).shdr).sh_addr as *mut got_entry;
    let mut i: c_int = 0;
    while i < (*sec).num_entries {
        if (*got.add(i as usize)).symbol_addr == val {
            return got.add(i as usize);
        }
        i += 1;
    }
    core::ptr::null_mut()
}

#[cfg(CONFIG_MODULE_SECTIONS)]
#[repr(C)]
pub struct plt_entry {
    /* Trampoline code to real target address. The return address
     * should be the original (pc+4) before entring plt entry. */
    pub insn_auipc: u32,
    pub insn_ld: u32,
    pub insn_jr: u32,
}

#[cfg(CONFIG_MODULE_SECTIONS)]
pub const OPC_AUIPC: u32 = 0x0017;
#[cfg(CONFIG_MODULE_SECTIONS)]
pub const OPC_LD: u32 = 0x3003;
#[cfg(CONFIG_MODULE_SECTIONS)]
pub const OPC_JALR: u32 = 0x0067;
#[cfg(CONFIG_MODULE_SECTIONS)]
pub const REG_T0: u32 = 0x5;
#[cfg(CONFIG_MODULE_SECTIONS)]
pub const REG_T1: u32 = 0x6;

#[cfg(CONFIG_MODULE_SECTIONS)]
#[inline]
pub fn emit_plt_entry(val: c_ulong, plt: c_ulong, got_plt: c_ulong) -> plt_entry {
    let offset = got_plt.wrapping_sub(plt);
    let hi20 = (offset.wrapping_add(0x800) & 0xfffff000) as u32;
    let lo12 = offset.wrapping_sub(hi20 as c_ulong) as u32;
    plt_entry {
        insn_auipc: OPC_AUIPC | (REG_T0 << 7) | hi20,
        insn_ld: OPC_LD | (lo12 << 20) | (REG_T0 << 15) | (REG_T1 << 7),
        insn_jr: OPC_JALR | (REG_T1 << 15),
    }
}

#[cfg(CONFIG_MODULE_SECTIONS)]
#[inline]
pub unsafe fn get_got_plt_idx(val: c_ulong, sec: *const mod_section) -> c_int {
    let got_plt = (*(*sec).shdr).sh_addr as *mut got_entry;
    let mut i: c_int = 0;
    while i < (*sec).num_entries {
        if (*got_plt.add(i as usize)).symbol_addr == val {
            return i;
        }
        i += 1;
    }
    -1
}

#[cfg(CONFIG_MODULE_SECTIONS)]
#[inline]
pub unsafe fn get_plt_entry(
    val: c_ulong,
    sec_plt: *const mod_section,
    sec_got_plt: *const mod_section,
) -> *mut plt_entry {
    let plt = (*(*sec_plt).shdr).sh_addr as *mut plt_entry;
    let got_plt_idx = get_got_plt_idx(val, sec_got_plt);
    if got_plt_idx >= 0 {
        plt.add(got_plt_idx as usize)
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn find_section(
    hdr: *const Elf_Ehdr,
    sechdrs: *const Elf_Shdr,
    name: *const c_char,
) -> *const Elf_Shdr {
    let secstrs = (sechdrs.add((*hdr).e_shstrndx as usize) as *const u8)
        .add((*sechdrs.add((*hdr).e_shstrndx as usize)).sh_offset as usize);
    let mut s = sechdrs;
    let se = sechdrs.add((*hdr).e_shnum as usize);
    while s < se {
        if strcmp(name, secstrs.add((*s).sh_name as usize) as *const c_char) == 0 {
            return s;
        }
        s = s.add(1);
    }
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
