/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 * Definitions for LLC (link layer control) message handling
 *
 * Translated from smc_llc.h.  Types and symbols supplied by smc_wr.h remain
 * external dependencies of this header.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

/* The C header includes "smc_wr.h". */

pub const SMC_LLC_FLAG_RESP: u8 = 0x80;
pub const SMC_LLC_WAIT_FIRST_TIME: u32 = 5 * HZ;
pub const SMC_LLC_WAIT_TIME: u32 = 2 * HZ;
pub const SMC_LLC_TESTLINK_DEFAULT_TIME: u32 = 30 * HZ;

/* HZ is supplied by the kernel environment. */
pub const HZ: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum smc_llc_reqresp {
    SMC_LLC_REQ = 0,
    SMC_LLC_RESP = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum smc_llc_msg_type {
    SMC_LLC_CONFIRM_LINK = 0x01,
    SMC_LLC_ADD_LINK = 0x02,
    SMC_LLC_ADD_LINK_CONT = 0x03,
    SMC_LLC_DELETE_LINK = 0x04,
    SMC_LLC_REQ_ADD_LINK = 0x05,
    SMC_LLC_CONFIRM_RKEY = 0x06,
    SMC_LLC_TEST_LINK = 0x07,
    SMC_LLC_CONFIRM_RKEY_CONT = 0x08,
    SMC_LLC_DELETE_RKEY = 0x09,
    SMC_LLC_CONFIRM_LINK_V2 = 0x21,
    SMC_LLC_ADD_LINK_V2 = 0x22,
    SMC_LLC_DELETE_LINK_V2 = 0x24,
    SMC_LLC_REQ_ADD_LINK_V2 = 0x25,
    SMC_LLC_CONFIRM_RKEY_V2 = 0x26,
    SMC_LLC_TEST_LINK_V2 = 0x27,
    SMC_LLC_DELETE_RKEY_V2 = 0x29,
}

/* Equivalent of smc_link_downing(state), using the external cmpxchg helper. */
pub unsafe fn smc_link_downing(state: *mut i32) -> bool {
    cmpxchg(state, SMC_LNK_ACTIVE, SMC_LNK_INACTIVE) == SMC_LNK_ACTIVE
}

pub const SMC_LLC_DEL_LOST_PATH: u32 = 0x0001_0000;
pub const SMC_LLC_DEL_OP_INIT_TERM: u32 = 0x0002_0000;
pub const SMC_LLC_DEL_PROG_INIT_TERM: u32 = 0x0003_0000;
pub const SMC_LLC_DEL_PROT_VIOL: u32 = 0x0004_0000;
pub const SMC_LLC_DEL_NO_ASYM_NEEDED: u32 = 0x0005_0000;
pub const SMC_LLC_DEL_NOLNK: u32 = 0x0010_0000; /* Unknown Link ID (no link) */
pub const SMC_LLC_DEL_NOLGR: u32 = 0x0020_0000; /* Unknown Link Group */

#[repr(C)] pub struct smc_link { _private: [u8; 0] }
#[repr(C)] pub struct smc_link_group { _private: [u8; 0] }
#[repr(C)] pub struct smc_sock { _private: [u8; 0] }
#[repr(C)] pub struct smc_buf_desc { _private: [u8; 0] }
#[repr(C)] pub struct smc_llc_flow { _private: [u8; 0] }
#[repr(C)] pub struct smc_llc_qentry { _private: [u8; 0] }

/* These inline definitions depend on the layouts declared by smc_wr.h. */
pub unsafe fn smc_llc_usable_link(_lgr: *mut smc_link_group) -> *mut smc_link {
    /* Source loop: return the first usable link in lgr->lnk, else NULL. */
    core::ptr::null_mut()
}

pub unsafe fn smc_llc_set_termination_rsn(_lgr: *mut smc_link_group, _rsn: u32) {
    /* Source operation: assign rsn only when lgr->llc_termination_rsn is zero. */
}

extern "C" {
    pub fn cmpxchg(state: *mut i32, old: i32, new: i32) -> i32;
    static SMC_LNK_ACTIVE: i32;
    static SMC_LNK_INACTIVE: i32;

    pub fn smc_llc_send_confirm_link(lnk: *mut smc_link, reqresp: smc_llc_reqresp) -> i32;
    pub fn smc_llc_send_add_link(link: *mut smc_link, mac: *mut u8, gid: *mut u8,
                                  link_new: *mut smc_link, reqresp: smc_llc_reqresp) -> i32;
    pub fn smc_llc_send_delete_link(link: *mut smc_link, link_del_id: u8,
                                    reqresp: smc_llc_reqresp, orderly: bool, reason: u32) -> i32;
    pub fn smc_llc_srv_delete_link_local(link: *mut smc_link, del_link_id: u8);
    pub fn smc_llc_lgr_init(lgr: *mut smc_link_group, smc: *mut smc_sock);
    pub fn smc_llc_lgr_clear(lgr: *mut smc_link_group);
    pub fn smc_llc_link_init(link: *mut smc_link) -> i32;
    pub fn smc_llc_link_active(link: *mut smc_link);
    pub fn smc_llc_link_clear(link: *mut smc_link, log: bool);
    pub fn smc_llc_do_confirm_rkey(send_link: *mut smc_link, rmb_desc: *mut smc_buf_desc) -> i32;
    pub fn smc_llc_do_delete_rkey(lgr: *mut smc_link_group, rmb_desc: *mut smc_buf_desc) -> i32;
    pub fn smc_llc_flow_initiate(lgr: *mut smc_link_group, ty: i32) -> i32;
    pub fn smc_llc_flow_stop(lgr: *mut smc_link_group, flow: *mut smc_llc_flow);
    pub fn smc_llc_eval_conf_link(qentry: *mut smc_llc_qentry, ty: smc_llc_reqresp) -> i32;
    pub fn smc_llc_link_set_uid(link: *mut smc_link);
    pub fn smc_llc_save_peer_uid(qentry: *mut smc_llc_qentry);
    pub fn smc_llc_wait(lgr: *mut smc_link_group, lnk: *mut smc_link, time_out: i32,
                        exp_msg: u8) -> *mut smc_llc_qentry;
    pub fn smc_llc_flow_qentry_clr(flow: *mut smc_llc_flow) -> *mut smc_llc_qentry;
    pub fn smc_llc_flow_qentry_del(flow: *mut smc_llc_flow);
    pub fn smc_llc_send_link_delete_all(lgr: *mut smc_link_group, ord: bool, rsn: u32);
    pub fn smc_llc_cli_add_link(link: *mut smc_link, qentry: *mut smc_llc_qentry) -> i32;
    pub fn smc_llc_srv_add_link(link: *mut smc_link, req_qentry: *mut smc_llc_qentry) -> i32;
    pub fn smc_llc_add_link_local(link: *mut smc_link);
    pub fn smc_llc_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
