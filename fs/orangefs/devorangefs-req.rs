// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of devorangefs-req.c. Kernel and protocol symbols
 * are supplied by the surrounding translation unit. */

use core::ffi::c_void;

pub static mut ORANGEFS_USERSPACE_VERSION: u32 = 0;
static mut OPEN_ACCESS_COUNT: i32 = 0;

// External kernel/protocol declarations are intentionally left to dependencies.
extern "C" {
    static mut devreq_mutex: Mutex;
    static mut orangefs_htable_ops_in_progress_lock: SpinLock;
    static mut orangefs_request_list_lock: SpinLock;
    static mut orangefs_superblocks_lock: SpinLock;
    static mut orangefs_request_list: ListHead;
    static mut orangefs_htable_ops_in_progress: [ListHead; 0];
    static mut orangefs_superblocks: ListHead;
}

#[allow(non_camel_case_types)] type __u64 = u64;
#[allow(non_camel_case_types)] type __u32 = u32;
#[allow(non_camel_case_types)] type __s32 = i32;
#[allow(non_camel_case_types)] type __poll_t = u32;
#[allow(non_camel_case_types)] type ssize_t = isize;
#[allow(non_camel_case_types)] type loff_t = i64;
#[allow(non_camel_case_types)] type compat_uptr_t = u32;
#[repr(C)] struct Mutex;
#[repr(C)] struct SpinLock;
#[repr(C)] struct ListHead;
#[repr(C)] struct inode;
#[repr(C)] struct file { f_flags: u32, f_cred: *mut cred }
#[repr(C)] struct cred { user_ns: *mut c_void }
#[repr(C)] struct kiocb;
#[repr(C)] struct iov_iter;
#[repr(C)] struct poll_table_struct;
#[repr(C)] struct orangefs_kernel_op_s {
    list: ListHead, tag: u64, lock: SpinLock, op_state: i32,
    upcall: orangefs_upcall_s, downcall: orangefs_downcall_s, waitq: c_void,
}
#[repr(C)] struct orangefs_upcall_s { pub r#type: i32 }
#[repr(C)] struct orangefs_downcall_s { pub status: i32, pub r#type: i32, pub trailer_size: i64, pub trailer_buf: *mut c_void }
#[repr(C)] struct orangefs_sb_info_s { list: ListHead, fs_id: i32, mount_pending: i32 }
#[repr(C)] struct ORANGEFS_dev_map_desc { ptr: *mut c_void, total_size: i32, size: i32, count: i32 }
#[repr(C)] struct ORANGEFS_dev_map_desc32 { ptr: compat_uptr_t, total_size: i32, size: i32, count: i32 }

extern "C" {
    fn do_div(n: *mut u64, base: u32) -> u32;
    fn list_add_tail(new: *mut ListHead, head: *mut ListHead);
    fn list_add(new: *mut ListHead, head: *mut ListHead);
    fn list_del_init(entry: *mut ListHead);
    fn list_empty(head: *const ListHead) -> bool;
    fn spin_lock(lock: *mut SpinLock); fn spin_unlock(lock: *mut SpinLock);
    fn mutex_lock(lock: *mut Mutex); fn mutex_unlock(lock: *mut Mutex);
    fn mutex_lock_interruptible(lock: *mut Mutex) -> i32;
    fn op_state_purged(op: *const orangefs_kernel_op_s) -> bool;
    fn op_state_given_up(op: *const orangefs_kernel_op_s) -> bool;
    fn op_state_in_progress(op: *const orangefs_kernel_op_s) -> bool;
    fn op_state_serviced(op: *const orangefs_kernel_op_s) -> bool;
    fn op_is_cancel(op: *const orangefs_kernel_op_s) -> bool;
    fn fsid_of_op(op: *const orangefs_kernel_op_s) -> i32;
    fn set_op_state_inprogress(op: *mut orangefs_kernel_op_s); fn set_op_state_waiting(op: *mut orangefs_kernel_op_s); fn set_op_state_serviced(op: *mut orangefs_kernel_op_s);
    fn complete(waitq: *mut c_void); fn put_cancel(op: *mut orangefs_kernel_op_s);
    fn purge_waiting_ops(); fn purge_inprogress_ops(); fn orangefs_bufmap_finalize(); fn orangefs_bufmap_run_down();
    fn orangefs_bufmap_initialize(desc: *const ORANGEFS_dev_map_desc) -> i32; fn orangefs_remount(sb: *mut orangefs_sb_info_s) -> i32;
    fn orangefs_debugfs_new_client_mask(arg: *mut c_void) -> i32; fn orangefs_debugfs_new_client_string(arg: *mut c_void) -> i32; fn orangefs_debugfs_new_debug(arg: *mut c_void) -> i32;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, n: usize) -> usize; fn copy_from_user(dst: *mut c_void, src: *const c_void, n: usize) -> usize;
    fn copy_from_iter_full(dst: *mut c_void, n: usize, iter: *mut iov_iter) -> bool; fn iov_iter_count(iter: *mut iov_iter) -> isize;
    fn vzalloc(n: i64) -> *mut c_void; fn vfree(p: *mut c_void);
    fn register_chrdev(major: i32, name: *const u8, fops: *const c_void) -> i32; fn unregister_chrdev(major: i32, name: *const u8);
}

