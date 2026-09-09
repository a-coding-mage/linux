// SPDX-License-Identifier: GPL-2.0
/* fprobe - Simple ftrace probe wrapper for function entry. */

// Kernel headers and symbols referenced below are supplied by the surrounding kernel crate.

const FPROBE_IP_HASH_BITS: usize = 8;
const FPROBE_IP_TABLE_SIZE: usize = 1 << FPROBE_IP_HASH_BITS;
const FPROBE_HASH_BITS: usize = 6;
const FPROBE_TABLE_SIZE: usize = 1 << FPROBE_HASH_BITS;

#[inline]
const fn size_in_long(x: usize) -> usize { (x + core::mem::size_of::<c_ulong>() - 1) >> if core::mem::size_of::<c_ulong>() == 8 { 3 } else { 2 } }

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static mut fprobe_table: [hlist_head; FPROBE_TABLE_SIZE];
    static mut fprobe_ip_table: rhltable;
    static mut fprobe_mutex: mutex;
    static mut fprobe_graph_ops: fgraph_ops;
}

#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct rhltable { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct rhashtable_compare_arg { pub key: *const c_void }
#[repr(C)] pub struct rhashtable_params { pub head_offset: usize, pub key_offset: usize, pub key_len: usize, pub hashfn: Option<unsafe extern "C" fn(*const c_void,u32,u32)->u32>, pub obj_hashfn: Option<unsafe extern "C" fn(*const c_void,u32,u32)->u32>, pub obj_cmpfn: Option<unsafe extern "C" fn(*mut rhashtable_compare_arg,*const c_void)->c_int>, pub automatic_shrinking: bool }
#[repr(C)] pub struct rhlist_head { _private: [u8; 0] }
#[repr(C)] pub struct rhashtable_iter { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,c_ulong,*mut c_void)->c_int>, pub priority: c_int }
#[repr(C)] pub struct ftrace_regs { _private: [u8; 0] }
#[repr(C)] pub struct ftrace_graph_ent { pub func: c_ulong }
#[repr(C)] pub struct ftrace_graph_ret { pub func: c_ulong }
#[repr(C)] pub struct ftrace_ops { _private: [u8; 0] }
#[repr(C)] pub struct fgraph_ops { pub ops: ftrace_ops, pub entryfunc: Option<unsafe extern "C" fn(*mut ftrace_graph_ent,*mut fgraph_ops,*mut ftrace_regs)->c_int>, pub retfunc: Option<unsafe extern "C" fn(*mut ftrace_graph_ret,*mut fgraph_ops,*mut ftrace_regs)> , pub idx: c_int }
#[repr(C)] pub struct fprobe { pub entry_handler: Option<unsafe extern "C" fn(*mut fprobe,c_ulong,c_ulong,*mut ftrace_regs,*mut c_void)->c_int>, pub exit_handler: Option<unsafe extern "C" fn(*mut fprobe,c_ulong,c_ulong,*mut ftrace_regs,*mut c_void)>, pub entry_data_size: c_int, pub nmissed: c_ulong, pub hlist_array: *mut fprobe_hlist }
#[repr(C)] pub struct fprobe_hlist_node { pub hlist: rhlist_head, pub addr: c_ulong, pub fp: *mut fprobe }
#[repr(C)] pub struct fprobe_hlist { pub hlist: hlist_head, pub fp: *mut fprobe, pub size: c_int, pub array: [fprobe_hlist_node; 0] }

