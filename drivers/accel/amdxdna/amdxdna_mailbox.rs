// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of amdxdna_mailbox.c. */

use core::ffi::c_void;
use core::mem::{size_of, MaybeUninit};
use core::ptr;

const MAGIC_VAL: u32 = 0x1D000000;
const MAGIC_VAL_MASK: u32 = 0xFF000000;
const MAX_MSG_ID_ENTRIES: usize = 256;
const MAILBOX_NAME: &[u8] = b"xdna_mailbox\0";
const MSG_PROTOCOL_VERSION: u32 = 0x1;
const TOMBSTONE: u32 = 0xDEADFACE;
const CHAN_RES_X2I: usize = 0;
const CHAN_RES_I2X: usize = 1;
const CHAN_RES_NUM: usize = 2;
const MSG_BODY_SZ: u32 = 0x7ff;
const MSG_PROTO_VER: u32 = 0xff0000;
const EINVAL: i32 = 22;
const ENOENT: i32 = 2;
const EPIPE: i32 = 32;
const ENOMEM: i32 = 12;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct drm_device { pub dev: *mut device }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }

#[repr(C)] pub struct xdna_mailbox_res { pub mbox_base: *mut u8, pub ringbuf_base: *mut u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct xdna_mailbox_chann_res {
    pub mb_head_ptr_reg: u32, pub mb_tail_ptr_reg: u32,
    pub rb_size: u32, pub rb_start_addr: u32,
}
#[repr(C)] pub struct xdna_mailbox_msg {
    pub handle: *mut c_void,
    pub notify_cb: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> i32>,
    pub send_size: usize, pub send_data: *const u8, pub opcode: u32,
}

#[repr(C)] pub struct mailbox { pub dev: *mut device, pub res: xdna_mailbox_res }
#[repr(C)] pub struct mailbox_channel {
    pub mb: *mut mailbox, pub res: [xdna_mailbox_chann_res; CHAN_RES_NUM],
    pub msix_irq: i32, pub iohub_int_addr: u32, pub chan_xa: xarray,
    pub next_msgid: u32, pub x2i_tail: u32, pub work_q: *mut workqueue_struct,
    pub rx_work: work_struct, pub i2x_head: u32, pub bad_state: bool,
}
#[repr(C, packed)] #[derive(Copy, Clone)] pub struct xdna_msg_header {
    pub total_size: u32, pub sz_ver: u32, pub id: u32, pub opcode: u32,
}
#[repr(C)] pub struct mailbox_pkg { pub header: xdna_msg_header, pub payload: [u32; 0] }
#[repr(C)] pub struct mailbox_msg {
    pub handle: *mut c_void,
    pub notify_cb: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> i32>,
    pub pkg_size: usize, pub pkg: mailbox_pkg,
}

extern "C" {
    fn readl(addr: *mut u8) -> u32; fn writel(v: u32, addr: *mut u8);
    fn memcpy_toio(dst: *mut u8, src: *const c_void, n: usize);
    fn memcpy_fromio(dst: *mut c_void, src: *mut u8, n: usize);
    fn xa_alloc_cyclic_irq(xa: *mut xarray, id: *mut u32, entry: *mut mailbox_msg, limit: u64, next: *mut u32, gfp: u32) -> i32;
    fn xa_erase_irq(xa: *mut xarray, id: u32) -> *mut mailbox_msg;
    fn xa_init_flags(xa: *mut xarray, flags: u32); fn xa_destroy(xa: *mut xarray);
    fn kfree(p: *mut c_void); fn kzalloc(n: usize, gfp: u32) -> *mut c_void;
    fn drmm_kzalloc(d: *mut drm_device, n: usize, gfp: u32) -> *mut c_void;
    fn create_singlethread_workqueue(name: *const u8) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct); fn drain_workqueue(wq: *mut workqueue_struct);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32,*mut c_void)->i32, flags: u32, name: *const u8, data: *mut c_void) -> i32;
    fn free_irq(irq: i32, data: *mut c_void);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn init_work(work: *mut work_struct, f: unsafe extern "C" fn(*mut work_struct));
    fn trace_mbox_set_tail(name:*const u8, irq:i32, opcode:u32, id:u32);
    fn trace_mbox_set_head(name:*const u8, irq:i32, opcode:u32, id:u32);
    fn trace_mbox_irq_handle(name:*const u8, irq:i32); fn trace_mbox_rx_worker(name:*const u8, irq:i32);
}

unsafe fn mailbox_reg_write(c: *mut mailbox_channel, r:u32, d:u32) { writel(d, (*(*c).mb).res.mbox_base.add(r as usize)); }
unsafe fn mailbox_reg_read(c:*mut mailbox_channel,r:u32)->u32 { readl((*(*c).mb).res.mbox_base.add(r as usize)) }
unsafe fn mailbox_irq_acknowledge(c:*mut mailbox_channel) { if (*c).iohub_int_addr != 0 { mailbox_reg_write(c,(*c).iohub_int_addr,0); } }
unsafe fn mailbox_irq_status(c:*mut mailbox_channel)->u32 { if (*c).iohub_int_addr != 0 { mailbox_reg_read(c,(*c).iohub_int_addr) } else { 0 } }
unsafe fn mailbox_set_headptr(c:*mut mailbox_channel,v:u32){ mailbox_reg_write(c,(*c).res[CHAN_RES_I2X].mb_head_ptr_reg,v);(*c).i2x_head=v; }
unsafe fn mailbox_set_tailptr(c:*mut mailbox_channel,v:u32){ mailbox_reg_write(c,(*c).res[CHAN_RES_X2I].mb_tail_ptr_reg,v);(*c).x2i_tail=v; }
unsafe fn mailbox_get_headptr(c:*mut mailbox_channel,t:usize)->u32{mailbox_reg_read(c,(*c).res[t].mb_head_ptr_reg)}
unsafe fn mailbox_get_tailptr(c:*mut mailbox_channel,t:usize)->u32{mailbox_reg_read(c,(*c).res[t].mb_tail_ptr_reg)}
unsafe fn mailbox_get_ringbuf_size(c:*mut mailbox_channel,t:usize)->u32{(*c).res[t].rb_size}
fn mailbox_validate_msgid(id:i32)->bool{(id as u32 & MAGIC_VAL_MASK)==MAGIC_VAL}

