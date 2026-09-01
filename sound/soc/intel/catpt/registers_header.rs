/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2020 Intel Corporation
 *
 * Author: Cezary Rojewski <cezary.rojewski@intel.com>
 */

/* Translated from C header: linux/bitops.h, linux/iopoll.h and
 * uapi/linux/pci_regs.h provide external bit and I/O helpers/constants.
 */

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

pub const PCI_PM_CTRL: u32 = 4;
pub const UINT_MAX: u32 = u32::MAX;

extern "C" {
    pub fn readl(addr: *const u8) -> u32;
    pub fn writel(value: u32, addr: *mut u8);
    pub fn hweight_long(value: u64) -> u32;
}

pub const CATPT_SHIM_REGS_SIZE: u32 = 4096;
pub const CATPT_DMA_REGS_SIZE: u32 = 1024;
pub const CATPT_DMA_COUNT: u32 = 2;
pub const CATPT_SSP_REGS_SIZE: u32 = 512;

/* DSP Shim registers */

pub const CATPT_SHIM_CS1: u32 = 0x00;
pub const CATPT_SHIM_ISC: u32 = 0x18;
pub const CATPT_SHIM_ISD: u32 = 0x20;
pub const CATPT_SHIM_IMC: u32 = 0x28;
pub const CATPT_SHIM_IMD: u32 = 0x30;
pub const CATPT_SHIM_IPCC: u32 = 0x38;
pub const CATPT_SHIM_IPCD: u32 = 0x40;
pub const CATPT_SHIM_CLKCTL: u32 = 0x78;
pub const CATPT_SHIM_CS2: u32 = 0x80;
pub const CATPT_SHIM_LTRC: u32 = 0xE0;
pub const CATPT_SHIM_HMDC: u32 = 0xE8;

pub const CATPT_CS_LPCS: u32 = BIT(31);
pub const fn CATPT_CS_SFCR(ssp: u32) -> u32 {
    BIT(27 + ssp)
}
pub const CATPT_CS_S1IOCS: u32 = BIT(23);
pub const CATPT_CS_S0IOCS: u32 = BIT(21);
pub const CATPT_CS_PCE: u32 = BIT(15);
pub const fn CATPT_CS_SDPM(ssp: u32) -> u32 {
    BIT(11 + ssp)
}
pub const CATPT_CS_STALL: u32 = BIT(10);
pub const CATPT_CS_DCS: u32 = GENMASK(6, 4);
/* b100 DSP core & audio fabric high clock */
pub const CATPT_CS_DCS_HIGH: u32 = 0x4 << 4;
pub const fn CATPT_CS_SBCS(ssp: u32) -> u32 {
    BIT(2 + ssp)
}
pub const CATPT_CS_RST: u32 = BIT(1);

pub const CATPT_ISC_IPCDB: u32 = BIT(1);
pub const CATPT_ISC_IPCCD: u32 = BIT(0);
pub const CATPT_ISD_DCPWM: u32 = BIT(31);
pub const CATPT_ISD_IPCCB: u32 = BIT(1);
pub const CATPT_ISD_IPCDD: u32 = BIT(0);

pub const CATPT_IMC_IPCDB: u32 = BIT(1);
pub const CATPT_IMC_IPCCD: u32 = BIT(0);
pub const CATPT_IMD_IPCCB: u32 = BIT(1);
pub const CATPT_IMD_IPCDD: u32 = BIT(0);

pub const CATPT_IPCC_BUSY: u32 = BIT(31);
pub const CATPT_IPCC_DONE: u32 = BIT(30);
pub const CATPT_IPCD_BUSY: u32 = BIT(31);
pub const CATPT_IPCD_DONE: u32 = BIT(30);

pub const CATPT_CLKCTL_CFCIP: u32 = BIT(31);
pub const CATPT_CLKCTL_SMOS: u32 = GENMASK(25, 24);

pub const fn CATPT_HMDC_HDDA(e: u32, ch: u32) -> u32 {
    BIT(8 * e + ch)
}

/* defaults to reset SHIM registers to after each power cycle */
pub const CATPT_CS_DEFAULT: u32 = 0x8480040E;
pub const CATPT_ISC_DEFAULT: u32 = 0x0;
pub const CATPT_ISD_DEFAULT: u32 = 0x0;
pub const CATPT_IMC_DEFAULT: u32 = 0x7FFF0003;
pub const CATPT_IMD_DEFAULT: u32 = 0x7FFF0003;
pub const CATPT_IPCC_DEFAULT: u32 = 0x0;
pub const CATPT_IPCD_DEFAULT: u32 = 0x0;
pub const CATPT_CLKCTL_DEFAULT: u32 = 0x7FF;
pub const CATPT_CS2_DEFAULT: u32 = 0x0;
pub const CATPT_LTRC_DEFAULT: u32 = 0x0;
pub const CATPT_HMDC_DEFAULT: u32 = 0x0;

