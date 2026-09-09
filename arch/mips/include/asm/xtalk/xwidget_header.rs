/*
 * xwidget.h - generic crosstalk widget header file, translated from C.
 *
 * Includes and externally supplied types are intentionally left as dependencies.
 */

pub const WIDGET_ID: u32 = 0x04;
pub const WIDGET_STATUS: u32 = 0x0c;
pub const WIDGET_ERR_UPPER_ADDR: u32 = 0x14;
pub const WIDGET_ERR_LOWER_ADDR: u32 = 0x1c;
pub const WIDGET_CONTROL: u32 = 0x24;
pub const WIDGET_REQ_TIMEOUT: u32 = 0x2c;
pub const WIDGET_INTDEST_UPPER_ADDR: u32 = 0x34;
pub const WIDGET_INTDEST_LOWER_ADDR: u32 = 0x3c;
pub const WIDGET_ERR_CMD_WORD: u32 = 0x44;
pub const WIDGET_LLP_CFG: u32 = 0x4c;
pub const WIDGET_TFLUSH: u32 = 0x54;

pub const WIDGET_REV_NUM: u32 = 0xf0000000;
pub const WIDGET_PART_NUM: u32 = 0x0ffff000;
pub const WIDGET_MFG_NUM: u32 = 0x00000ffe;
pub const WIDGET_REV_NUM_SHFT: u32 = 28;
pub const WIDGET_PART_NUM_SHFT: u32 = 12;
pub const WIDGET_MFG_NUM_SHFT: u32 = 1;

#[inline]
pub const fn xwidget_part_num(widgetid: u32) -> u32 { (widgetid & WIDGET_PART_NUM) >> WIDGET_PART_NUM_SHFT }
#[inline]
pub const fn xwidget_rev_num(widgetid: u32) -> u32 { (widgetid & WIDGET_REV_NUM) >> WIDGET_REV_NUM_SHFT }
#[inline]
pub const fn xwidget_mfg_num(widgetid: u32) -> u32 { (widgetid & WIDGET_MFG_NUM) >> WIDGET_MFG_NUM_SHFT }

pub const WIDGET_LLP_REC_CNT: u32 = 0xff000000;
pub const WIDGET_LLP_TX_CNT: u32 = 0x00ff0000;
pub const WIDGET_PENDING: u32 = 0x0000001f;
pub const WIDGET_ERR_UPPER_ADDR_ONLY: u32 = 0x0000ffff;
pub const WIDGET_F_BAD_PKT: u32 = 0x00010000;
pub const WIDGET_LLP_XBAR_CRD: u32 = 0x0000f000;
pub const WIDGET_LLP_XBAR_CRD_SHFT: u32 = 12;
pub const WIDGET_CLR_RLLP_CNT: u32 = 0x00000800;
pub const WIDGET_CLR_TLLP_CNT: u32 = 0x00000400;
pub const WIDGET_SYS_END: u32 = 0x00000200;
pub const WIDGET_MAX_TRANS: u32 = 0x000001f0;
pub const WIDGET_WIDGET_ID: u32 = 0x0000000f;
pub const WIDGET_INT_VECTOR: u32 = 0xff000000;
pub const WIDGET_INT_VECTOR_SHFT: u32 = 24;
pub const WIDGET_TARGET_ID: u32 = 0x000f0000;
pub const WIDGET_TARGET_ID_SHFT: u32 = 16;
pub const WIDGET_UPP_ADDR: u32 = 0x0000ffff;
pub const WIDGET_DIDN: u32 = 0xf0000000;
pub const WIDGET_SIDN: u32 = 0x0f000000;
pub const WIDGET_PACTYP: u32 = 0x00f00000;
pub const WIDGET_TNUM: u32 = 0x000f8000;
pub const WIDGET_COHERENT: u32 = 0x00004000;
pub const WIDGET_DS: u32 = 0x00003000;
pub const WIDGET_GBR: u32 = 0x00000800;
pub const WIDGET_VBPM: u32 = 0x00000400;
pub const WIDGET_ERROR: u32 = 0x00000200;
pub const WIDGET_BARRIER: u32 = 0x00000100;
pub const WIDGET_LLP_MAXRETRY: u32 = 0x03ff0000;
pub const WIDGET_LLP_MAXRETRY_SHFT: u32 = 16;
pub const WIDGET_LLP_NULLTIMEOUT: u32 = 0x0000fc00;
pub const WIDGET_LLP_NULLTIMEOUT_SHFT: u32 = 10;
pub const WIDGET_LLP_MAXBURST: u32 = 0x000003ff;
pub const WIDGET_LLP_MAXBURST_SHFT: u32 = 0;

