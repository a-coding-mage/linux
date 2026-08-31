// SPDX-License-Identifier: GPL-2.0-only
/*
 * KVM dirty ring implementation
 *
 * Copyright 2019 Red Hat, Inc.
 */

// Dependencies from:
// <linux/kvm_host.h>, <linux/kvm.h>, <linux/vmalloc.h>,
// <linux/kvm_dirty_ring.h>, <trace/events/kvm.h>, and "kvm_mm.h".

use core::ffi::c_void;

pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type s64 = i64;

pub const KVM_DIRTY_RING_RSVD_ENTRIES: u32 = 0; // external constant from KVM headers
pub const KVM_USER_MEM_SLOTS: i32 = 0; // external constant from KVM headers
pub const KVM_DIRTY_GFN_F_DIRTY: u32 = 0; // external constant from KVM headers
pub const KVM_DIRTY_GFN_F_RESET: u32 = 0; // external constant from KVM headers
pub const KVM_REQ_DIRTY_RING_SOFT_FULL: i32 = 0; // external constant from KVM headers
pub const KVM_EXIT_DIRTY_RING_FULL: u32 = 0; // external constant from KVM headers
pub const PAGE_SIZE: u32 = 0; // external constant from kernel headers
pub const INT_MAX: i32 = i32::MAX;
pub const ENOMEM: i32 = 12;
pub const EINTR: i32 = 4;
pub const BITS_PER_LONG: s64 = core::mem::size_of::<usize>() as s64 * 8;

#[repr(C)]
pub struct kvm {
    pub slots_lock: c_void,
    pub dirty_ring_size: u32,
    pub dirty_ring_with_bitmap: bool,
}

#[repr(C)]
pub struct kvm_vcpu {
    pub dirty_ring: kvm_dirty_ring,
    pub run: *mut kvm_run,
}

#[repr(C)]
pub struct kvm_run {
    pub exit_reason: u32,
}

#[repr(C)]
pub struct kvm_dirty_ring {
    pub dirty_gfns: *mut kvm_dirty_gfn,
    pub size: u32,
    pub soft_limit: u32,
    pub dirty_index: u32,
    pub reset_index: u32,
    pub index: i32,
}

#[repr(C)]
pub struct kvm_dirty_gfn {
    pub flags: u32,
    pub slot: u32,
    pub offset: u64,
}

#[repr(C)]
pub struct kvm_memory_slot {
    pub npages: u64,
}

#[repr(C)]
pub struct kvm_memslots {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    static current: *mut c_void;

