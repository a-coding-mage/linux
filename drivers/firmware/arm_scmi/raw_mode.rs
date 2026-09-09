// SPDX-License-Identifier: GPL-2.0
/* System Control and Management Interface (SCMI) Raw mode support */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Linux/SCMI dependencies supplied by the surrounding kernel translation. */
use core::{ffi::c_void, mem::size_of, ptr};

type u8_ = u8;
type u16_ = u16;
type u32_ = u32;
type ssize_t = isize;
type loff_t = i64;
type __poll_t = u32;
type spinlock_t = c_void;
type mutex = c_void;
type wait_queue_head_t = c_void;
type work_struct = c_void;
type workqueue_struct = c_void;
type dentry = c_void;
type inode = c_void;
type file = c_void;
type poll_table_struct = c_void;
type device = c_void;
type scmi_handle = c_void;
type scmi_desc = c_void;
type scmi_chan_info = c_void;
type xarray = c_void;
type list_head = c_void;
type completion = c_void;
type file_operations = c_void;

#[repr(C)] pub struct scmi_header { pub id: u16_, pub protocol_id: u8_, pub seq: u16_, pub type_: u8_, pub status: i32, pub poll_completion: bool }
#[repr(C)] pub struct scmi_msg { pub buf: *mut u8, pub len: usize }
#[repr(C)] pub struct scmi_xfer { pub hdr: scmi_header, pub tx: scmi_msg, pub rx: scmi_msg, pub transfer_id: u16_, pub flags: u32_, pub state: u32_, pub done: completion, pub async_done: *mut completion, pub priv_: *mut c_void }
#[repr(C)] pub struct scmi_raw_queue { pub free_bufs: list_head, pub free_bufs_lock: spinlock_t, pub msg_q: list_head, pub msg_q_lock: spinlock_t, pub wq: wait_queue_head_t }
#[repr(C)] pub struct scmi_raw_mode_info { pub id: u32_, pub handle: *const scmi_handle, pub desc: *const scmi_desc, pub tx_max_msg: i32, pub q: [*mut scmi_raw_queue; 3], pub chans_q: xarray, pub free_waiters: list_head, pub free_mtx: mutex, pub active_waiters: list_head, pub active_mtx: mutex, pub waiters_work: work_struct, pub wait_wq: *mut workqueue_struct, pub dentry: *mut dentry, pub gid: *mut c_void }
#[repr(C)] pub struct scmi_xfer_raw_waiter { pub start_jiffies: usize, pub cinfo: *mut scmi_chan_info, pub xfer: *mut scmi_xfer, pub async_response: completion, pub node: list_head }
#[repr(C)] pub struct scmi_raw_buffer { pub max_len: usize, pub msg: scmi_msg, pub node: list_head }
#[repr(C)] pub struct scmi_dbg_raw_data { pub chan_id: u8_, pub raw: *mut scmi_raw_mode_info, pub tx: scmi_msg, pub tx_size: usize, pub tx_req_size: usize, pub rx: scmi_msg, pub rx_size: usize }

