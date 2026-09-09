// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-mv78x00/mpp.c
 *
 * MPP functions for Marvell MV78x00 SoCs
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn mv78xx0_pcie_id(dev: *mut u32, rev: *mut u32);
    fn orion_mpp_conf(mpp_list: *mut u32, mask: u32, mpp_max: u32, base: usize);
    fn printk(fmt: *const i8, ...);
}

// External constants supplied by the surrounding kernel translation.
extern "C" {
    static KERN_ERR: i8;
    static MV78100_DEV_ID: u32;
    static MV78100_REV_A0: u32;
    static MPP_78100_A0_MASK: u32;
    static MPP_MAX: u32;
    static DEV_BUS_VIRT_BASE: usize;
}

unsafe fn mv78xx0_variant() -> u32 {
    let mut dev: u32 = 0;
    let mut rev: u32 = 0;

    mv78xx0_pcie_id(&mut dev, &mut rev);

    if dev == MV78100_DEV_ID && rev >= MV78100_REV_A0 {
        return MPP_78100_A0_MASK;
    }

    let fmt = b"MPP setup: unknown mv78x00 variant (dev %#x rev %#x)\n\0";
    let _ = KERN_ERR;
    printk(fmt.as_ptr() as *const i8, dev, rev);
    0
}

pub unsafe fn mv78xx0_mpp_conf(mpp_list: *mut u32) {
    orion_mpp_conf(mpp_list, mv78xx0_variant(), MPP_MAX, DEV_BUS_VIRT_BASE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
