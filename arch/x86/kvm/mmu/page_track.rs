// SPDX-License-Identifier: GPL-2.0-only
/*
 * Support KVM guest page tracking
 *
 * This feature allows us to track page access in guest. Currently, only
 * write access is tracked.
 *
 * Copyright(C) 2015 Intel Corporation.
 *
 * Author:
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 */

// C includes: linux/lockdep.h, linux/kvm_host.h, linux/rculist.h,
// "mmu.h", "mmu_internal.h", and "page_track.h".

unsafe fn kvm_external_write_tracking_enabled(kvm: *mut kvm) -> bool {
    #[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
    {
        // Read external_write_tracking_enabled before related pointers. Pairs
        // with the smp_store_release in kvm_page_track_write_tracking_enable().
        return smp_load_acquire(&(*kvm).arch.external_write_tracking_enabled);
    }
    #[cfg(not(CONFIG_KVM_EXTERNAL_WRITE_TRACKING))]
    {
        let _ = kvm;
        false
    }
}

pub unsafe fn kvm_page_track_write_tracking_enabled(kvm: *mut kvm) -> bool {
    kvm_external_write_tracking_enabled(kvm)
        || kvm_shadow_root_allocated(kvm)
        || !tdp_enabled
}

pub unsafe fn kvm_page_track_free_memslot(slot: *mut kvm_memory_slot) {
    vfree((*slot).arch.gfn_write_track);
    (*slot).arch.gfn_write_track = core::ptr::null_mut();
}

unsafe fn __kvm_page_track_write_tracking_alloc(
    slot: *mut kvm_memory_slot,
    npages: c_ulong,
) -> c_int {
    let size: usize = core::mem::size_of_val(&(*slot).arch.gfn_write_track);

    if (*slot).arch.gfn_write_track.is_null() {
        (*slot).arch.gfn_write_track =
            __vcalloc(npages as usize, size, GFP_KERNEL_ACCOUNT);
    }

    if !(*slot).arch.gfn_write_track.is_null() { 0 } else { -ENOMEM }
}

pub unsafe fn kvm_page_track_create_memslot(
    kvm: *mut kvm,
    slot: *mut kvm_memory_slot,
    npages: c_ulong,
) -> c_int {
    if !kvm_page_track_write_tracking_enabled(kvm) {
        return 0;
    }
    __kvm_page_track_write_tracking_alloc(slot, npages)
}

pub unsafe fn kvm_page_track_write_tracking_alloc(slot: *mut kvm_memory_slot) -> c_int {
    __kvm_page_track_write_tracking_alloc(slot, (*slot).npages)
}

unsafe fn update_gfn_write_track(slot: *mut kvm_memory_slot, gfn: gfn_t, count: i16) {
    let index: c_int = gfn_to_index(gfn, (*slot).base_gfn, PG_LEVEL_4K);
    let val: u16 = *(*slot).arch.gfn_write_track.add(index as usize);
    let next = val as i32 + count as i32;

    if WARN_ON_ONCE(next < 0 || next > USHRT_MAX as i32) {
        return;
    }
    *(*slot).arch.gfn_write_track.add(index as usize) = next as u16;
}

pub unsafe fn __kvm_write_track_add_gfn(
    kvm: *mut kvm,
    slot: *mut kvm_memory_slot,
    gfn: gfn_t,
) {
    lockdep_assert_held_write(&(*kvm).mmu_lock);
    lockdep_assert_once(lockdep_is_held(&(*kvm).slots_lock) || srcu_read_lock_held(&(*kvm).srcu));
    if KVM_BUG_ON(!kvm_page_track_write_tracking_enabled(kvm), kvm) { return; }
    update_gfn_write_track(slot, gfn, 1);
    kvm_mmu_gfn_disallow_lpage(slot, gfn);
    if kvm_mmu_slot_gfn_write_protect(kvm, slot, gfn, PG_LEVEL_4K) {
        kvm_flush_remote_tlbs(kvm);
    }
}

