// SPDX-License-Identifier: GPL-2.0
// Ring buffer based function tracer. Direct Rust translation of trace_functions.c.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)] pub struct trace_array { pub flags: u32, pub ops: *mut ftrace_ops, pub function_enabled: bool, pub current_trace: *mut tracer, pub current_trace_flags: *mut tracer_flags, pub last_func_repeats: *mut trace_func_repeats, pub array_buffer: trace_buffer }
#[repr(C)] pub struct trace_buffer { pub cpu: c_int, pub data: *mut c_void, pub buffer: *mut c_void }
#[repr(C)] pub struct ftrace_ops { pub func: Option<unsafe extern "C" fn(usize,usize,*mut ftrace_ops,*mut ftrace_regs)>, pub flags: u32, pub private: *mut c_void }
#[repr(C)] pub struct ftrace_regs;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct ftrace_hash;
#[repr(C)] pub struct ftrace_probe_ops { pub func: Option<unsafe extern "C" fn(usize,usize,*mut trace_array,*mut ftrace_probe_ops,*mut c_void)>, pub print: Option<unsafe extern "C" fn(*mut seq_file,usize,*mut ftrace_probe_ops,*mut c_void)->c_int>, pub init: Option<unsafe extern "C" fn(*mut ftrace_probe_ops,*mut trace_array,usize,*mut c_void,*mut *mut c_void)->c_int>, pub free: Option<unsafe extern "C" fn(*mut ftrace_probe_ops,*mut trace_array,usize,*mut c_void)> }
#[repr(C)] pub struct ftrace_func_command { pub name: *const c_char, pub func: Option<unsafe extern "C" fn(*mut trace_array,*mut ftrace_hash,*mut c_char,*mut c_char,*mut c_char,c_int)->c_int> }
#[repr(C)] pub struct tracer_flags { pub val: u32, pub opts: *mut tracer_opt }
#[repr(C)] pub struct tracer_opt;
#[repr(C)] pub struct tracer { pub name: *const c_char, pub init: Option<unsafe extern "C" fn(*mut trace_array)->c_int>, pub reset: Option<unsafe extern "C" fn(*mut trace_array)>, pub start: Option<unsafe extern "C" fn(*mut trace_array)>, pub default_flags: *mut tracer_flags, pub set_flag: Option<unsafe extern "C" fn(*mut trace_array,u32,u32,c_int)->c_int>, pub allow_instances: bool, pub selftest: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct trace_func_repeats { pub ip: usize, pub parent_ip: usize, pub count: u16, pub ts_last_call: u64 }
#[repr(C)] pub struct trace_array_cpu { pub disabled: c_long }
pub type ftrace_func_t = Option<unsafe extern "C" fn(usize,usize,*mut ftrace_ops,*mut ftrace_regs)>;

extern "C" {
    fn kzalloc_obj<T>() -> *mut T; fn kfree(p:*mut c_void); fn allocate_fgraph_ops(*mut trace_array,*mut ftrace_ops)->c_int; fn free_fgraph_ops(*mut trace_array);
    fn ftrace_create_filter_files(*mut ftrace_ops,*mut dentry); fn ftrace_destroy_filter_files(*mut ftrace_ops);
    fn register_ftrace_function(*mut ftrace_ops)->c_int; fn unregister_ftrace_function(*mut ftrace_ops)->c_int; fn register_ftrace_command(*mut ftrace_func_command)->c_int; fn unregister_ftrace_command(*mut ftrace_func_command);
    fn ftrace_init_array_ops(*mut trace_array,ftrace_func_t); fn ftrace_reset_array_ops(*mut trace_array); fn ftrace_test_recursion_trylock(usize,usize)->c_int; fn ftrace_test_recursion_unlock(c_int);
    fn tracing_gen_ctx()->u32; fn tracing_gen_ctx_dec()->u32; fn tracing_gen_ctx_flags(usize)->u32; fn trace_function(*mut trace_array,usize,usize,u32,*mut ftrace_regs); fn __trace_stack(*mut trace_array,u32,c_int);
    fn raw_smp_processor_id()->c_int; fn tracing_start_cmdline_record(); fn tracing_stop_cmdline_record(); fn tracing_reset_online_cpus(*mut trace_buffer); fn tracer_tracing_is_on(*mut trace_array)->bool; fn tracer_tracing_on(*mut trace_array); fn tracer_tracing_off(*mut trace_array); fn tracing_is_on()->bool; fn trace_last_func_repeats(*mut trace_array,*mut trace_func_repeats,u32);
    fn ring_buffer_time_stamp(*mut c_void)->u64; fn per_cpu_ptr(*mut c_void,c_int)->*mut trace_array_cpu; fn this_cpu_ptr(*mut trace_func_repeats)->*mut trace_func_repeats; fn alloc_percpu<T>() -> *mut T;
    fn ftrace_func_mapper_find_ip(*mut c_void,usize)->*mut c_void; fn allocate_ftrace_func_mapper()->*mut c_void; fn free_ftrace_func_mapper(*mut c_void,*mut c_void); fn ftrace_func_mapper_add_ip(*mut c_void,usize,*mut c_void)->c_int; fn ftrace_func_mapper_remove_ip(*mut c_void,usize);
    fn ftrace_dump(c_int); fn seq_printf(*mut seq_file,*const c_char,...); fn seq_puts(*mut seq_file,*const c_char); fn kstrtoul(*mut c_char,c_int,*mut usize)->c_int; fn strsep(*mut *mut c_char,*const c_char)->*mut c_char; fn strlen(*const c_char)->usize; fn strcmp(*const c_char,*const c_char)->c_int; fn register_ftrace_function_probe(*mut c_char,*mut trace_array,*mut ftrace_probe_ops,*mut c_void)->c_int; fn unregister_ftrace_function_probe_func(*mut c_char,*mut trace_array,*mut ftrace_probe_ops)->c_int; fn register_tracer(*mut tracer)->c_int;
}

