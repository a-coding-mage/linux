// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 Richard Weinberger <richrd@nod.at>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/slab.h, linux/sched.h, linux/mm.h, asm/page.h, asm/elf.h, linux/init.h

pub static mut um_vdso_addr: ::core::primitive::c_ulong = 0;
static mut um_vdso: *mut page = ::core::ptr::null_mut();

extern "C" {
    static mut task_size: ::core::primitive::c_ulong;
    static mut vdso_start: ::core::ffi::c_char;
    static mut vdso_end: ::core::ffi::c_char;
}

#[allow(improper_ctypes)]
extern "C" {
    fn alloc_page(gfp_mask: ::core::primitive::c_uint) -> *mut page;
    fn panic(fmt: *const ::core::ffi::c_char) -> !;
    fn page_address(page: *mut page) -> *mut ::core::ffi::c_void;
    fn copy_page(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void);
    fn mmap_write_lock_killable(mm: *mut mm_struct) -> ::core::primitive::c_int;
    fn _install_special_mapping(
        mm: *mut mm_struct,
        addr: ::core::primitive::c_ulong,
        len: ::core::primitive::c_ulong,
        vm_flags: ::core::primitive::c_ulong,
        spec: *mut vm_special_mapping,
    ) -> *mut vm_area_struct;
    fn mmap_write_unlock(mm: *mut mm_struct);
    fn IS_ERR(ptr: *mut vm_area_struct) -> bool;
    fn PTR_ERR(ptr: *mut vm_area_struct) -> ::core::primitive::c_long;
}

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
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_special_mapping {
    pub name: *const ::core::ffi::c_char,
    pub pages: *mut *mut page,
}

// Build-time kernel constants and current-task access are supplied externally.
extern "C" {
    static mut current_mm: *mut mm_struct;
}

unsafe fn init_vdso() -> ::core::primitive::c_int {
    if (vdso_end as usize).wrapping_sub(vdso_start as usize) > PAGE_SIZE as usize {
        panic(b"BUG_ON(vdso_end - vdso_start > PAGE_SIZE)\0".as_ptr() as *const _);
    }

    um_vdso_addr = task_size.wrapping_sub(PAGE_SIZE as ::core::primitive::c_ulong);

    um_vdso = alloc_page(GFP_KERNEL);
    if um_vdso.is_null() {
        panic(b"Cannot allocate vdso\n\0".as_ptr() as *const _);
    }

    copy_page(
        page_address(um_vdso),
        vdso_start as *const ::core::ffi::c_void,
    );

    0
}

// subsys_initcall(init_vdso)

pub unsafe fn arch_setup_additional_pages(
    _bprm: *mut linux_binprm,
    _uses_interp: ::core::primitive::c_int,
) -> ::core::primitive::c_long {
    let mut vma: *mut vm_area_struct;
    let mm: *mut mm_struct = current_mm;
    let mut vdso_mapping = vm_special_mapping {
        name: b"[vdso]\0".as_ptr() as *const _,
        pages: &raw mut um_vdso,
    };

    if mmap_write_lock_killable(mm) != 0 {
        return -EINTR as ::core::primitive::c_long;
    }

    vma = _install_special_mapping(
        mm,
        um_vdso_addr,
        PAGE_SIZE as ::core::primitive::c_ulong,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC,
        &mut vdso_mapping,
    );

    mmap_write_unlock(mm);

    if IS_ERR(vma) {
        PTR_ERR(vma)
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
