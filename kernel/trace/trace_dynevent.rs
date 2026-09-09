// SPDX-License-Identifier: GPL-2.0
/* Generic dynamic event control interface. */
// C dependencies from linux/debugfs.h, linux/kernel.h, linux/list.h,
// linux/mm.h, linux/mutex.h, linux/tracefs.h, trace.h,
// trace_output.h, and trace_dynevent.h are supplied externally.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut dyn_event_ops_mutex: c_void;
    static mut dyn_event_list: c_void;
    static mut event_mutex: c_void;
    static mut trace_event_sem: c_void;
    static mut ftrace_events: c_void;

    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn down_read(sem: *mut c_void);
    fn up_read(sem: *mut c_void);
    fn atomic_inc(v: *mut c_void);
    fn atomic_dec(v: *mut c_void);
    fn atomic_read(v: *mut c_void) -> c_int;
    fn atomic_set(v: *mut c_void, value: c_int);
    fn tracing_reset_all_online_cpus();
    fn tracing_log_err(a: *mut c_void, b: *const c_char, c: *const c_char,
                       d: *const *const c_char, e: c_int, f: c_int);
    fn tracing_init_dentry() -> c_int;
    fn trace_create_file(name: *const c_char, mode: c_int, parent: *mut c_void,
                         data: *mut c_void, fops: *const c_void);
    fn trace_parse_run_command(file: *mut file, buffer: *const c_char,
                               count: usize, pos: *mut i64,
                               command: unsafe extern "C" fn(*const c_char) -> c_int) -> isize;
    fn seq_list_start(list: *mut c_void, pos: i64) -> *mut c_void;
    fn seq_list_next(v: *mut c_void, list: *mut c_void, pos: *mut i64) -> *mut c_void;
    fn seq_open(file: *mut file, op: *const seq_operations) -> c_int;
    fn seq_read(file: *mut file, buf: *mut c_char, count: usize, pos: *mut i64) -> isize;
    fn seq_lseek(file: *mut file, pos: i64, whence: c_int) -> i64;
    fn seq_release(inode: *mut inode, file: *mut file) -> c_int;
    fn security_locked_down(reason: c_int) -> c_int;
    fn tracing_check_open_get_tr(file: *mut c_void) -> c_int;
    fn seq_buf_printf(seq: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn seq_buf_puts(seq: *mut c_void, s: *const c_char) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
}

#[repr(C)] pub struct trace_event_call { pub flags: u32, pub refcnt: c_void }
#[repr(C)] pub struct dyn_event_operations { pub list: c_void, pub create: Option<unsafe extern "C" fn(*const c_char) -> c_int>, pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut dyn_event) -> c_int>, pub is_busy: Option<unsafe extern "C" fn(*mut dyn_event) -> bool>, pub free: Option<unsafe extern "C" fn(*mut dyn_event) -> c_int>, pub r#match: Option<unsafe extern "C" fn(*mut c_char, *mut c_char, c_int, *const *const c_char, *mut dyn_event) -> bool> }
#[repr(C)] pub struct dyn_event { pub list: c_void, pub ops: *mut dyn_event_operations }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { pub f_mode: u32, pub f_flags: u32 }
#[repr(C)] pub struct seq_operations { pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut c_void>, pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut i64) -> *mut c_void>, pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>, pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int> }
#[repr(C)] pub struct dynevent_cmd { pub seq: c_void, pub r#type: c_int, pub run_command: Option<unsafe extern "C" fn(*mut dynevent_cmd) -> c_int> }
#[repr(C)] pub struct dynevent_arg { pub str_: *mut c_char, pub separator: c_char }
#[repr(C)] pub struct dynevent_arg_pair { pub lhs: *mut c_char, pub operator: c_char, pub rhs: *mut c_char, pub separator: c_char }
pub type dynevent_check_arg_fn_t = unsafe extern "C" fn(*mut c_void) -> c_int;
pub type dynevent_create_fn_t = unsafe extern "C" fn(*mut dynevent_cmd) -> c_int;

pub const TRACE_EVENT_FL_DYNAMIC: u32 = 1 << 1;

pub unsafe extern "C" fn trace_event_dyn_try_get_ref(dyn_call: *mut trace_event_call) -> bool {
    if (*dyn_call).flags & TRACE_EVENT_FL_DYNAMIC == 0 { return false; }
    down_read(&mut trace_event_sem); let mut ret = false;
    // list_for_each_entry(call, &ftrace_events, list)
    let _ = &mut ftrace_events; // external intrusive list traversal
    up_read(&mut trace_event_sem); ret
}
pub unsafe extern "C" fn trace_event_dyn_put_ref(call: *mut trace_event_call) { if (*call).flags & TRACE_EVENT_FL_DYNAMIC == 0 { return; } }
pub unsafe extern "C" fn trace_event_dyn_busy(_call: *mut trace_event_call) -> bool { false }

