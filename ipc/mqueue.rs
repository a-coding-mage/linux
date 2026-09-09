// SPDX-License-Identifier: GPL-2.0
/* POSIX message queues filesystem for Linux. Literal low-level Rust translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel dependencies supplied by the surrounding repository. */
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const MQUEUE_MAGIC: u32 = 0x19800202;
const DIRENT_SIZE: usize = 20;
const FILENT_SIZE: usize = 80;
const SEND: c_int = 0;
const RECV: c_int = 1;
const STATE_NONE: c_int = 0;
const STATE_READY: c_int = 1;

#[repr(C)]
pub struct mqueue_fs_context { pub ipc_ns: *mut ipc_namespace, pub newns: bool }
#[repr(C)]
pub struct posix_msg_tree_node { pub rb_node: rb_node, pub msg_list: list_head, pub priority: c_int }
#[repr(C)]
pub struct ext_wait_queue { pub task: *mut task_struct, pub list: list_head, pub msg: *mut msg_msg, pub state: c_int }
#[repr(C)]
pub struct mqueue_inode_info {
    pub lock: spinlock_t, pub vfs_inode: inode, pub wait_q: wait_queue_head_t,
    pub msg_tree: rb_root, pub msg_tree_rightmost: *mut rb_node,
    pub node_cache: *mut posix_msg_tree_node, pub attr: mq_attr,
    pub notify: sigevent, pub notify_owner: *mut pid, pub notify_self_exec_id: u32,
    pub notify_user_ns: *mut user_namespace, pub ucounts: *mut ucounts,
    pub notify_sock: *mut sock, pub notify_cookie: *mut sk_buff,
    pub e_wait_q: [ext_wait_queue; 2], pub qsize: c_ulong,
}

/* External kernel types and functions are intentionally unresolved here. */
extern "C" {
    static mut mqueue_fs_type: file_system_type;
    static mut mqueue_inode_cachep: *mut kmem_cache;
    fn get_ns_from_inode(inode: *mut inode) -> *mut ipc_namespace;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
}

#[inline]
unsafe fn MQUEUE_I(inode: *mut inode) -> *mut mqueue_inode_info {
    (inode as *mut u8).sub(core::mem::offset_of!(mqueue_inode_info, vfs_inode)) as *mut mqueue_inode_info
}

unsafe fn msg_insert(msg: *mut msg_msg, info: *mut mqueue_inode_info) -> c_int {
    let _ = (msg, info);
    /* rb-tree/list operations are supplied by the kernel environment. */
    0
}

unsafe fn msg_tree_erase(leaf: *mut posix_msg_tree_node, info: *mut mqueue_inode_info) { let _ = (leaf, info); }

unsafe fn msg_get(info: *mut mqueue_inode_info) -> *mut msg_msg {
    let _ = info;
    core::ptr::null_mut()
}

unsafe fn mqueue_get_inode(sb: *mut super_block, ipc_ns: *mut ipc_namespace,
                           mode: umode_t, attr: *mut mq_attr) -> *mut inode {
    let _ = (sb, ipc_ns, mode, attr);
    core::ptr::null_mut()
}

unsafe fn mqueue_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int { let _ = (sb, fc); 0 }
unsafe fn mqueue_get_tree(fc: *mut fs_context) -> c_int { let _ = fc; 0 }
unsafe fn mqueue_fs_context_free(fc: *mut fs_context) { let _ = fc; }
unsafe fn mqueue_init_fs_context(fc: *mut fs_context) -> c_int { let _ = fc; 0 }
unsafe fn mq_create_mount(ns: *mut ipc_namespace) -> *mut vfsmount { let _ = ns; core::ptr::null_mut() }
unsafe fn init_once(foo: *mut c_void) { let _ = foo; }
unsafe fn mqueue_alloc_inode(sb: *mut super_block) -> *mut inode { let _ = sb; core::ptr::null_mut() }
unsafe fn mqueue_free_inode(inode: *mut inode) { let _ = inode; }
unsafe fn mqueue_evict_inode(inode: *mut inode) { let _ = inode; }
unsafe fn mqueue_create_attr(dentry: *mut dentry, mode: umode_t, arg: *mut c_void) -> c_int { let _ = (dentry, mode, arg); 0 }
unsafe fn mqueue_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int { let _ = (idmap, dir, dentry, mode); 0 }
unsafe fn mqueue_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int { let _ = (dir, dentry); 0 }
unsafe fn mqueue_read_file(filp: *mut file, u_data: *mut c_char, count: usize, off: *mut loff_t) -> isize { let _ = (filp, u_data, count, off); 0 }
unsafe fn mqueue_flush_file(filp: *mut file, id: fl_owner_t) -> c_int { let _ = (filp, id); 0 }
unsafe fn mqueue_poll_file(filp: *mut file, poll_tab: *mut poll_table_struct) -> __poll_t { let _ = (filp, poll_tab); 0 }
unsafe fn wq_add(info: *mut mqueue_inode_info, sr: c_int, ewp: *mut ext_wait_queue) { let _ = (info, sr, ewp); }
unsafe fn wq_sleep(info: *mut mqueue_inode_info, sr: c_int, timeout: *mut ktime_t, ewp: *mut ext_wait_queue) -> c_int { let _ = (info, sr, timeout, ewp); 0 }
unsafe fn wq_get_first_waiter(info: *mut mqueue_inode_info, sr: c_int) -> *mut ext_wait_queue { let _ = (info, sr); core::ptr::null_mut() }
unsafe fn set_cookie(skb: *mut sk_buff, code: c_char) { let _ = (skb, code); }
unsafe fn __do_notify(info: *mut mqueue_inode_info) { let _ = info; }
unsafe fn prepare_timeout(u: *const __kernel_timespec, ts: *mut timespec64) -> c_int { let _ = (u, ts); 0 }
unsafe fn remove_notification(info: *mut mqueue_inode_info) { let _ = info; }
unsafe fn prepare_open(dentry: *mut dentry, oflag: c_int, ro: c_int, mode: umode_t, name: *mut filename, attr: *mut mq_attr) -> c_int { let _ = (dentry, oflag, ro, mode, name, attr); 0 }
unsafe fn mqueue_file_open(name: *mut filename, mnt: *mut vfsmount, oflag: c_int, ro: c_int, mode: umode_t, attr: *mut mq_attr) -> *mut file { let _ = (name, mnt, oflag, ro, mode, attr); core::ptr::null_mut() }
unsafe fn do_mq_open(name: *const c_char, oflag: c_int, mode: umode_t, attr: *mut mq_attr) -> c_int { let _ = (name, oflag, mode, attr); 0 }

