/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of vdso2c.h.  The original header is included twice from
 * vdso2c.c and BITSFUNC supplies the architecture-specific function prefix.
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

extern "C" {
    fn fprintf(outfile: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fwrite(ptr: *const c_void, size: usize, count: usize, stream: *mut FILE) -> usize;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn fail(format: *const c_char, ... ) -> !;
}

#[repr(C)]
pub struct FILE { _private: [u8; 0] }

/* These architecture-dependent types, constants, and helpers are supplied by
 * the including translation unit, as in the original C header. */
extern "C" {
    static required_syms: [RequiredSym; NSYMS];
}

#[repr(C)]
pub struct RequiredSym {
    pub name: *const c_char,
    pub export: c_int,
}

const NSYMS: usize = 0; // Supplied by the including architecture-specific source.

#[allow(non_snake_case)]
pub unsafe fn BITSFUNC_copy(outfile: *mut FILE, data: *const u8, len: usize) {
    let mut i = 0usize;
    while i < len {
        if i % 10 == 0 {
            fprintf(outfile, b"\n\t\0".as_ptr() as *const c_char);
        }
        fprintf(outfile, b"0x%02X, \0".as_ptr() as *const c_char, *data.add(i) as c_int);
        i += 1;
    }
}

pub unsafe fn BITSFUNC_extract(
    data: *const u8,
    data_len: usize,
    outfile: *mut FILE,
    sec: *mut ElfShdr,
    name: *const c_char,
) {
    let offset = GET_LE(&(*sec).sh_offset) as usize;
    let len = GET_LE(&(*sec).sh_size) as usize;

    if offset + len > data_len {
        fail(b"section to extract overruns input data\0".as_ptr() as *const c_char);
    }

    fprintf(outfile, b"static const unsigned char %s[%zu] = {\0".as_ptr() as *const c_char, name, len);
    BITSFUNC_copy(outfile, data.add(offset), len);
    fprintf(outfile, b"\n};\n\n\0".as_ptr() as *const c_char);
}

pub unsafe fn BITSFUNC_go(
    raw_addr: *mut c_void,
    raw_len: usize,
    stripped_addr: *mut c_void,
    stripped_len: usize,
    outfile: *mut FILE,
    image_name: *const c_char,
) {
    let mut found_load = 0;
    let mut load_size: u64 = u64::MAX;
    let mut mapping_size: u64;
    let hdr = raw_addr as *mut ElfEhdr;
    let mut symtab_hdr: *mut ElfShdr = ptr::null_mut();
    let mut strtab_hdr: *mut ElfShdr;
    let mut secstrings_hdr: *mut ElfShdr;
    let mut alt_sec: *mut ElfShdr = ptr::null_mut();
    let mut extable_sec: *mut ElfShdr = ptr::null_mut();
    let mut dyn_: *mut ElfDyn = ptr::null_mut();
    let mut dyn_end: *mut ElfDyn = ptr::null_mut();
    let mut i: u64;
    let mut syms_nr: u64;
    let secstrings: *const c_char;
    let mut syms = [0i64; NSYMS];
    let pt = (raw_addr as *mut u8).add(GET_LE(&(*hdr).e_phoff) as usize) as *mut ElfPhdr;

    if GET_LE(&(*hdr).e_type) != ET_DYN { fail(b"input is not a shared object\n\0".as_ptr() as *const c_char); }
    i = 0;
    while i < GET_LE(&(*hdr).e_phnum) as u64 {
        if GET_LE(&(*pt.add(i as usize)).p_type) == PT_LOAD {
            if found_load != 0 { fail(b"multiple PT_LOAD segs\n\0".as_ptr() as *const c_char); }
            if GET_LE(&(*pt.add(i as usize)).p_offset) != 0 || GET_LE(&(*pt.add(i as usize)).p_vaddr) != 0 { fail(b"PT_LOAD in wrong place\n\0".as_ptr() as *const c_char); }
            if GET_LE(&(*pt.add(i as usize)).p_memsz) != GET_LE(&(*pt.add(i as usize)).p_filesz) { fail(b"cannot handle memsz != filesz\n\0".as_ptr() as *const c_char); }
            load_size = GET_LE(&(*pt.add(i as usize)).p_memsz); found_load = 1;
        } else if GET_LE(&(*pt.add(i as usize)).p_type) == PT_DYNAMIC {
            dyn_ = (raw_addr as *mut u8).add(GET_LE(&(*pt.add(i as usize)).p_offset) as usize) as *mut ElfDyn;
            dyn_end = (dyn_ as *mut u8).add(GET_LE(&(*pt.add(i as usize)).p_memsz) as usize) as *mut ElfDyn;
        }
        i += 1;
    }
    if found_load == 0 { fail(b"no PT_LOAD seg\n\0".as_ptr() as *const c_char); }
    if stripped_len < load_size as usize { fail(b"stripped input is too short\n\0".as_ptr() as *const c_char); }
    if dyn_.is_null() { fail(b"input has no PT_DYNAMIC section -- your toolchain is buggy\n\0".as_ptr() as *const c_char); }

    i = 0;
    while dyn_.add(i as usize) < dyn_end && GET_LE(&(*dyn_.add(i as usize)).d_tag) != DT_NULL {
        let tag = GET_LE(&(*dyn_.add(i as usize)).d_tag);
        if tag == DT_REL || tag == DT_RELSZ || tag == DT_RELA || tag == DT_RELENT || tag == DT_TEXTREL { fail(b"vdso image contains dynamic relocations\n\0".as_ptr() as *const c_char); }
        i += 1;
    }

    secstrings_hdr = (raw_addr as *mut u8).add((GET_LE(&(*hdr).e_shoff) + GET_LE(&(*hdr).e_shentsize) * GET_LE(&(*hdr).e_shstrndx)) as usize) as *mut ElfShdr;
    secstrings = (raw_addr as *mut u8).add(GET_LE(&(*secstrings_hdr).sh_offset) as usize) as *const c_char;
    i = 0;
    while i < GET_LE(&(*hdr).e_shnum) as u64 {
        let sh = (raw_addr as *mut u8).add((GET_LE(&(*hdr).e_shoff) + GET_LE(&(*hdr).e_shentsize) * i) as usize) as *mut ElfShdr;
        if GET_LE(&(*sh).sh_type) == SHT_SYMTAB { symtab_hdr = sh; }
        if strcmp(secstrings.add(GET_LE(&(*sh).sh_name) as usize), b".altinstructions\0".as_ptr() as *const c_char) == 0 { alt_sec = sh; }
        if strcmp(secstrings.add(GET_LE(&(*sh).sh_name) as usize), b"__ex_table\0".as_ptr() as *const c_char) == 0 { extable_sec = sh; }
        i += 1;
    }
    if symtab_hdr.is_null() { fail(b"no symbol table\n\0".as_ptr() as *const c_char); }
    strtab_hdr = (raw_addr as *mut u8).add((GET_LE(&(*hdr).e_shoff) + GET_LE(&(*hdr).e_shentsize) * GET_LE(&(*symtab_hdr).sh_link)) as usize) as *mut ElfShdr;
    syms_nr = GET_LE(&(*symtab_hdr).sh_size) / GET_LE(&(*symtab_hdr).sh_entsize);
    i = 0;
    while i < syms_nr {
        let sym = (raw_addr as *mut u8).add((GET_LE(&(*symtab_hdr).sh_offset) + GET_LE(&(*symtab_hdr).sh_entsize) * i) as usize) as *mut ElfSym;
        let sym_name = (raw_addr as *mut u8).add((GET_LE(&(*strtab_hdr).sh_offset) + GET_LE(&(*sym).st_name)) as usize) as *const c_char;
        let mut k = 0usize;
        while k < NSYMS {
            if strcmp(sym_name, required_syms[k].name) == 0 {
                if syms[k] != 0 { fail(b"duplicate symbol %s\n\0".as_ptr() as *const c_char, required_syms[k].name); }
                syms[k] = GET_LE(&(*sym).st_value) as i64;
            }
            k += 1;
        }
        i += 1;
    }
    if image_name.is_null() { fwrite(stripped_addr, stripped_len, 1, outfile); return; }
    mapping_size = ((stripped_len as u64 + 4095) / 4096) * 4096;
    fprintf(outfile, b"/* AUTOMATICALLY GENERATED -- DO NOT EDIT */\n\n\0".as_ptr() as *const c_char);
    fprintf(outfile, b"#include <linux/linkage.h>\n#include <linux/init.h>\n#include <asm/page_types.h>\n#include <asm/vdso.h>\n\n\0".as_ptr() as *const c_char);
    fprintf(outfile, b"static unsigned char raw_data[%lu] __ro_after_init __aligned(PAGE_SIZE) = {\0".as_ptr() as *const c_char, mapping_size);
    i = 0;
    while i < stripped_len as u64 {
        if i % 10 == 0 { fprintf(outfile, b"\n\t\0".as_ptr() as *const c_char); }
        fprintf(outfile, b"0x%02X, \0".as_ptr() as *const c_char, *(stripped_addr as *const u8).add(i as usize) as c_int);
        i += 1;
    }
    fprintf(outfile, b"\n};\n\n\0".as_ptr() as *const c_char);
    if !extable_sec.is_null() { BITSFUNC_extract(raw_addr as *const u8, raw_len, outfile, extable_sec, b"extable\0".as_ptr() as *const c_char); }
    fprintf(outfile, b"const struct vdso_image %s = {\n\t.data = raw_data,\n\t.size = %lu,\n\0".as_ptr() as *const c_char, image_name, mapping_size);
    if !alt_sec.is_null() { fprintf(outfile, b"\t.alt = %lu,\n\t.alt_len = %lu,\n\0".as_ptr() as *const c_char, GET_LE(&(*alt_sec).sh_offset), GET_LE(&(*alt_sec).sh_size)); }
    if !extable_sec.is_null() { fprintf(outfile, b"\t.extable_base = %lu,\n\t.extable_len = %lu,\n\t.extable = extable,\n\0".as_ptr() as *const c_char, GET_LE(&(*extable_sec).sh_offset), GET_LE(&(*extable_sec).sh_size)); }
    i = 0;
    while i < NSYMS as u64 { if required_syms[i as usize].export != 0 && syms[i as usize] != 0 { fprintf(outfile, b"\t.sym_%s = %lld,\n\0".as_ptr() as *const c_char, required_syms[i as usize].name, syms[i as usize]); } i += 1; }
    fprintf(outfile, b"};\n\nstatic __init int init_%s(void) {\n\treturn init_vdso_image(&%s);\n};\nsubsys_initcall(init_%s);\n\0".as_ptr() as *const c_char, image_name, image_name, image_name);
    let _ = (raw_len, mapping_size, alt_sec, extable_sec, required_syms);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
