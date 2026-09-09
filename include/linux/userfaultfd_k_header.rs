/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/userfaultfd_k.h. */

/* The following items depend on the kernel types and helpers supplied by the
 * including translation unit.  CONFIG_USERFAULTFD selects the corresponding
 * branch at build time. */

#[cfg(CONFIG_USERFAULTFD)]
pub const __VM_UFFD_FLAGS: u64 = VM_UFFD_MISSING | VM_UFFD_MINOR | VM_UFFD_WP | VM_UFFD_RWP;
#[cfg(CONFIG_USERFAULTFD)]
pub const __VMA_UFFD_FLAGS: _ = mk_vma_flags_from_masks(VMA_UFFD_MISSING, VMA_UFFD_WP, VMA_UFFD_MINOR, VMA_UFFD_RWP);
#[cfg(CONFIG_USERFAULTFD)]
pub const UFFD_SHARED_FCNTL_FLAGS: _ = O_CLOEXEC | O_NONBLOCK;

#[cfg(CONFIG_USERFAULTFD)]
#[repr(C)]
pub struct userfaultfd_ctx {
    pub fault_pending_wqh: wait_queue_head_t,
    pub fault_wqh: wait_queue_head_t,
    pub fd_wqh: wait_queue_head_t,
    pub event_wqh: wait_queue_head_t,
    pub refile_seq: seqcount_spinlock_t,
    pub refcount: refcount_t,
    pub flags: core::ffi::c_uint,
    pub features: core::ffi::c_uint,
    pub released: bool,
    pub map_changing_lock: rw_semaphore,
    pub mmap_changing: atomic_t,
    pub mm: *mut mm_struct,
}

#[cfg(CONFIG_USERFAULTFD)]
extern "C" {
    pub fn handle_userfault(vmf: *mut vm_fault, reason: core::ffi::c_ulong) -> vm_fault_t;
}

#[cfg(CONFIG_USERFAULTFD)]
#[repr(C)]
pub struct vm_uffd_ops {
    pub can_userfault: Option<unsafe extern "C" fn(*mut vm_area_struct, vm_flags_t) -> bool>,
    pub get_folio_noalloc: Option<unsafe extern "C" fn(*mut inode, pgoff_t) -> *mut folio>,
    pub alloc_folio: Option<unsafe extern "C" fn(*mut vm_area_struct, core::ffi::c_ulong) -> *mut folio>,
    pub filemap_add: Option<unsafe extern "C" fn(*mut folio, *mut vm_area_struct, core::ffi::c_ulong) -> core::ffi::c_int>,
    pub filemap_remove: Option<unsafe extern "C" fn(*mut folio, *mut vm_area_struct)>,
}

#[cfg(CONFIG_USERFAULTFD)]
pub type uffd_flags_t = core::ffi::c_uint;

#[cfg(CONFIG_USERFAULTFD)]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum mfill_atomic_mode { MFILL_ATOMIC_COPY, MFILL_ATOMIC_ZEROPAGE, MFILL_ATOMIC_CONTINUE, MFILL_ATOMIC_POISON, NR_MFILL_ATOMIC_MODES }

#[cfg(CONFIG_USERFAULTFD)]
pub const MFILL_ATOMIC_MODE_BITS: u32 = const_ilog2(NR_MFILL_ATOMIC_MODES as u32 - 1) + 1;
#[cfg(CONFIG_USERFAULTFD)]
#[inline]
pub const fn MFILL_ATOMIC_BIT(nr: u32) -> uffd_flags_t { BIT(MFILL_ATOMIC_MODE_BITS + nr) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline]
pub const fn MFILL_ATOMIC_FLAG(nr: u32) -> uffd_flags_t { MFILL_ATOMIC_BIT(nr) }
#[cfg(CONFIG_USERFAULTFD)]
pub const MFILL_ATOMIC_MODE_MASK: uffd_flags_t = MFILL_ATOMIC_BIT(0) - 1;
#[cfg(CONFIG_USERFAULTFD)]
pub const MFILL_ATOMIC_WP: uffd_flags_t = MFILL_ATOMIC_FLAG(0);

