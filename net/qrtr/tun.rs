// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2018, Linaro Ltd */

// Linux kernel and qrtr declarations are supplied by the surrounding crate.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct qrtr_endpoint {
    pub xmit: Option<unsafe extern "C" fn(*mut qrtr_endpoint, *mut sk_buff) -> c_int>,
}
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: usize }
#[repr(C)] pub struct sk_buff_head;
#[repr(C)] pub struct wait_queue_head_t;
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file { pub f_flags: c_ulong, pub private_data: *mut c_void }
#[repr(C)] pub struct kiocb { pub ki_filp: *mut file }
#[repr(C)] pub struct iov_iter;
#[repr(C)] pub struct poll_table;
#[repr(C)] pub struct file_operations;
#[repr(C)] pub struct miscdevice;

extern "C" {
    fn skb_queue_tail(queue: *mut sk_buff_head, skb: *mut sk_buff);
    fn wake_up_interruptible(queue: *mut wait_queue_head_t);
    fn skb_queue_head_init(queue: *mut sk_buff_head);
    fn init_waitqueue_head(queue: *mut wait_queue_head_t);
    fn qrtr_endpoint_register(ep: *mut qrtr_endpoint, nid: c_int) -> c_int;
    fn qrtr_endpoint_unregister(ep: *mut qrtr_endpoint);
    fn skb_dequeue(queue: *mut sk_buff_head) -> *mut sk_buff;
    fn iov_iter_count(iter: *const iov_iter) -> usize;
    fn copy_to_iter(src: *const c_void, count: usize, to: *mut iov_iter) -> usize;
    fn kfree_skb(skb: *mut sk_buff);
    fn qrtr_endpoint_post(ep: *mut qrtr_endpoint, data: *mut c_void, len: usize) -> isize;
    fn copy_from_iter_full(dst: *mut c_void, len: usize, from: *mut iov_iter) -> bool;
    fn kfree(ptr: *mut c_void);
    fn poll_wait(file: *mut file, queue: *mut wait_queue_head_t, wait: *mut poll_table);
    fn skb_queue_empty(queue: *const sk_buff_head) -> bool;
    fn skb_queue_purge(queue: *mut sk_buff_head);
    fn misc_register(dev: *mut miscdevice) -> c_int;
    fn misc_deregister(dev: *mut miscdevice);
    fn kzalloc(size: usize, flags: c_ulong) -> *mut c_void;
}

const ENOMEM: c_int = 12;
const EAGAIN: c_int = 11;
const ERESTARTSYS: c_int = 512;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const O_NONBLOCK: c_ulong = 0x800;
const KMALLOC_MAX_SIZE: usize = usize::MAX;
const GFP_KERNEL: c_ulong = 0;
const EPOLLIN: u32 = 0x001;
const EPOLLRDNORM: u32 = 0x040;
const QRTR_EP_NID_AUTO: c_int = -1;

#[repr(C)]
struct qrtr_tun {
    ep: qrtr_endpoint,
    queue: sk_buff_head,
    readq: wait_queue_head_t,
}

unsafe extern "C" fn qrtr_tun_send(ep: *mut qrtr_endpoint, skb: *mut sk_buff) -> c_int {
    let tun = (ep as *mut u8).sub(core::mem::offset_of!(qrtr_tun, ep)) as *mut qrtr_tun;
    skb_queue_tail(&mut (*tun).queue, skb);
    // wake up any blocking processes, waiting for new data
    wake_up_interruptible(&mut (*tun).readq);
    0
}

unsafe extern "C" fn qrtr_tun_open(_inode: *mut inode, filp: *mut file) -> c_int {
    let tun = kzalloc(core::mem::size_of::<qrtr_tun>(), GFP_KERNEL) as *mut qrtr_tun;
    if tun.is_null() { return -ENOMEM; }
    skb_queue_head_init(&mut (*tun).queue);
    init_waitqueue_head(&mut (*tun).readq);
    (*tun).ep.xmit = Some(qrtr_tun_send);
    (*filp).private_data = tun as *mut c_void;
    let ret = qrtr_endpoint_register(&mut (*tun).ep, QRTR_EP_NID_AUTO);
    if ret != 0 {
        (*filp).private_data = core::ptr::null_mut();
        kfree(tun as *mut c_void);
        return ret;
    }
    0
}

unsafe extern "C" fn qrtr_tun_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    let filp = (*iocb).ki_filp;
    let tun = (*filp).private_data as *mut qrtr_tun;
    let mut skb;
    loop {
        skb = skb_dequeue(&mut (*tun).queue);
        if !skb.is_null() { break; }
        if (*filp).f_flags & O_NONBLOCK != 0 { return -EAGAIN as isize; }
        // Wait until we get data or the endpoint goes away
        return -ERESTARTSYS as isize;
    }
    let count = core::cmp::min(iov_iter_count(to), (*skb).len);
    let count = if copy_to_iter((*skb).data as *const c_void, count, to) != count { -EFAULT as isize } else { count as isize };
    kfree_skb(skb);
    count
}

unsafe extern "C" fn qrtr_tun_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let filp = (*iocb).ki_filp;
    let tun = (*filp).private_data as *mut qrtr_tun;
    let len = iov_iter_count(from);
    if len == 0 { return -EINVAL as isize; }
    if len > KMALLOC_MAX_SIZE { return -ENOMEM as isize; }
    let kbuf = kzalloc(len, GFP_KERNEL);
    if kbuf.is_null() { return -ENOMEM as isize; }
    if !copy_from_iter_full(kbuf, len, from) { kfree(kbuf); return -EFAULT as isize; }
    let ret = qrtr_endpoint_post(&mut (*tun).ep, kbuf, len);
    kfree(kbuf);
    if ret < 0 { ret } else { len as isize }
}

unsafe extern "C" fn qrtr_tun_poll(filp: *mut file, wait: *mut poll_table) -> u32 {
    let tun = (*filp).private_data as *mut qrtr_tun;
    poll_wait(filp, &mut (*tun).readq, wait);
    if !skb_queue_empty(&(*tun).queue) { EPOLLIN | EPOLLRDNORM } else { 0 }
}

unsafe extern "C" fn qrtr_tun_release(_inode: *mut inode, filp: *mut file) -> c_int {
    let tun = (*filp).private_data as *mut qrtr_tun;
    qrtr_endpoint_unregister(&mut (*tun).ep);
    // Discard all SKBs
    skb_queue_purge(&mut (*tun).queue);
    kfree(tun as *mut c_void);
    0
}

// The file_operations and miscdevice initializers are provided by the kernel-facing bindings.
static mut QRTR_TUN_OPS: *const file_operations = core::ptr::null();
static mut QRTR_TUN_MISCDEV: *mut miscdevice = core::ptr::null_mut();

unsafe extern "C" fn qrtr_tun_init() -> c_int {
    let ret = misc_register(QRTR_TUN_MISCDEV);
    if ret != 0 { /* pr_err("failed to register Qualcomm IPC Router tun device\n") */ }
    ret
}

unsafe extern "C" fn qrtr_tun_exit() { misc_deregister(QRTR_TUN_MISCDEV); }

// module_init(qrtr_tun_init); module_exit(qrtr_tun_exit);
// MODULE_DESCRIPTION("Qualcomm IPC Router TUN device");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
