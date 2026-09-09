// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2008-2009 Cisco Systems, Inc.  All rights reserved.
// Copyright (c) 2007-2008 Intel Corporation.  All rights reserved.

// Dependencies are supplied by the surrounding kernel/libfc translation.

pub const FCOE_MAX_CMD_LEN: usize = 16;
pub const FCOE_MTU: usize = 2158;
pub const FCOE_CTLR_START_DELAY: u32 = 2000;
pub const FCOE_CTLR_SOL_TOV: u32 = 2000;
pub const FCOE_CTLR_FCF_LIMIT: u32 = 20;
pub const FCOE_CTLR_VN2VN_LOGIN_LIMIT: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_state {
    FIP_ST_DISABLED,
    FIP_ST_LINK_WAIT,
    FIP_ST_AUTO,
    FIP_ST_NON_FIP,
    FIP_ST_ENABLED,
    FIP_ST_VNMP_START,
    FIP_ST_VNMP_PROBE1,
    FIP_ST_VNMP_PROBE2,
    FIP_ST_VNMP_CLAIM,
    FIP_ST_VNMP_UP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_mode {
    FIP_MODE_AUTO,
    FIP_MODE_NON_FIP,
    FIP_MODE_FABRIC,
    FIP_MODE_VN2VN,
}

#[repr(C)]
pub struct fcoe_ctlr {
    pub state: fip_state,
    pub mode: fip_mode,
    pub lp: *mut fc_lport,
    pub sel_fcf: *mut fcoe_fcf,
    pub fcfs: list_head,
    pub cdev: *mut fcoe_ctlr_device,
    pub fcf_count: u16,
    pub sol_time: c_ulong,
    pub sel_time: c_ulong,
    pub port_ka_time: c_ulong,
    pub ctlr_ka_time: c_ulong,
    pub timer: timer_list,
    pub timer_work: work_struct,
    pub recv_work: work_struct,
    pub fip_recv_list: sk_buff_head,
    pub flogi_req: *mut sk_buff,
    pub rnd_state: rnd_state,
    pub port_id: u32,
    pub user_mfs: u16,
    pub flogi_oxid: u16,
    pub flogi_req_send: u8,
    pub flogi_count: u8,
    pub map_dest: bool,
    pub fip_resp: bool,
    pub spma: u8,
    pub probe_tries: u8,
    pub priority: u8,
    pub dest_addr: [u8; ETH_ALEN],
    pub ctl_src_addr: [u8; ETH_ALEN],
    pub send: Option<unsafe extern "C" fn(*mut fcoe_ctlr, *mut sk_buff)>,
    pub update_mac: Option<unsafe extern "C" fn(*mut fc_lport, *mut u8)>,
    pub get_src_addr: Option<unsafe extern "C" fn(*mut fc_lport) -> *mut u8>,
    pub ctlr_mutex: mutex,
    pub ctlr_lock: spinlock_t,
}

#[inline]
pub unsafe fn fcoe_ctlr_priv(ctlr: *const fcoe_ctlr) -> *mut core::ffi::c_void {
    (ctlr.add(1)) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn fcoe_ctlr_to_ctlr_dev(x: *mut fcoe_ctlr) -> *mut fcoe_ctlr_device { (*x).cdev }

#[repr(C)]
pub struct fcoe_fcf {
    pub list: list_head,
    pub event_work: work_struct,
    pub fip: *mut fcoe_ctlr,
    pub fcf_dev: *mut fcoe_fcf_device,
    pub time: c_ulong,
    pub switch_name: u64,
    pub fabric_name: u64,
    pub fc_map: u32,
    pub vfid: u16,
    pub fcf_mac: [u8; ETH_ALEN],
    pub fcoe_mac: [u8; ETH_ALEN],
    pub pri: u8,
    pub flogi_sent: u8,
    pub flags: u16,
    pub fka_period: u32,
    pub fd_flags: u8,
}

#[inline]
pub unsafe fn fcoe_fcf_to_fcf_dev(x: *mut fcoe_fcf) -> *mut fcoe_fcf_device { (*x).fcf_dev }

#[repr(C)]
pub struct fcoe_rport {
    pub rdata: fc_rport_priv,
    pub time: c_ulong,
    pub fcoe_len: u16,
    pub flags: u16,
    pub login_count: u8,
    pub enode_mac: [u8; ETH_ALEN],
    pub vn_mac: [u8; ETH_ALEN],
}

extern "C" {
    pub fn fcoe_ctlr_init(ctlr: *mut fcoe_ctlr, mode: fip_mode);
    pub fn fcoe_ctlr_destroy(ctlr: *mut fcoe_ctlr);
    pub fn fcoe_ctlr_link_up(ctlr: *mut fcoe_ctlr);
    pub fn fcoe_ctlr_link_down(ctlr: *mut fcoe_ctlr) -> i32;
    pub fn fcoe_ctlr_els_send(ctlr: *mut fcoe_ctlr, lp: *mut fc_lport, skb: *mut sk_buff) -> i32;
    pub fn fcoe_ctlr_recv(ctlr: *mut fcoe_ctlr, skb: *mut sk_buff);
    pub fn fcoe_ctlr_recv_flogi(ctlr: *mut fcoe_ctlr, lp: *mut fc_lport, frame: *mut fc_frame) -> i32;
    pub fn fcoe_wwn_from_mac(mac: *mut u8, scheme: c_uint, port: c_uint) -> u64;
    pub fn fcoe_libfc_config(lp: *mut fc_lport, ctlr: *mut fcoe_ctlr, tmpl: *const libfc_function_template, init_fcp: i32) -> i32;
    pub fn fcoe_fc_crc(fp: *mut fc_frame) -> u32;
    pub fn fcoe_start_io(skb: *mut sk_buff) -> i32;
    pub fn fcoe_get_wwn(netdev: *mut net_device, wwn: *mut u64, kind: i32) -> i32;
    pub fn __fcoe_get_lesb(lp: *mut fc_lport, lesb: *mut fc_els_lesb, netdev: *mut net_device);
    pub fn fcoe_wwn_to_str(wwn: u64, buf: *mut c_char, len: i32);
    pub fn fcoe_validate_vport_create(vport: *mut fc_vport) -> i32;
    pub fn fcoe_link_speed_update(lp: *mut fc_lport) -> i32;
    pub fn fcoe_get_lesb(lp: *mut fc_lport, lesb: *mut fc_els_lesb);
    pub fn fcoe_ctlr_get_lesb(ctlr_dev: *mut fcoe_ctlr_device);
}

#[inline]
pub unsafe fn is_fip_mode(fip: *mut fcoe_ctlr) -> bool { (*fip).state == fip_state::FIP_ST_ENABLED }

pub const FCOE_TRANSPORT_DEFAULT: &str = "fcoe";

#[repr(C)]
pub struct fcoe_transport {
    pub name: [c_char; IFNAMSIZ],
    pub attached: bool,
    pub list: list_head,
    pub r#match: Option<unsafe extern "C" fn(*mut net_device) -> bool>,
    pub alloc: Option<unsafe extern "C" fn(*mut net_device) -> i32>,
    pub create: Option<unsafe extern "C" fn(*mut net_device, fip_mode) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*mut net_device) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut net_device) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut net_device) -> i32>,
}

