/* SPDX-License-Identifier: GPL-2.0 */

// DCR accessors use PowerPC-specific inline assembly, preserved here as macros.
macro_rules! mfdcr {
    ($rn:expr) => {{
        let mut rval: usize;
        unsafe {
            core::arch::asm!("mfdcr {0},{1}", out(reg) rval, const $rn);
        }
        rval
    }};
}

macro_rules! mtdcr {
    ($rn:expr, $val:expr) => {{
        unsafe {
            core::arch::asm!("mtdcr {0},{1}", const $rn, in(reg) $val);
        }
    }};
}

macro_rules! mfdcrx {
    ($rn:expr) => {{
        let mut rval: usize;
        unsafe {
            core::arch::asm!("mfdcrx {0},{1}", out(reg) rval, in(reg) $rn);
        }
        rval
    }};
}

macro_rules! mtdcrx {
    ($rn:expr, $val:expr) => {{
        unsafe {
            core::arch::asm!("mtdcrx {0},{1}", in(reg) $rn, in(reg) $val);
        }
    }};
}

/* 440GP/440GX SDRAM controller DCRs */
pub const DCRN_SDRAM0_CFGADDR: usize = 0x010;
pub const DCRN_SDRAM0_CFGDATA: usize = 0x011;

macro_rules! SDRAM0_READ { ($offset:expr) => {{ mtdcr!(DCRN_SDRAM0_CFGADDR, $offset); mfdcr!(DCRN_SDRAM0_CFGDATA) }}; }
macro_rules! SDRAM0_WRITE { ($offset:expr, $data:expr) => {{ mtdcr!(DCRN_SDRAM0_CFGADDR, $offset); mtdcr!(DCRN_SDRAM0_CFGDATA, $data); }}; }

pub const SDRAM0_B0CR: usize = 0x40;
pub const SDRAM0_B1CR: usize = 0x44;
pub const SDRAM0_B2CR: usize = 0x48;
pub const SDRAM0_B3CR: usize = 0x4c;

pub static SDRAM_BXCR: [usize; 4] = [SDRAM0_B0CR, SDRAM0_B1CR, SDRAM0_B2CR, SDRAM0_B3CR];

pub const SDRAM_CONFIG_BANK_ENABLE: usize = 0x00000001;
pub const SDRAM_CONFIG_SIZE_MASK: usize = 0x000e0000;
macro_rules! SDRAM_CONFIG_BANK_SIZE { ($reg:expr) => { 0x00400000usize << (($reg & SDRAM_CONFIG_SIZE_MASK) >> 17) }; }

/* 440GP External Bus Controller (EBC) */
pub const DCRN_EBC0_CFGADDR: usize = 0x012;
pub const DCRN_EBC0_CFGDATA: usize = 0x013;
pub const EBC_NUM_BANKS: usize = 8;
pub const EBC_B0CR: usize = 0x00;
pub const EBC_B1CR: usize = 0x01;
pub const EBC_B2CR: usize = 0x02;
pub const EBC_B3CR: usize = 0x03;
pub const EBC_B4CR: usize = 0x04;
pub const EBC_B5CR: usize = 0x05;
pub const EBC_B6CR: usize = 0x06;
pub const EBC_B7CR: usize = 0x07;
macro_rules! EBC_BXCR { ($n:expr) => { $n }; }
pub const EBC_BXCR_BAS: usize = 0xfff00000;
pub const EBC_BXCR_BS: usize = 0x000e0000;
macro_rules! EBC_BXCR_BANK_SIZE { ($reg:expr) => { 0x100000usize << (($reg & EBC_BXCR_BS) >> 17) }; }
pub const EBC_BXCR_BU: usize = 0x00018000;
pub const EBC_BXCR_BU_OFF: usize = 0x00000000;
pub const EBC_BXCR_BU_RO: usize = 0x00008000;
pub const EBC_BXCR_BU_WO: usize = 0x00010000;
pub const EBC_BXCR_BU_RW: usize = 0x00018000;
pub const EBC_BXCR_BW: usize = 0x00006000;
pub const EBC_B0AP: usize = 0x10;
pub const EBC_B1AP: usize = 0x11;
pub const EBC_B2AP: usize = 0x12;
pub const EBC_B3AP: usize = 0x13;
pub const EBC_B4AP: usize = 0x14;
pub const EBC_B5AP: usize = 0x15;
pub const EBC_B6AP: usize = 0x16;
pub const EBC_B7AP: usize = 0x17;
macro_rules! EBC_BXAP { ($n:expr) => { 0x10usize + ($n) }; }
pub const EBC_BEAR: usize = 0x20;
pub const EBC_BESR: usize = 0x21;
pub const EBC_CFG: usize = 0x23;
pub const EBC_CID: usize = 0x24;

