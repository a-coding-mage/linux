// SPDX-License-Identifier: GPL-2.0-only
/*
 * VDSO implementations.
 *
 * Copyright (C) 2012 ARM Limited
 *
 * Author: Will Deacon <will.deacon@arm.com>
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum VdsoAbi {
    VDSO_ABI_AA64,
    VDSO_ABI_AA32,
}

#[repr(C)]
pub struct VdsoAbiInfo {
    pub name: *const core::ffi::c_char,
    pub vdso_code_start: *const core::ffi::c_char,
    pub vdso_code_end: *const core::ffi::c_char,
    pub vdso_pages: usize,
    // Code Mapping
    pub cm: *mut VmSpecialMapping,
}

#[repr(C)]
pub struct VmSpecialMapping {
    pub name: *const core::ffi::c_char,
    pub pages: *mut *mut Page,
    pub mremap: Option<unsafe extern "C" fn(*const VmSpecialMapping, *mut VmAreaStruct) -> i32>,
}

#[repr(C)]
pub struct Page;
#[repr(C)]
pub struct VmAreaStruct {
    pub vm_start: usize,
}
#[repr(C)]
pub struct MmStruct {
    pub context: MmContext,
}
#[repr(C)]
pub struct MmContext {
    pub vdso: *mut core::ffi::c_void,
    pub sigpage: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct LinuxBinprm;

extern "C" {
    static mut vdso_start: core::ffi::c_char;
    static mut vdso_end: core::ffi::c_char;
    #[cfg(CONFIG_COMPAT_VDSO)]
    static mut vdso32_start: core::ffi::c_char;
    #[cfg(CONFIG_COMPAT_VDSO)]
    static mut vdso32_end: core::ffi::c_char;
    static mut current: *mut TaskStruct;
    static mut vdso_info: [VdsoAbiInfo; 2];
}

#[repr(C)]
pub struct TaskStruct {
    pub mm: *mut MmStruct,
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub enum Aarch32Map {
    AA32_MAP_VECTORS,
    AA32_MAP_SIGPAGE,
    AA32_MAP_VDSO,
}

#[cfg(CONFIG_COMPAT)]
static mut aarch32_vectors_page: *mut Page = core::ptr::null_mut();
#[cfg(CONFIG_COMPAT)]
static mut aarch32_sig_page: *mut Page = core::ptr::null_mut();

unsafe extern "C" fn vdso_mremap(_sm: *const VmSpecialMapping, new_vma: *mut VmAreaStruct) -> i32 {
    (*(*current).mm).context.vdso = (*new_vma).vm_start as *mut core::ffi::c_void;
    0
}

unsafe fn __vdso_init(abi: VdsoAbi) -> i32 {
    let index = abi as usize;
    let info = &mut vdso_info[index];
    if core::slice::from_raw_parts(info.vdso_code_start as *const u8, 4)
        != [0x7f, b'E', b'L', b'F']
    {
        return -22; // -EINVAL
    }

    info.vdso_pages = (info.vdso_code_end as usize - info.vdso_code_start as usize) >> PAGE_SHIFT;
    let vdso_pagelist = kzalloc_objs::<*mut Page>(info.vdso_pages);
    if vdso_pagelist.is_null() {
        return -12; // -ENOMEM
    }

    let pfn = sym_to_pfn(info.vdso_code_start);
    for i in 0..info.vdso_pages {
        *vdso_pagelist.add(i) = pfn_to_page(pfn + i);
    }
    (*info.cm).pages = vdso_pagelist;
    0
}

unsafe fn __setup_additional_pages(
    abi: VdsoAbi,
    mm: *mut MmStruct,
    _bprm: *mut LinuxBinprm,
    _uses_interp: i32,
) -> i32 {
    let info = &vdso_info[abi as usize];
    let vdso_text_len = info.vdso_pages << PAGE_SHIFT;
    let vdso_mapping_len = vdso_text_len + VDSO_NR_PAGES * PAGE_SIZE;
    let mut vdso_base = get_unmapped_area(core::ptr::null_mut(), 0, vdso_mapping_len, 0, 0);
    if is_err_value(vdso_base) {
        return ptr_err(err_ptr(vdso_base));
    }
    let mut ret = vdso_install_vvar_mapping(mm, vdso_base);
    if is_err(ret) {
        return ptr_err(ret);
    }
    let mut gp_flags = 0;
    if system_supports_bti_kernel() {
        gp_flags = VM_ARM64_BTI;
    }
    vdso_base += VDSO_NR_PAGES * PAGE_SIZE;
    (*mm).context.vdso = vdso_base as *mut core::ffi::c_void;
    ret = install_special_mapping(
        mm, vdso_base, vdso_text_len,
        VM_READ | VM_EXEC | gp_flags | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC | VM_SEALED_SYSMAP,
        info.cm,
    );
    if is_err(ret) {
        (*mm).context.vdso = core::ptr::null_mut();
        return ptr_err(ret);
    }
    0
}

#[cfg(CONFIG_COMPAT)]
unsafe extern "C" fn aarch32_sigpage_mremap(_sm: *const VmSpecialMapping, new_vma: *mut VmAreaStruct) -> i32 {
    (*(*current).mm).context.sigpage = (*new_vma).vm_start as *mut core::ffi::c_void;
    0
}

#[cfg(CONFIG_COMPAT)]
static mut aarch32_vdso_maps: [VmSpecialMapping; 3] = [
    VmSpecialMapping { name: c"[vectors]".as_ptr(), pages: &raw mut aarch32_vectors_page, mremap: None },
    VmSpecialMapping { name: c"[sigpage]".as_ptr(), pages: &raw mut aarch32_sig_page, mremap: Some(aarch32_sigpage_mremap) },
    VmSpecialMapping { name: c"[vdso]".as_ptr(), pages: core::ptr::null_mut(), mremap: Some(vdso_mremap) },
];

#[cfg(CONFIG_COMPAT)]
pub unsafe extern "C" fn aarch32_setup_additional_pages(bprm: *mut LinuxBinprm, uses_interp: i32) -> i32 {
    let mm = (*current).mm;
    if mmap_write_lock_killable(mm) != 0 { return -4; }
    let mut ret = aarch32_kuser_helpers_setup(mm);
    if ret == 0 && is_enabled_compat_vdso() { ret = __setup_additional_pages(VdsoAbi::VDSO_ABI_AA32, mm, bprm, uses_interp); }
    if ret == 0 { ret = aarch32_sigreturn_setup(mm); }
    mmap_write_unlock(mm);
    ret
}

unsafe fn aarch32_kuser_helpers_setup(mm: *mut MmStruct) -> i32 { 0 }
unsafe fn aarch32_sigreturn_setup(mm: *mut MmStruct) -> i32 { 0 }

static mut aarch64_vdso_map: VmSpecialMapping = VmSpecialMapping {
    name: c"[vdso]".as_ptr(), pages: core::ptr::null_mut(), mremap: Some(vdso_mremap),
};

pub unsafe extern "C" fn vdso_init() -> i32 {
    vdso_info[VdsoAbi::VDSO_ABI_AA64 as usize].cm = &raw mut aarch64_vdso_map;
    __vdso_init(VdsoAbi::VDSO_ABI_AA64)
}

pub unsafe extern "C" fn arch_setup_additional_pages(bprm: *mut LinuxBinprm, uses_interp: i32) -> i32 {
    let mm = (*current).mm;
    if mmap_write_lock_killable(mm) != 0 { return -4; }
    let ret = __setup_additional_pages(VdsoAbi::VDSO_ABI_AA64, mm, bprm, uses_interp);
    mmap_write_unlock(mm);
    ret
}

// External kernel definitions and constants referenced above are provided by the kernel tree.
extern "C" {
    fn kzalloc_objs<T>(n: usize) -> *mut T;
    fn sym_to_pfn(p: *const core::ffi::c_char) -> usize;
    fn pfn_to_page(pfn: usize) -> *mut Page;
    fn get_unmapped_area(file: *mut core::ffi::c_void, addr: usize, len: usize, p: usize, flags: usize) -> usize;
    fn vdso_install_vvar_mapping(mm: *mut MmStruct, addr: usize) -> *mut core::ffi::c_void;
    fn system_supports_bti_kernel() -> bool;
    fn install_special_mapping(mm: *mut MmStruct, addr: usize, len: usize, flags: usize, sm: *mut VmSpecialMapping) -> *mut core::ffi::c_void;
    fn is_err_value(v: usize) -> bool;
    fn is_err(v: *mut core::ffi::c_void) -> bool;
    fn err_ptr(v: usize) -> *mut core::ffi::c_void;
    fn ptr_err(v: *mut core::ffi::c_void) -> i32;
    fn mmap_write_lock_killable(mm: *mut MmStruct) -> i32;
    fn mmap_write_unlock(mm: *mut MmStruct);
    fn is_enabled_compat_vdso() -> bool;
}

const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const VDSO_NR_PAGES: usize = 1;
const VM_READ: usize = 1 << 0;
const VM_EXEC: usize = 1 << 1;
const VM_MAYREAD: usize = 1 << 2;
const VM_MAYWRITE: usize = 1 << 3;
const VM_MAYEXEC: usize = 1 << 4;
const VM_SEALED_SYSMAP: usize = 1 << 5;
const VM_ARM64_BTI: usize = 1 << 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
