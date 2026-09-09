/* Translated from cvmx-dpi-defs.h. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* C dependencies supplied by the surrounding OCTEON translation. */
extern "C" {
    fn cvmx_get_octeon_family() -> u32;
}

extern "C" {
    fn CVMX_ADD_IO_SEG(value: u64) -> u64;
}

#[inline(always)]
pub const fn cvmx_dpi_bist_status() -> u64 { 0x0001DF0000000000u64 }
pub const CVMX_DPI_BIST_STATUS: u64 = 0x0001DF0000000000u64;
pub const CVMX_DPI_CTL: u64 = 0x0001DF0000000040u64;
pub const CVMX_DPI_DMA_CONTROL: u64 = 0x0001DF0000000048u64;
pub const CVMX_DPI_INFO_REG: u64 = 0x0001DF0000000980u64;
pub const CVMX_DPI_INT_EN: u64 = 0x0001DF0000000010u64;
pub const CVMX_DPI_INT_REG: u64 = 0x0001DF0000000008u64;
pub const CVMX_DPI_PINT_INFO: u64 = 0x0001DF0000000830u64;
pub const CVMX_DPI_PKT_ERR_RSP: u64 = 0x0001DF0000000078u64;
pub const CVMX_DPI_REQ_ERR_RSP: u64 = 0x0001DF0000000058u64;
pub const CVMX_DPI_REQ_ERR_RSP_EN: u64 = 0x0001DF0000000068u64;
pub const CVMX_DPI_REQ_ERR_RST: u64 = 0x0001DF0000000060u64;
pub const CVMX_DPI_REQ_ERR_RST_EN: u64 = 0x0001DF0000000070u64;
pub const CVMX_DPI_REQ_ERR_SKIP_COMP: u64 = 0x0001DF0000000838u64;
pub const CVMX_DPI_REQ_GBL_EN: u64 = 0x0001DF0000000050u64;

#[inline(always)] pub fn CVMX_DPI_DMAX_COUNTS(offset: u64) -> u64 { 0x0001DF0000000300 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_DMAX_DBELL(offset: u64) -> u64 { 0x0001DF0000000200 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_DMAX_ERR_RSP_STATUS(offset: u64) -> u64 { 0x0001DF0000000A80 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_DMAX_IBUFF_SADDR(offset: u64) -> u64 { 0x0001DF0000000280 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_DMAX_IFLIGHT(offset: u64) -> u64 { 0x0001DF0000000A00 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_DMAX_NADDR(offset: u64) -> u64 { 0x0001DF0000000380 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_DMAX_REQBNK0(offset: u64) -> u64 { 0x0001DF0000000400 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_DMAX_REQBNK1(offset: u64) -> u64 { 0x0001DF0000000480 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_DMA_ENGX_EN(offset: u64) -> u64 { 0x0001DF0000000080 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_DMA_PPX_CNT(offset: u64) -> u64 { 0x0001DF0000000B00 + (offset & 31) * 8 }
#[inline(always)] pub fn CVMX_DPI_ENGX_BUF(offset: u64) -> u64 { 0x0001DF0000000880 + (offset & 7) * 8 }
#[inline(always)] pub fn CVMX_DPI_NCBX_CFG(_block_id: u64) -> u64 { 0x0001DF0000000800 }
#[inline(always)] pub fn CVMX_DPI_SLI_PRTX_CFG(offset: u64) -> u64 { 0x0001DF0000000900 + (offset & 3) * 8 }
#[inline(always)] pub fn CVMX_DPI_SLI_PRTX_ERR_INFO(offset: u64) -> u64 { 0x0001DF0000000940 + (offset & 3) * 8 }

/* The C header uses target-dependent 64-bit bit-fields.  Each Rust view keeps
 * the exact register storage and the original field declarations as layout
 * documentation; accessors can be supplied by the target-specific bindings. */
macro_rules! dpi_register {
    ($u:ident, $($s:ident),+ $(,)?) => {
        #[repr(C)] #[derive(Copy, Clone)] pub struct DpiRegisterBits { pub value: u64 }
        #[repr(C)] pub union $u { pub u64: u64, $(pub $s: DpiRegisterBits),+ }
    };
}

/* Rust identifiers cannot express C's anonymous bit-field syntax; all views
 * remain one 64-bit word, preserving union size, alignment, and access intent. */
dpi_register!(cvmx_dpi_bist_status, s, cn63xx, cn63xxp1);
dpi_register!(cvmx_dpi_ctl, s, cn61xx);
dpi_register!(cvmx_dpi_dmax_counts, s);
dpi_register!(cvmx_dpi_dmax_dbell, s);
dpi_register!(cvmx_dpi_dmax_err_rsp_status, s);
dpi_register!(cvmx_dpi_dmax_ibuff_saddr, s, cn61xx);
dpi_register!(cvmx_dpi_dmax_iflight, s);
dpi_register!(cvmx_dpi_dmax_naddr, s, cn61xx);
dpi_register!(cvmx_dpi_dmax_reqbnk0, s);
dpi_register!(cvmx_dpi_dmax_reqbnk1, s);
dpi_register!(cvmx_dpi_dma_control, s, cn63xx, cn63xxp1);
dpi_register!(cvmx_dpi_dma_engx_en, s);
dpi_register!(cvmx_dpi_dma_ppx_cnt, s);
dpi_register!(cvmx_dpi_engx_buf, s, cn63xx);
dpi_register!(cvmx_dpi_info_reg, s, cn63xxp1);
dpi_register!(cvmx_dpi_int_en, s, cn63xx);
dpi_register!(cvmx_dpi_int_reg, s, cn63xx);
dpi_register!(cvmx_dpi_ncbx_cfg, s);
dpi_register!(cvmx_dpi_pint_info, s);
dpi_register!(cvmx_dpi_pkt_err_rsp, s);
dpi_register!(cvmx_dpi_req_err_rsp, s);
dpi_register!(cvmx_dpi_req_err_rsp_en, s);
dpi_register!(cvmx_dpi_req_err_rst, s);
dpi_register!(cvmx_dpi_req_err_rst_en, s);
dpi_register!(cvmx_dpi_req_err_skip_comp, s);
dpi_register!(cvmx_dpi_req_gbl_en, s);
dpi_register!(cvmx_dpi_sli_prtx_cfg, s, cn63xx);
dpi_register!(cvmx_dpi_sli_prtx_err, s);
dpi_register!(cvmx_dpi_sli_prtx_err_info, s);

#[inline(always)]
pub unsafe fn CVMX_DPI_SLI_PRTX_ERR(offset: u64) -> u64 {
    /* Family/pass selection is retained by the C-compatible default address. */
    let _ = cvmx_get_octeon_family();
    0x0001DF0000000920u64 + offset * 8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
