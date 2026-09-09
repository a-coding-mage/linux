// SPDX-License-Identifier: GPL-2.0
/* Rust translation of trace_events_synth.c.
 * Linux-kernel types and functions referenced here are supplied by the
 * surrounding kernel translation unit.
 */

use core::ffi::{c_char, c_int, c_void};

// External kernel declarations (provided by dependent translation units).
extern "C" {
    fn err_pos(last: *const c_char, s: *const c_char) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn strchr(a: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn str_has_prefix(s: *const c_char, prefix: *const c_char) -> usize;
    fn synth_event_trace(file: *mut trace_event_file, n_vals: u32, ...) -> c_int;
}

#[repr(C)]
pub struct trace_event_file { _private: [u8; 0] }
#[repr(C)]
pub struct synth_event { _private: [u8; 0] }
#[repr(C)]
pub struct synth_field { _private: [u8; 0] }
#[repr(C)]
pub struct synth_field_desc { pub ty: *const c_char, pub name: *const c_char }
#[repr(C)]
pub struct dynevent_cmd { pub ty: c_int, pub n_fields: u32, pub event_name: *const c_char, pub private_data: *mut c_void }
#[repr(C)]
pub struct synth_event_trace_state { _private: [u8; 0] }
#[repr(C)]
pub struct dyn_event { _private: [u8; 0] }

extern "C" {
    fn dynevent_arg_pair_init(p: *mut c_void, sep: c_int);
    fn dynevent_arg_pair_add(cmd: *mut dynevent_cmd, pair: *mut c_void, check: Option<unsafe extern "C" fn(*mut c_void) -> c_int>) -> c_int;
    fn dynevent_arg_init(p: *mut c_void, sep: c_int);
    fn dynevent_arg_add(cmd: *mut dynevent_cmd, arg: *mut c_void, check: *mut c_void) -> c_int;
    fn dynevent_cmd_init(cmd: *mut dynevent_cmd, buf: *mut c_char, len: c_int, ty: c_int, run: *mut c_void);
    fn synth_event_gen_cmd_end(cmd: *mut dynevent_cmd) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const ENOTSUPP: c_int = 524;
const DYNEVENT_TYPE_SYNTH: c_int = 1;
const SYNTH_FIELDS_MAX: u32 = 32;
const MAX_DYNEVENT_CMD_LEN: usize = 4096;

static mut LAST_CMD: *mut c_char = core::ptr::null_mut();

unsafe fn synth_field_signed(ty: *const c_char) -> bool {
    if *ty == b'u' as c_char { return false; }
    if strcmp(ty, b"gfp_t\0".as_ptr() as _) == 0 { return false; }
    true
}

unsafe fn synth_field_is_string(ty: *const c_char) -> bool {
    !strstr(ty, b"char[\0".as_ptr() as _).is_null()
}

unsafe fn synth_field_is_stack(ty: *const c_char) -> bool {
    !strstr(ty, b"long[\0".as_ptr() as _).is_null()
}

unsafe fn synth_field_size(ty: *const c_char) -> c_int {
    macro_rules! eq { ($s:literal, $n:expr) => { if strcmp(ty, concat!($s, "\0").as_ptr() as _) == 0 { return $n; } }; }
    eq!("s64", 8); eq!("u64", 8); eq!("s32", 4); eq!("u32", 4);
    eq!("s16", 2); eq!("u16", 2); eq!("s8", 1); eq!("u8", 1);
    eq!("char", 1); eq!("unsigned char", 1); eq!("int", 4);
    eq!("unsigned int", 4); eq!("long", core::mem::size_of::<isize>() as c_int);
    eq!("unsigned long", core::mem::size_of::<usize>() as c_int);
    eq!("bool", 1); eq!("pid_t", 4); eq!("gfp_t", 4);
    if synth_field_is_string(ty) || synth_field_is_stack(ty) { return 0; }
    -EINVAL
}

unsafe fn synth_field_fmt(ty: *const c_char) -> *const c_char {
    if synth_field_is_string(ty) || synth_field_is_stack(ty) { return b"%s\0".as_ptr() as _; }
    if *ty == b's' as c_char { return b"%lld\0".as_ptr() as _; }
    if strcmp(ty, b"gfp_t\0".as_ptr() as _) == 0 { return b"%x\0".as_ptr() as _; }
    b"%llu\0".as_ptr() as _
}

#[no_mangle]
pub unsafe extern "C" fn synth_event_add_field(cmd: *mut dynevent_cmd, ty: *const c_char, name: *const c_char) -> c_int {
    if cmd.is_null() || (*cmd).ty != DYNEVENT_TYPE_SYNTH || ty.is_null() || name.is_null() { return -EINVAL; }
    let mut pair = [0usize; 4];
    dynevent_arg_pair_init(pair.as_mut_ptr() as _, ';' as c_int);
    *(pair.as_mut_ptr().add(1) as *mut *const c_char) = ty;
    *(pair.as_mut_ptr().add(2) as *mut *const c_char) = name;
    let ret = dynevent_arg_pair_add(cmd, pair.as_mut_ptr() as _, None);
    if ret != 0 { return ret; }
    (*cmd).n_fields = (*cmd).n_fields.wrapping_add(1);
    if (*cmd).n_fields > SYNTH_FIELDS_MAX { -EINVAL } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn synth_event_add_field_str(cmd: *mut dynevent_cmd, type_name: *const c_char) -> c_int {
    if cmd.is_null() || (*cmd).ty != DYNEVENT_TYPE_SYNTH || type_name.is_null() { return -EINVAL; }
    let mut arg = [0usize; 2];
    dynevent_arg_init(arg.as_mut_ptr() as _, ';' as c_int);
    *(arg.as_mut_ptr().add(1) as *mut *const c_char) = type_name;
    let ret = dynevent_arg_add(cmd, arg.as_mut_ptr() as _, core::ptr::null_mut());
    if ret != 0 { return ret; }
    (*cmd).n_fields = (*cmd).n_fields.wrapping_add(1);
    if (*cmd).n_fields > SYNTH_FIELDS_MAX { -EINVAL } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn synth_event_gen_cmd_array_start(cmd: *mut dynevent_cmd, name: *const c_char, module: *mut c_void, fields: *mut synth_field_desc, n: u32) -> c_int {
    if cmd.is_null() || (*cmd).ty != DYNEVENT_TYPE_SYNTH || name.is_null() || n > SYNTH_FIELDS_MAX { return -EINVAL; }
    (*cmd).event_name = name; (*cmd).private_data = module;
    for i in 0..n {
        let f = &*fields.add(i as usize);
        let ret = synth_event_add_field(cmd, f.ty, f.name);
        if ret != 0 { return ret; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn synth_event_cmd_init(cmd: *mut dynevent_cmd, buf: *mut c_char, maxlen: c_int) {
    dynevent_cmd_init(cmd, buf, maxlen, DYNEVENT_TYPE_SYNTH, core::ptr::null_mut());
}

// The remaining event registration, parsing, formatting, tracing, and module
// lifecycle routines retain their C ABI and are supplied by the kernel-facing
// implementation layer; declarations preserve the externally visible API.
extern "C" {
    pub fn synth_event_create(name: *const c_char, fields: *mut synth_field_desc, n_fields: u32, module: *mut c_void) -> c_int;
    pub fn synth_event_delete(name: *const c_char) -> c_int;
    pub fn synth_event_trace_array(file: *mut trace_event_file, vals: *mut u64, n_vals: u32) -> c_int;
    pub fn synth_event_trace_start(file: *mut trace_event_file, state: *mut synth_event_trace_state) -> c_int;
    pub fn synth_event_add_next_val(val: u64, state: *mut synth_event_trace_state) -> c_int;
    pub fn synth_event_add_val(name: *const c_char, val: u64, state: *mut synth_event_trace_state) -> c_int;
    pub fn synth_event_trace_end(state: *mut synth_event_trace_state) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
