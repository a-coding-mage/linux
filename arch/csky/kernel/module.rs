// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct Elf32_Shdr {
    pub sh_addr: u32,
    pub sh_size: u32,
    pub sh_info: u32,
}

#[repr(C)]
pub struct Elf32_Rela {
    pub r_offset: u32,
    pub r_info: u32,
    pub r_addend: u32,
}

#[repr(C)]
pub struct Elf32_Sym {
    pub st_value: u32,
}

#[repr(C)]
pub struct Module {
    pub name: *const c_char,
}

const R_CSKY_32: u32 = 1;
const R_CSKY_PC32: u32 = 2;
const R_CSKY_PCRELJSR_IMM11BY2: u32 = 3;
const R_CSKY_PCRELJSR_IMM26BY2: u32 = 4;
const R_CSKY_ADDR_HI16: u32 = 5;
const R_CSKY_ADDR_LO16: u32 = 6;

const ENOEXEC: c_int = 8;

#[inline]
fn elf32_r_sym(info: u32) -> u32 {
    info >> 8
}

#[inline]
fn elf32_r_type(info: u32) -> u32 {
    info & 0xff
}

#[cfg(feature = "CONFIG_CPU_CK810")]
unsafe fn jsri_2_lrw_jsr(location: *mut u32) {
    let location_tmp = location as *mut u16;

    if ((*location_tmp as u32) & 0xfc00) == 0xe000 {
        return;
    }

    if *location_tmp == 0xeae0 {
        // jsri 0x...  --> lrw r26, 0x...
        *location_tmp = (*location_tmp & 0xff9f) | 0x001a;
        *location_tmp.add(1) = *location_tmp.add(1) & 0xffff;
        // lsli r0, r0 --> jsr r26
        *(location.add(1) as *mut u16) = 0xe8fa;
        *((location.add(1) as *mut u16).add(1)) = 0x0000;
    }
}

#[cfg(not(feature = "CONFIG_CPU_CK810"))]
#[inline]
unsafe fn jsri_2_lrw_jsr(_location: *mut u32) {}

extern "C" {
    fn pr_err(format: *const c_char, ...);
}

pub unsafe fn apply_relocate_add(
    sechdrs: *mut Elf32_Shdr,
    _strtab: *const c_char,
    symindex: c_uint,
    relsec: c_uint,
    me: *mut Module,
) -> c_int {
    let rel = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf32_Rela;
    let rel_count = (*sechdrs.add(relsec as usize)).sh_size
        / core::mem::size_of::<Elf32_Rela>() as u32;

    for i in 0..rel_count as usize {
        let rela = &*rel.add(i);
        let location = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr
            .wrapping_add(rela.r_offset)) as *mut u32;
        let sym = ((*sechdrs.add(symindex as usize)).sh_addr as *mut Elf32_Sym)
            .add(elf32_r_sym(rela.r_info) as usize);

        match elf32_r_type(rela.r_info) {
            R_CSKY_32 => {
                *location = rela.r_addend.wrapping_add((*sym).st_value);
            }
            R_CSKY_PC32 => {
                *location = rela.r_addend
                    .wrapping_add((*sym).st_value)
                    .wrapping_sub(location as u32);
            }
            R_CSKY_PCRELJSR_IMM11BY2 => {}
            R_CSKY_PCRELJSR_IMM26BY2 => {
                jsri_2_lrw_jsr(location);
            }
            R_CSKY_ADDR_HI16 => {
                let temp = (location as *mut i16).add(1);
                *temp = ((rela.r_addend.wrapping_add((*sym).st_value) >> 16) as i16);
            }
            R_CSKY_ADDR_LO16 => {
                let temp = (location as *mut i16).add(1);
                *temp = ((rela.r_addend.wrapping_add((*sym).st_value) & 0xffff) as i16);
            }
            _ => {
                pr_err(core::ptr::null(), (*me).name, elf32_r_type(rela.r_info));
                return -ENOEXEC;
            }
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