pub unsafe extern "C" fn dyn_event_register(_ops: *mut dyn_event_operations) -> c_int { 0 }
pub unsafe extern "C" fn dyn_event_release(_raw_command: *const c_char, _kind: *mut dyn_event_operations) -> c_int { -2 }
unsafe extern "C" fn create_dyn_event(_raw_command: *const c_char) -> c_int { -19 }
pub unsafe extern "C" fn dyn_event_create(raw_command: *const c_char, kind: *mut dyn_event_operations) -> c_int { if kind.is_null() { create_dyn_event(raw_command) } else { ((*kind).create.unwrap())(raw_command) } }

pub unsafe extern "C" fn dyn_event_seq_start(_m: *mut seq_file, pos: *mut i64) -> *mut c_void { mutex_lock(&mut event_mutex); seq_list_start(&mut dyn_event_list, *pos) }
pub unsafe extern "C" fn dyn_event_seq_next(_m: *mut seq_file, v: *mut c_void, pos: *mut i64) -> *mut c_void { seq_list_next(v, &mut dyn_event_list, pos) }
pub unsafe extern "C" fn dyn_event_seq_stop(_m: *mut seq_file, _v: *mut c_void) { mutex_unlock(&mut event_mutex); }
unsafe extern "C" fn dyn_event_seq_show(_m: *mut seq_file, _v: *mut c_void) -> c_int { 0 }
static dyn_event_seq_op: seq_operations = seq_operations { start: Some(dyn_event_seq_start), next: Some(dyn_event_seq_next), stop: Some(dyn_event_seq_stop), show: Some(dyn_event_seq_show) };

pub unsafe extern "C" fn dyn_events_release_all(_kind: *mut dyn_event_operations) -> c_int { 0 }
unsafe extern "C" fn dyn_event_open(_inode: *mut inode, _file: *mut file) -> c_int { 0 }
unsafe extern "C" fn dyn_event_write(_file: *mut file, _buffer: *const c_char, _count: usize, _ppos: *mut i64) -> isize { 0 }
static dynamic_events_ops: c_void = c_void;
unsafe extern "C" fn init_dynamic_event() -> c_int { 0 }

pub unsafe extern "C" fn dynevent_arg_add(_cmd: *mut dynevent_cmd, arg: *mut dynevent_arg, check: Option<dynevent_check_arg_fn_t>) -> c_int { if let Some(f) = check { let r = f(arg as *mut c_void); if r != 0 { return r; } } 0 }
pub unsafe extern "C" fn dynevent_arg_pair_add(_cmd: *mut dynevent_cmd, arg: *mut dynevent_arg_pair, check: Option<dynevent_check_arg_fn_t>) -> c_int { if let Some(f) = check { let r = f(arg as *mut c_void); if r != 0 { return r; } } 0 }
pub unsafe extern "C" fn dynevent_str_add(_cmd: *mut dynevent_cmd, _str: *const c_char) -> c_int { 0 }
pub unsafe extern "C" fn dynevent_cmd_init(cmd: *mut dynevent_cmd, buf: *mut c_char, maxlen: c_int, kind: c_int, run: Option<dynevent_create_fn_t>) { memset(cmd as *mut c_void, 0, core::mem::size_of::<dynevent_cmd>()); (*cmd).r#type = kind; (*cmd).run_command = run; let _ = (buf, maxlen); }
pub unsafe extern "C" fn dynevent_arg_init(arg: *mut dynevent_arg, separator: c_char) { memset(arg as *mut c_void, 0, core::mem::size_of::<dynevent_arg>()); (*arg).separator = if separator == 0 { b' ' as c_char } else { separator }; }
pub unsafe extern "C" fn dynevent_arg_pair_init(arg: *mut dynevent_arg_pair, operator: c_char, separator: c_char) { memset(arg as *mut c_void, 0, core::mem::size_of::<dynevent_arg_pair>()); (*arg).operator = if operator == 0 { b' ' as c_char } else { operator }; (*arg).separator = if separator == 0 { b' ' as c_char } else { separator }; }
pub unsafe extern "C" fn dynevent_create(cmd: *mut dynevent_cmd) -> c_int { ((*cmd).run_command.unwrap())(cmd) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
