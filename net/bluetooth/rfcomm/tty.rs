// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of bluetooth/rfcomm/tty.c. */
use core::ffi::c_void;

pub const RFCOMM_TTY_PORTS: u32 = RFCOMM_MAX_DEV;
pub const RFCOMM_TTY_MAJOR: u32 = 216;
pub const RFCOMM_TTY_MINOR: u32 = 0;

#[repr(C)]
pub struct rfcomm_dev {
    pub port: tty_port, pub list: list_head, pub name: [u8; 12],
    pub id: i32, pub flags: usize, pub err: i32, pub status: usize,
    pub src: bdaddr_t, pub dst: bdaddr_t, pub channel: u8,
    pub modem_status: u32, pub dlc: *mut rfcomm_dlc, pub tty_dev: *mut device,
    pub wmem_alloc: atomic_t, pub pending: sk_buff_head,
}

/* Includes and symbols from Linux and Bluetooth headers are external dependencies. */
extern "C" {
    static mut rfcomm_tty_driver: *mut tty_driver;
    static mut rfcomm_dev_list: list_head;
    fn rfcomm_dlc_lock(*mut rfcomm_dlc); fn rfcomm_dlc_unlock(*mut rfcomm_dlc);
    fn rfcomm_dlc_put(*mut rfcomm_dlc); fn rfcomm_dlc_open(*mut rfcomm_dlc,*mut bdaddr_t,*mut bdaddr_t,u8)->i32;
    fn rfcomm_dlc_close(*mut rfcomm_dlc,i32); fn rfcomm_dlc_throttle(*mut rfcomm_dlc); fn rfcomm_dlc_unthrottle(*mut rfcomm_dlc);
    fn tty_port_tty_hangup(*mut tty_port,bool); fn tty_flip_buffer_push(*mut tty_port);
    fn tty_insert_flip_string(*mut tty_port,*const u8,usize)->i32; fn kfree_skb(*mut sk_buff);
    fn skb_queue_empty(*mut sk_buff_head)->bool; fn skb_queue_tail(*mut sk_buff_head,*mut sk_buff);
    fn atomic_dec(*mut atomic_t); fn tty_port_tty_wakeup(*mut tty_port);
}

#[repr(C)] pub struct tty_port { pub ops:*const tty_port_operations, pub count:i32, pub open_wait:wait_queue_head }
#[repr(C)] pub struct tty_port_operations { pub destruct:Option<unsafe extern "C" fn(*mut tty_port)>, pub activate:Option<unsafe extern "C" fn(*mut tty_port,*mut tty_struct)->i32>, pub shutdown:Option<unsafe extern "C" fn(*mut tty_port)>, pub carrier_raised:Option<unsafe extern "C" fn(*mut tty_port)->bool> }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct bdaddr_t { pub b:[u8;6] }
#[repr(C)] pub struct atomic_t { pub counter:i32 }
#[repr(C)] pub struct sk_buff_head { pub opaque:[usize;8] }
#[repr(C)] pub struct wait_queue_head { pub opaque:[usize;4] }
#[repr(C)] pub struct tty_driver { pub opaque:[usize;32] }
#[repr(C)] pub struct tty_struct { pub driver_data:*mut c_void, pub index:i32 }
#[repr(C)] pub struct device { pub parent:*mut device }
#[repr(C)] pub struct rfcomm_dlc { pub owner:*mut rfcomm_dev, pub state:i32, pub mtu:usize, pub remote_v24_sig:u8, pub opaque:[usize;8] }
#[repr(C)] pub struct sk_buff { pub data:*mut u8, pub len:usize, pub sk:*mut c_void }
#[repr(C)] pub struct sock { pub sk_state:i32, pub sk_receive_queue:sk_buff_head, pub sk_rmem_alloc:atomic_t }