pub const WIDGET_XBOW_MFGR_NUM: i32 = 0x0;
pub const WIDGET_XXBOW_MFGR_NUM: i32 = 0x0;
pub const WIDGET_ODYS_MFGR_NUM: i32 = 0x023;
pub const WIDGET_TPU_MFGR_NUM: i32 = 0x024;
pub const WIDGET_XBRDG_MFGR_NUM: i32 = 0x024;
pub const WIDGET_HEART_MFGR_NUM: i32 = 0x036;
pub const WIDGET_BRIDG_MFGR_NUM: i32 = 0x036;
pub const WIDGET_HUB_MFGR_NUM: i32 = 0x036;
pub const WIDGET_BDRCK_MFGR_NUM: i32 = 0x036;
pub const WIDGET_IMPCT_MFGR_NUM: i32 = 0x2aa;
pub const WIDGET_KONA_MFGR_NUM: i32 = 0x2aa;
pub const WIDGET_NULL_MFGR_NUM: i32 = -1;

pub const WIDGET_XBOW_PART_NUM: i32 = 0x0000;
pub const WIDGET_HEART_PART_NUM: i32 = 0xc001;
pub const WIDGET_BRIDG_PART_NUM: i32 = 0xc002;
pub const WIDGET_IMPCT_PART_NUM: i32 = 0xc003;
pub const WIDGET_ODYS_PART_NUM: i32 = 0xc013;
pub const WIDGET_HUB_PART_NUM: i32 = 0xc101;
pub const WIDGET_KONA_PART_NUM: i32 = 0xc102;
pub const WIDGET_BDRCK_PART_NUM: i32 = 0xc110;
pub const WIDGET_TPU_PART_NUM: i32 = 0xc202;
pub const WIDGET_XXBOW_PART_NUM: i32 = 0xd000;
pub const WIDGET_XBRDG_PART_NUM: i32 = 0xd002;
pub const WIDGET_NULL_PART_NUM: i32 = -1;

#[repr(C)]
pub struct widget_ident {
    pub mfgr: u32,
    pub part: u32,
    pub name: *const core::ffi::c_char,
    pub revs: [*const core::ffi::c_char; 16],
}

