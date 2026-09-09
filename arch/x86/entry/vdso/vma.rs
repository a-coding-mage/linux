// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2007 Andi Kleen, SUSE Labs.
 *
 * This contains most of the x86 vDSO kernel-side code.
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

static_assert!(VDSO_NR_PAGES + VDSO_NR_VCLOCK_PAGES == __VDSO_PAGES);

static mut VCLOCKS_USED: u32 = 0;

#[cfg(CONFIG_X86_64)]
static mut VDSO64_ENABLED: u32 = 1;

#[inline(never)]
pub unsafe fn init_vdso_image(image: *const vdso_image) -> i32 {
    build_bug_on!(VDSO_CLOCKMODE_MAX >= 32);
    bug_on!((*image).size % PAGE_SIZE != 0);

    apply_alternatives(
        ((*image).data.add((*image).alt)) as *mut alt_instr,
        ((*image).data.add((*image).alt + (*image).alt_len)) as *mut alt_instr,
    );

    0
}

pub struct linux_binprm;

unsafe fn vdso_fault(
    _sm: *const vm_special_mapping,
    vma: *mut vm_area_struct,
    vmf: *mut vm_fault,
) -> vm_fault_t {
    let image = (*(*(*vma).vm_mm).context.vdso_image);

    if image.is_null() || ((*vmf).pgoff << PAGE_SHIFT) >= (*image).size {
        return VM_FAULT_SIGBUS;
    }

    (*vmf).page = virt_to_page((*image).data.add((*vmf).pgoff << PAGE_SHIFT));
    get_page((*vmf).page);
    0
}

unsafe fn vdso_fix_landing(image: *const vdso_image, new_vma: *mut vm_area_struct) {
    let regs = current_pt_regs();
    let ipoffset = (*regs).ip - current.mm.context.vdso as usize;

    if ipoffset < (*image).size {
        (*regs).ip = (*new_vma).vm_start + ipoffset;
    }
}

#[cfg(CONFIG_FUTEX_ROBUST_UNLOCK)]
unsafe fn vdso_futex_robust_unlock_update_ips() {
    let image = current.mm.context.vdso_image;
    let vdso = current.mm.context.vdso as usize;
    let fd = &mut current.mm.futex;
    let mut idx: u32 = 0;

    futex_reset_cs_ranges(fd);

    #[cfg(CONFIG_X86_64)]
    {
        futex_set_vdso_cs_range(
            fd,
            idx,
            vdso + (*image).sym___futex_list64_try_unlock_cs_start,
            vdso + (*image).sym___futex_list64_try_unlock_cs_end,
            false,
        );
        idx += 1;
    }

    #[cfg(any(CONFIG_X86_32, CONFIG_COMPAT))]
    {
        futex_set_vdso_cs_range(
            fd,
            idx,
            vdso + (*image).sym___futex_list32_try_unlock_cs_start,
            vdso + (*image).sym___futex_list32_try_unlock_cs_end,
            true,
        );
    }
}

#[cfg(not(CONFIG_FUTEX_ROBUST_UNLOCK))]
#[inline]
unsafe fn vdso_futex_robust_unlock_update_ips() {}

unsafe fn vdso_mremap(
    _sm: *const vm_special_mapping,
    new_vma: *mut vm_area_struct,
) -> i32 {
    let image = current.mm.context.vdso_image;

    vdso_fix_landing(image, new_vma);
    current.mm.context.vdso = (*new_vma).vm_start as *mut core::ffi::c_void;
    vdso_futex_robust_unlock_update_ips();

    0
}

unsafe fn vvar_vclock_fault(
    _sm: *const vm_special_mapping,
    vma: *mut vm_area_struct,
    vmf: *mut vm_fault,
) -> vm_fault_t {
    match (*vmf).pgoff {
        VDSO_PAGE_PVCLOCK_OFFSET => {
            let pvti = pvclock_get_pvti_cpu0_va();
            if !pvti.is_null() && vclock_was_used(VDSO_CLOCKMODE_PVCLOCK) {
                return vmf_insert_pfn_prot(
                    vma,
                    (*vmf).address,
                    __pa(pvti) >> PAGE_SHIFT,
                    pgprot_decrypted((*vma).vm_page_prot),
                );
            }
        }
        VDSO_PAGE_HVCLOCK_OFFSET => {
            let pfn = hv_get_tsc_pfn();
            if pfn != 0 && vclock_was_used(VDSO_CLOCKMODE_HVCLOCK) {
                return vmf_insert_pfn(vma, (*vmf).address, pfn);
            }
        }
        _ => {}
    }

    VM_FAULT_SIGBUS
}

static vdso_mapping: vm_special_mapping = vm_special_mapping {
    name: "[vdso]",
    fault: Some(vdso_fault),
    mremap: Some(vdso_mremap),
};