pub unsafe extern "C" fn xdna_mailbox_send_msg(c:*mut mailbox_channel,msg:*const xdna_mailbox_msg,_tx_timeout:u64)->i32 {
    let pkg_size=size_of::<xdna_msg_header>()+(*msg).send_size;
    if pkg_size>mailbox_get_ringbuf_size(c,CHAN_RES_X2I) as usize || (*msg).send_size%4!=0 { return -EINVAL; }
    if (*msg).send_size>=4 && *( (*msg).send_data as *const u32)==TOMBSTONE { return -EINVAL; }
    if (*c).bad_state{return -EPIPE;}
    let p=kzalloc(size_of::<mailbox_msg>()+pkg_size,0) as *mut mailbox_msg; if p.is_null(){return -ENOMEM;}
    (*p).handle=(*msg).handle;(*p).notify_cb=(*msg).notify_cb;(*p).pkg_size=pkg_size;
    (*p).pkg.header=xdna_msg_header{total_size:(*msg).send_size as u32,sz_ver:((*msg).send_size as u32&MSG_BODY_SZ)|(MSG_PROTOCOL_VERSION<<16),id:0,opcode:(*msg).opcode};
    memcpy_toio((*p).pkg.payload.as_mut_ptr() as *mut u8,(*msg).send_data as *const c_void,(*msg).send_size);
    let mut id=0u32; let ret=xa_alloc_cyclic_irq(&mut (*c).chan_xa,&mut id,p,255,&mut (*c).next_msgid,0);
    if ret<0 { kfree(p as *mut c_void); return ret; }
    (*p).pkg.header.id=id|MAGIC_VAL;
    let x=mailbox_get_headptr(c,CHAN_RES_X2I); let mut t=(*c).x2i_tail;
    let sz=mailbox_get_ringbuf_size(c,CHAN_RES_X2I)-4; let start=(*c).res[CHAN_RES_X2I].rb_start_addr;
    let mut nt=t+pkg_size as u32;
    if t>=x && nt>sz { writel(TOMBSTONE,(*(*c).mb).res.ringbuf_base.add(start as usize+t as usize));t=0;nt=pkg_size as u32; }
    if t<x && nt>=x { kfree(p as *mut c_void); return -EINVAL; }
    memcpy_toio((*(*c).mb).res.ringbuf_base.add(start as usize+t as usize),&(*p).pkg as *const mailbox_pkg as *const c_void,pkg_size);
    mailbox_set_tailptr(c,t+pkg_size as u32); trace_mbox_set_tail(MAILBOX_NAME.as_ptr(),(*c).msix_irq,(*p).pkg.header.opcode,(*p).pkg.header.id); 0
}

pub unsafe extern "C" fn xdna_mailbox_alloc_channel(mb:*mut mailbox)->*mut mailbox_channel { let c=kzalloc(size_of::<mailbox_channel>(),0) as *mut mailbox_channel;if c.is_null(){return ptr::null_mut();}(*c).mb=mb;(*c).work_q=create_singlethread_workqueue(MAILBOX_NAME.as_ptr());if (*c).work_q.is_null(){kfree(c as *mut c_void);return ptr::null_mut();}c }
pub unsafe extern "C" fn xdna_mailbox_free_channel(c:*mut mailbox_channel){if c.is_null(){return;}destroy_workqueue((*c).work_q);kfree(c as *mut c_void);}
pub unsafe extern "C" fn xdna_mailbox_start_channel(c:*mut mailbox_channel,x:*const xdna_mailbox_chann_res,i:*const xdna_mailbox_chann_res,addr:u32,irq:i32)->i32{(*c).msix_irq=irq;(*c).iohub_int_addr=addr;ptr::copy_nonoverlapping(x,&mut (*c).res[0],1);ptr::copy_nonoverlapping(i,&mut (*c).res[1],1);xa_init_flags(&mut (*c).chan_xa,0);(*c).x2i_tail=mailbox_get_tailptr(c,0);(*c).i2x_head=mailbox_get_headptr(c,1);mailbox_irq_acknowledge(c);0}
pub unsafe extern "C" fn xdna_mailbox_stop_channel(c:*mut mailbox_channel){if c.is_null(){return;}free_irq((*c).msix_irq,c as *mut c_void);drain_workqueue((*c).work_q);xa_destroy(&mut (*c).chan_xa);}

pub unsafe extern "C" fn xdnam_mailbox_create(d:*mut drm_device,res:*const xdna_mailbox_res)->*mut mailbox { let m=drmm_kzalloc(d,size_of::<mailbox>(),0) as *mut mailbox;if m.is_null(){return ptr::null_mut();}(*m).dev=(*d).dev;ptr::copy_nonoverlapping(res,&mut (*m).res,1);m }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
