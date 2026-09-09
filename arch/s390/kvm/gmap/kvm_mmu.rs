// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit:
// kvm_types.h, kvm_host.h, s390.h, gmap.h, dat.h, and kvm_mmu.h.

/*
 * Get (and clear) the dirty memory log for a memory slot.
 */
pub unsafe fn s390_kvm_mmu_get_dirty_log(
    kvm: *mut kvm,
    log: *mut kvm_dirty_log,
) -> i32 {
    let mut r: i32;
    let mut n: libc::c_ulong;
    let mut memslot: *mut kvm_memory_slot;
    let mut is_dirty: i32;

    if kvm_is_ucontrol(kvm) {
        return -EINVAL;
    }

    mutex_lock(&mut (*kvm).slots_lock);

    r = -EINVAL;
    if (*log).slot >= KVM_USER_MEM_SLOTS {
        mutex_unlock(&mut (*kvm).slots_lock);
        return r;
    }

    r = kvm_get_dirty_log(kvm, log, &mut is_dirty, &mut memslot);
    if r != 0 {
        mutex_unlock(&mut (*kvm).slots_lock);
        return r;
    }

    /* Clear the dirty log */
    if is_dirty != 0 {
        n = kvm_dirty_bitmap_bytes(memslot);
        memset((*memslot).dirty_bitmap as *mut libc::c_void, 0, n);
    }
    r = 0;
    mutex_unlock(&mut (*kvm).slots_lock);
    r
}

pub unsafe fn s390_kvm_mmu_prepare_memory_region(
    kvm: *mut kvm,
    _old: *const kvm_memory_slot,
    new: *mut kvm_memory_slot,
    change: kvm_mr_change,
) -> i32 {
    if kvm_is_ucontrol(kvm) && !new.is_null() && (*new).id < KVM_USER_MEM_SLOTS {
        return -EINVAL;
    }

    /* When we are protected, we should not change the memory slots */
    if kvm_s390_pv_get_handle(kvm) != 0 {
        return -EINVAL;
    }

    if change != KVM_MR_DELETE && change != KVM_MR_FLAGS_ONLY {
        /*
         * A few sanity checks. The memory in userland is ok to be
         * fragmented into various different vmas. It is okay to mmap()
         * and munmap() stuff in this slot after doing this call at any
         * time.
         */
        if (*new).userspace_addr & !PAGE_MASK != 0 {
            return -EINVAL;
        }
        if ((*new).base_gfn + (*new).npages) * PAGE_SIZE > (*kvm).arch.mem_limit {
            return -EINVAL;
        }
        if !asce_contains_gfn(
            (*(*kvm).arch.gmap).asce,
            (*new).base_gfn + (*new).npages - 1,
        ) {
            return -EINVAL;
        }
    }

    if !kvm_s390_is_migration_mode(kvm) {
        return 0;
    }

    /*
     * Turn off migration mode when:
     * - userspace creates a new memslot with dirty logging off,
     * - userspace modifies an existing memslot (MOVE or FLAGS_ONLY) and
     *   dirty logging is turned off.
     * Migration mode expects dirty page logging being enabled to store
     * its dirty bitmap.
     */
    if change != KVM_MR_DELETE && (*new).flags & KVM_MEM_LOG_DIRTY_PAGES == 0 {
        WARN(
            kvm_s390_vm_stop_migration(kvm),
            "Failed to stop migration mode",
        );
    }

    0
}

pub unsafe fn s390_kvm_mmu_commit_memory_region(
    kvm: *mut kvm,
    old: *mut kvm_memory_slot,
    new: *const kvm_memory_slot,
    change: kvm_mr_change,
) {
    // C __free(kvm_s390_mmu_cache) cleanup is represented by the explicit
    // ownership/cleanup convention supplied by the surrounding translation.
    let mut mc: *mut kvm_s390_mmu_cache = core::ptr::null_mut();
    let mut rc: i32 = 0;

    // guard(mutex)(&kvm->slots_arch_lock);
    mutex_lock(&mut (*kvm).slots_arch_lock);

    if change == KVM_MR_FLAGS_ONLY {
        mutex_unlock(&mut (*kvm).slots_arch_lock);
        return;
    }

    mc = kvm_s390_new_mmu_cache();
    if mc.is_null() {
        rc = -ENOMEM;
        mutex_unlock(&mut (*kvm).slots_arch_lock);
        if rc != 0 {
            pr_warn("failed to commit memory region\n");
        }
        return;
    }

    // scoped_guard(write_lock, &kvm->mmu_lock)
    write_lock(&mut (*kvm).mmu_lock);
    kvm_s390_update_cmma_dirty(kvm, old);
    match change {
        KVM_MR_DELETE => {
            rc = dat_delete_slot(
                mc,
                (*(*kvm).arch.gmap).asce,
                (*old).base_gfn,
                (*old).npages,
            );
        }
        KVM_MR_MOVE => {
            rc = dat_delete_slot(
                mc,
                (*(*kvm).arch.gmap).asce,
                (*old).base_gfn,
                (*old).npages,
            );
            if rc == 0 {
                rc = dat_create_slot(
                    mc,
                    (*(*kvm).arch.gmap).asce,
                    (*new).base_gfn,
                    (*new).npages,
                );
            }
        }
        KVM_MR_CREATE => {
            rc = dat_create_slot(
                mc,
                (*(*kvm).arch.gmap).asce,
                (*new).base_gfn,
                (*new).npages,
            );
        }
        KVM_MR_FLAGS_ONLY => {}
        _ => {
            WARN(1, "Unknown KVM MR CHANGE: %d\n", change);
        }
    }
    write_unlock(&mut (*kvm).mmu_lock);
    mutex_unlock(&mut (*kvm).slots_arch_lock);

    if rc != 0 {
        pr_warn("failed to commit memory region\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
