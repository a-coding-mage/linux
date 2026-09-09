/*
 * Definitions for TX4937/TX4938
 * Copyright (C) 2000-2001 Toshiba Corporation
 *
 * 2003-2005 (c) MontaVista Software, Inc. This file is licensed under the
 * terms of the GNU General Public License version 2.
 *
 * Support for TX4938 in 2.6 - Manish Lachwani (mlachwani@mvista.com)
 */

// Some controllers are compatible with TX4927; symbols from that header are
// intentionally left as external dependencies.

#[cfg(target_pointer_width = "64")]
pub const TX4938_REG_BASE: u64 = 0xffffffffff1f0000;
#[cfg(not(target_pointer_width = "64"))]
pub const TX4938_REG_BASE: u32 = 0xff1f0000;
pub const TX4938_REG_SIZE: u32 = 0x00010000;

pub const TX4938_NDFMC_REG: u64 = TX4938_REG_BASE as u64 + 0x5000;
pub const TX4938_SRAMC_REG: u64 = TX4938_REG_BASE as u64 + 0x6000;
pub const TX4938_PCIC1_REG: u64 = TX4938_REG_BASE as u64 + 0x7000;
pub const TX4938_SDRAMC_REG: u64 = TX4938_REG_BASE as u64 + 0x8000;
pub const TX4938_EBUSC_REG: u64 = TX4938_REG_BASE as u64 + 0x9000;
pub const TX4938_PCIC_REG: u64 = TX4938_REG_BASE as u64 + 0xd000;
pub const TX4938_CCFG_REG: u64 = TX4938_REG_BASE as u64 + 0xe000;
pub const TX4938_NR_TMR: u32 = 3;
pub const TX4938_NR_SIO: u32 = 2;
pub const TX4938_PIO_REG: u64 = TX4938_REG_BASE as u64 + 0xf500;
pub const TX4938_IRC_REG: u64 = TX4938_REG_BASE as u64 + 0xf600;
pub const TX4938_ACLC_REG: u64 = TX4938_REG_BASE as u64 + 0xf700;
pub const TX4938_SPI_REG: u64 = TX4938_REG_BASE as u64 + 0xf800;

#[inline] pub const fn TX4938_DMA_REG(ch: u64) -> u64 { TX4938_REG_BASE as u64 + 0xb000 + ch * 0x800 }
#[inline] pub const fn TX4938_TMR_REG(ch: u64) -> u64 { TX4938_REG_BASE as u64 + 0xf000 + ch * 0x100 }
#[inline] pub const fn TX4938_SIO_REG(ch: u64) -> u64 { TX4938_REG_BASE as u64 + 0xf300 + ch * 0x100 }

#[repr(C)]
pub struct tx4938_sramc_reg { pub cr: u64 }
#[repr(C)]
pub struct tx4938_ccfg_reg {
    pub ccfg: u64, pub crir: u64, pub pcfg: u64, pub toea: u64, pub clkctr: u64,
    pub unused0: u64, pub garbc: u64, pub unused1: u64, pub unused2: u64,
    pub ramp: u64, pub unused3: u64, pub jmpadr: u64,
}

pub const TX4938_IR_ECCERR: u32 = 0; pub const TX4938_IR_WTOERR: u32 = 1;
pub const TX4938_NUM_IR_INT: u32 = 6;
#[inline] pub const fn TX4938_IR_INT(n: u32) -> u32 { 2 + n }
pub const TX4938_NUM_IR_SIO: u32 = 2;
#[inline] pub const fn TX4938_IR_SIO(n: u32) -> u32 { 8 + n }
pub const TX4938_NUM_IR_DMA: u32 = 4;
#[inline] pub const fn TX4938_IR_DMA(ch: u32, n: u32) -> u32 { (if ch != 0 { 27 } else { 10 }) + n }
pub const TX4938_IR_PIO: u32 = 14; pub const TX4938_IR_PDMAC: u32 = 15;
pub const TX4938_IR_PCIC: u32 = 16; pub const TX4938_NUM_IR_TMR: u32 = 3;
#[inline] pub const fn TX4938_IR_TMR(n: u32) -> u32 { 17 + n }
pub const TX4938_IR_NDFMC: u32 = 21; pub const TX4938_IR_PCIERR: u32 = 22;
pub const TX4938_IR_PCIPME: u32 = 23; pub const TX4938_IR_ACLC: u32 = 24;
pub const TX4938_IR_ACLCPME: u32 = 25; pub const TX4938_IR_PCIC1: u32 = 26;
pub const TX4938_IR_SPI: u32 = 31; pub const TX4938_NUM_IR: u32 = 32;
pub const TX4938_IR_ETH0: u32 = TX4938_IR_INT(4); pub const TX4938_IR_ETH1: u32 = TX4938_IR_INT(3);
pub const TX4938_IRC_INT: u32 = 2; pub const TX4938_NUM_PIO: u32 = 16;

