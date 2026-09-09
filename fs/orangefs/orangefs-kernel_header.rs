/* SPDX-License-Identifier: GPL-2.0 */
/* C headers and build-time dependencies are supplied by the surrounding kernel port. */

pub const ORANGEFS_DEFAULT_OP_TIMEOUT_SECS: i32 = 20;
pub const ORANGEFS_BUFMAP_WAIT_TIMEOUT_SECS: i32 = 30;
pub const ORANGEFS_DEFAULT_SLOT_TIMEOUT_SECS: i32 = 900; /* 15 minutes */
pub const ORANGEFS_REQDEVICE_NAME: &str = "pvfs2-req";
pub const ORANGEFS_DEVREQ_MAGIC: u32 = 0x20030529;
pub const ORANGEFS_PURGE_RETRY_COUNT: u32 = 0x00000005;

/* Sizes retain the original dependency on the protocol structures. */
pub const MAX_DEV_REQ_UPSIZE: usize = 2 * core::mem::size_of::<i32>()
    + core::mem::size_of::<u64>() + core::mem::size_of::<orangefs_upcall_s>();
pub const MAX_DEV_REQ_DOWNSIZE: usize = 2 * core::mem::size_of::<i32>()
    + core::mem::size_of::<u64>() + core::mem::size_of::<orangefs_downcall_s>();

#[repr(C)]
#[derive(Copy, Clone)]
pub enum orangefs_vfs_op_states {
    OP_VFS_STATE_UNKNOWN = 0,
    OP_VFS_STATE_WAITING = 1,
    OP_VFS_STATE_INPROGR = 2,
    OP_VFS_STATE_SERVICED = 4,
    OP_VFS_STATE_PURGED = 8,
    OP_VFS_STATE_GIVEN_UP = 16,
}

pub extern "C" {
    pub static orangefs_xattr_handlers: *const *const xattr_handler;
    pub fn orangefs_get_acl(inode: *mut inode, ty: i32, rcu: bool) -> *mut posix_acl;
    pub fn orangefs_set_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, ty: i32) -> i32;
    pub fn __orangefs_set_acl(inode: *mut inode, acl: *mut posix_acl, ty: i32) -> i32;
}

#[repr(C)]
pub union orangefs_kernel_op_s__bindgen_ty_1 { pub uses_shared_memory: i32, pub slot_to_free: i32 }

#[repr(C)]
pub struct orangefs_kernel_op_s {
    pub op_state: orangefs_vfs_op_states,
    pub tag: u64,
    pub _bindgen_union: orangefs_kernel_op_s__bindgen_ty_1,
    pub upcall: orangefs_upcall_s,
    pub downcall: orangefs_downcall_s,
    pub waitq: completion,
    pub lock: spinlock_t,
    pub attempts: i32,
    pub list: list_head,
}

pub unsafe fn set_op_state_waiting(op: *mut orangefs_kernel_op_s) { (*op).op_state = orangefs_vfs_op_states::OP_VFS_STATE_WAITING; }
pub unsafe fn set_op_state_inprogress(op: *mut orangefs_kernel_op_s) { (*op).op_state = orangefs_vfs_op_states::OP_VFS_STATE_INPROGR; }
pub unsafe fn set_op_state_given_up(op: *mut orangefs_kernel_op_s) { (*op).op_state = orangefs_vfs_op_states::OP_VFS_STATE_GIVEN_UP; }
pub unsafe fn set_op_state_serviced(op: *mut orangefs_kernel_op_s) { (*op).op_state = orangefs_vfs_op_states::OP_VFS_STATE_SERVICED; complete(&mut (*op).waitq); }
pub unsafe fn op_state_waiting(op: *const orangefs_kernel_op_s) -> bool { (*op).op_state as i32 & 1 != 0 }
pub unsafe fn op_state_in_progress(op: *const orangefs_kernel_op_s) -> bool { (*op).op_state as i32 & 2 != 0 }
pub unsafe fn op_state_serviced(op: *const orangefs_kernel_op_s) -> bool { (*op).op_state as i32 & 4 != 0 }
pub unsafe fn op_state_purged(op: *const orangefs_kernel_op_s) -> bool { (*op).op_state as i32 & 8 != 0 }
pub unsafe fn op_state_given_up(op: *const orangefs_kernel_op_s) -> bool { (*op).op_state as i32 & 16 != 0 }
pub unsafe fn op_is_cancel(op: *const orangefs_kernel_op_s) -> bool { (*op).upcall.type_ == ORANGEFS_VFS_OP_CANCEL }

pub extern "C" { pub fn op_release(op: *mut orangefs_kernel_op_s); pub fn orangefs_bufmap_put(slot: i32); }
pub unsafe fn put_cancel(op: *mut orangefs_kernel_op_s) { orangefs_bufmap_put((*op)._bindgen_union.slot_to_free); op_release(op); }
pub unsafe fn set_op_state_purged(op: *mut orangefs_kernel_op_s) {
    spin_lock(&mut (*op).lock);
    if op_is_cancel(op) { list_del_init(&mut (*op).list); spin_unlock(&mut (*op).lock); put_cancel(op); }
    else { (*op).op_state = core::mem::transmute((*op).op_state as i32 | 8); complete(&mut (*op).waitq); spin_unlock(&mut (*op).lock); }
}

