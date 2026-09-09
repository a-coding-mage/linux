// SPDX-License-Identifier: GPL-2.0-only
/*
 * Extensible Firmware Interface
 *
 * Based on Extensible Firmware Interface Specification version 2.4
 *
 * Copyright (C) 2013, 2014 Linaro Ltd.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

unsafe fn region_is_misaligned(md: *const efi_memory_desc_t) -> bool {
    if PAGE_SIZE == EFI_PAGE_SIZE {
        return false;
    }
    !page_aligned((*md).phys_addr) || !page_aligned((*md).num_pages << EFI_PAGE_SHIFT)
}

/*
 * Only regions of type EFI_RUNTIME_SERVICES_CODE need to be
 * executable, everything else can be mapped with the XN bits
 * set. Also take the new (optional) RO/XP bits into account.
 */
unsafe fn create_mapping_protection(md: *mut efi_memory_desc_t) -> ptval_t {
    let attr: u64 = (*md).attribute;
    let type_: u32 = (*md).type_;

    if type_ == EFI_MEMORY_MAPPED_IO {
        let mut prot: pgprot_t = __pgprot(PROT_DEVICE_nGnRE);

        if arm64_is_protected_mmio((*md).phys_addr, (*md).num_pages << EFI_PAGE_SHIFT) {
            prot = pgprot_encrypted(prot);
        } else {
            prot = pgprot_decrypted(prot);
        }
        return pgprot_val(prot);
    }

    if region_is_misaligned(md) {
        static mut CODE_IS_MISALIGNED: bool = false;

        /*
         * Regions that are not aligned to the OS page size cannot be
         * mapped with strict permissions, as those might interfere
         * with the permissions that are needed by the adjacent
         * region's mapping. However, if we haven't encountered any
         * misaligned runtime code regions so far, we can safely use
         * non-executable permissions for non-code regions.
         */
        CODE_IS_MISALIGNED |= type_ == EFI_RUNTIME_SERVICES_CODE;

        return if CODE_IS_MISALIGNED {
            pgprot_val(PAGE_KERNEL_EXEC)
        } else {
            pgprot_val(PAGE_KERNEL)
        };
    }

    /* R-- */
    if (attr & (EFI_MEMORY_XP | EFI_MEMORY_RO)) == (EFI_MEMORY_XP | EFI_MEMORY_RO) {
        return pgprot_val(PAGE_KERNEL_RO);
    }

    /* R-X */
    if attr & EFI_MEMORY_RO != 0 {
        return pgprot_val(PAGE_KERNEL_ROX);
    }

    /* RW- */
    if ((attr & (EFI_MEMORY_RP | EFI_MEMORY_WP | EFI_MEMORY_XP)) == EFI_MEMORY_XP)
        || type_ != EFI_RUNTIME_SERVICES_CODE
    {
        return pgprot_val(PAGE_KERNEL);
    }

    /* RWX */
    pgprot_val(PAGE_KERNEL_EXEC)
}

pub unsafe fn efi_create_mapping(mm: *mut mm_struct, md: *mut efi_memory_desc_t) -> i32 {
    let prot_val: ptval_t = create_mapping_protection(md);
    let mut page_mappings_only: bool = (*md).type_ == EFI_RUNTIME_SERVICES_CODE
        || (*md).type_ == EFI_RUNTIME_SERVICES_DATA;

    /*
     * If this region is not aligned to the page size used by the OS, the
     * mapping will be rounded outwards, and may end up sharing a page
     * frame with an adjacent runtime memory region. Given that the page
     * table descriptor covering the shared page will be rewritten when the
     * adjacent region gets mapped, we must avoid block mappings here so we
     * don't have to worry about splitting them when that happens.
     */
    if region_is_misaligned(md) {
        page_mappings_only = true;
    }

    create_pgd_mapping(
        mm,
        (*md).phys_addr,
        (*md).virt_addr,
        (*md).num_pages << EFI_PAGE_SHIFT,
        __pgprot(prot_val | PTE_NG),
        page_mappings_only,
    );
    0
}

#[repr(C)]
pub struct set_perm_data {
    pub md: *const efi_memory_desc_t,
    pub has_bti: bool,
}

