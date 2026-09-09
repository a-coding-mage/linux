// SPDX-License-Identifier: GPL-2.0-or-later

/*
 *    Copyright (C) 2004 Benjamin Herrenschmidt, IBM Corp.
 *                         <benh@kernel.crashing.org>
 */

// Kernel and architecture dependencies are supplied by other translation units.

const VDSO_ALIGNMENT: usize = 1usize << 16;

unsafe extern "C" {
    static mut vdso32_start: u8;
    static mut vdso32_end: u8;
    static mut vdso64_start: u8;
    static mut vdso64_end: u8;
}

unsafe fn vdso_mremap(
    sm: *const vm_special_mapping,
    new_vma: *mut vm_area_struct,
    text_size: usize,
) -> i32 {
    let new_size = unsafe { (*new_vma).vm_end.wrapping_sub((*new_vma).vm_start) };

    if new_size != text_size {
        return -EINVAL;
    }

    unsafe {
        (*current).mm.context.vdso = (*new_vma).vm_start as *mut core::ffi::c_void;
    }

    0
}

unsafe fn vdso32_mremap(sm: *const vm_special_mapping, new_vma: *mut vm_area_struct) -> i32 {
    unsafe {
        vdso_mremap(
            sm,
            new_vma,
            (&vdso32_end as *const u8 as usize).wrapping_sub(&vdso32_start as *const u8 as usize),
        )
    }
}

unsafe fn vdso64_mremap(sm: *const vm_special_mapping, new_vma: *mut vm_area_struct) -> i32 {
    unsafe {
        vdso_mremap(
            sm,
            new_vma,
            (&vdso64_end as *const u8 as usize).wrapping_sub(&vdso64_start as *const u8 as usize),
        )
    }
}

unsafe fn vdso_close(sm: *const vm_special_mapping, vma: *mut vm_area_struct) {
    let mm = unsafe { (*vma).vm_mm };

    /*
     * close() is called for munmap() but also for mremap(). In the mremap()
     * case the vdso pointer has already been updated by the mremap() hook
     * above, so it must not be set to NULL here.
     */
    if unsafe { (*vma).vm_start != (*mm).context.vdso as usize } {
        return;
    }

    unsafe { (*mm).context.vdso = core::ptr::null_mut(); }
}

static mut vdso32_spec: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const i8,
    mremap: Some(vdso32_mremap),
    close: Some(vdso_close),
};

static mut vdso64_spec: vm_special_mapping = vm_special_mapping {
    name: b"[vdso]\0".as_ptr() as *const i8,
    mremap: Some(vdso64_mremap),
    close: Some(vdso_close),
};

/*
 * This is called from binfmt_elf, we create the special vma for the
 * vDSO and insert it into the mm struct tree
 */
