/* SPDX-License-Identifier: GPL-2.0 */
/* KVM guest address space mapping code */

// Dependencies supplied by the surrounding kernel translation.

pub const GMAP_FLAG_SHADOW: u32 = 0;
pub const GMAP_FLAG_OWNS_PAGETABLES: u32 = 1;
pub const GMAP_FLAG_IS_UCONTROL: u32 = 2;
pub const GMAP_FLAG_ALLOW_HPAGE_1M: u32 = 3;
pub const GMAP_FLAG_ALLOW_HPAGE_2G: u32 = 4;
pub const GMAP_FLAG_PFAULT_ENABLED: u32 = 5;
pub const GMAP_FLAG_USES_SKEYS: u32 = 6;
pub const GMAP_FLAG_USES_CMM: u32 = 7;
pub const GMAP_FLAG_EXPORT_ON_UNMAP: u32 = 8;

#[repr(C)]
pub struct Gmap {
    pub flags: ::core::ffi::c_ulong,
    pub edat_level: u8,
    pub invalidated: bool,
    pub kvm: *mut Kvm,
    pub asce: Asce,
    pub list: ListHead,
    pub children_lock: Spinlock,
    pub children: ListHead,
    pub scb_users: ListHead,
    pub parent: *mut Gmap,
    pub guest_asce: Asce,
    pub host_to_rmap_lock: Spinlock,
    pub host_to_rmap: RadixTreeRoot,
    pub refcount: Refcount,
}

#[repr(C)]
pub struct GmapCache {
    pub list: ListHead,
    pub gmap: *mut Gmap,
}

#[macro_export]
macro_rules! gmap_for_each_rmap_safe {
    ($pos:ident, $n:ident, $head:expr) => {
        for $pos in $head {
            let $n = $pos.map_or(::core::ptr::null_mut(), |p| unsafe { (*p).next });
            let _ = $n;
        }
    };
}

extern "C" {
    pub fn s390_replace_asce(gmap: *mut Gmap) -> i32;
    pub fn gmap_age_gfn(gmap: *mut Gmap, start: Gfn, end: Gfn) -> bool;
    pub fn gmap_unmap_gfn_range(gmap: *mut Gmap, slot: *mut KvmMemorySlot, start: Gfn, end: Gfn) -> bool;
    pub fn gmap_try_fixup_minor(gmap: *mut Gmap, fault: *mut GuestFault) -> i32;
    pub fn gmap_new(kvm: *mut Kvm, limit: Gfn) -> *mut Gmap;
    pub fn gmap_new_child(parent: *mut Gmap, limit: Gfn) -> *mut Gmap;
    pub fn gmap_remove_child(child: *mut Gmap);
    pub fn gmap_dispose(gmap: *mut Gmap);
    pub fn gmap_link(mc: *mut KvmS390MmuCache, gmap: *mut Gmap, fault: *mut GuestFault, slot: *mut KvmMemorySlot) -> i32;
    pub fn gmap_sync_dirty_log(gmap: *mut Gmap, start: Gfn, end: Gfn);
    pub fn gmap_set_limit(gmap: *mut Gmap, limit: Gfn) -> i32;
    pub fn gmap_ucas_translate(mc: *mut KvmS390MmuCache, gmap: *mut Gmap, gaddr: *mut Gpa) -> i32;
    pub fn gmap_ucas_map(gmap: *mut Gmap, p_gfn: Gfn, c_gfn: Gfn, count: ::core::ffi::c_ulong) -> i32;
    pub fn gmap_ucas_unmap(gmap: *mut Gmap, c_gfn: Gfn, count: ::core::ffi::c_ulong);
    pub fn gmap_pv_destroy_range(gmap: *mut Gmap, start: Gfn, end: Gfn, interruptible: bool) -> i32;
    pub fn gmap_insert_rmap(mc: *mut KvmS390MmuCache, sg: *mut Gmap, p_gfn: Gfn, r_gfn: Gfn, level: i32) -> i32;
    pub fn gmap_protect_rmap(mc: *mut KvmS390MmuCache, sg: *mut Gmap, p_gfn: Gfn, r_gfn: Gfn, pfn: KvmPfn, level: i32, wr: bool) -> i32;
    pub fn _gmap_handle_vsie_unshadow_event(parent: *mut Gmap, gfn: Gfn);
    pub fn gmap_create_shadow(mc: *mut KvmS390MmuCache, gmap: *mut Gmap, asce: Asce, edat_level: i32) -> *mut Gmap;
    pub fn gmap_split_huge_pages(gmap: *mut Gmap);
}

