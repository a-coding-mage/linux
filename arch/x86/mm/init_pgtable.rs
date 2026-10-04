// SPDX-License-Identifier: GPL-2.0-only
// Header algorithms shared by init.rs and init_64.rs. These private, inlined
// definitions have no exported ABI or second strong provider. The configured
// X86_64 owner has five compile-time levels and folds PGD at runtime on L4.
mod init_pgtable {
    use super::*;

    const PRESENT: c_ulong = b::RUST_MM_PAGE_PRESENT;
    const PROTNONE: c_ulong = b::RUST_MM_PAGE_PROTNONE;
    const DIRTY: c_ulong = b::RUST_MM_PAGE_DIRTY;
    const RW: c_ulong = b::RUST_MM_PAGE_RW;

    #[inline(always)]
    unsafe fn physical_mask() -> c_ulong {
        #[cfg(CONFIG_DYNAMIC_PHYSICAL_MASK)]
        {
            b::physical_mask as c_ulong
        }
        #[cfg(not(CONFIG_DYNAMIC_PHYSICAL_MASK))]
        {
            (1 as c_ulong)
                .wrapping_shl(b::RUST_MM_PHYSICAL_MASK_SHIFT)
                .wrapping_sub(1)
        }
    }

    #[inline(always)]
    unsafe fn pte_pfn_mask() -> c_ulong {
        PAGE_MASK & physical_mask()
    }

    #[inline(always)]
    unsafe fn page_table_flags() -> c_ulong {
        #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
        {
            b::RUST_MM_PAGE_TABLE_NOENC | b::sme_me_mask as c_ulong
        }
        #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
        {
            b::RUST_MM_PAGE_TABLE_NOENC
        }
    }

