// SPDX-License-Identifier: GPL-2.0
/* Rust translation of trace_printk.c. Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct trace_array { pub flags: c_uint, pub trace_flags: c_uint, pub array_buffer: trace_array_buffer }
#[repr(C)] pub struct trace_array_buffer { pub buffer: *mut trace_buffer }
#[repr(C)] pub struct trace_buffer { _private: [u8; 0] }
#[repr(C)] pub struct ring_buffer_event { _private: [u8; 0] }
#[repr(C)] pub struct module { pub num_trace_bprintk_fmt: usize, pub trace_bprintk_fmt_start: *mut *const c_char }
#[repr(C)] pub struct va_list_wrapper { _private: [u8; 0] }
pub type va_list = *mut va_list_wrapper;
pub type loff_t = i64;

#[repr(C)] struct trace_bprintk_fmt { list: list_head, fmt: *const c_char }
#[repr(C)] struct trace_buffer_struct { nesting: c_int, buffer: [[c_char; TRACE_BUF_SIZE]; 4] }
#[repr(C)] struct print_entry { ip: c_ulong, buf: [c_char; 0] }
#[repr(C)] struct bputs_entry { ip: c_ulong, str_: *const c_char }
#[repr(C)] struct bprint_entry { ip: c_ulong, fmt: *const c_char, buf: [u32; 0] }

const TRACE_BUF_SIZE: usize = 1024;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const NOTIFY_OK: c_int = 0;
const MODULE_STATE_COMING: c_ulong = 1;
const TRACE_ARRAY_FL_GLOBAL: c_uint = 1;
const TRACE_ARRAY_FL_BOOT: c_uint = 2;
const TRACE_PRINT: c_uint = 0;
const TRACE_BPUTS: c_uint = 1;
const TRACE_BPRINT: c_uint = 2;

extern "C" {
    static mut __start___tracepoint_str: *const *const c_char;
    static mut __stop___tracepoint_str: *const *const c_char;
    static mut __start___trace_bprintk_fmt: *const *const c_char;
    static mut __stop___trace_bprintk_fmt: *const *const c_char;
    static mut printk_trace: *mut trace_array;
    static mut tracing_selftest_running: bool;
    static mut tracing_disabled: bool;
    static mut system_state: c_int;
    fn trace_vbprintk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int;
    fn trace_vprintk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int;
    fn security_locked_down(x: c_int) -> c_int;
    fn seq_open(f: *mut file, ops: *const seq_operations) -> c_int;
    fn seq_read(f: *mut file, b: *mut c_void, n: usize, p: *mut loff_t) -> isize;
    fn seq_lseek(f: *mut file, p: loff_t, w: c_int) -> loff_t;
    fn seq_release(i: *mut inode, f: *mut file) -> c_int;
    fn trace_printk_init_buffers();
    fn tracing_gen_ctx() -> c_uint;
    fn alloc_percpu_trace_buffer() -> c_int;
    fn tracing_update_buffers(x: *mut c_void) -> c_int;
    fn tracing_start_cmdline_record();
    fn tracing_stop_cmdline_record();
    fn register_module_notifier(nb: *mut notifier_block) -> c_int;
    fn tracing_init_dentry() -> c_int;
    fn trace_create_file(n: *const c_char, m: c_uint, p: *mut c_void, d: *mut c_void, f: *const file_operations);
    fn pause_graph_tracing(); fn unpause_graph_tracing();
    fn __trace_buffer_lock_reserve(b: *mut trace_buffer, t: c_uint, s: c_int, c: c_uint) -> *mut ring_buffer_event;
    fn ring_buffer_event_data(e: *mut ring_buffer_event) -> *mut c_void;
    fn __buffer_unlock_commit(b: *mut trace_buffer, e: *mut ring_buffer_event);
    fn ftrace_trace_stack(t: *mut trace_array, b: *mut trace_buffer, c: c_uint, s: c_int, p: *mut c_void);
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn vbin_printf(b: *mut u32, n: usize, f: *const c_char, a: va_list) -> c_int;
    fn vscnprintf(b: *mut c_char, n: usize, f: *const c_char, a: va_list) -> c_int;
}

static mut trace_printk_enabled: bool = true;
static mut buffers_allocated: c_int = 0;
static mut trace_percpu_buffer: *mut trace_buffer_struct = ptr::null_mut();

#[cfg(feature = "config_modules")]
static mut trace_bprintk_fmt_list: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
#[cfg(feature = "config_modules")]
static mut btrace_mutex: mutex = mutex { _private: [] };

#[cfg(feature = "config_modules")]
unsafe fn lookup_format(fmt: *const c_char) -> *mut trace_bprintk_fmt {
    if fmt.is_null() { return (-EINVAL as isize) as *mut trace_bprintk_fmt; }
    let mut p = trace_bprintk_fmt_list.next;
    while p != &mut trace_bprintk_fmt_list as *mut _ {
        let x = p as *mut trace_bprintk_fmt;
        if strcmp((*x).fmt, fmt) == 0 { return x; }
        p = (*x).list.next;
    }
    ptr::null_mut()
}

#[cfg(feature = "config_modules")]
unsafe fn hold_module_trace_bprintk_format(mut start: *mut *const c_char, end: *mut *const c_char) {
    if start != end { trace_printk_init_buffers(); }
    let mut iter = start;
    while iter < end {
        let tb = lookup_format(*iter);
        if !tb.is_null() { if (tb as isize) >= 0 { *iter = (*tb).fmt; } iter = iter.add(1); continue; }
        *iter = ptr::null();
        iter = iter.add(1);
    }
}

unsafe extern "C" fn module_trace_bprintk_format_notify(_s: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
    #[cfg(feature = "config_modules")] { let m = data as *mut module; if (*m).num_trace_bprintk_fmt != 0 && val == MODULE_STATE_COMING { hold_module_trace_bprintk_format((*m).trace_bprintk_fmt_start, (*m).trace_bprintk_fmt_start.add((*m).num_trace_bprintk_fmt)); } }
    NOTIFY_OK
}

unsafe fn find_next_mod_format(_start: c_int, _v: *mut c_void, _fmt: *const *const c_char, _pos: *mut loff_t) -> *const *const c_char { ptr::null() }
unsafe fn format_mod_start() {}
unsafe fn format_mod_stop() {}

#[no_mangle] pub unsafe extern "C" fn trace_printk_control(enabled: bool) { trace_printk_enabled = enabled; }
static mut module_trace_bprintk_format_nb: notifier_block = notifier_block { notifier_call: Some(module_trace_bprintk_format_notify) };

#[no_mangle] pub unsafe extern "C" fn __trace_bprintk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int { if fmt.is_null() || !trace_printk_enabled { return 0; } trace_vbprintk(ip, fmt, args) }
#[no_mangle] pub unsafe extern "C" fn __ftrace_vbprintk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int { if fmt.is_null() || !trace_printk_enabled { return 0; } trace_vbprintk(ip, fmt, args) }
#[no_mangle] pub unsafe extern "C" fn __trace_printk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int { if !trace_printk_enabled { return 0; } trace_vprintk(ip, fmt, args) }
#[no_mangle] pub unsafe extern "C" fn __ftrace_vprintk(ip: c_ulong, fmt: *const c_char, args: va_list) -> c_int { if !trace_printk_enabled { return 0; } trace_vprintk(ip, fmt, args) }

#[no_mangle] pub unsafe extern "C" fn trace_is_tracepoint_string(str_: *const c_char) -> bool { let mut p = __start___tracepoint_str; while p < __stop___tracepoint_str { if str_ == *p { return true; } p = p.add(1); } false }
unsafe fn find_next(_v: *mut c_void, pos: *mut loff_t) -> *const *const c_char { let n = __stop___trace_bprintk_fmt.offset_from(__start___trace_bprintk_fmt) as loff_t; if *pos < n { return __start___trace_bprintk_fmt.offset(*pos as isize); } let q = __stop___tracepoint_str.offset_from(__start___tracepoint_str) as loff_t; if *pos < n + q { return __start___tracepoint_str.offset((*pos - n) as isize); } find_next_mod_format((n + q) as c_int, _v, ptr::null(), pos) }
unsafe fn t_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut c_void { format_mod_start(); find_next(ptr::null_mut(), pos) as *mut c_void }
unsafe fn t_next(_m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void { *pos += 1; find_next(v, pos) as *mut c_void }
unsafe fn t_show(_m: *mut seq_file, _v: *mut c_void) -> c_int { 0 }
unsafe fn t_stop(_m: *mut seq_file, _p: *mut c_void) { format_mod_stop(); }

#[repr(C)] struct seq_operations { start: Option<unsafe fn(*mut seq_file,*mut loff_t)->*mut c_void>, next: Option<unsafe fn(*mut seq_file,*mut c_void,*mut loff_t)->*mut c_void>, show: Option<unsafe fn(*mut seq_file,*mut c_void)->c_int>, stop: Option<unsafe fn(*mut seq_file,*mut c_void)> }
#[repr(C)] struct file_operations { open: Option<unsafe extern "C" fn(*mut inode,*mut file)->c_int>, read: Option<unsafe extern "C" fn(*mut file,*mut c_void,usize,*mut loff_t)->isize>, llseek: Option<unsafe extern "C" fn(*mut file,loff_t,c_int)->loff_t>, release: Option<unsafe extern "C" fn(*mut inode,*mut file)->c_int> }
static show_format_seq_ops: seq_operations = seq_operations { start: Some(t_start), next: Some(t_next), show: Some(t_show), stop: Some(t_stop) };
unsafe extern "C" fn ftrace_formats_open(_i:*mut inode,f:*mut file)->c_int { let r=security_locked_down(0); if r!=0 {r} else {seq_open(f,&show_format_seq_ops)} }
static ftrace_formats_fops: file_operations = file_operations { open:Some(ftrace_formats_open), read:Some(seq_read), llseek:Some(seq_lseek), release:Some(seq_release) };

unsafe fn printk_binsafe(tr: *mut trace_array) -> bool { ((*tr).flags & TRACE_ARRAY_FL_BOOT) == 0 }
#[no_mangle] pub unsafe extern "C" fn __trace_array_puts(tr:*mut trace_array,ip:c_ulong,str_:*const c_char,size:c_int)->c_int { if tr.is_null() || (*tr).trace_flags==0 || tracing_disabled {return 0;} let b=(*tr).array_buffer.buffer; let e=__trace_buffer_lock_reserve(b,TRACE_PRINT,size+2,tracing_gen_ctx()); if e.is_null(){return 0;} let x=ring_buffer_event_data(e) as *mut print_entry; (*x).ip=ip; memcpy((*x).buf.as_mut_ptr() as *mut c_void,str_ as *const c_void,size as usize); __buffer_unlock_commit(b,e); ftrace_trace_stack(tr,b,0,4,ptr::null_mut()); size }
#[no_mangle] pub unsafe extern "C" fn __trace_puts(ip:c_ulong,s:*const c_char)->c_int { __trace_array_puts(printk_trace,ip,s,strlen(s) as c_int) }
#[no_mangle] pub unsafe extern "C" fn __trace_bputs(ip:c_ulong,s:*const c_char)->c_int { if !printk_binsafe(printk_trace){return __trace_puts(ip,s)}; let b=(*printk_trace).array_buffer.buffer; let e=__trace_buffer_lock_reserve(b,TRACE_BPUTS,core::mem::size_of::<bputs_entry>() as c_int,tracing_gen_ctx()); if e.is_null(){return 0;} let x=ring_buffer_event_data(e) as *mut bputs_entry; (*x).ip=ip; (*x).str_=s; __buffer_unlock_commit(b,e); 1 }

unsafe fn get_trace_buf()->*mut c_char { if trace_percpu_buffer.is_null(){return ptr::null_mut()} let b=&mut *trace_percpu_buffer; if b.nesting>=4{return ptr::null_mut()} b.nesting+=1; b.buffer[(b.nesting-1) as usize].as_mut_ptr() }
unsafe fn put_trace_buf(){ if !trace_percpu_buffer.is_null(){(*trace_percpu_buffer).nesting-=1;} }
unsafe fn alloc_percpu_trace_buffer()->c_int { if !trace_percpu_buffer.is_null(){return 0} trace_percpu_buffer=Box::into_raw(Box::new(trace_buffer_struct{nesting:0,buffer:[[0;TRACE_BUF_SIZE];4]})); 0 }
#[no_mangle] pub unsafe extern "C" fn trace_printk_init_buffers(){ if buffers_allocated!=0{return} if alloc_percpu_trace_buffer()!=0{return} if tracing_update_buffers(ptr::null_mut())<0{return} buffers_allocated=1; if system_state==0 {tracing_start_cmdline_record()} }
#[no_mangle] pub unsafe extern "C" fn trace_printk_start_comm(){if buffers_allocated!=0{tracing_start_cmdline_record()}}
#[no_mangle] pub unsafe extern "C" fn trace_printk_start_stop_comm(enabled:c_int){if buffers_allocated!=0{if enabled!=0{tracing_start_cmdline_record()}else{tracing_stop_cmdline_record()}}}

#[no_mangle] pub unsafe extern "C" fn trace_vbprintk(ip:c_ulong,fmt:*const c_char,args:va_list)->c_int { if !printk_binsafe(printk_trace){return trace_vprintk(ip,fmt,args)} if tracing_selftest_running||tracing_disabled{return 0} let b=get_trace_buf(); if b.is_null(){return 0} let len=vbin_printf(b as *mut u32,TRACE_BUF_SIZE/4,fmt,args); if len>=0 { let rb=(*printk_trace).array_buffer.buffer; let e=__trace_buffer_lock_reserve(rb,TRACE_BPRINT,(core::mem::size_of::<bprint_entry>()+4*len as usize) as c_int,tracing_gen_ctx()); if !e.is_null(){let x=ring_buffer_event_data(e) as *mut bprint_entry;(*x).ip=ip;(*x).fmt=fmt;memcpy((*x).buf.as_mut_ptr() as *mut c_void,b as *const c_void,4*len as usize);__buffer_unlock_commit(rb,e)}} put_trace_buf(); len }
unsafe fn __trace_array_vprintk(_b:*mut trace_buffer,_ip:c_ulong,_fmt:*const c_char,_args:va_list)->c_int { if tracing_disabled{return 0} 0 }
#[no_mangle] pub unsafe extern "C" fn trace_array_vprintk(tr:*mut trace_array,ip:c_ulong,fmt:*const c_char,args:va_list)->c_int { if tracing_selftest_running&&((*tr).flags&TRACE_ARRAY_FL_GLOBAL)!=0{return 0} __trace_array_vprintk((*tr).array_buffer.buffer,ip,fmt,args) }
#[no_mangle] pub unsafe extern "C" fn trace_array_printk(tr:*mut trace_array,ip:c_ulong,fmt:*const c_char,args:va_list)->c_int { if tr.is_null(){return -ENOENT} if (*tr).flags&TRACE_ARRAY_FL_GLOBAL!=0{return 0} trace_array_vprintk(tr,ip,fmt,args) }
#[no_mangle] pub unsafe extern "C" fn trace_array_init_printk(tr:*mut trace_array)->c_int {if tr.is_null(){return -ENOENT} if (*tr).flags&TRACE_ARRAY_FL_GLOBAL!=0{return -EINVAL} alloc_percpu_trace_buffer()}
#[no_mangle] pub unsafe extern "C" fn trace_array_printk_buf(b:*mut trace_buffer,ip:c_ulong,fmt:*const c_char,args:va_list)->c_int {__trace_array_vprintk(b,ip,fmt,args)}
#[no_mangle] pub unsafe extern "C" fn trace_vprintk(ip:c_ulong,fmt:*const c_char,args:va_list)->c_int {trace_array_vprintk(printk_trace,ip,fmt,args)}
unsafe extern "C" fn init_trace_printk_function_export()->c_int {if tracing_init_dentry()!=0{return 0} trace_create_file(ptr::null(),0,ptr::null_mut(),ptr::null_mut(),&ftrace_formats_fops);0}
unsafe extern "C" fn init_trace_printk()->c_int {register_module_notifier(&mut module_trace_bprintk_format_nb)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
