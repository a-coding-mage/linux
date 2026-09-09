/* SPDX-License-Identifier: GPL-2.0 */
#![allow(non_camel_case_types, non_snake_case, dead_code, improper_ctypes)]

/* Translated from linux/trace_events.h. Types supplied by included kernel headers
 * remain external dependencies. Configuration-dependent declarations are retained
 * under Rust cfg comments/conditions where their intent is file-local. */

pub const EVENT_NULL_STR: &[u8] = b"(null)\0";
pub const TRACE_EVENT_STR_MAX: usize = 512;
pub const PERF_MAX_TRACE_SIZE: usize = 8192;
pub const MAX_FILTER_STR_VAL: u32 = 256;
pub const MAX_DYNEVENT_CMD_LEN: usize = 2048;

#[repr(C)] pub struct trace_array { _private: [u8; 0] }
#[repr(C)] pub struct array_buffer { _private: [u8; 0] }
#[repr(C)] pub struct tracer { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct bpf_prog { _private: [u8; 0] }
#[repr(C)] pub union bpf_attr { _private: [u8; 0] }
#[repr(C)] pub struct trace_seq { _private: [u8; 0] }
#[repr(C)] pub struct trace_print_flags { _private: [u8; 0] }
#[repr(C)] pub struct trace_print_flags_u64 { _private: [u8; 0] }
#[repr(C)] pub struct trace_iterator { _private: [u8; 0] }
#[repr(C)] pub struct trace_event { _private: [u8; 0] }
#[repr(C)] pub struct trace_buffer { _private: [u8; 0] }
#[repr(C)] pub struct ring_buffer_event { _private: [u8; 0] }
#[repr(C)] pub struct trace_event_file { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct perf_event { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct event_filter { _private: [u8; 0] }
#[repr(C)] pub struct synth_event { _private: [u8; 0] }
#[repr(C)] pub struct synth_trace_event { _private: [u8; 0] }
#[repr(C)] pub struct bpf_raw_tp_link { _private: [u8; 0] }
#[repr(C)] pub struct bpf_raw_event_map { _private: [u8; 0] }
#[repr(C)] pub struct tracepoint { pub name: *const core::ffi::c_char }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct seq_buf { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }

pub type u16 = u16; pub type u32 = u32; pub type u64 = u64;
pub type loff_t = i64; pub type trace_print_func = unsafe extern "C" fn(*mut trace_iterator, i32, *mut trace_event) -> print_line_t;
pub type dynevent_create_fn_t = unsafe extern "C" fn(*mut dynevent_cmd) -> i32;

#[repr(C)] pub struct trace_dynamic_info { pub offset: u16, pub len: u16 }
#[repr(C)] pub struct trace_entry { pub type_: u16, pub flags: u8, pub preempt_count: u8, pub pid: i32 }
#[repr(C)] pub struct trace_event_functions { pub trace: Option<trace_print_func>, pub raw: Option<trace_print_func>, pub hex: Option<trace_print_func>, pub binary: Option<trace_print_func> }
#[repr(C)] pub struct trace_event { pub node: hlist_node, pub type_: i32, pub funcs: *mut trace_event_functions }

