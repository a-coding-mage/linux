// SPDX-License-Identifier: GPL-2.0-only
/*
 * Memory Encryption Support Common Code
 *
 * Copyright (C) 2016 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/* Override for DMA direct allocation check - ARCH_HAS_FORCE_DMA_UNENCRYPTED */
pub unsafe fn force_dma_unencrypted(dev: *mut device) -> bool {
    /*
     * For SEV, all DMA must be to unencrypted addresses.
     */
    if cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) {
        return true;
    }

    /*
     * For SME, all DMA must be to unencrypted addresses if the
     * device does not support DMA to addresses that include the
     * encryption mask.
     */
    if cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) {
        let dma_enc_mask: u64 = DMA_BIT_MASK(__ffs64(sme_me_mask));
        let dma_dev_mask: u64 = min_not_zero((*dev).coherent_dma_mask,
                                              (*dev).bus_dma_limit);

        if dma_dev_mask <= dma_enc_mask {
            return true;
        }
    }

    false
}

unsafe fn print_mem_encrypt_feature_info() {
    pr_info!("Memory Encryption Features active: ");

    match cc_vendor {
        CC_VENDOR_INTEL => {
            pr_cont!("Intel TDX\n");
        }
        CC_VENDOR_AMD => {
            pr_cont!("AMD");

            /* Secure Memory Encryption */
            if cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT) {
                /*
                 * SME is mutually exclusive with any of the SEV
                 * features below.
                 */
                pr_cont!(" SME\n");
                return;
            }

            /* Secure Encrypted Virtualization */
            if cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) {
                pr_cont!(" SEV");
            }

            /* Encrypted Register State */
            if cc_platform_has(CC_ATTR_GUEST_STATE_ENCRYPT) {
                pr_cont!(" SEV-ES");
            }

            /* Secure Nested Paging */
            if cc_platform_has(CC_ATTR_GUEST_SEV_SNP) {
                pr_cont!(" SEV-SNP");
            }

            pr_cont!("\n");

            sev_show_status();
        }
        _ => {
            pr_cont!("Unknown\n");
        }
    }
}

/* Architecture __weak replacement functions */
pub unsafe fn mem_encrypt_init() {
    if !cc_platform_has(CC_ATTR_MEM_ENCRYPT) {
        return;
    }

    /* Call into SWIOTLB to update the SWIOTLB DMA buffers */
    swiotlb_update_mem_attributes();

    snp_secure_tsc_prepare();

    print_mem_encrypt_feature_info();
}

pub unsafe fn mem_encrypt_setup_arch() {
    let total_mem: phys_addr_t = memblock_phys_mem_size();
    let mut size: c_ulong;

    /*
     * Do RMP table fixups after the e820 tables have been setup by
     * e820__memory_setup().
     */
    if cc_platform_has(CC_ATTR_HOST_SEV_SNP) {
        snp_fixup_e820_tables();
    }

    if !cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) {
        return;
    }

    /*
     * For SEV and TDX, all DMA has to occur via shared/unencrypted pages.
     * Kernel uses SWIOTLB to make this happen without changing device
     * drivers. However, depending on the workload being run, the
     * default 64MB of SWIOTLB may not be enough and SWIOTLB may
     * run out of buffers for DMA, resulting in I/O errors and/or
     * performance degradation especially with high I/O workloads.
     *
     * Adjust the default size of SWIOTLB using a percentage of guest
     * memory for SWIOTLB buffers. Also, as the SWIOTLB bounce buffer
     * memory is allocated from low memory, ensure that the adjusted size
     * is within the limits of low available memory.
     *
     * The percentage of guest memory used here for SWIOTLB buffers
     * is more of an approximation of the static adjustment which
     * 64MB for <1G, and ~128M to 256M for 1G-to-4G, i.e., the 6%
     */
    size = total_mem * 6 / 100;
    size = clamp_val(size, IO_TLB_DEFAULT_SIZE, SZ_1G);
    swiotlb_adjust_size(size);

    /* Set restricted memory access for virtio. */
    virtio_set_mem_acc_cb(virtio_require_restricted_mem_acc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
