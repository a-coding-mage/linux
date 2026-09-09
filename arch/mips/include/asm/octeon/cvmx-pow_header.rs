/* Rust translation of cvmx-pow.h.  Hardware bitfields are represented by
 * raw 64-bit words; accessors and external definitions are supplied by the
 * surrounding OCTEON bindings. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

pub const CVMX_ENABLE_POW_CHECKS: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pow_tag_req_t { pub u64: u64, pub s: cvmx_pow_tag_req_s }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct cvmx_pow_tag_req_s {
    pub tag: u32, pub r#type: u64, pub grp: u64, pub qos: u64,
    pub op: u64, pub index: u64, pub no_sched: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pow_load_addr_t {
    pub u64: u64,
    pub swork: cvmx_pow_load_addr_swork,
    pub sstatus: cvmx_pow_load_addr_sstatus,
    pub smemload: cvmx_pow_load_addr_smemload,
    pub sindexload: cvmx_pow_load_addr_sindexload,
    pub snull_rd: cvmx_pow_load_addr_snull_rd,
}
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_load_addr_swork { pub mem_region:u64,pub is_io:u64,pub did:u64,pub wait:u64 }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_load_addr_sstatus { pub mem_region:u64,pub is_io:u64,pub did:u64,pub coreid:u64,pub get_rev:u64,pub get_cur:u64,pub get_wqp:u64 }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_load_addr_smemload { pub mem_region:u64,pub is_io:u64,pub did:u64,pub index:u64,pub get_des:u64,pub get_wqp:u64 }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_load_addr_sindexload { pub mem_region:u64,pub is_io:u64,pub did:u64,pub qosgrp:u64,pub get_des_get_tail:u64,pub get_rmt:u64 }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_load_addr_snull_rd { pub mem_region:u64,pub is_io:u64,pub did:u64 }

#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_pow_tag_load_resp_t { pub u64:u64, pub s_work:cvmx_pow_resp_work, pub s_sstatus2:cvmx_pow_resp_status2, pub s_sstatus4:cvmx_pow_resp_wqp, pub s_null_rd:cvmx_pow_resp_null }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_resp_work { pub addr:u64,pub no_work:u64 }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_resp_status2 { pub tag:u32,pub tag_type:u64,pub tail:u64,pub head:u64,pub grp:u64,pub index:u64,pub link_index:u64 }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_resp_wqp { pub wqp:u64,pub grp:u64,pub index:u64,pub link_index:u64 }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_resp_null { pub state:u64 }

#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_pow_tag_store_addr_t { pub u64:u64, pub stag:cvmx_pow_store_addr_stag }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_store_addr_stag { pub mem_reg:u64,pub is_io:u64,pub did:u64,pub offset:u64,pub addr:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union cvmx_pow_iobdma_store_t { pub u64:u64, pub s:cvmx_pow_iobdma_s }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct cvmx_pow_iobdma_s { pub scraddr:u64,pub len:u64,pub did:u64,pub wait:u64 }

#[repr(i32)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum cvmx_pow_tag_type { CVMX_POW_TAG_TYPE_ORDERED=0, CVMX_POW_TAG_TYPE_ATOMIC=1, CVMX_POW_TAG_TYPE_NULL=2, CVMX_POW_TAG_TYPE_NULL_NULL=3 }
#[repr(i32)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum cvmx_pow_wait { CVMX_POW_NO_WAIT=0, CVMX_POW_WAIT=1 }
pub const CVMX_POW_TAG_OP_SWTAG:u64=0; pub const CVMX_POW_TAG_OP_SWTAG_FULL:u64=1; pub const CVMX_POW_TAG_OP_SWTAG_DESCH:u64=2; pub const CVMX_POW_TAG_OP_DESCH:u64=3; pub const CVMX_POW_TAG_OP_ADDWQ:u64=4; pub const CVMX_POW_TAG_OP_UPDATE_WQP_GRP:u64=5; pub const CVMX_POW_TAG_OP_SET_NSCHED:u64=6; pub const CVMX_POW_TAG_OP_CLR_NSCHED:u64=7; pub const CVMX_POW_TAG_OP_NOP:u64=15;
pub const CVMX_TAG_SW_BITS:u32=8; pub const CVMX_TAG_SW_SHIFT:u32=24; pub const CVMX_TAG_SW_BITS_INTERNAL:u32=1; pub const CVMX_TAG_SUBGROUP_MASK:u32=0xffff; pub const CVMX_TAG_SUBGROUP_SHIFT:u32=16; pub const CVMX_TAG_SUBGROUP_PKO:u32=1;

extern "C" { pub fn cvmx_pow_capture(buffer:*mut core::ffi::c_void, size:i32)->i32; pub fn cvmx_pow_display(buffer:*mut core::ffi::c_void,size:i32); pub fn cvmx_pow_get_num_entries()->i32; }
#[inline] pub fn cvmx_pow_tag_compose(sw_bits:u64,hw_bits:u64)->u32 { (((sw_bits & 0xff)<<24)|(hw_bits & 0xffffff)) as u32 }
#[inline] pub fn cvmx_pow_tag_get_sw_bits(tag:u64)->u32 { ((tag>>24)&0xff) as u32 }
#[inline] pub fn cvmx_pow_tag_get_hw_bits(tag:u64)->u32 { (tag&0xffffff) as u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
