// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DaVinci DA850 AHCI SATA platform driver
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const DRV_NAME: &str = "ahci_da850";
const HARDRESET_RETRIES: i32 = 5;

/* SATA PHY Control Register offset from AHCI base */
const SATA_P0PHYCR_REG: usize = 0x178;

#[inline]
const fn sata_phy_mpy(x: u32) -> u32 { x << 0 }
#[inline]
const fn sata_phy_los(x: u32) -> u32 { x << 6 }
#[inline]
const fn sata_phy_rx_cdr(x: u32) -> u32 { x << 10 }
#[inline]
const fn sata_phy_rxeq(x: u32) -> u32 { x << 13 }
#[inline]
const fn sata_phy_txswing(x: u32) -> u32 { x << 19 }
#[inline]
const fn sata_phy_enpll(x: u32) -> u32 { x << 31 }

unsafe fn da850_sata_init(dev: *mut device, pwrdn_reg: *mut core::ffi::c_void,
                          ahci_base: *mut core::ffi::c_void, mpy: u32) {
    let mut val: u32;

    /* Enable SATA clock receiver */
    val = readl(pwrdn_reg);
    val &= !BIT(0);
    writel(val, pwrdn_reg);

    val = sata_phy_mpy(mpy) | sata_phy_los(1) | sata_phy_rx_cdr(4)
        | sata_phy_rxeq(1) | sata_phy_txswing(3) | sata_phy_enpll(1);

    writel(val, (ahci_base as *mut u8).add(SATA_P0PHYCR_REG) as *mut core::ffi::c_void);
}

unsafe fn ahci_da850_calculate_mpy(refclk_rate: u64) -> u32 {
    let pll_output: u32 = 1500000000;
    let needed: u32;

    /*
     * We need to determine the value of the multiplier (MPY) bits.
     * In order to include the 12.5 multiplier we need to first divide
     * the refclk rate by ten.
     *
     * __div64_32() turned out to be unreliable, sometimes returning
     * false results.
     */
    WARN((refclk_rate % 10) != 0, "refclk must be divisible by 10");
    needed = (pll_output as u64 / (refclk_rate / 10)) as u32;

    /*
     * What we have now is (multiplier * 10).
     *
     * Let's determine the actual register value we need to write.
     */
    match needed {
        50 => 0x1,
        60 => 0x2,
        80 => 0x4,
        100 => 0x5,
        120 => 0x6,
        125 => 0x7,
        150 => 0x8,
        200 => 0x9,
        250 => 0xa,
        _ => {
            /* We should have divided evenly - if not, return an invalid value. */
            0
        }
    }
}

unsafe fn ahci_da850_softreset(link: *mut ata_link, class: *mut u32,
                                deadline: u64) -> i32 {
    let pmp = sata_srst_pmp(link);
    /*
     * There's an issue with the SATA controller on da850 SoCs: if we
     * enable Port Multiplier support, but the drive is connected directly
     * to the board, it can't be detected. As a workaround: if PMP is
     * enabled, we first call ahci_do_softreset() and pass it the result of
     * sata_srst_pmp(). If this call fails, we retry with pmp = 0.
     */
    let ret = ahci_do_softreset(link, class, pmp, deadline, ahci_check_ready);
    if pmp != 0 && ret == -EBUSY {
        return ahci_do_softreset(link, class, 0, deadline, ahci_check_ready);
    }
    ret
}

unsafe fn ahci_da850_hardreset(link: *mut ata_link, class: *mut u32,
                                deadline: u64) -> i32 {
    let mut retry = HARDRESET_RETRIES;
    let mut ret: i32;
    let mut online: bool;
    /* In order to correctly service the LCD controller of the da850 SoC,
     * we increased the PLL0 frequency to 456MHz from the default 300MHz.
     * This made the SATA controller unstable and the hardreset operation
     * does not always succeed the first time. Before really giving up to
     * bring up the link, retry the reset a couple times.
     */
    loop {
        (ret, online) = ahci_do_hardreset(link, class, deadline);
        if online { return ret; }
        if retry == 0 { break; }
        retry -= 1;
    }
    ret
}

static mut ahci_da850_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ahci_platform_ops,
    reset_softreset: Some(ahci_da850_softreset),
    /* No need to override .pmp_softreset - it's only used for actual PMP-enabled ports. */
    reset_hardreset: Some(ahci_da850_hardreset),
    pmp_reset_hardreset: Some(ahci_da850_hardreset),
};

static ahci_da850_port_info: ata_port_info = ata_port_info {
    flags: AHCI_FLAG_COMMON, pio_mask: ATA_PIO4, udma_mask: ATA_UDMA6,
    port_ops: unsafe { &ahci_da850_port_ops },
};

static ahci_platform_sht: scsi_host_template = AHCI_SHT(DRV_NAME);

unsafe fn ahci_da850_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let hpriv = ahci_platform_get_resources(pdev, 0);
    if IS_ERR(hpriv) { return PTR_ERR(hpriv); }

    /*
     * Internally ahci_platform_get_resources() calls the bulk clocks
     * get method or falls back to using a single clk_get_optional().
     * This AHCI SATA controller uses two clocks: functional clock
     * with "fck" connection id and external reference clock with "refclk" id.
     * If we haven't got all of them re-try the clocks getting procedure
     * with the explicitly specified ids.
     */
    if (*hpriv).n_clks < 2 {
        (*hpriv).clks = devm_kcalloc(dev, 2, core::mem::size_of::<clk_bulk>(), GFP_KERNEL);
        if (*hpriv).clks.is_null() { return -ENOMEM; }
        (*hpriv).clks.add(0).id = "fck";
        (*hpriv).clks.add(1).id = "refclk";
        (*hpriv).n_clks = 2;
        let rc = devm_clk_bulk_get(dev, (*hpriv).n_clks, (*hpriv).clks);
        if rc != 0 { return rc; }
    }

    let mpy = ahci_da850_calculate_mpy(clk_get_rate((*hpriv).clks.add(1).clk));
    if mpy == 0 {
        dev_err(dev, "invalid REFCLK multiplier value: 0x%x", mpy);
        return -EINVAL;
    }
    let pwrdn_reg = devm_platform_ioremap_resource(pdev, 1);
    if IS_ERR(pwrdn_reg) { return PTR_ERR(pwrdn_reg); }
    let rc = ahci_platform_enable_resources(hpriv);
    if rc != 0 { return rc; }
    da850_sata_init(dev, pwrdn_reg, (*hpriv).mmio, mpy);
    let rc = ahci_platform_init_host(pdev, hpriv, &ahci_da850_port_info, &ahci_platform_sht);
    if rc == 0 { return 0; }
    ahci_platform_disable_resources(hpriv);
    rc
}

static ahci_da850_pm_ops: dev_pm_ops = SIMPLE_DEV_PM_OPS!(ahci_platform_suspend,
                                                            ahci_platform_resume);

static ahci_da850_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "ti,da850-ahci" },
    of_device_id { /* sentinel */ ..unsafe { core::mem::zeroed() } },
];

static mut ahci_da850_driver: platform_driver = platform_driver {
    probe: Some(ahci_da850_probe),
    remove: Some(ata_platform_remove_one),
    driver: device_driver {
        name: DRV_NAME,
        of_match_table: ahci_da850_of_match.as_ptr(),
        pm: &ahci_da850_pm_ops,
    },
};

// MODULE_DEVICE_TABLE(of, ahci_da850_of_match);
// module_platform_driver(ahci_da850_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
