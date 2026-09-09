// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external; this file corresponds directly to vdso.c.

extern "C" {
    static mut vdso_start: [core::ffi::c_char; 0];
    static mut vdso_end: [core::ffi::c_char; 0];

    static mut current: *mut task_struct;

    fn kzalloc_objs<T>(count: usize) -> *mut T;
    fn virt_to_page(addr: *mut core::ffi::c_char) -> *mut page;
    fn get_unmapped_area(
        file: *mut core::ffi::c_void,
        addr: usize,
        len: usize,
        pgoff: usize,
        flags: usize,
    ) -> usize;
    fn mmap_write_lock(mm: *mut mm_struct);
    fn mmap_write_unlock(mm: *mut mm_struct);
    fn _install_special_mapping(
        mm: *mut mm_struct,
        addr: usize,
        len: usize,
        vm_flags: usize,
        mapping: *mut vm_special_mapping,
    ) -> *mut vm_area_struct;
    fn ptr_err<T>(ptr: *mut T) -> isize;
    fn is_err_value(value: usize) -> bool;
    fn pr_err(message: *const core::ffi::c_char);
}

extern "C" {
    type page;
    type linux_binprm;
    type vm_area_struct;
}

#[repr(C)]
struct task_struct {
    mm: *mut mm_struct,
}

#[repr(C)]
struct mm_context {
    vdso: *mut core::ffi::c_void,
}

#[repr(C)]
struct mm_struct {
    context: mm_context,
}

#[repr(C)]
struct vm_special_mapping {
    name: *const core::ffi::c_char,
    pages: *mut *mut page,
}

const PAGE_SHIFT: usize = 12;
const VM_READ: usize = 0x00000001;
const VM_WRITE: usize = 0x00000002;
const VM_EXEC: usize = 0x00000004;
const VM_MAYREAD: usize = 0x00000010;
const VM_MAYWRITE: usize = 0x00000020;
const VM_MAYEXEC: usize = 0x00000040;
const ENOMEM: isize = 12;

static mut vdso_pages: u32 = 0;
static mut vdso_pagelist: *mut *mut page = core::ptr::null_mut();

unsafe fn vdso_init() -> isize {
    let mut i: u32;

    vdso_pages = ((vdso_end.as_ptr() as usize).wrapping_sub(vdso_start.as_ptr() as usize)
        >> PAGE_SHIFT) as u32;
    vdso_pagelist = kzalloc_objs::<*mut page>(vdso_pages as usize);
    if vdso_pagelist.is_null() {
        pr_err(b"vdso: pagelist allocation failed\0".as_ptr() as *const core::ffi::c_char);
        return -ENOMEM;
    }

    i = 0;
    while i < vdso_pages {
        let pg: *mut page;

        pg = virt_to_page(
            vdso_start
                .as_mut_ptr()
                .add((i as usize) << PAGE_SHIFT),
        );
        *vdso_pagelist.add(i as usize) = pg;
        i += 1;
    }

    0
}

// arch_initcall(vdso_init);

unsafe fn arch_setup_additional_pages(
    _bprm: *mut linux_binprm,
    _uses_interp: i32,
) -> isize {
    let mut vma: *mut vm_area_struct;
    let mm: *mut mm_struct = (*current).mm;
    let mut vdso_base: usize;
    let vdso_len: usize;
    let mut ret: isize;
    static mut vdso_mapping: vm_special_mapping = vm_special_mapping {
        name: b"[vdso]\0".as_ptr() as *const core::ffi::c_char,
        pages: core::ptr::null_mut(),
    };

    vdso_len = (vdso_pages as usize) << PAGE_SHIFT;

    mmap_write_lock(mm);
    vdso_base = get_unmapped_area(core::ptr::null_mut(), 0, vdso_len, 0, 0);
    if is_err_value(vdso_base) {
        ret = vdso_base as isize;
        mmap_write_unlock(mm);
        return ret;
    }

    /*
     * Put vDSO base into mm struct. We need to do this before calling
     * install_special_mapping or the perf counter mmap tracking code
     * will fail to recognise it as a vDSO (since arch_vma_name fails).
     */
    (*mm).context.vdso = vdso_base as *mut core::ffi::c_void;

    vdso_mapping.pages = vdso_pagelist;
    vma = _install_special_mapping(
        mm,
        vdso_base,
        (vdso_pages as usize) << PAGE_SHIFT,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC,
        &mut vdso_mapping,
    );

    if (vma as *mut core::ffi::c_void).is_null() {
        ret = ptr_err(vma);
        (*mm).context.vdso = core::ptr::null_mut();
        mmap_write_unlock(mm);
        return ret;
    }

    vdso_base += (vdso_pages as usize) << PAGE_SHIFT;
    ret = 0;
    mmap_write_unlock(mm);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
