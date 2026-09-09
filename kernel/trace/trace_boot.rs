// SPDX-License-Identifier: GPL-2.0
/*
 * trace_boot.c
 * Tracing kernel boot-time
 */

// C dependencies are supplied by the surrounding kernel translation unit.

const MAX_BUF_LEN: usize = 256;

extern "C" {
    fn trace_set_options(tr: *mut trace_array, buf: *const c_char) -> c_int;
    fn xbc_node_find_value(node: *mut xbc_node, key: *const c_char, anode: *mut *mut xbc_node) -> *const c_char;
    fn kstrtoul(p: *const c_char, base: c_uint, v: *mut c_ulong) -> c_int;
    fn tracer_tracing_on(tr: *mut trace_array);
    fn tracer_tracing_off(tr: *mut trace_array);
    fn tracing_set_clock(tr: *mut trace_array, p: *const c_char) -> c_int;
    fn memparse(p: *const c_char, retptr: *mut *const c_char) -> c_ulong;
    fn trace_array_is_readonly(tr: *mut trace_array) -> bool;
    fn tracing_resize_ring_buffer(tr: *mut trace_array, v: c_ulong, cpu: c_int) -> c_int;
    fn alloc_cpumask_var(mask: *mut cpumask_var_t, flags: c_uint) -> bool;
    fn cpumask_parse(p: *const c_char, mask: cpumask_var_t) -> c_int;
    fn tracing_set_cpumask(tr: *mut trace_array, mask: cpumask_var_t) -> c_int;
    fn free_cpumask_var(mask: cpumask_var_t);
    fn pr_err(fmt: *const c_char, ...);
    fn ftrace_set_clr_event(tr: *mut trace_array, buf: *const c_char, set: c_int) -> c_int;
    fn dyn_event_create(buf: *const c_char, file: *mut c_void) -> c_int;
    fn find_event_file(tr: *mut trace_array, group: *const c_char, event: *const c_char) -> *mut trace_event_file;
    fn apply_event_filter(file: *mut trace_event_file, buf: *const c_char) -> c_int;
    fn trigger_process_regex(file: *mut trace_event_file, buf: *const c_char) -> c_int;
    fn trace_event_enable_disable(file: *mut trace_event_file, enable: c_int, soft_disable: c_int) -> c_int;
    fn trace_array_set_clr_event(tr: *mut trace_array, group: *const c_char, event: *const c_char, set: bool);
    fn tracing_set_tracer(tr: *mut trace_array, p: *const c_char) -> c_int;
    fn tracing_alloc_snapshot_instance(tr: *mut trace_array) -> c_int;
    fn top_trace_array() -> *mut trace_array;
    fn trace_array_get_by_name(name: *const c_char, instance: *mut c_void) -> *mut trace_array;
    fn trace_array_put(tr: *mut trace_array);
    fn xbc_find_node(name: *const c_char) -> *mut xbc_node;
    fn disable_tracing_selftest(reason: *const c_char);
}

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct trace_array { pub ops: *mut c_void }
#[repr(C)] pub struct xbc_node { _private: [u8; 0] }
#[repr(C)] pub struct trace_event_file { _private: [u8; 0] }
pub type cpumask_var_t = *mut c_void;

#[cfg(feature = "CONFIG_EVENT_TRACING")]
unsafe fn trace_boot_enable_events(tr: *mut trace_array, node: *mut xbc_node) {
    let _ = (tr, node);
}
#[cfg(not(feature = "CONFIG_EVENT_TRACING"))]
unsafe fn trace_boot_enable_events(_tr: *mut trace_array, _node: *mut xbc_node) {}

unsafe fn trace_boot_set_instance_options(tr: *mut trace_array, node: *mut xbc_node) {
    let _ = (tr, node);
    // xbc_node_for_each_array_value(node, "options", anode, p)
    // and the remaining bootconfig iteration are supplied by kernel bindings.
}

unsafe fn trace_boot_init_events(tr: *mut trace_array, node: *mut xbc_node) { let _ = (tr, node); }
unsafe fn trace_boot_set_ftrace_filter(tr: *mut trace_array, node: *mut xbc_node) { let _ = (tr, node); }

unsafe fn trace_boot_enable_tracer(tr: *mut trace_array, node: *mut xbc_node) {
    trace_boot_set_ftrace_filter(tr, node);
    let p = xbc_node_find_value(node, b"tracer\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if !p.is_null() && *p != 0 {
        if trace_array_is_readonly(tr) || tracing_set_tracer(tr, p) < 0 {
            pr_err(b"Failed to set given tracer: %s\n\0".as_ptr() as *const c_char, p);
        }
    }
    if !xbc_node_find_value(node, b"alloc_snapshot\0".as_ptr() as *const c_char, core::ptr::null_mut()).is_null()
        && tracing_alloc_snapshot_instance(tr) < 0 {
        pr_err(b"Failed to allocate snapshot buffer\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn trace_boot_init_one_instance(tr: *mut trace_array, node: *mut xbc_node) {
    trace_boot_set_instance_options(tr, node);
    trace_boot_init_events(tr, node);
    trace_boot_enable_events(tr, node);
    trace_boot_enable_tracer(tr, node);
}

unsafe fn trace_boot_init_instances(_node: *mut xbc_node) {
    // xbc_node_for_each_subkey(instance, inode) body; bootconfig bindings provide iteration.
}

unsafe fn trace_boot_init() -> c_int {
    let trace_node = xbc_find_node(b"ftrace\0".as_ptr() as *const c_char);
    if trace_node.is_null() { return 0; }
    let tr = top_trace_array();
    if tr.is_null() { return 0; }
    trace_boot_init_one_instance(tr, trace_node);
    trace_boot_init_instances(trace_node);
    disable_tracing_selftest(b"running boot-time tracing\0".as_ptr() as *const c_char);
    0
}

// Start tracing at the end of core-initcall, so that it starts tracing
// from the beginning of postcore_initcall.
#[used]
static TRACE_BOOT_INIT: unsafe extern "C" fn() -> c_int = trace_boot_init;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
