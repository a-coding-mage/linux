/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iommu_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

/* CONFIG_OF_IOMMU */
#[cfg(feature = "CONFIG_OF_IOMMU")]
unsafe extern "C" {
    pub fn of_iommu_configure(
        dev: *mut device,
        master_np: *mut device_node,
        id: *const u32,
    ) -> core::ffi::c_int;

    pub fn of_iommu_get_resv_regions(dev: *mut device, list: *mut list_head);
}

/* !CONFIG_OF_IOMMU */
#[cfg(not(feature = "CONFIG_OF_IOMMU"))]
#[inline]
pub unsafe fn of_iommu_configure(
    _dev: *mut device,
    _master_np: *mut device_node,
    _id: *const u32,
) -> core::ffi::c_int {
    -ENODEV
}

#[cfg(not(feature = "CONFIG_OF_IOMMU"))]
#[inline]
pub unsafe fn of_iommu_get_resv_regions(_dev: *mut device, _list: *mut list_head) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
