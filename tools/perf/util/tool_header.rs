/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/tool.h.
// C includes omitted: <stdbool.h>, <linux/types.h>.

use std::os::raw::{c_char, c_int};

pub type s64 = i64;
pub type u64 = u64;

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ordered_events {
    _private: [u8; 0],
}

pub type event_sample = Option<
    unsafe extern "C" fn(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int,
>;

pub type event_op = Option<
    unsafe extern "C" fn(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int,
>;

pub type event_attr_op = Option<
    unsafe extern "C" fn(
        tool: *const perf_tool,
        event: *mut perf_event,
        pevlist: *mut *mut evlist,
    ) -> c_int,
>;

pub type event_op2 = Option<
    unsafe extern "C" fn(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> c_int,
>;

pub type event_op3 = Option<
    unsafe extern "C" fn(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> s64,
>;

pub type event_op4 = Option<
    unsafe extern "C" fn(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
        data: u64,
        str_: *const c_char,
    ) -> c_int,
>;

pub type event_oe = Option<
    unsafe extern "C" fn(
        tool: *const perf_tool,
        event: *mut perf_event,
        oe: *mut ordered_events,
    ) -> c_int,
>;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum show_feature_header {
    SHOW_FEAT_NO_HEADER = 0,
    SHOW_FEAT_HEADER,
    SHOW_FEAT_HEADER_FULL_INFO,
}

#[repr(C)]
pub struct perf_tool {
    pub sample: event_sample,
    pub read: event_sample,
    pub callchain_deferred: event_sample,
    pub mmap: event_op,
    pub mmap2: event_op,
    pub comm: event_op,
    pub namespaces: event_op,
    pub cgroup: event_op,
    pub fork: event_op,
    pub exit: event_op,
    pub lost: event_op,
    pub lost_samples: event_op,
    pub aux: event_op,
    pub itrace_start: event_op,
    pub aux_output_hw_id: event_op,
    pub context_switch: event_op,
    pub throttle: event_op,
    pub unthrottle: event_op,
    pub ksymbol: event_op,
    pub bpf: event_op,
    pub text_poke: event_op,
    pub attr: event_attr_op,
    pub event_update: event_attr_op,
    pub tracing_data: event_op2,
    pub finished_round: event_oe,
    pub build_id: event_op2,
    pub id_index: event_op2,
    pub auxtrace_info: event_op2,
    pub auxtrace_error: event_op2,
    pub time_conv: event_op2,
    pub thread_map: event_op2,
    pub cpu_map: event_op2,
    pub stat_config: event_op2,
    pub stat: event_op2,
    pub stat_round: event_op2,
    pub feature: event_op2,
    pub finished_init: event_op2,
    pub bpf_metadata: event_op2,
    pub schedstat_cpu: event_op2,
    pub schedstat_domain: event_op2,
    pub compressed: event_op4,
    pub auxtrace: event_op3,
    pub ordered_events: bool,
    pub ordering_requires_timestamps: bool,
    pub namespace_events: bool,
    pub cgroup_events: bool,
    pub no_warn: bool,
    pub dont_split_sample_group: bool,
    pub merge_deferred_callchains: bool,
    pub show_feat_hdr: show_feature_header,
}

unsafe extern "C" {
    pub fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);

    pub fn perf_tool__compressed_is_stub(tool: *const perf_tool) -> bool;

    pub fn process_event_sample_stub(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
}

#[repr(C)]
pub struct delegate_tool {
    /** @tool: The actual tool that calls the delegate. */
    pub tool: perf_tool,
    /** @delegate: The tool that is delegated to. */
    pub delegate: *mut perf_tool,
}

unsafe extern "C" {
    pub fn delegate_tool__init(tool: *mut delegate_tool, delegate: *mut perf_tool);
}
