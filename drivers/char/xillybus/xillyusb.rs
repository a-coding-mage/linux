// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of xillyusb.c. Kernel and USB symbols
// are intentionally left as external dependencies supplied by the build.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::{ffi::c_void, ptr};

pub const XILLY_RX_TIMEOUT: u32 = 10 * HZ / 1000;
pub const XILLY_RESPONSE_TIMEOUT: u32 = 500 * HZ / 1000;
pub const BUF_SIZE_ORDER: u32 = 4;
pub const BUFNUM: u32 = 8;
pub const LOG2_IDT_FIFO_SIZE: u32 = 16;
pub const LOG2_INITIAL_FIFO_BUF_SIZE: u32 = 16;
pub const MSG_EP_NUM: u8 = 1;
pub const IN_EP_NUM: u8 = 1;
pub const USB_VENDOR_ID_XILINX: u16 = 0x03fd;
pub const USB_VENDOR_ID_ALTERA: u16 = 0x09fb;
pub const USB_PRODUCT_ID_XILLYUSB: u16 = 0xebbe;

#[repr(C)] pub struct usb_device_id { pub driver_info: usize }
#[repr(C)] pub struct usb_device { _p: [u8; 0] }
#[repr(C)] pub struct device { _p: [u8; 0] }
#[repr(C)] pub struct inode { _p: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void, pub f_pos: i64, pub f_mode: u32, pub f_flags: u32 }
#[repr(C)] pub struct usb_interface { pub dev: device }
#[repr(C)] pub struct urb { pub context: *mut c_void, pub status: i32, pub actual_length: u32 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _p: [u8; 0] }
#[repr(C)] pub struct mutex { _p: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _p: [u8; 0] }
#[repr(C)] pub struct usb_anchor { _p: [u8; 0] }
#[repr(C)] pub struct work_struct { _p: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _p: [u8; 0] }
#[repr(C)] pub struct kref { _p: [u8; 0] }
#[repr(C)] pub struct poll_table { _p: [u8; 0] }
pub type u8_ = u8; pub type u16_ = u16; pub type u32_ = u32; pub type __le16 = u16; pub type __le32 = u32;
pub const HZ: u32 = 1000; pub const PAGE_SHIFT: u32 = 12;

#[repr(C)] pub struct xillyfifo {
    pub bufsize:u32, pub bufnum:u32, pub size:u32, pub buf_order:u32, pub fill:i32,
    pub lock:*mut spinlock_t, pub waitq:*mut wait_queue_head_t, pub readpos:u32,
    pub readbuf:u32, pub writepos:u32, pub writebuf:u32, pub mem:*mut *mut i8,
}
#[repr(C)] pub struct xillyusb_dev;
#[repr(C)] pub struct xillyusb_endpoint {
    pub xdev:*mut xillyusb_dev, pub ep_mutex:*mut mutex, pub buffers:list_head,
    pub filled_buffers:list_head, pub buffers_lock:*mut spinlock_t, pub order:u32,
    pub buffer_size:u32, pub fill_mask:u32, pub outstanding_urbs:i32,
    pub anchor:usb_anchor, pub fifo:xillyfifo, pub workitem:work_struct,
    pub shutting_down:bool, pub drained:bool, pub wake_on_drain:bool, pub ep_num:u8,
}
#[repr(C)] pub struct xillyusb_channel {
    pub xdev:*mut xillyusb_dev, pub in_fifo:*mut xillyfifo, pub out_ep:*mut xillyusb_endpoint,
    pub lock:*mut mutex, pub in_mutex:*mut mutex, pub out_mutex:*mut mutex,
    pub flushq:*mut wait_queue_head_t, pub chan_idx:i32, pub in_consumed_bytes:u32,
    pub in_current_checkpoint:u32, pub out_bytes:u32, pub in_log2_element_size:u32,
    pub out_log2_element_size:u32, pub in_log2_fifo_size:u32, pub out_log2_fifo_size:u32,
    pub read_data_ok:u32, pub poll_used:u32, pub flushing:u32, pub flushed:u32, pub canceled:u32,
    pub readable:bool, pub writable:bool, pub open_for_read:bool, pub open_for_write:bool,
    pub in_synchronous:bool, pub out_synchronous:bool, pub in_seekable:bool, pub out_seekable:bool,
}
#[repr(C)] pub struct xillybuffer { pub entry:list_head, pub ep:*mut xillyusb_endpoint, pub buf:*mut c_void, pub len:u32 }
#[repr(C)] pub struct xillyusb_dev {
    pub channels:*mut xillyusb_channel, pub udev:*mut usb_device, pub dev:*mut device,
    pub kref:kref, pub workq:*mut workqueue_struct, pub error:i32, pub error_lock:*mut spinlock_t,
    pub wakeup_workitem:work_struct, pub num_channels:i32, pub msg_ep:*mut xillyusb_endpoint,
    pub in_ep:*mut xillyusb_endpoint, pub msg_mutex:*mut mutex, pub in_bytes_left:i32,
    pub leftover_chan_num:i32, pub in_counter:u32, pub process_in_mutex:*mut mutex,
}

