/*
 * Copyright (c) 2017 Oracle and/or its affiliates. All rights reserved.
 */

/*
 * This file is included up to twice from vdso2c.c.  It generates code for
 * 32-bit and 64-bit vDSOs.  We will eventually need both for 64-bit builds,
 * since 32-bit vDSOs will then be built for 32-bit userspace.
 *
 * The ELF types, constants, byte-order accessors, and C stdio helpers below
 * are supplied by the surrounding translation unit.
 */

extern "C" {
    fn fail(message: *const u8) -> !;
    fn fwrite(ptr: *const core::ffi::c_void, size: usize, count: usize,
              stream: *mut FILE) -> usize;
    fn fprintf(stream: *mut FILE, format: *const u8, ...) -> i32;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

/* BITSFUNC(go) in the C source. */
pub unsafe fn BITSFUNC_go(
    raw_addr: *mut u8,
    raw_len: usize,
    stripped_addr: *mut u8,
    stripped_len: usize,
    outfile: *mut FILE,
    name: *const u8,
) {
    let mut found_load: i32 = 0;
    let mut load_size: usize = usize::MAX; /* Work around bogus warning */
    let mut mapping_size: usize;
    let mut i: usize;
    let mut j: usize;
    let mut symtab_hdr: *mut ELF_Shdr = core::ptr::null_mut();
    let hdr: *mut ELF_Ehdr = raw_addr as *mut ELF_Ehdr;
    let mut dyn_: *mut ELF_Dyn = core::ptr::null_mut();
    let mut dyn_end: *mut ELF_Dyn = core::ptr::null_mut();
    let pt: *mut ELF_Phdr = raw_addr.add(get_be_e_phoff(hdr)) as *mut ELF_Phdr;

    /* Walk the segment table. */
    i = 0;
    while i < get_be_e_phnum(hdr) {
        let p = pt.add(i);
        if get_be_p_type(p) == PT_LOAD {
            if found_load != 0 {
                fail(b"multiple PT_LOAD segs\0".as_ptr());
            }
            if get_be_p_offset(p) != 0 || get_be_p_vaddr(p) != 0 {
                fail(b"PT_LOAD in wrong place\0".as_ptr());
            }
            if get_be_p_memsz(p) != get_be_p_filesz(p) {
                fail(b"cannot handle memsz != filesz\0".as_ptr());
            }
            load_size = get_be_p_memsz(p);
            found_load = 1;
        } else if get_be_p_type(p) == PT_DYNAMIC {
            dyn_ = raw_addr.add(get_be_p_offset(p)) as *mut ELF_Dyn;
            dyn_end = raw_addr.add(get_be_p_offset(p) + get_be_p_memsz(p)) as *mut ELF_Dyn;
        }
        i += 1;
    }
    if found_load == 0 { fail(b"no PT_LOAD seg\0".as_ptr()); }
    if stripped_len < load_size { fail(b"stripped input is too short\0".as_ptr()); }

    /* Walk the dynamic table */
    i = 0;
    while dyn_.add(i) < dyn_end && get_be_d_tag(dyn_.add(i)) != DT_NULL {
        let tag = get_be_d_tag(dyn_.add(i));
        let val = get_be_d_val(dyn_.add(i));
        if (tag == DT_RELSZ || tag == DT_RELASZ) && val != 0 {
            fail(b"vdso image contains dynamic relocations\0".as_ptr());
        }
        i += 1;
    }

    /* Walk the section table */
    i = 0;
    while i < get_be_e_shnum(hdr) {
        let sh = raw_addr.add(get_be_e_shoff(hdr) + get_be_e_shentsize(hdr) * i) as *mut ELF_Shdr;
        if get_be_sh_type(sh) == SHT_SYMTAB { symtab_hdr = sh; }
        i += 1;
    }
    if symtab_hdr.is_null() { fail(b"no symbol table\n\0".as_ptr()); }
    if name.is_null() {
        fwrite(stripped_addr as *const core::ffi::c_void, stripped_len, 1, outfile);
        return;
    }
    mapping_size = (stripped_len + 8191) / 8192 * 8192;
    fprintf(outfile, b"/* AUTOMATICALLY GENERATED -- DO NOT EDIT */\n\n\0".as_ptr());
    fprintf(outfile, b"#include <linux/cache.h>\n#include <asm/vdso.h>\n\n\0".as_ptr());
    fprintf(outfile, b"static unsigned char raw_data[%lu] __ro_after_init __aligned(8192)= {\0".as_ptr(), mapping_size);
    j = 0;
    while j < stripped_len {
        if j % 10 == 0 { fprintf(outfile, b"\n\t\0".as_ptr()); }
        fprintf(outfile, b"0x%02X, \0".as_ptr(), *stripped_addr.add(j) as i32);
        j += 1;
    }
    fprintf(outfile, b"\n};\n\n\0".as_ptr());
    fprintf(outfile, b"const struct vdso_image %s_builtin = {\n\t.data = raw_data,\n\t.size = %lu,\n};\n\0".as_ptr(), name, mapping_size);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
