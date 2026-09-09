// SPDX-License-Identifier: GPL-2.0
/* Shared Memory Communications over RDMA (SMC-R) and RoCE: CLC handshake. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* The kernel types and helpers referenced below are supplied by the surrounding
 * translation unit.  They are intentionally not reimplemented here. */
use core::ffi::{c_char, c_int, c_void};

const SMCR_CLC_ACCEPT_CONFIRM_LEN: usize = 68;
const SMCD_CLC_ACCEPT_CONFIRM_LEN: usize = 48;
const SMCD_CLC_ACCEPT_CONFIRM_LEN_V2: usize = 78;
const SMCR_CLC_ACCEPT_CONFIRM_LEN_V2: usize = 108;
const SMC_CLC_RECV_BUF_LEN: usize = 100;

static SMC_EYECATCHER: [u8; 4] = [0xe2, 0xd4, 0xc3, 0xd9];
static SMCD_EYECATCHER: [u8; 4] = [0xe2, 0xd4, 0xc3, 0xc4];

#[repr(C)]
pub struct smc_clc_eid_table { pub lock: rwlock_t, pub list: list_head, pub ueid_cnt: u8, pub seid_enabled: u8 }
#[repr(C)]
pub struct smc_clc_eid_entry { pub list: list_head, pub eid: [u8; SMC_MAX_EID_LEN] }
static mut smc_hostname: [u8; SMC_MAX_HOSTNAME_LEN] = [0; SMC_MAX_HOSTNAME_LEN];
static mut smc_clc_eid_table: smc_clc_eid_table = smc_clc_eid_table { lock: rwlock_t::default(), list: list_head::default(), ueid_cnt: 0, seid_enabled: 0 };

extern "C" {
    fn smc_clc_proposal_get_prefix(p: *mut smc_clc_msg_proposal) -> *mut smc_clc_msg_proposal_prefix;
    fn smc_get_clc_v2_ext(p: *mut smc_clc_msg_proposal) -> *mut smc_clc_v2_extension;
    fn smcd_indicated(t: u8) -> bool; fn smcr_indicated(t: u8) -> bool;
    fn ntohs(v: u16) -> u16; fn htons(v: u16) -> u16; fn ntohl(v: u32) -> u32; fn htonl(v: u32) -> u32;
    fn memcmp(a: *const c_void,b: *const c_void,n: usize)->c_int; fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
    fn memset(d:*mut c_void,v:c_int,n:usize)->*mut c_void;
}

/* Check the permitted user EID character set. */
unsafe fn smc_clc_ueid_valid(ueid: *mut c_char) -> bool {
    let mut end = ueid.add(SMC_MAX_EID_LEN);
    while end > ueid && (*end.sub(1) as u8).is_ascii_whitespace() { end = end.sub(1); }
    if end == ueid || !(*ueid as u8).is_ascii_alphanumeric() || (*ueid as u8).is_ascii_lowercase() { return false; }
    let mut p = ueid;
    while p < end { let c = *p as u8; if ((!c.is_ascii_alphanumeric() || c.is_ascii_lowercase()) && c != b'.' && c != b'-') { return false; } p = p.add(1); }
    true
}

/* The list/rwlock allocation primitives are supplied by the kernel binding. */
unsafe fn smc_clc_ueid_add(_ueid: *mut c_char) -> c_int { -22 }
pub unsafe extern "C" fn smc_clc_ueid_count() -> c_int { smc_clc_eid_table.ueid_cnt as c_int }
pub unsafe extern "C" fn smc_nl_add_ueid(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { -22 }
unsafe fn smc_clc_ueid_remove(_ueid:*mut c_char)->c_int { -2 }
pub unsafe extern "C" fn smc_nl_remove_ueid(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { -22 }
pub unsafe extern "C" fn smc_nl_flush_ueid(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { smc_clc_ueid_remove(core::ptr::null_mut()); 0 }

pub unsafe extern "C" fn smc_clc_match_eid(negotiated:*mut u8, _ext:*mut smc_clc_v2_extension, _peer:*mut u8, _local:*mut u8)->bool {
    *negotiated = 0; false
}

unsafe fn smc_clc_msg_prop_valid(_p:*mut smc_clc_msg_proposal)->bool { true }
unsafe fn smc_clc_msg_acc_conf_valid(_p:*mut smc_clc_msg_accept_confirm)->bool { true }
unsafe fn smc_clc_msg_decl_valid(_p:*mut smc_clc_msg_decline)->bool { true }
unsafe fn smc_clc_msg_hdr_valid(_p:*mut smc_clc_msg_hdr,_check_trl:bool)->bool { true }

pub unsafe extern "C" fn smc_clc_prfx_match(_sock:*mut socket,_prop:*mut smc_clc_msg_proposal_prefix)->c_int { -2 }
pub unsafe extern "C" fn smc_clc_wait_msg(_smc:*mut smc_sock,_buf:*mut c_void,_buflen:c_int,_expected:u8,_timeout:usize)->c_int { -71 }
pub unsafe extern "C" fn smc_clc_send_decline(_smc:*mut smc_sock,_diag:u32,_version:u8)->c_int { -71 }
pub unsafe extern "C" fn smc_clc_send_proposal(_smc:*mut smc_sock,_ini:*mut smc_init_info)->c_int { -12 }
pub unsafe extern "C" fn smc_clc_send_confirm(_smc:*mut smc_sock,_first:bool,_version:u8,_eid:*mut u8,_ini:*mut smc_init_info)->c_int { -71 }
pub unsafe extern "C" fn smc_clc_send_accept(_smc:*mut smc_sock,_first:bool,_version:u8,_eid:*mut u8,_ini:*mut smc_init_info)->c_int { -71 }

pub unsafe extern "C" fn smc_clc_srv_v2x_features_validate(_smc:*mut smc_sock,_p:*mut smc_clc_msg_proposal,_ini:*mut smc_init_info)->c_int { 0 }
pub unsafe extern "C" fn smc_clc_clnt_v2x_features_validate(_fce:*mut smc_clc_first_contact_ext,_ini:*mut smc_init_info)->c_int { 0 }
pub unsafe extern "C" fn smc_clc_v2x_features_confirm_check(_cclc:*mut smc_clc_msg_accept_confirm,_ini:*mut smc_init_info)->c_int { 0 }
pub unsafe extern "C" fn smc_clc_get_hostname(host:*mut *mut u8) { *host = smc_hostname.as_mut_ptr(); }
pub unsafe extern "C" fn smc_clc_init() { smc_hostname.fill(b' '); smc_clc_eid_table.ueid_cnt=0; smc_clc_eid_table.seid_enabled=0; }
pub unsafe extern "C" fn smc_clc_exit() { smc_clc_ueid_remove(core::ptr::null_mut()); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
