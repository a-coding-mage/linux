/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation provide
// `BUG_ON`, `is_power_of_2`, and the integer types used here.

#[inline]
pub fn iommu_device_max_index(size: usize, offset: usize, dma_mask: u64) -> usize {
    if (size as u64).wrapping_add(offset as u64) > dma_mask {
        (dma_mask.wrapping_sub(offset as u64).wrapping_add(1)) as usize
    } else {
        size
    }
}

#[inline]
pub fn iommu_is_span_boundary(
    index: u32,
    nr: u32,
    mut shift: usize,
    boundary_size: usize,
) -> bool {
    BUG_ON(!is_power_of_2(boundary_size));

    shift = shift.wrapping_add(index as usize) & boundary_size.wrapping_sub(1);
    shift.wrapping_add(nr as usize) > boundary_size
}

extern "C" {
    pub fn iommu_area_alloc(
        map: *mut usize,
        size: usize,
        start: usize,
        nr: u32,
        shift: usize,
        boundary_size: usize,
        align_mask: usize,
    ) -> usize;
}

#[inline]
pub fn iommu_num_pages(addr: usize, len: usize, io_page_size: usize) -> usize {
    let size = (addr & io_page_size.wrapping_sub(1)).wrapping_add(len);

    size.wrapping_add(io_page_size).wrapping_sub(1) / io_page_size
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