#[inline]
pub unsafe fn uses_skeys(gmap: *mut Gmap) -> bool { test_bit(GMAP_FLAG_USES_SKEYS, &(*gmap).flags) }
#[inline]
pub unsafe fn uses_cmm(gmap: *mut Gmap) -> bool { test_bit(GMAP_FLAG_USES_CMM, &(*gmap).flags) }
#[inline]
pub unsafe fn pfault_enabled(gmap: *mut Gmap) -> bool { test_bit(GMAP_FLAG_PFAULT_ENABLED, &(*gmap).flags) }
#[inline]
pub unsafe fn is_ucontrol(gmap: *mut Gmap) -> bool { test_bit(GMAP_FLAG_IS_UCONTROL, &(*gmap).flags) }
#[inline]
pub unsafe fn is_shadow(gmap: *mut Gmap) -> bool { test_bit(GMAP_FLAG_SHADOW, &(*gmap).flags) }
#[inline]
pub unsafe fn owns_page_tables(gmap: *mut Gmap) -> bool { test_bit(GMAP_FLAG_OWNS_PAGETABLES, &(*gmap).flags) }

#[inline]
pub unsafe fn gmap_put(gmap: *mut Gmap) -> *mut Gmap {
    if refcount_dec_and_test(&mut (*gmap).refcount) { gmap_dispose(gmap); }
    ::core::ptr::null_mut()
}
#[inline]
pub unsafe fn gmap_get(gmap: *mut Gmap) { warn_on_once(unlikely(!refcount_inc_not_zero(&mut (*gmap).refcount))); }

#[inline]
pub unsafe fn gmap_handle_vsie_unshadow_event(parent: *mut Gmap, gfn: Gfn) {
    spin_lock(&mut (*parent).children_lock);
    _gmap_handle_vsie_unshadow_event(parent, gfn);
    spin_unlock(&mut (*parent).children_lock);
}

#[cfg(kvm_s390_manages_s390_guest)]
extern "C" { pub fn _gmap_unmap_prefix(gmap: *mut Gmap, gfn: Gfn, end: Gfn, hint: bool) -> bool; }
#[cfg(not(kvm_s390_manages_s390_guest))]
#[inline] pub unsafe fn _gmap_unmap_prefix(_: *mut Gmap, _: Gfn, _: Gfn, _: bool) -> bool { true }
#[inline] pub unsafe fn gmap_mkold_prefix(g: *mut Gmap, s: Gfn, e: Gfn) -> bool { _gmap_unmap_prefix(g,s,e,true) }
#[inline] pub unsafe fn gmap_unmap_prefix(g: *mut Gmap, s: Gfn, e: Gfn) -> bool { _gmap_unmap_prefix(g,s,e,false) }

#[inline]
pub fn pte_needs_unshadow(oldpte: Pte, newpte: Pte, pgste: Pgste) -> bool {
    if !pgste.vsie_notif { return false; }
    if pgste.vsie_gmem { return oldpte.h.p != newpte.h.p || newpte.h.i; }
    !newpte.h.p || !newpte.s.pr
}

#[inline]
pub fn crste_needs_unshadow(oldcrste: Crste, newcrste: Crste) -> bool {
    if !oldcrste.s.fc1.vsie_notif { return false; }
    newcrste.h.p != oldcrste.h.p || newcrste.h.i || !newcrste.s.fc1.vsie_notif
}

#[cfg(kvm_s390_manages_s390_guest)]
extern "C" {
    pub fn gmap_enable_skeys(gmap: *mut Gmap) -> i32;
    pub fn _gmap_set_cmma_all(gmap: *mut Gmap, dirty: bool);
}
#[cfg(kvm_s390_manages_s390_guest)]
#[inline] pub unsafe fn gmap_set_cmma_all_dirty(g: *mut Gmap) { _gmap_set_cmma_all(g, true); }
#[cfg(kvm_s390_manages_s390_guest)]
#[inline] pub unsafe fn gmap_set_cmma_all_clean(g: *mut Gmap) { _gmap_set_cmma_all(g, false); }