#[repr(C)] pub struct trace_iterator {
    pub tr: *mut trace_array, pub trace: *mut tracer, pub array_buffer: *mut array_buffer,
    pub private: *mut core::ffi::c_void, pub cpu_file: i32, pub mutex: mutex,
    pub buffer_iter: *mut *mut core::ffi::c_void, pub iter_flags: usize,
    pub temp: *mut core::ffi::c_void, pub temp_size: u32, pub fmt: *mut i8, pub fmt_size: u32,
    pub wait_index: atomic_t, pub tmp_seq: trace_seq, pub started: *mut core::ffi::c_void,
    pub closed: bool, pub snapshot: bool, pub seq: trace_seq, pub ent: *mut trace_entry,
    pub lost_events: usize, pub leftover: i32, pub ent_size: i32, pub cpu: i32, pub ts: u64,
    pub pos: loff_t, pub idx: i64,
}
#[repr(C)] pub struct trace_event_class { pub system: *const i8, pub probe: *mut core::ffi::c_void, pub reg: Option<unsafe extern "C" fn(*mut trace_event_call, trace_reg, *mut core::ffi::c_void) -> i32>, pub fields_array: *mut trace_event_fields, pub get_fields: Option<unsafe extern "C" fn(*mut trace_event_call) -> *mut list_head>, pub fields: list_head, pub raw_init: Option<unsafe extern "C" fn(*mut trace_event_call) -> i32> }
#[repr(C)] pub struct trace_event_fields { pub type_: *const i8, pub name: *const i8, pub size: i32, pub align: i32, pub is_signed: u32, pub needs_test: u32, pub filter_type: i32, pub len: i32 }
#[repr(C)] pub struct trace_event_call { pub list: list_head, pub class: *mut trace_event_class, pub name: *const i8, pub event: trace_event, pub print_fmt: *mut i8, pub module: *mut core::ffi::c_void, pub data: *mut core::ffi::c_void, pub flags: i32 }
#[repr(C)] pub struct trace_event_buffer { pub buffer: *mut trace_buffer, pub event: *mut ring_buffer_event, pub trace_file: *mut trace_event_file, pub entry: *mut core::ffi::c_void, pub trace_ctx: u32, pub regs: *mut pt_regs }
#[repr(C)] pub struct dynevent_cmd { pub seq: seq_buf, pub event_name: *const i8, pub n_fields: u32, pub type_: dynevent_type, pub run_command: Option<dynevent_create_fn_t>, pub private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct synth_field_desc { pub type_: *const i8, pub name: *const i8 }
#[repr(C)] pub struct synth_event_trace_state { pub fbuffer: trace_event_buffer, pub entry: *mut synth_trace_event, pub buffer: *mut trace_buffer, pub event: *mut synth_event, pub cur_field: u32, pub n_u64: u32, pub disabled: bool, pub add_next: bool, pub add_name: bool }
#[repr(C)] pub struct trace_event_file_full { pub list: list_head, pub event_call: *mut trace_event_call, pub filter: *mut event_filter, pub tr: *mut trace_array, pub triggers: list_head, pub flags: usize, pub ref_: refcount_t, pub sm_ref: atomic_t, pub tm_ref: atomic_t }

#[repr(C)] #[derive(Copy,Clone)] pub enum print_line_t { TRACE_TYPE_PARTIAL_LINE=0, TRACE_TYPE_HANDLED=1, TRACE_TYPE_UNHANDLED=2, TRACE_TYPE_NO_CONSUME=3 }
#[repr(C)] pub enum trace_reg { TRACE_REG_REGISTER, TRACE_REG_UNREGISTER, TRACE_REG_PERF_REGISTER, TRACE_REG_PERF_UNREGISTER, TRACE_REG_PERF_OPEN, TRACE_REG_PERF_CLOSE, TRACE_REG_PERF_ADD, TRACE_REG_PERF_DEL }
#[repr(C)] pub enum dynevent_type { DYNEVENT_TYPE_SYNTH=1, DYNEVENT_TYPE_KPROBE, DYNEVENT_TYPE_NONE }
#[repr(C)] pub enum event_trigger_type { ETT_NONE=0, ETT_TRACE_ONOFF=1, ETT_SNAPSHOT=2, ETT_STACKTRACE=4, ETT_EVENT_ENABLE=8, ETT_EVENT_HIST=16, ETT_HIST_ENABLE=32, ETT_EVENT_EPROBE=64 }

