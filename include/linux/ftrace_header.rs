/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/ftrace.h. Configuration-dependent declarations
 * remain conditional so their intent and external interfaces are preserved. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_void};

pub const ARCH_SUPPORTS_FTRACE_OPS: c_int = 0;

#[repr(C)] pub struct ftrace_ops { pub func: Option<ftrace_func_t>, pub next: *mut ftrace_ops, pub flags: usize, pub private: *mut c_void, pub saved_func: Option<ftrace_func_t> }
#[repr(C)] pub struct ftrace_regs {}
#[repr(C)] pub struct dyn_ftrace { pub ip: usize, pub flags: usize, pub arch: dyn_arch_ftrace }
#[repr(C)] pub struct dyn_arch_ftrace;
#[repr(C)] pub struct ftrace_hash;
#[repr(C)] pub struct ftrace_func_entry { pub hlist: hlist_node, pub ip: usize, pub direct: usize }
#[repr(C)] pub struct hlist_node;
#[repr(C)] pub struct module;
#[repr(C)] pub struct pt_regs;
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file;
#[repr(C)] pub struct ctl_table;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct list_head;

pub type ftrace_func_t = unsafe extern "C" fn(usize, usize, *mut ftrace_ops, *mut ftrace_regs);
pub type ftrace_ops_func_t = unsafe extern "C" fn(*mut ftrace_ops, usize, ftrace_ops_cmd) -> c_int;

unsafe extern "C" { pub fn arch_ftrace_match_adjust(str_: *mut c_char, search: *const c_char) -> *mut c_char; }
unsafe extern "C" { pub fn ftrace_return_to_handler(arg: usize) -> usize; }
unsafe extern "C" { pub fn ftrace_boot_snapshot(); }
unsafe extern "C" { pub fn trace_init(); pub fn early_trace_init(); }
unsafe extern "C" { pub fn ftrace_mod_address_lookup(addr: usize, size: *mut usize, off: *mut usize, modname: *mut *mut c_char, modbuildid: *mut *const u8, sym: *mut c_char) -> c_int; }
unsafe extern "C" { pub fn ftrace_mod_get_kallsym(symnum: u32, value: *mut usize, typ: *mut c_char, name: *mut c_char, module_name: *mut c_char, exported: *mut c_int) -> c_int; }
unsafe extern "C" { pub static mut ftrace_enabled: c_int; }

pub const FTRACE_OPS_FL_ENABLED: usize = 1<<0; pub const FTRACE_OPS_FL_DYNAMIC: usize = 1<<1;
pub const FTRACE_OPS_FL_SAVE_REGS: usize = 1<<2; pub const FTRACE_OPS_FL_SAVE_REGS_IF_SUPPORTED: usize = 1<<3;
pub const FTRACE_OPS_FL_RECURSION: usize = 1<<4; pub const FTRACE_OPS_FL_STUB: usize = 1<<5;
pub const FTRACE_OPS_FL_INITIALIZED: usize = 1<<6; pub const FTRACE_OPS_FL_DELETED: usize = 1<<7;
pub const FTRACE_OPS_FL_ADDING: usize = 1<<8; pub const FTRACE_OPS_FL_REMOVING: usize = 1<<9;
pub const FTRACE_OPS_FL_MODIFYING: usize = 1<<10; pub const FTRACE_OPS_FL_ALLOC_TRAMP: usize = 1<<11;
pub const FTRACE_OPS_FL_IPMODIFY: usize = 1<<12; pub const FTRACE_OPS_FL_PID: usize = 1<<13;
pub const FTRACE_OPS_FL_RCU: usize = 1<<14; pub const FTRACE_OPS_FL_TRACE_ARRAY: usize = 1<<15;
pub const FTRACE_OPS_FL_PERMANENT: usize = 1<<16; pub const FTRACE_OPS_FL_DIRECT: usize = 1<<17;
pub const FTRACE_OPS_FL_SUBOP: usize = 1<<18; pub const FTRACE_OPS_FL_GRAPH: usize = 1<<19;
pub const FTRACE_OPS_FL_SAVE_ARGS: usize = FTRACE_OPS_FL_SAVE_REGS;

#[repr(C)] pub struct ftrace_ops_hash { pub notrace_hash: *mut ftrace_hash, pub filter_hash: *mut ftrace_hash, pub regex_lock: *mut c_void }
pub const FTRACE_HASH_DEFAULT_BITS: usize = 10;
#[repr(C)] pub enum ftrace_ops_cmd { FTRACE_OPS_CMD_ENABLE_SHARE_IPMODIFY_SELF, FTRACE_OPS_CMD_ENABLE_SHARE_IPMODIFY_PEER, FTRACE_OPS_CMD_DISABLE_SHARE_IPMODIFY_PEER }
#[repr(C)] pub enum ftrace_tracing_type_t { FTRACE_TYPE_ENTER=0, FTRACE_TYPE_RETURN }
unsafe extern "C" { pub static mut ftrace_ops_list: *mut ftrace_ops; pub static mut ftrace_list_end: ftrace_ops; pub static mut ftrace_tracing_type: ftrace_tracing_type_t; }

