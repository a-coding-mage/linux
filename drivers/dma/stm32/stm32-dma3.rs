// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of the STM32 DMA3 controller implementation.
// External kernel types, helpers, and registration interfaces are supplied by the
// surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const STM32_DMA3_SECCFGR: u32 = 0x00;
pub const STM32_DMA3_PRIVCFGR: u32 = 0x04;
pub const STM32_DMA3_RCFGLOCKR: u32 = 0x08;
pub const STM32_DMA3_MISR: u32 = 0x0c;
pub const STM32_DMA3_SMISR: u32 = 0x10;
pub const STM32_DMA3_CLBAR: u32 = 0x50;
pub const STM32_DMA3_CCIDCFGR: u32 = 0x54;
pub const STM32_DMA3_CSEMCR: u32 = 0x58;
pub const STM32_DMA3_CFCR: u32 = 0x5c;
pub const STM32_DMA3_CSR: u32 = 0x60;
pub const STM32_DMA3_CCR: u32 = 0x64;
pub const STM32_DMA3_CTR1: u32 = 0x90;
pub const STM32_DMA3_CTR2: u32 = 0x94;
pub const STM32_DMA3_CBR1: u32 = 0x98;
pub const STM32_DMA3_CSAR: u32 = 0x9c;
pub const STM32_DMA3_CDAR: u32 = 0xa0;
pub const STM32_DMA3_CLLR: u32 = 0xcc;
pub const STM32_DMA3_HWCFGR13: u32 = 0xfc0;
pub const STM32_DMA3_HWCFGR12: u32 = 0xfc4;
pub const STM32_DMA3_HWCFGR4: u32 = 0xfe4;
pub const STM32_DMA3_HWCFGR3: u32 = 0xfe8;
pub const STM32_DMA3_HWCFGR2: u32 = 0xfec;
pub const STM32_DMA3_HWCFGR1: u32 = 0xff0;
pub const STM32_DMA3_VERR: u32 = 0xff4;

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(h: u32, l: u32) -> u32 { ((1u32 << (h-l+1)) - 1) << l }
const fn reg(base: u32, x: u32) -> u32 { base + 0x80 * x }

pub const CLBAR_LBA: u32 = genmask(31, 16);
pub const CCIDCFGR_CFEN: u32 = bit(0);
pub const CCIDCFGR_SEM_EN: u32 = bit(1);
pub const CCIDCFGR_SCID: u32 = genmask(5, 4);
pub const CSEMCR_SEM_MUTEX: u32 = bit(0);
pub const CSEMCR_SEM_CCID: u32 = genmask(5, 4);
pub const CSR_IDLEF: u32 = bit(0);
pub const CSR_ALL_F: u32 = genmask(13, 8);
pub const CSR_FIFOL: u32 = genmask(24, 16);
pub const CCR_EN: u32 = bit(0);
pub const CCR_RESET: u32 = bit(1);
pub const CCR_SUSP: u32 = bit(2);
pub const CCR_TCIE: u32 = bit(8);
pub const CCR_ALLIE: u32 = genmask(13, 8);
pub const CCR_LSM: u32 = bit(16);
pub const CCR_LAP: u32 = bit(17);
pub const CCR_PRIO: u32 = genmask(23, 22);
pub const CTR1_SINC: u32 = bit(3);
pub const CTR1_SBL_1: u32 = genmask(9, 4);
pub const CTR1_DINC: u32 = bit(19);
pub const CTR1_DBL_1: u32 = genmask(25, 20);
pub const CTR1_SDW_LOG2: u32 = genmask(1, 0);
pub const CTR1_PAM: u32 = genmask(12, 11);
pub const CTR1_SAP: u32 = bit(14);
pub const CTR1_DDW_LOG2: u32 = genmask(17, 16);
pub const CTR1_DAP: u32 = bit(30);
pub const CTR2_REQSEL: u32 = genmask(7, 0);
pub const CTR2_SWREQ: u32 = bit(9);
pub const CTR2_DREQ: u32 = bit(10);
pub const CTR2_BREQ: u32 = bit(11);
pub const CTR2_PFREQ: u32 = bit(12);
pub const CTR2_TCEM: u32 = genmask(31, 30);
pub const CBR1_BNDT: u32 = genmask(15, 0);
pub const CLLR_LA: u32 = genmask(15, 2);
pub const CLLR_ULL: u32 = bit(16);
pub const CLLR_UDA: u32 = bit(27);
pub const CLLR_USA: u32 = bit(28);
pub const CLLR_UB1: u32 = bit(29);
pub const CLLR_UT2: u32 = bit(30);
pub const CLLR_UT1: u32 = bit(31);

#[repr(u32)] pub enum ccidcfgr_cid { CCIDCFGR_CID0, CCIDCFGR_CID1, CCIDCFGR_CID2 }
#[repr(u32)] pub enum ccr_prio { CCR_PRIO_LOW, CCR_PRIO_MID, CCR_PRIO_HIGH, CCR_PRIO_VERY_HIGH }
#[repr(u32)] pub enum ctr1_dw { CTR1_DW_BYTE, CTR1_DW_HWORD, CTR1_DW_WORD, CTR1_DW_DWORD }
#[repr(u32)] pub enum ctr1_pam { CTR1_PAM_0S_LT, CTR1_PAM_SE_RT, CTR1_PAM_PACK_UNPACK }
#[repr(u32)] pub enum ctr2_tcem { CTR2_TCEM_BLOCK, CTR2_TCEM_REPEAT_BLOCK, CTR2_TCEM_LLI, CTR2_TCEM_CHANNEL }
#[repr(u32)] pub enum stm32_dma3_master_ports { AXI64, AHB32, AHB32_AHB32, AXI64_AHB32, AXI64_AXI64, AXI128_AHB32 }
#[repr(u32)] pub enum stm32_dma3_port_data_width { DW_32, DW_64, DW_128, DW_INVALID }

#[repr(C, packed(1), align(32))]
pub struct stm32_dma3_hwdesc { pub ctr1:u32, pub ctr2:u32, pub cbr1:u32, pub csar:u64, pub cdar:u64, pub cllr:u32 }
#[repr(C)] pub struct stm32_dma3_lli { pub hwdesc:*mut stm32_dma3_hwdesc, pub hwdesc_addr:u64 }
#[repr(C)] pub struct stm32_dma3_dt_conf { pub ch_id:u32, pub req_line:u32, pub ch_conf:u32, pub tr_conf:u32 }
#[repr(C)] pub struct stm32_dma3_swdesc { pub vdesc:*mut c_void, pub ccr:u32, pub cyclic:bool, pub lli_size:u32, pub lli:*mut stm32_dma3_lli }

pub const STM32_DMA3_MAX_BLOCK_SIZE: u32 = CBR1_BNDT & !63;
pub const STM32_DMA3_MAX_BURST_LEN: u32 = 1 + 63;

// The remaining driver operations retain the C driver's externally supplied
// kernel ABI and are intentionally declared here for binding integration.
extern "C" {
    fn stm32_dma3_probe(pdev: *mut c_void) -> i32;
    fn stm32_dma3_remove(pdev: *mut c_void);
    fn stm32_dma3_runtime_suspend(dev: *mut c_void) -> i32;
    fn stm32_dma3_runtime_resume(dev: *mut c_void) -> i32;
    fn stm32_dma3_pm_suspend(dev: *mut c_void) -> i32;
    fn stm32_dma3_pm_resume(dev: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