#[repr(C)] pub struct orangefs_inode_s { pub refn: orangefs_object_kref, pub link_target: [u8; ORANGEFS_NAME_MAX as usize], pub xattr_sem: rw_semaphore, pub vfs_inode: inode, pub last_failed_block_index_read: sector_t, pub getattr_time: c_ulong, pub mapping_time: c_ulong, pub attr_valid: i32, pub attr_uid: kuid_t, pub attr_gid: kgid_t, pub bitlock: c_ulong, pub xattr_cache: [hlist_head; 16] }
#[repr(C)] pub struct orangefs_sb_info_s { pub root_khandle: orangefs_khandle, pub fs_id: i32, pub id: i32, pub flags: i32, pub devname: [u8; ORANGEFS_MAX_SERVER_ADDR_LEN as usize], pub sb: *mut super_block, pub mount_pending: i32, pub no_list: i32, pub list: list_head }
pub const ORANGEFS_OPT_INTR: i32 = 0x01;
pub const ORANGEFS_OPT_LOCAL_LOCK: i32 = 0x02;
#[repr(C)] pub struct orangefs_stats { pub cache_hits: c_ulong, pub cache_misses: c_ulong, pub reads: c_ulong, pub writes: c_ulong }
#[repr(C)] pub struct orangefs_cached_xattr { pub node: hlist_node, pub key: [u8; ORANGEFS_MAX_XATTR_NAMELEN as usize], pub val: [u8; ORANGEFS_MAX_XATTR_VALUELEN as usize], pub length: ssize_t, pub timeout: c_ulong }
#[repr(C)] pub struct orangefs_write_range { pub pos: loff_t, pub len: usize, pub uid: kuid_t, pub gid: kgid_t }

pub extern "C" { pub static mut orangefs_stats: orangefs_stats; }
pub unsafe fn ORANGEFS_I(inode: *mut inode) -> *mut orangefs_inode_s { container_of(inode, orangefs_inode_s, vfs_inode) }
pub unsafe fn ORANGEFS_SB(sb: *mut super_block) -> *mut orangefs_sb_info_s { (*sb).s_fs_info as *mut orangefs_sb_info_s }
pub unsafe fn orangefs_khandle_to_ino(khandle: *mut orangefs_khandle) -> ino_t {
    let mut u = [0u8; 8];
    u[0]=(*khandle).u[0]^(*khandle).u[4]; u[1]=(*khandle).u[1]^(*khandle).u[5]; u[2]=(*khandle).u[2]^(*khandle).u[6]; u[3]=(*khandle).u[3]^(*khandle).u[7];
    u[4]=(*khandle).u[12]^(*khandle).u[8]; u[5]=(*khandle).u[13]^(*khandle).u[9]; u[6]=(*khandle).u[14]^(*khandle).u[10]; u[7]=(*khandle).u[15]^(*khandle).u[11];
    u64::from_ne_bytes(u) as ino_t
}
pub unsafe fn get_khandle_from_ino(inode: *mut inode) -> *mut orangefs_khandle { &mut (*ORANGEFS_I(inode)).refn.khandle }
pub unsafe fn is_root_handle(inode: *mut inode) -> i32 { if ORANGEFS_khandle_cmp(&(*ORANGEFS_SB((*inode).i_sb)).root_khandle, get_khandle_from_ino(inode)) != 0 { 0 } else { 1 } }
pub unsafe fn match_handle(resp_handle: orangefs_khandle, inode: *mut inode) -> i32 { if ORANGEFS_khandle_cmp(&resp_handle, get_khandle_from_ino(inode)) != 0 { 0 } else { 1 } }

