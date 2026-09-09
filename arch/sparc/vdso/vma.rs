// SPDX-License-Identifier: GPL-2.0-only
/*
 * Set up the VMAs to tell the VM about the vDSO.
 * Copyright 2007 Andi Kleen, SUSE Labs.
 */
/* Copyright (c) 2017 Oracle and/or its affiliates. All rights reserved. */

// Kernel headers and architecture-provided symbols are supplied by other files.

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

pub static mut vdso_enabled: c_uint = 1;

#[cfg(CONFIG_SPARC64)]
static mut vdso_mapping64: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const c_char,
    pages: core::ptr::null_mut(),
};

#[cfg(CONFIG_COMPAT)]
static mut vdso_mapping32: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const c_char,
    pages: core::ptr::null_mut(),
};

#[repr(C)]
struct page;
#[repr(C)]
struct mm_struct {
    context: mm_context,
}
#[repr(C)]
struct mm_context {
    vdso: *mut c_void,
}
#[repr(C)]
struct task_struct {
    flags: c_ulong,
    mm: *mut mm_struct,
}
#[repr(C)]
struct vm_area_struct;
#[repr(C)]
struct linux_binprm;
#[repr(C)]
struct vdso_image {
    size: usize,
    data: *const u8,
}
#[repr(C)]
struct vm_special_mapping {
    name: *const c_char,
    pages: *mut *mut page,
}

extern "C" {
    static mut current: *mut task_struct;
    static vdso_image_64_builtin: vdso_image;
    static vdso_image_32_builtin: vdso_image;
    fn kzalloc_objs(size: usize) -> *mut *mut page;
    fn alloc_page(gfp: c_ulong) -> *mut page;
    fn page_address(page: *mut page) -> *mut c_void;
    fn copy_page(dst: *mut c_void, src: *const u8);
    fn __free_page(page: *mut page);
    fn kfree(ptr: *mut c_void);
    fn get_random_u32_below(x: c_uint) -> c_uint;
    fn mmap_write_lock(mm: *mut mm_struct);
    fn mmap_write_unlock(mm: *mut mm_struct);
    fn get_unmapped_area(file: *mut c_void, addr: c_ulong, len: usize, pgoff: c_ulong, flags: c_ulong) -> c_ulong;
    fn _install_special_mapping(mm: *mut mm_struct, addr: c_ulong, len: usize, flags: c_ulong, mapping: *mut vm_special_mapping) -> *mut vm_area_struct;
    fn vdso_install_vvar_mapping(mm: *mut mm_struct, addr: c_ulong) -> *mut vm_area_struct;
    fn do_munmap(mm: *mut mm_struct, start: c_ulong, len: usize, uf: *mut c_void) -> c_int;
    fn kstrtoul(s: *const c_char, base: c_uint, result: *mut c_ulong) -> c_int;
}

const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: c_uint = 12;
const PTRS_PER_PTE: c_uint = 512;
const PF_RANDOMIZE: c_ulong = 0x00400000;
const VM_READ: c_ulong = 0x00000001;
const VM_EXEC: c_ulong = 0x00000004;
const VM_MAYREAD: c_ulong = 0x00000010;
const VM_MAYWRITE: c_ulong = 0x00000020;
const VM_MAYEXEC: c_ulong = 0x00000040;
const VDSO_NR_PAGES: usize = 1;

unsafe fn init_vdso_image(image: *const vdso_image, mapping: *mut vm_special_mapping, _elf64: bool) -> c_int {
    let cnpages = (*image).size / PAGE_SIZE;
    let mut cpp: *mut *mut page = core::ptr::null_mut();
    if (*image).size % PAGE_SIZE != 0 { return vdso_oom(cpp, cnpages, mapping); }
    cpp = kzalloc_objs(core::mem::size_of::<*mut page>() * cnpages);
    (*mapping).pages = cpp;
    if cpp.is_null() { return vdso_oom(cpp, cnpages, mapping); }
    for i in 0..cnpages {
        let cp = alloc_page(0);
        if cp.is_null() { return vdso_oom(cpp, cnpages, mapping); }
        *cpp.add(i) = cp;
        copy_page(page_address(cp), (*image).data.add(i * PAGE_SIZE));
    }
    0
}

