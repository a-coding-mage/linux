/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Translation of ib_sa.h; included kernel symbols are external dependencies. */

pub const OPA_SA_CLASS_VERSION: u8 = 0x80;
pub const IB_SA_CLASS_VERSION: u32 = 2;
pub const IB_SA_METHOD_GET_TABLE: u8 = 0x12;
pub const IB_SA_METHOD_GET_TABLE_RESP: u8 = 0x92;
pub const IB_SA_METHOD_DELETE: u8 = 0x15;
pub const IB_SA_METHOD_DELETE_RESP: u8 = 0x95;
pub const IB_SA_METHOD_GET_MULTI: u8 = 0x14;
pub const IB_SA_METHOD_GET_MULTI_RESP: u8 = 0x94;
pub const IB_SA_METHOD_GET_TRACE_TBL: u8 = 0x13;

#[repr(u32)] pub enum IbSaSelector { IB_SA_GT=0, IB_SA_LT=1, IB_SA_EQ=2, IB_SA_BEST=3 }
#[repr(u32)] pub enum IbSaMcJoinStates { FULLMEMBER_JOIN, NONMEMBER_JOIN, SENDONLY_NONMEBER_JOIN, SENDONLY_FULLMEMBER_JOIN, NUM_JOIN_MEMBERSHIP_TYPES }
#[repr(u32)] pub enum SaPathRecType { SA_PATH_REC_TYPE_IB, SA_PATH_REC_TYPE_ROCE_V1, SA_PATH_REC_TYPE_ROCE_V2, SA_PATH_REC_TYPE_OPA }

pub const IB_SA_CAP_MASK2_SENDONLY_FULL_MEM_SUPPORT: u32 = 1 << 12;
macro_rules! IB_SA_COMP_MASK { ($n:expr) => { 1u64 << $n }; }
pub const IB_DEFAULT_SERVICE_LEASE: u32 = 0xffff_ffff;

#[repr(C)] pub struct SaPathRecIb { pub dlid: u16, pub slid: u16, pub raw_traffic: u8 }
#[repr(C)] pub struct SaPathRecRoce { pub route_resolved: bool, pub dmac: [u8; 6] }
#[repr(C)] pub struct SaPathRecOpa { pub dlid: u32, pub slid: u32, pub raw_traffic: u8, pub l2_8B:u8, pub l2_10B:u8, pub l2_9B:u8, pub l2_16B:u8, pub qos_type:u8, pub qos_priority:u8 }

#[repr(C)] pub union SaPathRecVariant { pub ib: SaPathRecIb, pub roce: SaPathRecRoce, pub opa: SaPathRecOpa }
#[repr(C)] pub struct SaPathRec { pub dgid: IbGid, pub sgid: IbGid, pub service_id:u64, pub flow_label:u32, pub hop_limit:u8, pub traffic_class:u8, pub reversible:u8, pub numb_path:u8, pub pkey:u16, pub qos_class:u16, pub sl:u8, pub mtu_selector:u8, pub mtu:u8, pub rate_selector:u8, pub rate:u8, pub packet_life_time_selector:u8, pub packet_life_time:u8, pub preference:u8, pub variant: SaPathRecVariant, pub rec_type: SaPathRecType, pub flags:u32 }
#[repr(C)] pub struct SaServiceRec { pub id:u64, pub gid:[u8;16], pub pkey:u16, pub reserved:[u8;2], pub lease:u32, pub key:[u8;16], pub name:[u8;64], pub data_8:[u8;16], pub data_16:[u16;8], pub data_32:[u32;4], pub data_64:[u64;2] }
#[repr(C)] pub struct IbSaMcMemberRec { pub mgid:IbGid, pub port_gid:IbGid, pub qkey:u32, pub mlid:u16, pub mtu_selector:u8, pub mtu:u8, pub traffic_class:u8, pub pkey:u16, pub rate_selector:u8, pub rate:u8, pub packet_life_time_selector:u8, pub packet_life_time:u8, pub sl:u8, pub flow_label:u32, pub hop_limit:u8, pub scope:u8, pub join_state:u8, pub proxy_join:u8 }
#[repr(C)] pub struct IbSaGuidInfoRec { pub lid:u16, pub block_num:u8, pub res1:u8, pub res2:u32, pub guid_info_list:[u8;64] }
#[repr(C)] pub struct IbSaClient { pub users: AtomicT, pub comp: Completion }
pub struct IbSaQuery;
#[repr(C)] pub struct IbSaMulticast { pub rec:IbSaMcMemberRec, pub comp_mask:IbSaCompMask, pub callback: Option<unsafe extern "C" fn(i32,*mut IbSaMulticast)->i32>, pub context:*mut core::ffi::c_void }

