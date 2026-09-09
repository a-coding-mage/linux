// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2007,2008 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by other translated Linux headers.

pub const PROTO_UNDEF: i32 = 0;
pub const PROTO_HOST: i32 = 1;
pub const PROTO_GADGET: i32 = 2;

pub const OTG_STS_SELECTOR: i32 = 0xF000;
pub const HOST_REQUEST_FLAG: i32 = 1;
pub const T_HOST_REQ_POLL: i32 = 1500;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum otg_fsm_timer {
    A_WAIT_VRISE,
    A_WAIT_VFALL,
    A_WAIT_BCON,
    A_AIDL_BDIS,
    B_ASE0_BRST,
    A_BIDL_ADIS,
    B_AIDL_BDIS,
    B_SE0_SRP,
    B_SRP_FAIL,
    A_WAIT_ENUM,
    B_DATA_PLS,
    B_SSEND_SRP,
    NUM_OTG_FSM_TIMERS,
}

#[repr(C)]
pub struct otg_fsm {
    pub id: i32,
    pub adp_change: i32,
    pub power_up: i32,
    pub a_srp_det: i32,
    pub a_vbus_vld: i32,
    pub b_conn: i32,
    pub a_bus_resume: i32,
    pub a_bus_suspend: i32,
    pub a_conn: i32,
    pub b_se0_srp: i32,
    pub b_ssend_srp: i32,
    pub b_sess_vld: i32,
    pub test_device: i32,
    pub a_bus_drop: i32,
    pub a_bus_req: i32,
    pub b_bus_req: i32,
    pub a_sess_vld: i32,
    pub b_bus_resume: i32,
    pub b_bus_suspend: i32,
    pub drv_vbus: i32,
    pub loc_conn: i32,
    pub loc_sof: i32,
    pub adp_prb: i32,
    pub adp_sns: i32,
    pub data_pulse: i32,
    pub a_set_b_hnp_en: i32,
    pub b_srp_done: i32,
    pub b_hnp_enable: i32,
    pub a_clr_err: i32,
    pub a_bus_drop_inf: i32,
    pub a_bus_req_inf: i32,
    pub a_clr_err_inf: i32,
    pub b_bus_req_inf: i32,
    pub a_suspend_req_inf: i32,
    pub a_wait_vrise_tmout: i32,
    pub a_wait_vfall_tmout: i32,
    pub a_wait_bcon_tmout: i32,
    pub a_aidl_bdis_tmout: i32,
    pub b_ase0_brst_tmout: i32,
    pub a_bidl_adis_tmout: i32,
    pub ops: *mut otg_fsm_ops,
    pub otg: *mut usb_otg,
    pub protocol: i32,
    pub lock: mutex,
    pub host_req_flag: *mut u8,
    pub hnp_polling_work: delayed_work,
    pub hnp_work_inited: bool,
    pub state_changed: bool,
}

#[repr(C)]
pub struct otg_fsm_ops {
    pub chrg_vbus: Option<unsafe extern "C" fn(*mut otg_fsm, i32)>,
    pub drv_vbus: Option<unsafe extern "C" fn(*mut otg_fsm, i32)>,
    pub loc_conn: Option<unsafe extern "C" fn(*mut otg_fsm, i32)>,
    pub loc_sof: Option<unsafe extern "C" fn(*mut otg_fsm, i32)>,
    pub start_pulse: Option<unsafe extern "C" fn(*mut otg_fsm)>,
    pub start_adp_prb: Option<unsafe extern "C" fn(*mut otg_fsm)>,
    pub start_adp_sns: Option<unsafe extern "C" fn(*mut otg_fsm)>,
    pub add_timer: Option<unsafe extern "C" fn(*mut otg_fsm, otg_fsm_timer)>,
    pub del_timer: Option<unsafe extern "C" fn(*mut otg_fsm, otg_fsm_timer)>,
    pub start_host: Option<unsafe extern "C" fn(*mut otg_fsm, i32) -> i32>,
    pub start_gadget: Option<unsafe extern "C" fn(*mut otg_fsm, i32) -> i32>,
}

