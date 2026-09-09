// SPDX-License-Identifier: GPL-2.0
/*
 *     Copyright IBM Corp. 2007, 2011
 *     Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Dependencies are supplied by the surrounding kernel Rust bindings.

pub unsafe fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t {
    // mio_wb_bit_mask may be set on a different CPU, but is only set once at
    // init and only read afterwards.
    __pgprot(pgprot_val(prot) | mio_wb_bit_mask)
}

unsafe fn ptep_ipte_local(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, nodat: c_int) {
    let mut opt: c_ulong;
    let mut asce: c_ulong;
    if machine_has_tlb_guest() {
        opt = 0;
        asce = READ_ONCE((*mm).context.gmap_asce);
        if asce == 0 || nodat != 0 { opt |= IPTE_NODAT; }
        if asce != !0 {
            asce = if asce != 0 { asce } else { (*mm).context.asce };
            opt |= IPTE_GUEST_ASCE;
        }
        __ptep_ipte(addr, ptep, opt, asce, IPTE_LOCAL);
    } else { __ptep_ipte(addr, ptep, 0, 0, IPTE_LOCAL); }
}

unsafe fn ptep_ipte_global(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, nodat: c_int) {
    let mut opt: c_ulong;
    let mut asce: c_ulong;
    if machine_has_tlb_guest() {
        opt = 0;
        asce = READ_ONCE((*mm).context.gmap_asce);
        if asce == 0 || nodat != 0 { opt |= IPTE_NODAT; }
        if asce != !0 {
            asce = if asce != 0 { asce } else { (*mm).context.asce };
            opt |= IPTE_GUEST_ASCE;
        }
        __ptep_ipte(addr, ptep, opt, asce, IPTE_GLOBAL);
    } else { __ptep_ipte(addr, ptep, 0, 0, IPTE_GLOBAL); }
}

unsafe fn ptep_flush_direct(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, nodat: c_int) -> pte_t {
    let old = *ptep;
    if pte_val(old) & _PAGE_INVALID != 0 { return old; }
    atomic_inc(&mut (*mm).context.flush_count);
    if cpu_has_tlb_lc() && cpumask_equal(mm_cpumask(mm), cpumask_of(smp_processor_id())) { ptep_ipte_local(mm, addr, ptep, nodat); } else { ptep_ipte_global(mm, addr, ptep, nodat); }
    atomic_dec(&mut (*mm).context.flush_count);
    old
}

unsafe fn ptep_flush_lazy(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, nodat: c_int) -> pte_t {
    let old = *ptep;
    if pte_val(old) & _PAGE_INVALID != 0 { return old; }
    atomic_inc(&mut (*mm).context.flush_count);
    if cpumask_equal(&(*mm).context.cpu_attach_mask, cpumask_of(smp_processor_id())) {
        set_pte(ptep, set_pte_bit(*ptep, __pgprot(_PAGE_INVALID)));
        (*mm).context.flush_mm = 1;
    } else { ptep_ipte_global(mm, addr, ptep, nodat); }
    atomic_dec(&mut (*mm).context.flush_count);
    old
}

pub unsafe fn ptep_xchg_direct(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, new: pte_t) -> pte_t {
    preempt_disable(); let old = ptep_flush_direct(mm, addr, ptep, 1); set_pte(ptep, new); preempt_enable(); old
}

pub unsafe fn ptep_reset_dat_prot(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, new: pte_t) {
    preempt_disable(); atomic_inc(&mut (*mm).context.flush_count);
    if cpumask_equal(mm_cpumask(mm), cpumask_of(smp_processor_id())) { __ptep_rdp(addr, ptep, 1); } else { __ptep_rdp(addr, ptep, 0); }
    // RDP clears only _PAGE_PROTECT; changing only software bits is allowed.
    set_pte(ptep, new); atomic_dec(&mut (*mm).context.flush_count); preempt_enable();
}

pub unsafe fn ptep_xchg_lazy(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, new: pte_t) -> pte_t {
    preempt_disable(); let old = ptep_flush_lazy(mm, addr, ptep, 1); set_pte(ptep, new); preempt_enable(); old
}

pub unsafe fn ptep_modify_prot_start(vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t) -> pte_t { ptep_flush_lazy((*vma).vm_mm, addr, ptep, 1) }
pub unsafe fn ptep_modify_prot_commit(_vma: *mut vm_area_struct, _addr: c_ulong, ptep: *mut pte_t, _old_pte: pte_t, pte: pte_t) { set_pte(ptep, pte); }

unsafe fn pmdp_idte_local(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t) { if machine_has_tlb_guest() { __pmdp_idte(addr, pmdp, IDTE_NODAT | IDTE_GUEST_ASCE, (*mm).context.asce, IDTE_LOCAL); } else { __pmdp_idte(addr, pmdp, 0, 0, IDTE_LOCAL); } }
unsafe fn pmdp_idte_global(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t) { if machine_has_tlb_guest() { __pmdp_idte(addr, pmdp, IDTE_NODAT | IDTE_GUEST_ASCE, (*mm).context.asce, IDTE_GLOBAL); } else { __pmdp_idte(addr, pmdp, 0, 0, IDTE_GLOBAL); } }

unsafe fn pmdp_flush_direct(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t) -> pmd_t {
    let old = *pmdp; if pmd_val(old) & _SEGMENT_ENTRY_INVALID != 0 { return old; }
    atomic_inc(&mut (*mm).context.flush_count);
    if cpu_has_tlb_lc() && cpumask_equal(mm_cpumask(mm), cpumask_of(smp_processor_id())) { pmdp_idte_local(mm, addr, pmdp); } else { pmdp_idte_global(mm, addr, pmdp); }
    atomic_dec(&mut (*mm).context.flush_count); old
}
unsafe fn pmdp_flush_lazy(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t) -> pmd_t {
    let old = *pmdp; if pmd_val(old) & _SEGMENT_ENTRY_INVALID != 0 { return old; }
    atomic_inc(&mut (*mm).context.flush_count);
    if cpumask_equal(&(*mm).context.cpu_attach_mask, cpumask_of(smp_processor_id())) { set_pmd(pmdp, set_pmd_bit(*pmdp, __pgprot(_SEGMENT_ENTRY_INVALID))); (*mm).context.flush_mm = 1; } else { pmdp_idte_global(mm, addr, pmdp); }
    atomic_dec(&mut (*mm).context.flush_count); old
}
pub unsafe fn pmdp_xchg_direct(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t, new: pmd_t) -> pmd_t { preempt_disable(); let old = pmdp_flush_direct(mm, addr, pmdp); set_pmd(pmdp, new); preempt_enable(); old }
pub unsafe fn pmdp_xchg_lazy(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t, new: pmd_t) -> pmd_t { preempt_disable(); let old = pmdp_flush_lazy(mm, addr, pmdp); set_pmd(pmdp, new); preempt_enable(); old }

unsafe fn pudp_idte_local(mm: *mut mm_struct, addr: c_ulong, pudp: *mut pud_t) { if machine_has_tlb_guest() { __pudp_idte(addr, pudp, IDTE_NODAT | IDTE_GUEST_ASCE, (*mm).context.asce, IDTE_LOCAL); } else { __pudp_idte(addr, pudp, 0, 0, IDTE_LOCAL); } }
unsafe fn pudp_idte_global(mm: *mut mm_struct, addr: c_ulong, pudp: *mut pud_t) { if machine_has_tlb_guest() { __pudp_idte(addr, pudp, IDTE_NODAT | IDTE_GUEST_ASCE, (*mm).context.asce, IDTE_GLOBAL); } else { __pudp_idte(addr, pudp, 0, 0, IDTE_GLOBAL); } }
unsafe fn pudp_flush_direct(mm: *mut mm_struct, addr: c_ulong, pudp: *mut pud_t) -> pud_t { let old = *pudp; if pud_val(old) & _REGION_ENTRY_INVALID != 0 { return old; } atomic_inc(&mut (*mm).context.flush_count); if cpu_has_tlb_lc() && cpumask_equal(mm_cpumask(mm), cpumask_of(smp_processor_id())) { pudp_idte_local(mm, addr, pudp); } else { pudp_idte_global(mm, addr, pudp); } atomic_dec(&mut (*mm).context.flush_count); old }
pub unsafe fn pudp_xchg_direct(mm: *mut mm_struct, addr: c_ulong, pudp: *mut pud_t, new: pud_t) -> pud_t { preempt_disable(); let old = pudp_flush_direct(mm, addr, pudp); set_pud(pudp, new); preempt_enable(); old }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pgtable_trans_huge_deposit(mm: *mut mm_struct, pmdp: *mut pmd_t, pgtable: pgtable_t) {
    let lh = pgtable as *mut list_head; assert_spin_locked(pmd_lockptr(mm, pmdp));
    if pmd_huge_pte(mm, pmdp).is_null() { INIT_LIST_HEAD(lh); } else { list_add(lh, pmd_huge_pte(mm, pmdp) as *mut list_head); }
    *pmd_huge_pte_ptr(mm, pmdp) = pgtable;
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe fn pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t {
    assert_spin_locked(pmd_lockptr(mm, pmdp)); let pgtable = pmd_huge_pte(mm, pmdp); let lh = pgtable as *mut list_head;
    if list_empty(lh) { *pmd_huge_pte_ptr(mm, pmdp) = core::ptr::null_mut(); } else { *pmd_huge_pte_ptr(mm, pmdp) = (*lh).next as pgtable_t; list_del(lh); }
    let ptep = pgtable as *mut pte_t; set_pte(ptep, __pte(_PAGE_INVALID)); set_pte(ptep.add(1), __pte(_PAGE_INVALID)); pgtable
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
