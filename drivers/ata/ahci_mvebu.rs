/*
 * AHCI glue platform driver for Marvell EBU SOCs
 *
 * Copyright (C) 2014 Marvell
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 * Marcin Wojtas <mw@semihalf.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// C dependencies: linux/ahci_platform.h, linux/kernel.h, linux/mbus.h,
// linux/module.h, linux/of.h, linux/platform_device.h, and ahci.h.

const DRV_NAME: &str = "ahci-mvebu";

const AHCI_VENDOR_SPECIFIC_0_ADDR: usize = 0xa0;
const AHCI_VENDOR_SPECIFIC_0_DATA: usize = 0xa4;

#[inline]
const fn AHCI_WINDOW_CTRL(win: usize) -> usize { 0x60 + (win << 4) }
#[inline]
const fn AHCI_WINDOW_BASE(win: usize) -> usize { 0x64 + (win << 4) }
#[inline]
const fn AHCI_WINDOW_SIZE(win: usize) -> usize { 0x68 + (win << 4) }

#[repr(C)]
struct ahci_mvebu_plat_data {
    plat_config: Option<unsafe extern "C" fn(*mut ahci_host_priv) -> i32>,
    flags: u32,
}

unsafe fn ahci_mvebu_mbus_config(
    hpriv: *mut ahci_host_priv,
    dram: *const mbus_dram_target_info,
) {
    for i in 0..4usize {
        writel(0, (*hpriv).mmio.add(AHCI_WINDOW_CTRL(i)));
        writel(0, (*hpriv).mmio.add(AHCI_WINDOW_BASE(i)));
        writel(0, (*hpriv).mmio.add(AHCI_WINDOW_SIZE(i)));
    }

    for i in 0..(*dram).num_cs as usize {
        let cs = (*dram).cs.add(i);
        writel(
            ((*cs).mbus_attr << 8) | ((*dram).mbus_dram_target_id << 4) | 1,
            (*hpriv).mmio.add(AHCI_WINDOW_CTRL(i)),
        );
        writel((*cs).base >> 16, (*hpriv).mmio.add(AHCI_WINDOW_BASE(i)));
        writel(((*cs).size.wrapping_sub(1)) & 0xffff0000,
               (*hpriv).mmio.add(AHCI_WINDOW_SIZE(i)));
    }
}

unsafe fn ahci_mvebu_regret_option(hpriv: *mut ahci_host_priv) {
    /* Enable the regret bit to allow the SATA unit to regret a request that
     * didn't receive an acknowlegde and avoid a deadlock. */
    writel(0x4, (*hpriv).mmio.add(AHCI_VENDOR_SPECIFIC_0_ADDR));
    writel(0x80, (*hpriv).mmio.add(AHCI_VENDOR_SPECIFIC_0_DATA));
}

unsafe extern "C" fn ahci_mvebu_armada_380_config(hpriv: *mut ahci_host_priv) -> i32 {
    let dram = mv_mbus_dram_info();
    let mut rc = 0;
    if !dram.is_null() { ahci_mvebu_mbus_config(hpriv, dram); } else { rc = -ENODEV; }
    ahci_mvebu_regret_option(hpriv);
    rc
}

unsafe extern "C" fn ahci_mvebu_armada_3700_config(hpriv: *mut ahci_host_priv) -> i32 {
    writel(0, (*hpriv).mmio.add(AHCI_VENDOR_SPECIFIC_0_ADDR));
    let mut reg = readl((*hpriv).mmio.add(AHCI_VENDOR_SPECIFIC_0_DATA));
    reg |= BIT(6);
    writel(reg, (*hpriv).mmio.add(AHCI_VENDOR_SPECIFIC_0_DATA));
    0
}