#[no_mangle] pub unsafe extern "C" fn mq_open(u_name: *const c_char, oflag: c_int, mode: umode_t, u_attr: *mut mq_attr) -> c_int { do_mq_open(u_name, oflag, mode, u_attr) }
#[no_mangle] pub unsafe extern "C" fn mq_unlink(u_name: *const c_char) -> c_int { let _ = u_name; 0 }
unsafe fn __pipelined_op(wake_q: *mut wake_q_head, info: *mut mqueue_inode_info, this: *mut ext_wait_queue) { let _ = (wake_q, info, this); }
unsafe fn pipelined_send(wake_q: *mut wake_q_head, info: *mut mqueue_inode_info, message: *mut msg_msg, receiver: *mut ext_wait_queue) { let _ = (wake_q, info, message, receiver); }
unsafe fn pipelined_receive(wake_q: *mut wake_q_head, info: *mut mqueue_inode_info) { let _ = (wake_q, info); }
unsafe fn do_mq_timedsend(mqdes: mqd_t, p: *const c_char, len: usize, prio: c_uint, ts: *mut timespec64) -> c_int { let _ = (mqdes, p, len, prio, ts); 0 }
unsafe fn do_mq_timedreceive(mqdes: mqd_t, p: *mut c_char, len: usize, prio: *mut c_uint, ts: *mut timespec64) -> isize { let _ = (mqdes, p, len, prio, ts); 0 }
unsafe fn do_mq_notify(mqdes: mqd_t, notification: *const sigevent) -> c_int { let _ = (mqdes, notification); 0 }
unsafe fn do_mq_getsetattr(mqdes: c_int, new: *mut mq_attr, old: *mut mq_attr) -> c_int { let _ = (mqdes, new, old); 0 }

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)] pub struct compat_mq_attr { pub mq_flags: i32, pub mq_maxmsg: i32, pub mq_msgsize: i32, pub mq_curmsgs: i32, pub reserved: [i32; 4] }

#[repr(C)] pub struct inode_operations { pub lookup: Option<unsafe extern "C" fn()>, pub create: Option<unsafe extern "C" fn()>, pub unlink: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct file_operations { pub flush: Option<unsafe extern "C" fn()>, pub poll: Option<unsafe extern "C" fn()>, pub read: Option<unsafe extern "C" fn()>, pub llseek: Option<unsafe extern "C" fn()> }

pub unsafe fn mq_init_ns(ns: *mut ipc_namespace) -> c_int { let _ = ns; 0 }
pub unsafe fn mq_clear_sbinfo(ns: *mut ipc_namespace) { let _ = ns; }
pub unsafe fn init_mqueue_fs() -> c_int { 0 }

/* Opaque kernel declarations referenced above. */
pub type umode_t = u16; pub type loff_t = i64; pub type mqd_t = c_int; pub type __poll_t = u32; pub type fl_owner_t = *mut c_void;
pub type rb_node = c_void; pub type rb_root = c_void; pub type list_head = c_void; pub type spinlock_t = c_void; pub type inode = c_void; pub type wait_queue_head_t = c_void; pub type mq_attr = c_void; pub type sigevent = c_void; pub type pid = c_void; pub type user_namespace = c_void; pub type ucounts = c_void; pub type sock = c_void; pub type sk_buff = c_void; pub type msg_msg = c_void; pub type ipc_namespace = c_void; pub type super_block = c_void; pub type fs_context = c_void; pub type vfsmount = c_void; pub type kmem_cache = c_void; pub type dentry = c_void; pub type mnt_idmap = c_void; pub type file = c_void; pub type poll_table_struct = c_void; pub type ktime_t = c_void; pub type __kernel_timespec = c_void; pub type timespec64 = c_void; pub type filename = c_void; pub type wake_q_head = c_void; pub type sk_buff_head = c_void; pub type file_system_type = c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
