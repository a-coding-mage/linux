/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_data_direction {
    DMA_BIDIRECTIONAL = 0,
    DMA_TO_DEVICE = 1,
    DMA_FROM_DEVICE = 2,
    DMA_NONE = 3,
}

#[inline]
pub fn valid_dma_direction(dir: dma_data_direction) -> i32 {
    if dir == dma_data_direction::DMA_BIDIRECTIONAL
        || dir == dma_data_direction::DMA_TO_DEVICE
        || dir == dma_data_direction::DMA_FROM_DEVICE
    {
        1
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