    fn lockdep_assert_held(lock: *const c_void);
    fn READ_ONCE_u32(p: *const u32) -> u32;
    fn READ_ONCE_u64(p: *const u64) -> u64;
    fn smp_store_release(p: *mut u32, v: u32);
    fn smp_load_acquire(p: *const u32) -> u32;
    fn smp_wmb();
    fn vzalloc(size: u32) -> *mut c_void;
    fn vfree(addr: *mut c_void);
    fn vmalloc_to_page(addr: *mut c_void) -> *mut page;
    fn kvm_arch_nr_memslot_as_ids(kvm: *mut kvm) -> i32;
    fn __kvm_memslots(kvm: *mut kvm, as_id: i32) -> *mut kvm_memslots;
    fn id_to_memslot(slots: *mut kvm_memslots, id: i32) -> *mut kvm_memory_slot;
    fn KVM_MMU_LOCK(kvm: *mut kvm);
    fn KVM_MMU_UNLOCK(kvm: *mut kvm);
    fn kvm_arch_mmu_enable_log_dirty_pt_masked(
        kvm: *mut kvm,
        memslot: *mut kvm_memory_slot,
        offset: u64,
        mask: usize,
    );
    fn signal_pending(task: *mut c_void) -> bool;
    fn cond_resched();
    fn trace_kvm_dirty_ring_reset(ring: *mut kvm_dirty_ring);
    fn trace_kvm_dirty_ring_push(ring: *mut kvm_dirty_ring, slot: u32, offset: u64);
    fn trace_kvm_dirty_ring_exit(vcpu: *mut kvm_vcpu);
    fn WARN_ON_ONCE(condition: bool) -> bool;
    fn kvm_make_request(req: i32, vcpu: *mut kvm_vcpu);
    fn kvm_check_request(req: i32, vcpu: *mut kvm_vcpu) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn kvm_cpu_dirty_log_size(_kvm: *mut kvm) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn kvm_dirty_ring_get_rsvd_entries(kvm: *mut kvm) -> u32 {
    KVM_DIRTY_RING_RSVD_ENTRIES.wrapping_add(kvm_cpu_dirty_log_size(kvm) as u32)
}

#[no_mangle]
pub unsafe extern "C" fn kvm_use_dirty_bitmap(kvm: *mut kvm) -> bool {
    lockdep_assert_held(core::ptr::addr_of!((*kvm).slots_lock));

    (*kvm).dirty_ring_size == 0 || (*kvm).dirty_ring_with_bitmap
}

// When CONFIG_NEED_KVM_DIRTY_RING_WITH_BITMAP is not defined.
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_allow_write_without_running_vcpu(_kvm: *mut kvm) -> bool {
    false
}

unsafe fn read_once_u32(p: *const u32) -> u32 {
    READ_ONCE_u32(p)
}

unsafe fn read_once_u64(p: *const u64) -> u64 {
    READ_ONCE_u64(p)
}

unsafe fn kvm_dirty_ring_used(ring: *mut kvm_dirty_ring) -> u32 {
    read_once_u32(core::ptr::addr_of!((*ring).dirty_index))
        .wrapping_sub(read_once_u32(core::ptr::addr_of!((*ring).reset_index)))
}

unsafe fn kvm_dirty_ring_soft_full(ring: *mut kvm_dirty_ring) -> bool {
    kvm_dirty_ring_used(ring) >= (*ring).soft_limit
}

unsafe fn kvm_dirty_ring_full(ring: *mut kvm_dirty_ring) -> bool {
    kvm_dirty_ring_used(ring) >= (*ring).size
}

unsafe fn fls64(mask: u64) -> u32 {
    63u32.wrapping_sub(mask.leading_zeros())
}

unsafe fn kvm_reset_dirty_gfn(kvm: *mut kvm, slot: u32, offset: u64, mask: usize) {
    let memslot: *mut kvm_memory_slot;
    let as_id: i32;
    let id: i32;

    as_id = (slot >> 16) as i32;
    id = (slot as u16) as i32;

    if as_id >= kvm_arch_nr_memslot_as_ids(kvm) || id >= KVM_USER_MEM_SLOTS {
        return;
    }

    memslot = id_to_memslot(__kvm_memslots(kvm, as_id), id);

    if memslot.is_null()
        || offset >= (*memslot).npages
        || offset.wrapping_add(fls64(mask as u64) as u64) >= (*memslot).npages
    {
        return;
    }

    KVM_MMU_LOCK(kvm);
    kvm_arch_mmu_enable_log_dirty_pt_masked(kvm, memslot, offset, mask);
    KVM_MMU_UNLOCK(kvm);
}

#[no_mangle]
pub unsafe extern "C" fn kvm_dirty_ring_alloc(
    kvm: *mut kvm,
    ring: *mut kvm_dirty_ring,
    index: i32,
    size: u32,
) -> i32 {
    (*ring).dirty_gfns = vzalloc(size) as *mut kvm_dirty_gfn;
    if (*ring).dirty_gfns.is_null() {
        return -ENOMEM;
    }

    (*ring).size = size / core::mem::size_of::<kvm_dirty_gfn>() as u32;
    (*ring).soft_limit = (*ring)
        .size
        .wrapping_sub(kvm_dirty_ring_get_rsvd_entries(kvm));
    (*ring).dirty_index = 0;
    (*ring).reset_index = 0;
    (*ring).index = index;

    0
}

unsafe fn kvm_dirty_gfn_set_invalid(gfn: *mut kvm_dirty_gfn) {
    smp_store_release(core::ptr::addr_of_mut!((*gfn).flags), 0);
}

unsafe fn kvm_dirty_gfn_set_dirtied(gfn: *mut kvm_dirty_gfn) {
    (*gfn).flags = KVM_DIRTY_GFN_F_DIRTY;
}

unsafe fn kvm_dirty_gfn_harvested(gfn: *mut kvm_dirty_gfn) -> bool {
    smp_load_acquire(core::ptr::addr_of!((*gfn).flags)) & KVM_DIRTY_GFN_F_RESET != 0
}

#[no_mangle]
pub unsafe extern "C" fn kvm_dirty_ring_reset(
    kvm: *mut kvm,
    ring: *mut kvm_dirty_ring,
    nr_entries_reset: *mut i32,
) -> i32 {
    /*
     * To minimize mmu_lock contention, batch resets for harvested entries
     * whose gfns are in the same slot, and are within N frame numbers of
     * each other, where N is the number of bits in an unsigned long.  For
     * simplicity, process the current set of entries when the next entry
     * can't be included in the batch.
     *
     * Track the current batch slot, the gfn offset into the slot for the
     * batch, and the bitmask of gfns that need to be reset (relative to
     * offset).  Note, the offset may be adjusted backwards, e.g. so that
     * a sequence of gfns X, X-1, ... X-N-1 can be batched.
     */
    let mut cur_slot: u32 = 0;
    let mut next_slot: u32;
    let mut cur_offset: u64 = 0;
    let mut next_offset: u64;
    let mut mask: usize = 0;
    let mut entry: *mut kvm_dirty_gfn;

    /*
     * Ensure concurrent calls to KVM_RESET_DIRTY_RINGS are serialized,
     * e.g. so that KVM fully resets all entries processed by a given call
     * before returning to userspace.  Holding slots_lock also protects
     * the various memslot accesses.
     */
    lockdep_assert_held(core::ptr::addr_of!((*kvm).slots_lock));

    while *nr_entries_reset < INT_MAX {
        if signal_pending(current) {
            return -EINTR;
        }

        entry = (*ring)
            .dirty_gfns
            .add(((*ring).reset_index & ((*ring).size - 1)) as usize);

        if !kvm_dirty_gfn_harvested(entry) {
            break;
        }

        next_slot = read_once_u32(core::ptr::addr_of!((*entry).slot));
        next_offset = read_once_u64(core::ptr::addr_of!((*entry).offset));

        /* Update the flags to reflect that this GFN is reset */
        kvm_dirty_gfn_set_invalid(entry);

        (*ring).reset_index = (*ring).reset_index.wrapping_add(1);
        *nr_entries_reset += 1;

        if mask != 0 {
            /*
             * While the size of each ring is fixed, it's possible
             * for the ring to be constantly re-dirtied/harvested
             * while the reset is in-progress (the hard limit exists
             * only to guard against the count becoming negative).
             */
            cond_resched();

            /*
             * Try to coalesce the reset operations when the guest
             * is scanning pages in the same slot.
             */
            if next_slot == cur_slot {
                let delta: s64 = next_offset.wrapping_sub(cur_offset) as s64;

                if delta >= 0 && delta < BITS_PER_LONG {
                    mask |= 1usize << delta;
                    continue;
                }

                /* Backwards visit, careful about overflows! */
                if delta > -BITS_PER_LONG
                    && delta < 0
                    && ((mask << -delta) >> -delta) == mask
                {
                    cur_offset = next_offset;
                    mask = (mask << -delta) | 1;
                    continue;
                }
            }

            /*
             * Reset the slot for all the harvested entries that
             * have been gathered, but not yet fully processed.
             */
            kvm_reset_dirty_gfn(kvm, cur_slot, cur_offset, mask);
        }

        /*
         * The current slot was reset or this is the first harvested
         * entry, (re)initialize the metadata.
         */
        cur_slot = next_slot;
        cur_offset = next_offset;
        mask = 1;
    }

    /*
     * Perform a final reset if there are harvested entries that haven't
     * been processed, which is guaranteed if at least one harvested was
     * found.  The loop only performs a reset when the "next" entry can't
     * be batched with the "current" entry(s), and that reset processes the
     * _current_ entry(s); i.e. the last harvested entry, a.k.a. next, will
     * always be left pending.
     */
    if mask != 0 {
        kvm_reset_dirty_gfn(kvm, cur_slot, cur_offset, mask);
    }

    /*
     * The request KVM_REQ_DIRTY_RING_SOFT_FULL will be cleared
     * by the VCPU thread next time when it enters the guest.
     */

    trace_kvm_dirty_ring_reset(ring);

    0
}

#[no_mangle]
pub unsafe extern "C" fn kvm_dirty_ring_push(vcpu: *mut kvm_vcpu, slot: u32, offset: u64) {
    let ring: *mut kvm_dirty_ring = core::ptr::addr_of_mut!((*vcpu).dirty_ring);
    let entry: *mut kvm_dirty_gfn;

    /* It should never get full */
    WARN_ON_ONCE(kvm_dirty_ring_full(ring));

    entry = (*ring)
        .dirty_gfns
        .add(((*ring).dirty_index & ((*ring).size - 1)) as usize);

    (*entry).slot = slot;
    (*entry).offset = offset;
    /*
     * Make sure the data is filled in before we publish this to
     * the userspace program.  There's no paired kernel-side reader.
     */
    smp_wmb();
    kvm_dirty_gfn_set_dirtied(entry);
    (*ring).dirty_index = (*ring).dirty_index.wrapping_add(1);
    trace_kvm_dirty_ring_push(ring, slot, offset);

    if kvm_dirty_ring_soft_full(ring) {
        kvm_make_request(KVM_REQ_DIRTY_RING_SOFT_FULL, vcpu);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_dirty_ring_check_request(vcpu: *mut kvm_vcpu) -> bool {
    /*
     * The VCPU isn't runnable when the dirty ring becomes soft full.
     * The KVM_REQ_DIRTY_RING_SOFT_FULL event is always set to prevent
     * the VCPU from running until the dirty pages are harvested and
     * the dirty ring is reset by userspace.
     */
    if kvm_check_request(KVM_REQ_DIRTY_RING_SOFT_FULL, vcpu)
        && kvm_dirty_ring_soft_full(core::ptr::addr_of_mut!((*vcpu).dirty_ring))
    {
        kvm_make_request(KVM_REQ_DIRTY_RING_SOFT_FULL, vcpu);
        (*(*vcpu).run).exit_reason = KVM_EXIT_DIRTY_RING_FULL;
        trace_kvm_dirty_ring_exit(vcpu);
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn kvm_dirty_ring_get_page(
    ring: *mut kvm_dirty_ring,
    offset: u32,
) -> *mut page {
    vmalloc_to_page(((*ring).dirty_gfns as *mut u8).add((offset * PAGE_SIZE) as usize) as *mut c_void)
}

#[no_mangle]
pub unsafe extern "C" fn kvm_dirty_ring_free(ring: *mut kvm_dirty_ring) {
    vfree((*ring).dirty_gfns as *mut c_void);
    (*ring).dirty_gfns = core::ptr::null_mut();
}