static vvar_vclock_mapping: vm_special_mapping = vm_special_mapping {
    name: "[vvar_vclock]",
    fault: Some(vvar_vclock_fault),
};

/* Add vdso and vvar mappings to current process. */
unsafe fn map_vdso(image: *const vdso_image, mut addr: usize) -> i32 {
    let mm = current.mm;
    let mut vma: *mut vm_area_struct;
    let mut text_start: usize;
    let mut ret: i32 = 0;

    if mmap_write_lock_killable(mm) != 0 {
        return -EINTR;
    }

    addr = get_unmapped_area(
        core::ptr::null_mut(),
        addr,
        (*image).size + __VDSO_PAGES * PAGE_SIZE,
        0,
        0,
    );
    if IS_ERR_VALUE(addr) {
        ret = addr as i32;
        goto_up_fail!();
    }

    text_start = addr + __VDSO_PAGES * PAGE_SIZE;
    vma = _install_special_mapping(
        mm,
        text_start,
        (*image).size,
        VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC | VM_SEALED_SYSMAP,
        &vdso_mapping,
    );
    if IS_ERR(vma) {
        ret = PTR_ERR(vma);
        goto_up_fail!();
    }

    vma = vdso_install_vvar_mapping(mm, addr);
    if IS_ERR(vma) {
        ret = PTR_ERR(vma);
        do_munmap(mm, text_start, (*image).size, core::ptr::null_mut());
        goto_up_fail!();
    }

    vma = _install_special_mapping(
        mm,
        VDSO_VCLOCK_PAGES_START(addr),
        VDSO_NR_VCLOCK_PAGES * PAGE_SIZE,
        VM_READ | VM_MAYREAD | VM_IO | VM_DONTDUMP | VM_PFNMAP | VM_SEALED_SYSMAP,
        &vvar_vclock_mapping,
    );
    if IS_ERR(vma) {
        ret = PTR_ERR(vma);
        do_munmap(mm, text_start, (*image).size, core::ptr::null_mut());
        do_munmap(mm, addr, VDSO_NR_PAGES * PAGE_SIZE, core::ptr::null_mut());
        goto_up_fail!();
    }

    current.mm.context.vdso = text_start as *mut core::ffi::c_void;
    current.mm.context.vdso_image = image;
    vdso_futex_robust_unlock_update_ips();

    mmap_write_unlock(mm);
    ret
}

pub unsafe fn map_vdso_once(image: *const vdso_image, addr: usize) -> i32 {
    let mm = current.mm;
    let mut vma: *mut vm_area_struct;
    let mut vmi = VMA_ITERATOR!(mm, 0);

    mmap_write_lock(mm);
    for_each_vma!(vmi, vma, {
        if vma_is_special_mapping(vma, &vdso_mapping)
            || vma_is_special_mapping(vma, &vdso_vvar_mapping)
            || vma_is_special_mapping(vma, &vvar_vclock_mapping)
        {
            mmap_write_unlock(mm);
            return -EEXIST;
        }
    });
    mmap_write_unlock(mm);
    map_vdso(image, addr)
}

unsafe fn load_vdso32() -> i32 {
    if vdso32_enabled != 1 {
        return 0;
    }
    map_vdso(&vdso32_image, 0)
}

pub unsafe fn arch_setup_additional_pages(
    _bprm: *mut linux_binprm,
    _uses_interp: i32,
) -> i32 {
    if IS_ENABLED!(CONFIG_X86_64) {
        if VDSO64_ENABLED == 0 {
            return 0;
        }
        return map_vdso(&vdso64_image, 0);
    }
    load_vdso32()
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn compat_arch_setup_additional_pages(
    _bprm: *mut linux_binprm,
    _uses_interp: i32,
    x32: bool,
) -> i32 {
    if IS_ENABLED!(CONFIG_X86_X32_ABI) && x32 {
        if VDSO64_ENABLED == 0 {
            return 0;
        }
        return map_vdso(&vdsox32_image, 0);
    }
    if IS_ENABLED!(CONFIG_IA32_EMULATION) {
        return load_vdso32();
    }
    0
}

pub unsafe fn arch_syscall_is_vdso_sigreturn(regs: *mut pt_regs) -> bool {
    let image = current.mm.context.vdso_image;
    let vdso = current.mm.context.vdso as usize;

    if in_ia32_syscall() && image == &vdso32_image {
        if (*regs).ip == vdso + (*image).sym_vdso32_sigreturn_landing_pad
            || (*regs).ip == vdso + (*image).sym_vdso32_rt_sigreturn_landing_pad
        {
            return true;
        }
    }
    false
}

#[cfg(CONFIG_X86_64)]
unsafe fn vdso_setup(s: *mut u8) -> i32 {
    VDSO64_ENABLED = simple_strtoul(s, core::ptr::null_mut(), 0) as u32;
    1
}

#[cfg(CONFIG_X86_64)]
__setup!("vdso=", vdso_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
