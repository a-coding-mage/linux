/*
 * Translation of tx4927.h. Linux include dependencies and build-time
 * configuration are intentionally left as external Rust dependencies.
 */

#[cfg(feature = "config_64bit")]
pub const TX4927_REG_BASE: u64 = 0xffffffffff1f0000;
#[cfg(not(feature = "config_64bit"))]
pub const TX4927_REG_BASE: u64 = 0xff1f0000;
pub const TX4927_REG_SIZE: u64 = 0x00010000;

pub const TX4927_SDRAMC_REG: u64 = TX4927_REG_BASE + 0x8000;
pub const TX4927_EBUSC_REG: u64 = TX4927_REG_BASE + 0x9000;
pub const TX4927_DMA_REG: u64 = TX4927_REG_BASE + 0xb000;
pub const TX4927_PCIC_REG: u64 = TX4927_REG_BASE + 0xd000;
pub const TX4927_CCFG_REG: u64 = TX4927_REG_BASE + 0xe000;
pub const TX4927_IRC_REG: u64 = TX4927_REG_BASE + 0xf600;
pub const TX4927_NR_TMR: usize = 3;
pub const TX4927_NR_SIO: usize = 2;
pub const TX4927_PIO_REG: u64 = TX4927_REG_BASE + 0xf500;
pub const TX4927_ACLC_REG: u64 = TX4927_REG_BASE + 0xf700;

pub const TX4927_IR_ECCERR: u32 = 0;
pub const TX4927_IR_WTOERR: u32 = 1;
pub const TX4927_NUM_IR_INT: u32 = 6;
pub const TX4927_NUM_IR_SIO: u32 = 2;
pub const TX4927_NUM_IR_DMA: u32 = 4;
pub const TX4927_IR_PIO: u32 = 14;
pub const TX4927_IR_PDMAC: u32 = 15;
pub const TX4927_IR_PCIC: u32 = 16;
pub const TX4927_NUM_IR_TMR: u32 = 3;
pub const TX4927_IR_PCIERR: u32 = 22;
pub const TX4927_IR_PCIPME: u32 = 23;
pub const TX4927_IR_ACLC: u32 = 24;
pub const TX4927_IR_ACLCPME: u32 = 25;
pub const TX4927_NUM_IR: u32 = 32;
pub const TX4927_IRC_INT: u32 = 2;
pub const TX4927_NUM_PIO: u32 = 16;

pub const fn TX4927_TMR_REG(ch: u64) -> u64 { TX4927_REG_BASE + 0xf000 + ch * 0x100 }
pub const fn TX4927_SIO_REG(ch: u64) -> u64 { TX4927_REG_BASE + 0xf300 + ch * 0x100 }
pub const fn TX4927_IR_INT(n: u32) -> u32 { 2 + n }
pub const fn TX4927_IR_SIO(n: u32) -> u32 { 8 + n }
pub const fn TX4927_IR_DMA(n: u32) -> u32 { 10 + n }
pub const fn TX4927_IR_TMR(n: u32) -> u32 { 17 + n }

#[repr(C)]
pub struct tx4927_sdramc_reg { pub cr: [u64; 4], pub unused0: [u64; 4], pub tr: u64, pub unused1: [u64; 2], pub cmd: u64 }
#[repr(C)]
pub struct tx4927_ebusc_reg { pub cr: [u64; 8] }
#[repr(C)]
pub struct tx4927_ccfg_reg {
    pub ccfg: u64, pub crir: u64, pub pcfg: u64, pub toea: u64, pub clkctr: u64,
    pub unused0: u64, pub garbc: u64, pub unused1: u64, pub unused2: u64, pub ramp: u64,
}