extern "C" {
    pub fn otg_statemachine(fsm: *mut otg_fsm) -> i32;
}

#[inline]
pub unsafe fn otg_chrg_vbus(fsm: *mut otg_fsm, on: i32) -> i32 {
    let ops = (*fsm).ops;
    match (*ops).chrg_vbus {
        None => -EOPNOTSUPP,
        Some(callback) => { callback(fsm, on); 0 }
    }
}

#[inline]
pub unsafe fn otg_drv_vbus(fsm: *mut otg_fsm, on: i32) -> i32 {
    let ops = (*fsm).ops;
    match (*ops).drv_vbus {
        None => -EOPNOTSUPP,
        Some(callback) => { if (*fsm).drv_vbus != on { (*fsm).drv_vbus = on; callback(fsm, on); } 0 }
    }
}

#[inline]
pub unsafe fn otg_loc_conn(fsm: *mut otg_fsm, on: i32) -> i32 {
    let ops = (*fsm).ops;
    match (*ops).loc_conn {
        None => -EOPNOTSUPP,
        Some(callback) => { if (*fsm).loc_conn != on { (*fsm).loc_conn = on; callback(fsm, on); } 0 }
    }
}

#[inline]
pub unsafe fn otg_loc_sof(fsm: *mut otg_fsm, on: i32) -> i32 {
    let ops = (*fsm).ops;
    match (*ops).loc_sof {
        None => -EOPNOTSUPP,
        Some(callback) => { if (*fsm).loc_sof != on { (*fsm).loc_sof = on; callback(fsm, on); } 0 }
    }
}

#[inline]
pub unsafe fn otg_start_pulse(fsm: *mut otg_fsm) -> i32 {
    match (*(*fsm).ops).start_pulse {
        None => -EOPNOTSUPP,
        Some(callback) => { if (*fsm).data_pulse == 0 { (*fsm).data_pulse = 1; callback(fsm); } 0 }
    }
}

#[inline]
pub unsafe fn otg_start_adp_prb(fsm: *mut otg_fsm) -> i32 {
    match (*(*fsm).ops).start_adp_prb {
        None => -EOPNOTSUPP,
        Some(callback) => { if (*fsm).adp_prb == 0 { (*fsm).adp_sns = 0; (*fsm).adp_prb = 1; callback(fsm); } 0 }
    }
}

#[inline]
pub unsafe fn otg_start_adp_sns(fsm: *mut otg_fsm) -> i32 {
    match (*(*fsm).ops).start_adp_sns {
        None => -EOPNOTSUPP,
        Some(callback) => { if (*fsm).adp_sns == 0 { (*fsm).adp_sns = 1; callback(fsm); } 0 }
    }
}

#[inline]
pub unsafe fn otg_add_timer(fsm: *mut otg_fsm, timer: otg_fsm_timer) -> i32 {
    match (*(*fsm).ops).add_timer {
        None => -EOPNOTSUPP,
        Some(callback) => { callback(fsm, timer); 0 }
    }
}

#[inline]
pub unsafe fn otg_del_timer(fsm: *mut otg_fsm, timer: otg_fsm_timer) -> i32 {
    match (*(*fsm).ops).del_timer {
        None => -EOPNOTSUPP,
        Some(callback) => { callback(fsm, timer); 0 }
    }
}

#[inline]
pub unsafe fn otg_start_host(fsm: *mut otg_fsm, on: i32) -> i32 {
    match (*(*fsm).ops).start_host {
        None => -EOPNOTSUPP,
        Some(callback) => callback(fsm, on),
    }
}

#[inline]
pub unsafe fn otg_start_gadget(fsm: *mut otg_fsm, on: i32) -> i32 {
    match (*(*fsm).ops).start_gadget {
        None => -EOPNOTSUPP,
        Some(callback) => callback(fsm, on),
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