#[repr(C)]
pub struct fcoe_percpu_s {
    pub kthread: *mut task_struct,
    pub work: work_struct,
    pub fcoe_rx_list: sk_buff_head,
    pub crc_eof_page: *mut page,
    pub crc_eof_offset: i32,
    pub lock: local_lock_t,
}

#[repr(C)]
pub struct fcoe_port {
    pub priv_: *mut core::ffi::c_void,
    pub lport: *mut fc_lport,
    pub fcoe_pending_queue: sk_buff_head,
    pub fcoe_pending_queue_active: u8,
    pub max_queue_depth: u32,
    pub min_queue_depth: u32,
    pub timer: timer_list,
    pub destroy_work: work_struct,
    pub data_src_addr: [u8; ETH_ALEN],
    pub get_netdev: Option<unsafe extern "C" fn(*const fc_lport) -> *mut net_device>,
}

#[inline]
pub unsafe fn fcoe_get_netdev(lport: *const fc_lport) -> *mut net_device {
    let port = lport_priv(lport) as *mut fcoe_port;
    match (*port).get_netdev { Some(f) => f(lport), None => core::ptr::null_mut() }
}

#[repr(C)]
pub struct fcoe_netdev_mapping {
    pub list: list_head,
    pub netdev: *mut net_device,
    pub ft: *mut fcoe_transport,
}

extern "C" {
    pub fn fcoe_clean_pending_queue(lp: *mut fc_lport);
    pub fn fcoe_check_wait_queue(lp: *mut fc_lport, skb: *mut sk_buff);
    pub fn fcoe_queue_timer(t: *mut timer_list);
    pub fn fcoe_get_paged_crc_eof(skb: *mut sk_buff, tlen: i32, fps: *mut fcoe_percpu_s) -> i32;
    pub fn fcoe_fcf_get_selected(fcf: *mut fcoe_fcf_device);
    pub fn fcoe_ctlr_set_fip_mode(ctlr_dev: *mut fcoe_ctlr_device);
    pub fn fcoe_transport_attach(ft: *mut fcoe_transport) -> i32;
    pub fn fcoe_transport_detach(ft: *mut fcoe_transport) -> i32;
    pub fn fcoe_ctlr_create_store(buf: *const c_char, count: usize) -> isize;
    pub fn fcoe_ctlr_destroy_store(buf: *const c_char, count: usize) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
