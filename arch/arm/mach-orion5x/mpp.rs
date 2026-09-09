// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-orion5x/mpp.c
 *
 * MPP functions for Marvell Orion 5x SoCs
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn orion5x_pcie_id(dev: *mut u32, rev: *mut u32);
    fn printk(format: *const u8, ...);
    fn orion_mpp_conf(
        mpp_list: *mut u32,
        variant_mask: u32,
        mpp_max: u32,
        base: usize,
    );
}

// The following constants are supplied by the corresponding translated headers:
// MV88F5181_DEV_ID, MV88F5182_DEV_ID, MV88F5281_DEV_ID, MPP_F5181_MASK,
// MPP_F5182_MASK, MPP_F5281_MASK, MPP_MAX, and ORION5X_DEV_BUS_VIRT_BASE.

unsafe fn orion5x_variant() -> u32 {
    let mut dev: u32 = 0;
    let mut rev: u32 = 0;

    orion5x_pcie_id(&mut dev as *mut u32, &mut rev as *mut u32);

    if dev == MV88F5181_DEV_ID {
        return MPP_F5181_MASK;
    }

    if dev == MV88F5182_DEV_ID {
        return MPP_F5182_MASK;
    }

    if dev == MV88F5281_DEV_ID {
        return MPP_F5281_MASK;
    }

    printk(
        b"MPP setup: unknown orion5x variant (dev %#x rev %#x)\n\0".as_ptr(),
        dev,
        rev,
    );
    0
}

pub unsafe fn orion5x_mpp_conf(mpp_list: *mut u32) {
    orion_mpp_conf(
        mpp_list,
        orion5x_variant(),
        MPP_MAX,
        ORION5X_DEV_BUS_VIRT_BASE,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
