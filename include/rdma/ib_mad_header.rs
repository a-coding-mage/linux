/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2004-2006 the original authors. */

/* Translated from ib_mad.h. External kernel types and byte-order helpers are
 * intentionally referenced rather than reimplemented here. */

pub const IB_MGMT_BASE_VERSION: u8 = 1;
pub const OPA_MGMT_BASE_VERSION: u8 = 0x80;
pub const OPA_SM_CLASS_VERSION: u8 = 0x80;
pub const IB_MGMT_CLASS_SUBN_LID_ROUTED: u8 = 0x01;
pub const IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE: u8 = 0x81;
pub const IB_MGMT_CLASS_SUBN_ADM: u8 = 0x03;
pub const IB_MGMT_CLASS_PERF_MGMT: u8 = 0x04;
pub const IB_MGMT_CLASS_BM: u8 = 0x05;
pub const IB_MGMT_CLASS_DEVICE_MGMT: u8 = 0x06;
pub const IB_MGMT_CLASS_CM: u8 = 0x07;
pub const IB_MGMT_CLASS_SNMP: u8 = 0x08;
pub const IB_MGMT_CLASS_DEVICE_ADM: u8 = 0x10;
pub const IB_MGMT_CLASS_BOOT_MGMT: u8 = 0x11;
pub const IB_MGMT_CLASS_BIS: u8 = 0x12;
pub const IB_MGMT_CLASS_CONG_MGMT: u8 = 0x21;
pub const IB_MGMT_CLASS_VENDOR_RANGE2_START: u8 = 0x30;
pub const IB_MGMT_CLASS_VENDOR_RANGE2_END: u8 = 0x4f;
pub const IB_OPENIB_OUI: u32 = 0x001405;
pub const IB_MGMT_METHOD_GET: u8 = 0x01;
pub const IB_MGMT_METHOD_SET: u8 = 0x02;
pub const IB_MGMT_METHOD_GET_RESP: u8 = 0x81;
pub const IB_MGMT_METHOD_SEND: u8 = 0x03;
pub const IB_MGMT_METHOD_TRAP: u8 = 0x05;
pub const IB_MGMT_METHOD_REPORT: u8 = 0x06;
pub const IB_MGMT_METHOD_REPORT_RESP: u8 = 0x86;
pub const IB_MGMT_METHOD_TRAP_REPRESS: u8 = 0x07;
pub const IB_MGMT_METHOD_GET_TABLE: u8 = 0x12;
pub const IB_MGMT_METHOD_RESP: u8 = 0x80;
pub const IB_BM_ATTR_MOD_RESP: u32 = 1u32.to_be();
pub const IB_MGMT_MAX_METHODS: usize = 128;
pub const IB_MGMT_MAD_STATUS_SUCCESS: u16 = 0x0000;
pub const IB_MGMT_MAD_STATUS_BUSY: u16 = 0x0001;
pub const IB_MGMT_MAD_STATUS_REDIRECT_REQD: u16 = 0x0002;
pub const IB_MGMT_MAD_STATUS_BAD_VERSION: u16 = 0x0004;
pub const IB_MGMT_MAD_STATUS_UNSUPPORTED_METHOD: u16 = 0x0008;
pub const IB_MGMT_MAD_STATUS_UNSUPPORTED_METHOD_ATTRIB: u16 = 0x000c;
pub const IB_MGMT_MAD_STATUS_INVALID_ATTRIB_VALUE: u16 = 0x001c;
pub const IB_MGMT_RMPP_VERSION: u8 = 1;
pub const IB_MGMT_RMPP_TYPE_DATA: u8 = 1;
pub const IB_MGMT_RMPP_TYPE_ACK: u8 = 2;
pub const IB_MGMT_RMPP_TYPE_STOP: u8 = 3;
pub const IB_MGMT_RMPP_TYPE_ABORT: u8 = 4;
pub const IB_MGMT_RMPP_FLAG_ACTIVE: u8 = 1;
pub const IB_MGMT_RMPP_FLAG_FIRST: u8 = 1 << 1;
pub const IB_MGMT_RMPP_FLAG_LAST: u8 = 1 << 2;
pub const IB_MGMT_RMPP_NO_RESPTIME: u8 = 0x1f;
pub const IB_MGMT_RMPP_STATUS_SUCCESS: u8 = 0;
pub const IB_MGMT_RMPP_STATUS_RESX: u8 = 1;
pub const IB_MGMT_RMPP_STATUS_ABORT_MIN: u8 = 118;
pub const IB_MGMT_RMPP_STATUS_T2L: u8 = 118;
pub const IB_MGMT_RMPP_STATUS_BAD_LEN: u8 = 119;
pub const IB_MGMT_RMPP_STATUS_BAD_SEG: u8 = 120;
pub const IB_MGMT_RMPP_STATUS_BADT: u8 = 121;
pub const IB_MGMT_RMPP_STATUS_W2S: u8 = 122;
pub const IB_MGMT_RMPP_STATUS_S2B: u8 = 123;
pub const IB_MGMT_RMPP_STATUS_BAD_STATUS: u8 = 124;
pub const IB_MGMT_RMPP_STATUS_UNV: u8 = 125;
pub const IB_MGMT_RMPP_STATUS_TMR: u8 = 126;
pub const IB_MGMT_RMPP_STATUS_UNSPEC: u8 = 127;
pub const IB_MGMT_RMPP_STATUS_ABORT_MAX: u8 = 127;
pub const IB_QP0: u32 = 0;
pub const IB_QP1: u32 = 1u32.to_be();
pub const IB_QP1_QKEY: u32 = 0x80010000;
pub const IB_QP_SET_QKEY: u32 = 0x80000000;
pub const IB_DEFAULT_PKEY_PARTIAL: u16 = 0x7fff;
pub const IB_DEFAULT_PKEY_FULL: u16 = 0xffff;
pub const IB_NOTICE_TYPE_FATAL: u8 = 0x80;
pub const IB_NOTICE_TYPE_URGENT: u8 = 0x81;
pub const IB_NOTICE_TYPE_SECURITY: u8 = 0x82;
pub const IB_NOTICE_TYPE_SM: u8 = 0x83;
pub const IB_NOTICE_TYPE_INFO: u8 = 0x84;
pub const IB_NOTICE_PROD_CA: u16 = 1u16.to_be();
pub const IB_NOTICE_PROD_SWITCH: u16 = 2u16.to_be();
pub const IB_NOTICE_PROD_ROUTER: u16 = 3u16.to_be();
pub const IB_NOTICE_PROD_CLASS_MGR: u16 = 4u16.to_be();