#[repr(C)] pub struct IbGid { pub global: IbGidGlobal }
#[repr(C)] pub struct IbGidGlobal { pub subnet_prefix:u64, pub interface_id:u64 }
pub type IbSaCompMask = u64;
pub type AtomicT = core::ffi::c_void; pub type Completion = core::ffi::c_void;
pub enum IbGidType { IB_GID_TYPE_IB, IB_GID_TYPE_ROCE, IB_GID_TYPE_ROCE_UDP_ENCAP }

pub unsafe fn sa_conv_pathrec_to_gid_type(rec:*mut SaPathRec)->IbGidType { match (*rec).rec_type { SaPathRecType::SA_PATH_REC_TYPE_ROCE_V1=>IbGidType::IB_GID_TYPE_ROCE, SaPathRecType::SA_PATH_REC_TYPE_ROCE_V2=>IbGidType::IB_GID_TYPE_ROCE_UDP_ENCAP, _=>IbGidType::IB_GID_TYPE_IB } }
pub fn sa_conv_gid_to_pathrec_type(t:IbGidType)->SaPathRecType { match t { IbGidType::IB_GID_TYPE_ROCE=>SaPathRecType::SA_PATH_REC_TYPE_ROCE_V1, IbGidType::IB_GID_TYPE_ROCE_UDP_ENCAP=>SaPathRecType::SA_PATH_REC_TYPE_ROCE_V2, _=>SaPathRecType::SA_PATH_REC_TYPE_IB } }

extern "C" {
    pub fn ib_sa_register_client(client:*mut IbSaClient); pub fn ib_sa_unregister_client(client:*mut IbSaClient);
    pub fn ib_sa_cancel_query(id:i32, query:*mut IbSaQuery);
    pub fn ib_sa_pack_path(rec:*mut SaPathRec, attribute:*mut core::ffi::c_void); pub fn ib_sa_pack_service(rec:*mut SaServiceRec, attribute:*mut core::ffi::c_void);
    pub fn ib_sa_unpack_service(attribute:*mut core::ffi::c_void, rec:*mut SaServiceRec); pub fn ib_sa_unpack_path(attribute:*mut core::ffi::c_void, rec:*mut SaPathRec);
    pub fn ib_sa_free_multicast(multicast:*mut IbSaMulticast);
}

#[inline] pub unsafe fn sa_path_is_roce(r:*const SaPathRec)->bool { matches!((*r).rec_type, SaPathRecType::SA_PATH_REC_TYPE_ROCE_V1|SaPathRecType::SA_PATH_REC_TYPE_ROCE_V2) }
#[inline] pub unsafe fn sa_path_is_opa(r:*const SaPathRec)->bool { (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_OPA }
#[inline] pub unsafe fn sa_path_set_slid(r:*mut SaPathRec, v:u32) { if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_IB { (*r).variant.ib.slid=v as u16 } else if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_OPA { (*r).variant.opa.slid=v } }
#[inline] pub unsafe fn sa_path_set_dlid(r:*mut SaPathRec, v:u32) { if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_IB { (*r).variant.ib.dlid=v as u16 } else if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_OPA { (*r).variant.opa.dlid=v } }
#[inline] pub unsafe fn sa_path_set_raw_traffic(r:*mut SaPathRec,v:u8) { if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_IB { (*r).variant.ib.raw_traffic=v } else if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_OPA { (*r).variant.opa.raw_traffic=v } }
#[inline] pub unsafe fn sa_path_get_slid(r:*const SaPathRec)->u32 { if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_IB { (*r).variant.ib.slid as u32 } else if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_OPA { (*r).variant.opa.slid } else { 0 } }
#[inline] pub unsafe fn sa_path_get_dlid(r:*const SaPathRec)->u32 { if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_IB { (*r).variant.ib.dlid as u32 } else if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_OPA { (*r).variant.opa.dlid } else { 0 } }
#[inline] pub unsafe fn sa_path_get_raw_traffic(r:*const SaPathRec)->u8 { if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_IB { (*r).variant.ib.raw_traffic } else if (*r).rec_type==SaPathRecType::SA_PATH_REC_TYPE_OPA { (*r).variant.opa.raw_traffic } else { 0 } }
#[inline] pub unsafe fn sa_path_set_dmac(r:*mut SaPathRec,d:*const u8) { if sa_path_is_roce(r) { core::ptr::copy_nonoverlapping(d,(*r).variant.roce.dmac.as_mut_ptr(),6); } }
#[inline] pub unsafe fn sa_path_set_dmac_zero(r:*mut SaPathRec) { if sa_path_is_roce(r) { (*r).variant.roce.dmac=[0;6]; } }
#[inline] pub unsafe fn sa_path_get_dmac(r:*mut SaPathRec)->*mut u8 { if sa_path_is_roce(r) { (*r).variant.roce.dmac.as_mut_ptr() } else { core::ptr::null_mut() } }

