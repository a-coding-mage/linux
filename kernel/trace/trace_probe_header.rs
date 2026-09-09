// SPDX-License-Identifier: GPL-2.0
// Rust translation of trace_probe.h. Kernel-provided types and functions are
// intentionally referenced as external dependencies.

pub const MAX_TRACE_ARGS: usize = 128;
pub const MAX_ARGSTR_LEN: usize = 255;
pub const MAX_ARRAY_LEN: usize = 64;
pub const MAX_ARG_NAME_LEN: usize = 32;
pub const MAX_BTF_ARGS_LEN: usize = 128;
pub const MAX_DENTRY_ARGS_LEN: usize = 256;
pub const MAX_STRING_SIZE: usize = PATH_MAX;
pub const MAX_PROBE_EVENT_SIZE: usize = 3072;

pub const FIELD_STRING_IP: &[u8] = b"__probe_ip\0";
pub const FIELD_STRING_RETIP: &[u8] = b"__probe_ret_ip\0";
pub const FIELD_STRING_FUNC: &[u8] = b"__probe_func\0";

pub const TP_FLAG_TRACE: u32 = 1;
pub const TP_FLAG_PROFILE: u32 = 2;

#[inline]
pub const fn make_data_loc(len: u32, offs: u32) -> u32 { (len << 16) | (offs & 0xffff) }
#[inline]
pub const fn get_loc_len(dl: u32) -> u32 { dl >> 16 }
#[inline]
pub const fn get_loc_offs(dl: u32) -> u32 { dl & 0xffff }

#[inline]
pub unsafe fn get_loc_data(dl: *mut u32, ent: *mut core::ffi::c_void) -> *mut u8 {
    (ent as *mut u8).add(get_loc_offs(*dl) as usize)
}
#[inline]
pub const fn update_data_loc(loc: u32, consumed: u32) -> u32 {
    make_data_loc(get_loc_len(loc) - consumed, get_loc_offs(loc) + consumed)
}

pub type PrintTypeFunc = unsafe extern "C" fn(*mut trace_seq, *mut core::ffi::c_void, *mut core::ffi::c_void) -> i32;

#[repr(C)]
pub enum fetch_op {
    FETCH_OP_NOP, FETCH_OP_REG, FETCH_OP_STACK, FETCH_OP_STACKP, FETCH_OP_RETVAL,
    FETCH_OP_IMM, FETCH_OP_COMM, FETCH_OP_CURRENT, FETCH_OP_ARG, FETCH_OP_FOFFS,
    FETCH_OP_IMMSTR, FETCH_OP_EDATA, FETCH_OP_TP_ARG, FETCH_OP_DEREF, FETCH_OP_UDEREF,
    FETCH_OP_CPU_PTR, FETCH_OP_ST_RAW, FETCH_OP_ST_MEM, FETCH_OP_ST_UMEM,
    FETCH_OP_ST_STRING, FETCH_OP_ST_USTRING, FETCH_OP_ST_SYMSTR, FETCH_OP_ST_EDATA,
    FETCH_OP_MOD_BF, FETCH_OP_LP_ARRAY, FETCH_OP_END, FETCH_OP_NOP_SYMBOL,
}
pub const FETCH_NOP_SYMBOL: fetch_op = fetch_op::FETCH_OP_NOP_SYMBOL;

#[repr(C)]
pub union fetch_insn_data {
    pub param: u32,
    pub size_offset: fetch_insn_size_offset,
    pub bitfield: fetch_insn_bitfield,
    pub immediate: usize,
    pub data: *mut core::ffi::c_void,
}
#[repr(C)] pub struct fetch_insn_size_offset { pub size: u32, pub offset: i32 }
#[repr(C)] pub struct fetch_insn_bitfield { pub basesize: u8, pub lshift: u8, pub rshift: u8 }
#[repr(C)] pub struct fetch_insn { pub op: fetch_op, pub data: fetch_insn_data }

pub const FETCH_INSN_MAX: usize = 16;
pub const FETCH_TOKEN_COMM: i32 = -ECOMM;

