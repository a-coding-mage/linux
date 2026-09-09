/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: ASMARM_DMA_IOMMU_H
// The following declarations are conditional on the C build configuration
// (__KERNEL__). This condition is preserved here as source-level intent.

use core::ffi::c_ulong;

#[repr(C)]
pub struct dma_iommu_mapping {
	/* iommu specific data */
	pub domain: *mut iommu_domain,

	pub bitmaps: *mut *mut c_ulong, /* array of bitmaps */
	pub nr_bitmaps: u32,            /* nr of elements in array */
	pub extensions: u32,
	pub bitmap_size: usize,         /* size of a single bitmap */
	pub bits: usize,                /* per bitmap */
	pub base: dma_addr_t,

	pub lock: spinlock_t,
	pub kref: kref,
}

extern "C" {
	pub fn arm_iommu_create_mapping(
		dev: *mut device,
		base: dma_addr_t,
		size: u64,
	) -> *mut dma_iommu_mapping;

	pub fn arm_iommu_release_mapping(mapping: *mut dma_iommu_mapping);

	pub fn arm_iommu_attach_device(
		dev: *mut device,
		mapping: *mut dma_iommu_mapping,
	) -> i32;
	pub fn arm_iommu_detach_device(dev: *mut device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
