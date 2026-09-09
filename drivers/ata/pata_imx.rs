/*
 * Freescale iMX PATA driver
 *
 * Copyright (C) 2011 Arnaud Patard <arnaud.patard@rtp-net.org>
 *
 * Based on pata_platform - Copyright (C) 2006 - 2007  Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * TODO:
 * - dmaengine support
 */

// External Linux kernel declarations are supplied by the surrounding build.

const DRV_NAME: &str = "pata_imx";

const PATA_IMX_ATA_TIME_OFF: u32 = 0x00;
const PATA_IMX_ATA_TIME_ON: u32 = 0x01;
const PATA_IMX_ATA_TIME_1: u32 = 0x02;
const PATA_IMX_ATA_TIME_2W: u32 = 0x03;
const PATA_IMX_ATA_TIME_2R: u32 = 0x04;
const PATA_IMX_ATA_TIME_AX: u32 = 0x05;
const PATA_IMX_ATA_TIME_PIO_RDX: u32 = 0x06;
const PATA_IMX_ATA_TIME_4: u32 = 0x07;
const PATA_IMX_ATA_TIME_9: u32 = 0x08;

const PATA_IMX_ATA_CONTROL: u32 = 0x24;
const PATA_IMX_ATA_CTRL_FIFO_RST_B: u32 = 1 << 7;
const PATA_IMX_ATA_CTRL_ATA_RST_B: u32 = 1 << 6;
const PATA_IMX_ATA_CTRL_IORDY_EN: u32 = 1 << 0;
const PATA_IMX_ATA_INT_EN: u32 = 0x2c;
const PATA_IMX_ATA_INTR_ATA_INTRQ2: u32 = 1 << 3;
const PATA_IMX_DRIVE_DATA: u32 = 0xa0;
const PATA_IMX_DRIVE_CONTROL: u32 = 0xd8;

static mut pio_t4: [u32; 5] = [30, 20, 15, 10, 10];
static mut pio_t9: [u32; 5] = [20, 15, 10, 10, 10];
static mut pio_tA: [u32; 5] = [35, 35, 35, 35, 35];

#[repr(C)]
struct pata_imx_priv {
    clk: *mut clk,
    /* timings/interrupt/control regs */
    host_regs: *mut core::ffi::c_void,
    ata_ctl: u32,
}

unsafe fn pata_imx_set_timing(adev: *mut ata_device, priv_: *mut pata_imx_priv) {
    let mut timing: ata_timing = core::mem::zeroed();
    let clkrate: usize = clk_get_rate((*priv_).clk);

    if (*adev).pio_mode < XFER_PIO_0 || (*adev).pio_mode > XFER_PIO_4 || clkrate == 0 {
        return;
    }

    let t: usize = 1_000_000_000usize / clkrate;
    ata_timing_compute(adev, (*adev).pio_mode, &mut timing, (t * 1000) as u32, 0);
    let mode: usize = ((*adev).pio_mode - XFER_PIO_0) as usize;

    writeb(3, (*priv_).host_regs.add(PATA_IMX_ATA_TIME_OFF as usize));
    writeb(3, (*priv_).host_regs.add(PATA_IMX_ATA_TIME_ON as usize));
    writeb(timing.setup, (*priv_).host_regs.add(PATA_IMX_ATA_TIME_1 as usize));
    writeb(timing.act8b, (*priv_).host_regs.add(PATA_IMX_ATA_TIME_2W as usize));
    writeb(timing.act8b, (*priv_).host_regs.add(PATA_IMX_ATA_TIME_2R as usize));
    writeb(1, (*priv_).host_regs.add(PATA_IMX_ATA_TIME_PIO_RDX as usize));
    writeb(pio_t4[mode] / t as u32 + 1, (*priv_).host_regs.add(PATA_IMX_ATA_TIME_4 as usize));
    writeb(pio_t9[mode] / t as u32 + 1, (*priv_).host_regs.add(PATA_IMX_ATA_TIME_9 as usize));
    writeb(pio_tA[mode] / t as u32 + 1, (*priv_).host_regs.add(PATA_IMX_ATA_TIME_AX as usize));
}

unsafe extern "C" fn pata_imx_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let priv_: *mut pata_imx_priv = (*(*ap).host).private_data as *mut pata_imx_priv;
    pata_imx_set_timing(adev, priv_);
    let mut val = __raw_readl((*priv_).host_regs.add(PATA_IMX_ATA_CONTROL as usize));
    if ata_pio_need_iordy(adev) { val |= PATA_IMX_ATA_CTRL_IORDY_EN; }
    else { val &= !PATA_IMX_ATA_CTRL_IORDY_EN; }
    __raw_writel(val, (*priv_).host_regs.add(PATA_IMX_ATA_CONTROL as usize));
}

static pata_imx_sht: scsi_host_template = ATA_PIO_SHT!(DRV_NAME);

static mut pata_imx_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    sff_data_xfer: Some(ata_sff_data_xfer32),
    cable_detect: Some(ata_cable_unknown),
    set_piomode: Some(pata_imx_set_piomode),
};

