/* SPDX-License-Identifier: GPL-2.0 */
/* Definitions related to Power Management Quality of Service (PM QoS). */

// External types and constants are supplied by the corresponding Linux bindings.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_qos_flags_status {
    PM_QOS_FLAGS_UNDEFINED = -1,
    PM_QOS_FLAGS_NONE,
    PM_QOS_FLAGS_SOME,
    PM_QOS_FLAGS_ALL,
}

pub const PM_QOS_DEFAULT_VALUE: i32 = -1;
pub const PM_QOS_LATENCY_ANY: i32 = S32_MAX;
pub const PM_QOS_LATENCY_ANY_NS: i64 = (PM_QOS_LATENCY_ANY as i64) * NSEC_PER_USEC;
pub const PM_QOS_CPU_LATENCY_DEFAULT_VALUE: i32 = 2000 * USEC_PER_SEC;
pub const PM_QOS_RESUME_LATENCY_DEFAULT_VALUE: i32 = PM_QOS_LATENCY_ANY;
pub const PM_QOS_RESUME_LATENCY_NO_CONSTRAINT: i32 = PM_QOS_LATENCY_ANY;
pub const PM_QOS_RESUME_LATENCY_NO_CONSTRAINT_NS: i64 = PM_QOS_LATENCY_ANY_NS;
pub const PM_QOS_LATENCY_TOLERANCE_DEFAULT_VALUE: i32 = 0;
pub const PM_QOS_MIN_FREQUENCY_DEFAULT_VALUE: i32 = 0;
pub const PM_QOS_MAX_FREQUENCY_DEFAULT_VALUE: i32 = FREQ_QOS_MAX_DEFAULT_VALUE;
pub const PM_QOS_LATENCY_TOLERANCE_NO_CONSTRAINT: i32 = -1;
pub const PM_QOS_FLAG_NO_POWER_OFF: i32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_qos_type { PM_QOS_UNITIALIZED, PM_QOS_MAX, PM_QOS_MIN }

#[repr(C)]
pub struct pm_qos_constraints {
    pub list: plist_head,
    pub target_value: i32,
    pub default_value: i32,
    pub no_constraint_value: i32,
    pub r#type: pm_qos_type,
    pub notifiers: *mut blocking_notifier_head,
}

#[repr(C)]
pub struct pm_qos_request { pub node: plist_node, pub qos: *mut pm_qos_constraints }

#[repr(C)]
pub struct pm_qos_flags_request { pub node: list_head, pub flags: i32 }

#[repr(C)]
pub struct pm_qos_flags { pub list: list_head, pub effective_flags: i32 }

pub const FREQ_QOS_MIN_DEFAULT_VALUE: i32 = 0;
pub const FREQ_QOS_MAX_DEFAULT_VALUE: i32 = S32_MAX;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum freq_qos_req_type { FREQ_QOS_MIN = 1, FREQ_QOS_MAX }

#[repr(C)]
pub struct freq_constraints {
    pub min_freq: pm_qos_constraints,
    pub min_freq_notifiers: blocking_notifier_head,
    pub max_freq: pm_qos_constraints,
    pub max_freq_notifiers: blocking_notifier_head,
}