pub unsafe extern "C" fn rfcomm_dev_data_ready(dlc:*mut rfcomm_dlc, skb:*mut sk_buff) {
    let dev=(*dlc).owner; if dev.is_null(){ kfree_skb(skb); return; }
    if !skb_queue_empty(&mut (*dev).pending) { skb_queue_tail(&mut (*dev).pending,skb); return; }
    tty_insert_flip_string(&mut (*dev).port,(*skb).data,(*skb).len);
    tty_flip_buffer_push(&mut (*dev).port); kfree_skb(skb);
}
pub unsafe extern "C" fn rfcomm_dev_state_change(dlc:*mut rfcomm_dlc, err:i32) {
    let dev=(*dlc).owner; if dev.is_null(){return;} (*dev).err=err;
    if (*dlc).state==BT_CONNECTED { rfcomm_reparent_device(dev); }
    else if (*dlc).state==BT_CLOSED { tty_port_tty_hangup(&mut (*dev).port,false); }
}
pub unsafe extern "C" fn rfcomm_dev_modem_status(dlc:*mut rfcomm_dlc, sig:u8) {
    let dev=(*dlc).owner; if dev.is_null(){return;}
    (*dev).modem_status=(((sig&RFCOMM_V24_RTC)!=0) as u32*TIOCM_DSR)
        |(((sig&RFCOMM_V24_RTR)!=0) as u32*TIOCM_CTS)
        |(((sig&RFCOMM_V24_IC)!=0) as u32*TIOCM_RI)
        |(((sig&RFCOMM_V24_DV)!=0) as u32*TIOCM_CD);
}
unsafe fn rfcomm_dev_destruct(port:*mut tty_port) {
    let dev=(port as *mut rfcomm_dev); let dlc=(*dev).dlc;
    rfcomm_dlc_lock(dlc); if (*dlc).owner==dev {(*dlc).owner=core::ptr::null_mut();} rfcomm_dlc_unlock(dlc);
    rfcomm_dlc_put(dlc);
}
unsafe fn rfcomm_dev_activate(port:*mut tty_port, _tty:*mut tty_struct)->i32 { rfcomm_dlc_open((*port as *mut rfcomm_dev).as_mut().unwrap().dlc,core::ptr::null_mut(),core::ptr::null_mut(),0) }
unsafe fn rfcomm_dev_shutdown(port:*mut tty_port) { rfcomm_dlc_close((*port as *mut rfcomm_dev).as_mut().unwrap().dlc,0); }
unsafe fn rfcomm_dev_carrier_raised(port:*mut tty_port)->bool { (*((*port) as *mut rfcomm_dev)).dlc.as_ref().unwrap().state==BT_CONNECTED }
pub unsafe extern "C" fn rfcomm_dev_ioctl(_sk:*mut sock,_cmd:u32,_arg:*mut c_void)->i32 { -EINVAL }
pub unsafe extern "C" fn rfcomm_init_ttys()->i32 { 0 }
pub unsafe extern "C" fn rfcomm_cleanup_ttys() {}
pub unsafe extern "C" fn rfcomm_tty_throttle(tty:*mut tty_struct) { let d=(*tty).driver_data as *mut rfcomm_dev; rfcomm_dlc_throttle((*d).dlc); }
pub unsafe extern "C" fn rfcomm_tty_unthrottle(tty:*mut tty_struct) { let d=(*tty).driver_data as *mut rfcomm_dev; rfcomm_dlc_unthrottle((*d).dlc); }
pub unsafe extern "C" fn rfcomm_wfree(_skb:*mut sk_buff) { }
const EINVAL:i32=22; const TIOCM_DSR:u32=0x100; const TIOCM_CTS:u32=0x20; const TIOCM_RI:u32=0x80; const TIOCM_CD:u32=0x40;
const RFCOMM_V24_RTC:u8=1; const RFCOMM_V24_RTR:u8=2; const RFCOMM_V24_IC:u8=4; const RFCOMM_V24_DV:u8=8;
const BT_CONNECTED:i32=1; const BT_CLOSED:i32=0; const RFCOMM_MAX_DEV:u32=256;
extern "C" { fn rfcomm_reparent_device(*mut rfcomm_dev); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
