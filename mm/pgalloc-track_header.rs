/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding translation unit: linux/mm.h,
 * linux/pgtable.h. */

/* CONFIG_MMU */
#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn p4d_alloc_track(
    mm: *mut mm_struct,
    pgd: *mut pgd_t,
    address: ::core::ffi::c_ulong,
    mod_mask: *mut pgtbl_mod_mask,
) -> *mut p4d_t {
    if pgd_none(*pgd) {
        if __p4d_alloc(mm, pgd, address) != 0 {
            return ::core::ptr::null_mut();
        }
        *mod_mask |= PGTBL_PGD_MODIFIED;
    }

    p4d_offset(pgd, address)
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn pud_alloc_track(
    mm: *mut mm_struct,
    p4d: *mut p4d_t,
    address: ::core::ffi::c_ulong,
    mod_mask: *mut pgtbl_mod_mask,
) -> *mut pud_t {
    if p4d_none(*p4d) {
        if __pud_alloc(mm, p4d, address) != 0 {
            return ::core::ptr::null_mut();
        }
        *mod_mask |= PGTBL_P4D_MODIFIED;
    }

    pud_offset(p4d, address)
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn pmd_alloc_track(
    mm: *mut mm_struct,
    pud: *mut pud_t,
    address: ::core::ffi::c_ulong,
    mod_mask: *mut pgtbl_mod_mask,
) -> *mut pmd_t {
    if pud_none(*pud) {
        if __pmd_alloc(mm, pud, address) != 0 {
            return ::core::ptr::null_mut();
        }
        *mod_mask |= PGTBL_PUD_MODIFIED;
    }

    pmd_offset(pud, address)
}

#[macro_export]
macro_rules! pte_alloc_kernel_track {
    ($pmd:expr, $address:expr, $mask:expr) => {{
        if pmd_none(*$pmd) {
            if __pte_alloc_kernel($pmd) != 0 {
                ::core::ptr::null_mut()
            } else {
                *$mask |= PGTBL_PMD_MODIFIED;
                pte_offset_kernel($pmd, $address)
            }
        } else {
            pte_offset_kernel($pmd, $address)
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
