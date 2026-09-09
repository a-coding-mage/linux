// SPDX-License-Identifier: GPL-2.0-only
/*
 * ixp4xx PATA/Compact Flash driver
 * Copyright (C) 2006-07 Tower Technologies
 * Author: Alessandro Zummo <a.zummo@towertech.it>
 *
 * An ATA driver to handle a Compact Flash connected
 * to the ixp4xx expansion bus in TrueIDE mode. The CF
 * must have it chip selects connected to two CS lines
 * on the ixp4xx. In the irq is not available, you might
 * want to modify both this driver and libata to run in
 * polling mode.
 */

const DRV_NAME: &str = "pata_ixp4xx_cf";
const DRV_VERSION: &str = "1.0";

#[repr(C)]
struct ixp4xx_pata {
    host: *mut ata_host,
    rmap: *mut regmap,
    cmd_csreg: u32,
    cmd: *mut core::ffi::c_void,
    ctl: *mut core::ffi::c_void,
}

const IXP4XX_EXP_TIMING_STRIDE: u32 = 0x04;
/* The timings for the chipselect is in bits 29..16 */
const IXP4XX_EXP_T1_T5_MASK: u32 = 0x3fff0000;
const IXP4XX_EXP_PIO_0_8: u32 = 0x0a470000;
const IXP4XX_EXP_PIO_1_8: u32 = 0x06430000;
const IXP4XX_EXP_PIO_2_8: u32 = 0x02410000;
const IXP4XX_EXP_PIO_3_8: u32 = 0x00820000;
const IXP4XX_EXP_PIO_4_8: u32 = 0x00400000;
const IXP4XX_EXP_PIO_0_16: u32 = 0x29640000;
const IXP4XX_EXP_PIO_1_16: u32 = 0x05030000;
const IXP4XX_EXP_PIO_2_16: u32 = 0x00b20000;
const IXP4XX_EXP_PIO_3_16: u32 = 0x00820000;
const IXP4XX_EXP_PIO_4_16: u32 = 0x00400000;
const IXP4XX_EXP_BW_MASK: u32 = (1 << 6) | (1 << 0);
const IXP4XX_EXP_BYTE_RD16: u32 = 1 << 6; /* Byte reads on half-word devices */
const IXP4XX_EXP_BYTE_EN: u32 = 1 << 0; /* Use 8bit data bus if set */

unsafe fn ixp4xx_set_8bit_timing(ixpp: *mut ixp4xx_pata, pio_mode: u8) {
    let timing = match pio_mode {
        XFER_PIO_0 => IXP4XX_EXP_PIO_0_8,
        XFER_PIO_1 => IXP4XX_EXP_PIO_1_8,
        XFER_PIO_2 => IXP4XX_EXP_PIO_2_8,
        XFER_PIO_3 => IXP4XX_EXP_PIO_3_8,
        XFER_PIO_4 => IXP4XX_EXP_PIO_4_8,
        _ => 0,
    };
    if timing != 0 { regmap_update_bits((*ixpp).rmap, (*ixpp).cmd_csreg, IXP4XX_EXP_T1_T5_MASK, timing); }
    regmap_update_bits((*ixpp).rmap, (*ixpp).cmd_csreg, IXP4XX_EXP_BW_MASK,
                       IXP4XX_EXP_BYTE_RD16 | IXP4XX_EXP_BYTE_EN);
}

unsafe fn ixp4xx_set_16bit_timing(ixpp: *mut ixp4xx_pata, pio_mode: u8) {
    let timing = match pio_mode {
        XFER_PIO_0 => IXP4XX_EXP_PIO_0_16,
        XFER_PIO_1 => IXP4XX_EXP_PIO_1_16,
        XFER_PIO_2 => IXP4XX_EXP_PIO_2_16,
        XFER_PIO_3 => IXP4XX_EXP_PIO_3_16,
        XFER_PIO_4 => IXP4XX_EXP_PIO_4_16,
        _ => 0,
    };
    if timing != 0 { regmap_update_bits((*ixpp).rmap, (*ixpp).cmd_csreg, IXP4XX_EXP_T1_T5_MASK, timing); }
    regmap_update_bits((*ixpp).rmap, (*ixpp).cmd_csreg, IXP4XX_EXP_BW_MASK, IXP4XX_EXP_BYTE_RD16);
}