pub const IB_MGMT_MAD_HDR: usize = 24;
pub const IB_MGMT_MAD_DATA: usize = 232;
pub const IB_MGMT_RMPP_HDR: usize = 36;
pub const IB_MGMT_RMPP_DATA: usize = 220;
pub const IB_MGMT_VENDOR_HDR: usize = 40;
pub const IB_MGMT_VENDOR_DATA: usize = 216;
pub const IB_MGMT_SA_HDR: usize = 56;
pub const IB_MGMT_SA_DATA: usize = 200;
pub const IB_MGMT_DEVICE_HDR: usize = 64;
pub const IB_MGMT_DEVICE_DATA: usize = 192;
pub const IB_MGMT_MAD_SIZE: usize = IB_MGMT_MAD_HDR + IB_MGMT_MAD_DATA;
pub const OPA_MGMT_MAD_DATA: usize = 2024;
pub const OPA_MGMT_RMPP_DATA: usize = 2012;
pub const OPA_MGMT_MAD_SIZE: usize = IB_MGMT_MAD_HDR + OPA_MGMT_MAD_DATA;

#[repr(C)]
pub struct ib_mad_hdr { pub base_version: u8, pub mgmt_class: u8, pub class_version: u8, pub method: u8, pub status: __be16, pub class_specific: __be16, pub tid: __be64, pub attr_id: __be16, pub resv: __be16, pub attr_mod: __be32 }
#[repr(C)]
pub struct ib_rmpp_hdr { pub rmpp_version: u8, pub rmpp_type: u8, pub rmpp_rtime_flags: u8, pub rmpp_status: u8, pub seg_num: __be32, pub paylen_newwin: __be32 }
pub type ib_sa_comp_mask = u64;
pub const fn IB_SA_COMP_MASK(n: u32) -> ib_sa_comp_mask { (1u64 << n).to_be() }
#[repr(C, packed)]
pub struct ib_sa_hdr { pub sm_key: __be64, pub attr_offset: __be16, pub reserved: __be16, pub comp_mask: ib_sa_comp_mask }
#[repr(C)]
pub struct ib_mad { pub mad_hdr: ib_mad_hdr, pub data: [u8; IB_MGMT_MAD_DATA] }
#[repr(C)]
pub struct opa_mad { pub mad_hdr: ib_mad_hdr, pub data: [u8; OPA_MGMT_MAD_DATA] }
#[repr(C)]
pub struct ib_rmpp_mad { pub mad_hdr: ib_mad_hdr, pub rmpp_hdr: ib_rmpp_hdr, pub data: [u8; IB_MGMT_RMPP_DATA] }
#[repr(C)]
pub struct opa_rmpp_mad { pub mad_hdr: ib_mad_hdr, pub rmpp_hdr: ib_rmpp_hdr, pub data: [u8; OPA_MGMT_RMPP_DATA] }
#[repr(C, packed)]
pub struct ib_sa_mad { pub mad_hdr: ib_mad_hdr, pub rmpp_hdr: ib_rmpp_hdr, pub sa_hdr: ib_sa_hdr, pub data: [u8; IB_MGMT_SA_DATA] }
#[repr(C)]
pub struct ib_vendor_mad { pub mad_hdr: ib_mad_hdr, pub rmpp_hdr: ib_rmpp_hdr, pub reserved: u8, pub oui: [u8; 3], pub data: [u8; IB_MGMT_VENDOR_DATA] }

