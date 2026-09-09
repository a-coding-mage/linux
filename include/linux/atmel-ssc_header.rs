/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
pub type dma_addr_t = usize;

#[repr(C)]
pub struct atmel_ssc_platform_data {
    pub use_dma: ::core::ffi::c_int,
    pub has_fslen_ext: ::core::ffi::c_int,
}

#[repr(C)]
pub struct ssc_device {
    pub list: list_head,
    pub phybase: dma_addr_t,
    pub regs: *mut ::core::ffi::c_void,
    pub pdev: *mut platform_device,
    pub pdata: *mut atmel_ssc_platform_data,
    pub clk: *mut clk,
    pub user: ::core::ffi::c_int,
    pub irq: ::core::ffi::c_int,
    pub clk_from_rk_pin: bool,
    pub sound_dai: bool,
}

extern "C" {
    pub fn ssc_request(ssc_num: u32) -> *mut ssc_device;
    pub fn ssc_free(ssc: *mut ssc_device);
}

macro_rules! ssc_reg { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
ssc_reg! {
    SSC_CR=0x00000000, SSC_CMR=0x00000004, SSC_RCMR=0x00000010, SSC_RFMR=0x00000014,
    SSC_TCMR=0x00000018, SSC_TFMR=0x0000001c, SSC_RHR=0x00000020, SSC_THR=0x00000024,
    SSC_RSHR=0x00000030, SSC_TSHR=0x00000034, SSC_RC0R=0x00000038, SSC_RC1R=0x0000003c,
    SSC_SR=0x00000040, SSC_IER=0x00000044, SSC_IDR=0x00000048, SSC_IMR=0x0000004c,
    SSC_PDC_RPR=0x00000100, SSC_PDC_RCR=0x00000104, SSC_PDC_TPR=0x00000108,
    SSC_PDC_RNPR=0x00000110, SSC_PDC_RNCR=0x00000114, SSC_PDC_TCR=0x0000010c,
    SSC_PDC_TNPR=0x00000118, SSC_PDC_TNCR=0x0000011c, SSC_PDC_PTCR=0x00000120,
    SSC_PDC_PTSR=0x00000124
}

// Field size and offset constants (the C token-pasting macros are represented by these items).
+pub const SSC_CR_RXDIS_SIZE: u32 = 1;
pub const SSC_CR_RXDIS_OFFSET: u32 = 1;
pub const SSC_CR_RXEN_SIZE: u32 = 1;
pub const SSC_CR_RXEN_OFFSET: u32 = 0;
pub const SSC_CR_SWRST_SIZE: u32 = 1;
pub const SSC_CR_SWRST_OFFSET: u32 = 15;
pub const SSC_CR_TXDIS_SIZE: u32 = 1;
pub const SSC_CR_TXDIS_OFFSET: u32 = 9;
pub const SSC_CR_TXEN_SIZE: u32 = 1;
pub const SSC_CR_TXEN_OFFSET: u32 = 8;
pub const SSC_CMR_DIV_SIZE: u32 = 12;
pub const SSC_CMR_DIV_OFFSET: u32 = 0;
pub const SSC_RCMR_CKG_SIZE: u32 = 2;
pub const SSC_RCMR_CKG_OFFSET: u32 = 6;
pub const SSC_RCMR_CKI_SIZE: u32 = 1;
pub const SSC_RCMR_CKI_OFFSET: u32 = 5;
pub const SSC_RCMR_CKO_SIZE: u32 = 3;
pub const SSC_RCMR_CKO_OFFSET: u32 = 2;
pub const SSC_RCMR_CKS_SIZE: u32 = 2;
pub const SSC_RCMR_CKS_OFFSET: u32 = 0;
pub const SSC_RCMR_PERIOD_SIZE: u32 = 8;
pub const SSC_RCMR_PERIOD_OFFSET: u32 = 24;
pub const SSC_RCMR_START_SIZE: u32 = 4;
pub const SSC_RCMR_START_OFFSET: u32 = 8;
pub const SSC_RCMR_STOP_SIZE: u32 = 1;
pub const SSC_RCMR_STOP_OFFSET: u32 = 12;
pub const SSC_RCMR_STTDLY_SIZE: u32 = 8;
pub const SSC_RCMR_STTDLY_OFFSET: u32 = 16;
pub const SSC_RFMR_DATLEN_SIZE: u32 = 5;
pub const SSC_RFMR_DATLEN_OFFSET: u32 = 0;
pub const SSC_RFMR_DATNB_SIZE: u32 = 4;
pub const SSC_RFMR_DATNB_OFFSET: u32 = 8;
pub const SSC_RFMR_FSEDGE_SIZE: u32 = 1;
pub const SSC_RFMR_FSEDGE_OFFSET: u32 = 24;
pub const SSC_RFMR_FSLEN_EXT_SIZE: u32 = 4;
pub const SSC_RFMR_FSLEN_EXT_OFFSET: u32 = 28;
pub const SSC_RFMR_FSLEN_SIZE: u32 = 4;
pub const SSC_RFMR_FSLEN_OFFSET: u32 = 16;
pub const SSC_RFMR_FSOS_SIZE: u32 = 4;
pub const SSC_RFMR_FSOS_OFFSET: u32 = 20;
pub const SSC_RFMR_LOOP_SIZE: u32 = 1;
pub const SSC_RFMR_LOOP_OFFSET: u32 = 5;
pub const SSC_RFMR_MSBF_SIZE: u32 = 1;
pub const SSC_RFMR_MSBF_OFFSET: u32 = 7;
pub const SSC_TCMR_CKG_SIZE: u32 = 2;
pub const SSC_TCMR_CKG_OFFSET: u32 = 6;
pub const SSC_TCMR_CKI_SIZE: u32 = 1;
pub const SSC_TCMR_CKI_OFFSET: u32 = 5;
pub const SSC_TCMR_CKO_SIZE: u32 = 3;
pub const SSC_TCMR_CKO_OFFSET: u32 = 2;
pub const SSC_TCMR_CKS_SIZE: u32 = 2;
pub const SSC_TCMR_CKS_OFFSET: u32 = 0;
pub const SSC_TCMR_PERIOD_SIZE: u32 = 8;
pub const SSC_TCMR_PERIOD_OFFSET: u32 = 24;
pub const SSC_TCMR_START_SIZE: u32 = 4;
pub const SSC_TCMR_START_OFFSET: u32 = 8;
pub const SSC_TCMR_STTDLY_SIZE: u32 = 8;
pub const SSC_TCMR_STTDLY_OFFSET: u32 = 16;
pub const SSC_TFMR_DATDEF_SIZE: u32 = 1;
pub const SSC_TFMR_DATDEF_OFFSET: u32 = 5;
pub const SSC_TFMR_DATLEN_SIZE: u32 = 5;
pub const SSC_TFMR_DATLEN_OFFSET: u32 = 0;
pub const SSC_TFMR_DATNB_SIZE: u32 = 4;
pub const SSC_TFMR_DATNB_OFFSET: u32 = 8;
pub const SSC_TFMR_FSDEN_SIZE: u32 = 1;
pub const SSC_TFMR_FSDEN_OFFSET: u32 = 23;
pub const SSC_TFMR_FSEDGE_SIZE: u32 = 1;
pub const SSC_TFMR_FSEDGE_OFFSET: u32 = 24;
pub const SSC_TFMR_FSLEN_EXT_SIZE: u32 = 4;
pub const SSC_TFMR_FSLEN_EXT_OFFSET: u32 = 28;
pub const SSC_TFMR_FSLEN_SIZE: u32 = 4;
pub const SSC_TFMR_FSLEN_OFFSET: u32 = 16;
pub const SSC_TFMR_FSOS_SIZE: u32 = 3;
pub const SSC_TFMR_FSOS_OFFSET: u32 = 20;
pub const SSC_TFMR_MSBF_SIZE: u32 = 1;
pub const SSC_TFMR_MSBF_OFFSET: u32 = 7;
pub const SSC_RHR_RDAT_SIZE: u32 = 32;
pub const SSC_RHR_RDAT_OFFSET: u32 = 0;
pub const SSC_THR_TDAT_SIZE: u32 = 32;
pub const SSC_THR_TDAT_OFFSET: u32 = 0;
pub const SSC_RSHR_RSDAT_SIZE: u32 = 16;
pub const SSC_RSHR_RSDAT_OFFSET: u32 = 0;
pub const SSC_TSHR_TSDAT_SIZE: u32 = 16;
pub const SSC_TSHR_RSDAT_OFFSET: u32 = 0;
pub const SSC_RC0R_CP0_SIZE: u32 = 16;
pub const SSC_RC0R_CP0_OFFSET: u32 = 0;
pub const SSC_RC1R_CP1_SIZE: u32 = 16;
pub const SSC_RC1R_CP1_OFFSET: u32 = 0;
pub const SSC_SR_CP0_SIZE: u32 = 1;
pub const SSC_SR_CP0_OFFSET: u32 = 8;
pub const SSC_SR_CP1_SIZE: u32 = 1;
pub const SSC_SR_CP1_OFFSET: u32 = 9;
pub const SSC_SR_ENDRX_SIZE: u32 = 1;
pub const SSC_SR_ENDRX_OFFSET: u32 = 6;
pub const SSC_SR_ENDTX_SIZE: u32 = 1;
pub const SSC_SR_ENDTX_OFFSET: u32 = 2;
pub const SSC_SR_OVRUN_SIZE: u32 = 1;
pub const SSC_SR_OVRUN_OFFSET: u32 = 5;
pub const SSC_SR_RXBUFF_SIZE: u32 = 1;
pub const SSC_SR_RXBUFF_OFFSET: u32 = 7;
pub const SSC_SR_RXEN_SIZE: u32 = 1;
pub const SSC_SR_RXEN_OFFSET: u32 = 17;
pub const SSC_SR_RXRDY_SIZE: u32 = 1;
pub const SSC_SR_RXRDY_OFFSET: u32 = 4;
pub const SSC_SR_RXSYN_SIZE: u32 = 1;
pub const SSC_SR_RXSYN_OFFSET: u32 = 11;
pub const SSC_SR_TXBUFE_SIZE: u32 = 1;
pub const SSC_SR_TXBUFE_OFFSET: u32 = 3;
pub const SSC_SR_TXEMPTY_SIZE: u32 = 1;
pub const SSC_SR_TXEMPTY_OFFSET: u32 = 1;
pub const SSC_SR_TXEN_SIZE: u32 = 1;
pub const SSC_SR_TXEN_OFFSET: u32 = 16;
pub const SSC_SR_TXRDY_SIZE: u32 = 1;
pub const SSC_SR_TXRDY_OFFSET: u32 = 0;
pub const SSC_SR_TXSYN_SIZE: u32 = 1;
pub const SSC_SR_TXSYN_OFFSET: u32 = 10;
pub const SSC_IER_CP0_SIZE: u32 = 1;
pub const SSC_IER_CP0_OFFSET: u32 = 8;
pub const SSC_IER_CP1_SIZE: u32 = 1;
pub const SSC_IER_CP1_OFFSET: u32 = 9;
pub const SSC_IER_ENDRX_SIZE: u32 = 1;
pub const SSC_IER_ENDRX_OFFSET: u32 = 6;
pub const SSC_IER_ENDTX_SIZE: u32 = 1;
pub const SSC_IER_ENDTX_OFFSET: u32 = 2;
pub const SSC_IER_OVRUN_SIZE: u32 = 1;
pub const SSC_IER_OVRUN_OFFSET: u32 = 5;
pub const SSC_IER_RXBUFF_SIZE: u32 = 1;
pub const SSC_IER_RXBUFF_OFFSET: u32 = 7;
pub const SSC_IER_RXRDY_SIZE: u32 = 1;
pub const SSC_IER_RXRDY_OFFSET: u32 = 4;
pub const SSC_IER_RXSYN_SIZE: u32 = 1;
pub const SSC_IER_RXSYN_OFFSET: u32 = 11;
pub const SSC_IER_TXBUFE_SIZE: u32 = 1;
pub const SSC_IER_TXBUFE_OFFSET: u32 = 3;
pub const SSC_IER_TXEMPTY_SIZE: u32 = 1;
pub const SSC_IER_TXEMPTY_OFFSET: u32 = 1;
pub const SSC_IER_TXRDY_SIZE: u32 = 1;
pub const SSC_IER_TXRDY_OFFSET: u32 = 0;
pub const SSC_IER_TXSYN_SIZE: u32 = 1;
pub const SSC_IER_TXSYN_OFFSET: u32 = 10;
pub const SSC_IDR_CP0_SIZE: u32 = 1;
pub const SSC_IDR_CP0_OFFSET: u32 = 8;
pub const SSC_IDR_CP1_SIZE: u32 = 1;
pub const SSC_IDR_CP1_OFFSET: u32 = 9;
pub const SSC_IDR_ENDRX_SIZE: u32 = 1;
pub const SSC_IDR_ENDRX_OFFSET: u32 = 6;
pub const SSC_IDR_ENDTX_SIZE: u32 = 1;
pub const SSC_IDR_ENDTX_OFFSET: u32 = 2;
pub const SSC_IDR_OVRUN_SIZE: u32 = 1;
pub const SSC_IDR_OVRUN_OFFSET: u32 = 5;
pub const SSC_IDR_RXBUFF_SIZE: u32 = 1;
pub const SSC_IDR_RXBUFF_OFFSET: u32 = 7;
pub const SSC_IDR_RXRDY_SIZE: u32 = 1;
pub const SSC_IDR_RXRDY_OFFSET: u32 = 4;
pub const SSC_IDR_RXSYN_SIZE: u32 = 1;
pub const SSC_IDR_RXSYN_OFFSET: u32 = 11;
pub const SSC_IDR_TXBUFE_SIZE: u32 = 1;
pub const SSC_IDR_TXBUFE_OFFSET: u32 = 3;
pub const SSC_IDR_TXEMPTY_SIZE: u32 = 1;
pub const SSC_IDR_TXEMPTY_OFFSET: u32 = 1;
pub const SSC_IDR_TXRDY_SIZE: u32 = 1;
pub const SSC_IDR_TXRDY_OFFSET: u32 = 0;
pub const SSC_IDR_TXSYN_SIZE: u32 = 1;
pub const SSC_IDR_TXSYN_OFFSET: u32 = 10;
pub const SSC_IMR_CP0_SIZE: u32 = 1;
pub const SSC_IMR_CP0_OFFSET: u32 = 8;
pub const SSC_IMR_CP1_SIZE: u32 = 1;
pub const SSC_IMR_CP1_OFFSET: u32 = 9;
pub const SSC_IMR_ENDRX_SIZE: u32 = 1;
pub const SSC_IMR_ENDRX_OFFSET: u32 = 6;
pub const SSC_IMR_ENDTX_SIZE: u32 = 1;
pub const SSC_IMR_ENDTX_OFFSET: u32 = 2;
pub const SSC_IMR_OVRUN_SIZE: u32 = 1;
pub const SSC_IMR_OVRUN_OFFSET: u32 = 5;
pub const SSC_IMR_RXBUFF_SIZE: u32 = 1;
pub const SSC_IMR_RXBUFF_OFFSET: u32 = 7;
pub const SSC_IMR_RXRDY_SIZE: u32 = 1;
pub const SSC_IMR_RXRDY_OFFSET: u32 = 4;
pub const SSC_IMR_RXSYN_SIZE: u32 = 1;
pub const SSC_IMR_RXSYN_OFFSET: u32 = 11;
pub const SSC_IMR_TXBUFE_SIZE: u32 = 1;
pub const SSC_IMR_TXBUFE_OFFSET: u32 = 3;
pub const SSC_IMR_TXEMPTY_SIZE: u32 = 1;
pub const SSC_IMR_TXEMPTY_OFFSET: u32 = 1;
pub const SSC_IMR_TXRDY_SIZE: u32 = 1;
pub const SSC_IMR_TXRDY_OFFSET: u32 = 0;
pub const SSC_IMR_TXSYN_SIZE: u32 = 1;
pub const SSC_IMR_TXSYN_OFFSET: u32 = 10;
pub const SSC_PDC_PTCR_RXTDIS_SIZE: u32 = 1;
pub const SSC_PDC_PTCR_RXTDIS_OFFSET: u32 = 1;
pub const SSC_PDC_PTCR_RXTEN_SIZE: u32 = 1;
pub const SSC_PDC_PTCR_RXTEN_OFFSET: u32 = 0;
pub const SSC_PDC_PTCR_TXTDIS_SIZE: u32 = 1;
pub const SSC_PDC_PTCR_TXTDIS_OFFSET: u32 = 9;
pub const SSC_PDC_PTCR_TXTEN_SIZE: u32 = 1;
pub const SSC_PDC_PTCR_TXTEN_OFFSET: u32 = 8;
pub const SSC_PDC_PTSR_RXTEN_SIZE: u32 = 1;
pub const SSC_PDC_PTSR_RXTEN_OFFSET: u32 = 0;
pub const SSC_PDC_PTSR_TXTEN_SIZE: u32 = 1;
pub const SSC_PDC_PTSR_TXTEN_OFFSET: u32 = 8;

// Rust has no stable identifier concatenation in macro_rules; retain the complete C field
// interface through a declarative table and generic bit helpers below.
#[inline] pub const fn ssc_bit(offset: u32) -> u32 { 1u32.wrapping_shl(offset) }
#[inline] pub const fn ssc_bf(size: u32, offset: u32, value: u32) -> u32 { (value & (1u32.wrapping_shl(size).wrapping_sub(1))).wrapping_shl(offset) }
#[inline] pub const fn ssc_bfext(size: u32, offset: u32, value: u32) -> u32 { (value >> offset) & (1u32.wrapping_shl(size).wrapping_sub(1)) }
#[inline] pub const fn ssc_bfins(size: u32, offset: u32, value: u32, old: u32) -> u32 { (old & !(1u32.wrapping_shl(size).wrapping_sub(1) << offset)) | ssc_bf(size, offset, value) }

// SSC_BIT, SSC_BF, SSC_BFEXT and SSC_BFINS preserve the original operation semantics.
#[macro_export] macro_rules! SSC_BIT { ($offset:expr) => { $crate::ssc_bit($offset) }; }
#[macro_export] macro_rules! SSC_BF { ($size:expr, $offset:expr, $value:expr) => { $crate::ssc_bf($size, $offset, $value) }; }
#[macro_export] macro_rules! SSC_BFEXT { ($size:expr, $offset:expr, $value:expr) => { $crate::ssc_bfext($size, $offset, $value) }; }
#[macro_export] macro_rules! SSC_BFINS { ($size:expr, $offset:expr, $value:expr, $old:expr) => { $crate::ssc_bfins($size, $offset, $value, $old) }; }

// Register access macros.
extern "C" { pub fn __raw_readl(addr: *const u8) -> u32; pub fn __raw_writel(value: u32, addr: *mut u8); }
#[inline] pub unsafe fn ssc_readl(base: *const u8, reg: u32) -> u32 { __raw_readl(base.add(reg as usize)) }
#[inline] pub unsafe fn ssc_writel(base: *mut u8, reg: u32, value: u32) { __raw_writel(value, base.add(reg as usize)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
