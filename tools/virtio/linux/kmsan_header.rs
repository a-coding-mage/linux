// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <linux/gfp.h>

pub unsafe fn kmsan_handle_dma(
    phys: phys_addr_t,
    size: size_t,
    dir: dma_data_direction,
) {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
