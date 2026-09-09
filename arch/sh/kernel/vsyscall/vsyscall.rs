// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/vsyscall/vsyscall.c
 *
 *  Copyright (C) 2006 Paul Mundt
 *
 * vDSO randomization
 * Copyright(C) 2005-2006, Red Hat, Inc., Ingo Molnar
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct linux_binprm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    pub context: mm_context,
}

#[repr(C)]
pub struct mm_context {
    pub vdso: *mut c_void,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_mm: *mut mm_struct,
    pub vm_start: c_ulong,
}

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: c_uint,
    pub proc_handler: Option<unsafe extern "C" fn()>,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}

#[repr(C)]
pub struct vm_special_mapping {
    pub name: *const c_char,
    pub pages: *mut *mut page,
}

extern "C" {
    pub static mut current: *mut task_struct;
    pub static vsyscall_trapa_start: c_char;
    pub static vsyscall_trapa_end: c_char;

    pub fn simple_strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulong;
    pub fn get_zeroed_page(gfp_mask: c_ulong) -> c_ulong;
    pub fn virt_to_page(addr: *mut c_void) -> *mut page;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    pub fn register_sysctl_init(path: *const c_char, table: *const ctl_table) -> *mut c_void;
    pub fn mmap_write_lock_killable(mm: *mut mm_struct) -> c_int;
    pub fn get_unmapped_area(file: *mut c_void, addr: c_ulong, len: c_ulong,
                             pgoff: c_ulong, flags: c_ulong) -> c_ulong;
    pub fn _install_special_mapping(mm: *mut mm_struct, addr: c_ulong, len: c_ulong,
                                    vm_flags: c_ulong,
                                    mapping: *mut vm_special_mapping) -> *mut vm_area_struct;
    pub fn mmap_write_unlock(mm: *mut mm_struct);
}

#[repr(C)]
pub struct task_struct {
    pub mm: *mut mm_struct,
}

pub const GFP_ATOMIC: c_ulong = 0x20;
pub const PAGE_SIZE: c_ulong = 4096;
pub const VM_READ: c_ulong = 0x00000001;
pub const VM_MAYREAD: c_ulong = 0x00000010;
pub const VM_MAYWRITE: c_ulong = 0x00000020;
pub const VM_MAYEXEC: c_ulong = 0x00000040;
pub const EINTR: c_int = 4;

pub static mut vdso_enabled: c_uint = 1;

static mut syscall_pages: [*mut page; 1] = [core::ptr::null_mut(); 1];

static mut vdso_mapping: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const c_char,
    pages: unsafe { core::ptr::addr_of_mut!(syscall_pages[0]) },
};

static vdso_table: [ctl_table; 1] = [ctl_table {
    procname: b"vdso_enabled\0".as_ptr() as *const c_char,
    data: unsafe { core::ptr::addr_of_mut!(vdso_enabled) as *mut c_void },
    maxlen: core::mem::size_of::<c_uint>(),
    mode: 0o644,
    proc_handler: None,
    extra1: core::ptr::null_mut(),
    extra2: core::ptr::null_mut(),
}];

unsafe extern "C" fn vdso_setup(s: *mut c_char) -> c_int {
    vdso_enabled = simple_strtoul(s, core::ptr::null_mut(), 0) as c_uint;
    1
}

pub unsafe extern "C" fn vsyscall_init() -> c_int {
    let syscall_page = get_zeroed_page(GFP_ATOMIC) as *mut c_void;
    syscall_pages[0] = virt_to_page(syscall_page);

    memcpy(
        syscall_page,
        core::ptr::addr_of!(vsyscall_trapa_start) as *const c_void,
        (core::ptr::addr_of!(vsyscall_trapa_end) as usize)
            - (core::ptr::addr_of!(vsyscall_trapa_start) as usize),
    );

    0
}

unsafe extern "C" fn vm_sysctl_init() -> c_int {
    register_sysctl_init(b"vm\0".as_ptr() as *const c_char, vdso_table.as_ptr());
    0
}

pub unsafe extern "C" fn arch_setup_additional_pages(
    _bprm: *mut linux_binprm,
    _uses_interp: c_int,
) -> c_int {
    let mm = (*current).mm;
    let mut addr: c_ulong;
    let vma: *mut vm_area_struct;
    let ret: c_int;

    if mmap_write_lock_killable(mm) != 0 {
        return -EINTR;
    }

    addr = get_unmapped_area(core::ptr::null_mut(), 0, PAGE_SIZE, 0, 0);
    if (addr as c_long) < 0 {
        ret = addr as c_int;
        mmap_write_unlock(mm);
        return ret;
    }

    vdso_mapping.pages = syscall_pages.as_mut_ptr();
    vma = _install_special_mapping(
        mm,
        addr,
        PAGE_SIZE,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC,
        core::ptr::addr_of_mut!(vdso_mapping),
    );
    ret = vma as isize as c_int;
    if (vma as isize) < 0 {
        mmap_write_unlock(mm);
        return ret;
    }

    (*mm).context.vdso = addr as *mut c_void;
    mmap_write_unlock(mm);
    0
}

pub unsafe extern "C" fn arch_vma_name(vma: *mut vm_area_struct) -> *const c_char {
    if !(*vma).vm_mm.is_null()
        && (*vma).vm_start == (*(*vma).vm_mm).context.vdso as c_ulong
    {
        return b"[vdso]\0".as_ptr() as *const c_char;
    }

    core::ptr::null()
}

type c_long = isize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
