/* SPDX-License-Identifier: GPL-2.0 */

// C header translation.
// The tracepoint implementation and registration are supplied by the tracepoint
// dependency; these declarations preserve the event payloads and call interface.

use core::ffi::c_char;

#[repr(C)]
pub struct SchedExtDumpEntry {
    pub line: *const c_char,
}

#[repr(C)]
pub struct SchedExtEventEntry {
    pub name: *const c_char,
    pub delta: i64,
}

#[repr(C)]
pub struct SchedExtBypassLbEntry {
    pub node: u32,
    pub nr_cpus: u32,
    pub nr_tasks: u32,
    pub nr_balanced: u32,
    pub before_min: u32,
    pub before_max: u32,
    pub after_min: u32,
    pub after_max: u32,
}

// `struct scx_sched` is declared by the scheduler dependency.
#[repr(C)]
pub struct ScxSched {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SchedExtExitEntry {
    pub name: *const c_char,
    pub level: i32,
    pub sub_cgroup_id: u64,
    pub cgrp_path: *const c_char,
    pub kind: u32,
}

extern "C" {
    pub fn trace_sched_ext_dump(line: *const c_char);
    pub fn trace_sched_ext_event(name: *const c_char, delta: i64);
    pub fn trace_sched_ext_bypass_lb(
        node: u32,
        nr_cpus: u32,
        nr_tasks: u32,
        nr_balanced: u32,
        before_min: u32,
        before_max: u32,
        after_min: u32,
        after_max: u32,
    );
    pub fn trace_sched_ext_exit(sch: *mut ScxSched, kind: u32);
}

// TRACE_EVENT(sched_ext_dump)
// TP_PROTO(const char *line)
// TP_STRUCT__entry(__string(line, line))
// TP_fast_assign(__assign_str(line))
// TP_printk("%s", __get_str(line))

// TRACE_EVENT(sched_ext_event)
// TP_PROTO(const char *name, __s64 delta)
// TP_STRUCT__entry(__string(name, name), __field(__s64, delta))
// TP_fast_assign(__assign_str(name); __entry->delta = delta)
// TP_printk("name %s delta %lld", __get_str(name), __entry->delta)

// TRACE_EVENT(sched_ext_bypass_lb)
// TP_PROTO(__u32 node, __u32 nr_cpus, __u32 nr_tasks, __u32 nr_balanced,
//          __u32 before_min, __u32 before_max, __u32 after_min, __u32 after_max)
// TP_STRUCT__entry contains the eight corresponding u32 fields above.
// TP_fast_assign copies each argument to its entry field.
// TP_printk("node %u: nr_cpus=%u nr_tasks=%u nr_balanced=%u min=%u->%u max=%u->%u", ...)

// TRACE_EVENT(sched_ext_exit)
// TP_PROTO(struct scx_sched *sch, __u32 kind)
// TP_STRUCT__entry contains name, level, sub_cgroup_id, cgrp_path, and kind.
// TP_fast_assign copies sch->level, sch->ops.sub_cgroup_id, and kind and assigns
// the scheduler name and cgroup path strings.
// TP_printk("sched %s level %d sub_cgroup_id %llu cgrp_path %s kind %u", ...)

// The C header includes <linux/tracepoint.h> and <trace/define_trace.h>;
// those dependencies provide the tracepoint machinery and registration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
