/* SPDX-License-Identifier: GPL-2.0 */
// Translated from trace/events/power.h. Linux tracepoint-generation includes
// and macros are represented by the corresponding Rust payload declarations.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const PWR_EVENT_EXIT: i32 = -1;

#[repr(C)]
pub struct cpu_entry { pub state: u32, pub cpu_id: u32 }
pub unsafe fn cpu_idle(_state: u32, _cpu_id: u32) {}

#[repr(C)]
pub struct cpu_idle_miss_entry { pub cpu_id: u32, pub state: u32, pub below: bool }
pub unsafe fn cpu_idle_miss(_cpu_id: u32, _state: u32, _below: bool) {}

#[cfg(CONFIG_ARM_PSCI_CPUIDLE)]
#[repr(C)]
pub struct psci_domain_idle_entry { pub cpu_id: u32, pub state: u32, pub s2idle: bool }
#[cfg(CONFIG_ARM_PSCI_CPUIDLE)]
pub unsafe fn psci_domain_idle_enter(_cpu_id: u32, _state: u32, _s2idle: bool) {}
#[cfg(CONFIG_ARM_PSCI_CPUIDLE)]
pub unsafe fn psci_domain_idle_exit(_cpu_id: u32, _state: u32, _s2idle: bool) {}

#[repr(C)]
pub struct pstate_sample_entry {
    pub core_busy: u32, pub scaled_busy: u32, pub from: u32, pub to: u32,
    pub mperf: u64, pub aperf: u64, pub tsc: u64, pub freq: u32, pub io_boost: u32,
}
pub unsafe fn pstate_sample(_core_busy: u32, _scaled_busy: u32, _from: u32, _to: u32,
                            _mperf: u64, _aperf: u64, _tsc: u64, _freq: u32,
                            _io_boost: u32) {}

pub fn pm_verb_symbolic(event: i32) -> &'static str {
    match event {
        PM_EVENT_SUSPEND => "suspend", PM_EVENT_RESUME => "resume",
        PM_EVENT_FREEZE => "freeze", PM_EVENT_QUIESCE => "quiesce",
        PM_EVENT_HIBERNATE => "hibernate", PM_EVENT_THAW => "thaw",
        PM_EVENT_RESTORE => "restore", PM_EVENT_RECOVER => "recover",
        PM_EVENT_POWEROFF => "poweroff", _ => "",
    }
}
extern "C" {
    pub static PM_EVENT_SUSPEND: i32; pub static PM_EVENT_RESUME: i32;
    pub static PM_EVENT_FREEZE: i32; pub static PM_EVENT_QUIESCE: i32;
    pub static PM_EVENT_HIBERNATE: i32; pub static PM_EVENT_THAW: i32;
    pub static PM_EVENT_RESTORE: i32; pub static PM_EVENT_RECOVER: i32;
    pub static PM_EVENT_POWEROFF: i32;
}

pub unsafe fn cpu_frequency(_frequency: u32, _cpu_id: u32) {}
#[repr(C)] pub struct cpu_frequency_limits_entry { pub min_freq: u32, pub max_freq: u32, pub cpu_id: u32 }
pub unsafe fn cpu_frequency_limits(_policy: *mut core::ffi::c_void) {}

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn device_pm_callback_start(_dev: *mut core::ffi::c_void, _pm_ops: *const i8, _event: i32) {}
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe fn device_pm_callback_end(_dev: *mut core::ffi::c_void, _error: i32) {}

#[repr(C)] pub struct suspend_resume_entry { pub action: *const i8, pub val: i32, pub start: bool }
pub unsafe fn suspend_resume(_action: *const i8, _val: i32, _start: bool) {}

#[repr(C)] pub struct wakeup_source_entry { pub name: *const i8, pub state: u64 }
pub unsafe fn wakeup_source_activate(_name: *const i8, _state: u32) {}
pub unsafe fn wakeup_source_deactivate(_name: *const i8, _state: u32) {}

#[cfg(CONFIG_ARCH_OMAP2PLUS)]
#[repr(C)] pub struct power_domain_entry { pub name: *const i8, pub state: u64, pub cpu_id: u64 }
#[cfg(CONFIG_ARCH_OMAP2PLUS)]
pub unsafe fn power_domain_target(_name: *const i8, _state: u32, _cpu_id: u32) {}

#[repr(C)] pub struct cpu_latency_qos_request_entry { pub value: i32 }
pub unsafe fn pm_qos_add_request(_value: i32) {}
pub unsafe fn pm_qos_update_request(_value: i32) {}
pub unsafe fn pm_qos_remove_request(_value: i32) {}

#[repr(C)] pub struct pm_qos_update_entry { pub action: i32, pub prev_value: i32, pub curr_value: i32 }
pub unsafe fn pm_qos_update_target(_action: i32, _prev_value: i32, _curr_value: i32) {}
pub unsafe fn pm_qos_update_flags(_action: i32, _prev_value: i32, _curr_value: i32) {}

#[repr(C)] pub struct dev_pm_qos_request_entry { pub name: *const i8, pub type_: i32, pub new_value: i32 }
pub unsafe fn dev_pm_qos_add_request(_name: *const i8, _type_: i32, _new_value: i32) {}
pub unsafe fn dev_pm_qos_update_request(_name: *const i8, _type_: i32, _new_value: i32) {}
pub unsafe fn dev_pm_qos_remove_request(_name: *const i8, _type_: i32, _new_value: i32) {}

#[repr(C)] pub struct guest_halt_poll_ns_entry { pub grow: bool, pub new: u32, pub old: u32 }
pub unsafe fn guest_halt_poll_ns(_grow: bool, _new: u32, _old: u32) {}
pub unsafe fn trace_guest_halt_poll_ns_grow(new: u32, old: u32) { guest_halt_poll_ns(true, new, old) }
pub unsafe fn trace_guest_halt_poll_ns_shrink(new: u32, old: u32) { guest_halt_poll_ns(false, new, old) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