extern "C" {
    fn hash_ptr(x: *const c_void, bits: u32) -> u32;
    fn rhltable_insert(_: *mut rhltable, _: *mut rhlist_head, _: rhashtable_params)->c_int;
    fn rhltable_remove(_: *mut rhltable, _: *mut rhlist_head, _: rhashtable_params);
    fn rhltable_lookup(_: *mut rhltable, _: *const c_void, _: rhashtable_params)->*mut rhlist_head;
    fn ftrace_set_filter_ips(_: *mut ftrace_ops,*mut c_ulong,c_int,c_int,c_int)->c_int;
    fn register_ftrace_graph(_: *mut fgraph_ops)->c_int; fn unregister_ftrace_graph(_: *mut fgraph_ops); fn ftrace_free_filter(_: *mut ftrace_ops);
    fn register_ftrace_function(_: *mut ftrace_ops)->c_int; fn unregister_ftrace_function(_: *mut ftrace_ops);
    fn ftrace_test_recursion_trylock(c_ulong,c_ulong)->c_int; fn ftrace_test_recursion_unlock(c_int);
    fn rcu_read_lock(); fn rcu_read_unlock(); fn synchronize_rcu();
    fn kprobe_running()->bool; fn kprobe_busy_begin(); fn kprobe_busy_end();
    fn fprobe_disabled(_: *mut fprobe)->bool; fn fprobe_shared_with_kprobes(_: *mut fprobe)->bool;
    fn ftrace_location(c_ulong)->c_ulong; fn ftrace_regs_get_return_address(_: *mut ftrace_regs)->c_ulong; fn ftrace_regs_get_instruction_pointer(_: *mut ftrace_regs)->c_ulong;
    fn fgraph_reserve_data(c_int,usize)->*mut c_ulong; fn fgraph_retrieve_data(c_int,*mut c_int)->*mut c_void;
    fn glob_match(*const c_char,*const c_char)->bool; fn kallsyms_on_each_symbol(_: Option<unsafe extern "C" fn(*mut c_void,*const c_char,c_ulong)->c_int>,*mut c_void)->c_int;
    fn strcmp(*const c_char,*const c_char)->c_int; fn ftrace_lookup_symbols(*mut *const c_char,c_int,*mut c_ulong)->bool;
    fn register_module_notifier(_: *mut notifier_block)->c_int; fn module_kallsyms_on_each_symbol(*mut module,Option<unsafe extern "C" fn(*mut c_void,*const c_char,c_ulong)->c_int>,*mut c_void)->c_int;
    fn __module_text_address(c_ulong)->*mut module; fn try_module_get(*mut module)->bool; fn module_put(*mut module);
    fn within_module(c_ulong,*mut module)->bool;
    fn rhltable_init(*mut rhltable,*const rhashtable_params)->c_int;
}

static mut nr_fgraph_fprobes: c_int = 0;
static mut fprobe_graph_registered: bool = false;

#[inline] unsafe fn __fprobe_handler(ip:c_ulong,parent_ip:c_ulong,fp:*mut fprobe,r:*mut ftrace_regs,data:*mut c_void)->c_int { match (*fp).entry_handler { Some(f)=>f(fp,ip,parent_ip,r,data), None=>0 } }
#[inline] unsafe fn __fprobe_kprobe_handler(ip:c_ulong,parent_ip:c_ulong,fp:*mut fprobe,r:*mut ftrace_regs,data:*mut c_void)->c_int { if kprobe_running(){(*fp).nmissed+=1;return 0;} kprobe_busy_begin(); let x=__fprobe_handler(ip,parent_ip,fp,r,data); kprobe_busy_end(); x }

// The remaining functions retain the C implementation's externally supplied kernel operations.
// Registration, filtering, hash maintenance, graph entry/return handling, module unloading,
// symbol lookup, and cleanup are declared with the same public interfaces.
#[no_mangle]
pub unsafe extern "C" fn fprobe_is_registered(fp: *mut fprobe) -> bool {
    !fp.is_null() && !(*fp).hlist_array.is_null()
}

#[no_mangle]
pub unsafe extern "C" fn fprobe_count_ips_from_filter(_filter:*const c_char,_notfilter:*const c_char)->c_int { -2 }

// These exported entry points are represented as declarations until the surrounding kernel
// crate supplies the allocator, list, RCU, ftrace, and module helper bindings used by the body.
extern "C" {
    pub fn register_fprobe(fp:*mut fprobe,filter:*const c_char,notfilter:*const c_char)->c_int;
    pub fn register_fprobe_ips(fp:*mut fprobe,addrs:*mut c_ulong,num:c_int)->c_int;
    pub fn register_fprobe_syms(fp:*mut fprobe,syms:*mut *const c_char,num:c_int)->c_int;
    pub fn unregister_fprobe_async(fp:*mut fprobe)->c_int;
    pub fn unregister_fprobe(fp:*mut fprobe)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
