// SPDX-License-Identifier: GPL-2.0
/*
 * IOMMU helper functions for the free area management
 */

unsafe extern "C" {
    fn bitmap_find_next_zero_area(
        map: *mut usize,
        size: usize,
        start: usize,
        nr: usize,
        align_mask: usize,
    ) -> usize;
    fn iommu_is_span_boundary(
        index: usize,
        nr: usize,
        shift: usize,
        boundary_size: usize,
    ) -> bool;
    fn bitmap_set(map: *mut usize, start: usize, len: usize);
}

#[inline]
unsafe fn align(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

pub unsafe fn iommu_area_alloc(
    map: *mut usize,
    mut size: usize,
    mut start: usize,
    nr: u32,
    shift: usize,
    boundary_size: usize,
    align_mask: usize,
) -> usize {
    let index: usize;

    /* We don't want the last of the limit */
    size = size.wrapping_sub(1);
    loop {
        index = bitmap_find_next_zero_area(map, size, start, nr as usize, align_mask);
        if index < size {
            if iommu_is_span_boundary(index, nr as usize, shift, boundary_size) {
                start = align(shift.wrapping_add(index), boundary_size).wrapping_sub(shift);
                continue;
            }
            bitmap_set(map, index, nr as usize);
            return index;
        }
        return usize::MAX;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