pub const IB_MGMT_CLASSPORTINFO_ATTR_ID: u16 = 1u16.to_be();
pub const IB_CLASS_PORT_INFO_RESP_TIME_MASK: u32 = 0x1f;
pub const IB_CLASS_PORT_INFO_RESP_TIME_FIELD_SIZE: u32 = 5;
#[repr(C)]
pub struct ib_class_port_info { pub base_version: u8, pub class_version: u8, pub capability_mask: __be16, pub cap_mask2_resp_time: __be32, pub redirect_gid: [u8;16], pub redirect_tcslfl: __be32, pub redirect_lid: __be16, pub redirect_pkey: __be16, pub redirect_qp: __be32, pub redirect_qkey: __be32, pub trap_gid: [u8;16], pub trap_tcslfl: __be32, pub trap_lid: __be16, pub trap_pkey: __be16, pub trap_hlqp: __be32, pub trap_qkey: __be32 }

#[repr(u32)]
pub enum ib_port_capability_mask_bits { IB_PORT_SM=1<<1, IB_PORT_NOTICE_SUP=1<<2, IB_PORT_TRAP_SUP=1<<3, IB_PORT_OPT_IPD_SUP=1<<4, IB_PORT_AUTO_MIGR_SUP=1<<5, IB_PORT_SL_MAP_SUP=1<<6, IB_PORT_MKEY_NVRAM=1<<7, IB_PORT_PKEY_NVRAM=1<<8, IB_PORT_LED_INFO_SUP=1<<9, IB_PORT_SM_DISABLED=1<<10, IB_PORT_SYS_IMAGE_GUID_SUP=1<<11, IB_PORT_PKEY_SW_EXT_PORT_TRAP_SUP=1<<12, IB_PORT_EXTENDED_SPEEDS_SUP=1<<14, IB_PORT_CAP_MASK2_SUP=1<<15, IB_PORT_CM_SUP=1<<16, IB_PORT_SNMP_TUNNEL_SUP=1<<17, IB_PORT_REINIT_SUP=1<<18, IB_PORT_DEVICE_MGMT_SUP=1<<19, IB_PORT_VENDOR_CLASS_SUP=1<<20, IB_PORT_DR_NOTICE_SUP=1<<21, IB_PORT_CAP_MASK_NOTICE_SUP=1<<22, IB_PORT_BOOT_MGMT_SUP=1<<23, IB_PORT_LINK_LATENCY_SUP=1<<24, IB_PORT_CLIENT_REG_SUP=1<<25, IB_PORT_OTHER_LOCAL_CHANGES_SUP=1<<26, IB_PORT_LINK_SPEED_WIDTH_TABLE_SUP=1<<27, IB_PORT_VENDOR_SPECIFIC_MADS_TABLE_SUP=1<<28, IB_PORT_MCAST_PKEY_TRAP_SUPPRESSION_SUP=1<<29, IB_PORT_MCAST_FDB_TOP_SUP=1<<30, IB_PORT_HIERARCHY_INFO_SUP=1u32<<31 }
#[repr(u32)]
pub enum ib_port_capability_mask2_bits { IB_PORT_SET_NODE_DESC_SUP=1, IB_PORT_EX_PORT_INFO_EX_SUP=1<<1, IB_PORT_VIRT_SUP=1<<2, IB_PORT_SWITCH_PORT_STATE_TABLE_SUP=1<<3, IB_PORT_LINK_WIDTH_2X_SUP=1<<4, IB_PORT_LINK_SPEED_HDR_SUP=1<<5, IB_PORT_LINK_SPEED_NDR_SUP=1<<10, IB_PORT_EXTENDED_SPEEDS2_SUP=1<<11, IB_PORT_LINK_SPEED_XDR_SUP=1<<12 }
pub const OPA_CLASS_PORT_INFO_PR_SUPPORT: u32 = 1 << 26;
#[repr(C, packed)]
pub struct opa_class_port_info { pub base_version:u8, pub class_version:u8, pub cap_mask:__be16, pub cap_mask2_resp_time:__be32, pub redirect_gid:[u8;16], pub redirect_tc_fl:__be32, pub redirect_lid:__be32, pub redirect_sl_qp:__be32, pub redirect_qkey:__be32, pub trap_gid:[u8;16], pub trap_tc_fl:__be32, pub trap_lid:__be32, pub trap_hl_qp:__be32, pub trap_qkey:__be32, pub trap_pkey:__be16, pub redirect_pkey:__be16, pub trap_sl_rsvd:u8, pub reserved:[u8;3] }

