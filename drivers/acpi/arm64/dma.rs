// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the corresponding Linux ACPI, device, and DMA headers.

pub unsafe fn acpi_arch_dma_setup(dev: *mut device) {
    let mut ret: i32;
    let mut end: u64;
    let mut mask: u64;
    let mut map: *const bus_dma_region = core::ptr::null();

    /*
     * If @dev is expected to be DMA-capable then the bus code that created
     * it should have initialised its dma_mask pointer by this point. For
     * now, we'll continue the legacy behaviour of coercing it to the
     * coherent mask if not, but we'll no longer do so quietly.
     */
    if (*dev).dma_mask.is_null() {
        dev_warn(dev, "DMA mask not set\0");
        (*dev).dma_mask = &mut (*dev).coherent_dma_mask;
    }

    if (*dev).coherent_dma_mask != 0 {
        end = (*dev).coherent_dma_mask;
    } else {
        end = (1_u64 << 32).wrapping_sub(1);
    }

    if !(*dev).dma_range_map.is_null() {
        dev_dbg(dev, "dma_range_map already set\0");
        return;
    }

    ret = acpi_dma_get_range(dev, &mut map);
    if ret == 0 && !map.is_null() {
        end = dma_range_map_max(map);
        (*dev).dma_range_map = map;
    }

    if ret == -ENODEV {
        ret = iort_dma_get_ranges(dev, &mut end);
    }
    if ret == 0 {
        /*
         * Limit coherent and dma mask based on size retrieved from
         * firmware.
         */
        mask = DMA_BIT_MASK(ilog2(end).wrapping_add(1));
        (*dev).bus_dma_limit = end;
        (*dev).coherent_dma_mask = core::cmp::min((*dev).coherent_dma_mask, mask);
        *(*dev).dma_mask = core::cmp::min(*(*dev).dma_mask, mask);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
