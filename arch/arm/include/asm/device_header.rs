/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Arch specific extensions to struct device
 */

// The following cfg feature names preserve the corresponding C build-time
// conditions from the original header.

#[repr(C)]
pub struct dev_archdata {
    #[cfg(feature = "CONFIG_ARM_DMA_USE_IOMMU")]
    pub mapping: *mut dma_iommu_mapping,

    // C declaration: unsigned int dma_ops_setup:1;
    pub dma_ops_setup: u8,
}

pub struct omap_device;

#[repr(C)]
pub struct pdev_archdata {
    #[cfg(feature = "CONFIG_ARCH_OMAP")]
    pub od: *mut omap_device,
}

#[cfg(feature = "CONFIG_ARM_DMA_USE_IOMMU")]
#[macro_export]
macro_rules! to_dma_iommu_mapping {
    ($dev:expr) => {{
        unsafe { (*($dev)).archdata.mapping }
    }};
}

#[cfg(not(feature = "CONFIG_ARM_DMA_USE_IOMMU"))]
#[macro_export]
macro_rules! to_dma_iommu_mapping {
    ($dev:expr) => {
        core::ptr::null_mut()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
