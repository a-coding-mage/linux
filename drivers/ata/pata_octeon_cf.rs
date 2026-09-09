/*
 * Driver for the Octeon bootbus compact flash.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005 - 2012 Cavium Inc.
 * Copyright (C) 2008 Wind River Systems
 */

/* Linux/kernel, libata, OF, platform, SCSI, trace, byteorder and Octeon
 * declarations are supplied by the surrounding kernel bindings. */

const DRV_NAME: &str = "pata_octeon_cf";
const DRV_VERSION: &str = "2.2";
const OCTEON_CF_BUSY_POLL_INTERVAL: u64 = 500000;
const DMA_CFG: u64 = 0;
const DMA_TIM: u64 = 0x20;
const DMA_INT: u64 = 0x38;
const DMA_INT_EN: u64 = 0x50;

#[repr(C)]
struct OcteonCfPort {
    delayed_finish: Hrtimer,
    ap: *mut AtaPort,
    dma_finished: i32,
    c0: *mut core::ffi::c_void,
    cs0: u32,
    cs1: u32,
    is_true_ide: bool,
    dma_base: u64,
}

static mut ENABLE_DMA: i32 = 0;

unsafe fn ns_to_tim_reg(tim_mult: u32, nsecs: u32) -> u32 {
    div_round_up(nsecs.wrapping_mul(octeon_get_io_clock_rate() / 1_000_000),
                 1000u32.wrapping_mul(tim_mult))
}

unsafe fn octeon_cf_set_boot_reg_cfg(cs: i32, multiplier: u32) {
    let mut reg_cfg: CvmxMioBootRegCfgx = core::mem::zeroed();
    let tim_mult = match multiplier { 8 => 3, 4 => 0, 2 => 2, _ => 1 };
    reg_cfg.u64 = cvmx_read_csr(CVMX_MIO_BOOT_REG_CFGX(cs));
    reg_cfg.s.dmack = 0;
    reg_cfg.s.tim_mult = tim_mult;
    reg_cfg.s.rd_dly = 0;
    reg_cfg.s.sam = 0;
    reg_cfg.s.we_ext = 0;
    reg_cfg.s.oe_ext = 0;
    reg_cfg.s.en = 1;
    reg_cfg.s.orbit = 0;
    reg_cfg.s.ale = 0;
    cvmx_write_csr(CVMX_MIO_BOOT_REG_CFGX(cs), reg_cfg.u64);
}

unsafe fn octeon_cf_set_piomode(ap: *mut AtaPort, dev: *mut AtaDevice) {
    let cf_port = (*ap).private_data as *mut OcteonCfPort;
    let mut reg_tim: CvmxMioBootRegTimx = core::mem::zeroed();
    let div: u32 = if octeon_get_io_clock_rate() <= 800_000_000 { 4 } else { 8 };
    let t = ((1_000_000_000_000i64 * div as i64) /
             octeon_get_io_clock_rate() as i64) as i32;
    let timing: AtaTiming = core::mem::zeroed();
    bug_on(ata_timing_compute(dev, (*dev).pio_mode, &timing, t, t));
    let mut t2 = timing.active as i32;
    if t2 != 0 { t2 -= 1; }
    let mut trh = ns_to_tim_reg(div, 20);
    if trh != 0 { trh -= 1; }
    let mut pause = timing.cycle as i32 - timing.active as i32 - timing.setup as i32 - trh as i32;
    if pause < 0 { pause = 0; }
    if pause != 0 { pause -= 1; }
    octeon_cf_set_boot_reg_cfg((*cf_port).cs0 as i32, div);
    if (*cf_port).is_true_ide { octeon_cf_set_boot_reg_cfg((*cf_port).cs1 as i32, div); }
    let use_iordy = ata_pio_need_iordy(dev);
    reg_tim.u64 = cvmx_read_csr(CVMX_MIO_BOOT_REG_TIMX((*cf_port).cs0 as i32));
    reg_tim.s.pagem = 0; reg_tim.s.waitm = use_iordy; reg_tim.s.pages = 0; reg_tim.s.ale = 0;
    reg_tim.s.page = 0; reg_tim.s.wait = 0; reg_tim.s.pause = pause as u32;
    reg_tim.s.wr_hld = trh; reg_tim.s.rd_hld = trh; reg_tim.s.we = t2 as u32; reg_tim.s.oe = t2 as u32;
    reg_tim.s.ce = ns_to_tim_reg(div, 5); reg_tim.s.adr = 0;
    cvmx_write_csr(CVMX_MIO_BOOT_REG_TIMX((*cf_port).cs0 as i32), reg_tim.u64);
    if (*cf_port).is_true_ide { cvmx_write_csr(CVMX_MIO_BOOT_REG_TIMX((*cf_port).cs1 as i32), reg_tim.u64); }
}

