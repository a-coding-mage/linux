/* Rust translation of cvmx-ipd-defs.h. Bit-field layouts are represented by
 * the original 64-bit register storage; the C ABI names are retained. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

#[inline(always)]
pub const fn cvmx_add_io_seg(x: u64) -> u64 { x }

macro_rules! reg { ($n:ident, $v:expr) => { pub const $n: u64 = cvmx_add_io_seg($v); }; }
macro_rules! regx { ($n:ident, $v:expr, $m:expr) => {
    #[inline(always)] pub const fn $n(offset: u64) -> u64 { cvmx_add_io_seg($v) + (offset & $m) * 8 }
}; }

reg!(CVMX_IPD_1ST_MBUFF_SKIP, 0x00014F0000000000);
reg!(CVMX_IPD_1st_NEXT_PTR_BACK, 0x00014F0000000150);
reg!(CVMX_IPD_2nd_NEXT_PTR_BACK, 0x00014F0000000158);
reg!(CVMX_IPD_BIST_STATUS, 0x00014F00000007F8);
regx!(CVMX_IPD_BPIDX_MBUF_TH, 0x00014F0000002000, 63);
regx!(CVMX_IPD_BPID_BP_COUNTERX, 0x00014F0000003000, 63);
reg!(CVMX_IPD_BP_PRT_RED_END, 0x00014F0000000328);
reg!(CVMX_IPD_CLK_COUNT, 0x00014F0000000338);
reg!(CVMX_IPD_CREDITS, 0x00014F0000004410);
reg!(CVMX_IPD_CTL_STATUS, 0x00014F0000000018);
reg!(CVMX_IPD_ECC_CTL, 0x00014F0000004408);
reg!(CVMX_IPD_FREE_PTR_FIFO_CTL, 0x00014F0000000780);
reg!(CVMX_IPD_FREE_PTR_VALUE, 0x00014F0000000788);
reg!(CVMX_IPD_HOLD_PTR_FIFO_CTL, 0x00014F0000000790);
reg!(CVMX_IPD_INT_ENB, 0x00014F0000000160);
reg!(CVMX_IPD_INT_SUM, 0x00014F0000000168);
reg!(CVMX_IPD_NEXT_PKT_PTR, 0x00014F00000007A0);
reg!(CVMX_IPD_NEXT_WQE_PTR, 0x00014F00000007A8);
reg!(CVMX_IPD_NOT_1ST_MBUFF_SKIP, 0x00014F0000000008);
#[inline(always)] pub const fn CVMX_IPD_ON_BP_DROP_PKTX(_block_id: u64) -> u64 { cvmx_add_io_seg(0x00014F0000004100) }
reg!(CVMX_IPD_PACKET_MBUFF_SIZE, 0x00014F0000000010);
reg!(CVMX_IPD_PKT_ERR, 0x00014F00000003F0);
reg!(CVMX_IPD_PKT_PTR_VALID, 0x00014F0000000358);
regx!(CVMX_IPD_PORTX_BP_PAGE_CNT, 0x00014F0000000028, 63);
regx!(CVMX_IPD_PORTX_BP_PAGE_CNT2, 0x00014F0000000368, 63);
regx!(CVMX_IPD_PORTX_BP_PAGE_CNT3, 0x00014F00000003D0, 63);
regx!(CVMX_IPD_PORT_BP_COUNTERS2_PAIRX, 0x00014F0000000388, 63);
regx!(CVMX_IPD_PORT_BP_COUNTERS3_PAIRX, 0x00014F00000003B0, 63);
regx!(CVMX_IPD_PORT_BP_COUNTERS4_PAIRX, 0x00014F0000000410, 63);
regx!(CVMX_IPD_PORT_BP_COUNTERS_PAIRX, 0x00014F00000001B8, 63);
reg!(CVMX_IPD_PORT_PTR_FIFO_CTL, 0x00014F0000000798);
regx!(CVMX_IPD_PORT_QOS_INTX, 0x00014F0000000808, 7);
regx!(CVMX_IPD_PORT_QOS_INT_ENBX, 0x00014F0000000848, 7);
regx!(CVMX_IPD_PORT_QOS_X_CNT, 0x00014F0000000888, 511);
#[inline(always)] pub const fn CVMX_IPD_PORT_SOPX(_block_id: u64) -> u64 { cvmx_add_io_seg(0x00014F0000004400) }
reg!(CVMX_IPD_PRC_HOLD_PTR_FIFO_CTL, 0x00014F0000000348);
reg!(CVMX_IPD_PRC_PORT_PTR_FIFO_CTL, 0x00014F0000000350);
reg!(CVMX_IPD_PTR_COUNT, 0x00014F0000000320);
reg!(CVMX_IPD_PWP_PTR_FIFO_CTL, 0x00014F0000000340);
regx!(CVMX_IPD_QOSX_RED_MARKS, 0x00014F0000000178, 7);
pub const CVMX_IPD_QOS0_RED_MARKS:u64=CVMX_IPD_QOSX_RED_MARKS(0); pub const CVMX_IPD_QOS1_RED_MARKS:u64=CVMX_IPD_QOSX_RED_MARKS(1); pub const CVMX_IPD_QOS2_RED_MARKS:u64=CVMX_IPD_QOSX_RED_MARKS(2); pub const CVMX_IPD_QOS3_RED_MARKS:u64=CVMX_IPD_QOSX_RED_MARKS(3); pub const CVMX_IPD_QOS4_RED_MARKS:u64=CVMX_IPD_QOSX_RED_MARKS(4); pub const CVMX_IPD_QOS5_RED_MARKS:u64=CVMX_IPD_QOSX_RED_MARKS(5); pub const CVMX_IPD_QOS6_RED_MARKS:u64=CVMX_IPD_QOSX_RED_MARKS(6); pub const CVMX_IPD_QOS7_RED_MARKS:u64=CVMX_IPD_QOSX_RED_MARKS(7);
reg!(CVMX_IPD_QUE0_FREE_PAGE_CNT, 0x00014F0000000330); reg!(CVMX_IPD_RED_DELAY,0x00014F0000004300); reg!(CVMX_IPD_RED_PORT_ENABLE,0x00014F00000002D8); reg!(CVMX_IPD_RED_PORT_ENABLE2,0x00014F00000003A8); reg!(CVMX_IPD_REQ_WGT,0x00014F0000004418); reg!(CVMX_IPD_SUB_PORT_BP_PAGE_CNT,0x00014F0000000148); reg!(CVMX_IPD_SUB_PORT_FCS,0x00014F0000000170); reg!(CVMX_IPD_SUB_PORT_QOS_CNT,0x00014F0000000800); reg!(CVMX_IPD_WQE_FPA_QUEUE,0x00014F0000000020); reg!(CVMX_IPD_WQE_PTR_VALID,0x00014F0000000360);
regx!(CVMX_IPD_RED_BPID_ENABLEX,0x00014F0000004200,0); regx!(CVMX_IPD_RED_QUEX_PARAM,0x00014F00000002E0,7);
pub const CVMX_IPD_RED_QUE0_PARAM:u64=CVMX_IPD_RED_QUEX_PARAM(0); pub const CVMX_IPD_RED_QUE1_PARAM:u64=CVMX_IPD_RED_QUEX_PARAM(1); pub const CVMX_IPD_RED_QUE2_PARAM:u64=CVMX_IPD_RED_QUEX_PARAM(2); pub const CVMX_IPD_RED_QUE3_PARAM:u64=CVMX_IPD_RED_QUEX_PARAM(3); pub const CVMX_IPD_RED_QUE4_PARAM:u64=CVMX_IPD_RED_QUEX_PARAM(4); pub const CVMX_IPD_RED_QUE5_PARAM:u64=CVMX_IPD_RED_QUEX_PARAM(5); pub const CVMX_IPD_RED_QUE6_PARAM:u64=CVMX_IPD_RED_QUEX_PARAM(6); pub const CVMX_IPD_RED_QUE7_PARAM:u64=CVMX_IPD_RED_QUEX_PARAM(7);

#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_ipd_register_bits { pub raw: u64 }
macro_rules! ipd_union { ($u:ident, $($f:ident),+ $(,)?) => { #[repr(C)] pub union $u { pub u64: u64, $(pub $f: cvmx_ipd_register_bits),+ } }; }
ipd_union!(cvmx_ipd_1st_mbuff_skip, s); ipd_union!(cvmx_ipd_1st_next_ptr_back,s); ipd_union!(cvmx_ipd_2nd_next_ptr_back,s); ipd_union!(cvmx_ipd_bist_status,s,cn30xx,cn52xx); ipd_union!(cvmx_ipd_bp_prt_red_end,s,cn30xx,cn52xx,cn63xx); ipd_union!(cvmx_ipd_bpidx_mbuf_th,s); ipd_union!(cvmx_ipd_bpid_bp_counterx,s); ipd_union!(cvmx_ipd_clk_count,s); ipd_union!(cvmx_ipd_credits,s); ipd_union!(cvmx_ipd_ctl_status,s,cn30xx,cn38xxp2,cn50xx,cn58xx,cn63xxp1); ipd_union!(cvmx_ipd_ecc_ctl,s); ipd_union!(cvmx_ipd_free_ptr_fifo_ctl,s); ipd_union!(cvmx_ipd_free_ptr_value,s); ipd_union!(cvmx_ipd_hold_ptr_fifo_ctl,s); ipd_union!(cvmx_ipd_int_enb,s,cn30xx,cn38xx,cn52xx); ipd_union!(cvmx_ipd_int_sum,s,cn30xx,cn38xx,cn52xx); ipd_union!(cvmx_ipd_next_pkt_ptr,s); ipd_union!(cvmx_ipd_next_wqe_ptr,s); ipd_union!(cvmx_ipd_not_1st_mbuff_skip,s); ipd_union!(cvmx_ipd_on_bp_drop_pktx,s); ipd_union!(cvmx_ipd_packet_mbuff_size,s); ipd_union!(cvmx_ipd_pkt_err,s); ipd_union!(cvmx_ipd_pkt_ptr_valid,s); ipd_union!(cvmx_ipd_portx_bp_page_cnt,s); ipd_union!(cvmx_ipd_portx_bp_page_cnt2,s); ipd_union!(cvmx_ipd_portx_bp_page_cnt3,s); ipd_union!(cvmx_ipd_port_bp_counters2_pairx,s); ipd_union!(cvmx_ipd_port_bp_counters3_pairx,s); ipd_union!(cvmx_ipd_port_bp_counters4_pairx,s); ipd_union!(cvmx_ipd_port_bp_counters_pairx,s); ipd_union!(cvmx_ipd_port_ptr_fifo_ctl,s); ipd_union!(cvmx_ipd_port_qos_x_cnt,s); ipd_union!(cvmx_ipd_port_qos_intx,s); ipd_union!(cvmx_ipd_port_qos_int_enbx,s); ipd_union!(cvmx_ipd_port_sopx,s); ipd_union!(cvmx_ipd_prc_hold_ptr_fifo_ctl,s); ipd_union!(cvmx_ipd_prc_port_ptr_fifo_ctl,s); ipd_union!(cvmx_ipd_ptr_count,s); ipd_union!(cvmx_ipd_pwp_ptr_fifo_ctl,s); ipd_union!(cvmx_ipd_qosx_red_marks,s); ipd_union!(cvmx_ipd_que0_free_page_cnt,s); ipd_union!(cvmx_ipd_red_bpid_enablex,s); ipd_union!(cvmx_ipd_red_delay,s); ipd_union!(cvmx_ipd_red_port_enable,s); ipd_union!(cvmx_ipd_red_port_enable2,s,cn52xx,cn63xx); ipd_union!(cvmx_ipd_red_quex_param,s); ipd_union!(cvmx_ipd_req_wgt,s); ipd_union!(cvmx_ipd_sub_port_bp_page_cnt,s); ipd_union!(cvmx_ipd_sub_port_fcs,s,cn30xx,cn38xx); ipd_union!(cvmx_ipd_sub_port_qos_cnt,s); ipd_union!(cvmx_ipd_wqe_fpa_queue,s); ipd_union!(cvmx_ipd_wqe_ptr_valid,s);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
