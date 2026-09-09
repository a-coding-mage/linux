/* Translated from cvmx-pemx-defs.h. */

/* CVMX_ADD_IO_SEG is supplied by the surrounding platform bindings. */
macro_rules! CVMX_PEMX_BAR1_INDEXX { ($offset:expr, $block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C00000A8u64) + (((($offset) & 15) + (($block_id) & 1) * 0x200000u64) * 8) }; }
macro_rules! CVMX_PEMX_BAR2_MASK { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000130u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_BAR_CTL { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000128u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_BIST_STATUS { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000018u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_BIST_STATUS2 { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000420u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_CFG_RD { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000030u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_CFG_WR { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000028u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_CPL_LUT_VALID { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000098u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_CTL_STATUS { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000000u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_DBG_INFO { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000008u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_DBG_INFO_EN { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C00000A0u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_DIAG_STATUS { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000020u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_INB_READ_CREDITS { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000138u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_INT_ENB { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000410u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_INT_ENB_INT { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000418u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_INT_SUM { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000408u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_P2N_BAR0_START { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000080u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_P2N_BAR1_START { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000088u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_P2N_BAR2_START { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000090u64) + (($block_id) & 1) * 0x1000000u64 }; }
macro_rules! CVMX_PEMX_P2P_BARX_END { ($offset:expr, $block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000048u64) + (((($offset) & 3) + (($block_id) & 1) * 0x100000u64) * 16) }; }
macro_rules! CVMX_PEMX_P2P_BARX_START { ($offset:expr, $block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000040u64) + (((($offset) & 3) + (($block_id) & 1) * 0x100000u64) * 16) }; }
macro_rules! CVMX_PEMX_TLP_CREDITS { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x00011800C0000038u64) + (($block_id) & 1) * 0x1000000u64 }; }

/* C bitfields are represented by their exact 64-bit register word. The field
 * names and widths below document the source layout; accessors are intentionally
 * left to the platform's existing register conventions. */
macro_rules! pemx_reg { ($union:ident, $view:ident, $( $field:ident : $width:expr ),* $(,)?) => {
    #[repr(C)] #[derive(Copy, Clone)] pub struct $view { pub bits: u64 }
    #[repr(C)] pub union $union { pub u64: u64, pub s: $view }
}; }

pemx_reg!(cvmx_pemx_bar1_indexx, cvmx_pemx_bar1_indexx_s, reserved_20_63:44, addr_idx:16, ca:1, end_swp:2, addr_v:1);
pemx_reg!(cvmx_pemx_bar2_mask, cvmx_pemx_bar2_mask_s, reserved_38_63:26, mask:35, reserved_0_2:3);
pemx_reg!(cvmx_pemx_bar_ctl, cvmx_pemx_bar_ctl_s, reserved_7_63:57, bar1_siz:3, bar2_enb:1, bar2_esx:2, bar2_cax:1);
pemx_reg!(cvmx_pemx_bist_status, cvmx_pemx_bist_status_s, reserved_8_63:56, retry:1, rqdata0:1, rqdata1:1, rqdata2:1, rqdata3:1, rqhdr1:1, rqhdr0:1, sot:1);
pemx_reg!(cvmx_pemx_bist_status2, cvmx_pemx_bist_status2_s, reserved_10_63:54, e2p_cpl:1, e2p_n:1, e2p_p:1, peai_p2e:1, pef_tpf1:1, pef_tpf0:1, pef_tnf:1, pef_tcf1:1, pef_tc0:1, ppf:1);
pemx_reg!(cvmx_pemx_cfg_rd, cvmx_pemx_cfg_rd_s, data:32, addr:32);
pemx_reg!(cvmx_pemx_cfg_wr, cvmx_pemx_cfg_wr_s, data:32, addr:32);
pemx_reg!(cvmx_pemx_cpl_lut_valid, cvmx_pemx_cpl_lut_valid_s, reserved_32_63:32, tag:32);
pemx_reg!(cvmx_pemx_ctl_status, cvmx_pemx_ctl_status_s, reserved_48_63:16, auto_sd:1, dnum:5, pbus:8, reserved_32_33:2, cfg_rtry:16, reserved_12_15:4, pm_xtoff:1, pm_xpme:1, ob_p_cmd:1, reserved_7_8:2, nf_ecrc:1, dly_one:1, lnk_enb:1, ro_ctlp:1, fast_lm:1, inv_ecrc:1, inv_lcrc:1);
pemx_reg!(cvmx_pemx_dbg_info, cvmx_pemx_dbg_info_s, reserved_31_63:33, ecrc_e:1, rawwpp:1, racpp:1, ramtlp:1, rarwdns:1, caar:1, racca:1, racur:1, rauc:1, rqo:1, fcuv:1, rpe:1, fcpvwt:1, dpeoosd:1, rtwdle:1, rdwdle:1, mre:1, rte:1, acto:1, rvdm:1, rumep:1, rptamrc:1, rpmerc:1, rfemrc:1, rnfemrc:1, rcemrc:1, rpoison:1, recrce:1, rtlplle:1, rtlpmal:1, spoison:1);
pemx_reg!(cvmx_pemx_dbg_info_en, cvmx_pemx_dbg_info_en_s, reserved_31_63:33, ecrc_e:1, rawwpp:1, racpp:1, ramtlp:1, rarwdns:1, caar:1, racca:1, racur:1, rauc:1, rqo:1, fcuv:1, rpe:1, fcpvwt:1, dpeoosd:1, rtwdle:1, rdwdle:1, mre:1, rte:1, acto:1, rvdm:1, rumep:1, rptamrc:1, rpmerc:1, rfemrc:1, rnfemrc:1, rcemrc:1, rpoison:1, recrce:1, rtlplle:1, rtlpmal:1, spoison:1);
pemx_reg!(cvmx_pemx_diag_status, cvmx_pemx_diag_status_s, reserved_4_63:60, pm_dst:1, pm_stat:1, pm_en:1, aux_en:1);
pemx_reg!(cvmx_pemx_inb_read_credits, cvmx_pemx_inb_read_credits_s, reserved_6_63:58, num:6);
pemx_reg!(cvmx_pemx_int_enb, cvmx_pemx_int_enb_s, reserved_14_63:50, crs_dr:1, crs_er:1, rdlk:1, exc:1, un_bx:1, un_b2:1, un_b1:1, up_bx:1, up_b2:1, up_b1:1, pmem:1, pmei:1, se:1, aeri:1);
pemx_reg!(cvmx_pemx_int_enb_int, cvmx_pemx_int_enb_int_s, reserved_14_63:50, crs_dr:1, crs_er:1, rdlk:1, exc:1, un_bx:1, un_b2:1, un_b1:1, up_bx:1, up_b2:1, up_b1:1, pmem:1, pmei:1, se:1, aeri:1);
pemx_reg!(cvmx_pemx_int_sum, cvmx_pemx_int_sum_s, reserved_14_63:50, crs_dr:1, crs_er:1, rdlk:1, exc:1, un_bx:1, un_b2:1, un_b1:1, up_bx:1, up_b2:1, up_b1:1, pmem:1, pmei:1, se:1, aeri:1);
pemx_reg!(cvmx_pemx_p2n_bar0_start, cvmx_pemx_p2n_bar0_start_s, addr:50, reserved_0_13:14);
pemx_reg!(cvmx_pemx_p2n_bar1_start, cvmx_pemx_p2n_bar1_start_s, addr:38, reserved_0_25:26);
pemx_reg!(cvmx_pemx_p2n_bar2_start, cvmx_pemx_p2n_bar2_start_s, addr:23, reserved_0_40:41);
pemx_reg!(cvmx_pemx_p2p_barx_end, cvmx_pemx_p2p_barx_end_s, addr:52, reserved_0_11:12);
pemx_reg!(cvmx_pemx_p2p_barx_start, cvmx_pemx_p2p_barx_start_s, addr:52, reserved_0_11:12);

#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_pemx_tlp_credits_s { pub bits: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_pemx_tlp_credits_cn61xx { pub bits: u64 }
#[repr(C)] pub union cvmx_pemx_tlp_credits { pub u64: u64, pub s: cvmx_pemx_tlp_credits_s, pub cn61xx: cvmx_pemx_tlp_credits_cn61xx }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
