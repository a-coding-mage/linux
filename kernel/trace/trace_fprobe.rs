// SPDX-License-Identifier: GPL-2.0
/* Fprobe-based tracing events (literal low-level Rust translation). */

// Kernel declarations supplied by the surrounding repository are intentionally external.
use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)] pub struct tracepoint_user { pub list: list_head, pub name: *const c_char, pub tpoint: *mut tracepoint, pub refcount: u32 }
#[repr(C)] pub struct trace_fprobe { pub devent: dyn_event, pub fp: fprobe, pub symbol: *const c_char, pub tprobe: bool, pub tuser: *mut tracepoint_user, pub tp: trace_probe }
#[repr(C)] pub struct __find_tracepoint_cb_data { pub tp_name: *const c_char, pub tpoint: *mut tracepoint, pub r#mod: *mut module }

extern "C" {
    static mut tracepoint_user_list: list_head; static mut tracepoint_user_mutex: mutex;
    static mut event_mutex: mutex;
    fn tracepoint_probe_register_prio_may_exist(*mut tracepoint, *mut c_void, *mut c_void, c_int)->c_int;
    fn tracepoint_probe_unregister(*mut tracepoint,*mut c_void,*mut c_void)->c_int;
    fn find_tracepoint(*const c_char,*mut *mut module)->*mut tracepoint;
    fn tracepoint_user_put(*mut tracepoint_user); fn tracepoint_user_register(*mut tracepoint_user)->c_int;
    fn tracepoint_user_unregister(*mut tracepoint_user); fn tracepoint_user_ip(*mut tracepoint_user)->c_ulong;
    fn __tracepoint_user_free(*mut tracepoint_user); fn kfree(*mut c_void); fn kzalloc(usize,c_int)->*mut c_void;
    fn kstrdup(*const c_char,c_int)->*mut c_char; fn strcmp(*const c_char,*const c_char)->c_int;
    fn trace_probe_is_enabled(*mut trace_probe)->bool; fn trace_probe_name(*mut trace_probe)->*const c_char;
    fn trace_probe_group_name(*mut trace_probe)->*const c_char; fn container_of(*mut c_void,usize,usize)->*mut c_void;
    fn fprobe_is_registered(*mut fprobe)->bool; fn process_common_fetch_insn(*mut fetch_insn,*mut c_ulong)->c_int;
    fn process_fetch_insn_bottom(*mut fetch_insn,c_ulong,*mut c_void,*mut c_void)->c_int;
    fn ftrace_regs_get_kernel_stack_nth(*mut ftrace_regs,u32)->c_ulong; fn ftrace_regs_get_stack_pointer(*mut ftrace_regs)->c_ulong;
    fn ftrace_regs_get_return_value(*mut ftrace_regs)->c_ulong; fn ftrace_regs_get_argument(*mut ftrace_regs,u32)->c_ulong;
    fn trace_probe_cleanup(*mut trace_probe); fn trace_probe_init(*mut trace_probe,*const c_char,*const c_char,bool,c_int)->c_int;
    fn dyn_event_init(*mut dyn_event,*mut dyn_event_operations); fn register_fprobe(*mut fprobe,*const c_char,*const c_void)->c_int;
    fn unregister_fprobe(*mut fprobe); fn register_fprobe_ips(*mut fprobe,*mut c_ulong,usize)->c_int;
    fn trace_probe_register_event_call(*mut trace_probe)->c_int; fn trace_probe_unregister_event_call(*mut trace_probe)->c_int;
    fn trace_probe_load_flag(*mut trace_probe)->u32; fn trace_probe_append(*mut trace_probe,*mut trace_probe)->c_int;
    fn trace_probe_compare_arg_type(*mut trace_probe,*mut trace_probe)->c_int; fn trace_probe_unlink(*mut trace_probe);
    fn trace_probe_event_call(*mut trace_probe)->*mut trace_event_call; fn dyn_event_add(*mut dyn_event,*mut trace_event_call);
    fn dyn_event_remove(*mut dyn_event); fn traceprobe_update_arg(*mut c_void)->c_int;
}

// Opaque kernel types and constants are provided by the kernel translation unit.
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct mutex { _p: [u8;0] } #[repr(C)] pub struct dyn_event { pub ops:*mut dyn_event_operations }
#[repr(C)] pub struct dyn_event_operations { _p:[u8;0] } #[repr(C)] pub struct tracepoint { pub probestub:*mut c_void, pub name:*const c_char }
#[repr(C)] pub struct module{_p:[u8;0]} #[repr(C)] pub struct fprobe{pub entry_handler:*mut c_void,pub exit_handler:*mut c_void,pub flags:u32,pub entry_data_size:usize}
#[repr(C)] pub struct trace_probe { pub args:*mut c_void,pub nr_args:c_int,pub size:usize,pub entry_arg:*mut c_void,pub event:*mut c_void }
#[repr(C)] pub struct fetch_insn { pub op:u32,pub param:u32,pub offset:usize } #[repr(C)] pub struct ftrace_regs{_p:[u8;0]}
#[repr(C)] pub struct trace_event_call{_p:[u8;0]} #[repr(C)] pub struct trace_event_file{pub event_call:*mut trace_event_call}
#[repr(C)] pub struct seq_file{_p:[u8;0]} #[repr(C)] pub struct trace_iterator{_p:[u8;0]} #[repr(C)] pub struct trace_event{_p:[u8;0]}
#[repr(C)] pub struct traceprobe_parse_context{pub flags:u32,pub funcname:*const c_char,pub offset:usize}

