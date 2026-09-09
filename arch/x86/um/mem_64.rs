// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_char;

// Provided by the surrounding Linux memory-management and architecture code.
#[allow(improper_ctypes)]
extern "C" {
    pub static um_vdso_addr: usize;
}

#[allow(non_camel_case_types)]
pub struct mm_struct;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct vm_area_struct {
    pub vm_mm: *mut mm_struct,
    pub vm_start: usize,
}

pub unsafe extern "C" fn arch_vma_name(vma: *mut vm_area_struct) -> *const c_char {
    if !(*vma).vm_mm.is_null() && (*vma).vm_start == um_vdso_addr {
        return b"[vdso]\0".as_ptr() as *const c_char;
    }

    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
