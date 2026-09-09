/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation.

#[repr(C)]
pub struct rpc_pipe_dir_head {
    pub pdh_entries: list_head,
    pub pdh_dentry: *mut dentry,
}

#[repr(C)]
pub struct rpc_pipe_dir_object_ops;

#[repr(C)]
pub struct rpc_pipe_dir_object {
    pub pdo_head: list_head,
    pub pdo_ops: *const rpc_pipe_dir_object_ops,
    pub pdo_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct rpc_pipe_dir_object_ops {
    pub create: Option<unsafe extern "C" fn(*mut dentry, *mut rpc_pipe_dir_object) -> i32>,
    pub destroy: Option<unsafe extern "C" fn(*mut dentry, *mut rpc_pipe_dir_object)>,
}

#[repr(C)]
pub struct rpc_pipe_msg {
    pub list: list_head,
    pub data: *mut core::ffi::c_void,
    pub len: usize,
    pub copied: usize,
    pub errno: i32,
}

#[repr(C)]
pub struct rpc_pipe_ops {
    pub upcall: Option<unsafe extern "C" fn(*mut file, *mut rpc_pipe_msg, *mut core::ffi::c_char, usize) -> isize>,
    pub downcall: Option<unsafe extern "C" fn(*mut file, *const core::ffi::c_char, usize) -> isize>,
    pub release_pipe: Option<unsafe extern "C" fn(*mut inode)>,
    pub open_pipe: Option<unsafe extern "C" fn(*mut inode) -> i32>,
    pub destroy_msg: Option<unsafe extern "C" fn(*mut rpc_pipe_msg)>,
}

#[repr(C)]
pub struct rpc_pipe {
    pub pipe: list_head,
    pub in_upcall: list_head,
    pub in_downcall: list_head,
    pub pipelen: i32,
    pub nreaders: i32,
    pub nwriters: i32,
    pub flags: i32,
    pub queue_timeout: delayed_work,
    pub ops: *const rpc_pipe_ops,
    pub lock: spinlock_t,
    pub dentry: *mut dentry,
}

pub const RPC_PIPE_WAIT_FOR_OPEN: i32 = 1;

#[repr(C)]
pub struct rpc_inode {
    pub vfs_inode: inode,
    pub private: *mut core::ffi::c_void,
    pub pipe: *mut rpc_pipe,
    pub waitq: wait_queue_head_t,
}

#[inline]
pub unsafe fn RPC_I(inode: *mut inode) -> *mut rpc_inode {
    // Equivalent to the Linux kernel container_of() macro.
    container_of!(inode, rpc_inode, vfs_inode)
}

pub const SUNRPC_PIPEFS_NFS_PRIO: u32 = 0;
pub const SUNRPC_PIPEFS_RPC_PRIO: u32 = 1;

pub const RPC_PIPEFS_MOUNT: u32 = 0;
pub const RPC_PIPEFS_UMOUNT: u32 = 1;

extern "C" {
    pub fn rpc_pipefs_notifier_register(nb: *mut notifier_block) -> i32;
    pub fn rpc_pipefs_notifier_unregister(nb: *mut notifier_block);
    pub fn rpc_d_lookup_sb(sb: *const super_block, dir_name: *const u8) -> *mut dentry;
    pub fn rpc_pipefs_init_net(net: *mut net) -> i32;
    pub fn rpc_pipefs_exit_net(net: *mut net);
    pub fn rpc_get_sb_net(net: *const net) -> *mut super_block;
    pub fn rpc_put_sb_net(net: *const net);
    pub fn rpc_pipe_generic_upcall(file: *mut file, msg: *mut rpc_pipe_msg, buf: *mut core::ffi::c_char, len: usize) -> isize;
    pub fn rpc_queue_upcall(pipe: *mut rpc_pipe, msg: *mut rpc_pipe_msg) -> i32;
}

#[inline]
pub fn rpc_msg_is_inflight(msg: *const rpc_pipe_msg) -> bool {
    unsafe { (*msg).copied != 0 && list_empty(&(*msg).list) }
}

pub struct rpc_clnt;

extern "C" {
    pub fn rpc_create_client_dir(dir: *mut dentry, name: *const core::ffi::c_char, clnt: *mut rpc_clnt) -> i32;
    pub fn rpc_remove_client_dir(clnt: *mut rpc_clnt) -> i32;
    pub fn rpc_init_pipe_dir_head(pdh: *mut rpc_pipe_dir_head);
    pub fn rpc_init_pipe_dir_object(pdo: *mut rpc_pipe_dir_object, pdo_ops: *const rpc_pipe_dir_object_ops, pdo_data: *mut core::ffi::c_void);
    pub fn rpc_add_pipe_dir_object(net: *mut net, pdh: *mut rpc_pipe_dir_head, pdo: *mut rpc_pipe_dir_object) -> i32;
    pub fn rpc_remove_pipe_dir_object(net: *mut net, pdh: *mut rpc_pipe_dir_head, pdo: *mut rpc_pipe_dir_object);
    pub fn rpc_find_or_alloc_pipe_dir_object(net: *mut net, pdh: *mut rpc_pipe_dir_head, match_: Option<unsafe extern "C" fn(*mut rpc_pipe_dir_object, *mut core::ffi::c_void) -> i32>, alloc: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut rpc_pipe_dir_object>, data: *mut core::ffi::c_void) -> *mut rpc_pipe_dir_object;
}

pub struct cache_detail;

extern "C" {
    pub fn rpc_create_cache_dir(dir: *mut dentry, name: *const core::ffi::c_char, umode: umode_t, detail: *mut cache_detail) -> *mut dentry;
    pub fn rpc_remove_cache_dir(dir: *mut dentry);
    pub fn rpc_mkpipe_data(ops: *const rpc_pipe_ops, flags: i32) -> *mut rpc_pipe;
    pub fn rpc_destroy_pipe_data(pipe: *mut rpc_pipe);
    pub fn rpc_mkpipe_dentry(dir: *mut dentry, name: *const core::ffi::c_char, private: *mut core::ffi::c_void, pipe: *mut rpc_pipe) -> i32;
    pub fn rpc_unlink(pipe: *mut rpc_pipe);
    pub fn register_rpc_pipefs() -> i32;
    pub fn unregister_rpc_pipefs();
    pub fn gssd_running(net: *mut net) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