#[repr(C)]
pub struct freq_qos_request {
    pub r#type: freq_qos_req_type,
    pub pnode: plist_node,
    pub qos: *mut freq_constraints,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_pm_qos_req_type {
    DEV_PM_QOS_RESUME_LATENCY = 1,
    DEV_PM_QOS_LATENCY_TOLERANCE,
    DEV_PM_QOS_MIN_FREQUENCY,
    DEV_PM_QOS_MAX_FREQUENCY,
    DEV_PM_QOS_FLAGS,
}

#[repr(C)]
pub union dev_pm_qos_request_data {
    pub pnode: plist_node,
    pub flr: pm_qos_flags_request,
    pub freq: freq_qos_request,
}

#[repr(C)]
pub struct dev_pm_qos_request {
    pub r#type: dev_pm_qos_req_type,
    pub data: dev_pm_qos_request_data,
    pub dev: *mut device,
}

#[repr(C)]
pub struct dev_pm_qos {
    pub resume_latency: pm_qos_constraints,
    pub latency_tolerance: pm_qos_constraints,
    pub freq: freq_constraints,
    pub flags: pm_qos_flags,
    pub resume_latency_req: *mut dev_pm_qos_request,
    pub latency_tolerance_req: *mut dev_pm_qos_request,
    pub flags_req: *mut dev_pm_qos_request,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_qos_req_action { PM_QOS_ADD_REQ, PM_QOS_UPDATE_REQ, PM_QOS_REMOVE_REQ }

#[inline]
pub unsafe fn dev_pm_qos_request_active(req: *mut dev_pm_qos_request) -> bool { !(*req).dev.is_null() }

extern "C" {
    pub fn pm_qos_read_value(c: *mut pm_qos_constraints) -> i32;
    pub fn pm_qos_update_target(c: *mut pm_qos_constraints, node: *mut plist_node, action: pm_qos_req_action, value: i32) -> i32;
    pub fn pm_qos_update_flags(pqf: *mut pm_qos_flags, req: *mut pm_qos_flags_request, action: pm_qos_req_action, val: i32) -> bool;
}

// CONFIG_CPU_IDLE: declarations are external; otherwise use these inline fallbacks.
#[inline] pub unsafe fn cpu_latency_qos_limit() -> i32 { INT_MAX }
#[inline] pub unsafe fn cpu_latency_qos_request_active(_req: *mut pm_qos_request) -> bool { false }
#[inline] pub unsafe fn cpu_latency_qos_add_request(_req: *mut pm_qos_request, _value: i32) {}
#[inline] pub unsafe fn cpu_latency_qos_update_request(_req: *mut pm_qos_request, _new_value: i32) {}
#[inline] pub unsafe fn cpu_latency_qos_remove_request(_req: *mut pm_qos_request) {}

// CONFIG_PM_QOS_CPU_SYSTEM_WAKEUP fallback.
#[inline] pub unsafe fn cpu_wakeup_latency_qos_limit() -> i32 { PM_QOS_RESUME_LATENCY_NO_CONSTRAINT }

// CONFIG_PM fallbacks.
#[inline] pub unsafe fn __dev_pm_qos_flags(_dev: *mut device, _mask: i32) -> pm_qos_flags_status { pm_qos_flags_status::PM_QOS_FLAGS_UNDEFINED }
#[inline] pub unsafe fn dev_pm_qos_flags(_dev: *mut device, _mask: i32) -> pm_qos_flags_status { pm_qos_flags_status::PM_QOS_FLAGS_UNDEFINED }
#[inline] pub unsafe fn __dev_pm_qos_resume_latency(_dev: *mut device) -> i32 { PM_QOS_RESUME_LATENCY_NO_CONSTRAINT }
#[inline] pub unsafe fn dev_pm_qos_read_value(_dev: *mut device, req_type: dev_pm_qos_req_type) -> i32 {
    match req_type {
        dev_pm_qos_req_type::DEV_PM_QOS_RESUME_LATENCY => PM_QOS_RESUME_LATENCY_NO_CONSTRAINT,
        dev_pm_qos_req_type::DEV_PM_QOS_MIN_FREQUENCY => PM_QOS_MIN_FREQUENCY_DEFAULT_VALUE,
        dev_pm_qos_req_type::DEV_PM_QOS_MAX_FREQUENCY => PM_QOS_MAX_FREQUENCY_DEFAULT_VALUE,
        _ => 0,
    }
}
#[inline] pub unsafe fn dev_pm_qos_add_request(_dev: *mut device, _req: *mut dev_pm_qos_request, _ty: dev_pm_qos_req_type, _value: i32) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_update_request(_req: *mut dev_pm_qos_request, _new_value: i32) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_remove_request(_req: *mut dev_pm_qos_request) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_add_notifier(_dev: *mut device, _notifier: *mut notifier_block, _ty: dev_pm_qos_req_type) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_remove_notifier(_dev: *mut device, _notifier: *mut notifier_block, _ty: dev_pm_qos_req_type) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_constraints_init(_dev: *mut device) {}
#[inline] pub unsafe fn dev_pm_qos_constraints_destroy(_dev: *mut device) {}
#[inline] pub unsafe fn dev_pm_qos_add_ancestor_request(_dev: *mut device, _req: *mut dev_pm_qos_request, _ty: dev_pm_qos_req_type, _value: i32) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_expose_latency_limit(_dev: *mut device, _value: i32) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_hide_latency_limit(_dev: *mut device) {}
#[inline] pub unsafe fn dev_pm_qos_expose_flags(_dev: *mut device, _value: i32) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_hide_flags(_dev: *mut device) {}
#[inline] pub unsafe fn dev_pm_qos_update_flags(_dev: *mut device, _mask: i32, _set: bool) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_get_user_latency_tolerance(_dev: *mut device) -> i32 { PM_QOS_LATENCY_TOLERANCE_NO_CONSTRAINT }
#[inline] pub unsafe fn dev_pm_qos_update_user_latency_tolerance(_dev: *mut device, _val: i32) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_expose_latency_tolerance(_dev: *mut device) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_hide_latency_tolerance(_dev: *mut device) {}
#[inline] pub unsafe fn dev_pm_qos_requested_resume_latency(_dev: *mut device) -> i32 { PM_QOS_RESUME_LATENCY_NO_CONSTRAINT }
#[inline] pub unsafe fn dev_pm_qos_requested_flags(_dev: *mut device) -> i32 { 0 }
#[inline] pub unsafe fn dev_pm_qos_raw_resume_latency(_dev: *mut device) -> i32 { PM_QOS_RESUME_LATENCY_NO_CONSTRAINT }

#[inline] pub unsafe fn freq_qos_request_active(req: *mut freq_qos_request) -> bool { !(*req).qos.is_null() }

extern "C" {
    pub fn freq_constraints_init(qos: *mut freq_constraints);
    pub fn freq_qos_read_value(qos: *mut freq_constraints, ty: freq_qos_req_type) -> i32;
    pub fn freq_qos_add_request(qos: *mut freq_constraints, req: *mut freq_qos_request, ty: freq_qos_req_type, value: i32) -> i32;
    pub fn freq_qos_update_request(req: *mut freq_qos_request, new_value: i32) -> i32;
    pub fn freq_qos_remove_request(req: *mut freq_qos_request) -> i32;
    pub fn freq_qos_apply(req: *mut freq_qos_request, action: pm_qos_req_action, value: i32) -> i32;
    pub fn freq_qos_add_notifier(qos: *mut freq_constraints, ty: freq_qos_req_type, notifier: *mut notifier_block) -> i32;
    pub fn freq_qos_remove_notifier(qos: *mut freq_constraints, ty: freq_qos_req_type, notifier: *mut notifier_block) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
