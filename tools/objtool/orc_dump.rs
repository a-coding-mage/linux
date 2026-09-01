// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Josh Poimboeuf <jpoimboe@redhat.com>
 */

// C dependencies originally included:
// <unistd.h>
// <asm/orc_types.h>
// <objtool/objtool.h>
// <objtool/orc.h>
// <objtool/warn.h>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

pub type size_t = usize;
pub type Elf64_Addr = u64;
pub type Elf64_Off = u64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;
pub type Elf64_Word = u32;
pub type Elf64_Half = u16;
pub type Elf64_Section = u16;

pub const O_RDONLY: c_int = 0;
pub const EV_CURRENT: c_uint = 1;
pub const ELF_C_READ_MMAP: c_uint = 5;
pub const STT_SECTION: u8 = 3;

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Scn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Data {
    pub d_buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Shdr {
    pub sh_name: Elf64_Word,
    pub sh_type: Elf64_Word,
    pub sh_flags: Elf64_Xword,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: Elf64_Off,
    pub sh_size: Elf64_Xword,
    pub sh_link: Elf64_Word,
    pub sh_info: Elf64_Word,
    pub sh_addralign: Elf64_Xword,
    pub sh_entsize: Elf64_Xword,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Rela {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
    pub r_addend: Elf64_Sxword,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Sym {
    pub st_name: Elf64_Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Elf64_Section,
    pub st_value: Elf64_Addr,
    pub st_size: Elf64_Xword,
}

#[repr(C)]
pub struct orc_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elf {
    pub ehdr: GElf_Ehdr,
}

impl Default for elf {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[inline]
pub const fn GELF_R_SYM(info: Elf64_Xword) -> Elf64_Xword {
    info >> 32
}

#[inline]
pub const fn GELF_ST_TYPE(info: u8) -> u8 {
    info & 0xf
}

unsafe extern "C" {
    fn elf_version(version: c_uint) -> c_uint;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn elf_begin(fd: c_int, cmd: c_uint, ref_: *mut Elf) -> *mut Elf;
    fn elf64_getehdr(elf: *mut Elf) -> *mut GElf_Ehdr;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn elf_getshdrnum(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn elf_getshdrstrndx(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn elf_getscn(elf: *mut Elf, index: size_t) -> *mut Elf_Scn;
    fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr;
    fn elf_strptr(elf: *mut Elf, section: size_t, offset: size_t) -> *mut c_char;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn gelf_getrela(data: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Rela) -> *mut GElf_Rela;
    fn gelf_getsym(data: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Sym) -> *mut GElf_Sym;
    fn printf(format: *const c_char, ...) -> c_int;
    fn orc_print_dump(elf: *mut elf, orc: *mut orc_entry, i: c_int);
    fn elf_end(elf: *mut Elf) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn ERROR_ELF(msg: *const c_char);
    fn ERROR(msg: *const c_char);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn orc_dump(filename: *const c_char) -> c_int {
    let mut fd: c_int;
    let mut nr_entries: c_int;
    let mut i: c_int;
    let mut orc_ip: *mut c_int = ptr::null_mut();
    let mut orc_size: c_int = 0;
    let mut orc: *mut orc_entry = ptr::null_mut();
    let mut name: *mut c_char;
    let mut nr_sections: size_t;
    let mut orc_ip_addr: Elf64_Addr = 0;
    let mut shstrtab_idx: size_t;
    let mut strtab_idx: size_t = 0;
    let mut elf: *mut Elf;
    let mut scn: *mut Elf_Scn;
    let mut sh: GElf_Shdr = core::mem::zeroed();
    let mut rela: GElf_Rela = core::mem::zeroed();
    let mut sym: GElf_Sym = core::mem::zeroed();
    let mut data: *mut Elf_Data;
    let mut symtab: *mut Elf_Data = ptr::null_mut();
    let mut rela_orc_ip: *mut Elf_Data = ptr::null_mut();
    let mut dummy_elf: elf = elf::default();

    elf_version(EV_CURRENT);

    fd = open(filename, O_RDONLY);
    if fd == -1 {
        perror(c"open".as_ptr());
        return -1;
    }

    elf = elf_begin(fd, ELF_C_READ_MMAP, ptr::null_mut());
    if elf.is_null() {
        ERROR_ELF(c"elf_begin".as_ptr());
        return -1;
    }

    if elf64_getehdr(elf).is_null() {
        ERROR_ELF(c"elf64_getehdr".as_ptr());
        return -1;
    }
    memcpy(
        &mut dummy_elf.ehdr as *mut GElf_Ehdr as *mut c_void,
        elf64_getehdr(elf) as *const c_void,
        size_of::<GElf_Ehdr>(),
    );

    if elf_getshdrnum(elf, &mut nr_sections) != 0 {
        ERROR_ELF(c"elf_getshdrnum".as_ptr());
        return -1;
    }

    if elf_getshdrstrndx(elf, &mut shstrtab_idx) != 0 {
        ERROR_ELF(c"elf_getshdrstrndx".as_ptr());
        return -1;
    }

    i = 0;
    while (i as size_t) < nr_sections {
        scn = elf_getscn(elf, i as size_t);
        if scn.is_null() {
            ERROR_ELF(c"elf_getscn".as_ptr());
            return -1;
        }

        if gelf_getshdr(scn, &mut sh).is_null() {
            ERROR_ELF(c"gelf_getshdr".as_ptr());
            return -1;
        }

        name = elf_strptr(elf, shstrtab_idx, sh.sh_name as size_t);
        if name.is_null() {
            ERROR_ELF(c"elf_strptr".as_ptr());
            return -1;
        }

        data = elf_getdata(scn, ptr::null_mut());
        if data.is_null() {
            ERROR_ELF(c"elf_getdata".as_ptr());
            return -1;
        }

        if strcmp(name, c".symtab".as_ptr()) == 0 {
            symtab = data;
        } else if strcmp(name, c".strtab".as_ptr()) == 0 {
            strtab_idx = i as size_t;
        } else if strcmp(name, c".orc_unwind".as_ptr()) == 0 {
            orc = (*data).d_buf as *mut orc_entry;
            orc_size = sh.sh_size as c_int;
        } else if strcmp(name, c".orc_unwind_ip".as_ptr()) == 0 {
            orc_ip = (*data).d_buf as *mut c_int;
            orc_ip_addr = sh.sh_addr;
        } else if strcmp(name, c".rela.orc_unwind_ip".as_ptr()) == 0 {
            rela_orc_ip = data;
        }

        i += 1;
    }

    if symtab.is_null() || strtab_idx == 0 || orc.is_null() || orc_ip.is_null() {
        return 0;
    }

    if orc_size as usize % size_of::<orc_entry>() != 0 {
        ERROR(c"bad .orc_unwind section size".as_ptr());
        return -1;
    }

    nr_entries = (orc_size as usize / size_of::<orc_entry>()) as c_int;
    i = 0;
    while i < nr_entries {
        if !rela_orc_ip.is_null() {
            if gelf_getrela(rela_orc_ip, i, &mut rela).is_null() {
                ERROR_ELF(c"gelf_getrela".as_ptr());
                return -1;
            }

            if gelf_getsym(symtab, GELF_R_SYM(rela.r_info) as c_int, &mut sym).is_null() {
                ERROR_ELF(c"gelf_getsym".as_ptr());
                return -1;
            }

            if GELF_ST_TYPE(sym.st_info) == STT_SECTION {
                scn = elf_getscn(elf, sym.st_shndx as size_t);
                if scn.is_null() {
                    ERROR_ELF(c"elf_getscn".as_ptr());
                    return -1;
                }

                if gelf_getshdr(scn, &mut sh).is_null() {
                    ERROR_ELF(c"gelf_getshdr".as_ptr());
                    return -1;
                }

                name = elf_strptr(elf, shstrtab_idx, sh.sh_name as size_t);
                if name.is_null() {
                    ERROR_ELF(c"elf_strptr".as_ptr());
                    return -1;
                }
            } else {
                name = elf_strptr(elf, strtab_idx, sym.st_name as size_t);
                if name.is_null() {
                    ERROR_ELF(c"elf_strptr".as_ptr());
                    return -1;
                }
            }

            printf(
                c"%s+%llx:".as_ptr(),
                name,
                rela.r_addend as c_ulonglong,
            );
        } else {
            printf(
                c"%llx:".as_ptr(),
                (orc_ip_addr
                    .wrapping_add((i as usize).wrapping_mul(size_of::<c_int>()) as Elf64_Addr)
                    .wrapping_add(*orc_ip.add(i as usize) as Elf64_Addr)) as c_ulonglong,
            );
        }

        orc_print_dump(&mut dummy_elf, orc, i);
        i += 1;
    }

    elf_end(elf);
    close(fd);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
