// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Module strict rwx
 *
 * Copyright (C) 2015 Rusty Russell
 */

// Dependencies supplied by the kernel/module environment are intentionally
// referenced here rather than reimplemented.

use core::ffi::c_char;

extern "C" {
    fn set_vm_flush_reset_perms(addr: *mut core::ffi::c_void);
    fn execmem_restore_rox(base: *mut core::ffi::c_void, size: usize) -> i32;
    fn set_memory_rox(start: usize, num_pages: i32) -> i32;
    fn set_memory_x(start: usize, num_pages: i32) -> i32;
    fn set_memory_ro(start: usize, num_pages: i32) -> i32;
    fn set_memory_nx(start: usize, num_pages: i32) -> i32;
    fn strcmp(a: *const c_char, b: *const c_char) -> i32;
}

extern "C" {
    static rodata_enabled: bool;
}

#[repr(C)]
pub struct module_memory {
    pub base: *mut core::ffi::c_void,
    pub size: usize,
    pub is_rox: bool,
}

#[repr(C)]
pub struct module {
    pub name: *const c_char,
    pub mem: [module_memory; 5],
}

#[repr(C)]
pub struct Elf_Ehdr {
    pub e_shnum: u16,
}

#[repr(C)]
pub struct Elf_Shdr {
    pub sh_name: u32,
    pub sh_flags: u64,
}

const PAGE_SHIFT: usize = 12;
const SHF_WRITE: u64 = 1;
const SHF_EXECINSTR: u64 = 4;
const SHF_RO_AFTER_INIT: u64 = 0x20000000;
const ENOEXEC: i32 = 8;
const MOD_RODATA: usize = 1;
const MOD_INIT_RODATA: usize = 2;
const MOD_RO_AFTER_INIT: usize = 3;

unsafe fn module_set_memory(
    mod_: *const module,
    type_: usize,
    set_memory: unsafe extern "C" fn(usize, i32) -> i32,
) -> i32 {
    let mod_mem = &(*mod_).mem[type_];

    if mod_mem.base.is_null() {
        return 0;
    }

    set_vm_flush_reset_perms(mod_mem.base);
    set_memory(mod_mem.base as usize, (mod_mem.size >> PAGE_SHIFT) as i32)
}

/*
 * Since some arches are moving towards PAGE_KERNEL module allocations
 * instead of PAGE_KERNEL_EXEC, keep module_enable_x() independent of
 * CONFIG_STRICT_MODULE_RWX because they are needed regardless of whether we
 * are strict.
 */
pub unsafe fn module_enable_text_rox(mod_: *const module) -> i32 {
    // for_class_mod_mem_type(type, text)
    for type_ in 0..3 {
        let mem = &(*mod_).mem[type_];
        let ret;

        if mem.is_rox {
            ret = execmem_restore_rox(mem.base, mem.size);
        } else if cfg!(feature = "CONFIG_STRICT_MODULE_RWX") {
            ret = module_set_memory(mod_, type_, set_memory_rox);
        } else {
            ret = module_set_memory(mod_, type_, set_memory_x);
        }
        if ret != 0 {
            return ret;
        }
    }
    0
}

pub unsafe fn module_enable_rodata_ro(mod_: *const module) -> i32 {
    let mut ret;

    if !cfg!(feature = "CONFIG_STRICT_MODULE_RWX") || !rodata_enabled {
        return 0;
    }

    ret = module_set_memory(mod_, MOD_RODATA, set_memory_ro);
    if ret != 0 {
        return ret;
    }
    ret = module_set_memory(mod_, MOD_INIT_RODATA, set_memory_ro);
    if ret != 0 {
        return ret;
    }

    0
}

pub unsafe fn module_enable_rodata_ro_after_init(mod_: *const module) -> i32 {
    if !cfg!(feature = "CONFIG_STRICT_MODULE_RWX") || !rodata_enabled {
        return 0;
    }

    module_set_memory(mod_, MOD_RO_AFTER_INIT, set_memory_ro)
}

pub unsafe fn module_enable_data_nx(mod_: *const module) -> i32 {
    if !cfg!(feature = "CONFIG_STRICT_MODULE_RWX") {
        return 0;
    }

    // for_class_mod_mem_type(type, data)
    for type_ in 0..5 {
        let ret = module_set_memory(mod_, type_, set_memory_nx);

        if ret != 0 {
            return ret;
        }
    }
    0
}

pub unsafe fn module_enforce_rwx_sections(
    hdr: *const Elf_Ehdr,
    sechdrs: *const Elf_Shdr,
    secstrings: *const c_char,
    mod_: *const module,
) -> i32 {
    let shf_wx = SHF_WRITE | SHF_EXECINSTR;
    let mut i = 0;

    if !cfg!(feature = "CONFIG_STRICT_MODULE_RWX") {
        return 0;
    }

    while i < (*hdr).e_shnum as usize {
        if ((*sechdrs.add(i)).sh_flags & shf_wx) == shf_wx {
            // pr_err("%s: section %s (index %d) has invalid WRITE|EXEC flags\n", ...)
            return -ENOEXEC;
        }
        i += 1;
    }

    0
}

static RO_AFTER_INIT: [&[u8]; 2] = [
    b".data..ro_after_init\0",
    b"__jump_table\0",
];

pub unsafe fn module_mark_ro_after_init(
    hdr: *const Elf_Ehdr,
    sechdrs: *mut Elf_Shdr,
    secstrings: *const c_char,
) {
    let mut i = 1usize;

    while i < (*hdr).e_shnum as usize {
        let shdr = &mut *sechdrs.add(i);

        for section in RO_AFTER_INIT.iter() {
            if strcmp(secstrings.add(shdr.sh_name as usize), section.as_ptr() as *const c_char) == 0 {
                shdr.sh_flags |= SHF_RO_AFTER_INIT;
                break;
            }
        }
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