pub const TX4938_CCFG_WDRST: u64 = 0x0000020000000000; pub const TX4938_CCFG_WDREXEN: u64 = 0x0000010000000000;
pub const TX4938_CCFG_BCFG_MASK: u64 = 0x000000ff00000000; pub const TX4938_CCFG_TINTDIS: u64 = 0x01000000;
pub const TX4938_CCFG_PCI66: u64 = 0x00800000; pub const TX4938_CCFG_PCIMODE: u64 = 0x00400000;
pub const TX4938_CCFG_PCI1_66: u64 = 0x00200000; pub const TX4938_CCFG_DIVMODE_MASK: u64 = 0x001e0000;
pub const TX4938_CCFG_DIVMODE_2: u64 = 0x4 << 17; pub const TX4938_CCFG_DIVMODE_2_5: u64 = 0xf << 17;
pub const TX4938_CCFG_DIVMODE_3: u64 = 0x5 << 17; pub const TX4938_CCFG_DIVMODE_4: u64 = 0x6 << 17;
pub const TX4938_CCFG_DIVMODE_4_5: u64 = 0xd << 17; pub const TX4938_CCFG_DIVMODE_8: u64 = 0;
pub const TX4938_CCFG_DIVMODE_10: u64 = 0xb << 17; pub const TX4938_CCFG_DIVMODE_12: u64 = 1 << 17;
pub const TX4938_CCFG_DIVMODE_16: u64 = 2 << 17; pub const TX4938_CCFG_DIVMODE_18: u64 = 9 << 17;
pub const TX4938_CCFG_BEOW: u64 = 0x10000; pub const TX4938_CCFG_WR: u64 = 0x8000; pub const TX4938_CCFG_TOE: u64 = 0x4000;
pub const TX4938_CCFG_PCIARB: u64 = 0x2000; pub const TX4938_CCFG_PCIDIVMODE_MASK: u64 = 0x1c00;
pub const TX4938_CCFG_PCIDIVMODE_4: u64 = 1 << 10; pub const TX4938_CCFG_PCIDIVMODE_4_5: u64 = 3 << 10;
pub const TX4938_CCFG_PCIDIVMODE_5: u64 = 5 << 10; pub const TX4938_CCFG_PCIDIVMODE_5_5: u64 = 7 << 10;
pub const TX4938_CCFG_PCIDIVMODE_8: u64 = 0; pub const TX4938_CCFG_PCIDIVMODE_9: u64 = 2 << 10;
pub const TX4938_CCFG_PCIDIVMODE_10: u64 = 4 << 10; pub const TX4938_CCFG_PCIDIVMODE_11: u64 = 6 << 10;
pub const TX4938_CCFG_PCI1DMD: u64 = 0x100; pub const TX4938_CCFG_SYSSP_MASK: u64 = 0xc0;
pub const TX4938_CCFG_ENDIAN: u64 = 4; pub const TX4938_CCFG_HALT: u64 = 2; pub const TX4938_CCFG_ACEHOLD: u64 = 1;

pub const TX4938_PCFG_ETH0_SEL: u64 = 0x8000000000000000; pub const TX4938_PCFG_ETH1_SEL: u64 = 0x4000000000000000;
pub const TX4938_PCFG_ATA_SEL: u64 = 0x2000000000000000; pub const TX4938_PCFG_ISA_SEL: u64 = 0x1000000000000000;
pub const TX4938_PCFG_SPI_SEL: u64 = 0x0800000000000000; pub const TX4938_PCFG_NDF_SEL: u64 = 0x0400000000000000;
pub const TX4938_PCFG_SDCLKDLY_MASK: u64 = 0x30000000;
#[inline] pub const fn TX4938_PCFG_SDCLKDLY(d: u64) -> u64 { d << 28 }
pub const TX4938_PCFG_SYSCLKEN: u64 = 0x08000000; pub const TX4938_PCFG_SDCLKEN_ALL: u64 = 0x07800000;
#[inline] pub const fn TX4938_PCFG_SDCLKEN(ch: u64) -> u64 { 0x00800000 << ch }
pub const TX4938_PCFG_PCICLKEN_ALL: u64 = 0x003f0000;
#[inline] pub const fn TX4938_PCFG_PCICLKEN(ch: u64) -> u64 { 0x00010000 << ch }
pub const TX4938_PCFG_SEL2: u64 = 0x200; pub const TX4938_PCFG_SEL1: u64 = 0x100; pub const TX4938_PCFG_DMASEL_ALL: u64 = 0xf;
pub const TX4938_PCFG_DMASEL0_DRQ0: u64 = 0; pub const TX4938_PCFG_DMASEL0_SIO1: u64 = 1;
pub const TX4938_PCFG_DMASEL1_DRQ1: u64 = 0; pub const TX4938_PCFG_DMASEL1_SIO1: u64 = 2;
pub const TX4938_PCFG_DMASEL2_DRQ2: u64 = 0; pub const TX4938_PCFG_DMASEL2_SIO0: u64 = 4;
pub const TX4938_PCFG_DMASEL3_DRQ3: u64 = 0; pub const TX4938_PCFG_DMASEL3_SIO0: u64 = 8;