/* PCI Configuration registers */

pub const CATPT_PCI_PMCAPID: u32 = 0x80;
pub const CATPT_PCI_PMCS: u32 = CATPT_PCI_PMCAPID + PCI_PM_CTRL;
pub const CATPT_PCI_VDRTCTL0: u32 = 0xA0;
pub const CATPT_PCI_VDRTCTL2: u32 = 0xA8;

pub const CATPT_VDRTCTL2_DTCGE: u32 = BIT(10);
pub const CATPT_VDRTCTL2_DCLCGE: u32 = BIT(1);
pub const CATPT_VDRTCTL2_CGEALL: u32 = 0xF7F;

/* LPT PCI Configuration bits */

pub const fn LPT_VDRTCTL0_DSRAMPGE(b: u32) -> u32 {
    BIT(16 + b)
}
pub const LPT_VDRTCTL0_DSRAMPGE_MASK: u32 = GENMASK(31, 16);
pub const fn LPT_VDRTCTL0_ISRAMPGE(b: u32) -> u32 {
    BIT(6 + b)
}
pub const LPT_VDRTCTL0_ISRAMPGE_MASK: u32 = GENMASK(15, 6);
pub const LPT_VDRTCTL0_D3SRAMPGD: u32 = BIT(2);
pub const LPT_VDRTCTL0_D3PGD: u32 = BIT(1);
pub const LPT_VDRTCTL0_APLLSE: u32 = BIT(0);

/* WPT PCI Configuration bits */

pub const fn WPT_VDRTCTL0_DSRAMPGE(b: u32) -> u32 {
    BIT(12 + b)
}
pub const WPT_VDRTCTL0_DSRAMPGE_MASK: u32 = GENMASK(31, 12);
pub const fn WPT_VDRTCTL0_ISRAMPGE(b: u32) -> u32 {
    BIT(2 + b)
}
pub const WPT_VDRTCTL0_ISRAMPGE_MASK: u32 = GENMASK(11, 2);
pub const WPT_VDRTCTL0_D3SRAMPGD: u32 = BIT(1);
pub const WPT_VDRTCTL0_D3PGD: u32 = BIT(0);

pub const WPT_VDRTCTL2_APLLSE: u32 = BIT(31);

/* defaults to reset SSP registers to after each power cycle */
pub const CATPT_SSC0_DEFAULT: u32 = 0x0;
pub const CATPT_SSC1_DEFAULT: u32 = 0x0;
pub const CATPT_SSS_DEFAULT: u32 = 0xF004;
pub const CATPT_SSIT_DEFAULT: u32 = 0x0;
pub const CATPT_SSD_DEFAULT: u32 = 0xC43893A3;
pub const CATPT_SSTO_DEFAULT: u32 = 0x0;
pub const CATPT_SSPSP_DEFAULT: u32 = 0x0;
pub const CATPT_SSTSA_DEFAULT: u32 = 0x0;
pub const CATPT_SSRSA_DEFAULT: u32 = 0x0;
pub const CATPT_SSTSS_DEFAULT: u32 = 0x0;
pub const CATPT_SSCR2_DEFAULT: u32 = 0x0;
pub const CATPT_SSPSP2_DEFAULT: u32 = 0x0;

/* Coredump register and its states */
pub const CATPT_DRAM_COREDUMP: u32 = 0x1F4;
pub const CATPT_COREDUMP_REQUEST: u32 = UINT_MAX;
pub const CATPT_COREDUMP_RELEASE: u32 = 0;

/* Physically the same block, access address differs between host and dsp */
pub const CATPT_DSP_DRAM_OFFSET: u32 = 0x400000;
pub const fn catpt_to_host_offset(offset: u32) -> u32 {
    offset & !CATPT_DSP_DRAM_OFFSET
}
pub const fn catpt_to_dsp_offset(offset: u32) -> u32 {
    offset | CATPT_DSP_DRAM_OFFSET
}

pub const CATPT_MEMBLOCK_SIZE: u32 = 0x8000;

#[repr(C)]
pub struct catpt_dev {
    pub lpe_ba: *mut u8,
    pub pci_ba: *mut u8,
    pub spec: *const catpt_spec,
    pub ipc: catpt_ipc,
}

#[repr(C)]
pub struct catpt_spec {
    pub dram_mask: u64,
    pub iram_mask: u64,
    pub host_dram_offset: usize,
    pub host_iram_offset: usize,
    pub host_shim_offset: usize,
    pub host_dma_offset: [usize; CATPT_DMA_COUNT as usize],
    pub host_ssp_offset: [usize; 0],
}

#[repr(C)]
pub struct catpt_ipc {
    pub config: catpt_ipc_config,
}

#[repr(C)]
pub struct catpt_ipc_config {
    pub inbox_offset: usize,
    pub outbox_offset: usize,
}