unsafe fn octeon_cf_set_dmamode(ap: *mut AtaPort, dev: *mut AtaDevice) {
    let cf_port = (*ap).private_data as *mut OcteonCfPort;
    let mut pin_defs: CvmxMioBootPinDefs = core::mem::zeroed();
    let mut dma_tim: CvmxMioBootDmaTimx = core::mem::zeroed();
    let timing = ata_timing_find_mode((*dev).dma_mode);
    let t0 = (*timing).cycle; let oe_a = (*timing).active; let tkr = (*timing).recover;
    let dma_ackh = (*timing).dmack_hold; let tim_mult = 4u32; let dma_arq = 8u32;
    let pause = 25u32.wrapping_sub(dma_arq * 1000 / (octeon_get_io_clock_rate() / 1_000_000));
    let oe_n = core::cmp::max(t0 - oe_a, tkr);
    pin_defs.u64 = cvmx_read_csr(CVMX_MIO_BOOT_PIN_DEFS);
    let c = ((*cf_port).dma_base & 8) >> 3;
    dma_tim.u64 = 0;
    dma_tim.s.dmack_pi = if pin_defs.u64 & (1u64 << (11 + c)) != 0 { 0 } else { 1 };
    dma_tim.s.oe_n = ns_to_tim_reg(tim_mult, oe_n); dma_tim.s.oe_a = ns_to_tim_reg(tim_mult, oe_a);
    dma_tim.s.dmack_s = ns_to_tim_reg(tim_mult, 20); dma_tim.s.dmack_h = ns_to_tim_reg(tim_mult, dma_ackh);
    dma_tim.s.dmarq = dma_arq; dma_tim.s.pause = ns_to_tim_reg(tim_mult, pause); dma_tim.s.rd_dly = 0;
    dma_tim.s.we_n = ns_to_tim_reg(tim_mult, oe_n); dma_tim.s.we_a = ns_to_tim_reg(tim_mult, oe_a);
    cvmx_write_csr((*cf_port).dma_base + DMA_TIM, dma_tim.u64);
}

unsafe fn octeon_cf_data_xfer8(qc: *mut AtaQueuedCmd, buffer: *mut u8, buflen: u32, rw: i32) -> u32 {
    let ap = (*(*qc).dev).link.ap; let data_addr = (*ap).ioaddr.data_addr; let mut words = buflen as usize;
    if rw != 0 { let mut count = 16; while words != 0 { iowrite8(*buffer, data_addr); buffer = buffer.add(1); words -= 1; count -= 1; if count == 0 { ioread8((*ap).ioaddr.altstatus_addr); count = 16; } } }
    else { ioread8_rep(data_addr, buffer, words); }
    buflen
}

unsafe fn octeon_cf_data_xfer16(qc: *mut AtaQueuedCmd, buffer: *mut u8, buflen: u32, rw: i32) -> u32 {
    let ap = (*(*qc).dev).link.ap; let data_addr = (*ap).ioaddr.data_addr; let mut words = (buflen / 2) as usize;
    if rw != 0 { let mut count = 16; while words != 0 { iowrite16(*(buffer as *const u16), data_addr); buffer = buffer.add(2); words -= 1; count -= 1; if count == 0 { ioread8((*ap).ioaddr.altstatus_addr); count = 16; } } }
    else { while words != 0 { *(buffer as *mut u16) = ioread16(data_addr); buffer = buffer.add(2); words -= 1; } }
    if buflen & 1 != 0 { let mut align_buf = 0u16; if rw == READ { align_buf = cpu_to_le16(ioread16(data_addr)); core::ptr::copy_nonoverlapping(&align_buf as *const u16 as *const u8, buffer, 1); } else { core::ptr::copy_nonoverlapping(buffer, &mut align_buf as *mut u16 as *mut u8, 1); iowrite16(le16_to_cpu(align_buf), data_addr); } }
    buflen
}

/* The remaining declarations retain the kernel driver's externally supplied
 * types and callbacks; their bodies are translated literally below. */
unsafe fn octeon_cf_tf_read16(ap: *mut AtaPort, tf: *mut AtaTaskfile) { let base = (*ap).ioaddr.data_addr; let mut blob = __raw_readw(base.add(0xc)); (*tf).error = blob >> 8; blob = __raw_readw(base.add(2)); (*tf).nsect = blob & 0xff; (*tf).lbal = blob >> 8; blob = __raw_readw(base.add(4)); (*tf).lbam = blob & 0xff; (*tf).lbah = blob >> 8; blob = __raw_readw(base.add(6)); (*tf).device = blob & 0xff; (*tf).status = blob >> 8; }
unsafe fn octeon_cf_check_status16(ap: *mut AtaPort) -> u8 { __raw_readw((*ap).ioaddr.data_addr.add(6)) >> 8 }
unsafe fn octeon_cf_dev_select(_ap: *mut AtaPort, _device: u32) {}
unsafe fn octeon_cf_ata_port_noaction(_ap: *mut AtaPort) {}
unsafe fn octeon_cf_check_atapi_dma(_qc: *mut AtaQueuedCmd) -> i32 { 0 }

/* Kernel callback declarations and registration metadata are preserved as
 * external bindings because their definitions belong to the surrounding tree. */
extern "C" {
    fn octeon_get_io_clock_rate() -> u32;
    fn cvmx_read_csr(addr: u64) -> u64;
    fn cvmx_write_csr(addr: u64, val: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