    // include/linux/pgtable.h: both subtraction operands wrap, including zero.
    #[inline(always)]
    fn addr_end(addr: c_ulong, end: c_ulong, size: c_ulong, mask: c_ulong) -> c_ulong {
        let boundary = addr.wrapping_add(size) & mask;
        if boundary.wrapping_sub(1) < end.wrapping_sub(1) {
            boundary
        } else {
            end
        }
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pgd_addr_end(addr: c_ulong, end: c_ulong) -> c_ulong {
        let size = (1 as c_ulong).wrapping_shl(b::pgdir_shift);
        addr_end(addr, end, size, !size.wrapping_sub(1))
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_p4d_addr_end(addr: c_ulong, end: c_ulong) -> c_ulong {
        addr_end(addr, end, P4D_SIZE, P4D_MASK)
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pud_addr_end(addr: c_ulong, end: c_ulong) -> c_ulong {
        addr_end(addr, end, PUD_SIZE, PUD_MASK)
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pmd_addr_end(addr: c_ulong, end: c_ulong) -> c_ulong {
        addr_end(addr, end, PMD_SIZE, PMD_MASK)
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_protval_4k_2_large(val: pgprotval_t) -> pgprotval_t {
        (val & !(_PAGE_PAT | b::RUST_MM_PAGE_PAT_LARGE))
            | ((val & _PAGE_PAT) << (b::RUST_MM_PAGE_BIT_PAT_LARGE - b::RUST_MM_PAGE_BIT_PAT))
    }

    // pgtable-invert.h: the zero entry is special; every other nonpresent
    // protection inverts the PFN, not just protections with PROTNONE set.
    #[inline(always)]
    fn protnone_mask(val: pgprotval_t) -> phys_addr_t {
        if val != 0 && val & PRESENT == 0 {
            !0
        } else {
            0
        }
    }

    #[inline(always)]
    unsafe fn massage_pgprot(prot: pgprot_t) -> pgprotval_t {
        if prot.pgprot & PRESENT != 0 {
            prot.pgprot & b::__supported_pte_mask
        } else {
            prot.pgprot
        }
    }

    // check_pgprot is a static inline, so its WARN_ONCE state is shared by all
    // three pfn_* calls in the original companion translation unit. PKEY and
    // other unsupported bits are masked only for present entries.
    #[inline(always)]
    unsafe fn check_pgprot(prot: pgprot_t) -> pgprotval_t {
        let massaged = massage_pgprot(prot);
        #[cfg(CONFIG_DEBUG_VM)]
        if prot.pgprot != massaged {
            b::rust_mm_warn_check_pgprot(
                prot.pgprot,
                prot.pgprot ^ massaged,
                b::__supported_pte_mask,
            );
        }
        massaged
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pfn_pte(page_nr: c_ulong, prot: pgprot_t) -> pte_t {
        let mut pfn = (page_nr as phys_addr_t).wrapping_shl(PAGE_SHIFT);
        b::rust_mm_warn_pfn_pte_shadow_stack(prot.pgprot & (DIRTY | RW) == DIRTY);
        pfn ^= protnone_mask(prot.pgprot);
        pfn &= pte_pfn_mask() as phys_addr_t;
        let flags = check_pgprot(prot);
        b::rust_mm___pte(pfn as c_ulong | flags)
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pfn_pmd(page_nr: c_ulong, prot: pgprot_t) -> pmd_t {
        let mut pfn = (page_nr as phys_addr_t).wrapping_shl(PAGE_SHIFT);
        pfn ^= protnone_mask(prot.pgprot);
        pfn &= (PMD_MASK & physical_mask()) as phys_addr_t;
        let flags = check_pgprot(prot);
        b::rust_mm___pmd(pfn as c_ulong | flags)
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pfn_pud(page_nr: c_ulong, prot: pgprot_t) -> pud_t {
        let mut pfn = (page_nr as phys_addr_t).wrapping_shl(PAGE_SHIFT);
        pfn ^= protnone_mask(prot.pgprot);
        pfn &= (PUD_MASK & physical_mask()) as phys_addr_t;
        let flags = check_pgprot(prot);
        b::rust_mm___pud(pfn as c_ulong | flags)
    }

    // The native fields are raw machine values. Under PARAVIRT_XXL, the *_val
    // operations are explicit alternative-patching hooks and may translate them.
    macro_rules! entry_val {
        ($name:ident, $ty:ty, $field:ident, $pv:ident) => {
            #[inline(always)]
            unsafe fn $name(value: $ty) -> c_ulong {
                #[cfg(CONFIG_PARAVIRT_XXL)]
                {
                    b::$pv(value)
                }
                #[cfg(not(CONFIG_PARAVIRT_XXL))]
                {
                    value.$field
                }
            }
        };
    }
    entry_val!(pmd_val, pmd_t, pmd, rust_mm_pv_pmd_val);
    entry_val!(pud_val, pud_t, pud, rust_mm_pv_pud_val);
    entry_val!(p4d_val, p4d_t, p4d, rust_mm_pv_p4d_val);
    entry_val!(pgd_val, pgd_t, pgd, rust_mm_pv_pgd_val);

    #[inline(always)]
    unsafe fn pti_set_user_pgtbl(_pgdp: *mut pgd_t, pgd: pgd_t) -> pgd_t {
        #[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
        if b::rust_mm_feature_pti() {
            // Explicit external PTI provider; only its feature dispatch is H2.
            return b::__pti_set_user_pgtbl(_pgdp, pgd);
        }
        pgd
    }

    #[inline(always)]
    unsafe fn native_set_p4d(p4dp: *mut p4d_t, p4d: p4d_t) {
        if b::rust_mm_pgtable_l5_enabled() || !cfg!(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION) {
            b::rust_mm_write_p4d_once(p4dp, p4d);
            return;
        }
        let pgd = pti_set_user_pgtbl(p4dp.cast(), pgd_t { pgd: p4d.p4d });
        b::rust_mm_write_p4d_once(p4dp, p4d_t { p4d: pgd.pgd });
    }

    #[inline(always)]
    unsafe fn native_set_pgd(pgdp: *mut pgd_t, pgd: pgd_t) {
        let value = pti_set_user_pgtbl(pgdp, pgd);
        b::rust_mm_write_pgd_once(pgdp, value);
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_set_pgd(pgdp: *mut pgd_t, pgd: pgd_t) {
        #[cfg(CONFIG_PARAVIRT_XXL)]
        {
            // paravirt.h:set_pgd chooses PGD or P4D before the PVOP site.
            if b::rust_mm_pgtable_l5_enabled() {
                b::rust_mm_pv_set_pgd(pgdp, pgd);
            } else {
                b::rust_mm_pv_set_p4d(pgdp.cast(), p4d_t { p4d: pgd.pgd });
            }
        }
        #[cfg(not(CONFIG_PARAVIRT_XXL))]
        native_set_pgd(pgdp, pgd);
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_set_p4d(p4dp: *mut p4d_t, p4d: p4d_t) {
        #[cfg(CONFIG_PARAVIRT_XXL)]
        b::rust_mm_pv_set_p4d(p4dp, p4d);
        #[cfg(not(CONFIG_PARAVIRT_XXL))]
        native_set_p4d(p4dp, p4d);
    }

    macro_rules! lower_setter {
        ($name:ident, $ty:ty, $pv:ident, $write:ident) => {
            #[inline(always)]
            pub(super) unsafe fn $name(p: *mut $ty, value: $ty) {
                #[cfg(CONFIG_PARAVIRT_XXL)]
                b::$pv(p, value);
                #[cfg(not(CONFIG_PARAVIRT_XXL))]
                b::$write(p, value);
            }
        };
    }
    lower_setter!(
        rust_mm_set_pud,
        pud_t,
        rust_mm_pv_set_pud,
        rust_mm_write_pud_once
    );
    lower_setter!(
        rust_mm_set_pmd,
        pmd_t,
        rust_mm_pv_set_pmd,
        rust_mm_write_pmd_once
    );
    lower_setter!(
        rust_mm_set_pte,
        pte_t,
        rust_mm_pv_set_pte,
        rust_mm_write_pte_once
    );

    // Clear aliases must use native zero values, not paravirt __p* conversion.
    // In particular p4d_clear must not reintroduce C native_set_p4d policy.
    #[inline(always)]
    pub(super) unsafe fn rust_mm_p4d_clear(p: *mut p4d_t) {
        rust_mm_set_p4d(p, p4d_t { p4d: 0 });
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pud_clear(p: *mut pud_t) {
        rust_mm_set_pud(p, pud_t { pud: 0 });
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pmd_clear(p: *mut pmd_t) {
        rust_mm_set_pmd(p, pmd_t { pmd: 0 });
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pte_clear(_mm: *mut mm_struct, _addr: c_ulong, p: *mut pte_t) {
        rust_mm_set_pte(p, pte_t { pte: 0 });
    }

    // Keep the flags masks even for low PRESENT/PROTNONE/PSE bits: the original
    // predicates use flags, and the PFN mask is runtime-configured on SME builds.
    #[inline(always)]
    unsafe fn pte_present(value: pte_t) -> bool {
        value.pte & !pte_pfn_mask() & (PRESENT | PROTNONE) != 0
    }

    #[inline(always)]
    unsafe fn pmd_present(value: pmd_t) -> bool {
        let mask = if value.pmd & _PAGE_PSE != 0 {
            PMD_MASK
        } else {
            PAGE_MASK
        };
        value.pmd & !(mask & physical_mask()) & (PRESENT | PROTNONE | _PAGE_PSE) != 0
    }

    #[inline(always)]
    unsafe fn pud_present(value: pud_t) -> bool {
        let mask = if value.pud & _PAGE_PSE != 0 {
            PUD_MASK
        } else {
            PAGE_MASK
        };
        value.pud & !(mask & physical_mask()) & PRESENT != 0
    }

    #[inline(always)]
    unsafe fn p4d_present(value: p4d_t) -> bool {
        value.p4d & !pte_pfn_mask() & PRESENT != 0
    }

    #[inline(always)]
    unsafe fn pgd_present(value: pgd_t) -> bool {
        !b::rust_mm_pgtable_l5_enabled() || value.pgd & !pte_pfn_mask() & PRESENT != 0
    }

    #[inline(always)]
    unsafe fn pte_val_same(value: pte_t) -> c_ulong {
        value.pte
    }

    // Read for the same-value comparison only after present() succeeds, as in
    // the original macro. A conflict warns and still performs the setter.
    macro_rules! safe_setter {
        ($name:ident, $ty:ty, $present:ident, $val:ident, $set:ident, $warn:ident) => {
            #[inline(always)]
            pub(super) unsafe fn $name(p: *mut $ty, value: $ty) {
                let conflict = $present(*p) && $val(*p) != $val(value);
                b::$warn(conflict);
                $set(p, value);
            }
        };
    }
    safe_setter!(
        rust_mm_set_pte_safe,
        pte_t,
        pte_present,
        pte_val_same,
        rust_mm_set_pte,
        rust_mm_warn_set_pte_safe
    );
    safe_setter!(
        rust_mm_set_pmd_safe,
        pmd_t,
        pmd_present,
        pmd_val,
        rust_mm_set_pmd,
        rust_mm_warn_set_pmd_safe
    );
    safe_setter!(
        rust_mm_set_pud_safe,
        pud_t,
        pud_present,
        pud_val,
        rust_mm_set_pud,
        rust_mm_warn_set_pud_safe
    );
    safe_setter!(
        rust_mm_set_p4d_safe,
        p4d_t,
        p4d_present,
        p4d_val,
        rust_mm_set_p4d,
        rust_mm_warn_set_p4d_safe
    );

    #[inline(always)]
    unsafe fn store_populated_pgd(p: *mut pgd_t, child: *mut p4d_t) {
        // paravirt set_pgd is a macro: its branch precedes evaluating the
        // population's entry expression. A by-value call would move the
        // make/address/encryption-state operations before the branch.
        #[cfg(CONFIG_PARAVIRT_XXL)]
        {
            if b::rust_mm_pgtable_l5_enabled() {
                let value = b::rust_mm___pgd(b::rust_mm___pa(child.cast()) | page_table_flags());
                b::rust_mm_pv_set_pgd(p, value);
            } else {
                let value = b::rust_mm___pgd(b::rust_mm___pa(child.cast()) | page_table_flags());
                b::rust_mm_pv_set_p4d(p.cast(), p4d_t { p4d: value.pgd });
            }
        }
        #[cfg(not(CONFIG_PARAVIRT_XXL))]
        rust_mm_set_pgd(
            p,
            b::rust_mm___pgd(b::rust_mm___pa(child.cast()) | page_table_flags()),
        );
    }

    macro_rules! populate_store {
        ($p:ident, $child:ident, $make:ident, rust_mm_set_pgd) => {
            store_populated_pgd($p, $child);
        };
        ($p:ident, $child:ident, $make:ident, $set:ident) => {
            let value = b::$make(b::rust_mm___pa($child.cast()) | page_table_flags());
            $set($p, value);
        };
    }

    // The safe C macros repeat their value expression. Do not precompute an
    // entry: a present entry constructs a comparison value before the warning,
    // then reconstructs the stored value; an absent entry constructs only the
    // stored value, after the warning predicate. __pa, SME state and paravirt
    // make/value hooks must therefore remain at each original evaluation site.
    macro_rules! populate_entry {
        ($p:ident, $child:ident, $make:ident, $set:ident) => {
            populate_store!($p, $child, $make, $set);
        };
        ($p:ident, $child:ident, $make:ident, $set:ident,
         $present:ident, $val:ident, $warn:ident) => {
            let conflict = $present(*$p)
                && $val(*$p)
                    != $val(b::$make(
                        b::rust_mm___pa($child.cast()) | page_table_flags(),
                    ));
            b::$warn(conflict);
            populate_store!($p, $child, $make, $set);
        };
    }

    // The hook's address evaluation precedes every entry construction. Normal
    // population has two __pa evaluations total; safe population has two when
    // absent and three when present, following the original macro expansion.
    macro_rules! populate {
        ($name:ident, $entry:ty, $child:ty, $hook:ident, $make:ident, $set:ident $(, $safe:ident)*) => {
            #[inline(always)]
            pub(super) unsafe fn $name(_mm: *mut mm_struct, p: *mut $entry, child: *mut $child) {
                #[cfg(CONFIG_PARAVIRT_XXL)]
                b::$hook(_mm, b::rust_mm___pa(child.cast()) >> PAGE_SHIFT);
                #[cfg(not(CONFIG_PARAVIRT_XXL))]
                let _ = b::rust_mm___pa(child.cast()) >> PAGE_SHIFT;
                populate_entry!(p, child, $make, $set $(, $safe)*);
            }
        };
    }
    populate!(
        rust_mm_pmd_populate_kernel,
        pmd_t,
        pte_t,
        rust_mm_pv_alloc_pte,
        rust_mm___pmd,
        rust_mm_set_pmd
    );
    populate!(
        rust_mm_pmd_populate_kernel_safe,
        pmd_t,
        pte_t,
        rust_mm_pv_alloc_pte,
        rust_mm___pmd,
        rust_mm_set_pmd,
        pmd_present,
        pmd_val,
        rust_mm_warn_pmd_populate_kernel_safe
    );
    populate!(
        rust_mm_pud_populate,
        pud_t,
        pmd_t,
        rust_mm_pv_alloc_pmd,
        rust_mm___pud,
        rust_mm_set_pud
    );
    populate!(
        rust_mm_pud_populate_safe,
        pud_t,
        pmd_t,
        rust_mm_pv_alloc_pmd,
        rust_mm___pud,
        rust_mm_set_pud,
        pud_present,
        pud_val,
        rust_mm_warn_pud_populate_safe
    );
    populate!(
        rust_mm_p4d_populate,
        p4d_t,
        pud_t,
        rust_mm_pv_alloc_pud,
        rust_mm___p4d,
        rust_mm_set_p4d
    );
    populate!(
        rust_mm_p4d_populate_safe,
        p4d_t,
        pud_t,
        rust_mm_pv_alloc_pud,
        rust_mm___p4d,
        rust_mm_set_p4d,
        p4d_present,
        p4d_val,
        rust_mm_warn_p4d_populate_safe
    );

    macro_rules! populate_pgd {
        ($name:ident, $set:ident $(, $safe:ident)*) => {
            #[inline(always)]
            pub(super) unsafe fn $name(_mm: *mut mm_struct, pgd: *mut pgd_t, p4d: *mut p4d_t) {
                if !b::rust_mm_pgtable_l5_enabled() {
                    return;
                }
                #[cfg(CONFIG_PARAVIRT_XXL)]
                b::rust_mm_pv_alloc_p4d(_mm, b::rust_mm___pa(p4d.cast()) >> PAGE_SHIFT);
                #[cfg(not(CONFIG_PARAVIRT_XXL))]
                let _ = b::rust_mm___pa(p4d.cast()) >> PAGE_SHIFT;
                populate_entry!(pgd, p4d, rust_mm___pgd, $set $(, $safe)*);
            }
        };
    }
    populate_pgd!(rust_mm_pgd_populate, rust_mm_set_pgd);
    populate_pgd!(
        rust_mm_pgd_populate_safe,
        rust_mm_set_pgd,
        pgd_present,
        pgd_val,
        rust_mm_warn_pgd_populate_safe
    );

    #[inline(always)]
    pub(super) unsafe fn rust_mm_p4d_alloc(
        mm: *mut mm_struct,
        pgd: *mut pgd_t,
        address: c_ulong,
    ) -> *mut p4d_t {
        if b::rust_mm_pgtable_l5_enabled()
            && (*pgd).pgd == 0
            && b::__p4d_alloc(mm, pgd, address) != 0
        {
            return null_mut();
        }
        // Re-evaluate the runtime fold and reload the entry after allocation.
        if !b::rust_mm_pgtable_l5_enabled() {
            return pgd.cast();
        }
        let base = b::rust_mm___va(pgd_val(*pgd) & pte_pfn_mask()).cast::<p4d_t>();
        let index = (address >> b::RUST_MM_P4D_SHIFT) & b::ptrs_per_p4d.wrapping_sub(1) as c_ulong;
        base.wrapping_add(index as usize)
    }

    #[inline(always)]
    pub(super) unsafe fn rust_mm_pud_alloc(
        mm: *mut mm_struct,
        p4d: *mut p4d_t,
        address: c_ulong,
    ) -> *mut pud_t {
        if (*p4d).p4d & !b::RUST_MM_PAGE_KNL_ERRATUM_MASK == 0
            && b::__pud_alloc(mm, p4d, address) != 0
        {
            return null_mut();
        }
        let base = b::rust_mm___va(p4d_val(*p4d) & pte_pfn_mask()).cast::<pud_t>();
        let index = (address >> b::RUST_MM_PUD_SHIFT) & (PTRS_PER_PUD as c_ulong).wrapping_sub(1);
        base.wrapping_add(index as usize)
    }
}
#[allow(unused_imports)]
use init_pgtable::*;
