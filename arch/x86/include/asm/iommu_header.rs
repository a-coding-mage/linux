/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding translation unit:
// linux/acpi.h
// asm/e820/api.h

unsafe extern "C" {
    pub static mut force_iommu: ::core::ffi::c_int;
    pub static mut no_iommu: ::core::ffi::c_int;
    pub static mut iommu_detected: ::core::ffi::c_int;
    pub static mut iommu_merge: ::core::ffi::c_int;
    pub static mut panic_on_overflow: ::core::ffi::c_int;
    pub static mut amd_iommu_snp_en: bool;

    #[cfg(CONFIG_SWIOTLB)]
    pub static mut x86_swiotlb_enable: bool;
}

#[cfg(not(CONFIG_SWIOTLB))]
pub const x86_swiotlb_enable: bool = false;

/* 10 seconds */
pub const DMAR_OPERATION_TIMEOUT: cycles_t = tsc_khz.wrapping_mul(10).wrapping_mul(1000);

#[inline]
pub unsafe fn arch_rmrr_sanity_check(
    rmrr: *mut acpi_dmar_reserved_memory,
) -> ::core::ffi::c_int {
    let start: u64 = (*rmrr).base_address;
    let end: u64 = (*rmrr).end_address.wrapping_add(1);
    let entry_type: ::core::ffi::c_int;

    entry_type = e820__get_entry_type(start, end);
    if entry_type == E820_TYPE_RESERVED || entry_type == E820_TYPE_NVS {
        return 0;
    }

    pr_err(
        "No firmware reserved region can cover this RMRR [%#018Lx-%#018Lx], contact BIOS vendor for fixes\n",
        start,
        end.wrapping_sub(1),
    );
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