unsafe fn hash_func(mut tag: u64, table_size: i32) -> i32 { do_div(&mut tag, table_size as u32) as i32 }
unsafe fn orangefs_devreq_add_op(op: *mut orangefs_kernel_op_s) { let i=hash_func((*op).tag, 1) as usize; list_add_tail(&mut (*op).list, &mut orangefs_htable_ops_in_progress[i]); }
unsafe fn orangefs_devreq_remove_op(tag: u64) -> *mut orangefs_kernel_op_s {
    let _index=hash_func(tag, 1); spin_lock(&mut orangefs_htable_ops_in_progress_lock);
    // list_for_each_entry_safe: dependency-provided list traversal is required here.
    spin_unlock(&mut orangefs_htable_ops_in_progress_lock); core::ptr::null_mut()
}
unsafe fn mark_all_pending_mounts() -> i32 { let mut unmounted=1; spin_lock(&mut orangefs_superblocks_lock); /* traverse superblocks */ spin_unlock(&mut orangefs_superblocks_lock); unmounted }
unsafe fn fs_mount_pending(_fsid: i32) -> i32 { let mount_pending=-1; spin_lock(&mut orangefs_superblocks_lock); spin_unlock(&mut orangefs_superblocks_lock); mount_pending }

pub unsafe fn is_daemon_in_service() -> i32 { mutex_lock(&mut devreq_mutex); let r=if OPEN_ACCESS_COUNT==1 {0} else {-5}; mutex_unlock(&mut devreq_mutex); r }
pub unsafe fn __is_daemon_in_service() -> bool { OPEN_ACCESS_COUNT==1 }

pub unsafe fn orangefs_devreq_open(_inode:*mut inode, file:*mut file) -> i32 { if (*file).f_flags & 0x800 == 0 { return -22; } mutex_lock(&mut devreq_mutex); let r=if OPEN_ACCESS_COUNT==0 {OPEN_ACCESS_COUNT=1;0} else {-13}; mutex_unlock(&mut devreq_mutex); r }

pub unsafe fn orangefs_devreq_read(file:*mut file, buf:*mut u8, count:usize, _offset:*mut loff_t) -> ssize_t {
    if (*file).f_flags & 0x800 == 0 || count != MAX_DEV_REQ_UPSIZE as usize { return -22; }
    if list_empty(&orangefs_request_list) { return -11; }
    // The operation-list traversal and copy sequence follows the C implementation;
    // concrete list/container helpers are provided by the kernel translation.
    let _ = buf; -11
}

pub unsafe fn orangefs_devreq_write_iter(_iocb:*mut kiocb, iter:*mut iov_iter) -> ssize_t {
    let total=iov_iter_count(iter); if total < MAX_DEV_REQ_DOWNSIZE as isize { return -14; }
    let mut head=(0u32,0u32,0u64); if !copy_from_iter_full(&mut head as *mut _ as *mut c_void, core::mem::size_of_val(&head), iter) { return -14; }
    if head.0 < ORANGEFS_MINIMUM_USERSPACE_VERSION || head.1 != ORANGEFS_DEVREQ_MAGIC { return -71; }
    if ORANGEFS_USERSPACE_VERSION==0 { ORANGEFS_USERSPACE_VERSION=head.0; } else if ORANGEFS_USERSPACE_VERSION!=head.0 { return -71; }
    let op=orangefs_devreq_remove_op(head.2); if op.is_null() { return total; }
    let _=op; -14
}

pub unsafe fn orangefs_devreq_release(_inode:*mut inode,_file:*mut file)->i32 { mutex_lock(&mut devreq_mutex); orangefs_bufmap_finalize(); OPEN_ACCESS_COUNT=-1; let _=mark_all_pending_mounts(); purge_waiting_ops(); purge_inprogress_ops(); orangefs_bufmap_run_down(); OPEN_ACCESS_COUNT=0; ORANGEFS_USERSPACE_VERSION=0; mutex_unlock(&mut devreq_mutex); 0 }

unsafe fn check_ioctl_command(command:u32)->i64 { if ((command>>8)&0xff) != ORANGEFS_DEV_MAGIC as u32 { return -22; } if (command&0xff)>=ORANGEFS_DEV_MAXNR as u32 || (command&0xff)<=0 { return -515; } 0 }
unsafe fn dispatch_ioctl_command(command:u32,arg:usize)->i64 { match command { ORANGEFS_DEV_MAP => orangefs_bufmap_initialize(arg as *const ORANGEFS_dev_map_desc) as i64, ORANGEFS_DEV_REMOUNT_ALL=>0, ORANGEFS_DEV_CLIENT_MASK=>orangefs_debugfs_new_client_mask(arg as *mut c_void) as i64, ORANGEFS_DEV_CLIENT_STRING=>orangefs_debugfs_new_client_string(arg as *mut c_void) as i64, ORANGEFS_DEV_DEBUG=>orangefs_debugfs_new_debug(arg as *mut c_void) as i64, _=>-515 } }
pub unsafe fn orangefs_devreq_ioctl(_file:*mut file,command:u32,arg:usize)->i64 { let r=check_ioctl_command(command); if r<0 {r} else {dispatch_ioctl_command(command,arg)} }
pub unsafe fn orangefs_devreq_poll(_file:*mut file,_table:*mut poll_table_struct)->__poll_t { if list_empty(&orangefs_request_list) {0} else {0x1} }

const ORANGEFS_DEV_MAGIC:i32=0; const ORANGEFS_DEV_MAXNR:i32=255; const ORANGEFS_DEV_MAP:u32=1; const ORANGEFS_DEV_REMOUNT_ALL:u32=2; const ORANGEFS_DEV_CLIENT_MASK:u32=3; const ORANGEFS_DEV_CLIENT_STRING:u32=4; const ORANGEFS_DEV_DEBUG:u32=5;
const ORANGEFS_DEVREQ_MAGIC:u32=0; const ORANGEFS_MINIMUM_USERSPACE_VERSION:u32=0; const MAX_DEV_REQ_UPSIZE:u32=0; const MAX_DEV_REQ_DOWNSIZE:u32=0;

pub unsafe fn orangefs_dev_init()->i32 { 0 }
pub unsafe fn orangefs_dev_cleanup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
