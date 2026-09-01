// SPDX-License-Identifier: GPL-2.0-only
/*
 * Kernel-based Virtual Machine driver for Linux
 *
 * This module enables kernel and guest-mode vCPU access to guest physical
 * memory with suitable invalidation mechanisms.
 *
 * Copyright (c) 2021 Amazon.com, Inc. or its affiliates.
 *
 * Authors:
 *   David Woodhouse <dwmw2@infradead.org>
 */

/*
 * Translated from C. External Linux/KVM types, constants, macros, and helpers
 * are expected to be supplied by the surrounding crate/bindings.
 */

use core::ffi::c_void;

extern "C" {
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn read_lock_irq(lock: *mut rwlock_t);
    fn read_unlock_irq(lock: *mut rwlock_t);
    fn write_lock_irq(lock: *mut rwlock_t);
    fn write_unlock_irq(lock: *mut rwlock_t);
    fn rwlock_init(lock: *mut rwlock_t);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn kvm_memslots(kvm: *mut kvm) -> *mut kvm_memslots;
    fn kvm_is_error_gpa(gpa: gpa_t) -> bool;
    fn kvm_is_error_hva(hva: libc_ulong) -> bool;
    fn is_error_noslot_pfn(pfn: kvm_pfn_t) -> bool;
    fn offset_in_page(addr: libc_ulong) -> libc_ulong;
    fn pfn_valid(pfn: kvm_pfn_t) -> bool;
    fn pfn_to_page(pfn: kvm_pfn_t) -> *mut page;
    fn kmap(page: *mut page) -> *mut c_void;
    fn kunmap(page: *mut page);
    fn pfn_to_hpa(pfn: kvm_pfn_t) -> phys_addr_t;
    fn memremap(offset: phys_addr_t, size: libc_ulong, flags: libc_ulong) -> *mut c_void;
    fn memunmap(addr: *mut c_void);
    fn smp_rmb();
    fn hva_to_pfn(kfp: *mut kvm_follow_pfn) -> kvm_pfn_t;
    fn kvm_release_page_unused(page: *mut page);
    fn kvm_release_page_clean(page: *mut page);
    fn cond_resched();
    fn gpa_to_gfn(gpa: gpa_t) -> gfn_t;
    fn __gfn_to_memslot(slots: *mut kvm_memslots, gfn: gfn_t) -> *mut kvm_memory_slot;
    fn gfn_to_hva_memslot(slot: *mut kvm_memory_slot, gfn: gfn_t) -> libc_ulong;
    fn access_ok(addr: *mut c_void, size: libc_ulong) -> bool;
    fn WARN_ON_ONCE(condition: bool) -> bool;
    fn KVM_BUG_ON(condition: bool, kvm: *mut kvm) -> bool;
}

type libc_ulong = u64;
type libc_int = i32;

