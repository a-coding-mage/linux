// SPDX-License-Identifier: GPL-2.0-only
/*
 * arc_hostlink.c: Pseudo-driver for Metaware provided "hostlink" facility
 *
 * Allows Linux userland access to host in absence of any peripherals.
 *
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

const PAGE_SIZE: usize = 4096;
const MISC_DYNAMIC_MINOR: i32 = 255;
const EAGAIN: i32 = 11;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_page_prot: usize,
    pub vm_start: usize,
    pub vm_pgoff: usize,
    pub vm_end: usize,
}

pub type IoctlFn = unsafe extern "C" fn(*mut file, u32, usize) -> isize;
pub type MmapFn = unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> i32;

#[repr(C)]
pub struct file_operations {
    pub unlocked_ioctl: Option<IoctlFn>,
    pub mmap: Option<MmapFn>,
}

#[repr(C)]
pub struct miscdevice {
    pub minor: i32,
    pub name: *const u8,
    pub fops: *const file_operations,
}

extern "C" {
    fn pgprot_noncached(prot: usize) -> usize;
    fn io_remap_pfn_range(
        vma: *mut vm_area_struct,
        addr: usize,
        pfn: usize,
        size: usize,
        prot: usize,
    ) -> i32;
    fn put_user(value: u32, user: *mut i32) -> i32;
    fn misc_register(dev: *mut miscdevice) -> i32;
    fn pr_warn(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
}

static mut __HOSTLINK__: [u8; 4 * PAGE_SIZE] = [0; 4 * PAGE_SIZE];

unsafe extern "C" fn arc_hl_mmap(
    _fp: *mut file,
    vma: *mut vm_area_struct,
) -> i32 {
    (*vma).vm_page_prot = pgprot_noncached((*vma).vm_page_prot);

    if io_remap_pfn_range(
        vma,
        (*vma).vm_start,
        (*vma).vm_pgoff,
        (*vma).vm_end - (*vma).vm_start,
        (*vma).vm_page_prot,
    ) != 0 {
        pr_warn(b"Hostlink buffer mmap ERROR\n\0".as_ptr());
        return -EAGAIN;
    }
    0
}

unsafe extern "C" fn arc_hl_ioctl(
    _file: *mut file,
    _cmd: u32,
    arg: usize,
) -> isize {
    // we only support, returning the physical addr to mmap in user space
    put_user(__HOSTLINK__.as_mut_ptr() as usize as u32, arg as *mut i32);
    0
}

static arc_hl_fops: file_operations = file_operations {
    unlocked_ioctl: Some(arc_hl_ioctl),
    mmap: Some(arc_hl_mmap),
};

static mut arc_hl_dev: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: b"hostlink\0".as_ptr(),
    fops: &arc_hl_fops,
};

unsafe extern "C" fn arc_hl_init() -> i32 {
    pr_info(
        b"ARC Hostlink driver mmap at 0x%p\n\0".as_ptr(),
        __HOSTLINK__.as_mut_ptr(),
    );
    misc_register(&mut arc_hl_dev)
}

// module_init(arc_hl_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