/* Errata Ref#226 workaround: preserve PxFBS while stopping the engine. */
unsafe extern "C" fn ahci_mvebu_stop_engine(ap: *mut ata_port) -> i32 {
    let port_mmio = ahci_port_base(ap);
    let mut tmp = readl(port_mmio.add(PORT_CMD));
    if (tmp & (PORT_CMD_START | PORT_CMD_LIST_ON)) == 0 { return 0; }
    let port_fbs = readl(port_mmio.add(PORT_FBS));
    tmp &= !PORT_CMD_START;
    writel(tmp, port_mmio.add(PORT_CMD));
    writel(port_fbs, port_mmio.add(PORT_FBS));
    tmp = ata_wait_register(ap, port_mmio.add(PORT_CMD), PORT_CMD_LIST_ON,
                            PORT_CMD_LIST_ON, 1, 500);
    if (tmp & PORT_CMD_LIST_ON) != 0 { return -EIO; }
    0
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn ahci_mvebu_suspend(pdev: *mut platform_device, _state: pm_message_t) -> i32 {
    ahci_platform_suspend_host(&mut (*pdev).dev)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn ahci_mvebu_resume(pdev: *mut platform_device) -> i32 {
    let host = platform_get_drvdata(pdev);
    let hpriv = (*host).private_data;
    let pdata = (*hpriv).plat_data as *const ahci_mvebu_plat_data;
    ((*pdata).plat_config.unwrap())(hpriv);
    ahci_platform_resume_host(&mut (*pdev).dev)
}

#[cfg(not(CONFIG_PM_SLEEP))]
const ahci_mvebu_suspend: Option<unsafe extern "C" fn(*mut platform_device, pm_message_t) -> i32> = None;
#[cfg(not(CONFIG_PM_SLEEP))]
const ahci_mvebu_resume: Option<unsafe extern "C" fn(*mut platform_device) -> i32> = None;

static ahci_mvebu_armada_380_plat_data: ahci_mvebu_plat_data = ahci_mvebu_plat_data {
    plat_config: Some(ahci_mvebu_armada_380_config), flags: 0,
};
static ahci_mvebu_armada_3700_plat_data: ahci_mvebu_plat_data = ahci_mvebu_plat_data {
    plat_config: Some(ahci_mvebu_armada_3700_config), flags: AHCI_HFLAG_SUSPEND_PHYS,
};

unsafe extern "C" fn ahci_mvebu_probe(pdev: *mut platform_device) -> i32 {
    let pdata = of_device_get_match_data(&(*pdev).dev) as *const ahci_mvebu_plat_data;
    if pdata.is_null() { return -EINVAL; }
    let hpriv = ahci_platform_get_resources(pdev, 0);
    if IS_ERR(hpriv) { return PTR_ERR(hpriv); }
    (*hpriv).flags |= (*pdata).flags;
    (*hpriv).plat_data = pdata as *mut _ as *mut core::ffi::c_void;
    let mut rc = ahci_platform_enable_resources(hpriv);
    if rc != 0 { return rc; }
    (*hpriv).stop_engine = Some(ahci_mvebu_stop_engine);
    rc = ((*pdata).plat_config.unwrap())(hpriv);
    if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }
    rc = ahci_platform_init_host(pdev, hpriv, &ahci_mvebu_port_info, &ahci_platform_sht);
    if rc != 0 { ahci_platform_disable_resources(hpriv); }
    rc
}

static ahci_mvebu_port_info: ata_port_info = ata_port_info {
    flags: AHCI_FLAG_COMMON,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: &ahci_platform_ops,
};

static ahci_platform_sht: scsi_host_template = AHCI_SHT(DRV_NAME);

static ahci_mvebu_of_match: [of_device_id; 3] = [
    of_device_id { compatible: "marvell,armada-380-ahci", data: &ahci_mvebu_armada_380_plat_data },
    of_device_id { compatible: "marvell,armada-3700-ahci", data: &ahci_mvebu_armada_3700_plat_data },
    of_device_id { compatible: "", data: core::ptr::null() }, // sentinel
];

static mut ahci_mvebu_driver: platform_driver = platform_driver {
    probe: Some(ahci_mvebu_probe),
    remove: Some(ata_platform_remove_one),
    suspend: ahci_mvebu_suspend,
    resume: ahci_mvebu_resume,
    driver: driver {
        name: DRV_NAME,
        of_match_table: ahci_mvebu_of_match.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, ahci_mvebu_of_match);
// module_platform_driver(ahci_mvebu_driver);
// MODULE_DESCRIPTION("Marvell EBU AHCI SATA driver");
// MODULE_AUTHOR("Thomas Petazzoni <thomas.petazzoni@free-electrons.com>, Marcin Wojtas <mw@semihalf.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:ahci_mvebu");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
