/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from fc_els.h. */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fc_els_cmd {
    ELS_LS_RJT=0x01, ELS_LS_ACC=0x02, ELS_PLOGI=0x03, ELS_FLOGI=0x04, ELS_LOGO=0x05,
    ELS_ABTX=0x06, ELS_RCS=0x07, ELS_RES=0x08, ELS_RSS=0x09, ELS_RSI=0x0a,
    ELS_ESTS=0x0b, ELS_ESTC=0x0c, ELS_ADVC=0x0d, ELS_RTV=0x0e, ELS_RLS=0x0f,
    ELS_ECHO=0x10, ELS_TEST=0x11, ELS_RRQ=0x12, ELS_REC=0x13, ELS_SRR=0x14,
    ELS_FPIN=0x16, ELS_EDC=0x17, ELS_RDP=0x18, ELS_RDF=0x19, ELS_PRLI=0x20,
    ELS_PRLO=0x21, ELS_SCN=0x22, ELS_TPLS=0x23, ELS_TPRLO=0x24, ELS_LCLM=0x25,
    ELS_GAID=0x30, ELS_FACT=0x31, ELS_FDACDT=0x32, ELS_NACT=0x33, ELS_NDACT=0x34,
    ELS_QOSR=0x40, ELS_RVCS=0x41, ELS_PDISC=0x50, ELS_FDISC=0x51, ELS_ADISC=0x52,
    ELS_RNC=0x53, ELS_FARP_REQ=0x54, ELS_FARP_REPL=0x55, ELS_RPS=0x56, ELS_RPL=0x57,
    ELS_RPBC=0x58, ELS_FAN=0x60, ELS_RSCN=0x61, ELS_SCR=0x62, ELS_RNFT=0x63,
    ELS_CSR=0x68, ELS_CSU=0x69, ELS_LINIT=0x70, ELS_LSTS=0x72, ELS_RNID=0x78,
    ELS_RLIR=0x79, ELS_LIRR=0x7a, ELS_SRL=0x7b, ELS_SBRP=0x7c, ELS_RPSC=0x7d,
    ELS_QSA=0x7e, ELS_EVFP=0x7f, ELS_LKA=0x80, ELS_AUTH_ELS=0x90,
}

pub const FC_ELS_CMDS_INIT: &[(u8, &str)] = &[
    (0x01,"LS_RJT"),(0x02,"LS_ACC"),(0x03,"PLOGI"),(0x04,"FLOGI"),(0x05,"LOGO"),(0x06,"ABTX"),(0x07,"RCS"),(0x08,"RES"),(0x09,"RSS"),(0x0a,"RSI"),(0x0b,"ESTS"),(0x0c,"ESTC"),(0x0d,"ADVC"),(0x0e,"RTV"),(0x0f,"RLS"),(0x10,"ECHO"),(0x11,"TEST"),(0x12,"RRQ"),(0x13,"REC"),(0x14,"SRR"),(0x16,"FPIN"),(0x17,"EDC"),(0x18,"RDP"),(0x19,"RDF"),(0x20,"PRLI"),(0x21,"PRLO"),(0x22,"SCN"),(0x23,"TPLS"),(0x24,"TPRLO"),(0x25,"LCLM"),(0x30,"GAID"),(0x31,"FACT"),(0x32,"FDACDT"),(0x33,"NACT"),(0x34,"NDACT"),(0x40,"QOSR"),(0x41,"RVCS"),(0x50,"PDISC"),(0x51,"FDISC"),(0x52,"ADISC"),(0x53,"RNC"),(0x54,"FARP_REQ"),(0x55,"FARP_REPL"),(0x56,"RPS"),(0x57,"RPL"),(0x58,"RPBC"),(0x60,"FAN"),(0x61,"RSCN"),(0x62,"SCR"),(0x63,"RNFT"),(0x68,"CSR"),(0x69,"CSU"),(0x70,"LINIT"),(0x72,"LSTS"),(0x78,"RNID"),(0x79,"RLIR"),(0x7a,"LIRR"),(0x7b,"SRL"),(0x7c,"SBRP"),(0x7d,"RPSC"),(0x7e,"QSA"),(0x7f,"EVFP"),(0x80,"LKA"),(0x90,"AUTH_ELS")
];

