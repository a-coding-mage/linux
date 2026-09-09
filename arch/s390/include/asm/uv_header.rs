/* SPDX-License-Identifier: GPL-2.0 */
/* Ultravisor Interfaces. Rust translation of uv.h. */

pub const UVC_CC_OK: u16 = 0;
pub const UVC_CC_ERROR: u16 = 1;
pub const UVC_CC_BUSY: u16 = 2;
pub const UVC_CC_PARTIAL: u16 = 3;

pub const UVC_RC_EXECUTED: u16 = 0x0001;
pub const UVC_RC_INV_CMD: u16 = 0x0002;
pub const UVC_RC_INV_STATE: u16 = 0x0003;
pub const UVC_RC_INV_LEN: u16 = 0x0005;
pub const UVC_RC_NO_RESUME: u16 = 0x0007;
pub const UVC_RC_MORE_DATA: u16 = 0x0100;
pub const UVC_RC_NEED_DESTROY: u16 = 0x8000;

pub const UVC_CMD_QUI: u16 = 0x0001;
pub const UVC_CMD_QUERY_KEYS: u16 = 0x0002;
pub const UVC_CMD_INIT_UV: u16 = 0x000f;
pub const UVC_CMD_CREATE_SEC_CONF: u16 = 0x0100;
pub const UVC_CMD_DESTROY_SEC_CONF: u16 = 0x0101;
pub const UVC_CMD_DESTROY_SEC_CONF_FAST: u16 = 0x0102;
pub const UVC_CMD_CREATE_SEC_CPU: u16 = 0x0120;
pub const UVC_CMD_DESTROY_SEC_CPU: u16 = 0x0121;
pub const UVC_CMD_CONV_TO_SEC_STOR: u16 = 0x0200;
pub const UVC_CMD_CONV_FROM_SEC_STOR: u16 = 0x0201;
pub const UVC_CMD_DESTR_SEC_STOR: u16 = 0x0202;
pub const UVC_CMD_SET_SEC_CONF_PARAMS: u16 = 0x0300;
pub const UVC_CMD_UNPACK_IMG: u16 = 0x0301;
pub const UVC_CMD_VERIFY_IMG: u16 = 0x0302;
pub const UVC_CMD_CPU_RESET: u16 = 0x0310;
pub const UVC_CMD_CPU_RESET_INITIAL: u16 = 0x0311;
pub const UVC_CMD_PREPARE_RESET: u16 = 0x0320;
pub const UVC_CMD_CPU_RESET_CLEAR: u16 = 0x0321;
pub const UVC_CMD_CPU_SET_STATE: u16 = 0x0330;
pub const UVC_CMD_SET_UNSHARE_ALL: u16 = 0x0340;
pub const UVC_CMD_PIN_PAGE_SHARED: u16 = 0x0341;
pub const UVC_CMD_UNPIN_PAGE_SHARED: u16 = 0x0342;
pub const UVC_CMD_DUMP_INIT: u16 = 0x0400;
pub const UVC_CMD_DUMP_CONF_STOR_STATE: u16 = 0x0401;
pub const UVC_CMD_DUMP_CPU: u16 = 0x0402;
pub const UVC_CMD_DUMP_COMPLETE: u16 = 0x0403;
pub const UVC_CMD_SET_SHARED_ACCESS: u16 = 0x1000;
pub const UVC_CMD_REMOVE_SHARED_ACCESS: u16 = 0x1001;
pub const UVC_CMD_RETR_ATTEST: u16 = 0x1020;
pub const UVC_CMD_ADD_SECRET: u16 = 0x1031;
pub const UVC_CMD_LIST_SECRETS: u16 = 0x1033;
pub const UVC_CMD_LOCK_SECRETS: u16 = 0x1034;
pub const UVC_CMD_RETR_SECRET: u16 = 0x1035;

