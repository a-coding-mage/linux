/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * DMA BUF Mapping Helpers
 *
 */

// Dependency equivalent of: #include <linux/dma-buf.h>

extern "C" {
    pub fn dma_buf_phys_vec_to_sgt(
        attach: *mut dma_buf_attachment,
        provider: *mut p2pdma_provider,
        phys_vec: *mut phys_vec,
        nr_ranges: usize,
        size: usize,
        dir: dma_data_direction,
    ) -> *mut sg_table;

    pub fn dma_buf_free_sgt(
        attach: *mut dma_buf_attachment,
        sgt: *mut sg_table,
        dir: dma_data_direction,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
