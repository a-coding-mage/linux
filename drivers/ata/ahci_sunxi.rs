// SPDX-License-Identifier: GPL-2.0-only
/*
 * Allwinner sunxi AHCI SATA platform driver
 * Copyright 2013 Olliver Schinagl <oliver@schinagl.nl>
 * Copyright 2014 Hans de Goede <hdegoede@redhat.com>
 *
 * based on the AHCI SATA platform driver by Jeff Garzik and Anton Vorontsov
 * Based on code from Allwinner Technology Co., Ltd. <www.allwinnertech.com>,
 * Daniel Wang <danielwang@allwinnertech.com>
 */

// C dependencies supplied by the surrounding kernel translation.

const DRV_NAME: &str = "ahci-sunxi";

// Insmod parameters
static mut enable_pmp: bool = false;

const AHCI_BISTAFR: usize = 0x00a0;
const AHCI_BISTCR: usize = 0x00a4;
const AHCI_BISTFCTR: usize = 0x00a8;
const AHCI_BISTSR: usize = 0x00ac;
const AHCI_BISTDECR: usize = 0x00b0;
const AHCI_DIAGNR0: usize = 0x00b4;
const AHCI_DIAGNR1: usize = 0x00b8;
const AHCI_OOBR: usize = 0x00bc;
const AHCI_PHYCS0R: usize = 0x00c0;
const AHCI_PHYCS1R: usize = 0x00c4;
const AHCI_PHYCS2R: usize = 0x00c8;
const AHCI_TIMER1MS: usize = 0x00e0;
const AHCI_GPARAM1R: usize = 0x00e8;
const AHCI_GPARAM2R: usize = 0x00ec;
const AHCI_PPARAMR: usize = 0x00f0;
const AHCI_TESTR: usize = 0x00f4;
const AHCI_VERSIONR: usize = 0x00f8;
const AHCI_IDR: usize = 0x00fc;
const AHCI_RWCR: usize = 0x00fc;
const AHCI_P0DMACR: usize = 0x0170;
const AHCI_P0PHYCR: usize = 0x0178;
const AHCI_P0PHYSR: usize = 0x017c;

unsafe fn sunxi_clrbits(reg: *mut core::ffi::c_void, clr_val: u32) {
    let mut reg_val: u32 = readl(reg);
    reg_val &= !clr_val;
    writel(reg_val, reg);
}

unsafe fn sunxi_setbits(reg: *mut core::ffi::c_void, set_val: u32) {
    let mut reg_val: u32 = readl(reg);
    reg_val |= set_val;
    writel(reg_val, reg);
}

unsafe fn sunxi_clrsetbits(reg: *mut core::ffi::c_void, clr_val: u32, set_val: u32) {
    let mut reg_val: u32 = readl(reg);
    reg_val &= !clr_val;
    reg_val |= set_val;
    writel(reg_val, reg);
}

unsafe fn sunxi_getbits(reg: *mut core::ffi::c_void, mask: u8, shift: u8) -> u32 {
    (readl(reg) >> shift) & mask as u32
}

unsafe fn ahci_sunxi_phy_init(dev: *mut device, reg_base: *mut core::ffi::c_void) -> i32 {
    let mut reg_val: u32;
    let mut timeout: i32;

    // This magic is from the original code
    writel(0, reg_base.add(AHCI_RWCR));
    msleep(5);

    sunxi_setbits(reg_base.add(AHCI_PHYCS1R), 1 << 19);
    sunxi_clrsetbits(reg_base.add(AHCI_PHYCS0R), 0x7 << 24, (0x5 << 24) | (1 << 23) | (1 << 18));
    sunxi_clrsetbits(reg_base.add(AHCI_PHYCS1R), (0x3 << 16) | (0x1f << 8) | (0x3 << 6), (0x2 << 16) | (0x6 << 8) | (0x2 << 6));
    sunxi_setbits(reg_base.add(AHCI_PHYCS1R), (1 << 28) | (1 << 15));
    sunxi_clrbits(reg_base.add(AHCI_PHYCS1R), 1 << 19);
    sunxi_clrsetbits(reg_base.add(AHCI_PHYCS0R), 0x7 << 20, 0x3 << 20);
    sunxi_clrsetbits(reg_base.add(AHCI_PHYCS2R), 0x1f << 5, 0x19 << 5);
    msleep(5);

    sunxi_setbits(reg_base.add(AHCI_PHYCS0R), 1 << 19);

    timeout = 250; // Power up takes aprox 50 us
    loop {
        reg_val = sunxi_getbits(reg_base.add(AHCI_PHYCS0R), 0x7, 28);
        if reg_val == 0x02 { break; }
        timeout -= 1;
        if timeout == 0 {
            dev_err(dev, "PHY power up failed.\n");
            return -EIO;
        }
        udelay(1);
    }

    sunxi_setbits(reg_base.add(AHCI_PHYCS2R), 1 << 24);
    timeout = 100; // Calibration takes aprox 10 us
    loop {
        reg_val = sunxi_getbits(reg_base.add(AHCI_PHYCS2R), 0x1, 24);
        if reg_val == 0x00 { break; }
        timeout -= 1;
        if timeout == 0 {
            dev_err(dev, "PHY calibration failed.\n");
            return -EIO;
        }
        udelay(1);
    }

    msleep(15);
    writel(0x7, reg_base.add(AHCI_RWCR));
    0
}

