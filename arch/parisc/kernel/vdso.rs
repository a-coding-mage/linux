// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (c) 2022 Helge Deller <deller@gmx.de>
 *
 *  based on arch/s390/kernel/vdso.c which is
 *  Copyright IBM Corp. 2008
 *  Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 */

// Dependencies are supplied by the surrounding kernel tree.

extern "C" {
    static mut vdso32_start: core::ffi::c_char;
    static mut vdso32_end: core::ffi::c_char;
    static mut vdso64_start: core::ffi::c_char;
    static mut vdso64_end: core::ffi::c_char;
}

unsafe extern "C" fn vdso_mremap(
    _sm: *const vm_special_mapping,
    vma: *mut vm_area_struct,
) -> i32 {
    (*current).mm.context.vdso_base = (*vma).vm_start;
    0
}

#[cfg(CONFIG_64BIT)]
static mut vdso64_mapping: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const core::ffi::c_char,
    mremap: Some(vdso_mremap),
    ..unsafe { core::mem::zeroed() }
};

static mut vdso32_mapping: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const core::ffi::c_char,
    mremap: Some(vdso_mremap),
    ..unsafe { core::mem::zeroed() }
};

/*
 * This is called from binfmt_elf, we create the special vma for the
 * vDSO and insert it into the mm struct tree
 */
pub unsafe extern "C" fn arch_setup_additional_pages(
    _bprm: *mut linux_binprm,
    _executable_stack: i32,
) -> i32 {
    let (mut vdso_text_start, vdso_text_len, vdso_mapping):
        (usize, usize, *mut vm_special_mapping);
    let mm: *mut mm_struct = (*current).mm as *mut mm_struct;
    let vma: *mut vm_area_struct;
    let rc: i32;

    if mmap_write_lock_killable(mm) != 0 {
        return -EINTR;
    }

    #[cfg(CONFIG_64BIT)]
    if !is_compat_task() {
        vdso_text_len = (&vdso64_end as *const _ as usize)
            .wrapping_sub(&vdso64_start as *const _ as usize);
        vdso_mapping = &raw mut vdso64_mapping;
    } else {
        vdso_text_len = (&vdso32_end as *const _ as usize)
            .wrapping_sub(&vdso32_start as *const _ as usize);
        vdso_mapping = &raw mut vdso32_mapping;
    }
    #[cfg(not(CONFIG_64BIT))]
    {
        vdso_text_len = (&vdso32_end as *const _ as usize)
            .wrapping_sub(&vdso32_start as *const _ as usize);
        vdso_mapping = &raw mut vdso32_mapping;
    }

    let mut map_base = (*mm).mmap_base;
    if (*current).flags & PF_RANDOMIZE != 0 {
        map_base = map_base.wrapping_sub(
            (get_random_u32_below(0x20) as usize).wrapping_mul(PAGE_SIZE),
        );
    }

    vdso_text_start = get_unmapped_area(core::ptr::null_mut(), map_base, vdso_text_len, 0, 0);

    /* VM_MAYWRITE for COW so gdb can set breakpoints */
    vma = _install_special_mapping(
        mm,
        vdso_text_start,
        vdso_text_len,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC,
        vdso_mapping,
    );
    if IS_ERR(vma) {
        do_munmap(mm, vdso_text_start, PAGE_SIZE, core::ptr::null_mut());
        rc = PTR_ERR(vma);
    } else {
        (*mm).context.vdso_base = vdso_text_start;
        rc = 0;
    }

    mmap_write_unlock(mm);
    rc
}

unsafe extern "C" fn vdso_setup_pages(
    start: *mut core::ffi::c_void,
    end: *mut core::ffi::c_void,
) -> *mut *mut page {
    let pages = (end as usize).wrapping_sub(start as usize) >> PAGE_SHIFT;
    let pagelist: *mut *mut page = kzalloc_objs::<*mut page>(pages + 1);
    if pagelist.is_null() {
        panic!("{}: Cannot allocate page list for VDSO", "vdso_setup_pages");
    }
    for i in 0..pages {
        *pagelist.add(i) = virt_to_page((start as *mut u8).add(i * PAGE_SIZE) as *mut core::ffi::c_void);
    }
    pagelist
}

unsafe extern "C" fn vdso_init() -> i32 {
    #[cfg(CONFIG_64BIT)]
    {
        vdso64_mapping.pages = vdso_setup_pages(
            &raw mut vdso64_start as *mut core::ffi::c_char as *mut core::ffi::c_void,
            &raw mut vdso64_end as *mut core::ffi::c_char as *mut core::ffi::c_void,
        );
    }
    // Preserve the original CONFIG_COMPAT || !CONFIG_64BIT build condition.
    #[cfg(any(CONFIG_COMPAT, not(CONFIG_64BIT)))]
    {
        vdso32_mapping.pages = vdso_setup_pages(
            &raw mut vdso32_start as *mut core::ffi::c_char as *mut core::ffi::c_void,
            &raw mut vdso32_end as *mut core::ffi::c_char as *mut core::ffi::c_void,
        );
    }
    0
}

// arch_initcall(vdso_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
