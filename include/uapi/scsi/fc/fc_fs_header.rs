/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Fibre Channel Framing and Signalling definitions. */

#[repr(C)]
pub struct fc_frame_header {
    pub fh_r_ctl: __u8,
    pub fh_d_id: [__u8; 3],
    pub fh_cs_ctl: __u8,
    pub fh_s_id: [__u8; 3],
    pub fh_type: __u8,
    pub fh_f_ctl: [__u8; 3],
    pub fh_seq_id: __u8,
    pub fh_df_ctl: __u8,
    pub fh_seq_cnt: __be16,
    pub fh_ox_id: __be16,
    pub fh_rx_id: __be16,
    pub fh_parm_offset: __be32,
}

pub const FC_FRAME_HEADER_LEN: u32 = 24;
pub const FC_MAX_PAYLOAD: u32 = 2112;
pub const FC_MIN_MAX_PAYLOAD: u32 = 256;
pub const FC_MAX_FRAME: u32 = FC_MAX_PAYLOAD + FC_FRAME_HEADER_LEN;
pub const FC_MIN_MAX_FRAME: u32 = FC_MIN_MAX_PAYLOAD + FC_FRAME_HEADER_LEN;

#[repr(i32)]
pub enum fc_rctl {
    FC_RCTL_DD_UNCAT = 0x00,
    FC_RCTL_DD_SOL_DATA = 0x01,
    FC_RCTL_DD_UNSOL_CTL = 0x02,
    FC_RCTL_DD_SOL_CTL = 0x03,
    FC_RCTL_DD_UNSOL_DATA = 0x04,
    FC_RCTL_DD_DATA_DESC = 0x05,
    FC_RCTL_DD_UNSOL_CMD = 0x06,
    FC_RCTL_DD_CMD_STATUS = 0x07,
    FC_RCTL_ELS_REQ = 0x22,
    FC_RCTL_ELS_REP = 0x23,
    FC_RCTL_ELS4_REQ = 0x32,
    FC_RCTL_ELS4_REP = 0x33,
    FC_RCTL_VFTH = 0x50,
    FC_RCTL_IFRH = 0x51,
    FC_RCTL_ENCH = 0x52,
    FC_RCTL_BA_NOP = 0x80,
    FC_RCTL_BA_ABTS = 0x81,
    FC_RCTL_BA_RMC = 0x82,
    FC_RCTL_BA_ACC = 0x84,
    FC_RCTL_BA_RJT = 0x85,
    FC_RCTL_BA_PRMT = 0x86,
    FC_RCTL_ACK_1 = 0xc0,
    FC_RCTL_ACK_0 = 0xc1,
    FC_RCTL_P_RJT = 0xc2,
    FC_RCTL_F_RJT = 0xc3,
    FC_RCTL_P_BSY = 0xc4,
    FC_RCTL_F_BSY = 0xc5,
    FC_RCTL_F_BSYL = 0xc6,
    FC_RCTL_LCR = 0xc7,
    FC_RCTL_END = 0xc9,
}
pub const FC_RCTL_ILS_REQ: fc_rctl = fc_rctl::FC_RCTL_DD_UNSOL_CTL;
pub const FC_RCTL_ILS_REP: fc_rctl = fc_rctl::FC_RCTL_DD_SOL_CTL;

