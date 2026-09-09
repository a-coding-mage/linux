// SPDX-License-Identifier: GPL-2.0-only
/* Faraday Technology FTIDE010 driver; translated from pata_ftide010.c. */

// Kernel dependencies supplied by the surrounding translation unit.

const DRV_NAME: &str = "pata_ftide010";

#[repr(C)]
struct Ftide010 {
    dev: *mut device,
    base: *mut core::ffi::c_void,
    pclk: *mut clk,
    host: *mut ata_host,
    master_cbl: u32,
    slave_cbl: u32,
    sg: *mut sata_gemini,
    master_to_sata0: bool,
    slave_to_sata0: bool,
    master_to_sata1: bool,
    slave_to_sata1: bool,
}

const FTIDE010_DMA_REG: usize = 0x00;
const FTIDE010_DMA_STATUS: usize = 0x02;
const FTIDE010_IDE_BMDTPR: usize = 0x04;
const FTIDE010_IDE_DEVICE_ID: usize = 0x08;
const FTIDE010_PIO_TIMING: usize = 0x10;
const FTIDE010_MWDMA_TIMING: usize = 0x11;
const FTIDE010_UDMA_TIMING0: usize = 0x12;
const FTIDE010_UDMA_TIMING1: usize = 0x13;
const FTIDE010_CLK_MOD: usize = 0x14;
const FTIDE010_CMD_DATA: usize = 0x20;
const FTIDE010_ERROR_FEATURES: usize = 0x21;
const FTIDE010_NSECT: usize = 0x22;
const FTIDE010_LBAL: usize = 0x23;
const FTIDE010_LBAM: usize = 0x24;
const FTIDE010_LBAH: usize = 0x25;
const FTIDE010_DEVICE: usize = 0x26;
const FTIDE010_STATUS_COMMAND: usize = 0x27;
const FTIDE010_ALTSTAT_CTRL: usize = 0x36;
const FTIDE010_UDMA_TIMING_MODE_56: u8 = 1 << 7;
const FTIDE010_CLK_MOD_DEV0_CLK_SEL: u8 = 1 << 0;
const FTIDE010_CLK_MOD_DEV1_CLK_SEL: u8 = 1 << 1;
const FTIDE010_CLK_MOD_DEV0_UDMA_EN: u8 = 1 << 4;
const FTIDE010_CLK_MOD_DEV1_UDMA_EN: u8 = 1 << 5;

static PIO_ACTIVE_TIME: [u8; 5] = [10, 10, 10, 3, 3];
static PIO_RECOVERY_TIME: [u8; 5] = [10, 3, 1, 3, 1];
static MWDMA_50_ACTIVE_TIME: [u8; 3] = [6, 2, 2];
static MWDMA_50_RECOVERY_TIME: [u8; 3] = [6, 2, 1];
static MWDMA_66_ACTIVE_TIME: [u8; 3] = [8, 3, 3];
static MWDMA_66_RECOVERY_TIME: [u8; 3] = [8, 2, 1];
static UDMA_50_SETUP_TIME: [u8; 6] = [3, 3, 2, 2, 1, 9];
static UDMA_50_HOLD_TIME: [u8; 6] = [3, 1, 1, 1, 1, 1];
static UDMA_66_SETUP_TIME: [u8; 7] = [4, 4, 3, 2, 1, 9, 9];
static UDMA_66_HOLD_TIME: [u8; 7] = [4, 2, 1, 1, 1, 1, 1];
static SET_MDMA_66_MHZ: [bool; 4] = [true, true, true, true];
static SET_UDMA_66_MHZ: [bool; 7] = [false, false, false, true, true, false, true];

unsafe fn ftide010_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    let ftide = (*(*ap).host).private_data as *mut Ftide010;
    let speed = (*adev).dma_mode as u8;
    let devno = ((*adev).devno & 1) as usize;
    let (udma_en_mask, f66m_en_mask) = if devno == 0 {
        (FTIDE010_CLK_MOD_DEV0_UDMA_EN, FTIDE010_CLK_MOD_DEV0_CLK_SEL)
    } else {
        (FTIDE010_CLK_MOD_DEV1_UDMA_EN, FTIDE010_CLK_MOD_DEV1_CLK_SEL)
    };
    let clk_addr = ((*ftide).base as *mut u8).add(FTIDE010_CLK_MOD);
    let mut clkreg = readb(clk_addr);
    clkreg &= !udma_en_mask;
    clkreg &= !f66m_en_mask;
    let mut timreg: u8;
    if speed & XFER_UDMA_0 != 0 {
        let i = (speed & !XFER_UDMA_0) as usize;
        dev_dbg((*ftide).dev, "set UDMA mode %02x, index %d\n", speed, i);
        clkreg |= udma_en_mask;
        if SET_UDMA_66_MHZ[i] {
            clkreg |= f66m_en_mask;
            timreg = (UDMA_66_SETUP_TIME[i] << 4) | UDMA_66_HOLD_TIME[i];
        } else { timreg = (UDMA_50_SETUP_TIME[i] << 4) | UDMA_50_HOLD_TIME[i]; }
        if i >= 5 { timreg |= FTIDE010_UDMA_TIMING_MODE_56; }
        dev_dbg((*ftide).dev, "UDMA write clkreg = %02x, timreg = %02x\n", clkreg, timreg);
        writeb(clkreg, clk_addr);
        writeb(timreg, ((*ftide).base as *mut u8).add(FTIDE010_UDMA_TIMING0 + devno));
    } else {
        let i = (speed & !XFER_MW_DMA_0) as usize;
        dev_dbg((*ftide).dev, "set MWDMA mode %02x, index %d\n", speed, i);
        if SET_MDMA_66_MHZ[i] {
            clkreg |= f66m_en_mask;
            timreg = (MWDMA_66_ACTIVE_TIME[i] << 4) | MWDMA_66_RECOVERY_TIME[i];
        } else { timreg = (MWDMA_50_ACTIVE_TIME[i] << 4) | MWDMA_50_RECOVERY_TIME[i]; }
        dev_dbg((*ftide).dev, "MWDMA write clkreg = %02x, timreg = %02x\n", clkreg, timreg);
        writeb(clkreg, clk_addr);
        writeb(timreg, ((*ftide).base as *mut u8).add(FTIDE010_MWDMA_TIMING));
    }
    (*ap).private_data = adev as *mut core::ffi::c_void;
}