pub unsafe fn catpt_num_dram(cdev: *const catpt_dev) -> u32 {
    hweight_long((*(*cdev).spec).dram_mask)
}

pub unsafe fn catpt_num_iram(cdev: *const catpt_dev) -> u32 {
    hweight_long((*(*cdev).spec).iram_mask)
}

pub unsafe fn catpt_dram_size(cdev: *const catpt_dev) -> u32 {
    catpt_num_dram(cdev).wrapping_mul(CATPT_MEMBLOCK_SIZE)
}

pub unsafe fn catpt_iram_size(cdev: *const catpt_dev) -> u32 {
    catpt_num_iram(cdev).wrapping_mul(CATPT_MEMBLOCK_SIZE)
}

/* registry I/O helpers */

pub unsafe fn catpt_dram_addr(cdev: *const catpt_dev) -> *mut u8 {
    (*cdev).lpe_ba.add((*(*cdev).spec).host_dram_offset)
}

pub unsafe fn catpt_iram_addr(cdev: *const catpt_dev) -> *mut u8 {
    (*cdev).lpe_ba.add((*(*cdev).spec).host_iram_offset)
}

pub unsafe fn catpt_shim_addr(cdev: *const catpt_dev) -> *mut u8 {
    (*cdev).lpe_ba.add((*(*cdev).spec).host_shim_offset)
}

pub unsafe fn catpt_dma_addr(cdev: *const catpt_dev, dma: usize) -> *mut u8 {
    (*cdev).lpe_ba.add((*(*cdev).spec).host_dma_offset[dma])
}

pub unsafe fn catpt_ssp_addr(cdev: *const catpt_dev, ssp: usize) -> *mut u8 {
    (*cdev).lpe_ba.add((*(*cdev).spec).host_ssp_offset[ssp])
}

pub unsafe fn catpt_inbox_addr(cdev: *const catpt_dev) -> *mut u8 {
    (*cdev).lpe_ba.add((*cdev).ipc.config.inbox_offset)
}

pub unsafe fn catpt_outbox_addr(cdev: *const catpt_dev) -> *mut u8 {
    (*cdev).lpe_ba.add((*cdev).ipc.config.outbox_offset)
}

pub unsafe fn catpt_writel_ssp(cdev: *const catpt_dev, ssp: usize, reg: usize, val: u32) {
    writel(val, catpt_ssp_addr(cdev, ssp).add(reg));
}

pub unsafe fn catpt_readl_dram(cdev: *const catpt_dev, reg: u32) -> u32 {
    readl(catpt_dram_addr(cdev).add((CATPT_DRAM_COREDUMP + reg - CATPT_DRAM_COREDUMP) as usize))
}

pub unsafe fn catpt_writel_dram(cdev: *const catpt_dev, reg: u32, val: u32) {
    writel(
        val,
        catpt_dram_addr(cdev).add((CATPT_DRAM_COREDUMP + reg - CATPT_DRAM_COREDUMP) as usize),
    );
}

pub unsafe fn catpt_readl_shim(cdev: *const catpt_dev, reg: u32) -> u32 {
    readl(catpt_shim_addr(cdev).add(reg as usize))
}

pub unsafe fn catpt_writel_shim(cdev: *const catpt_dev, reg: u32, val: u32) {
    writel(val, catpt_shim_addr(cdev).add(reg as usize));
}

pub unsafe fn catpt_updatel_shim(cdev: *const catpt_dev, reg: u32, mask: u32, val: u32) {
    catpt_writel_shim(cdev, reg, (catpt_readl_shim(cdev, reg) & !mask) | val);
}

/* C macro catpt_readl_poll_shim(cdev, reg, val, cond, delay_us, timeout_us)
 * expands to readl_poll_timeout(catpt_shim_addr(cdev) + CATPT_SHIM_##reg,
 * val, cond, delay_us, timeout_us). Its condition expression cannot be
 * represented file-locally as a Rust value-preserving helper.
 */

pub unsafe fn catpt_readl_pci(cdev: *const catpt_dev, reg: u32) -> u32 {
    readl((*cdev).pci_ba.add(reg as usize))
}

pub unsafe fn catpt_writel_pci(cdev: *const catpt_dev, reg: u32, val: u32) {
    writel(val, (*cdev).pci_ba.add(reg as usize));
}

pub unsafe fn catpt_updatel_pci(cdev: *const catpt_dev, reg: u32, mask: u32, val: u32) {
    catpt_writel_pci(cdev, reg, (catpt_readl_pci(cdev, reg) & !mask) | val);
}

/* C macro catpt_readl_poll_pci(cdev, reg, val, cond, delay_us, timeout_us)
 * expands to readl_poll_timeout((cdev)->pci_ba + CATPT_PCI_##reg,
 * val, cond, delay_us, timeout_us). Its condition expression cannot be
 * represented file-locally as a Rust value-preserving helper.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
