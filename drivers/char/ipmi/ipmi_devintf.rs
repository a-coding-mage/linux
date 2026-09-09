// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of ipmi_devintf.c. Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

use core::ffi::c_void;

#[repr(C)]
pub struct ipmi_file_private {
    pub user: *mut ipmi_user,
    pub recv_msg_lock: spinlock_t,
    pub recv_msgs: list_head,
    pub fasync_queue: *mut fasync_struct,
    pub wait: wait_queue_head_t,
    pub recv_mutex: mutex,
    pub default_retries: i32,
    pub default_retry_time_ms: u32,
}

extern "C" {
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn list_empty(head: *const list_head) -> bool;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn wake_up_interruptible(wait: *mut wait_queue_head_t);
    fn kill_fasync(queue: *mut *mut fasync_struct, sig: i32, band: u32);
    fn poll_wait(file: *mut file, wait: *mut wait_queue_head_t, table: *mut poll_table);
    fn fasync_helper(fd: i32, file: *mut file, on: i32, queue: *mut *mut fasync_struct) -> i32;
    fn iminor(inode: *mut inode) -> i32;
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut c_void);
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn spin_lock_init(l: *mut spinlock_t);
    fn init_waitqueue_head(w: *mut wait_queue_head_t);
    fn mutex_init(m: *mut mutex);
    fn ipmi_create_user(if_num: i32, h: *const ipmi_user_hndl, data: *mut ipmi_file_private, user: *mut *mut ipmi_user) -> i32;
    fn ipmi_destroy_user(user: *mut ipmi_user);
    fn ipmi_free_recv_msg(msg: *mut ipmi_recv_msg);
    fn ipmi_validate_addr(addr: *const ipmi_addr, len: u32) -> i32;
    fn ipmi_request_settime(user: *mut ipmi_user, addr: *const ipmi_addr, msgid: i64, msg: *mut kernel_ipmi_msg, data: *mut c_void, priority: i32, retries: i32, retry_time_ms: u32) -> i32;
    fn ipmi_addr_length(ty: u16) -> i32;
    fn ipmi_register_for_cmd(user: *mut ipmi_user, netfn: u8, cmd: u8, chans: u32) -> i32;
    fn ipmi_unregister_for_cmd(user: *mut ipmi_user, netfn: u8, cmd: u8, chans: u32) -> i32;
    fn ipmi_set_gets_events(user: *mut ipmi_user, val: i32) -> i32;
    fn ipmi_set_my_address(user: *mut ipmi_user, channel: u8, val: u32) -> i32;
    fn ipmi_get_my_address(user: *mut ipmi_user, channel: u8, val: *mut u8) -> i32;
    fn ipmi_set_my_LUN(user: *mut ipmi_user, channel: u8, val: u32) -> i32;
    fn ipmi_get_my_LUN(user: *mut ipmi_user, channel: u8, val: *mut u8) -> i32;
    fn ipmi_get_maintenance_mode(user: *mut ipmi_user) -> i32;
    fn ipmi_set_maintenance_mode(user: *mut ipmi_user, mode: i32) -> i32;
}

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct fasync_struct { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct ipmi_user { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct poll_table { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct ipmi_addr { pub addr_type: u16, pub data: [u8; 32] }
#[repr(C)] pub struct kernel_ipmi_msg { pub netfn: u8, pub cmd: u8, pub data_len: u16, pub data: *mut u8 }
#[repr(C)] pub struct ipmi_recv_msg { pub link: list_head, pub addr: ipmi_addr, pub recv_type: i32, pub msgid: i64, pub msg: kernel_ipmi_msg }
#[repr(C)] pub struct ipmi_msg { pub netfn: u8, pub cmd: u8, pub data_len: u16, pub data: *mut u8 }
#[repr(C)] pub struct ipmi_req { pub addr: *mut ipmi_addr, pub addr_len: u32, pub msgid: i64, pub msg: ipmi_msg }
#[repr(C)] pub struct ipmi_req_settime { pub req: ipmi_req, pub retries: i32, pub retry_time_ms: u32 }
#[repr(C)] pub struct ipmi_recv { pub recv_type: i32, pub addr: *mut ipmi_addr, pub addr_len: u32, pub msgid: i64, pub msg: ipmi_msg }
#[repr(C)] pub struct ipmi_cmdspec { pub netfn: u8, pub cmd: u8 }
#[repr(C)] pub struct ipmi_cmdspec_chans { pub netfn: u8, pub cmd: u8, pub chans: u32 }
#[repr(C)] pub struct ipmi_channel_lun_address_set { pub channel: u8, pub value: u32 }
#[repr(C)] pub struct ipmi_timing_parms { pub retries: i32, pub retry_time_ms: u32 }
#[repr(C)] pub struct ipmi_user_hndl { pub ipmi_recv_hndl: Option<unsafe extern "C" fn(*mut ipmi_recv_msg, *mut c_void)> }

const EINVAL: i32 = 22; const EFAULT: i32 = 14; const ENOMEM: i32 = 12;
const EAGAIN: i32 = 11; const EMSGSIZE: i32 = 90; const ENOTTY: i32 = 25;
const IPMI_MAX_MSG_LENGTH: u16 = 272; const IPMI_CHAN_ALL: u32 = 0xffff_ffff;
const EPOLLIN: u32 = 0x001; const EPOLLRDNORM: u32 = 0x040; const POLL_IN: u32 = 1;

unsafe extern "C" fn file_receive_handler(msg: *mut ipmi_recv_msg, data: *mut c_void) {
    let priv_ = data as *mut ipmi_file_private; let mut flags = 0usize;
    spin_lock_irqsave(&mut (*priv_).recv_msg_lock, &mut flags);
    let was_empty = list_empty(&(*priv_).recv_msgs);
    list_add_tail(&mut (*msg).link, &mut (*priv_).recv_msgs);
    spin_unlock_irqrestore(&mut (*priv_).recv_msg_lock, flags);
    if was_empty { wake_up_interruptible(&mut (*priv_).wait); kill_fasync(&mut (*priv_).fasync_queue, 29, POLL_IN); }
}

#[no_mangle] pub unsafe extern "C" fn ipmi_poll(file: *mut file, wait: *mut poll_table) -> u32 {
    let p = (*file).private_data as *mut ipmi_file_private; let mut mask = 0; let mut flags = 0;
    poll_wait(file, &mut (*p).wait, wait); spin_lock_irqsave(&mut (*p).recv_msg_lock, &mut flags);
    if !list_empty(&(*p).recv_msgs) { mask |= EPOLLIN | EPOLLRDNORM; }
    spin_unlock_irqrestore(&mut (*p).recv_msg_lock, flags); mask
}

unsafe fn handle_send_req(user: *mut ipmi_user, req: *mut ipmi_req, retries: i32, retry_time_ms: u32) -> i32 {
    if (*req).addr_len as usize > core::mem::size_of::<ipmi_addr>() { return -EINVAL; }
    let mut addr = core::mem::MaybeUninit::<ipmi_addr>::uninit();
    if copy_from_user(addr.as_mut_ptr() as *mut c_void, (*req).addr as *const c_void, (*req).addr_len as usize) != 0 { return -EFAULT; }
    let addr = addr.assume_init(); let mut msg = kernel_ipmi_msg { netfn: (*req).msg.netfn, cmd: (*req).msg.cmd, data_len: (*req).msg.data_len, data: kmalloc(IPMI_MAX_MSG_LENGTH as usize, 0) };
    if msg.data.is_null() { return -ENOMEM; }
    let rv = if ipmi_validate_addr(&addr, (*req).addr_len) != 0 { -EINVAL } else if !(*req).msg.data.is_null() && (*req).msg.data_len > IPMI_MAX_MSG_LENGTH { -EMSGSIZE } else {
        if !(*req).msg.data.is_null() && copy_from_user(msg.data as *mut c_void, (*req).msg.data as *const c_void, (*req).msg.data_len as usize) != 0 { -EFAULT } else { if (*req).msg.data.is_null() { msg.data_len = 0; } ipmi_request_settime(user, &addr, (*req).msgid, &mut msg, core::ptr::null_mut(), 0, retries, retry_time_ms) }
    }; kfree(msg.data as *mut c_void); rv
}

// The remaining ioctl, compat-ioctl, device registration, and module lifecycle
// definitions retain the C entry points and are declared for kernel linkage.
extern "C" { pub fn ipmi_ioctl(file: *mut file, cmd: u32, data: usize) -> isize; pub fn compat_ipmi_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize; pub fn init_ipmi_devintf() -> i32; pub fn cleanup_ipmi(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
