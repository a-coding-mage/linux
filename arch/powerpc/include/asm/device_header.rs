/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Arch specific extensions to struct device
 */

/* Forward declarations from the C header. */
// struct device_node;
// #ifdef CONFIG_PPC64
// struct pci_dn;
// struct iommu_table;
// #endif

/*
 * Arch extensions to struct device.
 *
 * When adding fields, consider macio_add_one_device in
 * drivers/macintosh/macio_asic.c
 */
#[repr(C)]
pub struct dev_archdata {
    /*
     * These two used to be a union. However, with the hybrid ops we need
     * both so here we store both a DMA offset for direct mappings and
     * an iommu_table for remapped DMA.
     */
    pub dma_offset: dma_addr_t,

    #[cfg(CONFIG_PPC64)]
    pub iommu_table_base: *mut iommu_table,

    #[cfg(CONFIG_PPC64)]
    pub pci_data: *mut pci_dn,

    #[cfg(CONFIG_EEH)]
    pub edev: *mut eeh_dev,

    #[cfg(CONFIG_FAIL_IOMMU)]
    pub fail_iommu: ::core::ffi::c_int,

    #[cfg(CONFIG_PCI_IOV)]
    pub iov_data: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct pdev_archdata {
    pub dma_mask: u64,
    /*
     * Pointer to nvdimm_pmu structure, to handle the unregistering
     * of pmu device
     */
    pub priv_: *mut ::core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