type gpa_t = u64;
type gfn_t = u64;
type kvm_pfn_t = u64;
type phys_addr_t = u64;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rwlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_memory_slot {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_memslots {
    pub generation: libc_ulong,
}

#[repr(C)]
pub struct kvm {
    pub gpc_lock: spinlock_t,
    pub gpc_list: list_head,
    pub mn_active_invalidate_count: libc_ulong,
    pub mmu_invalidate_seq: libc_ulong,
}

#[repr(C)]
pub struct gfn_to_pfn_cache {
    pub lock: rwlock_t,
    pub refresh_lock: mutex,
    pub kvm: *mut kvm,
    pub list: list_head,
    pub memslot: *mut kvm_memory_slot,
    pub generation: libc_ulong,
    pub gpa: gpa_t,
    pub uhva: libc_ulong,
    pub pfn: kvm_pfn_t,
    pub khva: *mut c_void,
    pub active: bool,
    pub valid: bool,
}

#[repr(C)]
pub struct kvm_follow_pfn {
    pub slot: *mut kvm_memory_slot,
    pub gfn: gfn_t,
    pub flags: libc_ulong,
    pub hva: libc_ulong,
    pub refcounted_page: *mut *mut page,
}

const PAGE_SIZE: libc_ulong = 4096;
const FOLL_WRITE: libc_ulong = 0x01;
const MEMREMAP_WB: libc_ulong = 0x01;
const KVM_PFN_ERR_FAULT: kvm_pfn_t = !0;
const INVALID_GPA: gpa_t = !0;
const KVM_HVA_ERR_BAD: libc_ulong = !0;
const EFAULT: libc_int = 14;
const EINVAL: libc_int = 22;
const EIO: libc_int = 5;

#[inline]
unsafe fn PAGE_ALIGN_DOWN(addr: libc_ulong) -> libc_ulong {
    addr & !(PAGE_SIZE - 1)
}

#[inline]
unsafe fn container_of_gfn_to_pfn_cache_list(ptr: *mut list_head) -> *mut gfn_to_pfn_cache {
    (ptr as *mut u8).sub(core::mem::offset_of!(gfn_to_pfn_cache, list)) as *mut gfn_to_pfn_cache
}

/*
 * MMU notifier 'invalidate_range_start' hook.
 */
#[no_mangle]
pub unsafe extern "C" fn gfn_to_pfn_cache_invalidate_start(
    kvm: *mut kvm,
    start: libc_ulong,
    end: libc_ulong,
) {
    let mut pos: *mut list_head;
    let mut gpc: *mut gfn_to_pfn_cache;

    spin_lock(&mut (*kvm).gpc_lock);
    pos = (*kvm).gpc_list.next;
    while pos != &mut (*kvm).gpc_list {
        gpc = container_of_gfn_to_pfn_cache_list(pos);
        read_lock_irq(&mut (*gpc).lock);

        /* Only a single page so no need to care about length */
        if (*gpc).valid
            && !is_error_noslot_pfn((*gpc).pfn)
            && (*gpc).uhva >= start
            && (*gpc).uhva < end
        {
            read_unlock_irq(&mut (*gpc).lock);

            /*
             * There is a small window here where the cache could
             * be modified, and invalidation would no longer be
             * necessary. Hence check again whether invalidation
             * is still necessary once the write lock has been
             * acquired.
             */

            write_lock_irq(&mut (*gpc).lock);
            if (*gpc).valid
                && !is_error_noslot_pfn((*gpc).pfn)
                && (*gpc).uhva >= start
                && (*gpc).uhva < end
            {
                (*gpc).valid = false;
            }
            write_unlock_irq(&mut (*gpc).lock);
            pos = (*pos).next;
            continue;
        }

        read_unlock_irq(&mut (*gpc).lock);
        pos = (*pos).next;
    }
    spin_unlock(&mut (*kvm).gpc_lock);
}

unsafe fn kvm_gpc_is_valid_len(gpa: gpa_t, uhva: libc_ulong, len: libc_ulong) -> bool {
    let offset: libc_ulong = if kvm_is_error_gpa(gpa) {
        offset_in_page(uhva)
    } else {
        offset_in_page(gpa)
    };

    /*
     * The cached access must fit within a single page. The 'len' argument
     * to activate() and refresh() exists only to enforce that.
     */
    offset + len <= PAGE_SIZE
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gpc_check(gpc: *mut gfn_to_pfn_cache, len: libc_ulong) -> bool {
    let slots: *mut kvm_memslots = kvm_memslots((*gpc).kvm);

    if !(*gpc).active {
        return false;
    }

    /*
     * If the page was cached from a memslot, make sure the memslots have
     * not been re-configured.
     */
    if !kvm_is_error_gpa((*gpc).gpa) && (*gpc).generation != (*slots).generation {
        return false;
    }

    if kvm_is_error_hva((*gpc).uhva) {
        return false;
    }

    if !kvm_gpc_is_valid_len((*gpc).gpa, (*gpc).uhva, len) {
        return false;
    }

    if !(*gpc).valid {
        return false;
    }

    true
}

unsafe fn gpc_map(pfn: kvm_pfn_t) -> *mut c_void {
    if pfn_valid(pfn) {
        return kmap(pfn_to_page(pfn));
    }

    /* CONFIG_HAS_IOMEM: map I/O memory when supported by the build. */
    memremap(pfn_to_hpa(pfn), PAGE_SIZE, MEMREMAP_WB)
}

unsafe fn gpc_unmap(pfn: kvm_pfn_t, khva: *mut c_void) {
    /* Unmap the old pfn/page if it was mapped before. */
    if is_error_noslot_pfn(pfn) || khva.is_null() {
        return;
    }

    if pfn_valid(pfn) {
        kunmap(pfn_to_page(pfn));
        return;
    }

    /* CONFIG_HAS_IOMEM: unmap I/O memory when supported by the build. */
    memunmap(khva);
}

#[inline]
unsafe fn mmu_notifier_retry_cache(kvm: *mut kvm, mmu_seq: libc_ulong) -> bool {
    /*
     * mn_active_invalidate_count acts for all intents and purposes
     * like mmu_invalidate_in_progress here; but the latter cannot
     * be used here because the invalidation of caches in the
     * mmu_notifier event occurs _before_ mmu_invalidate_in_progress
     * is elevated.
     *
     * Note, it does not matter that mn_active_invalidate_count
     * is not protected by gpc->lock.  It is guaranteed to
     * be elevated before the mmu_notifier acquires gpc->lock, and
     * isn't dropped until after mmu_invalidate_seq is updated.
     */
    if (*kvm).mn_active_invalidate_count != 0 {
        return true;
    }

    /*
     * Ensure mn_active_invalidate_count is read before
     * mmu_invalidate_seq.  This pairs with the smp_wmb() in
     * mmu_notifier_invalidate_range_end() to guarantee either the
     * old (non-zero) value of mn_active_invalidate_count or the
     * new (incremented) value of mmu_invalidate_seq is observed.
     */
    smp_rmb();
    (*kvm).mmu_invalidate_seq != mmu_seq
}

unsafe fn hva_to_pfn_retry(gpc: *mut gfn_to_pfn_cache) -> kvm_pfn_t {
    /* Note, the new page offset may be different than the old! */
    let old_khva: *mut c_void = PAGE_ALIGN_DOWN((*gpc).khva as libc_ulong) as *mut c_void;
    let mut new_pfn: kvm_pfn_t = KVM_PFN_ERR_FAULT;
    let mut new_khva: *mut c_void = core::ptr::null_mut();
    let mut mmu_seq: libc_ulong;
    let mut page: *mut page = core::ptr::null_mut();

    let mut kfp = kvm_follow_pfn {
        slot: (*gpc).memslot,
        gfn: gpa_to_gfn((*gpc).gpa),
        flags: FOLL_WRITE,
        hva: (*gpc).uhva,
        refcounted_page: &mut page,
    };

    /* lockdep_assert_held(&gpc->refresh_lock); */
    /* lockdep_assert_held_write(&gpc->lock); */

    /*
     * Invalidate the cache prior to dropping gpc->lock, the gpa=>uhva
     * assets have already been updated and so a concurrent check() from a
     * different task may not fail the gpa/uhva/generation checks.
     */
    (*gpc).valid = false;

    loop {
        mmu_seq = (*(*gpc).kvm).mmu_invalidate_seq;
        smp_rmb();

        write_unlock_irq(&mut (*gpc).lock);

        /*
         * If the previous iteration "failed" due to an mmu_notifier
         * event, release the pfn and unmap the kernel virtual address
         * from the previous attempt.  Unmapping might sleep, so this
         * needs to be done after dropping the lock.  Opportunistically
         * check for resched while the lock isn't held.
         */
        if new_pfn != KVM_PFN_ERR_FAULT {
            /*
             * Keep the mapping if the previous iteration reused
             * the existing mapping and didn't create a new one.
             */
            if new_khva != old_khva {
                gpc_unmap(new_pfn, new_khva);
            }

            kvm_release_page_unused(page);

            cond_resched();
        }

        new_pfn = hva_to_pfn(&mut kfp);
        if is_error_noslot_pfn(new_pfn) {
            break;
        }

        /*
         * Obtain a new kernel mapping if KVM itself will access the
         * pfn.  Note, kmap() and memremap() can both sleep, so this
         * too must be done outside of gpc->lock!
         */
        if new_pfn == (*gpc).pfn {
            new_khva = old_khva;
        } else {
            new_khva = gpc_map(new_pfn);
        }

        if new_khva.is_null() {
            kvm_release_page_unused(page);
            break;
        }

        write_lock_irq(&mut (*gpc).lock);

        /*
         * Other tasks must wait for _this_ refresh to complete before
         * attempting to refresh.
         */
        WARN_ON_ONCE((*gpc).valid);

        if !mmu_notifier_retry_cache((*gpc).kvm, mmu_seq) {
            (*gpc).valid = true;
            (*gpc).pfn = new_pfn;
            (*gpc).khva = (new_khva as *mut u8).add(offset_in_page((*gpc).uhva) as usize) as *mut c_void;

            /*
             * Put the reference to the _new_ page.  The page is now tracked by the
             * cache and can be safely migrated, swapped, etc... as the cache will
             * invalidate any mappings in response to relevant mmu_notifier events.
             */
            kvm_release_page_clean(page);

            return 0;
        }
    }

    write_lock_irq(&mut (*gpc).lock);

    -(EFAULT as kvm_pfn_t)
}

unsafe fn __kvm_gpc_refresh(
    gpc: *mut gfn_to_pfn_cache,
    gpa: gpa_t,
    uhva: libc_ulong,
) -> libc_int {
    let mut page_offset: libc_ulong;
    let mut unmap_old: bool = false;
    let old_uhva: libc_ulong;
    let old_pfn: kvm_pfn_t;
    let mut hva_change: bool = false;
    let old_khva: *mut c_void;
    let mut ret: libc_int;

    /* Either gpa or uhva must be valid, but not both */
    if WARN_ON_ONCE(kvm_is_error_gpa(gpa) == kvm_is_error_hva(uhva)) {
        return -EINVAL;
    }

    /* lockdep_assert_held(&gpc->refresh_lock); */

    write_lock_irq(&mut (*gpc).lock);

    if !(*gpc).active {
        ret = -EINVAL;
        goto_out_unlock(gpc, ret)
    } else {
        old_pfn = (*gpc).pfn;
        old_khva = PAGE_ALIGN_DOWN((*gpc).khva as libc_ulong) as *mut c_void;
        old_uhva = PAGE_ALIGN_DOWN((*gpc).uhva);

        if kvm_is_error_gpa(gpa) {
            page_offset = offset_in_page(uhva);

            (*gpc).gpa = INVALID_GPA;
            (*gpc).memslot = core::ptr::null_mut();
            (*gpc).uhva = PAGE_ALIGN_DOWN(uhva);

            if (*gpc).uhva != old_uhva {
                hva_change = true;
            }
        } else {
            let slots: *mut kvm_memslots = kvm_memslots((*gpc).kvm);

            page_offset = offset_in_page(gpa);

            if (*gpc).gpa != gpa
                || (*gpc).generation != (*slots).generation
                || kvm_is_error_hva((*gpc).uhva)
            {
                let gfn: gfn_t = gpa_to_gfn(gpa);

                (*gpc).gpa = gpa;
                (*gpc).generation = (*slots).generation;
                (*gpc).memslot = __gfn_to_memslot(slots, gfn);
                (*gpc).uhva = gfn_to_hva_memslot((*gpc).memslot, gfn);

                if kvm_is_error_hva((*gpc).uhva) {
                    ret = -EFAULT;
                    if ret != 0 {
                        (*gpc).valid = false;
                        (*gpc).pfn = KVM_PFN_ERR_FAULT;
                        (*gpc).khva = core::ptr::null_mut();
                    }
                    unmap_old = old_pfn != (*gpc).pfn;
                    write_unlock_irq(&mut (*gpc).lock);
                    if unmap_old {
                        gpc_unmap(old_pfn, old_khva);
                    }
                    return ret;
                }

                /*
                 * Even if the GPA and/or the memslot generation changed, the
                 * HVA may still be the same.
                 */
                if (*gpc).uhva != old_uhva {
                    hva_change = true;
                }
            } else {
                (*gpc).uhva = old_uhva;
            }
        }

        /* Note: the offset must be correct before calling hva_to_pfn_retry() */
        (*gpc).uhva = (*gpc).uhva.wrapping_add(page_offset);

        /*
         * If the userspace HVA changed or the PFN was already invalid,
         * drop the lock and do the HVA to PFN lookup again.
         */
        if !(*gpc).valid || hva_change {
            ret = hva_to_pfn_retry(gpc) as libc_int;
        } else {
            /*
             * If the HVA->PFN mapping was already valid, don't unmap it.
             * But do update gpc->khva because the offset within the page
             * may have changed.
             */
            (*gpc).khva = (old_khva as *mut u8).add(page_offset as usize) as *mut c_void;
            ret = 0;
            write_unlock_irq(&mut (*gpc).lock);
            return ret;
        }

        /*
         * Invalidate the cache and purge the pfn/khva if the refresh failed.
         * Some/all of the uhva, gpa, and memslot generation info may still be
         * valid, leave it as is.
         */
        if ret != 0 {
            (*gpc).valid = false;
            (*gpc).pfn = KVM_PFN_ERR_FAULT;
            (*gpc).khva = core::ptr::null_mut();
        }

        /* Detect a pfn change before dropping the lock! */
        unmap_old = old_pfn != (*gpc).pfn;

        write_unlock_irq(&mut (*gpc).lock);

        if unmap_old {
            gpc_unmap(old_pfn, old_khva);
        }

        ret
    }
}

unsafe fn goto_out_unlock(gpc: *mut gfn_to_pfn_cache, ret: libc_int) -> libc_int {
    write_unlock_irq(&mut (*gpc).lock);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gpc_refresh(
    gpc: *mut gfn_to_pfn_cache,
    len: libc_ulong,
) -> libc_int {
    let uhva: libc_ulong;

    mutex_lock(&mut (*gpc).refresh_lock);

    if !kvm_gpc_is_valid_len((*gpc).gpa, (*gpc).uhva, len) {
        mutex_unlock(&mut (*gpc).refresh_lock);
        return -EINVAL;
    }

    /*
     * If the GPA is valid then ignore the HVA, as a cache can be GPA-based
     * or HVA-based, not both.  For GPA-based caches, the HVA will be
     * recomputed during refresh if necessary.
     */
    uhva = if kvm_is_error_gpa((*gpc).gpa) {
        (*gpc).uhva
    } else {
        KVM_HVA_ERR_BAD
    };

    let ret = __kvm_gpc_refresh(gpc, (*gpc).gpa, uhva);
    mutex_unlock(&mut (*gpc).refresh_lock);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gpc_init(gpc: *mut gfn_to_pfn_cache, kvm: *mut kvm) {
    rwlock_init(&mut (*gpc).lock);
    mutex_init(&mut (*gpc).refresh_lock);

    (*gpc).kvm = kvm;
    (*gpc).pfn = KVM_PFN_ERR_FAULT;
    (*gpc).gpa = INVALID_GPA;
    (*gpc).uhva = KVM_HVA_ERR_BAD;
    (*gpc).active = false;
    (*gpc).valid = false;
}

unsafe fn __kvm_gpc_activate(
    gpc: *mut gfn_to_pfn_cache,
    gpa: gpa_t,
    uhva: libc_ulong,
    len: libc_ulong,
) -> libc_int {
    let kvm: *mut kvm = (*gpc).kvm;

    if !kvm_gpc_is_valid_len(gpa, uhva, len) {
        return -EINVAL;
    }

    mutex_lock(&mut (*gpc).refresh_lock);

    let ret = if !(*gpc).active {
        if KVM_BUG_ON((*gpc).valid, kvm) {
            mutex_unlock(&mut (*gpc).refresh_lock);
            return -EIO;
        }

        spin_lock(&mut (*kvm).gpc_lock);
        list_add(&mut (*gpc).list, &mut (*kvm).gpc_list);
        spin_unlock(&mut (*kvm).gpc_lock);

        /*
         * Activate the cache after adding it to the list, a concurrent
         * refresh must not establish a mapping until the cache is
         * reachable by mmu_notifier events.
         */
        write_lock_irq(&mut (*gpc).lock);
        (*gpc).active = true;
        write_unlock_irq(&mut (*gpc).lock);

        __kvm_gpc_refresh(gpc, gpa, uhva)
    } else {
        __kvm_gpc_refresh(gpc, gpa, uhva)
    };

    mutex_unlock(&mut (*gpc).refresh_lock);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gpc_activate(
    gpc: *mut gfn_to_pfn_cache,
    gpa: gpa_t,
    len: libc_ulong,
) -> libc_int {
    /*
     * Explicitly disallow INVALID_GPA so that the magic value can be used
     * by KVM to differentiate between GPA-based and HVA-based caches.
     */
    if WARN_ON_ONCE(kvm_is_error_gpa(gpa)) {
        return -EINVAL;
    }

    __kvm_gpc_activate(gpc, gpa, KVM_HVA_ERR_BAD, len)
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gpc_activate_hva(
    gpc: *mut gfn_to_pfn_cache,
    uhva: libc_ulong,
    len: libc_ulong,
) -> libc_int {
    if !access_ok(uhva as *mut c_void, len) {
        return -EINVAL;
    }

    __kvm_gpc_activate(gpc, INVALID_GPA, uhva, len)
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gpc_deactivate(gpc: *mut gfn_to_pfn_cache) {
    let kvm: *mut kvm = (*gpc).kvm;
    let old_pfn: kvm_pfn_t;
    let old_khva: *mut c_void;

    mutex_lock(&mut (*gpc).refresh_lock);

    if (*gpc).active {
        /*
         * Deactivate the cache before removing it from the list, KVM
         * must stall mmu_notifier events until all users go away, i.e.
         * until gpc->lock is dropped and refresh is guaranteed to fail.
         */
        write_lock_irq(&mut (*gpc).lock);
        (*gpc).active = false;
        (*gpc).valid = false;

        /*
         * Leave the GPA => uHVA cache intact, it's protected by the
         * memslot generation.  The PFN lookup needs to be redone every
         * time as mmu_notifier protection is lost when the cache is
         * removed from the VM's gpc_list.
         */
        old_khva = ((*gpc).khva as *mut u8).sub(offset_in_page((*gpc).khva as libc_ulong) as usize)
            as *mut c_void;
        (*gpc).khva = core::ptr::null_mut();

        old_pfn = (*gpc).pfn;
        (*gpc).pfn = KVM_PFN_ERR_FAULT;
        write_unlock_irq(&mut (*gpc).lock);

        spin_lock(&mut (*kvm).gpc_lock);
        list_del(&mut (*gpc).list);
        spin_unlock(&mut (*kvm).gpc_lock);

        gpc_unmap(old_pfn, old_khva);
    }

    mutex_unlock(&mut (*gpc).refresh_lock);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
