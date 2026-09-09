/* Translated from cvmx-iob-defs.h.  The C bitfields are represented by the
 * underlying 64-bit register value; bit positions depend on target endianness
 * (__BIG_ENDIAN_BITFIELD in the original header). */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

extern "Rust" {
    fn CVMX_ADD_IO_SEG(value: u64) -> u64;
}

pub const CVMX_IOB_BIST_STATUS: u64 = 0x00011800F00007F8;
pub const CVMX_IOB_CTL_STATUS: u64 = 0x00011800F0000050;
pub const CVMX_IOB_DWB_PRI_CNT: u64 = 0x00011800F0000028;
pub const CVMX_IOB_FAU_TIMEOUT: u64 = 0x00011800F0000000;
pub const CVMX_IOB_I2C_PRI_CNT: u64 = 0x00011800F0000010;
pub const CVMX_IOB_INB_CONTROL_MATCH: u64 = 0x00011800F0000078;
pub const CVMX_IOB_INB_CONTROL_MATCH_ENB: u64 = 0x00011800F0000088;
pub const CVMX_IOB_INB_DATA_MATCH: u64 = 0x00011800F0000070;
pub const CVMX_IOB_INB_DATA_MATCH_ENB: u64 = 0x00011800F0000080;
pub const CVMX_IOB_INT_ENB: u64 = 0x00011800F0000060;
pub const CVMX_IOB_INT_SUM: u64 = 0x00011800F0000058;
pub const CVMX_IOB_N2C_L2C_PRI_CNT: u64 = 0x00011800F0000020;
pub const CVMX_IOB_N2C_RSP_PRI_CNT: u64 = 0x00011800F0000008;
pub const CVMX_IOB_OUTB_COM_PRI_CNT: u64 = 0x00011800F0000040;
pub const CVMX_IOB_OUTB_CONTROL_MATCH: u64 = 0x00011800F0000098;
pub const CVMX_IOB_OUTB_CONTROL_MATCH_ENB: u64 = 0x00011800F00000A8;
pub const CVMX_IOB_OUTB_DATA_MATCH: u64 = 0x00011800F0000090;
pub const CVMX_IOB_OUTB_DATA_MATCH_ENB: u64 = 0x00011800F00000A0;
pub const CVMX_IOB_OUTB_FPA_PRI_CNT: u64 = 0x00011800F0000048;
pub const CVMX_IOB_OUTB_REQ_PRI_CNT: u64 = 0x00011800F0000038;
pub const CVMX_IOB_P2C_REQ_PRI_CNT: u64 = 0x00011800F0000018;
pub const CVMX_IOB_PKT_ERR: u64 = 0x00011800F0000068;
pub const CVMX_IOB_TO_CMB_CREDITS: u64 = 0x00011800F00000B0;
pub const CVMX_IOB_TO_NCB_DID_00_CREDITS: u64 = 0x00011800F0000800;
pub const CVMX_IOB_TO_NCB_DID_111_CREDITS: u64 = 0x00011800F0000B78;
pub const CVMX_IOB_TO_NCB_DID_223_CREDITS: u64 = 0x00011800F0000EF8;
pub const CVMX_IOB_TO_NCB_DID_24_CREDITS: u64 = 0x00011800F00008C0;
pub const CVMX_IOB_TO_NCB_DID_32_CREDITS: u64 = 0x00011800F0000900;
pub const CVMX_IOB_TO_NCB_DID_40_CREDITS: u64 = 0x00011800F0000940;
pub const CVMX_IOB_TO_NCB_DID_55_CREDITS: u64 = 0x00011800F00009B8;
pub const CVMX_IOB_TO_NCB_DID_64_CREDITS: u64 = 0x00011800F0000A00;
pub const CVMX_IOB_TO_NCB_DID_79_CREDITS: u64 = 0x00011800F0000A78;
pub const CVMX_IOB_TO_NCB_DID_96_CREDITS: u64 = 0x00011800F0000B00;
pub const CVMX_IOB_TO_NCB_DID_98_CREDITS: u64 = 0x00011800F0000B10;

