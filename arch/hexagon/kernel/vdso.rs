// SPDX-License-Identifier: GPL-2.0-only
/*
 * vDSO implementation for Hexagon
 *
 * Copyright (c) 2011, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel implementation.

use core::ffi::c_void;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hexagon_vdso {
    pub rt_signal_trampoline: [u32; 2],
}

#[repr(C)]
pub struct linux_binprm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_mm: *mut mm_struct,
    pub vm_start: usize,
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
pub struct vm_special_mapping {
    pub name: *const u8,
    pub pages: *mut *mut page,
}

extern "C" {
    static mut current: *mut task_struct;
    static mut __rt_sigtramp_template: [u32; 2];

    fn alloc_page(gfp_mask: u32) -> *mut page;
    fn panic(format: *const u8) -> !;
    fn vmap(pages: *const *mut page, count: usize, flags: u32, prot: u32)
        -> *mut hexagon_vdso;
    fn clear_page(address: *mut c_void);
    fn vunmap(address: *mut c_void);
    fn mmap_write_lock_killable(mm: *mut mm_struct) -> i32;
    fn get_unmapped_area(
        file: *mut c_void,
        addr: usize,
        len: usize,
        pgoff: usize,
        flags: usize,
    ) -> usize;
    fn _install_special_mapping(
        mm: *mut mm_struct,
        addr: usize,
        len: usize,
        vm_flags: usize,
        spec: *mut vm_special_mapping,
    ) -> *mut vm_area_struct;
    fn mmap_write_unlock(mm: *mut mm_struct);
    fn ptr_err<T>(ptr: *mut T) -> isize;
}

#[repr(C)]
pub struct task_struct {
    pub mm: *mut mm_struct,
}

static mut vdso_page: *mut page = core::ptr::null_mut();

/* Create a vDSO page holding the signal trampoline.
 * We want this for a non-executable stack.
 */
unsafe fn vdso_init() -> i32 {
    let vdso: *mut hexagon_vdso;

    vdso_page = alloc_page(GFP_KERNEL);
    if vdso_page.is_null() {
        panic(b"Cannot allocate vdso\0".as_ptr());
    }

    vdso = vmap(&vdso_page, 1, 0, PAGE_KERNEL);
    if vdso.is_null() {
        panic(b"Cannot map vdso\0".as_ptr());
    }
    clear_page(vdso.cast());

    /* Install the signal trampoline; currently looks like this:
     *     r6 = #__NR_rt_sigreturn;
     *     trap0(#1);
     */
    (*vdso).rt_signal_trampoline[0] = __rt_sigtramp_template[0];
    (*vdso).rt_signal_trampoline[1] = __rt_sigtramp_template[1];

    vunmap(vdso.cast());

    0
}

// arch_initcall(vdso_init);

/*
 * Called from binfmt_elf.  Create a VMA for the vDSO page.
 */
pub unsafe fn arch_setup_additional_pages(
    _bprm: *mut linux_binprm,
    _uses_interp: i32,
) -> i32 {
    let ret: i32;
    let vdso_base: usize;
    let vma: *mut vm_area_struct;
    let mm: *mut mm_struct = (*current).mm;
    let mut vdso_mapping = vm_special_mapping {
        name: b"[vdso]\0".as_ptr(),
        pages: core::ptr::null_mut(),
    };

    if mmap_write_lock_killable(mm) != 0 {
        return -EINTR;
    }

    /* Try to get it loaded right near ld.so/glibc. */
    vdso_base = get_unmapped_area(core::ptr::null_mut(), STACK_TOP, PAGE_SIZE, 0, 0);
    if is_err_value(vdso_base) {
        ret = vdso_base as i32;
        mmap_write_unlock(mm);
        return ret;
    }

    /* MAYWRITE to allow gdb to COW and set breakpoints. */
    vdso_mapping.pages = &mut vdso_page;
    vma = _install_special_mapping(
        mm,
        vdso_base,
        PAGE_SIZE,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC,
        &mut vdso_mapping,
    );

    ret = ptr_err(vma) as i32;
    if is_err(vma) {
        mmap_write_unlock(mm);
        return ret;
    }

    (*mm).context.vdso = vdso_base as *mut c_void;
    mmap_write_unlock(mm);
    0
}

pub unsafe fn arch_vma_name(vma: *mut vm_area_struct) -> *const u8 {
    if !(*vma).vm_mm.is_null()
        && (*vma).vm_start == (*(*vma).vm_mm).context.vdso as usize
    {
        return b"[vdso]\0".as_ptr();
    }
    core::ptr::null()
}

// Constants and predicates supplied by the architecture/kernel headers.
extern "C" {
    static GFP_KERNEL: u32;
    static PAGE_KERNEL: u32;
    static STACK_TOP: usize;
    static PAGE_SIZE: usize;
    static EINTR: i32;
    static VM_READ: usize;
    static VM_EXEC: usize;
    static VM_MAYREAD: usize;
    static VM_MAYWRITE: usize;
    static VM_MAYEXEC: usize;
}

unsafe fn is_err_value(value: usize) -> bool {
    value >= (!0usize - 4095)
}

unsafe fn is_err<T>(ptr: *mut T) -> bool {
    is_err_value(ptr as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
