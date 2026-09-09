/* Translated from cvmx-wqe.h. */

use core::ffi::c_int;

/* Dependency supplied by asm/octeon/cvmx-packet.h. */
/* use crate::cvmx_packet::*; */

#[inline]
pub const fn oct_tag_type_string(x: u64) -> &'static str {
    if x == CVMX_POW_TAG_TYPE_ORDERED { "ORDERED" }
    else if x == CVMX_POW_TAG_TYPE_ATOMIC { "ATOMIC" }
    else if x == CVMX_POW_TAG_TYPE_NULL { "NULL" }
    else { "NULL_NULL" }
}

/* External constants supplied by the packet/POW headers. */
pub const CVMX_POW_TAG_TYPE_ORDERED: u64 = 0;
pub const CVMX_POW_TAG_TYPE_ATOMIC: u64 = 1;
pub const CVMX_POW_TAG_TYPE_NULL: u64 = 2;

/* C bit-fields are represented as their containing 64-bit words; field
 * declarations below preserve the source names and intended widths. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_wqe_word2_s {
    pub u64: u64,
    pub bufs: u8, pub ip_offset: u8, pub vlan_valid: u8, pub vlan_stacked: u8,
    pub unassigned: u8, pub vlan_cfi: u8, pub vlan_id: u16, pub pr: u8,
    pub unassigned2: u16, pub dec_ipcomp: u8, pub tcp_or_udp: u8,
    pub dec_ipsec: u8, pub is_v6: u8, pub software: u8, pub l4_error: u8,
    pub is_frag: u8, pub ip_exc: u8, pub is_bcast: u8, pub is_mcast: u8,
    pub not_ip: u8, pub rcv_error: u8, pub err_code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_wqe_word2_s_cn68xx {
    pub u64: u64,
    pub bufs: u8, pub ip_offset: u8, pub vlan_valid: u8, pub vlan_stacked: u8,
    pub unassigned: u8, pub vlan_cfi: u8, pub vlan_id: u16, pub port: u16,
    pub dec_ipcomp: u8, pub tcp_or_udp: u8, pub dec_ipsec: u8, pub is_v6: u8,
    pub software: u8, pub l4_error: u8, pub is_frag: u8, pub ip_exc: u8,
    pub is_bcast: u8, pub is_mcast: u8, pub not_ip: u8, pub rcv_error: u8,
    pub err_code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_wqe_word2_svlan { pub u64: u64, pub unused1: u16, pub vlan: u16, pub unused2: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_wqe_word2_snoip {
    pub u64: u64, pub bufs: u8, pub unused: u8, pub vlan_valid: u8,
    pub vlan_stacked: u8, pub unassigned: u8, pub vlan_cfi: u8, pub vlan_id: u16,
    pub pr: u8, pub unassigned2: u16, pub software: u8, pub unassigned3: u8,
    pub is_rarp: u8, pub is_arp: u8, pub is_bcast: u8, pub is_mcast: u8,
    pub not_ip: u8, pub rcv_error: u8, pub err_code: u8,
}

#[repr(C)]
pub union cvmx_pip_wqe_word2 {
    pub u64: u64,
    pub s: cvmx_pip_wqe_word2_s,
    pub s_cn68xx: cvmx_pip_wqe_word2_s_cn68xx,
    pub svlan: cvmx_pip_wqe_word2_svlan,
    pub snoip: cvmx_pip_wqe_word2_snoip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_wqe_word0_cn38xx { pub hw_chksum: u16, pub unused: u8, pub next_ptr: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_wqe_word0_cn68xx { pub l4ptr: u8, pub unused0: u8, pub l3ptr: u8, pub l2ptr: u8, pub unused1: u32, pub bpid: u8, pub unused2: u8, pub pknd: u8 }
#[repr(C)]
pub union cvmx_pip_wqe_word0 { pub cn38xx: cvmx_pip_wqe_word0_cn38xx, pub cn68xx: cvmx_pip_wqe_word0_cn68xx }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_wqe_word1_tag { pub len: u16, pub varies: u16, pub tag_type: u8, pub tag: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_wqe_word1_cn68xx { pub len: u16, pub zero_0: u8, pub qos: u8, pub zero_1: u8, pub grp: u8, pub zero_2: u8, pub tag_type: u8, pub tag: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_wqe_word1_cn38xx { pub len: u16, pub ipprt: u8, pub qos: u8, pub grp: u8, pub tag_type: u8, pub tag: u32 }
#[repr(C)]
pub union cvmx_wqe_word1 { pub u64: u64, pub s: cvmx_wqe_word1_tag, pub cn68xx: cvmx_wqe_word1_cn68xx, pub cn38xx: cvmx_wqe_word1_cn38xx }

#[repr(C, align(128))]
pub struct cvmx_wqe {
    pub word0: cvmx_wqe_word0,
    pub word1: cvmx_wqe_word1,
    pub word2: cvmx_pip_wqe_word2,
    pub packet_ptr: cvmx_buf_ptr,
    pub packet_data: [u8; 96],
}

extern "C" {
    pub fn octeon_has_feature(feature: c_int) -> c_int;
}
pub const OCTEON_FEATURE_CN68XX_WQE: c_int = 0;

#[inline] pub unsafe fn cvmx_wqe_get_port(work: *mut cvmx_wqe) -> c_int { let _ = work; 0 }
#[inline] pub unsafe fn cvmx_wqe_set_port(work: *mut cvmx_wqe, port: c_int) { let _ = (work, port); }
#[inline] pub unsafe fn cvmx_wqe_get_grp(work: *mut cvmx_wqe) -> c_int { let _ = work; 0 }
#[inline] pub unsafe fn cvmx_wqe_set_grp(work: *mut cvmx_wqe, grp: c_int) { let _ = (work, grp); }
#[inline] pub unsafe fn cvmx_wqe_get_qos(work: *mut cvmx_wqe) -> c_int { let _ = work; 0 }
#[inline] pub unsafe fn cvmx_wqe_set_qos(work: *mut cvmx_wqe, qos: c_int) { let _ = (work, qos); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