pub const TX4938_CLKCTR_NDFCKD: u64 = 0x0001000000000000; pub const TX4938_CLKCTR_NDFRST: u64 = 0x0000000100000000;
pub const TX4938_CLKCTR_ETH1CKD: u64 = 0x80000000; pub const TX4938_CLKCTR_ETH0CKD: u64 = 0x40000000;
pub const TX4938_CLKCTR_SPICKD: u64 = 0x20000000; pub const TX4938_CLKCTR_SRAMCKD: u64 = 0x10000000;
pub const TX4938_CLKCTR_PCIC1CKD: u64 = 0x08000000; pub const TX4938_CLKCTR_DMA1CKD: u64 = 0x04000000;
pub const TX4938_CLKCTR_ACLCKD: u64 = 0x02000000; pub const TX4938_CLKCTR_PIOCKD: u64 = 0x01000000;
pub const TX4938_CLKCTR_DMACKD: u64 = 0x00800000; pub const TX4938_CLKCTR_PCICKD: u64 = 0x00400000;
pub const TX4938_CLKCTR_TM0CKD: u64 = 0x00100000; pub const TX4938_CLKCTR_TM1CKD: u64 = 0x00080000; pub const TX4938_CLKCTR_TM2CKD: u64 = 0x00040000;
pub const TX4938_CLKCTR_SIO0CKD: u64 = 0x00020000; pub const TX4938_CLKCTR_SIO1CKD: u64 = 0x00010000;
pub const TX4938_CLKCTR_ETH1RST: u64 = 0x8000; pub const TX4938_CLKCTR_ETH0RST: u64 = 0x4000; pub const TX4938_CLKCTR_SPIRST: u64 = 0x2000;
pub const TX4938_CLKCTR_SRAMRST: u64 = 0x1000; pub const TX4938_CLKCTR_PCIC1RST: u64 = 0x800; pub const TX4938_CLKCTR_DMA1RST: u64 = 0x400;
pub const TX4938_CLKCTR_ACLRST: u64 = 0x200; pub const TX4938_CLKCTR_PIORST: u64 = 0x100; pub const TX4938_CLKCTR_DMARST: u64 = 0x80;
pub const TX4938_CLKCTR_PCIRST: u64 = 0x40; pub const TX4938_CLKCTR_TM0RST: u64 = 0x10; pub const TX4938_CLKCTR_TM1RST: u64 = 8;
pub const TX4938_CLKCTR_TM2RST: u64 = 4; pub const TX4938_CLKCTR_SIO0RST: u64 = 2; pub const TX4938_CLKCTR_SIO1RST: u64 = 1;