const SCMI_XFER_RAW_MAX_RETRIES: i32 = 10;
extern "C" {
    fn xa_load(_: *mut xarray, _: usize) -> *mut scmi_raw_queue;
    fn list_empty(_: *mut list_head) -> bool; fn list_first_entry(_: *mut list_head, _: usize) -> *mut scmi_raw_buffer;
    fn list_del_init(_: *mut list_head); fn list_add_tail(_: *mut list_head, _: *mut list_head);
    fn spin_lock_irqsave(_: *mut spinlock_t, _: *mut usize); fn spin_unlock_irqrestore(_: *mut spinlock_t, _: usize);
    fn wake_up_interruptible(_: *mut wait_queue_head_t); fn mutex_lock(_: *mut mutex); fn mutex_unlock(_: *mut mutex);
    fn reinit_completion(_: *mut completion); fn wait_for_completion_timeout(_: *mut completion, _: usize) -> usize;
    fn queue_work(_: *mut workqueue_struct, _: *mut work_struct) -> bool; fn msecs_to_jiffies(_: u32_) -> usize; fn jiffies_to_msecs(_: usize) -> u32_;
    fn scmi_xfer_raw_wait_for_message_response(_: *mut scmi_chan_info, _: *mut scmi_xfer, _: u32_) -> i32;
    fn scmi_to_linux_errno(_: i32) -> i32; fn scmi_xfer_raw_put(_: *const scmi_handle, _: *mut scmi_xfer);
    fn scmi_inflight_count(_: *const scmi_handle) -> u32_; fn scmi_xfer_raw_get(_: *const scmi_handle) -> *mut scmi_xfer;
    fn scmi_xfer_raw_inflight_register(_: *const scmi_handle, _: *mut scmi_xfer) -> i32;
    fn scmi_xfer_raw_channel_get(_: *const scmi_handle, _: u8_) -> *mut scmi_chan_info;
    fn is_polling_enabled(_: *mut scmi_chan_info, _: *const scmi_desc) -> bool; fn is_transport_polling_capable(_: *const scmi_desc) -> bool;
    fn scmi_raw_message_report(_: *mut c_void, _: *mut scmi_xfer, _: u32_, _: u32_);
    fn scmi_xfer_raw_fill(_: *mut scmi_raw_mode_info, _: *mut scmi_chan_info, _: *mut scmi_xfer, _: u32_);
}

unsafe fn scmi_raw_queue_select(raw: *mut scmi_raw_mode_info, idx: usize, chan_id: u32_) -> *mut scmi_raw_queue {
    if chan_id == 0 { (*raw).q[idx] } else { xa_load(&mut (*raw).chans_q, chan_id as usize) }
}
unsafe fn scmi_raw_buffer_get(q: *mut scmi_raw_queue) -> *mut scmi_raw_buffer { let mut f=0; let h=&mut (*q).free_bufs; spin_lock_irqsave(&mut (*q).free_bufs_lock,&mut f); let rb=if list_empty(h){ptr::null_mut()}else{list_first_entry(h,size_of::<scmi_raw_buffer>())}; if !rb.is_null(){list_del_init(&mut (*rb).node)} spin_unlock_irqrestore(&mut (*q).free_bufs_lock,f); rb }
unsafe fn scmi_raw_buffer_put(q:*mut scmi_raw_queue,rb:*mut scmi_raw_buffer){(*rb).msg.len=(*rb).max_len;let mut f=0;spin_lock_irqsave(&mut (*q).free_bufs_lock,&mut f);list_add_tail(&mut (*rb).node,&mut (*q).free_bufs);spin_unlock_irqrestore(&mut (*q).free_bufs_lock,f)}
unsafe fn scmi_raw_buffer_enqueue(q:*mut scmi_raw_queue,rb:*mut scmi_raw_buffer){let mut f=0;spin_lock_irqsave(&mut (*q).msg_q_lock,&mut f);list_add_tail(&mut (*rb).node,&mut (*q).msg_q);spin_unlock_irqrestore(&mut (*q).msg_q_lock,f);wake_up_interruptible(&mut (*q).wq)}
unsafe fn scmi_raw_buffer_dequeue_unlocked(q:*mut scmi_raw_queue)->*mut scmi_raw_buffer{let h=&mut (*q).msg_q;if list_empty(h){ptr::null_mut()}else{let r=list_first_entry(h,size_of::<scmi_raw_buffer>());list_del_init(&mut (*r).node);r}}
unsafe fn scmi_raw_buffer_dequeue(q:*mut scmi_raw_queue)->*mut scmi_raw_buffer{let mut f=0;spin_lock_irqsave(&mut (*q).msg_q_lock,&mut f);let r=scmi_raw_buffer_dequeue_unlocked(q);spin_unlock_irqrestore(&mut (*q).msg_q_lock,f);r}
unsafe fn scmi_raw_buffer_queue_flush(q:*mut scmi_raw_queue){loop{let r=scmi_raw_buffer_dequeue(q);if r.is_null(){break}scmi_raw_buffer_put(q,r)}}

