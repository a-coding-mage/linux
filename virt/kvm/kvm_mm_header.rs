/* SPDX-License-Identifier: GPL-2.0-only */

/* Dependencies from the original header:
 * #include <linux/kvm.h>
 * #include <linux/kvm_types.h>
 */

/*
 * Architectures can choose whether to use an rwlock or spinlock
 * for the mmu_lock.  These macros, for use in common code
 * only, avoids using #ifdefs in places that must deal with
 * multiple architectures.
 */

/* Original condition: #ifdef KVM_HAVE_MMU_RWLOCK */
#[cfg(KVM_HAVE_MMU_RWLOCK)]
#[inline]
pub unsafe fn KVM_MMU_LOCK_INIT(kvm: *mut kvm) {
    unsafe { rwlock_init(core::ptr::addr_of_mut!((*kvm).mmu_lock)) };
}

#[cfg(KVM_HAVE_MMU_RWLOCK)]
#[inline]
pub unsafe fn KVM_MMU_LOCK(kvm: *mut kvm) {
    unsafe { write_lock(core::ptr::addr_of_mut!((*kvm).mmu_lock)) };
}

#[cfg(KVM_HAVE_MMU_RWLOCK)]
#[inline]
pub unsafe fn KVM_MMU_UNLOCK(kvm: *mut kvm) {
    unsafe { write_unlock(core::ptr::addr_of_mut!((*kvm).mmu_lock)) };
}

/* Original condition: #else of KVM_HAVE_MMU_RWLOCK */
#[cfg(not(KVM_HAVE_MMU_RWLOCK))]
#[inline]
pub unsafe fn KVM_MMU_LOCK_INIT(kvm: *mut kvm) {
    unsafe { spin_lock_init(core::ptr::addr_of_mut!((*kvm).mmu_lock)) };
}

#[cfg(not(KVM_HAVE_MMU_RWLOCK))]
#[inline]
pub unsafe fn KVM_MMU_LOCK(kvm: *mut kvm) {
    unsafe { spin_lock(core::ptr::addr_of_mut!((*kvm).mmu_lock)) };
}

#[cfg(not(KVM_HAVE_MMU_RWLOCK))]
#[inline]
pub unsafe fn KVM_MMU_UNLOCK(kvm: *mut kvm) {
    unsafe { spin_unlock(core::ptr::addr_of_mut!((*kvm).mmu_lock)) };
}

#[repr(C)]
pub struct kvm_follow_pfn {
    pub slot: *const kvm_memory_slot,
    pub gfn: gfn_t,

    pub hva: core::ffi::c_ulong,

    /* FOLL_* flags modifying lookup behavior, e.g. FOLL_WRITE. */
    pub flags: core::ffi::c_uint,

    /*
     * Pin the page (effectively FOLL_PIN, which is an mm/ internal flag).
     * The page *must* be pinned if KVM will write to the page via a kernel
     * mapping, e.g. via kmap(), mremap(), etc.
     */
    pub pin: bool,

    /*
     * If non-NULL, try to get a writable mapping even for a read fault.
     * Set to true if a writable mapping was obtained.
     */
    pub map_writable: *mut bool,

    /*
     * Optional output.  Set to a valid "struct page" if the returned pfn
     * is for a refcounted or pinned struct page, NULL if the returned pfn
     * has no struct page or if the struct page is not being refcounted
     * (e.g. tail pages of non-compound higher order allocations from
     * IO/PFNMAP mappings).
     */
    pub refcounted_page: *mut *mut page,
}

unsafe extern "C" {
    pub fn hva_to_pfn(kfp: *mut kvm_follow_pfn) -> kvm_pfn_t;
}

/* Original condition: #ifdef CONFIG_HAVE_KVM_PFNCACHE */
#[cfg(CONFIG_HAVE_KVM_PFNCACHE)]
unsafe extern "C" {
    pub fn gfn_to_pfn_cache_invalidate_start(
        kvm: *mut kvm,
        start: core::ffi::c_ulong,
        end: core::ffi::c_ulong,
    );
}

/* Original condition: #else of CONFIG_HAVE_KVM_PFNCACHE */
#[cfg(not(CONFIG_HAVE_KVM_PFNCACHE))]
#[inline]
pub unsafe fn gfn_to_pfn_cache_invalidate_start(
    kvm: *mut kvm,
    start: core::ffi::c_ulong,
    end: core::ffi::c_ulong,
) {
    let _ = (kvm, start, end);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