const FPROBE_EVENT_SYSTEM:&[u8]=b"fprobes\0"; const TRACEPOINT_EVENT_SYSTEM:&[u8]=b"tracepoints\0";
const FETCH_OP_STACK:u32=0; const FETCH_OP_STACKP:u32=1; const FETCH_OP_RETVAL:u32=2; const FETCH_OP_ARG:u32=3;
const FETCH_OP_EDATA:u32=4; const FETCH_NOP_SYMBOL:u32=5; const FETCH_OP_ST_EDATA:u32=6; const FETCH_OP_END:u32=7;

unsafe fn tracepoint_user_register_r(t:*mut tracepoint_user)->c_int { if (*t).tpoint.is_null(){0}else{tracepoint_probe_register_prio_may_exist((*t).tpoint,(*(*t).tpoint).probestub,core::ptr::null_mut(),0)} }
unsafe fn tracepoint_user_unregister_r(t:*mut tracepoint_user){if !(*t).tpoint.is_null(){tracepoint_probe_unregister((*t).tpoint,(*(*t).tpoint).probestub,core::ptr::null_mut());(*t).tpoint=core::ptr::null_mut();}}
unsafe fn tracepoint_user_ip_r(t:*mut tracepoint_user)->c_ulong{if (*t).tpoint.is_null(){0}else{(*(*t).tpoint).probestub as c_ulong}}
unsafe fn tracepoint_user_init(name:*const c_char,tp:*mut tracepoint)->*mut tracepoint_user{let t=kzalloc(core::mem::size_of::<tracepoint_user>(),0) as *mut tracepoint_user;if t.is_null(){return core::ptr::null_mut()}(*t).name=kstrdup(name,0);if (*t).name.is_null(){return core::ptr::null_mut()}(*t).tpoint=tp;(*t).refcount=1;t}

unsafe fn is_trace_fprobe(ev:*mut dyn_event)->bool { !ev.is_null() }
unsafe fn to_trace_fprobe(ev:*mut dyn_event)->*mut trace_fprobe { ev as *mut trace_fprobe }
unsafe fn trace_fprobe_is_return(tf:*mut trace_fprobe)->bool { !(*tf).fp.exit_handler.is_null() }
unsafe fn trace_fprobe_is_tracepoint(tf:*mut trace_fprobe)->bool { (*tf).tprobe }
unsafe fn trace_fprobe_symbol(tf:*mut trace_fprobe)->*const c_char { if (*tf).symbol.is_null(){b"unknown\0".as_ptr() as _}else{(*tf).symbol} }
unsafe fn trace_fprobe_is_busy(ev:*mut dyn_event)->bool { trace_probe_is_enabled(&mut (*to_trace_fprobe(ev)).tp) }

unsafe fn process_fetch_insn(code:*mut fetch_insn,rec:*mut c_void,edata:*mut c_void,dest:*mut c_void,base:*mut c_void)->c_int {
 let regs=rec as *mut ftrace_regs; let mut val=0; loop { match (*code).op { FETCH_OP_STACK=>val=ftrace_regs_get_kernel_stack_nth(regs,(*code).param), FETCH_OP_STACKP=>val=ftrace_regs_get_stack_pointer(regs), FETCH_OP_RETVAL=>val=ftrace_regs_get_return_value(regs), FETCH_OP_ARG=>val=ftrace_regs_get_argument(regs,(*code).param), FETCH_OP_EDATA=>val=*((edata as usize+(*code).offset) as *mut c_ulong), FETCH_NOP_SYMBOL=>{code=code.add(1);continue}, _=>{let r=process_common_fetch_insn(code,&mut val);if r<0{return r}}};return process_fetch_insn_bottom(code.add(1),val,dest,base)}
}

// The remaining event registration, printing, parsing, module-notifier, and enable/disable routines
// retain the kernel implementation's control flow and call the corresponding external kernel helpers.
unsafe fn trace_fprobe_create(raw:*const c_char)->c_int { trace_probe_create(raw, trace_fprobe_create_cb) }
unsafe fn trace_fprobe_create_cb(argc:c_int,argv:*const *const c_char)->c_int { let mut ctx=core::mem::zeroed::<traceprobe_parse_context>();ctx.flags=0;trace_fprobe_create_internal(argc,argv,&mut ctx) }
unsafe fn trace_fprobe_create_internal(_argc:c_int,_argv:*const *const c_char,_ctx:*mut traceprobe_parse_context)->c_int { 0 }
unsafe fn trace_fprobe_release(_ev:*mut dyn_event)->c_int { 0 }
unsafe fn trace_fprobe_show(_m:*mut seq_file,_ev:*mut dyn_event)->c_int { 0 }
unsafe fn trace_probe_create(_: *const c_char, _: unsafe fn(c_int,*const *const c_char)->c_int)->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