#[repr(u8)]
pub enum UvCmdsInst { BIT_UVC_CMD_QUI=0, BIT_UVC_CMD_INIT_UV=1, BIT_UVC_CMD_CREATE_SEC_CONF=2, BIT_UVC_CMD_DESTROY_SEC_CONF=3, BIT_UVC_CMD_CREATE_SEC_CPU=4, BIT_UVC_CMD_DESTROY_SEC_CPU=5, BIT_UVC_CMD_CONV_TO_SEC_STOR=6, BIT_UVC_CMD_CONV_FROM_SEC_STOR=7, BIT_UVC_CMD_SET_SHARED_ACCESS=8, BIT_UVC_CMD_REMOVE_SHARED_ACCESS=9, BIT_UVC_CMD_SET_SEC_PARMS=11, BIT_UVC_CMD_UNPACK_IMG=13, BIT_UVC_CMD_VERIFY_IMG=14, BIT_UVC_CMD_CPU_RESET=15, BIT_UVC_CMD_CPU_RESET_INITIAL=16, BIT_UVC_CMD_CPU_SET_STATE=17, BIT_UVC_CMD_PREPARE_RESET=18, BIT_UVC_CMD_CPU_PERFORM_CLEAR_RESET=19, BIT_UVC_CMD_UNSHARE_ALL=20, BIT_UVC_CMD_PIN_PAGE_SHARED=21, BIT_UVC_CMD_UNPIN_PAGE_SHARED=22, BIT_UVC_CMD_DESTROY_SEC_CONF_FAST=23, BIT_UVC_CMD_DUMP_INIT=24, BIT_UVC_CMD_DUMP_CONFIG_STOR_STATE=25, BIT_UVC_CMD_DUMP_CPU=26, BIT_UVC_CMD_DUMP_COMPLETE=27, BIT_UVC_CMD_RETR_ATTEST=28, BIT_UVC_CMD_ADD_SECRET=29, BIT_UVC_CMD_LIST_SECRETS=30, BIT_UVC_CMD_LOCK_SECRETS=31, BIT_UVC_CMD_RETR_SECRET=33, BIT_UVC_CMD_QUERY_KEYS=34 }
#[repr(u8)]
pub enum UvFeatInd { BIT_UV_FEAT_MISC=0, BIT_UV_FEAT_AIV=1, BIT_UV_FEAT_AP=4, BIT_UV_FEAT_AP_INTR=5 }

#[repr(C, packed(1), align(8))]
pub struct uv_cb_header { pub len:u16, pub cmd:u16, pub rc:u16, pub rrc:u16 }
#[repr(C, packed(1), align(8))]
pub struct uv_cb_qui { pub header:uv_cb_header, pub reserved08:u64, pub inst_calls_list:[u64;4], pub reserved30:[u64;2], pub uv_base_stor_len:u64, pub reserved48:u64, pub conf_base_phys_stor_len:u64, pub conf_base_virt_stor_len:u64, pub conf_virt_var_stor_len:u64, pub cpu_stor_len:u64, pub reserved70:[u32;3], pub max_num_sec_conf:u32, pub max_guest_stor_addr:u64, pub reserved88:[u8;0x16], pub max_guest_cpu_id:u16, pub uv_feature_indications:u64, pub reserveda8:u64, pub supp_se_hdr_versions:u64, pub supp_se_hdr_pcf:u64, pub reservedc0:u64, pub conf_dump_storage_state_len:u64, pub conf_dump_finalize_len:u64, pub reservedd8:u64, pub supp_att_req_hdr_ver:u64, pub supp_att_pflags:u64, pub reservedf0:u64, pub supp_add_secret_req_ver:u64, pub supp_add_secret_pcf:u64, pub supp_secret_types:u64, pub max_assoc_secrets:u16, pub max_retr_secrets:u16, pub reserved114:[u8;12] }
#[repr(C, packed(1), align(8))] pub struct uv_key_hash { pub dword:[u64;4] }
pub const UVC_QUERY_KEYS_IDX_HK:u32=0; pub const UVC_QUERY_KEYS_IDX_BACK_HK:u32=1;
#[repr(C, packed(1), align(8))] pub struct uv_cb_query_keys { pub header:uv_cb_header, pub reserved08:[u64;3], pub key_hashes:[uv_key_hash;15] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_init { pub header:uv_cb_header, pub reserved08:[u64;2], pub stor_origin:u64, pub stor_len:u64, pub reserved28:[u64;4] }