unsafe fn set_permissions(ptep: *mut pte_t, _addr: c_ulong, data: *mut c_void) -> i32 {
    let spd: *mut set_perm_data = data as *mut set_perm_data;
    let md: *const efi_memory_desc_t = (*spd).md;
    let mut pte: pte_t = __ptep_get(ptep);

    if (*md).attribute & EFI_MEMORY_RO != 0 {
        pte = set_pte_bit(pte, __pgprot(PTE_RDONLY));
    }
    if (*md).attribute & EFI_MEMORY_XP != 0 {
        pte = set_pte_bit(pte, __pgprot(PTE_PXN));
    } else if system_supports_bti_kernel() && (*spd).has_bti {
        pte = set_pte_bit(pte, __pgprot(PTE_GP));
    }
    __set_pte(ptep, pte);
    0
}

pub unsafe fn efi_set_mapping_permissions(
    mm: *mut mm_struct,
    md: *mut efi_memory_desc_t,
    has_bti: bool,
) -> i32 {
    let mut data = set_perm_data { md, has_bti };

    BUG_ON((*md).type_ != EFI_RUNTIME_SERVICES_CODE && (*md).type_ != EFI_RUNTIME_SERVICES_DATA);

    if region_is_misaligned(md) {
        return 0;
    }

    /*
     * Calling apply_to_page_range() is only safe on regions that are
     * guaranteed to be mapped down to pages. Since we are only called
     * for regions that have been mapped using efi_create_mapping() above
     * (and this is checked by the generic Memory Attributes table parsing
     * routines), there is no need to check that again here.
     */
    apply_to_page_range(
        mm,
        (*md).virt_addr,
        (*md).num_pages << EFI_PAGE_SHIFT,
        set_permissions,
        &mut data as *mut set_perm_data as *mut c_void,
    )
}

/*
 * UpdateCapsule() depends on the system being shutdown via
 * ResetSystem().
 */
pub unsafe fn efi_poweroff_required() -> bool {
    efi_enabled(EFI_RUNTIME_SERVICES)
}

pub unsafe extern "C" fn efi_handle_corrupted_x18(s: efi_status_t, f: *const c_char) -> efi_status_t {
    pr_err_ratelimited(FW_BUG "register x18 corrupted by EFI %s\n", f);
    s
}

pub unsafe extern "C" fn arch_efi_call_virt_setup() {
    efi_runtime_assert_lock_held();

    if preemptible() && ((*current).flags & PF_KTHREAD) != 0 {
        /*
         * Disable migration to ensure that a preempted EFI runtime
         * service call will be resumed on the same CPU. This avoids
         * potential issues with EFI runtime calls that are preempted
         * while polling for an asynchronous completion of a secure
         * firmware call, which may not permit the CPU to change.
         */
        migrate_disable();
        kthread_use_mm(&efi_mm);
    } else {
        efi_virtmap_load();
    }

    __efi_fpsimd_begin();

    /*
     * Enable access to the valid TTBR0_EL1 and invoke the errata
     * workaround directly since there is no return from exception when
     * invoking the EFI run-time services.
     */
    uaccess_ttbr0_enable();
    post_ttbr_update_workaround();
}

pub unsafe extern "C" fn arch_efi_call_virt_teardown() {
    __efi_fpsimd_end();

    /*
     * Defer the switch to the current thread's TTBR0_EL1 until
     * uaccess_enable(). Do so before efi_virtmap_unload() updates the
     * saved TTBR0 value, so the userland page tables are not activated
     * inadvertently over the back of an exception.
     */
    uaccess_ttbr0_disable();

    if preemptible() && ((*current).flags & PF_KTHREAD) != 0 {
        kthread_unuse_mm(&efi_mm);
        migrate_enable();
    } else {
        efi_virtmap_unload();
    }
}

pub static mut efi_rt_stack_top: *mut u64 = core::ptr::null_mut();

pub unsafe extern "C" fn __efi_rt_asm_recover() -> !;

pub unsafe fn efi_runtime_fixup_exception(regs: *mut pt_regs, msg: *const c_char) -> bool {
    /* Check whether the exception occurred while running the firmware */
    if !current_in_efi() || (*regs).pc >= TASK_SIZE_64 {
        return false;
    }

    pr_err(FW_BUG "Unable to handle %s in EFI runtime service\n", msg);
    add_taint(TAINT_FIRMWARE_WORKAROUND, LOCKDEP_STILL_OK);
    clear_bit(EFI_RUNTIME_SERVICES, &mut efi.flags);

    (*regs).regs[0] = EFI_ABORTED;
    (*regs).regs[30] = *efi_rt_stack_top.offset(-1);
    (*regs).pc = __efi_rt_asm_recover as usize as u64;

    if IS_ENABLED(CONFIG_SHADOW_CALL_STACK) {
        (*regs).regs[18] = *efi_rt_stack_top.offset(-2);
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
