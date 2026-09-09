/* SPDX-License-Identifier: GPL-2.0 */

// C header guard omitted from executable Rust.

unsafe extern "C" {
    pub fn flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: ::core::ffi::c_ulong);
}

#[inline]
pub unsafe fn check_and_get_huge_psize(shift: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if (shift & 1) != 0 {
        // Not a power of 4
        return -EINVAL;
    }

    shift_to_mmu_psize(shift)
}

#[inline]
pub unsafe fn arch_make_huge_pte(
    entry: pte_t,
    shift: ::core::ffi::c_uint,
    _flags: vm_flags_t,
) -> pte_t {
    let tsize: ::core::ffi::c_uint = shift - _PAGE_PSIZE_SHIFT_OFFSET;
    let val: pte_basic_t = (tsize << _PAGE_PSIZE_SHIFT) & _PAGE_PSIZE_MSK;

    __pte((pte_val(entry) & !(_PAGE_PSIZE_MSK as pte_basic_t)) | val)
}

// #define arch_make_huge_pte arch_make_huge_pte

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