unsafe fn ftide010_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    let ftide = (*(*ap).host).private_data as *mut Ftide010;
    let pio = ((*adev).pio_mode - XFER_PIO_0) as usize;
    dev_dbg((*ftide).dev, "set PIO mode %02x, index %d\n", (*adev).pio_mode, pio);
    writeb((PIO_ACTIVE_TIME[pio] << 4) | PIO_RECOVERY_TIME[pio], ((*ftide).base as *mut u8).add(FTIDE010_PIO_TIMING));
}

unsafe fn ftide010_qc_issue(qc: *mut ata_queued_cmd) -> u32 {
    let ap = (*qc).ap;
    let adev = (*qc).dev;
    if adev != (*ap).private_data as *mut ata_device && ata_dma_enabled(adev) { ftide010_set_dmamode(ap, adev); }
    ata_bmdma_qc_issue(qc)
}

static mut PATA_FTIDE010_PORT_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    set_dmamode: Some(ftide010_set_dmamode),
    set_piomode: Some(ftide010_set_piomode),
    qc_issue: Some(ftide010_qc_issue),
    ..ata_port_operations::DEFAULT
};

static mut PATA_FTIDE010_PORT_INFO: ata_port_info = ata_port_info {
    flags: ATA_FLAG_SLAVE_POSS,
    mwdma_mask: ATA_MWDMA2,
    udma_mask: ATA_UDMA6,
    pio_mask: ATA_PIO4,
    port_ops: &PATA_FTIDE010_PORT_OPS,
    ..ata_port_info::DEFAULT
};

unsafe fn pata_ftide010_gemini_port_start(ap: *mut ata_port) -> i32 {
    let ftide = (*(*ap).host).private_data as *mut Ftide010;
    let ret = ata_bmdma_port_start(ap);
    if ret != 0 { return ret; }
    let mut bridges = 0;
    for (enabled, bridge) in [((*ftide).master_to_sata0, 0), ((*ftide).master_to_sata1, 1),
                               ((*ftide).slave_to_sata0 && !(*ftide).master_to_sata0, 0),
                               ((*ftide).slave_to_sata1 && !(*ftide).master_to_sata1, 1)] {
        if enabled && gemini_sata_start_bridge((*ftide).sg, bridge) == 0 { bridges += 1; }
    }
    dev_info((*ftide).dev, "brought %d bridges online\n", bridges);
    if bridges > 0 { 0 } else { -EINVAL }
}

unsafe fn pata_ftide010_gemini_port_stop(ap: *mut ata_port) {
    let ftide = (*(*ap).host).private_data as *mut Ftide010;
    for (enabled, bridge) in [((*ftide).master_to_sata0, 0), ((*ftide).master_to_sata1, 1),
                               ((*ftide).slave_to_sata0 && !(*ftide).master_to_sata0, 0),
                               ((*ftide).slave_to_sata1 && !(*ftide).master_to_sata1, 1)] {
        if enabled { gemini_sata_stop_bridge((*ftide).sg, bridge); }
    }
}

unsafe fn pata_ftide010_gemini_cable_detect(ap: *mut ata_port) -> i32 {
    let ftide = (*(*ap).host).private_data as *mut Ftide010;
    (*ftide).master_cbl as i32
}

unsafe fn pata_ftide010_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let ftide = devm_kzalloc(dev, core::mem::size_of::<Ftide010>(), GFP_KERNEL) as *mut Ftide010;
    if ftide.is_null() { return -ENOMEM; }
    (*ftide).dev = dev;
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    (*ftide).base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if IS_ERR((*ftide).base) { return PTR_ERR((*ftide).base); }
    (*ftide).pclk = devm_clk_get(dev, "PCLK\0".as_ptr() as *const i8);
    if !IS_ERR((*ftide).pclk) && clk_prepare_enable((*ftide).pclk) != 0 { return -EINVAL; }
    (*ftide).master_cbl = ATA_CBL_PATA40;
    (*ftide).slave_cbl = ATA_CBL_PATA40;
    let ppi = [&PATA_FTIDE010_PORT_INFO as *const ata_port_info, core::ptr::null()];
    (*ftide).host = ata_host_alloc_pinfo(dev, ppi.as_ptr(), 1);
    if (*ftide).host.is_null() { return -ENOMEM; }
    (*(*ftide).host).private_data = ftide as *mut core::ffi::c_void;
    ata_host_activate((*ftide).host, irq, ata_bmdma_interrupt, 0, core::ptr::null())
}

unsafe fn pata_ftide010_remove(pdev: *mut platform_device) {
    let host = platform_get_drvdata(pdev) as *mut ata_host;
    let ftide = (*host).private_data as *mut Ftide010;
    ata_host_detach((*ftide).host);
    clk_disable_unprepare((*ftide).pclk);
}

// CONFIG_SATA_GEMINI supplies the Gemini bridge initialization and mux mapping.
// MODULE_DEVICE_TABLE, module_platform_driver, and MODULE_* declarations are
// retained as build-system metadata by the surrounding kernel translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
