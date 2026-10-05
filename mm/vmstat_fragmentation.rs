// SPDX-License-Identifier: GPL-2.0-only
// Internal Rust-only aggregate; never crosses the C ABI.
struct ContigPageInfo {
    free_pages: c_ulong,
    free_blocks_total: c_ulong,
    free_blocks_suitable: c_ulong,
}
unsafe fn fill_contig_page_info(z: *mut zone, suitable_order: c_uint) -> ContigPageInfo {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mut info = ContigPageInfo {
            free_pages: 0,
            free_blocks_total: 0,
            free_blocks_suitable: 0,
        };
        for order in 0..RUST_VMSTAT_NR_PAGE_ORDERS {
            let blocks = rust_vmstat_free_blocks(z, order);
            info.free_blocks_total = info.free_blocks_total.wrapping_add(blocks);
            info.free_pages = info.free_pages.wrapping_add(blocks.wrapping_shl(order));
            if order >= suitable_order {
                info.free_blocks_suitable = info
                    .free_blocks_suitable
                    .wrapping_add(blocks.wrapping_shl(order - suitable_order));
            }
        }
        info
    }
}
unsafe fn __fragmentation_index(order: c_uint, info: &ContigPageInfo) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let requested = (1 as c_ulong).wrapping_shl(order);
        if rust_vmstat_warn_order(order > RUST_VMSTAT_MAX_PAGE_ORDER) {
            return 0;
        }
        if info.free_blocks_total == 0 {
            return 0;
        }
        if info.free_blocks_suitable != 0 {
            return -1000;
        }
        // div_u64's divisor is u32 even on a 64-bit kernel. Preserve both
        // native conversions and the 64-bit ULL promotion before multiplication.
        let inner = rust_vmstat_div_u64(
            (info.free_pages as u64).wrapping_mul(1000),
            requested as u32,
        );
        (1000u64.wrapping_sub(rust_vmstat_div_u64(
            1000u64.wrapping_add(inner),
            info.free_blocks_total as u32,
        ))) as c_int
    }
}
#[no_mangle]
unsafe extern "C" fn fragmentation_index(z: *mut zone, order: c_uint) -> c_int {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe { __fragmentation_index(order, &fill_contig_page_info(z, order)) }
}
#[no_mangle]
unsafe extern "C" fn extfrag_for_order(z: *mut zone, order: c_uint) -> c_uint {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let info = fill_contig_page_info(z, order);
        if info.free_pages == 0 {
            return 0;
        }
        let unusable = info
            .free_pages
            .wrapping_sub(info.free_blocks_suitable.wrapping_shl(order));
        // The original multiplies in unsigned long, then converts to u64.
        rust_vmstat_div_u64(unusable.wrapping_mul(100) as u64, info.free_pages as u32) as c_uint
    }
}