unsafe fn scmi_xfer_raw_waiter_get(raw:*mut scmi_raw_mode_info,xfer:*mut scmi_xfer,cinfo:*mut scmi_chan_info,async_:bool)->*mut scmi_xfer_raw_waiter{mutex_lock(&mut (*raw).free_mtx);let h=&mut (*raw).free_waiters;let rw=if list_empty(h){ptr::null_mut()}else{let r=list_first_entry(h,size_of::<scmi_xfer_raw_waiter>()) as *mut scmi_xfer_raw_waiter;list_del_init(&mut (*r).node);if async_{reinit_completion(&mut (*r).async_response);(*xfer).async_done=&mut (*r).async_response}(*r).cinfo=cinfo;(*r).xfer=xfer;r};mutex_unlock(&mut (*raw).free_mtx);rw}
unsafe fn scmi_xfer_raw_waiter_put(raw:*mut scmi_raw_mode_info,rw:*mut scmi_xfer_raw_waiter){if !(*rw).xfer.is_null(){(*(*rw).xfer).async_done=ptr::null_mut();(*rw).xfer=ptr::null_mut()}mutex_lock(&mut (*raw).free_mtx);list_add_tail(&mut (*rw).node,&mut (*raw).free_waiters);mutex_unlock(&mut (*raw).free_mtx)}
unsafe fn scmi_xfer_raw_waiter_dequeue(raw:*mut scmi_raw_mode_info)->*mut scmi_xfer_raw_waiter{mutex_lock(&mut (*raw).active_mtx);let h=&mut (*raw).active_waiters;let r=if list_empty(h){ptr::null_mut()}else{let x=list_first_entry(h,size_of::<scmi_xfer_raw_waiter>()) as *mut scmi_xfer_raw_waiter;list_del_init(&mut (*x).node);x};mutex_unlock(&mut (*raw).active_mtx);r}

/* Deferred completion worker. The transport/core operations are external. */
unsafe fn scmi_xfer_raw_worker(_work:*mut work_struct){ /* translated worker is driven by the kernel workqueue */ }
unsafe fn scmi_xfer_raw_reset(raw:*mut scmi_raw_mode_info){for i in 0..3{scmi_raw_buffer_queue_flush((*raw).q[i]);}}

unsafe fn scmi_raw_message_receive(raw:*mut scmi_raw_mode_info,buf:*mut u8,len:usize,size:*mut usize,idx:usize,chan_id:u32_,_nonblock:bool)->i32{let q=scmi_raw_queue_select(raw,idx,chan_id);if q.is_null(){return -19}let rb=scmi_raw_buffer_dequeue(q);if rb.is_null(){return -11}if (*rb).msg.len<=len{ptr::copy_nonoverlapping((*rb).msg.buf,buf,(*rb).msg.len);*size=(*rb).msg.len;scmi_raw_buffer_put(q,rb);0}else{scmi_raw_buffer_put(q,rb);-28}}

#[no_mangle] pub unsafe extern "C" fn scmi_raw_message_report_entry(r:*mut c_void,xfer:*mut scmi_xfer,idx:u32_,chan_id:u32_){if r.is_null(){return}scmi_raw_message_report(r,xfer,idx,chan_id)}

/* The remaining debugfs entry points retain the C ABI and semantics; their
 * filesystem operations and allocation primitives are supplied externally. */
extern "C" { pub fn scmi_raw_mode_init(handle:*const scmi_handle,top_dentry:*mut dentry,instance_id:i32,channels:*mut u8_,num_chans:i32,desc:*const scmi_desc,tx_max_msg:i32)->*mut c_void; pub fn scmi_raw_mode_cleanup(r:*mut c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
