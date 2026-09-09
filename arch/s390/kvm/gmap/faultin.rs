// SPDX-License-Identifier: GPL-2.0
/*
 *  KVM guest fault handling.
 *
 *    Copyright IBM Corp. 2025
 *    Author(s): Claudio Imbrenda <imbrenda@linux.ibm.com>
 */

// Dependencies supplied by the Linux KVM headers and local headers:
// kvm_types.h, kvm_host.h, gmap.h, faultin.h, and trace_gmap.h.

extern "C" {
    pub fn kvm_arch_setup_async_pf(vcpu: *mut kvm_vcpu) -> bool;
}

/*
 * kvm_s390_faultin_gfn() - handle a dat fault.
 * @vcpu: The vCPU whose gmap is to be fixed up, or NULL if operating on the VM.
 * @kvm: The VM whose gmap is to be fixed up, or NULL if operating on a vCPU.
 * @f: The guest fault that needs to be resolved.
 *
 * Return:
 * * 0 on success
 * * < 0 in case of error
 * * > 0 in case of guest exceptions
 *
 * Context:
 * * The mm lock must not be held before calling
 * * kvm->srcu must be held
 * * may sleep
 */
pub unsafe fn kvm_s390_faultin_gfn(
    mut vcpu: *mut kvm_vcpu,
    mut kvm: *mut kvm,
    f: *mut guest_fault,
) -> i32 {
    let mut local_mc: *mut kvm_s390_mmu_cache = core::ptr::null_mut();
    let mut mc: *mut kvm_s390_mmu_cache = core::ptr::null_mut();
    let mut slot: *mut kvm_memory_slot;
    let mut inv_seq: ::core::ffi::c_ulong;
    let mut rc: i32 = -EAGAIN;
    let mut foll: i32;

    foll = if (*f).write_attempt { FOLL_WRITE } else { 0 };
    foll |= if (*f).attempt_pfault { FOLL_NOWAIT } else { 0 };

    if !vcpu.is_null() {
        kvm = (*vcpu).kvm;
        mc = (*vcpu).arch.mc;
    }

    lockdep_assert_held(&(*kvm).srcu);

    // C scoped_guard(read_lock, &kvm->mmu_lock)
    {
        if gmap_try_fixup_minor((*kvm).arch.gmap, f) == 0 {
            return 0;
        }
    }

    if mc.is_null() {
        local_mc = kvm_s390_new_mmu_cache();
        if local_mc.is_null() {
            return -ENOMEM;
        }
        mc = local_mc;
    }

    while rc == -EAGAIN {
        (*f).valid = false;
        inv_seq = (*kvm).mmu_invalidate_seq;
        /* Pairs with the smp_wmb() in kvm_mmu_invalidate_end(). */
        smp_rmb();

        if !vcpu.is_null() {
            slot = kvm_vcpu_gfn_to_memslot(vcpu, (*f).gfn);
        } else {
            slot = gfn_to_memslot(kvm, (*f).gfn);
        }
        (*f).pfn = __kvm_faultin_pfn(slot, (*f).gfn, foll, &mut (*f).writable, &mut (*f).page);

        /* Needs I/O, try to setup async pfault (only possible with FOLL_NOWAIT). */
        if (*f).pfn == KVM_PFN_ERR_NEEDS_IO {
            if !(*f).attempt_pfault {
                return -EAGAIN;
            }
            if vcpu.is_null() {
                return -EINVAL;
            }
            trace_kvm_s390_major_guest_pfault(vcpu);
            if kvm_arch_setup_async_pf(vcpu) {
                return 0;
            }
            (*vcpu).stat.pfault_sync += 1;
            /* Could not setup async pfault, try again synchronously. */
            foll &= !FOLL_NOWAIT;
            (*f).pfn = __kvm_faultin_pfn(slot, (*f).gfn, foll, &mut (*f).writable, &mut (*f).page);
        }

        /* Access outside memory, addressing exception. */
        if is_noslot_pfn((*f).pfn) {
            return PGM_ADDRESSING;
        }
        /* Fatal signal pending: bail out. */
        if is_sigpending_pfn((*f).pfn) {
            return -EINTR;
        }
        /* Check if it's read-only memory; don't try to actually handle that case. */
        if (*f).pfn == KVM_PFN_ERR_RO_FAULT {
            return -EOPNOTSUPP;
        }
        /* Any other error. */
        if is_error_pfn((*f).pfn) {
            return -EFAULT;
        }

        /* Loop, release the faulted page. */
        if mmu_invalidate_retry_gfn_unsafe(kvm, inv_seq, (*f).gfn) {
            kvm_release_faultin_page(kvm, (*f).page, true, false);
            continue;
        }

        // C scoped_guard(read_lock, &kvm->mmu_lock)
        {
            if !mmu_invalidate_retry_gfn(kvm, inv_seq, (*f).gfn) {
                (*f).valid = true;
                rc = gmap_link(mc, (*kvm).arch.gmap, f, slot);
            }
            kvm_release_faultin_page(kvm, (*f).page, rc != 0, (*f).write_attempt);
        }

        if rc == -ENOMEM {
            rc = kvm_s390_mmu_cache_topup(mc);
            if rc != 0 {
                return rc;
            }
            rc = -EAGAIN;
        }
    }

    rc
}

pub unsafe fn kvm_s390_get_guest_page(
    kvm: *mut kvm,
    f: *mut guest_fault,
    gfn: gfn_t,
    w: bool,
) -> i32 {
    let slot: *mut kvm_memory_slot = gfn_to_memslot(kvm, gfn);
    let foll: i32 = if w { FOLL_WRITE } else { 0 };

    (*f).write_attempt = w;
    (*f).gfn = gfn;
    (*f).pfn = __kvm_faultin_pfn(slot, gfn, foll, &mut (*f).writable, &mut (*f).page);
    if is_noslot_pfn((*f).pfn) {
        return PGM_ADDRESSING;
    }
    if is_sigpending_pfn((*f).pfn) {
        return -EINTR;
    }
    if (*f).pfn == KVM_PFN_ERR_NEEDS_IO {
        return -EAGAIN;
    }
    if is_error_pfn((*f).pfn) {
        return -EFAULT;
    }

    (*f).valid = true;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