pub unsafe fn __kvm_write_track_remove_gfn(
    kvm: *mut kvm,
    slot: *mut kvm_memory_slot,
    gfn: gfn_t,
) {
    lockdep_assert_held_write(&(*kvm).mmu_lock);
    lockdep_assert_once(lockdep_is_held(&(*kvm).slots_lock) || srcu_read_lock_held(&(*kvm).srcu));
    if KVM_BUG_ON(!kvm_page_track_write_tracking_enabled(kvm), kvm) { return; }
    update_gfn_write_track(slot, gfn, -1);
    kvm_mmu_gfn_allow_lpage(slot, gfn);
}

unsafe fn __kvm_gfn_is_write_tracked(slot: *const kvm_memory_slot, gfn: gfn_t) -> bool {
    if slot.is_null() { return false; }
    let index = gfn_to_index(gfn, (*slot).base_gfn, PG_LEVEL_4K);
    READ_ONCE(*(*slot).arch.gfn_write_track.add(index as usize)) != 0
}

pub unsafe fn kvm_gfn_is_write_tracked(
    kvm: *mut kvm,
    slot: *const kvm_memory_slot,
    gfn: gfn_t,
) -> bool {
    if slot.is_null() || !kvm_page_track_write_tracking_enabled(kvm) { return false; }
    BUILD_BUG_ON!(KVM_MAX_NR_ADDRESS_SPACES > 2);
    if __kvm_gfn_is_write_tracked(slot, gfn) { return true; }
    if kvm_arch_nr_memslot_as_ids(kvm) > 1 {
        let other_slot = __gfn_to_memslot(__kvm_memslots(kvm, (*slot).as_id ^ 1), gfn);
        if __kvm_gfn_is_write_tracked(other_slot, gfn) { return true; }
    }
    false
}

#[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
pub unsafe fn kvm_page_track_cleanup(kvm: *mut kvm) {
    let head = &mut (*kvm).arch.track_notifier_head;
    cleanup_srcu_struct(&mut head.track_srcu);
}

#[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
pub unsafe fn kvm_page_track_init(kvm: *mut kvm) -> c_int {
    let head = &mut (*kvm).arch.track_notifier_head;
    INIT_HLIST_HEAD(&mut head.track_notifier_list);
    init_srcu_struct(&mut head.track_srcu)
}

// The remaining external-notifier implementation is conditionally compiled
// in the C source and retains its source-level locking and SRCU traversal.
#[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
pub unsafe fn kvm_enable_external_write_tracking(kvm: *mut kvm) -> c_int {
    if (*kvm).arch.vm_type == KVM_X86_TDX_VM { return -EOPNOTSUPP; }
    mutex_lock(&mut (*kvm).slots_arch_lock);
    if kvm_page_track_write_tracking_enabled(kvm) { goto_out_success!(); }
    for i in 0..kvm_arch_nr_memslot_as_ids(kvm) {
        let slots = __kvm_memslots(kvm, i);
        let mut slot: *mut kvm_memory_slot = core::ptr::null_mut();
        let mut bkt = 0;
        kvm_for_each_memslot!(slot, bkt, slots, {
            let r = kvm_page_track_write_tracking_alloc(slot);
            if r != 0 { mutex_unlock(&mut (*kvm).slots_arch_lock); return r; }
        });
    }
    smp_store_release(&mut (*kvm).arch.external_write_tracking_enabled, true);
    mutex_unlock(&mut (*kvm).slots_arch_lock);
    0
}

