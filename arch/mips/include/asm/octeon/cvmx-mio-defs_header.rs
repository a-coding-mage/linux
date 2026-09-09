/* Rust translation of cvmx-mio-defs.h.  Bitfield alternatives are retained
 * as raw register words; callers may apply the documented masks/shifts. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

extern "Rust" {
    fn CVMX_ADD_IO_SEG(value: u64) -> u64;
}

macro_rules! reg { ($name:ident, $addr:expr) => { pub const $name: u64 = unsafe { CVMX_ADD_IO_SEG($addr) }; }; }
macro_rules! regx { ($name:ident, $addr:expr, $mask:expr, $scale:expr) => {
    #[inline] pub const fn $name(offset: u64) -> u64 { unsafe { CVMX_ADD_IO_SEG($addr) } + (offset & $mask) * $scale }
}; }

reg!(CVMX_MIO_BOOT_BIST_STAT, 0x00011800000000F8u64); reg!(CVMX_MIO_BOOT_COMP, 0x00011800000000B8u64);
regx!(CVMX_MIO_BOOT_DMA_CFGX,0x0001180000000100,3,8); regx!(CVMX_MIO_BOOT_DMA_INTX,0x0001180000000138,3,8);
regx!(CVMX_MIO_BOOT_DMA_INT_ENX,0x0001180000000150,3,8); regx!(CVMX_MIO_BOOT_DMA_TIMX,0x0001180000000120,3,8);
reg!(CVMX_MIO_BOOT_ERR,0x00011800000000A0); reg!(CVMX_MIO_BOOT_INT,0x00011800000000A8); reg!(CVMX_MIO_BOOT_LOC_ADR,0x0001180000000090);
regx!(CVMX_MIO_BOOT_LOC_CFGX,0x0001180000000080,1,8); reg!(CVMX_MIO_BOOT_LOC_DAT,0x0001180000000098); reg!(CVMX_MIO_BOOT_PIN_DEFS,0x00011800000000C0);
regx!(CVMX_MIO_BOOT_REG_CFGX,0x0001180000000000,7,8); regx!(CVMX_MIO_BOOT_REG_TIMX,0x0001180000000040,7,8); reg!(CVMX_MIO_BOOT_THR,0x00011800000000B0);

macro_rules! regs { ($(($n:ident,$a:expr)),* $(,)?) => { $(reg!($n,$a);)* }; }
regs![(CVMX_MIO_EMM_BUF_DAT,0x00011800000020E8),(CVMX_MIO_EMM_BUF_IDX,0x00011800000020E0),(CVMX_MIO_EMM_CFG,0x0001180000002000),(CVMX_MIO_EMM_CMD,0x0001180000002058),(CVMX_MIO_EMM_DMA,0x0001180000002050),(CVMX_MIO_EMM_INT,0x0001180000002078),(CVMX_MIO_EMM_INT_EN,0x0001180000002080),(CVMX_MIO_EMM_RCA,0x00011800000020A0),(CVMX_MIO_EMM_RSP_HI,0x0001180000002070),(CVMX_MIO_EMM_RSP_LO,0x0001180000002068),(CVMX_MIO_EMM_RSP_STS,0x0001180000002060),(CVMX_MIO_EMM_SAMPLE,0x0001180000002090),(CVMX_MIO_EMM_STS_MASK,0x0001180000002098),(CVMX_MIO_EMM_SWITCH,0x0001180000002048),(CVMX_MIO_EMM_WDOG,0x0001180000002088),(CVMX_MIO_PLL_CTL,0x0001180000001448),(CVMX_MIO_PLL_SETTING,0x0001180000001440),(CVMX_MIO_RST_BOOT,0x0001180000001600),(CVMX_MIO_RST_CFG,0x0001180000001610),(CVMX_MIO_RST_CKILL,0x0001180000001638),(CVMX_MIO_RST_DELAY,0x0001180000001608),(CVMX_MIO_RST_INT,0x0001180000001628),(CVMX_MIO_RST_INT_EN,0x0001180000001630)];
regx!(CVMX_MIO_EMM_MODEX,0x0001180000002008,3,8); regx!(CVMX_MIO_QLMX_CFG,0x0001180000001590,7,8); regx!(CVMX_MIO_RST_CNTLX,0x0001180000001648,3,8); regx!(CVMX_MIO_RST_CTLX,0x0001180000001618,1,8);

macro_rules! opaque_regs { ($($n:ident),* $(,)?) => { $(#[repr(C)] pub union $n { pub u64: u64, pub bytes: [u8; 8] })* }; }
opaque_regs!(cvmx_mio_boot_bist_stat,cvmx_mio_boot_comp,cvmx_mio_boot_dma_cfgx,cvmx_mio_boot_dma_intx,cvmx_mio_boot_dma_int_enx,cvmx_mio_boot_dma_timx,cvmx_mio_boot_err,cvmx_mio_boot_int,cvmx_mio_boot_loc_adr,cvmx_mio_boot_loc_cfgx,cvmx_mio_boot_loc_dat,cvmx_mio_boot_pin_defs,cvmx_mio_boot_reg_cfgx,cvmx_mio_boot_reg_timx,cvmx_mio_boot_thr,cvmx_mio_emm_buf_dat,cvmx_mio_emm_buf_idx,cvmx_mio_emm_cfg,cvmx_mio_emm_cmd,cvmx_mio_emm_dma,cvmx_mio_emm_int,cvmx_mio_emm_int_en,cvmx_mio_emm_modex,cvmx_mio_emm_rca,cvmx_mio_emm_rsp_hi,cvmx_mio_emm_rsp_lo,cvmx_mio_emm_rsp_sts,cvmx_mio_emm_sample,cvmx_mio_emm_sts_mask,cvmx_mio_emm_switch,cvmx_mio_emm_wdog,cvmx_mio_fus_bnk_datx,cvmx_mio_fus_dat0,cvmx_mio_fus_dat1,cvmx_mio_fus_dat2,cvmx_mio_fus_dat3,cvmx_mio_fus_ema,cvmx_mio_fus_pdf,cvmx_mio_fus_pll,cvmx_mio_fus_prog,cvmx_mio_fus_prog_times,cvmx_mio_fus_rcmd,cvmx_mio_fus_read_times,cvmx_mio_fus_repair_res0,cvmx_mio_fus_repair_res1,cvmx_mio_fus_repair_res2,cvmx_mio_fus_spr_repair_res,cvmx_mio_fus_spr_repair_sum,cvmx_mio_fus_tgg,cvmx_mio_fus_unlock,cvmx_mio_fus_wadr,cvmx_mio_gpio_comp,cvmx_mio_ndf_dma_cfg,cvmx_mio_ndf_dma_int,cvmx_mio_ndf_dma_int_en,cvmx_mio_pll_ctl,cvmx_mio_pll_setting,cvmx_mio_ptp_clock_cfg,cvmx_mio_ptp_clock_comp,cvmx_mio_ptp_clock_hi,cvmx_mio_ptp_clock_lo,cvmx_mio_ptp_evt_cnt,cvmx_mio_ptp_timestamp,cvmx_mio_qlmx_cfg,cvmx_mio_rst_boot,cvmx_mio_rst_cfg,cvmx_mio_rst_ckill,cvmx_mio_rst_cntlx,cvmx_mio_rst_ctlx,cvmx_mio_rst_delay,cvmx_mio_rst_int,cvmx_mio_rst_int_en,cvmx_mio_twsx_int,cvmx_mio_twsx_sw_twsi,cvmx_mio_twsx_sw_twsi_ext,cvmx_mio_twsx_twsi_sw,cvmx_mio_uartx_dlh,cvmx_mio_uartx_dll,cvmx_mio_uartx_far,cvmx_mio_uartx_fcr,cvmx_mio_uartx_htx,cvmx_mio_uartx_ier,cvmx_mio_uartx_iir,cvmx_mio_uartx_lcr,cvmx_mio_uartx_lsr,cvmx_mio_uartx_mcr,cvmx_mio_uartx_msr,cvmx_mio_uartx_rbr,cvmx_mio_uartx_rfl,cvmx_mio_uartx_rfw,cvmx_mio_uartx_sbcr,cvmx_mio_uartx_scr,cvmx_mio_uartx_sfe,cvmx_mio_uartx_srr,cvmx_mio_uartx_srt,cvmx_mio_uartx_srts,cvmx_mio_uartx_stt,cvmx_mio_uartx_tfl,cvmx_mio_uartx_tfr,cvmx_mio_uartx_thr,cvmx_mio_uartx_usr,cvmx_mio_uart2_dlh,cvmx_mio_uart2_dll,cvmx_mio_uart2_far,cvmx_mio_uart2_fcr,cvmx_mio_uart2_htx,cvmx_mio_uart2_ier,cvmx_mio_uart2_iir,cvmx_mio_uart2_lcr,cvmx_mio_uart2_lsr,cvmx_mio_uart2_mcr,cvmx_mio_uart2_msr,cvmx_mio_uart2_rbr,cvmx_mio_uart2_rfl,cvmx_mio_uart2_rfw,cvmx_mio_uart2_sbcr,cvmx_mio_uart2_scr,cvmx_mio_uart2_sfe,cvmx_mio_uart2_srr,cvmx_mio_uart2_srt,cvmx_mio_uart2_srts,cvmx_mio_uart2_stt,cvmx_mio_uart2_tfl,cvmx_mio_uart2_tfr,cvmx_mio_uart2_thr,cvmx_mio_uart2_usr);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