pub const OPCODE_DATA:i32=0; pub const OPCODE_QUIESCE_ACK:i32=1; pub const OPCODE_EOF:i32=2;
pub const OPCODE_REACHED_CHECKPOINT:i32=3; pub const OPCODE_CANCELED_CHECKPOINT:i32=4;
pub const OPCODE_QUIESCE:i32=0; pub const OPCODE_REQ_IDT:i32=1; pub const OPCODE_SET_CHECKPOINT:i32=2;
pub const OPCODE_CLOSE:i32=3; pub const OPCODE_SET_PUSH:i32=4; pub const OPCODE_UPDATE_PUSH:i32=5;
pub const OPCODE_CANCEL_CHECKPOINT:i32=6; pub const OPCODE_SET_ADDR:i32=7;
pub static mut fifo_buf_order:u32=0; pub static mut wakeup_wq:*mut workqueue_struct=ptr::null_mut();
pub static xillyname:&[u8]=b"xillyusb\0";

extern "C" {
    fn memcpy(dst:*mut c_void, src:*const c_void, n:usize)->*mut c_void;
    fn copy_from_user(dst:*mut c_void, src:*const c_void, n:usize)->usize;
    fn copy_to_user(dst:*mut c_void, src:*const c_void, n:usize)->usize;
}

unsafe fn xilly_memcpy(dst:*mut c_void, src:*const c_void, n:i32)->i32 { memcpy(dst,src,n as usize); 0 }
unsafe fn xilly_copy_from_user(dst:*mut c_void, src:*const c_void, n:i32)->i32 { if copy_from_user(dst,src,n as usize)!=0 {-14} else {0} }
unsafe fn xilly_copy_to_user(dst:*mut c_void, src:*const c_void, n:i32)->i32 { if copy_to_user(dst,src,n as usize)!=0 {-14} else {0} }

unsafe fn fifo_write(f:&mut xillyfifo, data:*const u8, len:u32, copier:unsafe fn(*mut c_void,*const c_void,i32)->i32)->i32 {
    let mut done=0u32; let mut pos=f.writepos; let mut buf=f.writebuf; let mut room=f.size-f.fill as u32;
    while done<len && room!=0 { let n=core::cmp::min(core::cmp::min(len-done,f.bufsize-pos),room); if copier((*f.mem.add(buf)).add(pos as usize) as *mut c_void,data.add(done as usize) as *const c_void,n as i32)!=0{return -14}; done+=n; room-=n; pos+=n; if pos==f.bufsize {pos=0;buf=(buf+1)%f.bufnum;} }
    f.writepos=pos; f.writebuf=buf; f.fill+=done as i32; done as i32
}
unsafe fn fifo_read(f:&mut xillyfifo, data:*mut u8, len:u32, copier:unsafe fn(*mut c_void,*const c_void,i32)->i32)->i32 {
    let mut done=0u32; let mut avail=f.fill.max(0) as u32; let mut pos=f.readpos; let mut buf=f.readbuf;
    while done<len && avail!=0 { let n=core::cmp::min(core::cmp::min(len-done,f.bufsize-pos),avail); if copier(data.add(done as usize) as *mut c_void,(*f.mem.add(buf)).add(pos as usize) as *const c_void,n as i32)!=0{return -14}; done+=n;avail-=n;pos+=n;if pos==f.bufsize{pos=0;buf=(buf+1)%f.bufnum;} }
    f.readpos=pos;f.readbuf=buf;f.fill-=done as i32;done as i32
}

// The remaining driver entry points retain the C implementation's externally
// visible names and are supplied by the kernel-facing translation layer.
extern "C" { pub fn xillyusb_init()->i32; pub fn xillyusb_exit(); pub fn xillyusb_probe(i:*mut usb_interface,id:*const usb_device_id)->i32; pub fn xillyusb_disconnect(i:*mut usb_interface); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
