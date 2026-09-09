/*
 * Broadcom specific AMBA
 * PCIe Gen 2 Core
 *
 * Copyright 2014, Broadcom Corporation
 * Copyright 2014, Rafał Miłecki <zajec5@gmail.com>
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// Dependencies supplied by the surrounding BCMA/Linux translation.

unsafe fn bcma_core_pcie2_cfg_write(
    pcie2: *mut bcma_drv_pcie2,
    addr: u32,
    val: u32,
) {
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDADDR, addr);
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDDATA, val);
}

unsafe fn bcma_core_pcie2_war_delay_perst_enab(
    pcie2: *mut bcma_drv_pcie2,
    enable: bool,
) -> u32 {
    let mut val: u32;

    /* restore back to default */
    val = pcie2_read32(pcie2, BCMA_CORE_PCIE2_CLK_CONTROL);
    val |= PCIE2_CLKC_DLYPERST;
    val &= !PCIE2_CLKC_DISSPROMLD;
    if enable {
        val &= !PCIE2_CLKC_DLYPERST;
        val |= PCIE2_CLKC_DISSPROMLD;
    }
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CLK_CONTROL, val);
    /* flush */
    pcie2_read32(pcie2, BCMA_CORE_PCIE2_CLK_CONTROL)
}

unsafe fn bcma_core_pcie2_set_ltr_vals(pcie2: *mut bcma_drv_pcie2) {
    /* LTR0 */
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDADDR, 0x844);
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDDATA, 0x883c883c);
    /* LTR1 */
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDADDR, 0x848);
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDDATA, 0x88648864);
    /* LTR2 */
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDADDR, 0x84c);
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDDATA, 0x90039003);
}

unsafe fn bcma_core_pcie2_hw_ltr_war(pcie2: *mut bcma_drv_pcie2) {
    let core_rev: u8 = (*(*pcie2).core).id.rev;
    let mut devstsctr2: u32;

    if core_rev < 2 || core_rev == 10 || core_rev > 13 {
        return;
    }

    pcie2_write32(
        pcie2,
        BCMA_CORE_PCIE2_CONFIGINDADDR,
        PCIE2_CAP_DEVSTSCTRL2_OFFSET,
    );
    devstsctr2 = pcie2_read32(pcie2, BCMA_CORE_PCIE2_CONFIGINDDATA);
    if devstsctr2 & PCIE2_CAP_DEVSTSCTRL2_LTRENAB != 0 {
        /* force the right LTR values */
        bcma_core_pcie2_set_ltr_vals(pcie2);

        /* TODO:
         * si_core_wrapperreg(pcie2, 3, 0x60, 0x8080, 0);
         */

        /* enable the LTR */
        devstsctr2 |= PCIE2_CAP_DEVSTSCTRL2_LTRENAB;
        pcie2_write32(
            pcie2,
            BCMA_CORE_PCIE2_CONFIGINDADDR,
            PCIE2_CAP_DEVSTSCTRL2_OFFSET,
        );
        pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDDATA, devstsctr2);

        /* set the LTR state to be active */
        pcie2_write32(pcie2, BCMA_CORE_PCIE2_LTR_STATE, PCIE2_LTR_ACTIVE);
        usleep_range(1000, 2000);

        /* set the LTR state to be sleep */
        pcie2_write32(pcie2, BCMA_CORE_PCIE2_LTR_STATE, PCIE2_LTR_SLEEP);
        usleep_range(1000, 2000);
    }
}

unsafe fn pciedev_crwlpciegen2(pcie2: *mut bcma_drv_pcie2) {
    let core_rev: u8 = (*(*pcie2).core).id.rev;
    let pciewar160 = core_rev == 7 || core_rev == 9 || core_rev == 11;
    let pciewar162 = core_rev == 5
        || core_rev == 7
        || core_rev == 8
        || core_rev == 9
        || core_rev == 11;

    if !pciewar160 && !pciewar162 {
        return;
    }

    /* TODO: disabled C implementation retained intentionally. */
}

unsafe fn pciedev_crwlpciegen2_180(pcie2: *mut bcma_drv_pcie2) {
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDADDR, PCIE2_PMCR_REFUP);
    pcie2_set32(pcie2, BCMA_CORE_PCIE2_CONFIGINDDATA, 0x1f);
}

unsafe fn pciedev_crwlpciegen2_182(pcie2: *mut bcma_drv_pcie2) {
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDADDR, PCIE2_SBMBX);
    pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDDATA, 1 << 0);
}

unsafe fn pciedev_reg_pm_clk_period(pcie2: *mut bcma_drv_pcie2) {
    let drv_cc: *mut bcma_drv_cc = &mut (*(*(*pcie2).core).bus).drv_cc;
    let core_rev: u8 = (*(*pcie2).core).id.rev;
    let alp_khz: u32;
    let pm_value: u32;

    if core_rev <= 13 {
        alp_khz = bcma_pmu_get_alp_clock(drv_cc) / 1000;
        pm_value = (1000000 * 2) / alp_khz;
        pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDADDR, PCIE2_PVT_REG_PM_CLK_PERIOD);
        pcie2_write32(pcie2, BCMA_CORE_PCIE2_CONFIGINDDATA, pm_value);
    }
}

pub unsafe fn bcma_core_pcie2_init(pcie2: *mut bcma_drv_pcie2) {
    let bus: *mut bcma_bus = (*(*pcie2).core).bus;
    let ci: *mut bcma_chipinfo = &mut (*bus).chipinfo;
    let tmp = pcie2_read32(pcie2, BCMA_CORE_PCIE2_SPROM(54));

    if ((tmp & 0xe) >> 1) == 2 {
        bcma_core_pcie2_cfg_write(pcie2, 0x4e0, 0x17);
    }

    (*pcie2).reqsize = match (*bus).chipinfo.id {
        BCMA_CHIP_ID_BCM4360 | BCMA_CHIP_ID_BCM4352 => 1024,
        _ => 128,
    };

    if (*ci).id == BCMA_CHIP_ID_BCM4360 && (*ci).rev > 3 {
        bcma_core_pcie2_war_delay_perst_enab(pcie2, true);
    }
    bcma_core_pcie2_hw_ltr_war(pcie2);
    pciedev_crwlpciegen2(pcie2);
    pciedev_reg_pm_clk_period(pcie2);
    pciedev_crwlpciegen2_180(pcie2);
    pciedev_crwlpciegen2_182(pcie2);
}

pub unsafe fn bcma_core_pcie2_up(pcie2: *mut bcma_drv_pcie2) {
    let bus: *mut bcma_bus = (*(*pcie2).core).bus;
    let dev: *mut pci_dev = (*bus).host_pci;
    let err = pcie_set_readrq(dev, (*pcie2).reqsize);

    if err != 0 {
        bcma_err(bus, "Error setting PCI_EXP_DEVCTL_READRQ: %d\n", err);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