#[inline]
pub unsafe fn _gmap_ptep_xchg(gmap: *mut Gmap, ptep: *mut Pte, mut newpte: Pte,
                              mut pgste: Pgste, gfn: Gfn, needs_lock: bool) -> Pgste {
    lockdep_assert_held(&(*(*gmap).kvm).mmu_lock);
    if !needs_lock { lockdep_assert_held(&(*gmap).children_lock); }
    else { lockdep_assert_not_held(&(*gmap).children_lock); }
    if pgste.prefix_notif && (newpte.h.p || newpte.h.i) {
        pgste.prefix_notif = false;
        gmap_unmap_prefix(gmap, gfn, gfn.wrapping_add(1));
    }
    if pte_needs_unshadow(*ptep, newpte, pgste) {
        pgste.vsie_notif = false;
        pgste.vsie_gmem = false;
        if needs_lock { gmap_handle_vsie_unshadow_event(gmap, gfn); }
        else { _gmap_handle_vsie_unshadow_event(gmap, gfn); }
    }
    if !(*ptep).s.d && newpte.s.d && !newpte.s.s { set_page_dirty(pfn_to_page(newpte.h.pfra)); }
    pgste.zero = false;
    __dat_ptep_xchg(ptep, pgste, newpte, gfn, (*gmap).asce, uses_skeys(gmap))
}

#[inline]
pub unsafe fn gmap_ptep_xchg(gmap: *mut Gmap, ptep: *mut Pte, newpte: Pte, pgste: Pgste, gfn: Gfn) -> Pgste {
    _gmap_ptep_xchg(gmap, ptep, newpte, pgste, gfn, true)
}

#[inline]
pub unsafe fn _gmap_crstep_xchg_atomic(gmap: *mut Gmap, crstep: *mut Crste,
                                        oldcrste: Crste, mut newcrste: Crste,
                                        mut gfn: Gfn, needs_lock: bool) -> bool {
    let align = if is_pmd(newcrste) { PAGE_ENTRIES } else { PAGE_ENTRIES * CRST_ENTRIES };
    if kvm_bug_on((*crstep).h.tt != oldcrste.h.tt || newcrste.h.tt != oldcrste.h.tt, (*gmap).kvm) { return true; }
    lockdep_assert_held(&(*(*gmap).kvm).mmu_lock);
    if !needs_lock { lockdep_assert_held(&(*gmap).children_lock); }
    gfn = align_down(gfn, align);
    if crste_prefix(oldcrste) && (newcrste.h.p || newcrste.h.i || !crste_prefix(newcrste)) {
        newcrste.s.fc1.prefix_notif = false;
        gmap_unmap_prefix(gmap, gfn, gfn.wrapping_add(align));
    }
    if crste_leaf(oldcrste) && crste_needs_unshadow(oldcrste, newcrste) {
        newcrste = oldcrste;
        newcrste.s.fc1.vsie_notif = false;
        if needs_lock { gmap_handle_vsie_unshadow_event(gmap, gfn); }
        else { _gmap_handle_vsie_unshadow_event(gmap, gfn); }
        let _ = dat_crstep_xchg_atomic(crstep, oldcrste, newcrste, gfn, (*gmap).asce);
        return false;
    }
    if !oldcrste.s.fc1.d && newcrste.s.fc1.d && !newcrste.s.fc1.s { set_page_dirty(phys_to_page(crste_origin_large(newcrste))); }
    dat_crstep_xchg_atomic(crstep, oldcrste, newcrste, gfn, (*gmap).asce)
}

#[inline]
pub unsafe fn gmap_crstep_xchg_atomic(g: *mut Gmap, c: *mut Crste, o: Crste, n: Crste, f: Gfn) -> bool {
    _gmap_crstep_xchg_atomic(g,c,o,n,f,true)
}

#[inline]
pub unsafe fn gmap_is_shadow_valid(sg: *mut Gmap, asce: Asce, edat_level: i32) -> bool {
    (*sg).guest_asce.val == asce.val && (*sg).edat_level as i32 == edat_level
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
