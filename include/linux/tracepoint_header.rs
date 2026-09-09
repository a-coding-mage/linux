/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/tracepoint.h. Included dependencies are supplied elsewhere. */

use core::ffi::c_void;

#[repr(C)]
pub struct trace_eval_map {
    pub system: *const core::ffi::c_char,
    pub eval_string: *const core::ffi::c_char,
    pub eval_value: c_ulong,
}

pub const TRACEPOINT_DEFAULT_PRIO: c_int = 10;

extern "C" {
    pub fn tracepoint_probe_register(tp: *mut tracepoint, probe: *mut c_void, data: *mut c_void) -> c_int;
    pub fn tracepoint_probe_register_prio(tp: *mut tracepoint, probe: *mut c_void, data: *mut c_void, prio: c_int) -> c_int;
    pub fn tracepoint_probe_register_prio_may_exist(tp: *mut tracepoint, probe: *mut c_void, data: *mut c_void, prio: c_int) -> c_int;
    pub fn tracepoint_probe_unregister(tp: *mut tracepoint, probe: *mut c_void, data: *mut c_void) -> c_int;
    pub fn for_each_kernel_tracepoint(fct: Option<unsafe extern "C" fn(*mut tracepoint, *mut c_void)>, priv_: *mut c_void);
}

#[inline]
pub unsafe fn tracepoint_probe_register_may_exist(tp: *mut tracepoint, probe: *mut c_void, data: *mut c_void) -> c_int {
    tracepoint_probe_register_prio_may_exist(tp, probe, data, TRACEPOINT_DEFAULT_PRIO)
}

#[cfg(feature = "CONFIG_MODULES")]
#[repr(C)]
pub struct tp_module { pub list: list_head, pub mod_: *mut module }

#[cfg(feature = "CONFIG_MODULES")]
extern "C" {
    pub fn trace_module_has_bad_taint(m: *mut module) -> bool;
    pub fn register_tracepoint_module_notifier(nb: *mut notifier_block) -> c_int;
    pub fn unregister_tracepoint_module_notifier(nb: *mut notifier_block) -> c_int;
    pub fn for_each_module_tracepoint(fct: Option<unsafe extern "C" fn(*mut tracepoint, *mut module, *mut c_void)>, priv_: *mut c_void);
    pub fn for_each_tracepoint_in_module(m: *mut module, fct: Option<unsafe extern "C" fn(*mut tracepoint, *mut module, *mut c_void)>, priv_: *mut c_void);
}

#[cfg(not(feature = "CONFIG_MODULES"))]
#[inline] pub unsafe fn trace_module_has_bad_taint(_: *mut module) -> bool { false }
#[cfg(not(feature = "CONFIG_MODULES"))]
#[inline] pub unsafe fn register_tracepoint_module_notifier(_: *mut notifier_block) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_MODULES"))]
#[inline] pub unsafe fn unregister_tracepoint_module_notifier(_: *mut notifier_block) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_MODULES"))]
#[inline] pub unsafe fn for_each_module_tracepoint(_: Option<unsafe extern "C" fn(*mut tracepoint, *mut module, *mut c_void)>, _: *mut c_void) {}
#[cfg(not(feature = "CONFIG_MODULES"))]
#[inline] pub unsafe fn for_each_tracepoint_in_module(_: *mut module, _: Option<unsafe extern "C" fn(*mut tracepoint, *mut module, *mut c_void)>, _: *mut c_void) {}

#[cfg(feature = "CONFIG_TRACEPOINTS")]
extern "C" {
    pub static mut tracepoint_srcu: srcu_struct;
    pub fn synchronize_rcu_tasks_trace();
    pub fn synchronize_srcu(s: *mut srcu_struct);
    pub fn call_srcu(s: *mut srcu_struct, rcu: *mut rcu_head, func: Option<unsafe extern "C" fn(*mut rcu_head)>);
    pub fn call_rcu_tasks_trace(rcu: *mut rcu_head, func: Option<unsafe extern "C" fn(*mut rcu_head)>);
}

