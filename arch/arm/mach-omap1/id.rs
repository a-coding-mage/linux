// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap1/id.c
 *
 * OMAP1 CPU identification code
 *
 * Copyright (C) 2004 Nokia Corporation
 * Written by Tony Lindgren <tony@atomide.com>
 */

// Translated from C. External kernel facilities are supplied by other files.

const OMAP_DIE_ID_0: usize = 0xfffe1800;
const OMAP_DIE_ID_1: usize = 0xfffe1804;
const OMAP_PRODUCTION_ID_0: usize = 0xfffe2000;
const OMAP_PRODUCTION_ID_1: usize = 0xfffe2004;
const OMAP32_ID_0: usize = 0xfffed400;
const OMAP32_ID_1: usize = 0xfffed404;

#[repr(C)]
struct OmapId {
    jtag_id: u16, /* Used to determine OMAP type */
    die_rev: u8,  /* Processor revision */
    omap_id: u32, /* OMAP revision */
    type_: u32,   /* Cpu id bits [31:08], cpu class bits [07:00] */
}

static mut omap_revision: u32 = 0;

/* Register values to detect the OMAP version */
static mut omap_ids: [OmapId; 20] = [
    OmapId { jtag_id: 0xb574, die_rev: 0x2, omap_id: 0x03310315, type_: 0x03100000 },
    OmapId { jtag_id: 0x355f, die_rev: 0x0, omap_id: 0x03320000, type_: 0x07300100 },
    OmapId { jtag_id: 0xb55f, die_rev: 0x0, omap_id: 0x03320000, type_: 0x07300300 },
    OmapId { jtag_id: 0xb62c, die_rev: 0x1, omap_id: 0x03320500, type_: 0x08500000 },
    OmapId { jtag_id: 0xb470, die_rev: 0x0, omap_id: 0x03310100, type_: 0x15100000 },
    OmapId { jtag_id: 0xb576, die_rev: 0x0, omap_id: 0x03320000, type_: 0x16100000 },
    OmapId { jtag_id: 0xb576, die_rev: 0x2, omap_id: 0x03320100, type_: 0x16110000 },
    OmapId { jtag_id: 0xb576, die_rev: 0x3, omap_id: 0x03320100, type_: 0x16100c00 },
    OmapId { jtag_id: 0xb576, die_rev: 0x0, omap_id: 0x03320200, type_: 0x16100d00 },
    OmapId { jtag_id: 0xb613, die_rev: 0x0, omap_id: 0x03320300, type_: 0x1610ef00 },
    OmapId { jtag_id: 0xb613, die_rev: 0x0, omap_id: 0x03320300, type_: 0x1610ef00 },
    OmapId { jtag_id: 0xb576, die_rev: 0x1, omap_id: 0x03320100, type_: 0x16110000 },
    OmapId { jtag_id: 0xb58c, die_rev: 0x2, omap_id: 0x03320200, type_: 0x16110b00 },
    OmapId { jtag_id: 0xb58c, die_rev: 0x3, omap_id: 0x03320200, type_: 0x16110c00 },
    OmapId { jtag_id: 0xb65f, die_rev: 0x0, omap_id: 0x03320400, type_: 0x16212300 },
    OmapId { jtag_id: 0xb65f, die_rev: 0x1, omap_id: 0x03320400, type_: 0x16212300 },
    OmapId { jtag_id: 0xb65f, die_rev: 0x1, omap_id: 0x03320500, type_: 0x16212300 },
    OmapId { jtag_id: 0xb5f7, die_rev: 0x0, omap_id: 0x03330000, type_: 0x17100000 },
    OmapId { jtag_id: 0xb5f7, die_rev: 0x1, omap_id: 0x03330100, type_: 0x17100000 },
    OmapId { jtag_id: 0xb5f7, die_rev: 0x2, omap_id: 0x03330100, type_: 0x17100000 },
];

extern "C" {
    fn omap_readl(addr: usize) -> u32;
    static mut system_serial_high: u32;
    static mut system_serial_low: u32;
}

pub unsafe fn omap_rev() -> u32 {
    omap_revision
}

unsafe fn omap_get_jtag_id() -> u16 {
    let mut prod_id = omap_readl(OMAP_PRODUCTION_ID_1);
    let omap_id = omap_readl(OMAP32_ID_1);
    if (prod_id >> 20) == 0 || prod_id == omap_id { prod_id = 0; }
    else { prod_id &= 0xffff; }
    if prod_id != 0 { return prod_id as u16; }
    ((omap_id >> 12) & 0xffff) as u16
}

unsafe fn omap_get_die_rev() -> u8 {
    let mut die_rev = omap_readl(OMAP_DIE_ID_1);
    if ((die_rev >> 12) & 0xffff) as u16 == omap_get_jtag_id() { die_rev = 0; }
    die_rev = (die_rev >> 17) & 0xf;
    if die_rev != 0 { return die_rev as u8; }
    ((omap_readl(OMAP32_ID_1) >> 28) & 0xf) as u8
}

pub unsafe fn omap_check_revision() {
    let jtag_id = omap_get_jtag_id();
    let die_rev = omap_get_die_rev();
    let omap_id = omap_readl(OMAP32_ID_0);
    system_serial_high = omap_readl(OMAP_DIE_ID_0);
    system_serial_low = omap_readl(OMAP_DIE_ID_1);

    for id in omap_ids.iter() {
        if jtag_id == id.jtag_id { omap_revision = id.type_; break; }
    }
    for id in omap_ids.iter() {
        if jtag_id == id.jtag_id && die_rev == id.die_rev { omap_revision = id.type_; break; }
    }
    for id in omap_ids.iter() {
        if jtag_id == id.jtag_id && die_rev == id.die_rev && omap_id == id.omap_id {
            omap_revision = id.type_; break;
        }
    }

    let cpu_type = (omap_revision >> 24) as u8;
    match cpu_type {
        0x07 | 0x08 => omap_revision |= 0x07,
        0x03 | 0x15 => omap_revision |= 0x15,
        0x16 | 0x17 => omap_revision |= 0x16,
        _ => { /* printk(KERN_INFO, unknown CPU type) */ }
    }
    /* printk/pr_info/pr_cont diagnostics are supplied by the kernel integration. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