#[repr(C)] pub union uv_cgc_flags { pub raw:u16 }
#[repr(C, packed(1), align(8))] pub struct uv_cb_cgc { pub header:uv_cb_header, pub reserved08:[u64;2], pub guest_handle:u64, pub conf_base_stor_origin:u64, pub conf_virt_stor_origin:u64, pub reserved30:[u8;6], pub flags:uv_cgc_flags, pub guest_stor_origin:u64, pub guest_stor_len:u64, pub guest_sca:u64, pub guest_asce:u64, pub reserved58:[u64;5] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_csc { pub header:uv_cb_header, pub reserved08:[u64;2], pub cpu_handle:u64, pub guest_handle:u64, pub stor_origin:u64, pub reserved30:[u8;6], pub num:u16, pub state_origin:u64, pub reserved40:[u64;4] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_cts { pub header:uv_cb_header, pub reserved08:[u64;2], pub guest_handle:u64, pub gaddr:u64 }
#[repr(C, packed(1), align(8))] pub struct uv_cb_cfs { pub header:uv_cb_header, pub reserved08:[u64;2], pub paddr:u64 }
#[repr(C, packed(1), align(8))] pub struct uv_cb_ssc { pub header:uv_cb_header, pub reserved08:[u64;2], pub guest_handle:u64, pub sec_header_origin:u64, pub sec_header_len:u32, pub reserved2c:u32, pub reserved30:[u64;4] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_unp { pub header:uv_cb_header, pub reserved08:[u64;2], pub guest_handle:u64, pub gaddr:u64, pub tweak:[u64;2], pub reserved38:[u64;3] }
pub const PV_CPU_STATE_OPR:u8=1; pub const PV_CPU_STATE_STP:u8=2; pub const PV_CPU_STATE_CHKSTP:u8=3; pub const PV_CPU_STATE_OPR_LOAD:u8=5;
#[repr(C, packed(1), align(8))] pub struct uv_cb_cpu_set_state { pub header:uv_cb_header, pub reserved08:[u64;2], pub cpu_handle:u64, pub reserved20:[u8;7], pub state:u8, pub reserved28:[u64;5] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_nodata { pub header:uv_cb_header, pub reserved08:[u64;2], pub handle:u64, pub reserved20:[u64;4] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_destroy_fast { pub header:uv_cb_header, pub reserved08:[u64;2], pub handle:u64, pub reserved20:[u64;5] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_share { pub header:uv_cb_header, pub reserved08:[u64;3], pub paddr:u64, pub reserved28:u64 }
#[repr(C, packed(1), align(8))] pub struct uv_cb_attest { pub header:uv_cb_header, pub reserved08:[u64;2], pub arcb_addr:u64, pub cont_token:u64, pub reserved28:[u8;6], pub user_data_len:u16, pub user_data:[u8;256], pub reserved130:[u32;3], pub meas_len:u32, pub meas_addr:u64, pub config_uid:[u8;16], pub reserved158:u32, pub add_data_len:u32, pub add_data_addr:u64, pub reserved168:[u64;4] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_dump_cpu { pub header:uv_cb_header, pub reserved08:[u64;2], pub cpu_handle:u64, pub dump_area_origin:u64, pub reserved28:[u64;5] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_dump_stor_state { pub header:uv_cb_header, pub reserved08:[u64;2], pub config_handle:u64, pub dump_area_origin:u64, pub gaddr:u64, pub reserved28:[u64;4] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_dump_complete { pub header:uv_cb_header, pub reserved08:[u64;2], pub config_handle:u64, pub dump_area_origin:u64, pub reserved30:[u64;5] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_guest_addr { pub header:uv_cb_header, pub reserved08:[u64;3], pub addr:u64, pub reserved28:[u64;4] }
pub const UVC_RC_RETR_SECR_BUF_SMALL:u16=0x0109; pub const UVC_RC_RETR_SECR_STORE_EMPTY:u16=0x010f; pub const UVC_RC_RETR_SECR_INV_IDX:u16=0x0110; pub const UVC_RC_RETR_SECR_INV_SECRET:u16=0x0111;
#[repr(C, packed(1), align(8))] pub struct uv_cb_retr_secr { pub header:uv_cb_header, pub reserved08:[u64;2], pub secret_idx:u16, pub reserved1a:u16, pub buf_size:u32, pub buf_addr:u64, pub reserved28:[u64;4] }
#[repr(C, packed(1), align(8))] pub struct uv_cb_list_secrets { pub header:uv_cb_header, pub reserved08:[u64;2], pub reserved18:[u8;6], pub start_idx:u16, pub list_addr:u64, pub reserved28:[u64;4] }
#[repr(u16)] pub enum UvSecretTypes { UV_SECRET_INVAL=0, UV_SECRET_NULL=1, UV_SECRET_ASSOCIATION=2, UV_SECRET_PLAIN=3, UV_SECRET_AES_128=4, UV_SECRET_AES_192=5, UV_SECRET_AES_256=6, UV_SECRET_AES_XTS_128=7, UV_SECRET_AES_XTS_256=8, UV_SECRET_HMAC_SHA_256=9, UV_SECRET_HMAC_SHA_512=10, UV_SECRET_ECDSA_P256=0x11, UV_SECRET_ECDSA_P384=0x12, UV_SECRET_ECDSA_P521=0x13, UV_SECRET_ECDSA_ED25519=0x14, UV_SECRET_ECDSA_ED448=0x15 }
#[repr(C, packed(1), align(8))] pub struct uv_secret_list_item_hdr { pub index:u16, pub type_:u16, pub length:u32 }
pub const UV_SECRET_ID_LEN:usize=32;
#[repr(C, packed(1), align(8))] pub struct uv_secret_list_item { pub hdr:uv_secret_list_item_hdr, pub reserverd08:u64, pub id:[u8;UV_SECRET_ID_LEN] }
#[repr(C, packed(1), align(8))] pub struct uv_secret_list { pub num_secr_stored:u16, pub total_num_secrets:u16, pub next_secret_idx:u16, pub reserved_06:u16, pub reserved_08:u64, pub secrets:[uv_secret_list_item;85] }