unsafe fn vdso_oom(cpp: *mut *mut page, cnpages: usize, mapping: *mut vm_special_mapping) -> c_int {
    if !cpp.is_null() {
        for i in 0..cnpages {
            let cp = *cpp.add(i);
            if !cp.is_null() { __free_page(cp); }
        }
        kfree(cpp as *mut c_void);
        (*mapping).pages = core::ptr::null_mut();
    }
    vdso_enabled = 0;
    -12
}

unsafe fn init_vdso() -> c_int {
    let mut err = 0;
    #[cfg(CONFIG_SPARC64)] {
        err = init_vdso_image(&vdso_image_64_builtin, &mut vdso_mapping64, true);
        if err != 0 { return err; }
    }
    #[cfg(CONFIG_COMPAT)] { err = init_vdso_image(&vdso_image_32_builtin, &mut vdso_mapping32, false); }
    err
}

unsafe fn vdso_addr(start: c_ulong, _len: c_uint) -> c_ulong {
    let offset = get_random_u32_below(PTRS_PER_PTE);
    start + ((offset as c_ulong) << PAGE_SHIFT)
}

unsafe fn map_vdso(image: *const vdso_image, mapping: *mut vm_special_mapping) -> c_int {
    let area_size = (*image).size + VDSO_NR_PAGES * PAGE_SIZE;
    let mm = (*current).mm;
    let mut addr = 0;
    let mut ret = 0;
    mmap_write_lock(mm);
    if (*current).flags & PF_RANDOMIZE != 0 {
        addr = get_unmapped_area(core::ptr::null_mut(), 0, area_size, 0, 0);
        if addr as isize < 0 { ret = addr as c_int; if ret != 0 { (*mm).context.vdso = core::ptr::null_mut(); } mmap_write_unlock(mm); return ret; }
        addr = vdso_addr(addr, area_size as c_uint);
    }
    addr = get_unmapped_area(core::ptr::null_mut(), addr, area_size, 0, 0);
    if addr as isize < 0 { ret = addr as c_int; if ret != 0 { (*mm).context.vdso = core::ptr::null_mut(); } mmap_write_unlock(mm); return ret; }
    let text_start = addr + (VDSO_NR_PAGES * PAGE_SIZE) as c_ulong;
    (*mm).context.vdso = text_start as *mut c_void;
    let vma = _install_special_mapping(mm, text_start, (*image).size,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC, mapping);
    if vma as isize < 0 { ret = vma as isize as c_int; if ret != 0 { (*mm).context.vdso = core::ptr::null_mut(); } mmap_write_unlock(mm); return ret; }
    let vma = vdso_install_vvar_mapping(mm, addr);
    if vma as isize < 0 { ret = vma as isize as c_int; do_munmap(mm, text_start, (*image).size, core::ptr::null_mut()); }
    if ret != 0 { (*mm).context.vdso = core::ptr::null_mut(); }
    mmap_write_unlock(mm);
    ret
}

pub unsafe fn arch_setup_additional_pages(_bprm: *mut linux_binprm, _uses_interp: c_int) -> c_int {
    if vdso_enabled == 0 { return 0; }
    #[cfg(CONFIG_COMPAT)] {
        if is_32bit_task() { return map_vdso(&vdso_image_32_builtin, &mut vdso_mapping32); }
    }
    map_vdso(&vdso_image_64_builtin, &mut vdso_mapping64)
}

extern "C" { fn is_32bit_task() -> bool; }

unsafe fn vdso_setup(s: *mut c_char) -> c_int {
    let mut val = 0;
    if kstrtoul(s, 10, &mut val) == 0 { vdso_enabled = val as c_uint; }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