#[inline] pub const fn TX4938_DMA_MCR_EIS(ch: u32) -> u32 { 0x10000000 << ch }
#[inline] pub const fn TX4938_DMA_MCR_DIS(ch: u32) -> u32 { 0x01000000 << ch }
pub const TX4938_DMA_MCR_RSFIF: u32 = 0x80;
#[inline] pub const fn TX4938_DMA_MCR_FIFUM(ch: u32) -> u32 { 8 << ch }
pub const TX4938_DMA_MCR_RPRT: u32 = 2; pub const TX4938_DMA_MCR_MSTEN: u32 = 1;
pub const TX4938_DMA_CCR_IMMCHN: u32 = 0x20000000; pub const TX4938_DMA_CCR_USEXFSZ: u32 = 0x10000000;
pub const TX4938_DMA_CCR_LE: u32 = 0x08000000; pub const TX4938_DMA_CCR_DBINH: u32 = 0x04000000; pub const TX4938_DMA_CCR_SBINH: u32 = 0x02000000;
pub const TX4938_DMA_CCR_CHRST: u32 = 0x01000000; pub const TX4938_DMA_CCR_RVBYTE: u32 = 0x00800000; pub const TX4938_DMA_CCR_ACKPOL: u32 = 0x00400000;
pub const TX4938_DMA_CCR_REQPL: u32 = 0x00200000; pub const TX4938_DMA_CCR_EGREQ: u32 = 0x00100000; pub const TX4938_DMA_CCR_CHDN: u32 = 0x00080000;
pub const TX4938_DMA_CCR_DNCTL: u32 = 0x00060000; pub const TX4938_DMA_CCR_EXTRQ: u32 = 0x00010000; pub const TX4938_DMA_CCR_INTRQD: u32 = 0x0000e000;
pub const TX4938_DMA_CCR_INTENE: u32 = 0x1000; pub const TX4938_DMA_CCR_INTENC: u32 = 0x800; pub const TX4938_DMA_CCR_INTENT: u32 = 0x400;
pub const TX4938_DMA_CCR_CHNEN: u32 = 0x200; pub const TX4938_DMA_CCR_XFACT: u32 = 0x100; pub const TX4938_DMA_CCR_SMPCHN: u32 = 0x20;
#[inline] pub const fn TX4938_DMA_CCR_XFSZ(order: u32) -> u32 { (order << 2) & 0x1c }
pub const TX4938_DMA_CCR_XFSZ_1W: u32 = TX4938_DMA_CCR_XFSZ(2); pub const TX4938_DMA_CCR_XFSZ_2W: u32 = TX4938_DMA_CCR_XFSZ(3);
pub const TX4938_DMA_CCR_XFSZ_4W: u32 = TX4938_DMA_CCR_XFSZ(4); pub const TX4938_DMA_CCR_XFSZ_8W: u32 = TX4938_DMA_CCR_XFSZ(5);
pub const TX4938_DMA_CCR_XFSZ_16W: u32 = TX4938_DMA_CCR_XFSZ(6); pub const TX4938_DMA_CCR_XFSZ_32W: u32 = TX4938_DMA_CCR_XFSZ(7);
pub const TX4938_DMA_CCR_MEMIO: u32 = 2; pub const TX4938_DMA_CCR_SNGAD: u32 = 1;
pub const TX4938_DMA_CSR_CHNEN: u32 = 0x400; pub const TX4938_DMA_CSR_STLXFER: u32 = 0x200; pub const TX4938_DMA_CSR_CHNACT: u32 = 0x100;
pub const TX4938_DMA_CSR_ABCHC: u32 = 0x80; pub const TX4938_DMA_CSR_NCHNC: u32 = 0x40; pub const TX4938_DMA_CSR_NTRNFC: u32 = 0x20;
pub const TX4938_DMA_CSR_EXTDN: u32 = 0x10; pub const TX4938_DMA_CSR_CFERR: u32 = 8; pub const TX4938_DMA_CSR_CHERR: u32 = 4;
pub const TX4938_DMA_CSR_DESERR: u32 = 2; pub const TX4938_DMA_CSR_SORERR: u32 = 1;

#[repr(C)] pub struct tx4938ide_platform_info { pub ioport_shift: u32, pub gbus_clock: u32, pub ebus_ch: u32 }

extern "C" {
    pub fn tx4938_wdt_init(); pub fn tx4938_setup(); pub fn tx4938_time_init(tmrnr: u32);
    pub fn tx4938_sio_init(sclk: u32, cts_mask: u32); pub fn tx4938_ethaddr_init(addr0: *mut u8, addr1: *mut u8);
    pub fn tx4938_report_pciclk() -> i32; pub fn tx4938_report_pci1clk(); pub fn tx4938_pciclk66_setup() -> i32;
    pub fn tx4938_pcic1_map_irq(dev: *const core::ffi::c_void, slot: u8) -> i32; pub fn tx4938_setup_pcierr_irq();
    pub fn tx4938_irq_init(); pub fn tx4938_mtd_init(ch: i32); pub fn tx4938_ndfmc_init(hold: u32, spw: u32);
    pub fn tx4938_ata_init(irq: u32, shift: u32, tune: i32); pub fn tx4938_dmac_init(memcpy_chan0: i32, memcpy_chan1: i32);
    pub fn tx4938_aclc_init(); pub fn tx4938_sramc_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
