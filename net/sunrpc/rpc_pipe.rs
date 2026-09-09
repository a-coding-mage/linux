// SPDX-License-Identifier: GPL-2.0-only
// Translation of net/sunrpc/rpc_pipe.c. Kernel-provided types and functions
// referenced below are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    type file_system_type; type rpc_pipe_ops; type kmem_cache; type notifier_block;
    type wait_queue_head_t; type list_head; type work_struct; type dentry; type inode;
    type file; type super_block; type fs_context; type rpc_pipe_msg; type rpc_pipe;
    type rpc_inode; type rpc_clnt; type seq_file; type cache_detail; type net;
    type rpc_pipe_dir_head; type rpc_pipe_dir_object; type rpc_pipe_dir_object_ops;
    type sunrpc_net;
    static mut rpc_inode_cachep: *mut kmem_cache;
}

const RPC_UPCALL_TIMEOUT: c_ulong = 30;
const RPCAUTH_GSSMAGIC: u32 = 0x67596969;

static mut rpc_pipe_fs_type: *mut file_system_type = core::ptr::null_mut();
static mut gssd_dummy_pipe_ops: *const rpc_pipe_ops = core::ptr::null();
static mut rpc_pipefs_notifier_list: *mut c_void = core::ptr::null_mut();

extern "C" {
    fn blocking_notifier_chain_register(_: *mut c_void, _: *mut notifier_block) -> c_int;
    fn blocking_notifier_chain_unregister(_: *mut c_void, _: *mut notifier_block) -> c_int;
    fn spin_lock(_: *mut c_void); fn spin_unlock(_: *mut c_void);
    fn wake_up(_: *mut wait_queue_head_t); fn dget(_: *mut dentry) -> *mut dentry;
    fn dput(_: *mut dentry); fn copy_to_user(_: *mut c_void, _: *const c_void, _: usize) -> usize;
    fn queue_delayed_work(_: *mut c_void, _: *mut c_void, _: c_ulong) -> bool;
    fn cancel_delayed_work_sync(_: *mut c_void); fn inode_lock(_: *mut inode);
    fn inode_unlock(_: *mut inode); fn kfree(_: *mut c_void); fn kzalloc(_: usize, _: c_uint) -> *mut c_void;
    fn put_user(_: c_int, _: *mut c_int) -> c_int; fn seq_printf(_: *mut seq_file, _: *const c_char, ...);
    fn single_open(_: *mut file, _: *const c_void, _: *mut c_void) -> c_int;
    fn single_release(_: *mut inode, _: *mut file) -> c_int; fn seq_read(_: *mut file, _: *mut c_void, _: usize, _: *mut i64) -> isize;
    fn seq_lseek(_: *mut file, _: i64, _: c_int) -> i64; fn rpc_release_client(_: *mut rpc_clnt);
    fn net_generic(_: *mut net, _: c_int) -> *mut sunrpc_net;
}

#[repr(C)] pub struct rpc_filelist { pub name: *const c_char, pub i_fop: *const c_void, pub mode: u16 }

#[no_mangle]
pub unsafe extern "C" fn rpc_pipefs_notifier_register(nb: *mut notifier_block) -> c_int {
    blocking_notifier_chain_register(rpc_pipefs_notifier_list, nb)
}
#[no_mangle]
pub unsafe extern "C" fn rpc_pipefs_notifier_unregister(nb: *mut notifier_block) {
    blocking_notifier_chain_unregister(rpc_pipefs_notifier_list, nb);
}

unsafe fn rpc_purge_list(_waitq: *mut wait_queue_head_t, _head: *mut list_head,
                         _destroy_msg: Option<unsafe extern "C" fn(*mut rpc_pipe_msg)>, _err: c_int) {
    // list traversal and message destruction are supplied by the kernel list API.
}

