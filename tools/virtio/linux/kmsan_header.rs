// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <linux/gfp.h>

pub unsafe fn kmsan_handle_dma(
    phys: phys_addr_t,
    size: size_t,
    dir: dma_data_direction,
) {
}
