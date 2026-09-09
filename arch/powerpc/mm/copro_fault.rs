// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CoProcessor (SPU/AFU) mm fault handler
 *
 * (C) Copyright IBM Deutschland Entwicklung GmbH 2007
 *
 * Author: Arnd Bergmann <arndb@de.ibm.com>
 * Author: Jeremy Kerr <jk@ozlabs.org>
 */

/*
 * This ought to be kept in sync with the powerpc specific do_page_fault
 * function. Currently, there are a few corner cases that we haven't had
 * to handle fortunately.
 */
pub unsafe fn copro_handle_mm_fault(
    mm: *mut mm_struct,
    ea: c_ulong,
    dsisr: c_ulong,
    flt: *mut vm_fault_t,
) -> c_int {
    let mut vma: *mut vm_area_struct;
    let is_write: c_ulong;
    let mut ret: c_int;

    if mm.is_null() {
        return -EFAULT;
    }

    if (*mm).pgd.is_null() {
        return -EFAULT;
    }

    vma = lock_mm_and_find_vma(mm, ea, core::ptr::null_mut());
    if vma.is_null() {
        return -EFAULT;
    }

    ret = -EFAULT;
    is_write = dsisr & DSISR_ISSTORE;
    if is_write != 0 {
        if (*vma).vm_flags & VM_WRITE == 0 {
            mmap_read_unlock(mm);
            return ret;
        }
    } else {
        if (*vma).vm_flags & (VM_READ | VM_EXEC) == 0 {
            mmap_read_unlock(mm);
            return ret;
        }
        /*
         * PROT_NONE is covered by the VMA check above.
         * and hash should get a NOHPTE fault instead of
         * a PROTFAULT in case fixup is needed for things
         * like autonuma.
         */
        if !radix_enabled() {
            WARN_ON_ONCE(dsisr & DSISR_PROTFAULT);
        }
    }

    ret = 0;
    *flt = handle_mm_fault(
        vma,
        ea,
        if is_write != 0 { FAULT_FLAG_WRITE } else { 0 },
        core::ptr::null_mut(),
    );

    /* The fault is fully completed (including releasing mmap lock) */
    if *flt & VM_FAULT_COMPLETED != 0 {
        return 0;
    }

    if *flt & VM_FAULT_ERROR != 0 {
        if *flt & VM_FAULT_OOM != 0 {
            ret = -ENOMEM;
            mmap_read_unlock(mm);
            return ret;
        } else if *flt & (VM_FAULT_SIGBUS | VM_FAULT_SIGSEGV) != 0 {
            ret = -EFAULT;
            mmap_read_unlock(mm);
            return ret;
        }
        BUG!();
    }

    mmap_read_unlock(mm);
    ret
}

// EXPORT_SYMBOL_GPL(copro_handle_mm_fault);

/* Preserved from the build-time CONFIG_PPC_64S_HASH_MMU conditional. */
#[cfg(CONFIG_PPC_64S_HASH_MMU)]
pub unsafe fn copro_calculate_slb(
    mm: *mut mm_struct,
    ea: u64,
    slb: *mut copro_slb,
) -> c_int {
    let (mut psize, mut ssize): (c_int, c_int);
    let mut vsid: u64;
    let vsidkey: u64;

    match get_region_id(ea) {
        USER_REGION_ID => {
            pr_devel!("%s: 0x%llx -- USER_REGION_ID\n", __func__, ea);
            if mm.is_null() {
                return 1;
            }
            psize = get_slice_psize(mm, ea);
            ssize = user_segment_size(ea);
            vsid = get_user_vsid(&mut (*mm).context, ea, ssize);
            vsidkey = SLB_VSID_USER;
        }
        VMALLOC_REGION_ID => {
            pr_devel!("%s: 0x%llx -- VMALLOC_REGION_ID\n", __func__, ea);
            psize = mmu_vmalloc_psize;
            ssize = mmu_kernel_ssize;
            vsid = get_kernel_vsid(ea, mmu_kernel_ssize);
            vsidkey = SLB_VSID_KERNEL;
        }
        IO_REGION_ID => {
            pr_devel!("%s: 0x%llx -- IO_REGION_ID\n", __func__, ea);
            psize = mmu_io_psize;
            ssize = mmu_kernel_ssize;
            vsid = get_kernel_vsid(ea, mmu_kernel_ssize);
            vsidkey = SLB_VSID_KERNEL;
        }
        LINEAR_MAP_REGION_ID => {
            pr_devel!("%s: 0x%llx -- LINEAR_MAP_REGION_ID\n", __func__, ea);
            psize = mmu_linear_psize;
            ssize = mmu_kernel_ssize;
            vsid = get_kernel_vsid(ea, mmu_kernel_ssize);
            vsidkey = SLB_VSID_KERNEL;
        }
        _ => {
            pr_debug!("%s: invalid region access at %016llx\n", __func__, ea);
            return 1;
        }
    }
    /* Bad address */
    if vsid == 0 {
        return 1;
    }

    vsid = (vsid << slb_vsid_shift(ssize)) | vsidkey;

    vsid |= mmu_psize_defs[psize as usize].sllp
        | if ssize == MMU_SEGSIZE_1T { SLB_VSID_B_1T } else { 0 };

    (*slb).esid = (ea & if ssize == MMU_SEGSIZE_1T { ESID_MASK_1T } else { ESID_MASK }) | SLB_ESID_V;
    (*slb).vsid = vsid;

    0
}

// EXPORT_SYMBOL_GPL(copro_calculate_slb);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
