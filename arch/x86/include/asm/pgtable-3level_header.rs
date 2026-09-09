/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Intel Physical Address Extension (PAE) Mode - three-level page
 * tables on PPro+ CPUs.
 *
 * Copyright (C) 1999 Ingo Molnar <mingo@redhat.com>
 */

macro_rules! pte_ERROR {
    ($e:expr) => { pr_err!("{}:{}: bad pte {:p}({:08lx}{:08lx})", file!(), line!(), &$e, $e.pte_high, $e.pte_low) };
}
macro_rules! pmd_ERROR {
    ($e:expr) => { pr_err!("{}:{}: bad pmd {:p}({:016Lx})", file!(), line!(), &$e, pmd_val($e)) };
}
macro_rules! pgd_ERROR {
    ($e:expr) => { pr_err!("{}:{}: bad pgd {:p}({:016Lx})", file!(), line!(), &$e, pgd_val($e)) };
}

/* C token-pasting macro pxx_xchg64; the concrete pte/pmd/pud forms are kept explicit. */
macro_rules! pxx_xchg64 {
    (pte, $ptr:expr, $val:expr) => {{
        let p = $ptr as *mut pteval_t;
        let mut o = unsafe { *p };
        while !try_cmpxchg64(p, &mut o, $val) {}
        native_make_pte(o)
    }};
    (pmd, $ptr:expr, $val:expr) => {{
        let p = $ptr as *mut pmdval_t;
        let mut o = unsafe { *p };
        while !try_cmpxchg64(p, &mut o, $val) {}
        native_make_pmd(o)
    }};
    (pud, $ptr:expr, $val:expr) => {{
        let p = $ptr as *mut pudval_t;
        let mut o = unsafe { *p };
        while !try_cmpxchg64(p, &mut o, $val) {}
        native_make_pud(o)
    }};
}

/* Rules for using set_pte: the pte being assigned must be either not present
 * or in a state where hardware will not attempt to update it. */
#[inline]
unsafe fn native_set_pte(ptep: *mut pte_t, pte: pte_t) {
    WRITE_ONCE!((*ptep).pte_high, pte.pte_high);
    smp_wmb!();
    WRITE_ONCE!((*ptep).pte_low, pte.pte_low);
}

#[inline]
unsafe fn native_set_pte_atomic(ptep: *mut pte_t, pte: pte_t) {
    pxx_xchg64!(pte, ptep, native_pte_val(pte));
}

#[inline]
unsafe fn native_set_pmd(pmdp: *mut pmd_t, pmd: pmd_t) {
    pxx_xchg64!(pmd, pmdp, native_pmd_val(pmd));
}

#[inline]
unsafe fn native_set_pud(pudp: *mut pud_t, mut pud: pud_t) {
    #[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
    { pud.p4d.pgd = pti_set_user_pgtbl(&mut (*pudp).p4d.pgd, pud.p4d.pgd); }
    pxx_xchg64!(pud, pudp, native_pud_val(pud));
}

#[inline]
unsafe fn native_pte_clear(_mm: *mut mm_struct, _addr: c_ulong, ptep: *mut pte_t) {
    WRITE_ONCE!((*ptep).pte_low, 0);
    smp_wmb!();
    WRITE_ONCE!((*ptep).pte_high, 0);
}

#[inline]
unsafe fn native_pmd_clear(pmdp: *mut pmd_t) {
    WRITE_ONCE!((*pmdp).pmd_low, 0);
    smp_wmb!();
    WRITE_ONCE!((*pmdp).pmd_high, 0);
}

#[inline]
unsafe fn native_pud_clear(_pudp: *mut pud_t) {}

#[inline]
unsafe fn pud_clear(pudp: *mut pud_t) {
    set_pud(pudp, __pud(0));
    /* In PAE mode a top-level PGD change would traditionally require a CR3 TLB flush.
     * Callers already provide the required flush where needed. */
}

#[cfg(CONFIG_SMP)]
unsafe fn native_ptep_get_and_clear(ptep: *mut pte_t) -> pte_t { pxx_xchg64!(pte, ptep, 0u64) }
#[cfg(CONFIG_SMP)]
unsafe fn native_pmdp_get_and_clear(pmdp: *mut pmd_t) -> pmd_t { pxx_xchg64!(pmd, pmdp, 0u64) }
#[cfg(CONFIG_SMP)]
unsafe fn native_pudp_get_and_clear(pudp: *mut pud_t) -> pud_t { pxx_xchg64!(pud, pudp, 0u64) }

#[cfg(not(CONFIG_SMP))]
macro_rules! native_ptep_get_and_clear { ($xp:expr) => { native_local_ptep_get_and_clear($xp) }; }
#[cfg(not(CONFIG_SMP))]
macro_rules! native_pmdp_get_and_clear { ($xp:expr) => { native_local_pmdp_get_and_clear($xp) }; }
#[cfg(not(CONFIG_SMP))]
macro_rules! native_pudp_get_and_clear { ($xp:expr) => { native_local_pudp_get_and_clear($xp) }; }

/* Swap-entry encoding for 32-bit swp_entry_t and 64-bit PTEs. */
pub const SWP_TYPE_BITS: u32 = 5;
pub const _SWP_TYPE_MASK: u32 = (1u32 << SWP_TYPE_BITS) - 1;
pub const SWP_OFFSET_FIRST_BIT: u32 = _PAGE_BIT_PROTNONE + 1;
pub const SWP_OFFSET_SHIFT: u32 = SWP_OFFSET_FIRST_BIT + SWP_TYPE_BITS;

macro_rules! MAX_SWAPFILES_CHECK { () => { BUILD_BUG_ON!(MAX_SWAPFILES_SHIFT > SWP_TYPE_BITS) }; }
macro_rules! __swp_type { ($x:expr) => { ($x).val & _SWP_TYPE_MASK }; }
macro_rules! __swp_offset { ($x:expr) => { ($x).val >> SWP_TYPE_BITS }; }
macro_rules! __swp_entry { ($type:expr, $offset:expr) => { swp_entry_t { val: (($type & _SWP_TYPE_MASK) | ($offset << SWP_TYPE_BITS)) } }; }
macro_rules! __swp_pteval_entry { ($type:expr, $offset:expr) => { ((!($offset as pteval_t) << SWP_OFFSET_SHIFT >> SWP_TYPE_BITS) | (($type as pteval_t) << (64 - SWP_TYPE_BITS))) }; }
macro_rules! __swp_entry_to_pte { ($x:expr) => { pte_t { pte: __swp_pteval_entry!(__swp_type!($x), __swp_offset!($x)) } }; }
macro_rules! __pteval_swp_type { ($x:expr) => { (($x).pte >> (64 - SWP_TYPE_BITS)) as c_ulong }; }
macro_rules! __pteval_swp_offset { ($x:expr) => { ((!($x).pte << SWP_TYPE_BITS >> SWP_OFFSET_SHIFT) as c_ulong) }; }
macro_rules! __pte_to_swp_entry { ($pte:expr) => { __swp_entry!(__pteval_swp_type!($pte), __pteval_swp_offset!($pte)) }; }

/* We borrow bit 7 to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: u64 = _PAGE_PSE;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