pub const TRACE_RECORD_CMDLINE: u32 = 1; pub const TRACE_RECORD_TGID: u32 = 2;
pub const TRACE_EVENT_FL_CAP_ANY: i32 = 1; pub const TRACE_EVENT_FL_NO_SET_FILTER: i32 = 2; pub const TRACE_EVENT_FL_IGNORE_ENABLE: i32 = 4; pub const TRACE_EVENT_FL_TRACEPOINT: i32 = 8; pub const TRACE_EVENT_FL_DYNAMIC: i32 = 16; pub const TRACE_EVENT_FL_KPROBE: i32 = 32; pub const TRACE_EVENT_FL_UPROBE: i32 = 64; pub const TRACE_EVENT_FL_EPROBE: i32 = 128; pub const TRACE_EVENT_FL_FPROBE: i32 = 256; pub const TRACE_EVENT_FL_CUSTOM: i32 = 512; pub const TRACE_EVENT_FL_TEST_STR: i32 = 1024; pub const TRACE_EVENT_FL_UKPROBE: i32 = TRACE_EVENT_FL_KPROBE | TRACE_EVENT_FL_UPROBE;

extern "C" {
    pub fn trace_print_flags_seq(*mut trace_seq,*const i8,usize,*const trace_print_flags,usize)->*const i8;
    pub fn trace_print_symbols_seq(*mut trace_seq,usize,*const trace_print_flags,usize)->*const i8;
    pub fn trace_print_bitmask_seq(*mut trace_iterator,*mut core::ffi::c_void,u32)->*const i8;
    pub fn trace_print_hex_seq(*mut trace_seq,*const u8,i32,bool)->*const i8;
    pub fn trace_print_array_seq(*mut trace_seq,*const core::ffi::c_void,i32,usize)->*const i8;
    pub fn trace_raw_output_prep(*mut trace_iterator,*mut trace_event)->i32;
    pub fn trace_event_printf(*mut trace_iterator,*const i8,...);
    pub fn register_trace_event(*mut trace_event)->i32; pub fn unregister_trace_event(*mut trace_event)->i32;
    pub fn trace_handle_return(*mut trace_seq)->print_line_t;
    pub fn tracing_gen_ctx_irq_test(u32)->u32; pub fn trace_event_buffer_reserve(*mut trace_event_buffer,*mut trace_event_file,usize)->*mut core::ffi::c_void; pub fn trace_event_buffer_commit(*mut trace_event_buffer);
    pub fn tracing_record_taskinfo(*mut task_struct,i32); pub fn tracing_record_cmdline(*mut task_struct); pub fn tracing_record_tgid(*mut task_struct);
    pub fn trace_event_reg(*mut trace_event_call,trace_reg,*mut core::ffi::c_void)->i32;
    pub fn dynevent_create(*mut dynevent_cmd)->i32; pub fn synth_event_delete(*const i8)->i32; pub fn synth_event_create(*const i8,*mut synth_field_desc,u32,*mut module)->i32;
    pub fn synth_event_trace(*mut trace_event_file,u32,...)->i32; pub fn synth_event_trace_array(*mut trace_event_file,*mut u64,u32)->i32;
    pub fn kprobe_event_delete(*const i8)->i32;
    pub fn filter_match_preds(*mut event_filter,*mut core::ffi::c_void)->i32;
    pub fn trace_event_raw_init(*mut trace_event_call)->i32; pub fn trace_add_event_call(*mut trace_event_call)->i32; pub fn trace_remove_event_call(*mut trace_event_call)->i32; pub fn trace_event_get_offsets(*mut trace_event_call)->i32;
    pub fn ftrace_set_clr_event(*mut trace_array,*mut i8,i32)->i32; pub fn trace_set_clr_event(*const i8,*const i8,i32)->i32;
}

// The remaining declarations are configuration- and macro-generated kernel APIs.
// Their source-level names and signatures are preserved here for downstream bindings.
pub const FILTER_OTHER: i32=0; pub const FILTER_STATIC_STRING: i32=1; pub const FILTER_DYN_STRING: i32=2; pub const FILTER_RDYN_STRING: i32=3; pub const FILTER_PTR_STRING: i32=4; pub const FILTER_TRACE_FN: i32=5; pub const FILTER_CPUMASK: i32=6; pub const FILTER_COMM: i32=7; pub const FILTER_CPU: i32=8; pub const FILTER_STACKTRACE: i32=9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