pub const FTRACE_UPDATE_CALLS: c_int=1; pub const FTRACE_DISABLE_CALLS: c_int=2; pub const FTRACE_UPDATE_TRACE_FUNC: c_int=4; pub const FTRACE_START_FUNC_RET: c_int=8; pub const FTRACE_STOP_FUNC_RET: c_int=16; pub const FTRACE_MAY_SLEEP: c_int=32;
pub const FTRACE_FL_ENABLED: usize=1<<31; pub const FTRACE_FL_REGS: usize=1<<30; pub const FTRACE_FL_REGS_EN: usize=1<<29; pub const FTRACE_FL_TRAMP: usize=1<<28; pub const FTRACE_FL_TRAMP_EN: usize=1<<27; pub const FTRACE_FL_IPMODIFY: usize=1<<26; pub const FTRACE_FL_DISABLED: usize=1<<25; pub const FTRACE_FL_DIRECT: usize=1<<24; pub const FTRACE_FL_DIRECT_EN: usize=1<<23; pub const FTRACE_FL_CALL_OPS: usize=1<<22; pub const FTRACE_FL_CALL_OPS_EN: usize=1<<21; pub const FTRACE_FL_TOUCHED: usize=1<<20; pub const FTRACE_FL_MODIFIED: usize=1<<19;
pub const FTRACE_REF_MAX_SHIFT: usize=19; pub const FTRACE_REF_MAX: usize=(1<<19)-1;
#[inline] pub unsafe fn ftrace_rec_count(rec:*const dyn_ftrace)->usize { (*rec).flags & FTRACE_REF_MAX }

unsafe extern "C" {
 pub fn ftrace_make_nop(m:*mut module,r:*mut dyn_ftrace,a:usize)->c_int; pub fn ftrace_make_call(r:*mut dyn_ftrace,a:usize)->c_int;
 pub fn ftrace_kill(); pub fn ftrace_sync_ipi(data:*mut c_void); pub fn ftrace_init();
 pub fn ftrace_set_filter_ip(o:*mut ftrace_ops,ip:usize,remove:c_int,reset:c_int)->c_int;
 pub fn ftrace_set_filter(o:*mut ftrace_ops,b:*mut u8,len:c_int,reset:c_int)->c_int;
 pub fn ftrace_set_notrace(o:*mut ftrace_ops,b:*mut u8,len:c_int,reset:c_int)->c_int;
 pub fn ftrace_free_filter(o:*mut ftrace_ops); pub fn ftrace_ops_set_global_filter(o:*mut ftrace_ops);
 pub fn ftrace_location(ip:usize)->usize; pub fn is_ftrace_trampoline(a:usize)->bool;
}

pub const FTRACE_RETFUNC_DEPTH: usize=50; pub const FTRACE_RETSTACK_ALLOC_SIZE: usize=32;
#[repr(C, packed)] pub struct ftrace_graph_ent { pub func:usize, pub depth:c_long }
#[repr(C, packed)] pub struct fgraph_retaddr_ent { pub ent:ftrace_graph_ent, pub retaddr:usize }
#[repr(C, packed)] pub struct ftrace_graph_ret { pub func:usize, pub depth:c_int, pub overrun:u32 }
#[repr(C)] pub struct fgraph_ops;
pub type trace_func_graph_ret_t=unsafe extern "C" fn(*mut ftrace_graph_ret,*mut fgraph_ops,*mut ftrace_regs);
pub type trace_func_graph_ent_t=unsafe extern "C" fn(*mut ftrace_graph_ent,*mut fgraph_ops,*mut ftrace_regs)->c_int;
#[repr(C)] pub struct ftrace_ret_stack { pub ret:usize,pub func:usize,pub retp:*mut usize }
unsafe extern "C" { pub fn ftrace_graph_entry_stub(*mut ftrace_graph_ent,*mut fgraph_ops,*mut ftrace_regs)->c_int; pub fn ftrace_pids_enabled(*mut ftrace_ops)->bool; pub fn function_graph_enter_regs(usize,usize,usize,*mut usize,*mut ftrace_regs)->c_int; }
#[inline] pub unsafe fn function_graph_enter(ret:usize,func:usize,fp:usize,retp:*mut usize)->c_int { function_graph_enter_regs(ret,func,fp,retp,core::ptr::null_mut()) }
unsafe extern "C" { pub fn ftrace_graph_init_task(*mut task_struct); pub fn ftrace_graph_exit_task(*mut task_struct); pub fn ftrace_graph_init_idle_task(*mut task_struct,c_int); pub fn pause_graph_tracing(); pub fn unpause_graph_tracing(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