/* Function declarations from the remaining OrangeFS kernel translation units. */
pub extern "C" {
    pub fn op_cache_initialize() -> i32; pub fn op_cache_finalize() -> i32; pub fn op_alloc(ty: i32) -> *mut orangefs_kernel_op_s; pub fn orangefs_new_tag(op: *mut orangefs_kernel_op_s); pub fn get_opname_string(op: *mut orangefs_kernel_op_s) -> *mut c_char;
    pub fn orangefs_inode_cache_initialize() -> i32; pub fn orangefs_inode_cache_finalize() -> i32; pub fn purge_inprogress_ops(); pub fn purge_waiting_ops();
    pub static mut orangefs_features: u64; pub static orangefs_fs_param_spec: *const fs_parameter_spec; pub fn orangefs_init_fs_context(fc: *mut fs_context) -> i32; pub fn orangefs_kill_sb(sb: *mut super_block); pub fn orangefs_remount(sb: *mut orangefs_sb_info_s) -> i32; pub fn fsid_key_table_initialize() -> i32; pub fn fsid_key_table_finalize();
    pub fn orangefs_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t; pub fn orangefs_new_inode(sb: *mut super_block, dir: *mut inode, mode: umode_t, dev: dev_t, ref_: *mut orangefs_object_kref) -> *mut inode; pub fn __orangefs_setattr(inode: *mut inode, attr: *mut iattr) -> i32; pub fn __orangefs_setattr_mode(dentry: *mut dentry, attr: *mut iattr) -> i32; pub fn orangefs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32; pub fn orangefs_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, mask: u32, flags: u32) -> i32; pub fn orangefs_permission(idmap: *mut mnt_idmap, inode: *mut inode, mask: i32) -> i32; pub fn orangefs_update_time(inode: *mut inode, ty: fs_update_time, flags: u32) -> i32; pub fn orangefs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> ssize_t; pub fn orangefs_iget(sb: *mut super_block, ref_: *mut orangefs_object_kref) -> *mut inode;
    pub static mut orangefs_userspace_version: u32; pub fn orangefs_dev_init() -> i32; pub fn orangefs_dev_cleanup(); pub fn is_daemon_in_service() -> i32; pub fn __is_daemon_in_service() -> bool;
    pub fn orangefs_revalidate_mapping(inode: *mut inode) -> i32; pub fn wait_for_direct_io(ty: ORANGEFS_io_type, inode: *mut inode, pos: *mut loff_t, iter: *mut iov_iter, len: usize, offset: loff_t, range: *mut orangefs_write_range, ret: *mut i32, file: *mut file) -> ssize_t; pub fn do_readv_writev(ty: ORANGEFS_io_type, file: *mut file, pos: *mut loff_t, iter: *mut iov_iter) -> ssize_t;
    pub fn fsid_of_op(op: *mut orangefs_kernel_op_s) -> i32; pub fn orangefs_inode_getxattr(inode: *mut inode, name: *const c_char, buffer: *mut c_void, size: usize) -> ssize_t; pub fn orangefs_inode_setxattr(inode: *mut inode, name: *const c_char, value: *const c_void, size: usize, flags: i32) -> i32; pub fn orangefs_inode_getattr(inode: *mut inode, flags: i32) -> i32; pub fn orangefs_inode_check_changed(inode: *mut inode) -> i32; pub fn orangefs_inode_setattr(inode: *mut inode) -> i32; pub fn orangefs_cancel_op_in_progress(op: *mut orangefs_kernel_op_s) -> bool; pub fn orangefs_normalize_to_errno(error_code: i32) -> i32;
    pub static mut orangefs_request_mutex: mutex; pub static mut op_timeout_secs: i32; pub static mut slot_timeout_secs: i32; pub static mut orangefs_cache_timeout_msecs: i32; pub static mut orangefs_dcache_timeout_msecs: i32; pub static mut orangefs_getattr_timeout_msecs: i32; pub static mut orangefs_superblocks: list_head; pub static mut orangefs_superblocks_lock: spinlock_t; pub static mut orangefs_request_list: list_head; pub static mut orangefs_request_list_lock: spinlock_t; pub static mut orangefs_request_list_waitq: wait_queue_head_t; pub static mut orangefs_htable_ops_in_progress: *mut list_head; pub static mut orangefs_htable_ops_in_progress_lock: spinlock_t; pub static mut hash_table_size: i32; pub static orangefs_file_operations: file_operations; pub static orangefs_symlink_inode_operations: inode_operations; pub static orangefs_dir_inode_operations: inode_operations; pub static orangefs_dir_operations: file_operations; pub static orangefs_dentry_operations: dentry_operations;
    pub fn service_operation(op: *mut orangefs_kernel_op_s, name: *const c_char, flags: i32) -> i32;
}

pub const ORANGEFS_GETATTR_NEW: i32 = 1; pub const ORANGEFS_GETATTR_SIZE: i32 = 2;
pub const ORANGEFS_OP_INTERRUPTIBLE: i32 = 1; pub const ORANGEFS_OP_PRIORITY: i32 = 2; pub const ORANGEFS_OP_CANCELLATION: i32 = 4; pub const ORANGEFS_OP_NO_MUTEX: i32 = 8; pub const ORANGEFS_OP_ASYNC: i32 = 16; pub const ORANGEFS_OP_WRITEBACK: i32 = 32;
pub unsafe fn get_interruptible_flag(inode: *mut inode) -> i32 { if (*ORANGEFS_SB((*inode).i_sb)).flags & ORANGEFS_OPT_INTR != 0 { ORANGEFS_OP_INTERRUPTIBLE } else { 0 } }
pub unsafe fn orangefs_set_timeout(dentry: *mut dentry) { (*dentry).d_fsdata = (jiffies + orangefs_dcache_timeout_msecs as c_ulong * HZ / 1000) as *mut c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