// Remaining notifier entry points preserve the C declarations and are
// intentionally expressed using the dependency-provided list/SRCU helpers.
#[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
pub unsafe fn kvm_page_track_register_notifier(
    kvm: *mut kvm,
    n: *mut kvm_page_track_notifier_node,
) -> c_int {
    if kvm.is_null() || (*kvm).mm != current().mm { return -ESRCH; }
    if !kvm_external_write_tracking_enabled(kvm) {
        let r = kvm_enable_external_write_tracking(kvm);
        if r != 0 { return r; }
    }
    kvm_get_kvm(kvm);
    let head = &mut (*kvm).arch.track_notifier_head;
    write_lock(&mut (*kvm).mmu_lock);
    hlist_add_head_rcu(&mut (*n).node, &mut head.track_notifier_list);
    write_unlock(&mut (*kvm).mmu_lock);
    0
}

#[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
pub unsafe fn kvm_page_track_unregister_notifier(
    kvm: *mut kvm,
    n: *mut kvm_page_track_notifier_node,
) {
    let head = &mut (*kvm).arch.track_notifier_head;
    write_lock(&mut (*kvm).mmu_lock);
    hlist_del_rcu(&mut (*n).node);
    write_unlock(&mut (*kvm).mmu_lock);
    synchronize_srcu(&mut head.track_srcu);
    kvm_put_kvm(kvm);
}

#[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
pub unsafe fn __kvm_page_track_write(kvm: *mut kvm, gpa: gpa_t, new: *const u8, bytes: c_int) {
    let head = &mut (*kvm).arch.track_notifier_head;
    if hlist_empty(&head.track_notifier_list) { return; }
    let idx = srcu_read_lock(&mut head.track_srcu);
    let mut n: *mut kvm_page_track_notifier_node = core::ptr::null_mut();
    hlist_for_each_entry_srcu!(n, &mut head.track_notifier_list, node, {
        if let Some(track_write) = (*n).track_write { track_write(gpa, new, bytes, n); }
    });
    srcu_read_unlock(&mut head.track_srcu, idx);
}

#[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
pub unsafe fn kvm_page_track_delete_slot(kvm: *mut kvm, slot: *mut kvm_memory_slot) {
    let head = &mut (*kvm).arch.track_notifier_head;
    if hlist_empty(&head.track_notifier_list) { return; }
    let idx = srcu_read_lock(&mut head.track_srcu);
    let mut n: *mut kvm_page_track_notifier_node = core::ptr::null_mut();
    hlist_for_each_entry_srcu!(n, &mut head.track_notifier_list, node, {
        if let Some(remove) = (*n).track_remove_region {
            remove((*slot).base_gfn, (*slot).npages, n);
        }
    });
    srcu_read_unlock(&mut head.track_srcu, idx);
}

#[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
pub unsafe fn kvm_write_track_add_gfn(kvm: *mut kvm, gfn: gfn_t) -> c_int {
    let idx = srcu_read_lock(&mut (*kvm).srcu);
    let slot = gfn_to_memslot(kvm, gfn);
    if slot.is_null() { srcu_read_unlock(&mut (*kvm).srcu, idx); return -EINVAL; }
    write_lock(&mut (*kvm).mmu_lock);
    __kvm_write_track_add_gfn(kvm, slot, gfn);
    write_unlock(&mut (*kvm).mmu_lock);
    srcu_read_unlock(&mut (*kvm).srcu, idx);
    0
}

#[cfg(CONFIG_KVM_EXTERNAL_WRITE_TRACKING)]
pub unsafe fn kvm_write_track_remove_gfn(kvm: *mut kvm, gfn: gfn_t) -> c_int {
    let idx = srcu_read_lock(&mut (*kvm).srcu);
    let slot = gfn_to_memslot(kvm, gfn);
    if slot.is_null() { srcu_read_unlock(&mut (*kvm).srcu, idx); return -EINVAL; }
    write_lock(&mut (*kvm).mmu_lock);
    __kvm_write_track_remove_gfn(kvm, slot, gfn);
    write_unlock(&mut (*kvm).mmu_lock);
    srcu_read_unlock(&mut (*kvm).srcu, idx);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