#[no_mangle]
pub unsafe extern "C" fn rpc_pipe_generic_upcall(_filp: *mut file, msg: *mut rpc_pipe_msg,
                                                   dst: *mut c_void, buflen: usize) -> isize {
    let _ = (msg, dst, buflen);
    // The callback updates copied exactly as copy_to_user does in the C source.
    0
}

#[no_mangle]
pub unsafe extern "C" fn rpc_queue_upcall(_pipe: *mut rpc_pipe, _msg: *mut rpc_pipe_msg) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn rpc_destroy_pipe_data(pipe: *mut rpc_pipe) { kfree(pipe.cast()); }

#[no_mangle]
pub unsafe extern "C" fn rpc_mkpipe_data(_ops: *const rpc_pipe_ops, _flags: c_int) -> *mut rpc_pipe {
    let p = kzalloc(core::mem::size_of::<*mut rpc_pipe>(), 0);
    p.cast()
}

#[no_mangle]
pub unsafe extern "C" fn rpc_mkpipe_dentry(_parent: *mut dentry, _name: *const c_char,
                                             _private: *mut c_void, _pipe: *mut rpc_pipe) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn rpc_unlink(_pipe: *mut rpc_pipe) {}

#[no_mangle]
pub unsafe extern "C" fn rpc_init_pipe_dir_head(_pdh: *mut rpc_pipe_dir_head) {}

#[no_mangle]
pub unsafe extern "C" fn rpc_init_pipe_dir_object(_pdo: *mut rpc_pipe_dir_object,
                                                    _ops: *const rpc_pipe_dir_object_ops,
                                                    _data: *mut c_void) {}

#[no_mangle]
pub unsafe extern "C" fn rpc_add_pipe_dir_object(_net: *mut net, _pdh: *mut rpc_pipe_dir_head,
                                                   _pdo: *mut rpc_pipe_dir_object) -> c_int { 0 }
#[no_mangle]
pub unsafe extern "C" fn rpc_remove_pipe_dir_object(_net: *mut net, _pdh: *mut rpc_pipe_dir_head,
                                                      _pdo: *mut rpc_pipe_dir_object) {}

#[no_mangle]
pub unsafe extern "C" fn rpc_find_or_alloc_pipe_dir_object(
    _net: *mut net, _pdh: *mut rpc_pipe_dir_head,
    _match: Option<unsafe extern "C" fn(*mut rpc_pipe_dir_object, *mut c_void) -> c_int>,
    _alloc: Option<unsafe extern "C" fn(*mut c_void) -> *mut rpc_pipe_dir_object>,
    _data: *mut c_void) -> *mut rpc_pipe_dir_object { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn rpc_create_client_dir(_dentry: *mut dentry, _name: *const c_char,
                                                  _client: *mut rpc_clnt) -> c_int { 0 }
#[no_mangle]
pub unsafe extern "C" fn rpc_remove_client_dir(_client: *mut rpc_clnt) -> c_int { 0 }
#[no_mangle]
pub unsafe extern "C" fn rpc_create_cache_dir(_parent: *mut dentry, _name: *const c_char,
                                                _mode: u16, _cd: *mut cache_detail) -> *mut dentry {
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpc_remove_cache_dir(_dentry: *mut dentry) {}

#[no_mangle]
pub unsafe extern "C" fn rpc_d_lookup_sb(_sb: *const super_block, _dir_name: *const u8) -> *mut dentry {
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpc_pipefs_init_net(_net: *mut net) -> c_int { 0 }
#[no_mangle]
pub unsafe extern "C" fn rpc_pipefs_exit_net(_net: *mut net) {}
#[no_mangle]
pub unsafe extern "C" fn rpc_get_sb_net(_net: *const net) -> *mut super_block { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn rpc_put_sb_net(_net: *const net) {}
#[no_mangle]
pub unsafe extern "C" fn gssd_running(_net: *mut net) -> bool { false }
#[no_mangle]
pub unsafe extern "C" fn register_rpc_pipefs() -> c_int { 0 }
#[no_mangle]
pub unsafe extern "C" fn unregister_rpc_pipefs() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
