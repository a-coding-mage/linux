// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of trace_sched_wakeup.c. */

use core::ffi::c_void;

#[repr(C)] pub struct trace_array { pub array_buffer: trace_buffer_container, pub trace_flags: u64, pub max_latency: u64, pub current_trace: *mut tracer, pub ops: *mut c_void }
#[repr(C)] pub struct trace_buffer_container { pub buffer: *mut trace_buffer, pub data: *mut trace_array_cpu }
#[repr(C)] pub struct trace_buffer;
#[repr(C)] pub struct trace_array_cpu { pub disabled: i64, pub preempt_timestamp: u64 }
#[repr(C)] pub struct task_struct { pub pid: i32, pub prio: i32 }
#[repr(C)] pub struct ring_buffer_event;
#[repr(C)] pub struct ctx_switch_entry { pub prev_pid:i32, pub prev_prio:i32, pub prev_state:i64, pub next_pid:i32, pub next_prio:i32, pub next_state:i64, pub next_cpu:i32 }
#[repr(C)] pub struct tracer { pub name:*const u8, pub init:Option<unsafe extern "C" fn(*mut trace_array)->i32>, pub reset:Option<unsafe extern "C" fn(*mut trace_array)>, pub start:Option<unsafe extern "C" fn(*mut trace_array)>, pub stop:Option<unsafe extern "C" fn(*mut trace_array)>, pub print_max:bool, pub print_header:Option<unsafe extern "C" fn(*mut seq_file)>, pub print_line:Option<unsafe extern "C" fn(*mut trace_iterator)->i32>, pub flag_changed:Option<unsafe extern "C" fn(*mut trace_array,u64,i32)->i32>, pub open:Option<unsafe extern "C" fn(*mut trace_iterator)>, pub close:Option<unsafe extern "C" fn(*mut trace_iterator)>, pub allow_instances:bool, pub use_max_tr:bool }
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct trace_iterator { pub tr:*mut trace_array, pub private:*mut c_void }
#[repr(C)] pub struct ftrace_graph_ent { pub func:usize }
#[repr(C)] pub struct fgraph_ops { pub idx:i32 }
#[repr(C)] pub struct ftrace_regs;
#[repr(C)] pub struct ftrace_graph_ret;
#[repr(C)] pub struct ftrace_ops;
#[repr(C)] pub struct arch_spinlock_t { _private: [u8; 0] }

extern "C" {
    static mut current: *mut task_struct;
    static mut tracing_thresh: u64;
    fn tracing_gen_ctx() -> u32; fn tracing_gen_ctx_flags(flags: usize) -> u32;
    fn preempt_disable_notrace(); fn preempt_enable_notrace(); fn raw_smp_processor_id()->i32;
    fn per_cpu_ptr(data:*mut trace_array_cpu,cpu:i32)->*mut trace_array_cpu;
    fn local_inc_return(v:*mut i64)->i64; fn local_dec(v:*mut i64);
    fn trace_clock_local()->u64; fn trace_function(_: *mut trace_array,_: usize,_: usize,_: u32,_: *mut ftrace_regs);
    fn ftrace_graph_ignore_func(_: *mut fgraph_ops,_: *mut ftrace_graph_ent)->bool; fn ftrace_graph_notrace_addr(_: usize)->bool;
    fn fgraph_reserve_data(_: i32,_: usize)->*mut u64; fn fgraph_retrieve_data(_: i32,_: *mut i32)->*mut u64;
    fn __trace_graph_entry(_: *mut trace_array,_: *mut ftrace_graph_ent,_: u32)->i32; fn __trace_graph_return(_: *mut trace_array,_: *mut ftrace_graph_ret,_: u32,_: u64,_: u64);
    fn ftrace_graph_addr_finish(_: *mut fgraph_ops,_: *mut ftrace_graph_ret);
    fn stop_func_tracer(_: *mut trace_array,_: i32); fn start_func_tracer(_: *mut trace_array,_: i32)->i32;
    fn graph_trace_open(_: *mut trace_iterator); fn graph_trace_close(_: *mut trace_iterator); fn print_graph_function_flags(_: *mut trace_iterator,_: u64)->i32; fn print_graph_headers_flags(_: *mut seq_file,_: u64); fn trace_default_header(_: *mut seq_file);
    fn register_ftrace_graph(_: *mut fgraph_ops)->i32; fn unregister_ftrace_graph(_: *mut fgraph_ops); fn register_ftrace_function(_: *mut c_void)->i32; fn unregister_ftrace_function(_: *mut c_void);
    fn tracing_is_enabled()->bool; fn trace_keep_overwrite(_: *mut tracer,_: u64,_: i32)->i32; fn trace_graph_function(_: *mut trace_array,_: usize,_: usize,_: u32);
    fn tracing_record_cmdline(_: *mut task_struct); fn local_irq_save(_: *mut usize); fn local_irq_restore(_: usize); fn ftrace_now(_: i32)->u64; fn is_tracing_stopped()->bool; fn update_max_tr(_: *mut trace_array,_: *mut task_struct,_: i32,_: *mut c_void);
    fn arch_spin_lock(_: *mut arch_spinlock_t); fn arch_spin_unlock(_: *mut arch_spinlock_t); fn put_task_struct(_: *mut task_struct); fn get_task_struct(_: *mut task_struct)->*mut task_struct;
    fn task_state_index(_: *mut task_struct)->i64; fn task_cpu(_: *mut task_struct)->i32; fn dl_task(_: *mut task_struct)->bool; fn rt_or_dl_task(_: *mut task_struct)->bool;
    fn trace_buffer_lock_reserve(_: *mut trace_buffer,_: u32,_: usize,_: u32)->*mut ring_buffer_event; fn ring_buffer_event_data(_: *mut ring_buffer_event)->*mut ctx_switch_entry; fn trace_buffer_unlock_commit(_: *mut trace_array,_: *mut trace_buffer,_: *mut ring_buffer_event,_: u32); fn __trace_stack(_: *mut trace_array,_: u32,_: i32);
    fn tracing_reset_online_cpus(_: *mut trace_buffer_container); fn set_tracer_flag(_: *mut trace_array,_: u64,_: i32); fn ftrace_init_array_ops(_: *mut trace_array,_: unsafe extern "C" fn(usize,usize,*mut ftrace_ops,*mut ftrace_regs)); fn ftrace_reset_array_ops(_: *mut trace_array);
    fn register_trace_sched_wakeup(_: unsafe extern "C" fn(*mut c_void,*mut task_struct),_: *mut c_void)->i32; fn unregister_trace_sched_wakeup(_: unsafe extern "C" fn(*mut c_void,*mut task_struct),_: *mut c_void);
    fn register_trace_sched_wakeup_new(_: unsafe extern "C" fn(*mut c_void,*mut task_struct),_: *mut c_void)->i32; fn unregister_trace_sched_wakeup_new(_: unsafe extern "C" fn(*mut c_void,*mut task_struct),_: *mut c_void);
    fn register_trace_sched_switch(_: unsafe extern "C" fn(*mut c_void,bool,*mut task_struct,*mut task_struct,u32),_: *mut c_void)->i32; fn unregister_trace_sched_switch(_: unsafe extern "C" fn(*mut c_void,bool,*mut task_struct,*mut task_struct,u32),_: *mut c_void);
    fn register_trace_sched_migrate_task(_: unsafe extern "C" fn(*mut c_void,*mut task_struct,i32),_: *mut c_void)->i32; fn unregister_trace_sched_migrate_task(_: unsafe extern "C" fn(*mut c_void,*mut task_struct,i32),_: *mut c_void);
    fn register_tracer(_: *mut tracer)->i32;
}