#[repr(C)]
pub struct fetch_type {
    pub name: *const core::ffi::c_char, pub size: usize, pub is_signed: bool,
    pub is_string: bool, pub print: Option<PrintTypeFunc>,
    pub fmt: *const core::ffi::c_char, pub fmttype: *const core::ffi::c_char,
}
pub type string = u32;
pub type string_size = u32;

extern "C" {
    pub fn print_type_u8(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_u16(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_u32(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_u64(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_s8(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_s16(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_s32(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_s64(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_x8(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_x16(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_x32(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_x64(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_char(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_string(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
    pub fn print_type_symbol(s: *mut trace_seq, data: *mut core::ffi::c_void, ent: *mut core::ffi::c_void) -> i32;
}

#[repr(C)] pub struct probe_arg { pub code: *mut fetch_insn, pub dynamic: bool, pub offset: u32, pub count: u32, pub name: *const core::ffi::c_char, pub comm: *const core::ffi::c_char, pub fmt: *mut core::ffi::c_char, pub type_: *const fetch_type }
#[repr(C)] pub struct probe_entry_arg { pub size: u32, pub code: [fetch_insn; 0] }
#[repr(C)] pub struct trace_uprobe_filter { pub rwlock: rwlock_t, pub nr_systemwide: i32, pub perf_events: list_head }
#[repr(C)] pub struct trace_probe_event { pub flags: u32, pub class: trace_event_class, pub call: trace_event_call, pub files: list_head, pub probes: list_head, pub filter: [trace_uprobe_filter; 0] }
#[repr(C)] pub struct trace_probe { pub list: list_head, pub event: *mut trace_probe_event, pub size: isize, pub nr_args: u32, pub entry_arg: *mut probe_entry_arg, pub args: [probe_arg; 0] }
#[repr(C)] pub struct event_file_link { pub file: *mut trace_event_file, pub list: list_head }

#[inline] pub unsafe fn trace_probe_load_flag(tp: *mut trace_probe) -> u32 { (*(*tp).event).flags }
#[inline] pub unsafe fn trace_probe_test_flag(tp: *mut trace_probe, flag: u32) -> bool { trace_probe_load_flag(tp) & flag != 0 }
#[inline] pub unsafe fn trace_probe_set_flag(tp: *mut trace_probe, flag: u32) { (*(*tp).event).flags |= flag; }
#[inline] pub unsafe fn trace_probe_clear_flag(tp: *mut trace_probe, flag: u32) { (*(*tp).event).flags &= !flag; }
#[inline] pub unsafe fn trace_probe_is_enabled(tp: *mut trace_probe) -> bool { trace_probe_test_flag(tp, TP_FLAG_TRACE | TP_FLAG_PROFILE) }
#[inline] pub unsafe fn trace_probe_event_call(tp: *mut trace_probe) -> *mut trace_event_call { &mut (*(*tp).event).call }
#[inline] pub unsafe fn trace_probe_probe_list(tp: *mut trace_probe) -> *mut list_head { &mut (*(*tp).event).probes }
#[inline] pub unsafe fn trace_probe_has_single_file(tp: *mut trace_probe) -> bool { list_is_singular(&(*(*tp).event).files) }

pub const TPARG_FL_RETURN: u32 = 1 << 0; pub const TPARG_FL_KERNEL: u32 = 1 << 1;
pub const TPARG_FL_FENTRY: u32 = 1 << 2; pub const TPARG_FL_TEVENT: u32 = 1 << 3;
pub const TPARG_FL_USER: u32 = 1 << 4; pub const TPARG_FL_FPROBE: u32 = 1 << 5;
pub const TPARG_FL_TPOINT: u32 = 1 << 6; pub const TPARG_FL_LOC_MASK: u32 = 0x1f;
#[inline] pub const fn tparg_is_function_entry(f: u32) -> bool { f & TPARG_FL_LOC_MASK == TPARG_FL_KERNEL | TPARG_FL_FENTRY }
#[inline] pub const fn tparg_is_function_return(f: u32) -> bool { f & TPARG_FL_LOC_MASK == TPARG_FL_KERNEL | TPARG_FL_RETURN }
#[inline] pub const fn tparg_is_event_probe(f: u32) -> bool { f & TPARG_FL_TEVENT != 0 }
pub const TRACEPROBE_MAX_NESTED_LEVEL: usize = 8;

#[repr(C)] pub enum parse_state_type { STATE_DEREF, STATE_TYPECAST }
#[repr(C)] pub struct parse_state { pub type_: i32, pub data: parse_state_data }
#[repr(C)] pub union parse_state_data { pub deref: parse_state_deref, pub typecast: parse_state_typecast }
#[repr(C)] pub struct parse_state_deref { pub deref: i32, pub offset: isize, pub cur_offs: i32, pub inner_arg: *mut core::ffi::c_char, pub is_cpu_read: bool }
#[repr(C)] pub struct parse_state_typecast { pub casttype: *mut core::ffi::c_char, pub fieldname: *mut core::ffi::c_char, pub orig_offset: i32, pub field_offset_diff: i32, pub inner_arg: *mut core::ffi::c_char }
#[repr(C)] pub struct traceprobe_parse_context { pub event: *mut trace_event_call, pub funcname: *const core::ffi::c_char, pub proto: *const btf_type, pub params: *const btf_param, pub nr_params: i32, pub btf: *mut btf, pub struct_btf: *mut btf, pub last_type: *const btf_type, pub last_struct: *const btf_type, pub last_bitoffs: u32, pub last_bitsize: u32, pub tp: *mut trace_probe, pub flags: u32, pub offset: i32, pub prefix_byteoffs: i32, pub stack: [parse_state; TRACEPROBE_MAX_NESTED_LEVEL + 1], pub depth: i32 }

#[repr(C)] pub enum probe_print_type { PROBE_PRINT_NORMAL, PROBE_PRINT_RETURN, PROBE_PRINT_EVENT }
#[repr(C)] pub struct trace_probe_log { pub subsystem: *const core::ffi::c_char, pub argv: *const *const core::ffi::c_char, pub argc: i32, pub index: i32 }
#[repr(C)] pub struct uprobe_dispatch_data { pub tu: *mut trace_uprobe, pub bp_addr: usize }

// The ERRORS X-macro expands to the following stable error-number namespace.
#[repr(i32)] pub enum trace_probe_error { TP_ERR_ARGIDX_2BIG, TP_ERR_ARGS_2LONG, TP_ERR_ARG_NAME_TOO_LONG, TP_ERR_ARG_TOO_LONG, TP_ERR_ARRAY_NO_CLOSE, TP_ERR_ARRAY_TOO_BIG, TP_ERR_BAD_ADDR_SUFFIX, TP_ERR_BAD_ARG_NAME, TP_ERR_BAD_ARG_NUM, TP_ERR_BAD_ARRAY_NUM, TP_ERR_BAD_ARRAY_SUFFIX, TP_ERR_BAD_ATTACH_ARG, TP_ERR_BAD_ATTACH_EVENT, TP_ERR_BAD_BITFIELD, TP_ERR_BAD_BTF_TID, TP_ERR_BAD_DEREF_OFFS, TP_ERR_BAD_EVENT_NAME, TP_ERR_BAD_FETCH_ARG, TP_ERR_BAD_FILE_OFFS, TP_ERR_BAD_GROUP_NAME, TP_ERR_BAD_HYPHEN, TP_ERR_BAD_IMM, TP_ERR_BAD_INSN_BNDRY, TP_ERR_BAD_MAXACT, TP_ERR_BAD_MAXACT_TYPE, TP_ERR_BAD_MEM_ADDR, TP_ERR_BAD_PROBE_ADDR, TP_ERR_BAD_REFCNT, TP_ERR_BAD_REFCNT_SUFFIX, TP_ERR_BAD_REG_NAME, TP_ERR_BAD_RETPROBE, TP_ERR_BAD_STACK_NUM, TP_ERR_BAD_STRING, TP_ERR_BAD_SYMSTRING, TP_ERR_BAD_TP_NAME, TP_ERR_BAD_TYPE, TP_ERR_BAD_TYPE4STR, TP_ERR_BAD_UPROBE_OFFS, TP_ERR_BAD_VAR, TP_ERR_BAD_VAR_ARGS, TP_ERR_COMM_CANT_DEREF, TP_ERR_DEREF_NEED_BRACE, TP_ERR_DEREF_OPEN_BRACE, TP_ERR_DIFF_ARG_TYPE, TP_ERR_DIFF_PROBE_TYPE, TP_ERR_DOUBLE_ARGS, TP_ERR_EVENT_EXIST, TP_ERR_EVENT_TOO_BIG, TP_ERR_EVENT_TOO_LONG, TP_ERR_FAIL_REG_PROBE, TP_ERR_FILE_NOT_FOUND, TP_ERR_FILE_ON_KPROBE, TP_ERR_GROUP_TOO_LONG, TP_ERR_IMMSTR_NO_CLOSE, TP_ERR_MAXACT_TOO_BIG, TP_ERR_NEED_STRING_TYPE, TP_ERR_NOFENTRY_ARGS, TP_ERR_NON_UNIQ_SYMBOL, TP_ERR_NOSUP_BTFARG, TP_ERR_NOSUP_DAT_ARG, TP_ERR_NOSUP_PERCPU, TP_ERR_NO_ARG_BODY, TP_ERR_NO_ARG_NAME, TP_ERR_NO_BTFARG, TP_ERR_NO_BTF_ENTRY, TP_ERR_NO_BTF_FIELD, TP_ERR_NO_EP_FILTER, TP_ERR_NO_EVENT_FIELD, TP_ERR_NO_EVENT_INFO, TP_ERR_NO_EVENT_NAME, TP_ERR_NO_GROUP_NAME, TP_ERR_NO_PTR_STRCT, TP_ERR_NO_REGULAR_FILE, TP_ERR_NO_RETVAL, TP_ERR_NO_TRACEPOINT, TP_ERR_REFCNT_OPEN_BRACE, TP_ERR_RETVAL_ON_PROBE, TP_ERR_SAME_PROBE, TP_ERR_SYM_ON_UPROBE, TP_ERR_TOO_MANY_ARGS, TP_ERR_TOO_MANY_EARGS, TP_ERR_TOO_MANY_NESTED, TP_ERR_TOO_MANY_OPS, TP_ERR_TYPECAST_BAD_ARROW, TP_ERR_TYPECAST_NOT_ALIGNED, TP_ERR_TYPECAST_NOT_EVENT, TP_ERR_TYPECAST_REQ_FIELD, TP_ERR_TYPECAST_SYM_OFFSET, TP_ERR_USED_ARG_NAME }

extern "C" {
    pub fn trace_probe_init(tp: *mut trace_probe, event: *const i8, group: *const i8, alloc_filter: bool, nargs: i32) -> i32;
    pub fn trace_probe_cleanup(tp: *mut trace_probe); pub fn trace_probe_append(tp: *mut trace_probe, to: *mut trace_probe) -> i32;
    pub fn trace_probe_unlink(tp: *mut trace_probe); pub fn trace_probe_register_event_call(tp: *mut trace_probe) -> i32;
    pub fn trace_probe_add_file(tp: *mut trace_probe, file: *mut trace_event_file) -> i32;
    pub fn trace_probe_remove_file(tp: *mut trace_probe, file: *mut trace_event_file) -> i32;
    pub fn trace_probe_get_file_link(tp: *mut trace_probe, file: *mut trace_event_file) -> *mut event_file_link;
    pub fn trace_probe_compare_arg_type(a: *mut trace_probe, b: *mut trace_probe) -> i32;
    pub fn trace_probe_match_command_args(tp: *mut trace_probe, argc: i32, argv: *const *const i8) -> bool;
    pub fn trace_probe_create(raw_command: *const i8, createfn: Option<unsafe extern "C" fn(i32, *const *const i8) -> i32>) -> i32;
    pub fn trace_probe_print_args(s: *mut trace_seq, args: *mut probe_arg, nr_args: i32, data: *mut u8, field: *mut core::ffi::c_void) -> i32;
    pub fn traceprobe_parse_probe_arg(tp: *mut trace_probe, i: i32, argv: *const i8, ctx: *mut traceprobe_parse_context) -> i32;
    pub fn traceprobe_finish_parse(ctx: *mut traceprobe_parse_context); pub fn trace_probe_log_init(subsystem: *const i8, argc: i32, argv: *const *const i8) -> *const i8;
    pub fn trace_probe_log_set_index(index: i32); pub fn trace_probe_log_clear(); pub fn __trace_probe_log_err(offset: i32, err: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