unsafe fn __arch_setup_additional_pages(
    bprm: *mut linux_binprm,
    uses_interp: i32,
) -> i32 {
    let (vdso_spec, vdso_size) = if unsafe { is_32bit_task() } {
        (
            unsafe { &mut vdso32_spec as *mut vm_special_mapping },
            unsafe { (&vdso32_end as *const u8 as usize).wrapping_sub(&vdso32_start as *const u8 as usize) },
        )
    } else {
        (
            unsafe { &mut vdso64_spec as *mut vm_special_mapping },
            unsafe { (&vdso64_end as *const u8 as usize).wrapping_sub(&vdso64_start as *const u8 as usize) },
        )
    };
    let vvar_size = VDSO_NR_PAGES * PAGE_SIZE;
    let mm = unsafe { (*current).mm };
    let mut mappings_size = vdso_size + vvar_size;
    mappings_size += (VDSO_ALIGNMENT - 1) & PAGE_MASK;

    /* Pick a base address for the vDSO in process space. */
    let mut vdso_base = unsafe { get_unmapped_area(core::ptr::null_mut(), 0, mappings_size, 0, 0) };
    if unsafe { IS_ERR_VALUE(vdso_base) } {
        return vdso_base as i32;
    }

    vdso_base = (vdso_base + VDSO_ALIGNMENT - 1) & !(VDSO_ALIGNMENT - 1);

    let mut vma = unsafe { vdso_install_vvar_mapping(mm, vdso_base) };
    if unsafe { IS_ERR(vma) } {
        return unsafe { PTR_ERR(vma) } as i32;
    }

    /*
     * our vma flags don't have VM_WRITE so by default, the process isn't
     * allowed to write those pages.
     * gdb can break that with ptrace interface, and thus trigger COW on
     * those pages but it's then your responsibility to never do that on the
     * "data" page of the vDSO or you'll stop getting kernel updates
     * and your nice userland gettimeofday will be totally dead.
     * It's fine to use that for setting breakpoints in the vDSO code
     * pages though.
     */
    vma = unsafe {
        _install_special_mapping(
            mm,
            vdso_base + vvar_size,
            vdso_size,
            VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC,
            vdso_spec,
        )
    };
    if unsafe { IS_ERR(vma) } {
        unsafe { do_munmap(mm, vdso_base, vvar_size, core::ptr::null_mut()); }
        return unsafe { PTR_ERR(vma) } as i32;
    }

    // Now that the mappings are in place, set the mm VDSO pointer
    unsafe { (*mm).context.vdso = (vdso_base + vvar_size) as *mut core::ffi::c_void; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn arch_setup_additional_pages(
    bprm: *mut linux_binprm,
    uses_interp: i32,
) -> i32 {
    let mm = unsafe { (*current).mm };
    unsafe { (*mm).context.vdso = core::ptr::null_mut(); }

    if unsafe { mmap_write_lock_killable(mm) } != 0 {
        return -EINTR;
    }

    let rc = unsafe { __arch_setup_additional_pages(bprm, uses_interp) };
    unsafe { mmap_write_unlock(mm); }
    rc
}

/* Build-time CONFIG_PPC64/CONFIG_VDSO32 sections are preserved by conditional compilation. */
unsafe fn vdso_fixup_features() {
    #[cfg(feature = "CONFIG_PPC64")]
    {
        VDSO_DO_FIXUPS!(feature, cur_cpu_spec.cpu_features, 64, ftr_fixup);
        VDSO_DO_FIXUPS!(feature, cur_cpu_spec.mmu_features, 64, mmu_ftr_fixup);
        VDSO_DO_FIXUPS!(feature, powerpc_firmware_features, 64, fw_ftr_fixup);
        VDSO_DO_FIXUPS!(lwsync, cur_cpu_spec.cpu_features, 64, lwsync_fixup);
    }
    #[cfg(feature = "CONFIG_VDSO32")]
    {
        VDSO_DO_FIXUPS!(feature, cur_cpu_spec.cpu_features, 32, ftr_fixup);
        VDSO_DO_FIXUPS!(feature, cur_cpu_spec.mmu_features, 32, mmu_ftr_fixup);
        #[cfg(feature = "CONFIG_PPC64")]
        VDSO_DO_FIXUPS!(feature, powerpc_firmware_features, 32, fw_ftr_fixup);
        VDSO_DO_FIXUPS!(lwsync, cur_cpu_spec.cpu_features, 32, lwsync_fixup);
    }
}

/* Called from setup_arch to initialize the bitmap of available syscalls. */
unsafe fn vdso_setup_syscall_map() {
    for i in 0..NR_syscalls {
        if unsafe { sys_call_table[i] != &sys_ni_syscall as *const _ as *const core::ffi::c_void } {
            unsafe { vdso_k_arch_data.syscall_map[i >> 5] |= 0x8000_0000u64 >> (i & 0x1f); }
        }
        if unsafe { IS_ENABLED_CONFIG_COMPAT && compat_sys_call_table[i] != &sys_ni_syscall as *const _ as *const core::ffi::c_void } {
            unsafe { vdso_k_arch_data.compat_syscall_map[i >> 5] |= 0x8000_0000u64 >> (i & 0x1f); }
        }
    }
}

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe extern "C" fn vdso_getcpu_init() -> i32 {
    let cpu = unsafe { get_cpu() };
    unsafe { WARN_ON_ONCE(cpu > 0xffff); }
    let node = unsafe { cpu_to_node(cpu) };
    unsafe { WARN_ON_ONCE(node > 0xffff); }
    let val = (cpu & 0xffff) | ((node & 0xffff) << 16);
    unsafe { mtspr(SPRN_SPRG_VDSO_WRITE, val); }
    unsafe { (*get_paca()).sprg_vdso = val; }
    unsafe { put_cpu(); }
    0
}

#[cfg(feature = "CONFIG_PPC64")]
early_initcall!(vdso_getcpu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