#[macro_export]
macro_rules! FC_RCTL_NAMES_INIT { () => {{
    let mut a: [&'static str; 256] = [""; 256];
    a[0x00]="uncat"; a[0x01]="sol data"; a[0x02]="unsol ctl"; a[0x03]="sol ctl/reply";
    a[0x04]="unsol data"; a[0x05]="data desc"; a[0x06]="unsol cmd"; a[0x07]="cmd status";
    a[0x22]="ELS req"; a[0x23]="ELS rep"; a[0x32]="FC-4 ELS req"; a[0x33]="FC-4 ELS rep";
    a[0x80]="BLS NOP"; a[0x81]="BLS abort"; a[0x82]="BLS remove connection"; a[0x84]="BLS accept";
    a[0x85]="BLS reject"; a[0x86]="BLS dedicated connection preempted"; a[0xc0]="LC ACK_1";
    a[0xc1]="LC ACK_0"; a[0xc2]="LC port reject"; a[0xc3]="LC fabric reject"; a[0xc4]="LC port busy";
    a[0xc5]="LC fabric busy to data frame"; a[0xc6]="LC fabric busy to link control frame";
    a[0xc7]="LC link credit reset"; a[0xc9]="LC end"; a
}}; }

#[repr(i32)]
pub enum fc_well_known_fid {
    FC_FID_NONE=0x000000, FC_FID_BCAST=0xffffff, FC_FID_FLOGI=0xfffffe, FC_FID_FCTRL=0xfffffd,
    FC_FID_DIR_SERV=0xfffffc, FC_FID_TIME_SERV=0xfffffb, FC_FID_MGMT_SERV=0xfffffa,
    FC_FID_QOS=0xfffff9, FC_FID_ALIASES=0xfffff8, FC_FID_SEC_KEY=0xfffff7,
    FC_FID_CLOCK=0xfffff6, FC_FID_MCAST_SERV=0xfffff5,
}
pub const FC_FID_WELL_KNOWN_MAX: u32 = 0xffffff;
pub const FC_FID_WELL_KNOWN_BASE: u32 = 0xfffff5;
pub const FC_FID_DOM_MGR: u32 = 0xfffc00;
pub const FC_FID_DOMAIN: u32 = 0;
pub const FC_FID_PORT: u32 = 1;
pub const FC_FID_LINK: u32 = 2;

#[repr(i32)]
pub enum fc_fh_type { FC_TYPE_BLS=0x00, FC_TYPE_ELS=0x01, FC_TYPE_IP=0x05, FC_TYPE_FCP=0x08, FC_TYPE_CT=0x20, FC_TYPE_ILS=0x22, FC_TYPE_NVME=0x28 }
#[macro_export]
macro_rules! FC_TYPE_NAMES_INIT { () => {{ let mut a: [&'static str; 256] = [""; 256]; a[0]="BLS"; a[1]="ELS"; a[5]="IP"; a[8]="FCP"; a[0x20]="CT"; a[0x22]="ILS"; a[0x28]="NVME"; a }}; }

pub const FC_XID_UNKNOWN: u32 = 0xffff; pub const FC_XID_MIN: u32 = 0; pub const FC_XID_MAX: u32 = 0xfffe;
pub const FC_FC_EX_CTX: u32 = 1 << 23; pub const FC_FC_SEQ_CTX: u32 = 1 << 22; pub const FC_FC_FIRST_SEQ: u32 = 1 << 21; pub const FC_FC_LAST_SEQ: u32 = 1 << 20; pub const FC_FC_END_SEQ: u32 = 1 << 19; pub const FC_FC_END_CONN: u32 = 1 << 18; pub const FC_FC_RES_B17: u32 = 1 << 17; pub const FC_FC_SEQ_INIT: u32 = 1 << 16; pub const FC_FC_X_ID_REASS: u32 = 1 << 15; pub const FC_FC_X_ID_INVAL: u32 = 1 << 14;
pub const FC_FC_ACK_1: u32 = 1 << 12; pub const FC_FC_ACK_N: u32 = 2 << 12; pub const FC_FC_ACK_0: u32 = 3 << 12; pub const FC_FC_RES_B11: u32 = 1 << 11; pub const FC_FC_RES_B10: u32 = 1 << 10; pub const FC_FC_RETX_SEQ: u32 = 1 << 9; pub const FC_FC_UNI_TX: u32 = 1 << 8;
#[inline] pub const fn FC_FC_CONT_SEQ(i: u32) -> u32 { i << 6 }
#[inline] pub const fn FC_FC_ABT_SEQ(i: u32) -> u32 { i << 4 }
pub const FC_FC_REL_OFF: u32 = 1 << 3; pub const FC_FC_RES2: u32 = 1 << 2;
#[inline] pub const fn FC_FC_FILL(i: u32) -> u32 { i & 3 }

#[repr(C)] pub struct fc_ba_acc { pub ba_seq_id_val: __u8, pub ba_seq_id: __u8, pub ba_resvd: [__u8;2], pub ba_ox_id: __be16, pub ba_rx_id: __be16, pub ba_low_seq_cnt: __be16, pub ba_high_seq_cnt: __be16 }
pub const FC_BA_SEQ_ID_VAL: __u8 = 0x80;
#[repr(C)] pub struct fc_ba_rjt { pub br_resvd: __u8, pub br_reason: __u8, pub br_explan: __u8, pub br_vendor: __u8 }
#[repr(i32)] pub enum fc_ba_rjt_reason { FC_BA_RJT_NONE=0, FC_BA_RJT_INVL_CMD=0x01, FC_BA_RJT_LOG_ERR=0x03, FC_BA_RJT_LOG_BUSY=0x05, FC_BA_RJT_PROTO_ERR=0x07, FC_BA_RJT_UNABLE=0x09, FC_BA_RJT_VENDOR=0xff }
#[repr(i32)] pub enum fc_ba_rjt_explan { FC_BA_RJT_EXP_NONE=0, FC_BA_RJT_INV_XID=0x03, FC_BA_RJT_ABT=0x05 }
#[repr(C)] pub struct fc_pf_rjt { pub rj_action: __u8, pub rj_reason: __u8, pub rj_resvd: __u8, pub rj_vendor: __u8 }
#[repr(i32)] pub enum fc_pf_rjt_reason {
    FC_RJT_NONE=0, FC_RJT_INVL_DID=0x01, FC_RJT_INVL_SID=0x02, FC_RJT_P_UNAV_T=0x03, FC_RJT_P_UNAV=0x04, FC_RJT_CLS_UNSUP=0x05, FC_RJT_DEL_USAGE=0x06, FC_RJT_TYPE_UNSUP=0x07, FC_RJT_LINK_CTL=0x08, FC_RJT_R_CTL=0x09, FC_RJT_F_CTL=0x0a, FC_RJT_OX_ID=0x0b, FC_RJT_RX_ID=0x0c, FC_RJT_SEQ_ID=0x0d, FC_RJT_DF_CTL=0x0e, FC_RJT_SEQ_CNT=0x0f, FC_RJT_PARAM=0x10, FC_RJT_EXCH_ERR=0x11, FC_RJT_PROTO=0x12, FC_RJT_LEN=0x13, FC_RJT_UNEXP_ACK=0x14, FC_RJT_FAB_CLASS=0x15, FC_RJT_LOGI_REQ=0x16, FC_RJT_SEQ_XS=0x17, FC_RJT_EXCH_EST=0x18, FC_RJT_FAB_UNAV=0x1a, FC_RJT_VC_ID=0x1b, FC_RJT_CS_CTL=0x1c, FC_RJT_INSUF_RES=0x1d, FC_RJT_INVL_CLS=0x1f, FC_RJT_PREEMT_RJT=0x20, FC_RJT_PREEMT_DIS=0x21, FC_RJT_MCAST_ERR=0x22, FC_RJT_MCAST_ET=0x23, FC_RJT_PRLI_REQ=0x24, FC_RJT_INVL_ATT=0x25, FC_RJT_VENDOR=0xff,
}
pub const FC_DEF_E_D_TOV: u64 = 2000; pub const FC_DEF_R_A_TOV: u64 = 10000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
