// SPDX-License-Identifier: GPL-2.0-only
/*
 * Power Management Quality of Service (PM QoS) support base.
 *
 * Copyright (C) 2020 Intel Corporation
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut pm_qos_lock: c_void;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: usize);
    fn plist_head_empty(list: *const c_void) -> bool;
    fn plist_first(list: *const c_void) -> *mut plist_node;
    fn plist_last(list: *const c_void) -> *mut plist_node;
    fn plist_del(node: *mut plist_node, list: *mut c_void);
    fn plist_node_init(node: *mut plist_node, prio: c_int);
    fn plist_add(node: *mut plist_node, list: *mut c_void);
    fn list_empty(list: *const c_void) -> bool;
    fn list_del(node: *mut c_void);
    fn list_add_tail(node: *mut c_void, list: *mut c_void);
    fn blocking_notifier_call_chain(n: *mut c_void, value: c_int, data: *mut c_void);
    fn trace_pm_qos_update_target(action: pm_qos_req_action, prev: c_int, curr: c_int);
    fn trace_pm_qos_update_flags(action: pm_qos_req_action, prev: i32, curr: i32);
    fn wake_up_all_idle_cpus();
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

#[repr(C)]
pub struct plist_node { pub prio: c_int, pub node: [u8; 0] }
#[repr(C)]
pub struct pm_qos_constraints {
    pub list: c_void,
    pub target_value: i32,
    pub default_value: i32,
    pub no_constraint_value: i32,
    pub type_: c_int,
    pub notifiers: *mut c_void,
}
#[repr(C)]
pub struct pm_qos_request { pub qos: *mut pm_qos_constraints, pub node: plist_node }
#[repr(C)]
pub struct pm_qos_flags_request { pub node: c_void, pub flags: i32 }
#[repr(C)]
pub struct pm_qos_flags { pub list: c_void, pub effective_flags: i32 }
#[repr(C)]
pub struct freq_constraints { pub min_freq: pm_qos_constraints, pub max_freq: pm_qos_constraints, pub min_freq_notifiers: c_void, pub max_freq_notifiers: c_void }
#[repr(C)]
pub struct freq_qos_request { pub qos: *mut freq_constraints, pub pnode: plist_node, pub type_: c_int }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }

pub type pm_qos_req_action = c_int;
pub type freq_qos_req_type = c_int;
pub const PM_QOS_MIN: c_int = 1;
pub const PM_QOS_MAX: c_int = 2;
pub const PM_QOS_ADD_REQ: pm_qos_req_action = 0;
pub const PM_QOS_UPDATE_REQ: pm_qos_req_action = 1;
pub const PM_QOS_REMOVE_REQ: pm_qos_req_action = 2;
pub const PM_QOS_DEFAULT_VALUE: i32 = -1;
pub const FREQ_QOS_MIN: freq_qos_req_type = 0;
pub const FREQ_QOS_MAX: freq_qos_req_type = 1;
pub const FREQ_QOS_MIN_DEFAULT_VALUE: i32 = 0;
pub const FREQ_QOS_MAX_DEFAULT_VALUE: i32 = i32::MAX;
pub const PM_QOS_CPU_LATENCY_DEFAULT_VALUE: i32 = 2_147_483_647;
pub const PM_QOS_RESUME_LATENCY_NO_CONSTRAINT: i32 = 2_147_483_647;

static mut cpu_latency_constraints: pm_qos_constraints = pm_qos_constraints { list: unsafe { core::mem::zeroed() }, target_value: PM_QOS_CPU_LATENCY_DEFAULT_VALUE, default_value: PM_QOS_CPU_LATENCY_DEFAULT_VALUE, no_constraint_value: PM_QOS_CPU_LATENCY_DEFAULT_VALUE, type_: PM_QOS_MIN, notifiers: core::ptr::null_mut() };

pub unsafe fn pm_qos_read_value(c: *const pm_qos_constraints) -> i32 { core::ptr::read_volatile(&(*c).target_value) }

unsafe fn pm_qos_get_value(c: *mut pm_qos_constraints) -> c_int {
    if plist_head_empty(&mut (*c).list as *mut _ as *const c_void) { return (*c).no_constraint_value; }
    match (*c).type_ {
        PM_QOS_MIN => (*plist_first(&mut (*c).list as *mut _ as *const c_void)).prio,
        PM_QOS_MAX => (*plist_last(&mut (*c).list as *mut _ as *const c_void)).prio,
        _ => { 0 }
    }
}
unsafe fn pm_qos_set_value(c: *mut pm_qos_constraints, value: i32) { core::ptr::write_volatile(&mut (*c).target_value, value); }

pub unsafe fn pm_qos_update_target(c: *mut pm_qos_constraints, node: *mut plist_node, action: pm_qos_req_action, value: c_int) -> c_int {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut pm_qos_lock, &mut flags);
    let prev_value = pm_qos_get_value(c);
    let new_value = if value == PM_QOS_DEFAULT_VALUE { (*c).default_value } else { value };
    match action {
        PM_QOS_REMOVE_REQ => plist_del(node, &mut (*c).list as *mut _ as *mut c_void),
        PM_QOS_UPDATE_REQ => { plist_del(node, &mut (*c).list as *mut _ as *mut c_void); plist_node_init(node, new_value); plist_add(node, &mut (*c).list as *mut _ as *mut c_void); },
        PM_QOS_ADD_REQ => { plist_node_init(node, new_value); plist_add(node, &mut (*c).list as *mut _ as *mut c_void); },
        _ => {}
    }
    let curr_value = pm_qos_get_value(c); pm_qos_set_value(c, curr_value); spin_unlock_irqrestore(&mut pm_qos_lock, flags);
    trace_pm_qos_update_target(action, prev_value, curr_value);
    if prev_value == curr_value { return 0; }
    if !(*c).notifiers.is_null() { blocking_notifier_call_chain((*c).notifiers, curr_value, core::ptr::null_mut()); }
    1
}

unsafe fn pm_qos_flags_remove_req(pqf: *mut pm_qos_flags, req: *mut pm_qos_flags_request) {
    list_del(&mut (*req).node); let mut val = 0i32; (*pqf).effective_flags = val; let _ = &mut val;
}
pub unsafe fn pm_qos_update_flags(pqf: *mut pm_qos_flags, req: *mut pm_qos_flags_request, action: pm_qos_req_action, val: i32) -> bool {
    let mut flags = 0usize; spin_lock_irqsave(&mut pm_qos_lock, &mut flags);
    let prev = if list_empty(&(*pqf).list) { 0 } else { (*pqf).effective_flags };
    match action { PM_QOS_REMOVE_REQ => pm_qos_flags_remove_req(pqf, req), PM_QOS_UPDATE_REQ => { pm_qos_flags_remove_req(pqf, req); (*req).flags = val; list_add_tail(&mut (*req).node, &mut (*pqf).list); (*pqf).effective_flags |= val; }, PM_QOS_ADD_REQ => { (*req).flags = val; list_add_tail(&mut (*req).node, &mut (*pqf).list); (*pqf).effective_flags |= val; }, _ => {} }
    let curr = if list_empty(&(*pqf).list) { 0 } else { (*pqf).effective_flags }; spin_unlock_irqrestore(&mut pm_qos_lock, flags); trace_pm_qos_update_flags(action, prev, curr); prev != curr
}

#[cfg(feature = "CONFIG_CPU_IDLE")]
pub unsafe fn cpu_latency_qos_limit() -> i32 { pm_qos_read_value(&cpu_latency_constraints) }

#[cfg(feature = "CONFIG_CPU_IDLE")]
pub unsafe fn cpu_latency_qos_request_active(req: *mut pm_qos_request) -> bool { (*req).qos == &mut cpu_latency_constraints }

#[cfg(feature = "CONFIG_CPU_IDLE")]
pub unsafe fn cpu_latency_qos_apply(req: *mut pm_qos_request, action: pm_qos_req_action, value: i32) { if pm_qos_update_target((*req).qos, &mut (*req).node, action, value) > 0 { wake_up_all_idle_cpus(); } }

#[inline] pub fn freq_qos_value_invalid(value: i32) -> bool { value < 0 && value != PM_QOS_DEFAULT_VALUE }

pub unsafe fn freq_constraints_init(qos: *mut freq_constraints) { (*qos).min_freq.list = core::mem::zeroed(); (*qos).min_freq.target_value = FREQ_QOS_MIN_DEFAULT_VALUE; (*qos).min_freq.default_value = FREQ_QOS_MIN_DEFAULT_VALUE; (*qos).min_freq.no_constraint_value = FREQ_QOS_MIN_DEFAULT_VALUE; (*qos).min_freq.type_ = PM_QOS_MAX; (*qos).max_freq.list = core::mem::zeroed(); (*qos).max_freq.target_value = FREQ_QOS_MAX_DEFAULT_VALUE; (*qos).max_freq.default_value = FREQ_QOS_MAX_DEFAULT_VALUE; (*qos).max_freq.no_constraint_value = FREQ_QOS_MAX_DEFAULT_VALUE; (*qos).max_freq.type_ = PM_QOS_MIN; }
pub unsafe fn freq_qos_read_value(qos: *mut freq_constraints, type_: freq_qos_req_type) -> i32 { if qos.is_null() { return if type_ == FREQ_QOS_MIN { FREQ_QOS_MIN_DEFAULT_VALUE } else { FREQ_QOS_MAX_DEFAULT_VALUE }; } if type_ == FREQ_QOS_MIN { pm_qos_read_value(&(*qos).min_freq) } else if type_ == FREQ_QOS_MAX { pm_qos_read_value(&(*qos).max_freq) } else { 0 } }
pub unsafe fn freq_qos_apply(req: *mut freq_qos_request, action: pm_qos_req_action, value: i32) -> c_int { match (*req).type_ { FREQ_QOS_MIN => pm_qos_update_target(&mut (*(*req).qos).min_freq, &mut (*req).pnode, action, value), FREQ_QOS_MAX => pm_qos_update_target(&mut (*(*req).qos).max_freq, &mut (*req).pnode, action, value), _ => -22 } }
pub unsafe fn freq_qos_add_request(qos: *mut freq_constraints, req: *mut freq_qos_request, type_: freq_qos_req_type, value: i32) -> c_int { if qos.is_null() || req.is_null() || freq_qos_value_invalid(value) { return -22; } (*req).qos = qos; (*req).type_ = type_; let ret = freq_qos_apply(req, PM_QOS_ADD_REQ, value); if ret < 0 { (*req).qos = core::ptr::null_mut(); (*req).type_ = 0; } ret }
pub unsafe fn freq_qos_update_request(req: *mut freq_qos_request, value: i32) -> c_int { if req.is_null() || freq_qos_value_invalid(value) { return -22; } if (*req).pnode.prio == value { return 0; } freq_qos_apply(req, PM_QOS_UPDATE_REQ, value) }
pub unsafe fn freq_qos_remove_request(req: *mut freq_qos_request) -> c_int { if req.is_null() { return -22; } let ret = freq_qos_apply(req, PM_QOS_REMOVE_REQ, PM_QOS_DEFAULT_VALUE); (*req).qos = core::ptr::null_mut(); (*req).type_ = 0; ret }

#[cfg(feature = "CONFIG_CPU_IDLE")]
pub unsafe fn cpu_latency_qos_add_request(req: *mut pm_qos_request, value: i32) { if req.is_null() || (value < 0 && value != PM_QOS_DEFAULT_VALUE) { return; } if cpu_latency_qos_request_active(req) { return; } (*req).qos = &mut cpu_latency_constraints; cpu_latency_qos_apply(req, PM_QOS_ADD_REQ, value); }
#[cfg(feature = "CONFIG_CPU_IDLE")]
pub unsafe fn cpu_latency_qos_update_request(req: *mut pm_qos_request, value: i32) { if req.is_null() || (value < 0 && value != PM_QOS_DEFAULT_VALUE) || !cpu_latency_qos_request_active(req) { return; } if (*req).node.prio != value { cpu_latency_qos_apply(req, PM_QOS_UPDATE_REQ, value); } }
#[cfg(feature = "CONFIG_CPU_IDLE")]
pub unsafe fn cpu_latency_qos_remove_request(req: *mut pm_qos_request) { if req.is_null() || !cpu_latency_qos_request_active(req) { return; } cpu_latency_qos_apply(req, PM_QOS_REMOVE_REQ, PM_QOS_DEFAULT_VALUE); memset(req as *mut c_void, 0, core::mem::size_of::<pm_qos_request>()); }

pub unsafe fn freq_qos_add_notifier(qos: *mut freq_constraints, type_: freq_qos_req_type, notifier: *mut notifier_block) -> c_int { if qos.is_null() || notifier.is_null() { return -22; } match type_ { FREQ_QOS_MIN | FREQ_QOS_MAX => 0, _ => -22 } }
pub unsafe fn freq_qos_remove_notifier(qos: *mut freq_constraints, type_: freq_qos_req_type, notifier: *mut notifier_block) -> c_int { if qos.is_null() || notifier.is_null() { return -22; } match type_ { FREQ_QOS_MIN | FREQ_QOS_MAX => 0, _ => -22 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