#[cfg(CONFIG_USERFAULTFD)]
#[inline]
pub fn uffd_flags_mode_is(flags: uffd_flags_t, expected: mfill_atomic_mode) -> bool { (flags & MFILL_ATOMIC_MODE_MASK) == expected as uffd_flags_t }
#[cfg(CONFIG_USERFAULTFD)]
#[inline]
pub fn uffd_flags_set_mode(mut flags: uffd_flags_t, mode: mfill_atomic_mode) -> uffd_flags_t { flags &= !MFILL_ATOMIC_MODE_MASK; flags | mode as uffd_flags_t }

extern "C" {
    #[cfg(CONFIG_USERFAULTFD)] pub fn uffd_wp_range(vma: *mut vm_area_struct, start: core::ffi::c_ulong, len: core::ffi::c_ulong, enable_wp: bool) -> core::ffi::c_long;
    #[cfg(CONFIG_USERFAULTFD)] pub fn mrwprotect_range(ctx: *mut userfaultfd_ctx, start: core::ffi::c_ulong, len: core::ffi::c_ulong, enable_rwp: bool) -> core::ffi::c_int;
    #[cfg(CONFIG_USERFAULTFD)] pub fn double_pt_lock(ptl1: *mut spinlock_t, ptl2: *mut spinlock_t);
    #[cfg(CONFIG_USERFAULTFD)] pub fn double_pt_unlock(ptl1: *mut spinlock_t, ptl2: *mut spinlock_t);
    #[cfg(CONFIG_USERFAULTFD)] pub fn move_pages_huge_pmd(mm: *mut mm_struct, dst_pmd: *mut pmd_t, src_pmd: *mut pmd_t, dst_pmdval: pmd_t, dst_vma: *mut vm_area_struct, src_vma: *mut vm_area_struct, dst_addr: core::ffi::c_ulong, src_addr: core::ffi::c_ulong) -> core::ffi::c_int;
}

#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn is_mergeable_vm_userfaultfd_ctx(vma: *mut vm_area_struct, vm_ctx: vm_userfaultfd_ctx) -> bool { (*vma).vm_userfaultfd_ctx.ctx == vm_ctx.ctx }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn uffd_disable_huge_pmd_share(vma: *mut vm_area_struct) -> bool { vma_test_any_mask(vma, mk_vma_flags_from_masks(VMA_UFFD_WP, VMA_UFFD_RWP, VMA_UFFD_MINOR)) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn uffd_disable_fault_around(vma: *mut vm_area_struct) -> bool { vma_test_any_mask(vma, mk_vma_flags_from_masks(VMA_UFFD_WP, VMA_UFFD_RWP, VMA_UFFD_MINOR)) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_missing(vma: *mut vm_area_struct) -> bool { vma_test_any_mask(vma, VMA_UFFD_MISSING) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_wp(vma: *mut vm_area_struct) -> bool { vma_test_any_mask(vma, VMA_UFFD_WP) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_minor(vma: *mut vm_area_struct) -> bool { vma_test_any_mask(vma, VMA_UFFD_MINOR) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_rwp(vma: *mut vm_area_struct) -> bool { if !IS_ENABLED(CONFIG_ARCH_HAS_PTE_PROTNONE) { false } else { vma_test_single_mask(vma, VMA_UFFD_RWP) } }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_protected(vma: *mut vm_area_struct) -> bool { userfaultfd_wp(vma) || userfaultfd_rwp(vma) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_pte_wp(vma: *mut vm_area_struct, pte: pte_t) -> bool { userfaultfd_wp(vma) && pte_uffd(pte) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_huge_pmd_wp(vma: *mut vm_area_struct, pmd: pmd_t) -> bool { userfaultfd_wp(vma) && pmd_uffd(pmd) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_pte_rwp(vma: *mut vm_area_struct, pte: pte_t) -> bool { userfaultfd_rwp(vma) && pte_uffd(pte) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_huge_pmd_rwp(vma: *mut vm_area_struct, pmd: pmd_t) -> bool { userfaultfd_rwp(vma) && pmd_uffd(pmd) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_armed(vma: *mut vm_area_struct) -> bool { vma_test_any_mask(vma, __VMA_UFFD_FLAGS) }

