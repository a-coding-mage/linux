/* SPDX-License-Identifier: GPL-2.0 */
/* iommu_common_header.rs: UltraSparc SBUS/PCI common iommu declarations.
 *
 * Copyright (C) 1999, 2008 David S. Miller (davem@davemloft.net)
 */

/*
 * Dependencies supplied by the surrounding kernel translation are intentionally
 * referenced here rather than reimplemented.
 */

/*
 * These give mapping size of each iommu pte/tlb.
 */
pub const IO_PAGE_SHIFT: u32 = 13;
pub const IO_PAGE_SIZE: usize = 1usize << IO_PAGE_SHIFT;
pub const IO_PAGE_MASK: usize = !(IO_PAGE_SIZE - 1);

#[macro_export]
macro_rules! IO_PAGE_ALIGN {
    ($addr:expr) => {
        (($addr + IO_PAGE_SIZE - 1) & IO_PAGE_MASK)
    };
}

pub const IO_TSB_ENTRIES: usize = 128 * 1024;
pub const IO_TSB_SIZE: usize = IO_TSB_ENTRIES * 8;

/*
 * This is the hardwired shift in the iotlb tag/data parts.
 */
pub const IOMMU_PAGE_SHIFT: u32 = 13;

#[macro_export]
macro_rules! SG_ENT_PHYS_ADDRESS {
    ($sg:expr) => {
        __pa(sg_virt($sg))
    };
}

#[inline]
pub unsafe fn sg_ent_phys_address(sg: *mut scatterlist) -> c_ulong {
    __pa(sg_virt(sg))
}

#[inline]
pub unsafe fn is_span_boundary(
    entry: c_ulong,
    shift: c_ulong,
    boundary_size: c_ulong,
    outs: *mut scatterlist,
    sg: *mut scatterlist,
) -> c_int {
    let paddr: c_ulong = sg_ent_phys_address(outs);
    let nr: c_int = iommu_num_pages(
        paddr,
        (*outs).dma_length.wrapping_add((*sg).length),
        IO_PAGE_SIZE as c_ulong,
    );

    iommu_is_span_boundary(entry, nr, shift, boundary_size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