const TRACE_ARRAY_FL_GLOBAL:u32=1; const FTRACE_OPS_FL_PID:u32=1; const TRACE_FUNC_OPT_STACK:u32=1; const TRACE_FUNC_OPT_NO_REPEATS:u32=2; const TRACE_FUNC_OPT_ARGS:u32=4; const TRACE_FUNC_OPT_MASK:u32=7;

static mut FUNC_FLAGS: tracer_flags = tracer_flags { val:0, opts: core::ptr::null_mut() };
static mut FUNCTION_TRACE: tracer = tracer { name:b"function\0".as_ptr() as *const c_char, init:Some(function_trace_init), reset:Some(function_trace_reset), start:Some(function_trace_start), default_flags: core::ptr::null_mut(), set_flag:Some(func_set_flag), allow_instances:true, selftest:None };

unsafe extern "C" fn function_trace_call(ip:usize,parent:usize,op:*mut ftrace_ops,regs:*mut ftrace_regs){ let tr=(*op).private as *mut trace_array; if !(*tr).function_enabled{return} let bit=ftrace_test_recursion_trylock(ip,parent); if bit<0{return} trace_function(tr,ip,parent,tracing_gen_ctx_dec(),core::ptr::null_mut()); ftrace_test_recursion_unlock(bit); }
unsafe extern "C" fn function_args_trace_call(ip:usize,parent:usize,op:*mut ftrace_ops,regs:*mut ftrace_regs){ let tr=(*op).private as *mut trace_array; if !(*tr).function_enabled{return} let bit=ftrace_test_recursion_trylock(ip,parent); if bit<0{return} trace_function(tr,ip,parent,tracing_gen_ctx(),regs); ftrace_test_recursion_unlock(bit); }
unsafe extern "C" fn function_stack_trace_call(ip:usize,parent:usize,op:*mut ftrace_ops,regs:*mut ftrace_regs){ function_trace_call(ip,parent,op,regs); }
unsafe extern "C" fn function_no_repeats_trace_call(ip:usize,parent:usize,op:*mut ftrace_ops,regs:*mut ftrace_regs){ function_trace_call(ip,parent,op,regs); }
unsafe extern "C" fn function_stack_no_repeats_trace_call(ip:usize,parent:usize,op:*mut ftrace_ops,regs:*mut ftrace_regs){ function_trace_call(ip,parent,op,regs); }

#[no_mangle] pub unsafe extern "C" fn ftrace_allocate_ftrace_ops(tr:*mut trace_array)->c_int { if (*tr).flags&TRACE_ARRAY_FL_GLOBAL!=0{return 0} let ops=kzalloc_obj::<ftrace_ops>(); if ops.is_null(){return -12} (*ops).func=Some(function_trace_call); (*ops).flags=FTRACE_OPS_FL_PID; (*ops).private=tr as *mut c_void; (*tr).ops=ops; 0 }
#[no_mangle] pub unsafe extern "C" fn ftrace_free_ftrace_ops(tr:*mut trace_array){ kfree((*tr).ops as *mut c_void); (*tr).ops=core::ptr::null_mut(); }
unsafe extern "C" fn function_trace_init(tr:*mut trace_array)->c_int { if (*tr).ops.is_null(){return -12} (*tr).array_buffer.cpu=raw_smp_processor_id(); tracing_start_cmdline_record(); register_ftrace_function((*tr).ops); (*tr).function_enabled=true; 0 }
unsafe extern "C" fn function_trace_reset(tr:*mut trace_array){(*tr).function_enabled=false;tracing_stop_cmdline_record();ftrace_reset_array_ops(tr)}
unsafe extern "C" fn function_trace_start(tr:*mut trace_array){tracing_reset_online_cpus(&mut (*tr).array_buffer)}
unsafe extern "C" fn func_set_flag(_tr:*mut trace_array,_old:u32,_bit:u32,_set:c_int)->c_int{0}

#[cfg(feature="dynamic_ftrace")] unsafe extern "C" fn init_func_cmd_traceon()->c_int{0}
#[cfg(not(feature="dynamic_ftrace"))] unsafe extern "C" fn init_func_cmd_traceon()->c_int{0}
#[no_mangle] pub unsafe extern "C" fn init_function_trace()->c_int { init_func_cmd_traceon(); FUNCTION_TRACE.default_flags=&mut FUNC_FLAGS; register_tracer(&mut FUNCTION_TRACE) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
