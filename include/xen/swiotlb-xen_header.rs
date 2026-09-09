/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __LINUX_SWIOTLB_XEN_H
// Dependencies supplied by the original includes:
//   <linux/swiotlb.h>
//   <asm/xen/swiotlb-xen.h>

extern "C" {
    pub fn xen_dma_sync_for_cpu(
        dev: *mut device,
        handle: dma_addr_t,
        size: size_t,
        dir: dma_data_direction,
    );

    pub fn xen_dma_sync_for_device(
        dev: *mut device,
        handle: dma_addr_t,
        size: size_t,
        dir: dma_data_direction,
    );

    pub static xen_swiotlb_dma_ops: dma_map_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