pub const TX4927_CCFG_WDRST: u64 = 0x0000020000000000; pub const TX4927_CCFG_WDREXEN: u64 = 0x0000010000000000;
pub const TX4927_CCFG_BCFG_MASK: u64 = 0x000000ff00000000; pub const TX4927_CCFG_TINTDIS: u64 = 0x01000000;
pub const TX4927_CCFG_PCI66: u64 = 0x00800000; pub const TX4927_CCFG_PCIMODE: u64 = 0x00400000;
pub const TX4927_CCFG_DIVMODE_MASK: u64 = 0x000e0000;
pub const TX4927_CCFG_DIVMODE_8: u64 = 0x0 << 17; pub const TX4927_CCFG_DIVMODE_12: u64 = 0x1 << 17; pub const TX4927_CCFG_DIVMODE_16: u64 = 0x2 << 17; pub const TX4927_CCFG_DIVMODE_10: u64 = 0x3 << 17; pub const TX4927_CCFG_DIVMODE_2: u64 = 0x4 << 17; pub const TX4927_CCFG_DIVMODE_3: u64 = 0x5 << 17; pub const TX4927_CCFG_DIVMODE_4: u64 = 0x6 << 17; pub const TX4927_CCFG_DIVMODE_2_5: u64 = 0x7 << 17;
pub const TX4927_CCFG_BEOW: u64 = 0x00010000; pub const TX4927_CCFG_WR: u64 = 0x00008000; pub const TX4927_CCFG_TOE: u64 = 0x00004000; pub const TX4927_CCFG_PCIARB: u64 = 0x00002000; pub const TX4927_CCFG_PCIDIVMODE_MASK: u64 = 0x00001800; pub const TX4927_CCFG_PCIDIVMODE_2_5: u64 = 0; pub const TX4927_CCFG_PCIDIVMODE_3: u64 = 0x800; pub const TX4927_CCFG_PCIDIVMODE_5: u64 = 0x1000; pub const TX4927_CCFG_PCIDIVMODE_6: u64 = 0x1800; pub const TX4927_CCFG_SYSSP_MASK: u64 = 0xc0; pub const TX4927_CCFG_ENDIAN: u64 = 4; pub const TX4927_CCFG_HALT: u64 = 2; pub const TX4927_CCFG_ACEHOLD: u64 = 1; pub const TX4927_CCFG_W1CBITS: u64 = TX4927_CCFG_WDRST | TX4927_CCFG_BEOW;

pub const TX4927_PCFG_SDCLKDLY_MASK: u64 = 0x30000000; pub const TX4927_PCFG_SYSCLKEN: u64 = 0x08000000; pub const TX4927_PCFG_SDCLKEN_ALL: u64 = 0x07800000; pub const TX4927_PCFG_PCICLKEN_ALL: u64 = 0x003f0000; pub const TX4927_PCFG_SEL2: u64 = 0x200; pub const TX4927_PCFG_SEL1: u64 = 0x100; pub const TX4927_PCFG_DMASEL_ALL: u64 = 0xff;
pub const TX4927_PCFG_DMASEL0_MASK: u64 = 3; pub const TX4927_PCFG_DMASEL1_MASK: u64 = 0xc; pub const TX4927_PCFG_DMASEL2_MASK: u64 = 0x30; pub const TX4927_PCFG_DMASEL3_MASK: u64 = 0xc0;
pub const TX4927_PCFG_DMASEL0_DRQ0: u64 = 0; pub const TX4927_PCFG_DMASEL0_SIO1: u64 = 1; pub const TX4927_PCFG_DMASEL0_ACL0: u64 = 2; pub const TX4927_PCFG_DMASEL0_ACL2: u64 = 3;
pub const TX4927_PCFG_DMASEL1_DRQ1: u64 = 0; pub const TX4927_PCFG_DMASEL1_SIO1: u64 = 4; pub const TX4927_PCFG_DMASEL1_ACL1: u64 = 8; pub const TX4927_PCFG_DMASEL1_ACL3: u64 = 0xc;
pub const TX4927_PCFG_DMASEL2_DRQ2: u64 = 0; pub const TX4927_PCFG_DMASEL2_SIO0: u64 = 0x10; pub const TX4927_PCFG_DMASEL2_ACL1: u64 = 0; pub const TX4927_PCFG_DMASEL2_ACL2: u64 = 0x20; pub const TX4927_PCFG_DMASEL2_ACL0: u64 = 0x30;
pub const TX4927_PCFG_DMASEL3_DRQ3: u64 = 0; pub const TX4927_PCFG_DMASEL3_SIO0: u64 = 0x40; pub const TX4927_PCFG_DMASEL3_ACL3: u64 = 0x80; pub const TX4927_PCFG_DMASEL3_ACL1: u64 = 0xc0;
pub const fn TX4927_PCFG_SDCLKDLY(d: u64) -> u64 { d << 28 }
pub const fn TX4927_PCFG_SDCLKEN(ch: u64) -> u64 { 0x00800000 << ch }
pub const fn TX4927_PCFG_PCICLKEN(ch: u64) -> u64 { 0x00010000 << ch }
pub const TX4927_CLKCTR_ACLCKD: u64 = 0x02000000; pub const TX4927_CLKCTR_PIOCKD: u64 = 0x01000000; pub const TX4927_CLKCTR_DMACKD: u64 = 0x00800000; pub const TX4927_CLKCTR_PCICKD: u64 = 0x00400000; pub const TX4927_CLKCTR_TM0CKD: u64 = 0x00100000; pub const TX4927_CLKCTR_TM1CKD: u64 = 0x00080000; pub const TX4927_CLKCTR_TM2CKD: u64 = 0x00040000; pub const TX4927_CLKCTR_SIO0CKD: u64 = 0x00020000; pub const TX4927_CLKCTR_SIO1CKD: u64 = 0x00010000; pub const TX4927_CLKCTR_ACLRST: u64 = 0x200; pub const TX4927_CLKCTR_PIORST: u64 = 0x100; pub const TX4927_CLKCTR_DMARST: u64 = 0x80; pub const TX4927_CLKCTR_PCIRST: u64 = 0x40; pub const TX4927_CLKCTR_TM0RST: u64 = 0x10; pub const TX4927_CLKCTR_TM1RST: u64 = 8; pub const TX4927_CLKCTR_TM2RST: u64 = 4; pub const TX4927_CLKCTR_SIO0RST: u64 = 2; pub const TX4927_CLKCTR_SIO1RST: u64 = 1;