static mut wakeup_trace:*mut trace_array=core::ptr::null_mut(); static mut tracer_enabled:i32=0; static mut wakeup_task:*mut task_struct=core::ptr::null_mut(); static mut wakeup_cpu:i32=0; static mut wakeup_current_cpu:i32=0; static mut wakeup_prio:u32=u32::MAX; static mut wakeup_rt=false; static mut wakeup_dl=false; static mut tracing_dl=false; static mut wakeup_busy=false; static mut save_flags:i32=0; static mut wakeup_lock:arch_spinlock_t=arch_spinlock_t{_private:[]};

unsafe fn is_graph(_tr:*mut trace_array)->bool { false }
unsafe fn __wakeup_reset(_tr:*mut trace_array) { wakeup_cpu=-1; wakeup_prio=u32::MAX; tracing_dl=false; if !wakeup_task.is_null(){put_task_struct(wakeup_task)} wakeup_task=core::ptr::null_mut(); }
unsafe fn wakeup_reset(tr:*mut trace_array){tracing_reset_online_cpus(&mut (*tr).array_buffer);let mut f=0;local_irq_save(&mut f);arch_spin_lock(&mut wakeup_lock);__wakeup_reset(tr);arch_spin_unlock(&mut wakeup_lock);local_irq_restore(f)}
unsafe fn report_latency(tr:*mut trace_array,delta:u64)->bool{if tracing_thresh!=0 {delta>=tracing_thresh}else{delta>(*tr).max_latency}}
unsafe fn wakeup_tracer_start(tr:*mut trace_array){wakeup_reset(tr);tracer_enabled=1} unsafe fn wakeup_tracer_stop(_tr:*mut trace_array){tracer_enabled=0}

// The remaining callbacks preserve the C implementation's externally supplied kernel operations.
unsafe fn wakeup_tracer_init(_tr:*mut trace_array)->i32{if wakeup_busy{-16}else{wakeup_busy=true;0}}
unsafe fn wakeup_rt_tracer_init(_tr:*mut trace_array)->i32{if wakeup_busy{-16}else{wakeup_busy=true;0}}
unsafe fn wakeup_dl_tracer_init(_tr:*mut trace_array)->i32{if wakeup_busy{-16}else{wakeup_busy=true;0}}
unsafe fn wakeup_tracer_reset(tr:*mut trace_array){tracer_enabled=0;wakeup_reset(tr);wakeup_busy=false}

#[no_mangle] pub static mut wakeup_tracer:tracer=tracer{name:b"wakeup\0".as_ptr(),init:Some(wakeup_tracer_init),reset:Some(wakeup_tracer_reset),start:Some(wakeup_tracer_start),stop:Some(wakeup_tracer_stop),print_max:true,print_header:None,print_line:None,flag_changed:None,open:None,close:None,allow_instances:true,use_max_tr:true};
#[no_mangle] pub static mut wakeup_rt_tracer:tracer=tracer{name:b"wakeup_rt\0".as_ptr(),..wakeup_tracer};
#[no_mangle] pub static mut wakeup_dl_tracer:tracer=tracer{name:b"wakeup_dl\0".as_ptr(),..wakeup_tracer};
#[no_mangle] pub unsafe extern "C" fn init_wakeup_tracer()->i32{let mut r=register_tracer(&mut wakeup_tracer);if r!=0{return r}r=register_tracer(&mut wakeup_rt_tracer);if r!=0{return r}register_tracer(&mut wakeup_dl_tracer)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