/* Component-mask declarations (IB_SA_COMP_MASK is supplied by ib_verbs.h). */
pub const IB_SA_PATH_REC_SERVICE_ID:u64=IB_SA_COMP_MASK!(0)|IB_SA_COMP_MASK!(1);
pub const IB_SA_PATH_REC_DGID:u64=IB_SA_COMP_MASK!(2); pub const IB_SA_PATH_REC_SGID:u64=IB_SA_COMP_MASK!(3);
pub const IB_SA_PATH_REC_DLID:u64=IB_SA_COMP_MASK!(4); pub const IB_SA_PATH_REC_SLID:u64=IB_SA_COMP_MASK!(5);
pub const IB_SA_PATH_REC_RAW_TRAFFIC:u64=IB_SA_COMP_MASK!(6); pub const IB_SA_PATH_REC_FLOW_LABEL:u64=IB_SA_COMP_MASK!(8);
pub const IB_SA_PATH_REC_HOP_LIMIT:u64=IB_SA_COMP_MASK!(9); pub const IB_SA_PATH_REC_TRAFFIC_CLASS:u64=IB_SA_COMP_MASK!(10);
pub const IB_SA_PATH_REC_REVERSIBLE:u64=IB_SA_COMP_MASK!(11); pub const IB_SA_PATH_REC_NUMB_PATH:u64=IB_SA_COMP_MASK!(12);
pub const IB_SA_PATH_REC_PKEY:u64=IB_SA_COMP_MASK!(13); pub const IB_SA_PATH_REC_QOS_CLASS:u64=IB_SA_COMP_MASK!(14);
pub const IB_SA_PATH_REC_SL:u64=IB_SA_COMP_MASK!(15); pub const IB_SA_PATH_REC_MTU_SELECTOR:u64=IB_SA_COMP_MASK!(16);
pub const IB_SA_PATH_REC_MTU:u64=IB_SA_COMP_MASK!(17); pub const IB_SA_PATH_REC_RATE_SELECTOR:u64=IB_SA_COMP_MASK!(18);
pub const IB_SA_PATH_REC_RATE:u64=IB_SA_COMP_MASK!(19); pub const IB_SA_PATH_REC_PACKET_LIFE_TIME_SELECTOR:u64=IB_SA_COMP_MASK!(20);
pub const IB_SA_PATH_REC_PACKET_LIFE_TIME:u64=IB_SA_COMP_MASK!(21); pub const IB_SA_PATH_REC_PREFERENCE:u64=IB_SA_COMP_MASK!(22);
pub const IB_SA_MCMEMBER_REC_MGID:u64=IB_SA_COMP_MASK!(0); pub const IB_SA_MCMEMBER_REC_PORT_GID:u64=IB_SA_COMP_MASK!(1);
pub const IB_SA_MCMEMBER_REC_QKEY:u64=IB_SA_COMP_MASK!(2); pub const IB_SA_MCMEMBER_REC_MLID:u64=IB_SA_COMP_MASK!(3);
pub const IB_SA_MCMEMBER_REC_JOIN_STATE:u64=IB_SA_COMP_MASK!(16); pub const IB_SA_MCMEMBER_REC_PROXY_JOIN:u64=IB_SA_COMP_MASK!(17);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
