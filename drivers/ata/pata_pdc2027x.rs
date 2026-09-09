// SPDX-License-Identifier: GPL-2.0-or-later
/* Promise PATA TX2/TX4/TX2000/133 IDE driver for pdc20268 to pdc20277. */
/* This is a source-level translation; Linux kernel dependencies are external. */

const DRV_NAME: &str = "pata_pdc2027x";
const DRV_VERSION: &str = "1.0";

const PDC_MMIO_BAR: usize = 5;
const PDC_UDMA_100: u32 = 0;
const PDC_UDMA_133: u32 = 1;
const PDC_100_MHZ: i64 = 100000000;
const PDC_133_MHZ: i64 = 133333333;
const PDC_SYS_CTL: u32 = 0x1100;
const PDC_ATA_CTL: u32 = 0x1104;
const PDC_GLOBAL_CTL: u32 = 0x1108;
const PDC_CTCR0: u32 = 0x110c;
const PDC_CTCR1: u32 = 0x1110;
const PDC_BYTE_COUNT: u32 = 0x1120;
const PDC_PLL_CTL: u32 = 0x1202;

#[repr(C)] struct Pdc2027xPioTiming { value0: u8, value1: u8, value2: u8 }
#[repr(C)] struct Pdc2027xMdmaTiming { value0: u8, value1: u8 }
#[repr(C)] struct Pdc2027xUdmaTiming { value0: u8, value1: u8, value2: u8 }

static PDC2027X_PIO_TIMING_TBL: [Pdc2027xPioTiming; 5] = [
    Pdc2027xPioTiming { value0: 0xfb, value1: 0x2b, value2: 0xac },
    Pdc2027xPioTiming { value0: 0x46, value1: 0x29, value2: 0xa4 },
    Pdc2027xPioTiming { value0: 0x23, value1: 0x26, value2: 0x64 },
    Pdc2027xPioTiming { value0: 0x27, value1: 0x0d, value2: 0x35 },
    Pdc2027xPioTiming { value0: 0x23, value1: 0x09, value2: 0x25 },
];
static PDC2027X_MDMA_TIMING_TBL: [Pdc2027xMdmaTiming; 3] = [
    Pdc2027xMdmaTiming { value0: 0xdf, value1: 0x5f },
    Pdc2027xMdmaTiming { value0: 0x6b, value1: 0x27 },
    Pdc2027xMdmaTiming { value0: 0x69, value1: 0x25 },
];
static PDC2027X_UDMA_TIMING_TBL: [Pdc2027xUdmaTiming; 7] = [
    Pdc2027xUdmaTiming { value0: 0x4a, value1: 0x0f, value2: 0xd5 },
    Pdc2027xUdmaTiming { value0: 0x3a, value1: 0x0a, value2: 0xd0 },
    Pdc2027xUdmaTiming { value0: 0x2a, value1: 0x07, value2: 0xcd },
    Pdc2027xUdmaTiming { value0: 0x1a, value1: 0x05, value2: 0xcd },
    Pdc2027xUdmaTiming { value0: 0x1a, value1: 0x03, value2: 0xcd },
    Pdc2027xUdmaTiming { value0: 0x1a, value1: 0x02, value2: 0xcb },
    Pdc2027xUdmaTiming { value0: 0x1a, value1: 0x01, value2: 0xcb },
];

// Kernel declarations and structures are supplied by the surrounding translation.
extern "C" {
    fn ioread8(p: *mut core::ffi::c_void) -> u8;
    fn ioread16(p: *mut core::ffi::c_void) -> u16;
    fn ioread32(p: *mut core::ffi::c_void) -> u32;
    fn iowrite16(v: u16, p: *mut core::ffi::c_void);
    fn iowrite32(v: u32, p: *mut core::ffi::c_void);
    fn msleep(ms: u32);
}