/* 440GP Clock, PM, chip control */
pub const DCRN_CPC0_SR: usize = 0x0b0;
pub const DCRN_CPC0_ER: usize = 0x0b1;
pub const DCRN_CPC0_FR: usize = 0x0b2;
pub const DCRN_CPC0_SYS0: usize = 0x0e0;
pub const CPC0_SYS0_TUNE: usize = 0xffc00000;
pub const CPC0_SYS0_FBDV_MASK: usize = 0x003c0000;
pub const CPC0_SYS0_FWDVA_MASK: usize = 0x00038000;
pub const CPC0_SYS0_FWDVB_MASK: usize = 0x00007000;
pub const CPC0_SYS0_OPDV_MASK: usize = 0x00000c00;
pub const CPC0_SYS0_EPDV_MASK: usize = 0x00000300;
/* Helper macros to compute the actual clock divider values from the encodings in the CPC0 register */
macro_rules! CPC0_SYS0_FBDV { ($reg:expr) => { (((((($reg & CPC0_SYS0_FBDV_MASK) >> 18).wrapping_sub(1)) & 0xf) + 1) }; }
macro_rules! CPC0_SYS0_FWDVA { ($reg:expr) => { 8usize - (($reg & CPC0_SYS0_FWDVA_MASK) >> 15) }; }
macro_rules! CPC0_SYS0_FWDVB { ($reg:expr) => { 8usize - (($reg & CPC0_SYS0_FWDVB_MASK) >> 12) }; }
macro_rules! CPC0_SYS0_OPDV { ($reg:expr) => { (($reg & CPC0_SYS0_OPDV_MASK) >> 10) + 1 }; }
macro_rules! CPC0_SYS0_EPDV { ($reg:expr) => { (($reg & CPC0_SYS0_EPDV_MASK) >> 8) + 1 }; }
pub const CPC0_SYS0_EXTSL: usize = 0x00000080;
pub const CPC0_SYS0_RW_MASK: usize = 0x00000060;
pub const CPC0_SYS0_RL: usize = 0x00000010;
pub const CPC0_SYS0_ZMIISL_MASK: usize = 0x0000000c;
pub const CPC0_SYS0_BYPASS: usize = 0x00000002;
pub const CPC0_SYS0_NTO1: usize = 0x00000001;
pub const DCRN_CPC0_SYS1: usize = 0x0e1;
pub const DCRN_CPC0_CUST0: usize = 0x0e2;
pub const DCRN_CPC0_CUST1: usize = 0x0e3;
pub const DCRN_CPC0_STRP0: usize = 0x0e4;
pub const DCRN_CPC0_STRP1: usize = 0x0e5;
pub const DCRN_CPC0_STRP2: usize = 0x0e6;
pub const DCRN_CPC0_STRP3: usize = 0x0e7;
pub const DCRN_CPC0_GPIO: usize = 0x0e8;
pub const DCRN_CPC0_PLB: usize = 0x0e9;
pub const DCRN_CPC0_CR1: usize = 0x0ea;
pub const DCRN_CPC0_CR0: usize = 0x0eb;
pub const CPC0_CR0_SWE: usize = 0x80000000;
pub const CPC0_CR0_CETE: usize = 0x40000000;
pub const CPC0_CR0_U1FCS: usize = 0x20000000;
pub const CPC0_CR0_U0DTE: usize = 0x10000000;
pub const CPC0_CR0_U0DRE: usize = 0x08000000;
pub const CPC0_CR0_U0DC: usize = 0x04000000;
pub const CPC0_CR0_U1DTE: usize = 0x02000000;
pub const CPC0_CR0_U1DRE: usize = 0x01000000;
pub const CPC0_CR0_U1DC: usize = 0x00800000;
pub const CPC0_CR0_U0EC: usize = 0x00400000;
pub const CPC0_CR0_U1EC: usize = 0x00200000;
pub const CPC0_CR0_UDIV_MASK: usize = 0x001f0000;
macro_rules! CPC0_CR0_UDIV { ($reg:expr) => { (($reg & CPC0_CR0_UDIV_MASK) >> 16) + 1 }; }
pub const DCRN_CPC0_MIRQ0: usize = 0x0ec;
pub const DCRN_CPC0_MIRQ1: usize = 0x0ed;
pub const DCRN_CPC0_JTAGID: usize = 0x0ef;

pub const DCRN_MAL0_CFG: usize = 0x180;
pub const MAL_RESET: usize = 0x80000000;

/* 440EP Clock/Power-on Reset regs */
pub const DCRN_CPR0_ADDR: usize = 0xc;
pub const DCRN_CPR0_DATA: usize = 0xd;
pub const CPR0_PLLD0: usize = 0x60;
pub const CPR0_OPBD0: usize = 0xc0;
pub const CPR0_PERD0: usize = 0xe0;
pub const CPR0_PRIMBD0: usize = 0xa0;
pub const CPR0_SCPID: usize = 0x120;
pub const CPR0_PLLC0: usize = 0x40;

/* 440GX/405EX Clock Control reg */
pub const DCRN_CPR0_CLKUPD: usize = 0x020;
pub const DCRN_CPR0_PLLC: usize = 0x040;
pub const DCRN_CPR0_PLLD: usize = 0x060;
pub const DCRN_CPR0_PRIMAD: usize = 0x080;
pub const DCRN_CPR0_PRIMBD: usize = 0x0a0;
pub const DCRN_CPR0_OPBD: usize = 0x0c0;
pub const DCRN_CPR0_PERD: usize = 0x0e0;
pub const DCRN_CPR0_MALD: usize = 0x100;

pub const DCRN_SDR0_CONFIG_ADDR: usize = 0xe;
pub const DCRN_SDR0_CONFIG_DATA: usize = 0xf;

/* SDR read/write helper macros */
macro_rules! SDR0_READ { ($offset:expr) => {{ mtdcr!(DCRN_SDR0_CONFIG_ADDR, $offset); mfdcr!(DCRN_SDR0_CONFIG_DATA) }}; }
macro_rules! SDR0_WRITE { ($offset:expr, $data:expr) => {{ mtdcr!(DCRN_SDR0_CONFIG_ADDR, $offset); mtdcr!(DCRN_SDR0_CONFIG_DATA, $data); }}; }

pub const DCRN_SDR0_UART0: usize = 0x0120;
pub const DCRN_SDR0_UART1: usize = 0x0121;
pub const DCRN_SDR0_UART2: usize = 0x0122;
pub const DCRN_SDR0_UART3: usize = 0x0123;

/* CPRs read/write helper macros - based off include/asm-ppc/ibm44x.h */
pub const DCRN_CPR0_CFGADDR: usize = 0xc;
pub const DCRN_CPR0_CFGDATA: usize = 0xd;

macro_rules! CPR0_READ { ($offset:expr) => {{ mtdcr!(DCRN_CPR0_CFGADDR, $offset); mfdcr!(DCRN_CPR0_CFGDATA) }}; }
macro_rules! CPR0_WRITE { ($offset:expr, $data:expr) => {{ mtdcr!(DCRN_CPR0_CFGADDR, $offset); mtdcr!(DCRN_CPR0_CFGDATA, $data); }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