#[cfg(feature = "CONFIG_TRACEPOINTS")]
#[inline] pub unsafe fn tracepoint_synchronize_unregister() { synchronize_rcu_tasks_trace(); synchronize_srcu(&raw mut tracepoint_srcu); }
#[cfg(feature = "CONFIG_TRACEPOINTS")]
#[inline] pub unsafe fn tracepoint_is_faultable(tp: *mut tracepoint) -> bool { (*tp).ext != core::ptr::null_mut() && (*(*tp).ext).faultable }
#[cfg(not(feature = "CONFIG_TRACEPOINTS"))]
#[inline] pub unsafe fn tracepoint_synchronize_unregister() {}
#[cfg(not(feature = "CONFIG_TRACEPOINTS"))]
#[inline] pub unsafe fn tracepoint_is_faultable(_: *mut tracepoint) -> bool { false }
#[cfg(feature = "CONFIG_TRACEPOINTS")]
#[inline] pub unsafe fn call_tracepoint_unregister_atomic(r: *mut rcu_head, f: Option<unsafe extern "C" fn(*mut rcu_head)>) { call_srcu(&raw mut tracepoint_srcu, r, f) }
#[cfg(feature = "CONFIG_TRACEPOINTS")]
#[inline] pub unsafe fn call_tracepoint_unregister_syscall(r: *mut rcu_head, f: Option<unsafe extern "C" fn(*mut rcu_head)>) { call_rcu_tasks_trace(r, f) }
#[cfg(not(feature = "CONFIG_TRACEPOINTS"))]
#[inline] pub unsafe fn call_tracepoint_unregister_atomic(_: *mut rcu_head, _: Option<unsafe extern "C" fn(*mut rcu_head)>) {}
#[cfg(not(feature = "CONFIG_TRACEPOINTS"))]
#[inline] pub unsafe fn call_tracepoint_unregister_syscall(_: *mut rcu_head, _: Option<unsafe extern "C" fn(*mut rcu_head)>) {}

#[cfg(feature = "CONFIG_HAVE_SYSCALL_TRACEPOINTS")]
extern "C" { pub fn syscall_regfunc() -> c_int; pub fn syscall_unregfunc(); }

/* C macro compatibility: these intentionally retain token-level expansion semantics. */
#[macro_export] macro_rules! PARAMS { ($($args:tt)*) => { $($args)* }; }
#[macro_export] macro_rules! TRACE_DEFINE_ENUM { ($x:expr) => {}; }
#[macro_export] macro_rules! TRACE_DEFINE_SIZEOF { ($x:ty) => {}; }
#[macro_export] macro_rules! EXPORT_TRACEPOINT_SYMBOL_GPL { ($name:ident) => {}; }
#[macro_export] macro_rules! EXPORT_TRACEPOINT_SYMBOL { ($name:ident) => {}; }
#[macro_export] macro_rules! TRACE_EVENT_FLAGS { ($event:ident, $flag:expr) => {}; }
#[macro_export] macro_rules! TRACE_EVENT_PERF_PERM { ($event:ident $(, $expr:expr)*) => {}; }
#[macro_export] macro_rules! tracepoint_string { ($str:expr) => { $str }; }