/* This sets up the timing on the chipselect CMD accordingly */
unsafe fn ixp4xx_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let ixpp = (*(*ap).host).private_data as *mut ixp4xx_pata;
    ata_dev_info(adev, "configured for PIO%d 8bit\n", (*adev).pio_mode - XFER_PIO_0);
    ixp4xx_set_8bit_timing(ixpp, (*adev).pio_mode);
}

unsafe fn ixp4xx_mmio_data_xfer(qc: *mut ata_queued_cmd, buf: *mut u8, buflen: u32, rw: i32) -> u32 {
    let mut words = buflen >> 1;
    let buf16 = buf as *mut u16;
    let adev = (*qc).dev;
    let ap = (*(*qc).dev).link.ap;
    let mmio = (*ap).ioaddr.data_addr;
    let ixpp = (*(*ap).host).private_data as *mut ixp4xx_pata;
    let mut flags: unsigned_long = 0;
    ata_dev_dbg(adev, if rw == READ { "READ" } else { "WRITE" }, buflen);
    spin_lock_irqsave((*ap).lock, &mut flags);
    ixp4xx_set_16bit_timing(ixpp, (*adev).pio_mode);
    udelay(5);
    for i in 0..words {
        if rw == READ { *buf16.add(i as usize) = readw(mmio); }
        else { writew(*buf16.add(i as usize), mmio); }
    }
    if (buflen & 0x01) != 0 {
        let mut align_buf: u16 = 0;
        let trailing_buf = buf.add((buflen - 1) as usize);
        if rw == READ { align_buf = readw(mmio); core::ptr::copy_nonoverlapping(&align_buf as *const u16 as *const u8, trailing_buf, 1); }
        else { core::ptr::copy_nonoverlapping(trailing_buf, &mut align_buf as *mut u16 as *mut u8, 1); writew(align_buf, mmio); }
        words += 1;
    }
    ixp4xx_set_8bit_timing(ixpp, (*adev).pio_mode);
    udelay(5);
    spin_unlock_irqrestore((*ap).lock, flags);
    words << 1
}

static mut ixp4xx_sht: scsi_host_template = scsi_host_template { /* ATA_PIO_SHT(DRV_NAME) */ };
static mut ixp4xx_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    sff_data_xfer: Some(ixp4xx_mmio_data_xfer),
    cable_detect: Some(ata_cable_40wire),
    set_piomode: Some(ixp4xx_set_piomode),
};
static mut ixp4xx_port_info: ata_port_info = ata_port_info {
    flags: ATA_FLAG_NO_ATAPI, pio_mask: ATA_PIO4, port_ops: &ixp4xx_port_ops,
};

unsafe fn ixp4xx_setup_port(ap: *mut ata_port, ixpp: *mut ixp4xx_pata, mut raw_cmd: unsigned_long, mut raw_ctl: unsigned_long) {
    let ioaddr = &mut (*ap).ioaddr;
    raw_ctl += 0x06;
    ioaddr.cmd_addr = (*ixpp).cmd;
    ioaddr.altstatus_addr = (*ixpp).ctl.add(6);
    ioaddr.ctl_addr = (*ixpp).ctl.add(6);
    ata_sff_std_ports(ioaddr);
    /* Build-time CONFIG_CPU_BIG_ENDIAN condition is preserved here. */
    if !cfg!(target_endian = "big") {
        *( &mut ioaddr.data_addr as *mut _ as *mut unsigned_long) ^= 0x02;
        for p in [&mut ioaddr.cmd_addr, &mut ioaddr.altstatus_addr, &mut ioaddr.ctl_addr, &mut ioaddr.error_addr, &mut ioaddr.feature_addr, &mut ioaddr.nsect_addr, &mut ioaddr.lbal_addr, &mut ioaddr.lbam_addr, &mut ioaddr.lbah_addr, &mut ioaddr.device_addr, &mut ioaddr.status_addr, &mut ioaddr.command_addr] { *(p as *mut _ as *mut unsigned_long) ^= 0x03; }
        raw_cmd ^= 0x03; raw_ctl ^= 0x03;
    }
    ata_port_desc(ap, "cmd 0x%lx ctl 0x%lx", raw_cmd, raw_ctl);
}

