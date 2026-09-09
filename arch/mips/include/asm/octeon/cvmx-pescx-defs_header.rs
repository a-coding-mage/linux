/* Translation of cvmx-pescx-defs.h. C bit-fields are represented as named
 * u64 members; the declared widths and ordering are retained in the macro
 * invocations below, with raw register access available through each union. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
macro_rules! cvmx_bitfield_struct {
    ($name:ident { $( $field:ident : $width:literal ),* $(,)? }) => {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $name { $( pub $field: u64, )* }
    };
}

#[inline] pub const fn CVMX_PESCX_BIST_STATUS(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000018u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_BIST_STATUS2(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000418u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_CFG_RD(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000030u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_CFG_WR(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000028u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_CPL_LUT_VALID(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000098u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_CTL_STATUS(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000000u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_CTL_STATUS2(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000400u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_DBG_INFO(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000008u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_DBG_INFO_EN(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C80000A0u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_DIAG_STATUS(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000020u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_P2N_BAR0_START(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000080u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_P2N_BAR1_START(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000088u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_P2N_BAR2_START(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000090u64) + (block_id & 1) * 0x8000000u64 }
#[inline] pub const fn CVMX_PESCX_P2P_BARX_END(offset: u64, block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000048u64) + (((offset & 3) + (block_id & 1) * 0x800000u64) * 16) }
#[inline] pub const fn CVMX_PESCX_P2P_BARX_START(offset: u64, block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000040u64) + (((offset & 3) + (block_id & 1) * 0x800000u64) * 16) }
#[inline] pub const fn CVMX_PESCX_TLP_CREDITS(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800C8000038u64) + (block_id & 1) * 0x8000000u64 }

extern "C" { fn CVMX_ADD_IO_SEG(address: u64) -> u64; }

cvmx_bitfield_struct!(cvmx_pescx_bist_status_s { sot:1, rqhdr0:1, rqhdr1:1, rqdata4:1, rqdata3:1, rqdata2:1, rqdata1:1, rqdata0:1, retry:1, ptlp_or:1, ntlp_or:1, ctlp_or:1, rqdata5:1, reserved_13_63:51 });
cvmx_bitfield_struct!(cvmx_pescx_bist_status_cn52xxp1 { sot:1, rqhdr0:1, rqhdr1:1, rqdata4:1, rqdata3:1, rqdata2:1, rqdata1:1, rqdata0:1, retry:1, ptlp_or:1, ntlp_or:1, ctlp_or:1, reserved_12_63:52 });
cvmx_bitfield_struct!(cvmx_pescx_bist_status2_s { ppf:1, pef_tc0:1, pef_tcf1:1, pef_tnf:1, pef_tpf0:1, pef_tpf1:1, rsl_p2e:1, peai_p2e:1, dbg_p2e:1, e2p_rsl:1, e2p_p:1, e2p_n:1, e2p_cpl:1, cto_p2e:1, reserved_14_63:50 });
cvmx_bitfield_struct!(cvmx_pescx_cfg_rd_s { addr:32, data:32 });
cvmx_bitfield_struct!(cvmx_pescx_cfg_wr_s { addr:32, data:32 });
cvmx_bitfield_struct!(cvmx_pescx_cpl_lut_valid_s { tag:32, reserved_32_63:32 });
cvmx_bitfield_struct!(cvmx_pescx_ctl_status_s { inv_lcrc:1, inv_ecrc:1, reserved_2_2:1, ro_ctlp:1, lnk_enb:1, dly_one:1, nf_ecrc:1, reserved_7_8:2, ob_p_cmd:1, pm_xpme:1, pm_xtoff:1, lane_swp:1, qlm_cfg:2, pbus:8, dnum:5, reserved_28_63:36 });
cvmx_bitfield_struct!(cvmx_pescx_ctl_status_cn56xx { inv_lcrc:1, inv_ecrc:1, reserved_2_2:1, ro_ctlp:1, lnk_enb:1, dly_one:1, nf_ecrc:1, reserved_7_8:2, ob_p_cmd:1, pm_xpme:1, pm_xtoff:1, reserved_12_12:1, qlm_cfg:2, pbus:8, dnum:5, reserved_28_63:36 });
cvmx_bitfield_struct!(cvmx_pescx_ctl_status2_s { pcierst:1, pclk_run:1, reserved_2_63:62 });
cvmx_bitfield_struct!(cvmx_pescx_ctl_status2_cn52xxp1 { pcierst:1, reserved_1_63:63 });
cvmx_bitfield_struct!(cvmx_pescx_dbg_info_s { spoison:1, rtlpmal:1, rtlplle:1, recrce:1, rpoison:1, rcemrc:1, rnfemrc:1, rfemrc:1, rpmerc:1, rptamrc:1, rumep:1, rvdm:1, acto:1, rte:1, mre:1, rdwdle:1, rtwdle:1, dpeoosd:1, fcpvwt:1, rpe:1, fcuv:1, rqo:1, rauc:1, racur:1, racca:1, caar:1, rarwdns:1, ramtlp:1, racpp:1, rawwpp:1, ecrc_e:1, reserved_31_63:33 });
cvmx_bitfield_struct!(cvmx_pescx_dbg_info_en_s { spoison:1, rtlpmal:1, rtlplle:1, recrce:1, rpoison:1, rcemrc:1, rnfemrc:1, rfemrc:1, rpmerc:1, rptamrc:1, rumep:1, rvdm:1, acto:1, rte:1, mre:1, rdwdle:1, rtwdle:1, dpeoosd:1, fcpvwt:1, rpe:1, fcuv:1, rqo:1, rauc:1, racur:1, racca:1, caar:1, rarwdns:1, ramtlp:1, racpp:1, rawwpp:1, ecrc_e:1, reserved_31_63:33 });
cvmx_bitfield_struct!(cvmx_pescx_diag_status_s { aux_en:1, pm_en:1, pm_stat:1, pm_dst:1, reserved_4_63:60 });
cvmx_bitfield_struct!(cvmx_pescx_p2n_bar0_start_s { reserved_0_13:14, addr:50 });
cvmx_bitfield_struct!(cvmx_pescx_p2n_bar1_start_s { reserved_0_25:26, addr:38 });
cvmx_bitfield_struct!(cvmx_pescx_p2n_bar2_start_s { reserved_0_38:39, addr:25 });
cvmx_bitfield_struct!(cvmx_pescx_p2p_barx_end_s { reserved_0_11:12, addr:52 });
cvmx_bitfield_struct!(cvmx_pescx_p2p_barx_start_s { reserved_0_11:12, addr:52 });
cvmx_bitfield_struct!(cvmx_pescx_tlp_credits_s { reserved_0_63:64 });
cvmx_bitfield_struct!(cvmx_pescx_tlp_credits_cn52xx { npei_p:8, npei_np:8, npei_cpl:8, pesc_p:8, pesc_np:8, pesc_cpl:8, peai_ppf:8, reserved_56_63:8 });
cvmx_bitfield_struct!(cvmx_pescx_tlp_credits_cn52xxp1 { npei_p:5, npei_np:5, npei_cpl:5, pesc_p:5, pesc_np:5, pesc_cpl:5, peai_ppf:8, reserved_38_63:26 });

macro_rules! cvmx_register_union { ($name:ident, $( $variant:ident : $ty:ty ),+ $(,)?) => {
    #[repr(C)] pub union $name { pub u64: u64, $( pub $variant: $ty, )+ }
}; }
cvmx_register_union!(cvmx_pescx_bist_status, s:cvmx_pescx_bist_status_s, cn52xxp1:cvmx_pescx_bist_status_cn52xxp1);
cvmx_register_union!(cvmx_pescx_bist_status2, s:cvmx_pescx_bist_status2_s);
cvmx_register_union!(cvmx_pescx_cfg_rd, s:cvmx_pescx_cfg_rd_s);
cvmx_register_union!(cvmx_pescx_cfg_wr, s:cvmx_pescx_cfg_wr_s);
cvmx_register_union!(cvmx_pescx_cpl_lut_valid, s:cvmx_pescx_cpl_lut_valid_s);
cvmx_register_union!(cvmx_pescx_ctl_status, s:cvmx_pescx_ctl_status_s, cn56xx:cvmx_pescx_ctl_status_cn56xx);
cvmx_register_union!(cvmx_pescx_ctl_status2, s:cvmx_pescx_ctl_status2_s, cn52xxp1:cvmx_pescx_ctl_status2_cn52xxp1);
cvmx_register_union!(cvmx_pescx_dbg_info, s:cvmx_pescx_dbg_info_s);
cvmx_register_union!(cvmx_pescx_dbg_info_en, s:cvmx_pescx_dbg_info_en_s);
cvmx_register_union!(cvmx_pescx_diag_status, s:cvmx_pescx_diag_status_s);
cvmx_register_union!(cvmx_pescx_p2n_bar0_start, s:cvmx_pescx_p2n_bar0_start_s);
cvmx_register_union!(cvmx_pescx_p2n_bar1_start, s:cvmx_pescx_p2n_bar1_start_s);
cvmx_register_union!(cvmx_pescx_p2n_bar2_start, s:cvmx_pescx_p2n_bar2_start_s);
cvmx_register_union!(cvmx_pescx_p2p_barx_end, s:cvmx_pescx_p2p_barx_end_s);
cvmx_register_union!(cvmx_pescx_p2p_barx_start, s:cvmx_pescx_p2p_barx_start_s);
cvmx_register_union!(cvmx_pescx_tlp_credits, s:cvmx_pescx_tlp_credits_s, cn52xx:cvmx_pescx_tlp_credits_cn52xx, cn52xxp1:cvmx_pescx_tlp_credits_cn52xxp1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
