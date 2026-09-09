/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations supplied by the Linux memory-management headers. */
#[repr(C)]
pub struct attribute_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

pub type vm_flags_t = ::core::ffi::c_ulong;

extern "C" {
    pub static mut khugepaged_max_ptes_none: ::core::ffi::c_uint;

    /* CONFIG_TRANSPARENT_HUGEPAGE */
    pub static mut khugepaged_attr_group: attribute_group;

    pub fn khugepaged_init() -> ::core::ffi::c_int;
    pub fn khugepaged_destroy();
    pub fn start_stop_khugepaged() -> ::core::ffi::c_int;
    pub fn __khugepaged_enter(mm: *mut mm_struct);
    pub fn __khugepaged_exit(mm: *mut mm_struct);
    #[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
    pub fn khugepaged_enter_vma(vma: *mut vm_area_struct, vm_flags: vm_flags_t);
    #[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
    pub fn khugepaged_min_free_kbytes_update();
    #[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
    pub fn current_is_khugepaged() -> bool;
    #[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
    pub fn collapse_pte_mapped_thp(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        install_pmd: bool,
    );
    pub fn mm_flags_test(flag: ::core::ffi::c_uint, mm: *mut mm_struct) -> bool;
}

/* MMF_VM_HUGEPAGE is supplied by the Linux memory-management headers. */

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
#[inline]
pub unsafe fn khugepaged_fork(mm: *mut mm_struct, oldmm: *mut mm_struct) {
    if mm_flags_test(MMF_VM_HUGEPAGE, oldmm) {
        __khugepaged_enter(mm);
    }
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
#[inline]
pub unsafe fn khugepaged_exit(mm: *mut mm_struct) {
    if mm_flags_test(MMF_VM_HUGEPAGE, mm) {
        __khugepaged_exit(mm);
    }
}

#[cfg(not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[inline]
pub unsafe fn khugepaged_fork(_mm: *mut mm_struct, _oldmm: *mut mm_struct) {}

#[cfg(not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[inline]
pub unsafe fn khugepaged_exit(_mm: *mut mm_struct) {}

#[cfg(not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[inline]
pub unsafe fn khugepaged_enter_vma(_vma: *mut vm_area_struct, _vm_flags: vm_flags_t) {}

#[cfg(not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[inline]
pub unsafe fn collapse_pte_mapped_thp(
    _mm: *mut mm_struct,
    _addr: ::core::ffi::c_ulong,
    _install_pmd: bool,
) {
}

#[cfg(not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[inline]
pub unsafe fn khugepaged_min_free_kbytes_update() {}

#[cfg(not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[inline]
pub unsafe fn current_is_khugepaged() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