unsafe fn ahci_sunxi_start_engine(ap: *mut ata_port) {
    let port_mmio = ahci_port_base(ap);
    let hpriv = (*(*ap).host).private_data;
    sunxi_clrsetbits((*hpriv).mmio.add(AHCI_P0DMACR), 0x0000ffff, 0x00004433);
    sunxi_setbits(port_mmio.add(PORT_CMD), PORT_CMD_START);
}

static ahci_sunxi_port_info: ata_port_info = ata_port_info {
    flags: AHCI_FLAG_COMMON | ATA_FLAG_NCQ | ATA_FLAG_NO_DIPM,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: &ahci_platform_ops,
};

static ahci_platform_sht: scsi_host_template = AHCI_SHT(DRV_NAME);

unsafe fn ahci_sunxi_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let hpriv = ahci_platform_get_resources(pdev, AHCI_PLATFORM_GET_RESETS);
    if IS_ERR(hpriv) { return PTR_ERR(hpriv); }
    (*hpriv).start_engine = Some(ahci_sunxi_start_engine);

    let mut rc = ahci_platform_enable_resources(hpriv);
    if rc != 0 { return rc; }
    rc = ahci_sunxi_phy_init(dev, (*hpriv).mmio);
    if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }

    (*hpriv).flags = AHCI_HFLAG_32BIT_ONLY | AHCI_HFLAG_NO_MSI | AHCI_HFLAG_YES_NCQ;
    if !enable_pmp { (*hpriv).flags |= AHCI_HFLAG_NO_PMP; }
    rc = ahci_platform_init_host(pdev, hpriv, &ahci_sunxi_port_info, &ahci_platform_sht);
    if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }
    0
}

// CONFIG_PM_SLEEP conditionally supplies the resume implementation.
unsafe fn ahci_sunxi_resume(dev: *mut device) -> i32 {
    let host = dev_get_drvdata(dev) as *mut ata_host;
    let hpriv = (*host).private_data;
    let mut rc = ahci_platform_enable_resources(hpriv);
    if rc != 0 { return rc; }
    rc = ahci_sunxi_phy_init(dev, (*hpriv).mmio);
    if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }
    rc = ahci_platform_resume_host(dev);
    if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }
    0
}

static ahci_sunxi_pm_ops: dev_pm_ops = SIMPLE_DEV_PM_OPS!(ahci_platform_suspend, ahci_sunxi_resume);

static ahci_sunxi_of_match: [of_device_id; 3] = [
    of_device_id { compatible: "allwinner,sun4i-a10-ahci" },
    of_device_id { compatible: "allwinner,sun8i-r40-ahci" },
    of_device_id { compatible: core::ptr::null() },
];

static mut ahci_sunxi_driver: platform_driver = platform_driver {
    probe: Some(ahci_sunxi_probe),
    remove: Some(ata_platform_remove_one),
    driver: driver {
        name: DRV_NAME,
        of_match_table: ahci_sunxi_of_match.as_ptr(),
        pm: &ahci_sunxi_pm_ops,
    },
};

module_platform_driver!(ahci_sunxi_driver);
// MODULE_DESCRIPTION("Allwinner sunxi AHCI SATA driver");
// MODULE_AUTHOR("Olliver Schinagl <oliver@schinagl.nl>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