/* External kernel declarations and module registration are supplied by the surrounding translation unit. */
unsafe fn ixp4xx_pata_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = dev.of_node;
    let ixpp = devm_kzalloc(dev, core::mem::size_of::<ixp4xx_pata>(), GFP_KERNEL) as *mut ixp4xx_pata;
    if ixpp.is_null() { return -ENOMEM; }
    (*ixpp).rmap = syscon_node_to_regmap((*np).parent);
    if IS_ERR((*ixpp).rmap) { return dev_err_probe(dev, PTR_ERR((*ixpp).rmap), "no regmap\n"); }
    let mut csindex: u32 = 0;
    let ret = of_property_read_u32_index(np, "reg", 0, &mut csindex);
    if ret != 0 { return dev_err_probe(dev, ret, "can't inspect CMD address\n"); }
    dev_info(dev, "using CS%d for PIO timing configuration\n", csindex);
    (*ixpp).cmd_csreg = csindex * IXP4XX_EXP_TIMING_STRIDE;
    let ppi = [&ixp4xx_port_info as *const ata_port_info, core::ptr::null()];
    (*ixpp).host = ata_host_alloc_pinfo(dev, ppi.as_ptr(), 1);
    if (*ixpp).host.is_null() { return -ENOMEM; }
    (*(*ixpp).host).private_data = ixpp as *mut core::ffi::c_void;
    let ret = dma_set_coherent_mask(dev, DMA_BIT_MASK(32));
    if ret != 0 { return ret; }
    let mut cmd: *mut resource = core::ptr::null_mut();
    (*ixpp).cmd = devm_platform_get_and_ioremap_resource(pdev, 0, &mut cmd);
    if IS_ERR((*ixpp).cmd) { return PTR_ERR((*ixpp).cmd); }
    let mut ctl: *mut resource = core::ptr::null_mut();
    (*ixpp).ctl = devm_platform_get_and_ioremap_resource(pdev, 1, &mut ctl);
    if IS_ERR((*ixpp).ctl) { return PTR_ERR((*ixpp).ctl); }
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    irq_set_irq_type(irq, IRQ_TYPE_EDGE_RISING);
    ixp4xx_setup_port((*ixpp).host.ports[0], ixpp, (*cmd).start, (*ctl).start);
    ata_print_version_once(dev, DRV_VERSION);
    ata_host_activate((*ixpp).host, irq, Some(ata_sff_interrupt), 0, &ixp4xx_sht)
}

static ixp4xx_pata_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "intel,ixp4xx-compact-flash" }, of_device_id { sentinel: true },
];
static mut ixp4xx_pata_platform_driver: platform_driver = platform_driver { name: DRV_NAME, of_match_table: &ixp4xx_pata_of_match, probe: ixp4xx_pata_probe, remove: ata_platform_remove_one };
// module_platform_driver(ixp4xx_pata_platform_driver);
// MODULE_AUTHOR("Alessandro Zummo <a.zummo@towertech.it>");
// MODULE_DESCRIPTION("low-level driver for ixp4xx Compact Flash PATA");
// MODULE_LICENSE("GPL");
// MODULE_VERSION(DRV_VERSION);
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