#[inline] pub unsafe fn ib_get_cpi_resp_time(cpi: *mut ib_class_port_info) -> u8 { u32::from_be((*cpi).cap_mask2_resp_time) as u8 & IB_CLASS_PORT_INFO_RESP_TIME_MASK as u8 }
#[inline] pub unsafe fn ib_set_cpi_resp_time(cpi:*mut ib_class_port_info, rtime:u8) { (*cpi).cap_mask2_resp_time = (u32::from_be((*cpi).cap_mask2_resp_time) & !IB_CLASS_PORT_INFO_RESP_TIME_MASK | (rtime as u32 & IB_CLASS_PORT_INFO_RESP_TIME_MASK)).to_be(); }
#[inline] pub unsafe fn ib_get_cpi_capmask2(cpi:*mut ib_class_port_info) -> u32 { u32::from_be((*cpi).cap_mask2_resp_time) >> IB_CLASS_PORT_INFO_RESP_TIME_FIELD_SIZE }
#[inline] pub unsafe fn ib_set_cpi_capmask2(cpi:*mut ib_class_port_info, capmask2:u32) { (*cpi).cap_mask2_resp_time = ((u32::from_be((*cpi).cap_mask2_resp_time) & IB_CLASS_PORT_INFO_RESP_TIME_MASK) | (capmask2 << IB_CLASS_PORT_INFO_RESP_TIME_FIELD_SIZE)).to_be(); }
#[inline] pub unsafe fn opa_get_cpi_capmask2(cpi:*mut opa_class_port_info) -> u32 { u32::from_be((*cpi).cap_mask2_resp_time) >> IB_CLASS_PORT_INFO_RESP_TIME_FIELD_SIZE }