extern "C" { pub fn __raw_readq(addr: *const u64) -> u64; pub fn ____raw_readq(addr: *const u64) -> u64; pub fn ____raw_writeq(value: u64, addr: *mut u64); }
pub unsafe fn TX4927_REV_PCODE(ptr: *const tx4927_ccfg_reg) -> u32 { (__raw_readq(&(*ptr).crir) >> 16) as u32 }
pub unsafe fn TX4927_SDRAMC_CR(ptr: *const tx4927_sdramc_reg, ch: usize) -> u64 { __raw_readq(&(*ptr).cr[ch]) }
pub unsafe fn TX4927_SDRAMC_BA(ptr: *const tx4927_sdramc_reg, ch: usize) -> u64 { (TX4927_SDRAMC_CR(ptr,ch) >> 49) << 21 }
pub unsafe fn TX4927_SDRAMC_SIZE(ptr: *const tx4927_sdramc_reg, ch: usize) -> u64 { (((TX4927_SDRAMC_CR(ptr,ch) >> 33) & 0x7fff) + 1) << 21 }
pub unsafe fn TX4927_EBUSC_CR(ptr: *const tx4927_ebusc_reg, ch: usize) -> u64 { __raw_readq(&(*ptr).cr[ch]) }
pub unsafe fn TX4927_EBUSC_BA(ptr: *const tx4927_ebusc_reg, ch: usize) -> u64 { (TX4927_EBUSC_CR(ptr,ch) >> 48) << 20 }
pub unsafe fn TX4927_EBUSC_SIZE(ptr: *const tx4927_ebusc_reg, ch: usize) -> u64 { 0x00100000u64 << ((TX4927_EBUSC_CR(ptr,ch) >> 8) & 0xf) }
pub unsafe fn TX4927_EBUSC_WIDTH(ptr: *const tx4927_ebusc_reg, ch: usize) -> u32 { 64 >> (((TX4927_EBUSC_CR(ptr,ch) >> 20) as u32) & 3) }

pub unsafe fn txx9_clear64(adr: *mut u64, bits: u64) { ____raw_writeq(____raw_readq(adr) & !bits, adr); }
pub unsafe fn txx9_set64(adr: *mut u64, bits: u64) { ____raw_writeq(____raw_readq(adr) | bits, adr); }
pub unsafe fn tx4927_ccfg_clear(adr: *mut u64, bits: u64) { ____raw_writeq(____raw_readq(adr) & !(TX4927_CCFG_W1CBITS | bits), adr); }
pub unsafe fn tx4927_ccfg_set(adr: *mut u64, bits: u64) { ____raw_writeq((____raw_readq(adr) & !TX4927_CCFG_W1CBITS) | bits, adr); }
pub unsafe fn tx4927_ccfg_change(adr: *mut u64, change: u64, new_value: u64) { ____raw_writeq((____raw_readq(adr) & !(TX4927_CCFG_W1CBITS | change)) | new_value, adr); }

extern "C" {
    pub fn tx4927_get_mem_size() -> u32; pub fn tx4927_wdt_init(); pub fn tx4927_setup(); pub fn tx4927_time_init(tmrnr: u32); pub fn tx4927_sio_init(sclk: u32, cts_mask: u32); pub fn tx4927_report_pciclk() -> i32; pub fn tx4927_pciclk66_setup() -> i32; pub fn tx4927_setup_pcierr_irq(); pub fn tx4927_irq_init(); pub fn tx4927_mtd_init(ch: i32); pub fn tx4927_dmac_init(memcpy_chan: i32); pub fn tx4927_aclc_init(dma_chan_out: u32, dma_chan_in: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
