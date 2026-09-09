// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004 Benjamin Herrenschmidt, IBM Corp.
 *                    <benh@kernel.crashing.org>
 * Copyright (C) 2012 ARM Limited
 * Copyright (C) 2015 Regents of the University of California
 */

// C dependencies supplied by the surrounding kernel build are intentionally
// referenced here rather than reimplemented.

const VVAR_SIZE: usize = (VDSO_NR_PAGES as usize) << PAGE_SHIFT;

#[repr(C)]
pub struct __vdso_info {
    pub name: *const core::ffi::c_char,
    pub vdso_code_start: *const core::ffi::c_char,
    pub vdso_code_end: *const core::ffi::c_char,
    pub vdso_pages: c_ulong,
    // Code Mapping
    pub cm: *mut vm_special_mapping,
}

static mut vdso_info: __vdso_info = __vdso_info {
    name: b"vdso\0".as_ptr() as *const core::ffi::c_char,
    vdso_code_start: vdso_start,
    vdso_code_end: vdso_end,
    vdso_pages: 0,
    cm: &mut rv_vdso_map,
};

#[cfg(CONFIG_COMPAT)]
static mut compat_vdso_info: __vdso_info = __vdso_info {
    name: b"compat_vdso\0".as_ptr() as *const core::ffi::c_char,
    vdso_code_start: compat_vdso_start,
    vdso_code_end: compat_vdso_end,
    vdso_pages: 0,
    cm: &mut rv_compat_vdso_map,
};

unsafe extern "C" fn vdso_mremap(
    _sm: *const vm_special_mapping,
    new_vma: *mut vm_area_struct,
) -> c_int {
    (*current).mm.context.vdso = (*new_vma).vm_start as *mut core::ffi::c_void;
    0
}

unsafe fn __vdso_init(info: *mut __vdso_info) {
    let mut i: c_uint;
    let vdso_pagelist: *mut *mut page;
    let pfn: c_ulong;

    if core::slice::from_raw_parts((*info).vdso_code_start as *const u8, 4)
        != *b"\x7fELF"
    {
        panic!("vDSO is not a valid ELF object!\n");
    }

    (*info).vdso_pages = ((*info).vdso_code_end as usize
        - (*info).vdso_code_start as usize) >> PAGE_SHIFT;

    vdso_pagelist = kzalloc_objs::<*mut page>((*info).vdso_pages);
    if vdso_pagelist.is_null() {
        panic!("vDSO kcalloc failed!\n");
    }

    // Grab the vDSO code pages.
    pfn = sym_to_pfn((*info).vdso_code_start);

    i = 0;
    while (i as usize) < (*info).vdso_pages {
        *vdso_pagelist.add(i as usize) = pfn_to_page(pfn + i as c_ulong);
        i += 1;
    }

    (*(*info).cm).pages = vdso_pagelist;
}

static mut rv_vdso_map: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const core::ffi::c_char,
    mremap: Some(vdso_mremap),
};

#[cfg(CONFIG_COMPAT)]
static mut rv_compat_vdso_map: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const core::ffi::c_char,
    mremap: Some(vdso_mremap),
};

unsafe fn vdso_init() -> c_int {
    // Hart implements zimop, expose cfi compiled vdso
    if IS_ENABLED(CONFIG_RISCV_USER_CFI)
        && riscv_has_extension_unlikely(RISCV_ISA_EXT_ZIMOP)
    {
        vdso_info.vdso_code_start = vdso_cfi_start;
        vdso_info.vdso_code_end = vdso_cfi_end;
    }

    __vdso_init(&mut vdso_info);
    #[cfg(CONFIG_COMPAT)]
    __vdso_init(&mut compat_vdso_info);

    0
}

unsafe fn __setup_additional_pages(
    mm: *mut mm_struct,
    _bprm: *mut linux_binprm,
    _uses_interp: c_int,
    info: *mut __vdso_info,
) -> c_int {
    let vdso_text_len: c_ulong = (*info).vdso_pages << PAGE_SHIFT;
    // Be sure to map the data page
    let vdso_mapping_len = vdso_text_len + VVAR_SIZE as c_ulong;
    let mut vdso_base = get_unmapped_area(
        core::ptr::null_mut(),
        0,
        vdso_mapping_len,
        0,
        0,
    );
    let mut ret: *mut core::ffi::c_void;

    if IS_ERR_VALUE(vdso_base) {
        ret = ERR_PTR(vdso_base);
        (*mm).context.vdso = core::ptr::null_mut();
        return PTR_ERR(ret);
    }

    ret = vdso_install_vvar_mapping(mm, vdso_base);
    if IS_ERR(ret) {
        (*mm).context.vdso = core::ptr::null_mut();
        return PTR_ERR(ret);
    }

    vdso_base += VVAR_SIZE as c_ulong;
    (*mm).context.vdso = vdso_base as *mut core::ffi::c_void;

    ret = _install_special_mapping(
        mm,
        vdso_base,
        vdso_text_len,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC | VM_SEALED_SYSMAP,
        (*info).cm,
    );

    if IS_ERR(ret) {
        (*mm).context.vdso = core::ptr::null_mut();
        return PTR_ERR(ret);
    }

    0
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn compat_arch_setup_additional_pages(
    bprm: *mut linux_binprm,
    uses_interp: c_int,
) -> c_int {
    let mm = (*current).mm;
    if mmap_write_lock_killable(mm) != 0 {
        return -EINTR;
    }
    let ret = __setup_additional_pages(mm, bprm, uses_interp, &mut compat_vdso_info);
    mmap_write_unlock(mm);
    ret
}

pub unsafe fn arch_setup_additional_pages(
    bprm: *mut linux_binprm,
    uses_interp: c_int,
) -> c_int {
    let mm = (*current).mm;
    if mmap_write_lock_killable(mm) != 0 {
        return -EINTR;
    }
    let ret = __setup_additional_pages(mm, bprm, uses_interp, &mut vdso_info);
    mmap_write_unlock(mm);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
