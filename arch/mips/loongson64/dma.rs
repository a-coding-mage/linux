// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation are omitted.

/// Equivalent to the C `struct device` declaration supplied by kernel headers.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    static node_id_offset: u32;

    fn swiotlb_init(force: bool, flags: u32);
}

// `dma_addr_t` and `phys_addr_t` are supplied by the surrounding kernel
// translation and retain their C integer representations here.

pub unsafe fn phys_to_dma(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t {
    let _ = dev;

    /* We extract 2bit node id (bit 44~47, only bit 44~45 used now) from
     * Loongson-3's 48bit address space and embed it into 40bit */
    let nid: i64 = ((paddr >> 44) & 0x3) as i64;

    (((nid << 44) ^ paddr as i64) | (nid << (*node_id_offset as i64))) as dma_addr_t
}

pub unsafe fn dma_to_phys(dev: *mut device, daddr: dma_addr_t) -> phys_addr_t {
    let _ = dev;

    /* We extract 2bit node id (bit 44~47, only bit 44~45 used now) from
     * Loongson-3's 48bit address space and embed it into 40bit */
    let nid: i64 = ((daddr >> *node_id_offset) & 0x3) as i64;

    (((nid << *node_id_offset as i64) ^ daddr as i64) | (nid << 44)) as phys_addr_t
}

pub unsafe fn plat_swiotlb_setup() {
    swiotlb_init(true, SWIOTLB_VERBOSE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
