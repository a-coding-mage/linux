/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2014 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

/* Dependencies supplied by the corresponding architecture headers. */

extern "C" {
    pub fn platform_maar_init(num_pairs: u32) -> u32;
    pub fn maar_init();

    fn write_c0_maari(value: u32);
    fn write_c0_maar(value: u64);
    fn back_to_back_c0_hazard();
    #[cfg(feature = "CONFIG_XPA")]
    fn writex_c0_maar(value: u64);
}

#[inline]
pub unsafe fn write_maar_pair(
    idx: u32,
    mut lower: phys_addr_t,
    mut upper: phys_addr_t,
    mut attrs: u32,
) {
    /* Addresses begin at bit 16, but are shifted right 4 bits. */
    assert!((lower & (0xffff | !(MIPS_MAAR_ADDR << 4))) == 0);
    assert!(((upper & 0xffff) == 0xffff)
        && (((upper & !0xffffu64) & !(MIPS_MAAR_ADDR << 4)) == 0));

    /* Automatically set MIPS_MAAR_VL. */
    attrs |= MIPS_MAAR_VL;

    /*
     * Write the upper address & attributes (both MIPS_MAAR_VL and
     * MIPS_MAAR_VH matter).
     */
    write_c0_maari(idx << 1);
    back_to_back_c0_hazard();
    write_c0_maar(((upper >> 4) & MIPS_MAAR_ADDR) | attrs as u64);
    back_to_back_c0_hazard();
    #[cfg(feature = "CONFIG_XPA")]
    {
        upper >>= MIPS_MAARX_ADDR_SHIFT;
        writex_c0_maar(((upper >> 4) & MIPS_MAARX_ADDR) | MIPS_MAARX_VH);
        back_to_back_c0_hazard();
    }

    /* Write the lower address & attributes. */
    write_c0_maari((idx << 1) | 0x1);
    back_to_back_c0_hazard();
    write_c0_maar((lower >> 4) | attrs as u64);
    back_to_back_c0_hazard();
    #[cfg(feature = "CONFIG_XPA")]
    {
        lower >>= MIPS_MAARX_ADDR_SHIFT;
        writex_c0_maar(((lower >> 4) & MIPS_MAARX_ADDR) | MIPS_MAARX_VH);
        back_to_back_c0_hazard();
    }
}

#[repr(C)]
pub struct maar_config {
    pub lower: phys_addr_t,
    pub upper: phys_addr_t,
    pub attrs: u32,
}

#[inline]
pub unsafe fn maar_config(
    cfg: *const maar_config,
    num_cfg: u32,
    num_pairs: u32,
) -> u32 {
    let mut i = 0u32;
    while i < core::cmp::min(num_cfg, num_pairs) {
        let entry = &*cfg.add(i as usize);
        write_maar_pair(i, entry.lower, entry.upper, entry.attrs);
        i += 1;
    }
    i
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