#[macro_export] macro_rules! DECLARE_TRACE {
    ($name:ident, ($($proto:tt)*), ($($args:tt)*)) => { crate::__DECLARE_TRACE!($name, ($($proto)*), ($($args)*), true, (*mut c_void, $($proto)*)); };
}
#[macro_export] macro_rules! DECLARE_TRACE_CONDITION { ($name:ident, ($($proto:tt)*), ($($args:tt)*), $cond:expr) => { crate::DECLARE_TRACE!($name, ($($proto)*), ($($args)*)); }; }
#[macro_export] macro_rules! DECLARE_TRACE_SYSCALL { ($name:ident, ($($proto:tt)*), ($($args:tt)*)) => { crate::DECLARE_TRACE!($name, ($($proto)*), ($($args)*)); }; }
#[macro_export] macro_rules! DECLARE_TRACE_EVENT { ($name:ident, ($($proto:tt)*), ($($args:tt)*)) => { crate::DECLARE_TRACE!($name, ($($proto)*), ($($args)*)); }; }
#[macro_export] macro_rules! DECLARE_TRACE_EVENT_CONDITION { ($name:ident, ($($proto:tt)*), ($($args:tt)*), $cond:expr) => { crate::DECLARE_TRACE_EVENT!($name, ($($proto)*), ($($args)*)); }; }
#[macro_export] macro_rules! DECLARE_TRACE_EVENT_SYSCALL { ($name:ident, ($($proto:tt)*), ($($args:tt)*)) => { crate::DECLARE_TRACE_EVENT!($name, ($($proto)*), ($($args)*)); }; }
#[macro_export] macro_rules! TRACE_EVENT { ($name:ident, $proto:tt, $args:tt, $struct:tt, $assign:tt, $print:tt) => { crate::DECLARE_TRACE_EVENT!($name, $proto, $args); }; }
#[macro_export] macro_rules! TRACE_EVENT_FN { ($name:ident, $proto:tt, $args:tt, $struct:tt, $assign:tt, $print:tt, $reg:tt, $unreg:tt) => { crate::DECLARE_TRACE_EVENT!($name, $proto, $args); }; }
#[macro_export] macro_rules! DECLARE_EVENT_NOP { ($name:ident, ($($proto:tt)*), $args:tt) => {}; }
#[macro_export] macro_rules! TRACE_EVENT_NOP { ($name:ident, $proto:tt, $args:tt, $struct:tt, $assign:tt, $print:tt) => {}; }
#[macro_export] macro_rules! DECLARE_EVENT_CLASS { ($($args:tt)*) => {}; }
#[macro_export] macro_rules! DEFINE_EVENT { ($template:ident, $name:ident, $proto:tt, $args:tt) => { crate::DECLARE_TRACE_EVENT!($name, $proto, $args); }; }
#[macro_export] macro_rules! DEFINE_EVENT_FN { ($template:ident, $name:ident, $proto:tt, $args:tt, $reg:tt, $unreg:tt) => { crate::DECLARE_TRACE_EVENT!($name, $proto, $args); }; }
#[macro_export] macro_rules! DEFINE_TRACE_FN { ($name:ident, $reg:tt, $unreg:tt, $proto:tt, $args:tt) => {}; }
#[macro_export] macro_rules! DEFINE_TRACE_SYSCALL { ($name:ident, $reg:tt, $unreg:tt, $proto:tt, $args:tt) => {}; }
#[macro_export] macro_rules! DEFINE_TRACE { ($name:ident, $proto:tt, $args:tt) => {}; }
#[macro_export] macro_rules! DEFINE_EVENT_PRINT { ($template:ident, $name:ident, $proto:tt, $args:tt, $print:tt) => { crate::DECLARE_TRACE_EVENT!($name, $proto, $args); }; }
#[macro_export] macro_rules! DEFINE_EVENT_CONDITION { ($template:ident, $name:ident, $proto:tt, $args:tt, $cond:expr) => { crate::DECLARE_TRACE_EVENT_CONDITION!($name, $proto, $args, $cond); }; }
#[macro_export] macro_rules! DEFINE_EVENT_NOP { ($template:ident, $name:ident, $proto:tt, $args:tt) => {}; }
#[macro_export] macro_rules! DECLARE_EVENT_CLASS_NOP { ($($args:tt)*) => {}; }
#[macro_export] macro_rules! __TRACEPOINT_ENTRY { ($name:ident) => {}; }
#[macro_export] macro_rules! __DO_TRACE_CALL { ($name:ident, $($args:tt)*) => {}; }
#[macro_export] macro_rules! __DEFINE_RUST_DO_TRACE { ($name:ident, $proto:tt, $args:tt) => {}; }
#[macro_export] macro_rules! DEFINE_RUST_DO_TRACE { ($name:ident, $proto:tt, $args:tt) => {}; }
#[macro_export] macro_rules! TRACEPOINT_CHECK { ($name:ident) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