pub type __u8 = u8; pub type __be16 = u16; pub type __be32 = u32; pub type __be64 = u64;
#[repr(C)] pub struct fc_els_ls_acc { pub la_cmd:__u8, pub la_resv:[__u8;3] }
#[repr(C)] pub struct fc_els_ls_rjt { pub er_cmd:__u8,pub er_resv:[__u8;4],pub er_reason:__u8,pub er_explan:__u8,pub er_vendor:__u8 }
#[repr(u32)] #[derive(Copy,Clone,Debug,PartialEq,Eq)] pub enum fc_els_rjt_reason { ELS_RJT_NONE=0,ELS_RJT_INVAL=0x01,ELS_RJT_LOGIC=0x03,ELS_RJT_BUSY=0x05,ELS_RJT_PROT=0x07,ELS_RJT_UNAB=0x09,ELS_RJT_UNSUP=0x0b,ELS_RJT_INPROG=0x0e,ELS_RJT_FIP=0x20,ELS_RJT_VENDOR=0xff }
#[repr(u32)] #[derive(Copy,Clone,Debug,PartialEq,Eq)] pub enum fc_els_rjt_explan { ELS_EXPL_NONE=0,ELS_EXPL_SPP_OPT_ERR=1,ELS_EXPL_SPP_ICTL_ERR=3,ELS_EXPL_AH=0x11,ELS_EXPL_AH_REQ=0x13,ELS_EXPL_SID=0x15,ELS_EXPL_OXID_RXID=0x17,ELS_EXPL_INPROG=0x19,ELS_EXPL_PLOGI_REQD=0x1e,ELS_EXPL_INSUF_RES=0x29,ELS_EXPL_UNAB_DATA=0x2a,ELS_EXPL_UNSUPR=0x2c,ELS_EXPL_INV_LEN=0x2d,ELS_EXPL_NOT_NEIGHBOR=0x62 }
#[repr(u32)] #[derive(Copy,Clone,Debug,PartialEq,Eq)] pub enum fc_ls_tlv_dtag { ELS_DTAG_LS_REQ_INFO=1,ELS_DTAG_LNK_FAULT_CAP=0x0001000d,ELS_DTAG_CG_SIGNAL_CAP=0x0001000f,ELS_DTAG_LNK_INTEGRITY=0x00020001,ELS_DTAG_DELIVERY=0x00020002,ELS_DTAG_PEER_CONGEST=0x00020003,ELS_DTAG_CONGESTION=0x00020004,ELS_DTAG_FPIN_REGISTER=0x00030001 }
#[repr(C)] pub struct fc_tlv_desc { pub desc_tag:__be32,pub desc_len:__be32,pub desc_value:[__u8;0] }
pub const FC_TLV_DESC_HDR_SZ:usize=8;
pub const fn fc_tlv_desc_length_from_sz(sz:usize)->usize { sz-FC_TLV_DESC_HDR_SZ }
pub unsafe fn fc_tlv_next_desc(desc:*mut u8)->*mut u8 { desc.add(((*(desc.add(4) as *const __be32)) as usize)+FC_TLV_DESC_HDR_SZ) }
#[repr(C)] pub struct fc_els_lsri_desc { pub desc_tag:__be32,pub desc_len:__be32,pub rqst_w0:fc_els_lsri_desc_word }
#[repr(C)] pub struct fc_els_lsri_desc_word { pub cmd:__u8,pub bytes:[__u8;3] }
#[repr(C)] pub union fc_els_csp_u { pub sp_plogi:fc_els_csp_plogi,pub sp_flogi_acc:fc_els_csp_flogi_acc }
#[repr(C)] pub struct fc_els_csp_plogi { pub _sp_tot_seq:__be16,pub _sp_rel_off:__be16 }
#[repr(C)] pub struct fc_els_csp_flogi_acc { pub _sp_r_a_tov:__be32 }
#[repr(C)] pub struct fc_els_csp { pub sp_hi_ver:__u8,pub sp_lo_ver:__u8,pub sp_bb_cred:__be16,pub sp_features:__be16,pub sp_bb_data:__be16,pub sp_u:fc_els_csp_u,pub sp_e_d_tov:__be32 }
pub const FC_SP_BB_DATA_MASK:u32=0xfff; pub const FC_SP_FT_NPIV:u32=0x8000; pub const FC_SP_FT_CIRO:u32=0x8000; pub const FC_SP_FT_CLAD:u32=0x8000; pub const FC_SP_FT_RAND:u32=0x4000; pub const FC_SP_FT_VAL:u32=0x2000; pub const FC_SP_FT_NPIV_ACC:u32=0x2000; pub const FC_SP_FT_FPORT:u32=0x1000; pub const FC_SP_FT_ABB:u32=0x0800; pub const FC_SP_FT_EDTR:u32=0x0400; pub const FC_SP_FT_MCAST:u32=0x0200; pub const FC_SP_FT_BCAST:u32=0x0100; pub const FC_SP_FT_HUNT:u32=0x80; pub const FC_SP_FT_SIMP:u32=0x40; pub const FC_SP_FT_SEC:u32=0x20; pub const FC_SP_FT_CSYN:u32=0x10; pub const FC_SP_FT_RTTOV:u32=8; pub const FC_SP_FT_HALF:u32=4; pub const FC_SP_FT_SEQC:u32=2; pub const FC_SP_FT_PAYL:u32=1;
#[repr(C)] pub struct fc_els_cssp { pub cp_class:__be16,pub cp_init:__be16,pub cp_recip:__be16,pub cp_rdfs:__be16,pub cp_con_seq:__be16,pub cp_ee_cred:__be16,pub cp_resv1:__u8,pub cp_open_seq:__u8,pub _cp_resv2:[__u8;2] }
pub const FC_CPC_VALID:u32=0x8000; pub const FC_CPC_IMIX:u32=0x4000; pub const FC_CPC_SEQ:u32=0x0800; pub const FC_CPC_CAMP:u32=0x0200; pub const FC_CPC_PRI:u32=0x80; pub const FC_CPI_CSYN:u32=0x10; pub const FC_CPR_CSYN:u32=8;
#[repr(C,packed)] pub struct fc_els_flogi { pub fl_cmd:__u8,pub _fl_resvd:[__u8;3],pub fl_csp:fc_els_csp,pub fl_wwpn:__be64,pub fl_wwnn:__be64,pub fl_cssp:[fc_els_cssp;4],pub fl_vend:[__u8;16] }
#[repr(C)] pub struct fc_els_spp { pub spp_type:__u8,pub spp_type_ext:__u8,pub spp_flags:__u8,pub _spp_resvd:__u8,pub spp_orig_pa:__be32,pub spp_resp_pa:__be32,pub spp_params:__be32 }
pub const FC_SPP_OPA_VAL:u8=0x80;pub const FC_SPP_RPA_VAL:u8=0x40;pub const FC_SPP_EST_IMG_PAIR:u8=0x20;pub const FC_SPP_RESP_MASK:u8=0x0f;
#[repr(C)] pub struct fc_els_rrq { pub rrq_cmd:__u8,pub rrq_zero:[__u8;3],pub rrq_resvd:__u8,pub rrq_s_id:[__u8;3],pub rrq_ox_id:__be16,pub rrq_rx_id:__be16 }
#[repr(C)] pub struct fc_els_rec { pub rec_cmd:__u8,pub rec_zero:[__u8;3],pub rec_resvd:__u8,pub rec_s_id:[__u8;3],pub rec_ox_id:__be16,pub rec_rx_id:__be16 }
#[repr(C)] pub struct fc_els_rec_acc { pub reca_cmd:__u8,pub reca_zero:[__u8;3],pub reca_ox_id:__be16,pub reca_rx_id:__be16,pub reca_resvd1:__u8,pub reca_ofid:[__u8;3],pub reca_resvd2:__u8,pub reca_rfid:[__u8;3],pub reca_fc4value:__be32,pub reca_e_stat:__be32 }
#[repr(C)] pub struct fc_els_prli { pub prli_cmd:__u8,pub prli_spp_len:__u8,pub prli_len:__be16 }
#[repr(C)] pub struct fc_els_prlo { pub prlo_cmd:__u8,pub prlo_obs:__u8,pub prlo_len:__be16 }
#[repr(C,packed)] pub struct fc_els_adisc { pub adisc_cmd:__u8,pub adisc_resv:[__u8;3],pub adisc_resv1:__u8,pub adisc_hard_addr:[__u8;3],pub adisc_wwpn:__be64,pub adisc_wwnn:__be64,pub adisc_resv2:__u8,pub adisc_port_id:[__u8;3] }
#[repr(C)] pub struct fc_els_logo { pub fl_cmd:__u8,pub fl_zero:[__u8;3],pub fl_resvd:__u8,pub fl_n_port_id:[__u8;3],pub fl_n_port_wwn:__be64 }
#[repr(C)] pub struct fc_els_rtv { pub rtv_cmd:__u8,pub rtv_zero:[__u8;3] }
#[repr(C)] pub struct fc_els_rtv_acc { pub rtv_cmd:__u8,pub rtv_zero:[__u8;3],pub rtv_r_a_tov:__be32,pub rtv_e_d_tov:__be32,pub rtv_toq:__be32 }
pub const FC_ELS_RTV_EDRES:u32=1<<26;pub const FC_ELS_RTV_RTTOV:u32=1<<19;
#[repr(C)] pub struct fc_els_scr { pub scr_cmd:__u8,pub scr_resv:[__u8;6],pub scr_reg_func:__u8 }
#[repr(u8)] pub enum fc_els_scr_func { ELS_SCRF_FAB=1,ELS_SCRF_NPORT=2,ELS_SCRF_FULL=3,ELS_SCRF_CLEAR=255 }
#[repr(C)] pub struct fc_els_rscn { pub rscn_cmd:__u8,pub rscn_page_len:__u8,pub rscn_plen:__be16 }
#[repr(C)] pub struct fc_els_rscn_page { pub rscn_page_flags:__u8,pub rscn_fid:[__u8;3] }
pub const ELS_RSCN_EV_QUAL_BIT:u8=2;pub const ELS_RSCN_EV_QUAL_MASK:u8=0xf;pub const ELS_RSCN_ADDR_FMT_BIT:u8=0;pub const ELS_RSCN_ADDR_FMT_MASK:u8=3;
#[repr(u8)] pub enum fc_els_rscn_ev_qual { ELS_EV_QUAL_NONE=0,ELS_EV_QUAL_NS_OBJ=1,ELS_EV_QUAL_PORT_ATTR=2,ELS_EV_QUAL_SERV_OBJ=3,ELS_EV_QUAL_SW_CONFIG=4,ELS_EV_QUAL_REM_OBJ=5 }
#[repr(u8)] pub enum fc_els_rscn_addr_fmt { ELS_ADDR_FMT_PORT=0,ELS_ADDR_FMT_AREA=1,ELS_ADDR_FMT_DOM=2,ELS_ADDR_FMT_FAB=3 }
#[repr(C)] pub struct fc_els_rnid { pub rnid_cmd:__u8,pub rnid_resv:[__u8;3],pub rnid_fmt:__u8,pub rnid_resv2:[__u8;3] }
#[repr(u8)] pub enum fc_els_rnid_fmt { ELS_RNIDF_NONE=0,ELS_RNIDF_GEN=0xdf }
#[repr(C)] pub struct fc_els_rnid_resp { pub rnid_cmd:__u8,pub rnid_resv:[__u8;3],pub rnid_fmt:__u8,pub rnid_cid_len:__u8,pub rnid_resv2:__u8,pub rnid_sid_len:__u8 }
#[repr(C)] pub struct fc_els_rnid_cid { pub rnid_wwpn:__be64,pub rnid_wwnn:__be64 }
#[repr(C)] pub struct fc_els_rnid_gen { pub rnid_vend_id:[__u8;16],pub rnid_atype:__be32,pub rnid_phys_port:__be32,pub rnid_att_nodes:__be32,pub rnid_node_mgmt:__u8,pub rnid_ip_ver:__u8,pub rnid_prot_port:__be16,pub rnid_ip_addr:[__be32;4],pub rnid_resvd:[__u8;2],pub rnid_vend_spec:__be16 }
#[repr(C)] pub struct fc_els_rpl { pub rpl_cmd:__u8,pub rpl_resv:[__u8;5],pub rpl_max_size:__be16,pub rpl_resv1:__u8,pub rpl_index:[__u8;3] }
#[repr(C)] pub struct fc_els_pnb { pub pnb_phys_pn:__be32,pub pnb_resv:__u8,pub pnb_port_id:[__u8;3],pub pnb_wwpn:__be64 }
#[repr(C)] pub struct fc_els_lesb { pub lesb_link_fail:__be32,pub lesb_sync_loss:__be32,pub lesb_sig_loss:__be32,pub lesb_prim_err:__be32,pub lesb_inv_word:__be32,pub lesb_inv_crc:__be32 }
#[repr(C)] pub struct fc_els_rps { pub rps_cmd:__u8,pub rps_resv:[__u8;2],pub rps_flag:__u8,pub rps_port_spec:__be64 }
#[repr(u8)] pub enum fc_els_rps_flag { FC_ELS_RPS_DID=0,FC_ELS_RPS_PPN=1,FC_ELS_RPS_WWPN=2 }
#[repr(C)] pub struct fc_els_rps_resp { pub rps_cmd:__u8,pub rps_resv:[__u8;2],pub rps_flag:__u8,pub rps_resv2:[__u8;2],pub rps_status:__be16,pub rps_lesb:fc_els_lesb }
#[repr(C)] pub struct fc_els_rls { pub rls_cmd:__u8,pub rls_resv:[__u8;4],pub rls_port_id:[__u8;3] }
#[repr(C)] pub struct fc_els_rls_resp { pub rls_cmd:__u8,pub rls_resv:[__u8;3],pub rls_lesb:fc_els_lesb }
#[repr(C)] pub struct fc_els_lirr { pub lirr_cmd:__u8,pub lirr_resv:[__u8;3],pub lirr_func:__u8,pub lirr_fmt:__u8,pub lirr_resv2:[__u8;2] }
#[repr(C)] pub struct fc_els_srl { pub srl_cmd:__u8,pub srl_resv:[__u8;3],pub srl_flag:__u8,pub srl_flag_param:[__u8;3] }
#[repr(C)] pub struct fc_els_rlir { pub rlir_cmd:__u8,pub rlir_resv:[__u8;3],pub rlir_fmt:__u8,pub rlir_clr_len:__u8,pub rlir_cld_len:__u8,pub rlir_slr_len:__u8 }
#[repr(C)] pub struct fc_els_clid { pub clid_iq:__u8,pub clid_ic:__u8,pub clid_epai:__be16 }
#[repr(u8)] pub enum fc_fpin_li_event_types { FPIN_LI_UNKNOWN=0,FPIN_LI_LINK_FAILURE=1,FPIN_LI_LOSS_OF_SYNC=2,FPIN_LI_LOSS_OF_SIG=3,FPIN_LI_PRIM_SEQ_ERR=4,FPIN_LI_INVALID_TX_WD=5,FPIN_LI_INVALID_CRC=6,FPIN_LI_DEVICE_SPEC=0xf }
#[repr(C)] pub struct fc_els_clir { pub clir_wwpn:__be64,pub clir_wwnn:__be64,pub clir_port_type:__u8,pub clir_port_id:[__u8;3],pub clir_conn_wwpn:__be64,pub clir_conn_wwnn:__be64,pub clir_fab_name:__be64,pub clir_phys_port:__be32,pub clir_trans_id:__be32,pub clir_resv:[__u8;3],pub clir_ts_fmt:__u8,pub clir_timestamp:__be64 }
#[repr(u8)] pub enum fc_els_clir_ts_fmt { ELS_CLIR_TS_UNKNOWN=0,ELS_CLIR_TS_SEC_FRAC=1,ELS_CLIR_TS_CSU=2 }
#[repr(u8)] pub enum fc_els_clid_iq { ELS_CLID_SWITCH=0x20,ELS_CLID_E_PORT=0x10,ELS_CLID_SEV_MASK=0x0c,ELS_CLID_SEV_INFO=0,ELS_CLID_SEV_INOP=8,ELS_CLID_SEV_DEG=4,ELS_CLID_LASER=2,ELS_CLID_FRU=1 }
#[repr(u8)] pub enum fc_els_clid_ic { ELS_CLID_IC_IMPL=1,ELS_CLID_IC_BER=2,ELS_CLID_IC_LOS=3,ELS_CLID_IC_NOS=4,ELS_CLID_IC_PST=5,ELS_CLID_IC_INVAL=6,ELS_CLID_IC_LOOP_TO=7,ELS_CLID_IC_LIP=8 }
pub const FC_ELS_RPS_LPEV:u8=1; pub const FC_ELS_RPS_PTP:u16=1<<5; pub const FC_ELS_RPS_LOOP:u16=1<<4; pub const FC_ELS_RPS_FAB:u16=1<<3; pub const FC_ELS_RPS_NO_SIG:u16=1<<2; pub const FC_ELS_RPS_NO_SYNC:u16=1<<1; pub const FC_ELS_RPS_RESET:u16=1;
pub const ELS_LIRR_SET_COND:u8=1;pub const ELS_LIRR_SET_UNCOND:u8=2;pub const ELS_LIRR_CLEAR:u8=0xff;pub const FC_ELS_SRL_ALL:u8=0;pub const FC_ELS_SRL_ONE:u8=1;pub const FC_ELS_SRL_EN_PER:u8=2;pub const FC_ELS_SRL_DIS_PER:u8=3;
#[repr(u8)] #[derive(Copy,Clone,Debug,PartialEq,Eq)] pub enum fc_els_spp_resp { FC_SPP_RESP_ACK=1,FC_SPP_RESP_RES=2,FC_SPP_RESP_INIT=3,FC_SPP_RESP_NO_PA=4,FC_SPP_RESP_CONF=5,FC_SPP_RESP_COND=6,FC_SPP_RESP_MULT=7,FC_SPP_RESP_INVL=8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
