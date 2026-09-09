/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by xen/swiotlb-xen.h and xen/xen-ops.h.

#[inline]
pub unsafe fn xen_setup_dma_ops(dev: *mut device) {
    // CONFIG_XEN is a build-time condition in the C source.  Enable the
    // contained code when the corresponding Rust configuration is present.
    #[cfg(feature = "CONFIG_XEN")]
    {
        if xen_swiotlb_detect() != 0 {
            (*dev).dma_ops = &xen_swiotlb_dma_ops;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