unsafe fn pata_imx_setup_port(ioaddr: *mut ata_ioports) {
    (*ioaddr).data_addr = (*ioaddr).cmd_addr.add((ATA_REG_DATA << 2) as usize);
    (*ioaddr).error_addr = (*ioaddr).cmd_addr.add((ATA_REG_ERR << 2) as usize);
    (*ioaddr).feature_addr = (*ioaddr).cmd_addr.add((ATA_REG_FEATURE << 2) as usize);
    (*ioaddr).nsect_addr = (*ioaddr).cmd_addr.add((ATA_REG_NSECT << 2) as usize);
    (*ioaddr).lbal_addr = (*ioaddr).cmd_addr.add((ATA_REG_LBAL << 2) as usize);
    (*ioaddr).lbam_addr = (*ioaddr).cmd_addr.add((ATA_REG_LBAM << 2) as usize);
    (*ioaddr).lbah_addr = (*ioaddr).cmd_addr.add((ATA_REG_LBAH << 2) as usize);
    (*ioaddr).device_addr = (*ioaddr).cmd_addr.add((ATA_REG_DEVICE << 2) as usize);
    (*ioaddr).status_addr = (*ioaddr).cmd_addr.add((ATA_REG_STATUS << 2) as usize);
    (*ioaddr).command_addr = (*ioaddr).cmd_addr.add((ATA_REG_CMD << 2) as usize);
}

unsafe extern "C" fn pata_imx_probe(pdev: *mut platform_device) -> i32 {
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    let priv_: *mut pata_imx_priv = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<pata_imx_priv>(), GFP_KERNEL) as *mut _;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).clk = devm_clk_get_enabled(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR((*priv_).clk) { dev_err(&mut (*pdev).dev, "Failed to get and enable clock\n"); return PTR_ERR((*priv_).clk); }
    let host = ata_host_alloc(&mut (*pdev).dev, 1);
    if host.is_null() { return -ENOMEM; }
    (*host).private_data = priv_ as *mut _;
    let ap = (*host).ports[0];
    (*ap).ops = &pata_imx_port_ops;
    (*ap).pio_mask = ATA_PIO4;
    (*ap).flags |= ATA_FLAG_SLAVE_POSS;
    let mut io_res: *mut resource = core::ptr::null_mut();
    (*priv_).host_regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut io_res);
    if IS_ERR((*priv_).host_regs) { return PTR_ERR((*priv_).host_regs); }
    (*ap).ioaddr.cmd_addr = (*priv_).host_regs.add(PATA_IMX_DRIVE_DATA as usize);
    (*ap).ioaddr.ctl_addr = (*priv_).host_regs.add(PATA_IMX_DRIVE_CONTROL as usize);
    (*ap).ioaddr.altstatus_addr = (*ap).ioaddr.ctl_addr;
    pata_imx_setup_port(&mut (*ap).ioaddr);
    ata_port_desc(ap, "cmd 0x%llx ctl 0x%llx", (*io_res).start as u64 + PATA_IMX_DRIVE_DATA as u64, (*io_res).start as u64 + PATA_IMX_DRIVE_CONTROL as u64);
    __raw_writel(PATA_IMX_ATA_CTRL_FIFO_RST_B | PATA_IMX_ATA_CTRL_ATA_RST_B, (*priv_).host_regs.add(PATA_IMX_ATA_CONTROL as usize));
    __raw_writel(PATA_IMX_ATA_INTR_ATA_INTRQ2, (*priv_).host_regs.add(PATA_IMX_ATA_INT_EN as usize));
    let ret = ata_host_activate(host, irq, Some(ata_sff_interrupt), 0, &pata_imx_sht);
    if ret != 0 { return ret; }
    0
}

unsafe extern "C" fn pata_imx_remove(pdev: *mut platform_device) {
    let host = platform_get_drvdata(pdev);
    let priv_ = (*host).private_data as *mut pata_imx_priv;
    ata_host_detach(host);
    __raw_writel(0, (*priv_).host_regs.add(PATA_IMX_ATA_INT_EN as usize));
}

// CONFIG_PM_SLEEP conditional is preserved; these declarations are available when enabled.
#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn pata_imx_suspend(dev: *mut device) -> i32 {
    let host = dev_get_drvdata(dev);
    let priv_ = (*host).private_data as *mut pata_imx_priv;
    ata_host_suspend(host, PMSG_SUSPEND);
    __raw_writel(0, (*priv_).host_regs.add(PATA_IMX_ATA_INT_EN as usize));
    (*priv_).ata_ctl = __raw_readl((*priv_).host_regs.add(PATA_IMX_ATA_CONTROL as usize));
    clk_disable_unprepare((*priv_).clk);
    0
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe extern "C" fn pata_imx_resume(dev: *mut device) -> i32 {
    let host = dev_get_drvdata(dev);
    let priv_ = (*host).private_data as *mut pata_imx_priv;
    let ret = clk_prepare_enable((*priv_).clk);
    if ret != 0 { return ret; }
    __raw_writel((*priv_).ata_ctl, (*priv_).host_regs.add(PATA_IMX_ATA_CONTROL as usize));
    __raw_writel(PATA_IMX_ATA_INTR_ATA_INTRQ2, (*priv_).host_regs.add(PATA_IMX_ATA_INT_EN as usize));
    ata_host_resume(host);
    0
}

static imx_pata_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: "fsl,imx27-pata" },
    of_device_id { compatible: core::option::Option::None },
];

static mut pata_imx_driver: platform_driver = platform_driver {
    probe: Some(pata_imx_probe),
    remove: Some(pata_imx_remove),
    driver: driver { name: DRV_NAME, of_match_table: imx_pata_dt_ids.as_ptr(), pm: &pata_imx_pm_ops },
};

// module_platform_driver(pata_imx_driver);
// MODULE_DEVICE_TABLE(of, imx_pata_dt_ids);
// MODULE_AUTHOR("Arnaud Patard <arnaud.patard@rtp-net.org>");
// MODULE_DESCRIPTION("low-level driver for iMX PATA");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