#[repr(C)] pub struct ib_mad_notice_attr { pub generic_type:u8, pub prod_type_msb:u8, pub prod_type_lsb:__be16, pub trap_num:__be16, pub issuer_lid:__be16, pub toggle_count:__be16, pub details: ib_mad_notice_details }
#[repr(C)] pub union ib_mad_notice_details { pub raw_data: ib_mad_notice_raw, pub ntc_129_131: ib_mad_notice_129_131, pub ntc_144: ib_mad_notice_144, pub ntc_145: ib_mad_notice_145, pub ntc_256: ib_mad_notice_256, pub ntc_257_258: ib_mad_notice_257_258 }
#[repr(C)] pub struct ib_mad_notice_raw { pub details:[u8;54] }
#[repr(C, packed)] pub struct ib_mad_notice_129_131 { pub reserved:__be16, pub lid:__be16, pub port_num:u8 }
#[repr(C, packed)] pub struct ib_mad_notice_144 { pub reserved:__be16, pub lid:__be16, pub reserved2:u8, pub local_changes:u8, pub new_cap_mask:__be32, pub reserved3:u8, pub change_flags:u8 }
#[repr(C, packed)] pub struct ib_mad_notice_145 { pub reserved:__be16, pub lid:__be16, pub reserved2:__be16, pub new_sys_guid:__be64 }
#[repr(C, packed)] pub struct ib_mad_notice_256 { pub reserved:__be16, pub lid:__be16, pub dr_slid:__be16, pub method:u8, pub reserved2:u8, pub attr_id:__be16, pub attr_mod:__be32, pub mkey:__be64, pub reserved3:u8, pub dr_trunc_hop:u8, pub dr_rtn_path:[u8;30] }
#[repr(C, packed)] pub struct ib_mad_notice_257_258 { pub reserved:__be16, pub lid1:__be16, pub lid2:__be16, pub key:__be32, pub sl_qp1:__be32, pub qp2:__be32, pub gid1: union_ib_gid, pub gid2: union_ib_gid }

pub type ib_mad_send_handler = unsafe extern "C" fn(*mut ib_mad_agent, *mut ib_mad_send_wc);
pub type ib_mad_recv_handler = unsafe extern "C" fn(*mut ib_mad_agent, *mut ib_mad_send_buf, *mut ib_mad_recv_wc);
#[repr(C)] pub struct ib_mad_send_buf { pub next:*mut ib_mad_send_buf, pub mad:*mut core::ffi::c_void, pub mad_agent:*mut ib_mad_agent, pub ah:*mut ib_ah, pub context:[*mut core::ffi::c_void;2], pub hdr_len:i32, pub data_len:i32, pub seg_count:i32, pub seg_size:i32, pub seg_rmpp_size:i32, pub timeout_ms:i32, pub retries:i32 }
extern "C" { pub fn ib_response_mad(hdr:*const ib_mad_hdr) -> i32; }
#[inline] pub unsafe fn ib_get_rmpp_resptime(hdr:*mut ib_rmpp_hdr)->u8 { (*hdr).rmpp_rtime_flags >> 3 }
#[inline] pub unsafe fn ib_get_rmpp_flags(hdr:*mut ib_rmpp_hdr)->u8 { (*hdr).rmpp_rtime_flags & 7 }
#[inline] pub unsafe fn ib_set_rmpp_resptime(hdr:*mut ib_rmpp_hdr,rtime:u8) { (*hdr).rmpp_rtime_flags=ib_get_rmpp_flags(hdr)|(rtime<<3); }
#[inline] pub unsafe fn ib_set_rmpp_flags(hdr:*mut ib_rmpp_hdr,flags:u8) { (*hdr).rmpp_rtime_flags=((*hdr).rmpp_rtime_flags&0xf8)|(flags&7); }