macro_rules! cvmx_iob_register_union {
    ($union:ident; $($view:ident),+ $(,)?) => {
        #[repr(C)]
        pub union $union { pub u64: u64, $(pub $view: $view),+ }
        $(#[repr(C)] #[derive(Copy, Clone)] pub struct $view { pub bits: u64 })+
    };
}

cvmx_iob_register_union!(cvmx_iob_bist_status; cvmx_iob_bist_status_s, cvmx_iob_bist_status_cn30xx, cvmx_iob_bist_status_cn61xx, cvmx_iob_bist_status_cn68xx);
cvmx_iob_register_union!(cvmx_iob_ctl_status; cvmx_iob_ctl_status_s, cvmx_iob_ctl_status_cn30xx, cvmx_iob_ctl_status_cn52xx, cvmx_iob_ctl_status_cn61xx, cvmx_iob_ctl_status_cn63xx, cvmx_iob_ctl_status_cn68xx);
cvmx_iob_register_union!(cvmx_iob_dwb_pri_cnt; cvmx_iob_dwb_pri_cnt_s);
cvmx_iob_register_union!(cvmx_iob_fau_timeout; cvmx_iob_fau_timeout_s);
cvmx_iob_register_union!(cvmx_iob_i2c_pri_cnt; cvmx_iob_i2c_pri_cnt_s);
cvmx_iob_register_union!(cvmx_iob_inb_control_match; cvmx_iob_inb_control_match_s);
cvmx_iob_register_union!(cvmx_iob_inb_control_match_enb; cvmx_iob_inb_control_match_enb_s);
cvmx_iob_register_union!(cvmx_iob_inb_data_match; cvmx_iob_inb_data_match_s);
cvmx_iob_register_union!(cvmx_iob_inb_data_match_enb; cvmx_iob_inb_data_match_enb_s);
cvmx_iob_register_union!(cvmx_iob_int_enb; cvmx_iob_int_enb_s, cvmx_iob_int_enb_cn30xx, cvmx_iob_int_enb_cn68xx);
cvmx_iob_register_union!(cvmx_iob_int_sum; cvmx_iob_int_sum_s, cvmx_iob_int_sum_cn30xx, cvmx_iob_int_sum_cn68xx);
cvmx_iob_register_union!(cvmx_iob_n2c_l2c_pri_cnt; cvmx_iob_n2c_l2c_pri_cnt_s);
cvmx_iob_register_union!(cvmx_iob_n2c_rsp_pri_cnt; cvmx_iob_n2c_rsp_pri_cnt_s);
cvmx_iob_register_union!(cvmx_iob_outb_com_pri_cnt; cvmx_iob_outb_com_pri_cnt_s);
cvmx_iob_register_union!(cvmx_iob_outb_control_match; cvmx_iob_outb_control_match_s);
cvmx_iob_register_union!(cvmx_iob_outb_control_match_enb; cvmx_iob_outb_control_match_enb_s);
cvmx_iob_register_union!(cvmx_iob_outb_data_match; cvmx_iob_outb_data_match_s);
cvmx_iob_register_union!(cvmx_iob_outb_data_match_enb; cvmx_iob_outb_data_match_enb_s);
cvmx_iob_register_union!(cvmx_iob_outb_fpa_pri_cnt; cvmx_iob_outb_fpa_pri_cnt_s);
cvmx_iob_register_union!(cvmx_iob_outb_req_pri_cnt; cvmx_iob_outb_req_pri_cnt_s);
cvmx_iob_register_union!(cvmx_iob_p2c_req_pri_cnt; cvmx_iob_p2c_req_pri_cnt_s);
cvmx_iob_register_union!(cvmx_iob_pkt_err; cvmx_iob_pkt_err_s, cvmx_iob_pkt_err_cn30xx);
cvmx_iob_register_union!(cvmx_iob_to_cmb_credits; cvmx_iob_to_cmb_credits_s, cvmx_iob_to_cmb_credits_cn52xx, cvmx_iob_to_cmb_credits_cn68xx);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_00_credits; cvmx_iob_to_ncb_did_00_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_111_credits; cvmx_iob_to_ncb_did_111_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_223_credits; cvmx_iob_to_ncb_did_223_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_24_credits; cvmx_iob_to_ncb_did_24_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_32_credits; cvmx_iob_to_ncb_did_32_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_40_credits; cvmx_iob_to_ncb_did_40_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_55_credits; cvmx_iob_to_ncb_did_55_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_64_credits; cvmx_iob_to_ncb_did_64_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_79_credits; cvmx_iob_to_ncb_did_79_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_96_credits; cvmx_iob_to_ncb_did_96_credits_s);
cvmx_iob_register_union!(cvmx_iob_to_ncb_did_98_credits; cvmx_iob_to_ncb_did_98_credits_s);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