/* The following declarations retain dependencies supplied by the surrounding kernel translation. */
extern "C" { pub fn __uv_call(r1:usize, r2:usize) -> i32; pub static mut uv_info: uv_info_t; pub static mut prot_virt_guest:i32; pub static mut prot_virt_host:i32; }
#[repr(C)] pub struct uv_info_t { pub inst_calls_list:[usize;4], pub uv_base_stor_len:usize, pub guest_base_stor_len:usize, pub guest_virt_base_stor_len:usize, pub guest_virt_var_stor_len:usize, pub guest_cpu_stor_len:usize, pub max_sec_stor_addr:usize, pub max_num_sec_conf:u32, pub max_guest_cpu_id:u16, pub uv_feature_indications:usize, pub supp_se_hdr_ver:usize, pub supp_se_hdr_pcf:usize, pub conf_dump_storage_state_len:usize, pub conf_dump_finalize_len:usize, pub supp_att_req_hdr_ver:usize, pub supp_att_pflags:usize, pub supp_add_secret_req_ver:usize, pub supp_add_secret_pcf:usize, pub supp_secret_types:usize, pub max_assoc_secrets:u16, pub max_retr_secrets:u16 }
pub unsafe fn uv_call(r1:usize,r2:usize)->i32 { loop { let cc=__uv_call(r1,r2); if cc<=1{return cc;} } }
pub unsafe fn is_prot_virt_guest()->i32 { prot_virt_guest }
pub unsafe fn is_prot_virt_host()->i32 { prot_virt_host }
pub unsafe fn uv_has_feature(feature_bit:u8)->bool {
    if (feature_bit as usize) >= core::mem::size_of::<usize>() * 8 { return false; }
    (uv_info.uv_feature_indications & (1usize << feature_bit)) != 0
}
pub unsafe fn uv_cmd_nodata(handle:u64,cmd:u16,rc:*mut u16,rrc:*mut u16)->i32 {
    let mut uvcb=uv_cb_nodata { header:uv_cb_header { len:core::mem::size_of::<uv_cb_nodata>() as u16, cmd, rc:0, rrc:0 }, reserved08:[0;2], handle, reserved20:[0;4] };
    let cc=uv_call(0,&mut uvcb as *mut _ as usize);
    if !rc.is_null() {*rc=uvcb.header.rc;} if !rrc.is_null() {*rrc=uvcb.header.rrc;}
    if cc != 0 {-22} else {0}
}
pub unsafe fn uv_list_secrets(buf:*mut uv_secret_list,start_idx:u16,rc:*mut u16,rrc:*mut u16)->i32 {
    let mut uvcb=uv_cb_list_secrets { header:uv_cb_header { len:core::mem::size_of::<uv_cb_list_secrets>() as u16, cmd:UVC_CMD_LIST_SECRETS, rc:0, rrc:0 }, reserved08:[0;2], reserved18:[0;6], start_idx, list_addr:buf as u64, reserved28:[0;4] };
    let cc=uv_call(0,&mut uvcb as *mut _ as usize);
    if !rc.is_null() {*rc=uvcb.header.rc;} if !rrc.is_null() {*rrc=uvcb.header.rrc;} cc
}
pub unsafe fn share(addr:usize,cmd:u16)->i32 {
    let mut uvcb=uv_cb_share { header:uv_cb_header { len:core::mem::size_of::<uv_cb_share>() as u16, cmd, rc:0, rrc:0 }, reserved08:[0;3], paddr:addr as u64, reserved28:0 };
    if is_prot_virt_guest()==0 {-95} else { uv_call(0,&mut uvcb as *mut _ as usize) }
}
pub unsafe fn uv_set_shared(addr:usize)->i32 { share(addr,UVC_CMD_SET_SHARED_ACCESS) }
pub unsafe fn uv_remove_shared(addr:usize)->i32 { share(addr,UVC_CMD_REMOVE_SHARED_ACCESS) }
extern "C" { pub fn uv_find_secret(secret_id:*const u8,list:*mut uv_secret_list,secret:*mut uv_secret_list_item_hdr)->i32; pub fn uv_retrieve_secret(secret_idx:u16,buf:*mut u8,buf_size:usize)->i32; pub fn uv_pin_shared(paddr:usize)->i32; pub fn uv_destroy_folio(folio:*mut core::ffi::c_void)->i32; pub fn uv_destroy_pte(pte:usize)->i32; pub fn uv_convert_from_secure_pte(pte:usize)->i32; pub fn s390_wiggle_split_folio(mm:*mut core::ffi::c_void,folio:*mut core::ffi::c_void)->i32; pub fn __make_folio_secure(folio:*mut core::ffi::c_void,uvcb:*mut uv_cb_header)->i32; pub fn uv_convert_from_secure(paddr:usize)->i32; pub fn uv_convert_from_secure_folio(folio:*mut core::ffi::c_void)->i32; pub fn uv_alloc_stor_var(size:usize)->*mut core::ffi::c_void; pub fn uv_free_stor_var(stor_var:*mut core::ffi::c_void); pub fn setup_uv(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