pub static widget_idents: &[widget_ident] = &[
    widget_ident { mfgr: WIDGET_XBOW_MFGR_NUM as u32, part: WIDGET_XBOW_PART_NUM as u32, name: c"xbow".as_ptr(), revs: [core::ptr::null(), c"1.0".as_ptr(), c"1.1".as_ptr(), c"1.2".as_ptr(), c"1.3".as_ptr(), c"2.0".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_HEART_MFGR_NUM as u32, part: WIDGET_HEART_PART_NUM as u32, name: c"heart".as_ptr(), revs: [core::ptr::null(), c"A".as_ptr(), c"B".as_ptr(), c"C".as_ptr(), c"D".as_ptr(), c"E".as_ptr(), c"F".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_BRIDG_MFGR_NUM as u32, part: WIDGET_BRIDG_PART_NUM as u32, name: c"bridge".as_ptr(), revs: [core::ptr::null(), c"A".as_ptr(), c"B".as_ptr(), c"C".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_IMPCT_MFGR_NUM as u32, part: WIDGET_IMPCT_PART_NUM as u32, name: c"impact".as_ptr(), revs: [core::ptr::null(), c"A".as_ptr(), c"B".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_ODYS_MFGR_NUM as u32, part: WIDGET_ODYS_PART_NUM as u32, name: c"odyssey".as_ptr(), revs: [core::ptr::null(), c"A".as_ptr(), c"B".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_HUB_MFGR_NUM as u32, part: WIDGET_HUB_PART_NUM as u32, name: c"hub".as_ptr(), revs: [core::ptr::null(), c"1.0".as_ptr(), c"2.0".as_ptr(), c"2.1".as_ptr(), c"2.2".as_ptr(), c"2.3".as_ptr(), c"2.4".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_KONA_MFGR_NUM as u32, part: WIDGET_KONA_PART_NUM as u32, name: c"kona".as_ptr(), revs: [core::ptr::null(); 16] },
    widget_ident { mfgr: WIDGET_BDRCK_MFGR_NUM as u32, part: WIDGET_BDRCK_PART_NUM as u32, name: c"bedrock".as_ptr(), revs: [core::ptr::null(), c"1.0".as_ptr(), c"1.1".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_TPU_MFGR_NUM as u32, part: WIDGET_TPU_PART_NUM as u32, name: c"tpu".as_ptr(), revs: [c"0".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_XXBOW_MFGR_NUM as u32, part: WIDGET_XXBOW_PART_NUM as u32, name: c"xxbow".as_ptr(), revs: [core::ptr::null(), c"1.0".as_ptr(), c"2.0".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_XBRDG_MFGR_NUM as u32, part: WIDGET_XBRDG_PART_NUM as u32, name: c"xbridge".as_ptr(), revs: [core::ptr::null(), c"A".as_ptr(), c"B".as_ptr(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    widget_ident { mfgr: WIDGET_NULL_MFGR_NUM as u32, part: WIDGET_NULL_PART_NUM as u32, name: core::ptr::null(), revs: [core::ptr::null(); 16] },
];

pub type widgetreg_t = u32;
#[repr(C)]
pub struct widget_cfg {
    pub w_pad_0: widgetreg_t, pub w_id: widgetreg_t, pub w_pad_1: widgetreg_t, pub w_status: widgetreg_t,
    pub w_pad_2: widgetreg_t, pub w_err_upper_addr: widgetreg_t, pub w_pad_3: widgetreg_t, pub w_err_lower_addr: widgetreg_t,
    pub w_pad_4: widgetreg_t, pub w_control: widgetreg_t, pub w_pad_5: widgetreg_t, pub w_req_timeout: widgetreg_t,
    pub w_pad_6: widgetreg_t, pub w_intdest_upper_addr: widgetreg_t, pub w_pad_7: widgetreg_t, pub w_intdest_lower_addr: widgetreg_t,
    pub w_pad_8: widgetreg_t, pub w_err_cmd_word: widgetreg_t, pub w_pad_9: widgetreg_t, pub w_llp_cfg: widgetreg_t,
    pub w_pad_10: widgetreg_t, pub w_tflush: widgetreg_t,
}
pub type widget_cfg_t = widget_cfg;

#[repr(C)]
pub struct w_err_cmd_word_f { pub didn: u32, pub sidn: u32, pub pactyp: u32, pub tnum: u32, pub ct: u32, pub ds: u32, pub gbr: u32, pub vbpm: u32, pub error: u32, pub bo: u32, pub other: u32 }
#[repr(C)]
pub union w_err_cmd_word_u { pub r: widgetreg_t, pub f: w_err_cmd_word_f }

pub type xwidget_info_t = *mut xwidget_info_s;
#[repr(C)] pub struct xwidget_info_s;
#[repr(C)] pub struct xwidget_hwid_s { pub part_num: xwidget_part_num_t, pub rev_num: xwidget_rev_num_t, pub mfg_num: xwidget_mfg_num_t }
pub type xwidget_hwid_t = *mut xwidget_hwid_s;

pub unsafe fn xwidget_hardware_id_match(hwid1: *const xwidget_hwid_s, hwid2: *const xwidget_hwid_s) -> bool {
    (*hwid1).part_num == (*hwid2).part_num &&
        ((*hwid1).mfg_num == XWIDGET_MFG_NUM_NONE || (*hwid2).mfg_num == XWIDGET_MFG_NUM_NONE || (*hwid1).mfg_num == (*hwid2).mfg_num)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