/* Remaining external operations and CONFIG_USERFAULTFD-disabled stubs retain
 * the header's declarations and behavior. */
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn handle_userfault(_vmf: *mut vm_fault, _reason: core::ffi::c_ulong) -> vm_fault_t { VM_FAULT_SIGBUS }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn uffd_wp_range(_vma: *mut vm_area_struct, _start: core::ffi::c_ulong, _len: core::ffi::c_ulong, _enable_wp: bool) -> core::ffi::c_long { 0 }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_remove(_vma: *mut vm_area_struct, _start: core::ffi::c_ulong, _end: core::ffi::c_ulong) -> bool { true }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_missing(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_wp(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_minor(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_rwp(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_protected(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_armed(_vma: *mut vm_area_struct) -> bool { false }

#[cfg(CONFIG_USERFAULTFD)]
extern "C" {
    pub fn vma_has_uffd_without_event_remap(vma: *mut vm_area_struct) -> bool;
    pub fn dup_userfaultfd(vma: *mut vm_area_struct, l: *mut list_head) -> core::ffi::c_int;
    pub fn dup_userfaultfd_complete(l: *mut list_head);
    pub fn dup_userfaultfd_fail(l: *mut list_head);
    pub fn mremap_userfaultfd_prep(vma: *mut vm_area_struct, ctx: *mut vm_userfaultfd_ctx);
    pub fn mremap_userfaultfd_complete(ctx: *mut vm_userfaultfd_ctx, from: core::ffi::c_ulong, to: core::ffi::c_ulong, len: core::ffi::c_ulong);
    pub fn mremap_userfaultfd_fail(ctx: *mut vm_userfaultfd_ctx);
    pub fn userfaultfd_remove(vma: *mut vm_area_struct, start: core::ffi::c_ulong, end: core::ffi::c_ulong) -> bool;
    pub fn userfaultfd_unmap_prep(vma: *mut vm_area_struct, start: core::ffi::c_ulong, end: core::ffi::c_ulong, uf: *mut list_head) -> core::ffi::c_int;
    pub fn userfaultfd_unmap_complete(mm: *mut mm_struct, uf: *mut list_head);
    pub fn userfaultfd_wp_unpopulated(vma: *mut vm_area_struct) -> bool;
    pub fn userfaultfd_wp_async(vma: *mut vm_area_struct) -> bool;
    pub fn userfaultfd_rwp_async(vma: *mut vm_area_struct) -> bool;
}

#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn userfaultfd_wp_use_markers(vma: *mut vm_area_struct) -> bool { if !userfaultfd_wp(vma) { return false; } if !vma_is_anonymous(vma) { return true; } userfaultfd_wp_unpopulated(vma) }
#[cfg(CONFIG_USERFAULTFD)]
#[inline] pub unsafe fn pte_swp_uffd_any(pte: pte_t) -> bool { if !uffd_supports_wp_marker() || pte_present(pte) { return false; } pte_swp_uffd(pte) || pte_is_uffd_wp_marker(pte) }

#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_huge_pmd_wp(_vma: *mut vm_area_struct, _pmd: pmd_t) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_pte_wp(_vma: *mut vm_area_struct, _pte: pte_t) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_huge_pmd_rwp(_vma: *mut vm_area_struct, _pmd: pmd_t) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_pte_rwp(_vma: *mut vm_area_struct, _pte: pte_t) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn is_mergeable_vm_userfaultfd_ctx(_vma: *mut vm_area_struct, _ctx: vm_userfaultfd_ctx) -> bool { true }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn uffd_disable_fault_around(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_wp_unpopulated(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_wp_async(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_rwp_async(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn vma_has_uffd_without_event_remap(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn userfaultfd_wp_use_markers(_vma: *mut vm_area_struct) -> bool { false }
#[cfg(not(CONFIG_USERFAULTFD))]
#[inline] pub fn pte_swp_uffd_any(_pte: pte_t) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
