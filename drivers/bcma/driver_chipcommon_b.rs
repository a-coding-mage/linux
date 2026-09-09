/*
 * Broadcom specific AMBA
 * ChipCommon B Unit driver
 *
 * Copyright 2014, Hauke Mehrtens <hauke@hauke-m.de>
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// Dependencies are supplied by the surrounding kernel/BCMA environment.

unsafe fn bcma_wait_reg(
    bus: *mut bcma_bus,
    addr: *mut core::ffi::c_void,
    mask: u32,
    value: u32,
    timeout: i32,
) -> bool {
    let deadline: unsigned_long = jiffies.wrapping_add(timeout as unsigned_long);
    let mut val: u32;

    loop {
        val = readl(addr);
        if (val & mask) == value {
            return true;
        }
        cpu_relax();
        udelay(10);
        if time_after_eq(jiffies, deadline) {
            break;
        }
    }

    bcma_err(bus, "Timeout waiting for register %p\n", addr);

    false
}

pub unsafe fn bcma_chipco_b_mii_write(
    ccb: *mut bcma_drv_cc_b,
    offset: u32,
    value: u32,
) {
    let bus: *mut bcma_bus = (*(*ccb).core).bus;
    let mii: *mut core::ffi::c_void = (*ccb).mii;

    writel(offset, mii.add(BCMA_CCB_MII_MNG_CTL as usize));
    bcma_wait_reg(
        bus,
        mii.add(BCMA_CCB_MII_MNG_CTL as usize),
        0x0100,
        0x0000,
        100,
    );
    writel(value, mii.add(BCMA_CCB_MII_MNG_CMD_DATA as usize));
    bcma_wait_reg(
        bus,
        mii.add(BCMA_CCB_MII_MNG_CTL as usize),
        0x0100,
        0x0000,
        100,
    );
}

pub unsafe fn bcma_core_chipcommon_b_init(ccb: *mut bcma_drv_cc_b) -> i32 {
    if (*ccb).setup_done {
        return 0;
    }

    (*ccb).setup_done = 1;
    (*ccb).mii = ioremap((*(*ccb).core).addr_s[1], BCMA_CORE_SIZE);
    if (*ccb).mii.is_null() {
        return -ENOMEM;
    }

    0
}

pub unsafe fn bcma_core_chipcommon_b_free(ccb: *mut bcma_drv_cc_b) {
    if !(*ccb).mii.is_null() {
        iounmap((*ccb).mii);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