#[inline] unsafe fn port_mmio(ap: *mut ata_port, offset: u32) -> *mut core::ffi::c_void {
    (*(*ap).host).iomap[PDC_MMIO_BAR].add((*ap).port_no as usize * 0x100 + offset as usize)
}
#[inline] unsafe fn dev_mmio(ap: *mut ata_port, adev: *mut ata_device, offset: u32) -> *mut core::ffi::c_void {
    port_mmio(ap, offset).add(if (*adev).devno != 0 { 8 } else { 0 })
}
unsafe fn pdc2027x_cable_detect(ap: *mut ata_port) -> i32 {
    let cgcr = ioread32(port_mmio(ap, PDC_GLOBAL_CTL));
    if cgcr & (1 << 26) != 0 { return ATA_CBL_PATA40; }
    ATA_CBL_PATA80
}
unsafe fn pdc2027x_port_enabled(ap: *mut ata_port) -> i32 { (ioread8(port_mmio(ap, PDC_ATA_CTL)) & 2) as i32 }
unsafe fn pdc2027x_prereset(link: *mut ata_link, deadline: usize) -> i32 {
    if pdc2027x_port_enabled((*link).ap) == 0 { return -2; }
    ata_sff_prereset(link, deadline)
}
unsafe fn pdc2027x_mode_filter(adev: *mut ata_device, mut mask: u32) -> u32 {
    let pair = ata_dev_pair(adev);
    if (*adev).class != ATA_DEV_ATA || (*adev).devno == 0 || pair.is_null() { return mask; }
    let mut model = [0u8; ATA_ID_PROD_LEN + 1];
    ata_id_c_string((*adev).id, model.as_mut_ptr(), ATA_ID_PROD, (ATA_ID_PROD_LEN + 1) as i32);
    if !strstr(model.as_ptr(), b"Maxtor\0".as_ptr()).is_null() && (*pair).dma_mode == XFER_UDMA_6 { mask &= !(1 << (6 + ATA_SHIFT_UDMA)); }
    mask
}
unsafe fn pdc2027x_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let pio = ((*adev).pio_mode - XFER_PIO_0) as usize;
    if pio > 4 { return; }
    let t = PDC2027X_PIO_TIMING_TBL[pio];
    let mut c0 = ioread32(dev_mmio(ap, adev, PDC_CTCR0));
    c0 = (c0 & 0xffff0000) | t.value0 as u32 | ((t.value1 as u32) << 8);
    iowrite32(c0, dev_mmio(ap, adev, PDC_CTCR0));
    let mut c1 = ioread32(dev_mmio(ap, adev, PDC_CTCR1));
    c1 = (c1 & 0x00ffffff) | ((t.value2 as u32) << 24);
    iowrite32(c1, dev_mmio(ap, adev, PDC_CTCR1));
}
unsafe fn pdc2027x_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let mode = (*adev).dma_mode;
    if mode >= XFER_UDMA_0 && mode <= XFER_UDMA_6 {
        let t = PDC2027X_UDMA_TIMING_TBL[(mode & 7) as usize];
        let mut c = ioread32(dev_mmio(ap, adev, PDC_CTCR1));
        c = (c & 0xff000000) | t.value0 as u32 | ((t.value1 as u32)<<8) | ((t.value2 as u32)<<16);
        iowrite32(c, dev_mmio(ap, adev, PDC_CTCR1));
    } else if mode >= XFER_MW_DMA_0 && mode <= XFER_MW_DMA_2 {
        let t = PDC2027X_MDMA_TIMING_TBL[(mode & 7) as usize];
        let mut c = ioread32(dev_mmio(ap, adev, PDC_CTCR0));
        c = (c & 0x0000ffff) | ((t.value0 as u32)<<16) | ((t.value1 as u32)<<24);
        iowrite32(c, dev_mmio(ap, adev, PDC_CTCR0));
    }
}
unsafe fn pdc2027x_check_atapi_dma(qc: *mut ata_queued_cmd) -> i32 {
    let op = (*(*qc).scsicmd).cmnd[0];
    match op { READ_10|WRITE_10|READ_12|WRITE_12|READ_6|WRITE_6|0xad|0xbe => 0, _ => 1 }
}

unsafe fn pdc2027x_set_mode(link: *mut ata_link, failed: *mut *mut ata_device) -> i32 {
    let rc = ata_set_mode(link, failed); if rc < 0 { return rc; }
    let ap = (*link).ap;
    ata_for_each_dev(link, |dev| { pdc2027x_set_piomode(ap, dev); if (*dev).xfer_shift != ATA_SHIFT_PIO { pdc2027x_set_dmamode(ap, dev); } });
    0
}