#[repr(C)] pub struct ib_mad_agent { pub device:*mut ib_device, pub qp:*mut ib_qp, pub recv_handler:ib_mad_recv_handler, pub send_handler:ib_mad_send_handler, pub context:*mut core::ffi::c_void, pub hi_tid:u32, pub flags:u32, pub security:*mut core::ffi::c_void, pub mad_agent_sec_list:list_head, pub port_num:u8, pub rmpp_version:u8, pub smp_allowed:bool }
pub const IB_MAD_USER_RMPP:u32 = IB_USER_MAD_USER_RMPP;
#[repr(C)] pub struct ib_mad_send_wc { pub send_buf:*mut ib_mad_send_buf, pub status:ib_wc_status, pub vendor_err:u32 }
#[repr(C)] pub struct ib_mad_recv_buf { pub list:list_head, pub grh:*mut ib_grh, pub mad:ib_mad_recv_union }
#[repr(C)] pub union ib_mad_recv_union { pub mad:*mut ib_mad, pub opa_mad:*mut opa_mad }
#[repr(C)] pub struct ib_mad_recv_wc { pub wc:*mut ib_wc, pub recv_buf:ib_mad_recv_buf, pub rmpp_list:list_head, pub mad_len:i32, pub mad_seg_size:usize }
#[repr(C)] pub struct ib_mad_reg_req { pub mgmt_class:u8, pub mgmt_class_version:u8, pub oui:[u8;3], pub method_mask:[usize; IB_MGMT_MAX_METHODS / (usize::BITS as usize)] }
extern "C" { pub fn ib_register_mad_agent(device:*mut ib_device,port_num:u32,qp_type:ib_qp_type,mad_reg_req:*mut ib_mad_reg_req,rmpp_version:u8,send_handler:ib_mad_send_handler,recv_handler:ib_mad_recv_handler,context:*mut core::ffi::c_void,registration_flags:u32)->*mut ib_mad_agent; pub fn ib_unregister_mad_agent(agent:*mut ib_mad_agent); pub fn ib_post_send_mad(send_buf:*mut ib_mad_send_buf,bad_send_buf:*mut *mut ib_mad_send_buf)->i32; pub fn ib_free_recv_mad(recv_wc:*mut ib_mad_recv_wc); pub fn ib_modify_mad(send_buf:*mut ib_mad_send_buf,timeout_ms:u32)->i32; pub fn ib_create_send_mad(agent:*mut ib_mad_agent,remote_qpn:u32,pkey_index:u16,rmpp_active:i32,hdr_len:i32,data_len:i32,gfp_mask:gfp_t,base_version:u8)->*mut ib_mad_send_buf; pub fn ib_is_mad_class_rmpp(mgmt_class:u8)->i32; pub fn ib_get_mad_data_offset(mgmt_class:u8)->i32; pub fn ib_get_rmpp_segment(send_buf:*mut ib_mad_send_buf,seg_num:i32)->*mut core::ffi::c_void; pub fn ib_free_send_mad(send_buf:*mut ib_mad_send_buf); pub fn ib_mad_kernel_rmpp_agent(agent:*const ib_mad_agent)->i32; }
#[inline] pub unsafe fn ib_cancel_mad(send_buf:*mut ib_mad_send_buf) { ib_modify_mad(send_buf,0); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
