// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DaVinci DM816 AHCI SATA platform driver
 *
 * Copyright (C) 2017 BayLibre SAS
 */

// Linux kernel and local AHCI dependencies are supplied by other translation units.

const AHCI_DM816_DRV_NAME: &str = "ahci-dm816";

#[inline]
const fn ahci_dm816_phy_enpll(x: u32) -> u32 { x << 0 }
#[inline]
const fn ahci_dm816_phy_mpy(x: u32) -> u32 { x << 1 }
#[inline]
const fn ahci_dm816_phy_los(x: u32) -> u32 { x << 12 }
#[inline]
const fn ahci_dm816_phy_rx_cdr(x: u32) -> u32 { x << 13 }
#[inline]
const fn ahci_dm816_phy_rxeq(x: u32) -> u32 { x << 16 }
#[inline]
const fn ahci_dm816_phy_txswing(x: u32) -> u32 { x << 23 }

const AHCI_DM816_P0PHYCR_REG: usize = 0x178;
const AHCI_DM816_P1PHYCR_REG: usize = 0x1f8;
const AHCI_DM816_PLL_OUT: u64 = 1_500_000_000;

static PLL_MPY_TABLE: [u64; 14] = [
    400, 500, 600, 800, 825, 1000, 1200,
    1250, 1500, 1600, 1650, 2000, 2200, 2500,
];

unsafe fn ahci_dm816_get_mpy_bits(refclk_rate: u64) -> i32 {
    let pll_multiplier = AHCI_DM816_PLL_OUT / (refclk_rate / 100);
    for i in 0..PLL_MPY_TABLE.len() {
        if PLL_MPY_TABLE[i] == pll_multiplier {
            return i as i32;
        }
    }
    -1
}

unsafe fn ahci_dm816_phy_init(
    hpriv: *mut ahci_host_priv,
    dev: *mut device,
) -> i32 {
    let refclk_rate: u64;
    let mpy: i32;
    let mut val: u32;

    if (*hpriv).n_clks < 2 {
        dev_err(dev, "reference clock not supplied\n");
        return -EINVAL;
    }

    refclk_rate = clk_get_rate((*hpriv).clks[1].clk);
    if refclk_rate % 100 != 0 {
        dev_err(dev, "reference clock rate must be divisible by 100\n");
        return -EINVAL;
    }

    mpy = ahci_dm816_get_mpy_bits(refclk_rate);
    if mpy < 0 {
        dev_err(dev, "can't calculate the MPY bits value\n");
        return -EINVAL;
    }

    // Enable the PHY and configure the first HBA port.
    val = ahci_dm816_phy_mpy(mpy as u32)
        | ahci_dm816_phy_los(1)
        | ahci_dm816_phy_rx_cdr(4)
        | ahci_dm816_phy_rxeq(1)
        | ahci_dm816_phy_txswing(3)
        | ahci_dm816_phy_enpll(1);
    writel(val, (*hpriv).mmio.add(AHCI_DM816_P0PHYCR_REG));

    // Configure the second HBA port.
    val = ahci_dm816_phy_los(1)
        | ahci_dm816_phy_rx_cdr(4)
        | ahci_dm816_phy_rxeq(1)
        | ahci_dm816_phy_txswing(3);
    writel(val, (*hpriv).mmio.add(AHCI_DM816_P1PHYCR_REG));

    0
}

unsafe fn ahci_dm816_softreset(
    link: *mut ata_link,
    class: *mut u32,
    deadline: u64,
) -> i32 {
    let pmp = sata_srst_pmp(link);
    let ret = ahci_do_softreset(link, class, pmp, deadline, ahci_check_ready);
    if pmp != 0 && ret == -EBUSY {
        return ahci_do_softreset(link, class, 0, deadline, ahci_check_ready);
    }
    ret
}

static mut AHCI_DM816_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: &ahci_platform_ops,
    reset: ata_port_reset_operations { softreset: Some(ahci_dm816_softreset) },
};

static AHCI_DM816_PORT_INFO: ata_port_info = ata_port_info {
    flags: AHCI_FLAG_COMMON,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: &AHCI_DM816_PORT_OPS,
};

static AHCI_DM816_PLATFORM_SHT: scsi_host_template = AHCI_SHT!(AHCI_DM816_DRV_NAME);

unsafe fn ahci_dm816_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let hpriv = ahci_platform_get_resources(pdev, 0);
    if IS_ERR(hpriv) { return PTR_ERR(hpriv); }

    let mut rc = ahci_platform_enable_resources(hpriv);
    if rc != 0 { return rc; }

    rc = ahci_dm816_phy_init(hpriv, dev);
    if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }

    rc = ahci_platform_init_host(
        pdev, hpriv, &AHCI_DM816_PORT_INFO, &AHCI_DM816_PLATFORM_SHT,
    );
    if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }
    0
}

static AHCI_DM816_PM_OPS: dev_pm_ops = SIMPLE_DEV_PM_OPS!(
    ahci_platform_suspend,
    ahci_platform_resume,
);

static AHCI_DM816_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "ti,dm816-ahci" },
    of_device_id { compatible: "" },
];

static mut AHCI_DM816_DRIVER: platform_driver = platform_driver {
    probe: Some(ahci_dm816_probe),
    remove: Some(ata_platform_remove_one),
    driver: device_driver {
        name: AHCI_DM816_DRV_NAME,
        of_match_table: &AHCI_DM816_OF_MATCH,
        pm: &AHCI_DM816_PM_OPS,
    },
};

module_platform_driver!(AHCI_DM816_DRIVER);
module_description!("DaVinci DM816 AHCI SATA platform driver");
module_author!("Bartosz Golaszewski <bgolaszewski@baylibre.com>");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