unsafe fn pdc_read_counter(host: *mut ata_host) -> i64 {
    let base = (*host).iomap[PDC_MMIO_BAR]; let mut retry = 1;
    loop {
        let lo = ioread32(base.add(PDC_BYTE_COUNT as usize)) & 0x7fff;
        let hi = ioread32(base.add(PDC_BYTE_COUNT as usize + 0x100)) & 0x7fff;
        let lov = ioread32(base.add(PDC_BYTE_COUNT as usize)) & 0x7fff;
        let hiv = ioread32(base.add(PDC_BYTE_COUNT as usize + 0x100)) & 0x7fff;
        if retry != 0 && !(hi == hiv && lo >= lov) { retry -= 1; continue; }
        return ((hi << 15) | lo) as i64;
    }
}
unsafe fn pdc_adjust_pll(host: *mut ata_host, pll_clock: i64, board_idx: u32) {
    let base = (*host).iomap[PDC_MMIO_BAR]; let khz = pll_clock / 1000;
    if khz < 5000 || khz > 70000 { return; }
    let required = if board_idx != 0 { PDC_133_MHZ } else { PDC_100_MHZ };
    let ratio = required / khz; let r = if ratio < 8600 { 0x0d } else if ratio < 12900 { 8 } else if ratio < 16100 { 6 } else if ratio < 64000 { 0 } else { return };
    let f = (ratio * (r + 2)) / 1000 - 2; if f < 0 || f > 127 { return; }
    iowrite16(((r << 8) | f) as u16, base.add(PDC_PLL_CTL as usize)); ioread16(base.add(PDC_PLL_CTL as usize)); msleep(30);
    ioread16(base.add(PDC_PLL_CTL as usize));
}
unsafe fn pdc_detect_pll_input_clock(host: *mut ata_host) -> i64 {
    let base = (*host).iomap[PDC_MMIO_BAR]; let scr = ioread32(base.add(PDC_SYS_CTL as usize));
    iowrite32(scr | (1 << 14), base.add(PDC_SYS_CTL as usize)); ioread32(base.add(PDC_SYS_CTL as usize));
    let start = pdc_read_counter(host); let start_time = ktime_get(); msleep(100); let end = pdc_read_counter(host); let end_time = ktime_get();
    iowrite32(scr & !(1 << 14), base.add(PDC_SYS_CTL as usize)); ioread32(base.add(PDC_SYS_CTL as usize));
    let elapsed = ktime_us_delta(end_time, start_time) as i64;
    (((start - end) & 0x3fffffff) / 100) * (100000000 / elapsed)
}
unsafe fn pdc_hardware_init(host: *mut ata_host, board_idx: u32) { let clock = pdc_detect_pll_input_clock(host); pdc_adjust_pll(host, clock, board_idx); }

unsafe fn pdc2027x_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    let cmd_offset = [0x17c0usize, 0x15c0]; let bmdma_offset = [0x1000usize, 0x1008];
    let board_idx = (*ent).driver_data as usize;
    let ppi = [&PDC2027X_PORT_INFO[board_idx] as *const ata_port_info, core::ptr::null()];
    let host = ata_host_alloc_pinfo(&mut (*pdev).dev, ppi.as_ptr(), 2); if host.is_null() { return -12; }
    let mut rc = pcim_enable_device(pdev); if rc != 0 { return rc; }
    rc = pcim_iomap_regions(pdev, 1 << PDC_MMIO_BAR, DRV_NAME.as_ptr() as *const i8); if rc != 0 { return rc; }
    (*host).iomap = pcim_iomap_table(pdev); rc = dma_set_mask_and_coherent(&mut (*pdev).dev, ATA_DMA_MASK); if rc != 0 { return rc; }
    let base = (*host).iomap[PDC_MMIO_BAR];
    for i in 0..2 { let ap = (*host).ports[i]; pdc_ata_setup_port(&mut (*ap).ioaddr, base.add(cmd_offset[i])); (*ap).ioaddr.bmdma_addr = base.add(bmdma_offset[i]); }
    pdc_hardware_init(host, board_idx as u32); pci_set_master(pdev);
    ata_host_activate(host, (*pdev).irq, ata_bmdma_interrupt, IRQF_SHARED, &PDC2027X_SHT)
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn pdc2027x_reinit_one(pdev: *mut pci_dev) -> i32 {
    let host = pci_get_drvdata(pdev); let rc = ata_pci_device_do_resume(pdev); if rc != 0 { return rc; }
    let board = if (*pdev).device == PCI_DEVICE_ID_PROMISE_20268 || (*pdev).device == PCI_DEVICE_ID_PROMISE_20270 { PDC_UDMA_100 } else { PDC_UDMA_133 };
    pdc_hardware_init(host, board); ata_host_resume(host); 0
}

// The PCI tables, operation structures, module metadata, and kernel callback macros
// are represented by the surrounding kernel bindings, preserving their original names.

// Remaining hardware setup routines retain the original kernel-facing ABI and operations.
unsafe fn pdc_ata_setup_port(port: *mut ata_ioports, base: *mut core::ffi::c_void) {
    (*port).cmd_addr = base; (*port).data_addr = base; (*port).feature_addr = base.add(5);
    (*port).error_addr = base.add(5); (*port).nsect_addr = base.add(10); (*port).lbal_addr = base.add(15);
    (*port).lbam_addr = base.add(16); (*port).lbah_addr = base.add(21); (*port).device_addr = base.add(26);
    (*port).command_addr = base.add(31); (*port).status_addr = base.add(31); (*port).altstatus_addr = base.add(0x81a);
    (*port).ctl_addr = base.add(0x81a);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
