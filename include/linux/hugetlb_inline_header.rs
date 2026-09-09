/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by the translated linux/mm.h header.

// CONFIG_HUGETLB_PAGE is a build-time configuration condition from the C
// source; preserve it here as the corresponding Rust cfg condition.
#[cfg(CONFIG_HUGETLB_PAGE)]
#[inline]
fn is_vma_hugetlb_flags(flags: *const vma_flags_t) -> bool {
    unsafe { vma_flags_test(flags, VMA_HUGETLB_BIT) }
}

#[cfg(not(CONFIG_HUGETLB_PAGE))]
#[inline]
fn is_vma_hugetlb_flags(_flags: *const vma_flags_t) -> bool {
    false
}

#[inline]
fn is_vm_hugetlb_page(vma: *const vm_area_struct) -> bool {
    unsafe { is_vma_hugetlb_flags(core::ptr::addr_of!((*vma).flags)) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
