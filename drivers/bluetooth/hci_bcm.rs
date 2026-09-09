// SPDX-License-Identifier: GPL-2.0-or-later
// Bluetooth HCI UART driver for Broadcom devices.
// C headers and kernel-provided symbols are intentionally external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

const BCM_NULL_PKT: u8 = 0x00;
const BCM_NULL_SIZE: usize = 0;
const BCM_LM_DIAG_PKT: u8 = 0x07;
const BCM_LM_DIAG_SIZE: usize = 63;
const BCM_TYPE49_PKT: u8 = 0x31;
const BCM_TYPE49_SIZE: usize = 0;
const BCM_TYPE52_PKT: u8 = 0x34;
const BCM_TYPE52_SIZE: usize = 0;
const BCM_AUTOSUSPEND_DELAY: u32 = 5000;
const BCM_NUM_SUPPLIES: usize = 2;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct serdev_device { pub dev: device }
#[repr(C)] pub struct tty_struct { pub dev: *mut device }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff_head { _private: [u8; 0] }
#[repr(C)] pub struct hci_dev { pub flags: [usize; 1], pub set_diag: Option<unsafe extern "C" fn(*mut hci_dev, bool) -> c_int>, pub set_bdaddr: *mut c_void }
#[repr(C)] pub struct hci_uart { pub serdev: *mut serdev_device, pub tty: *mut tty_struct, pub hdev: *mut hci_dev, pub priv_: *mut bcm_data, pub flags: [usize; 1], pub init_speed: u32, pub oper_speed: u32, pub proto: *const hci_uart_proto }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct regulator_bulk_data { pub supply: *const c_char }
#[repr(C)] pub struct platform_device { pub dev: device }

#[repr(C)] pub struct bcm_device_data { pub no_early_set_baudrate: bool, pub drive_rts_on_open: bool, pub no_uart_clock_set: bool, pub max_autobaud_speed: u32, pub max_speed: u32 }
#[repr(C)] pub struct bcm_device {
    pub serdev_hu: hci_uart, pub list: list_head, pub dev: *mut device, pub name: *const c_char,
    pub device_wakeup: *mut gpio_desc, pub shutdown: *mut gpio_desc, pub reset: *mut gpio_desc,
    pub set_device_wakeup: Option<unsafe extern "C" fn(*mut bcm_device, bool) -> c_int>,
    pub set_shutdown: Option<unsafe extern "C" fn(*mut bcm_device, bool) -> c_int>,
    pub txco_clk: *mut clk, pub lpo_clk: *mut clk, pub supplies: [regulator_bulk_data; BCM_NUM_SUPPLIES],
    pub res_enabled: bool, pub init_speed: u32, pub oper_speed: u32, pub irq: c_int,
    pub irq_active_low: bool, pub irq_acquired: bool, pub hu: *mut hci_uart, pub is_suspended: bool,
    pub no_early_set_baudrate: bool, pub drive_rts_on_open: bool, pub no_uart_clock_set: bool,
    pub use_autobaud_mode: bool, pub pcm_int_params: [u8; 5], pub max_autobaud_speed: u32,
}
#[repr(C)] pub struct bcm_data { pub rx_skb: *mut sk_buff, pub txq: sk_buff_head, pub dev: *mut bcm_device }
#[repr(C)] pub struct bcm_update_uart_baud_rate { pub zero: u16, pub baud_rate: u32 }
#[repr(C)] pub struct bcm_write_uart_clock_setting { pub type_: u8 }
#[repr(C)] pub struct bcm_set_sleep_mode { pub sleep_mode: u8, pub idle_host: u8, pub idle_dev: u8, pub bt_wake_active: u8, pub host_wake_active: u8, pub allow_host_sleep: u8, pub combine_modes: u8, pub tristate_control: u8, pub usb_auto_sleep: u8, pub usb_resume_timeout: u8, pub break_to_host: u8, pub pulsed_host_wake: u8 }
#[repr(C)] pub struct bcm_set_pcm_int_params { pub data: [u8; 5] }
#[repr(C)] pub struct h4_recv_pkt { pub typ: u8, pub hlen: u8, pub loff: u8, pub lsize: u8, pub maxlen: usize, pub recv: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct hci_uart_proto { pub id: c_int, pub name: *const c_char, pub manufacturer: c_int, pub init_speed: u32, pub open: Option<unsafe extern "C" fn(*mut hci_uart)->c_int>, pub close: Option<unsafe extern "C" fn(*mut hci_uart)->c_int>, pub flush: Option<unsafe extern "C" fn(*mut hci_uart)->c_int>, pub setup: Option<unsafe extern "C" fn(*mut hci_uart)->c_int>, pub set_baudrate: Option<unsafe extern "C" fn(*mut hci_uart,u32)->c_int>, pub recv: Option<unsafe extern "C" fn(*mut hci_uart,*const c_void,c_int)->c_int>, pub enqueue: Option<unsafe extern "C" fn(*mut hci_uart,*mut sk_buff)->c_int>, pub dequeue: Option<unsafe extern "C" fn(*mut hci_uart)->*mut sk_buff> }

extern "C" { fn hci_uart_set_baudrate(hu:*mut hci_uart,speed:u32); fn serdev_device_set_baudrate(s:*mut serdev_device,speed:u32); fn __hci_cmd_sync(h:*mut hci_dev,opcode:u16,len:usize,param:*const c_void,timeout:u32)->*mut sk_buff; fn kfree_skb(s:*mut sk_buff); fn btbcm_initialize(h:*mut hci_dev,done:*mut bool,auto_:bool)->c_int; fn btbcm_finalize(h:*mut hci_dev,done:*mut bool,auto_:bool)->c_int; fn btbcm_set_bdaddr(); fn hci_get_drvdata(h:*mut hci_dev)->*mut hci_uart; fn hci_uart_has_flow_control(h:*mut hci_uart)->bool; fn hci_uart_set_flow_control(h:*mut hci_uart,on:bool); fn hci_uart_tx_wakeup(h:*mut hci_uart); fn h4_recv_buf(hu:*mut hci_uart,skb:*mut sk_buff,data:*const c_void,count:c_int,p:*const h4_recv_pkt,n:usize)->*mut sk_buff; fn hci_recv_frame(); fn hci_recv_diag(); fn skb_queue_tail(q:*mut sk_buff_head,s:*mut sk_buff); fn skb_queue_purge(q:*mut sk_buff_head); fn skb_dequeue(q:*mut sk_buff_head)->*mut sk_buff; fn skb_put_u8(s:*mut sk_buff,v:u8); fn bt_skb_alloc(n:usize,gfp:u32)->*mut sk_buff; fn skb_push(s:*mut sk_buff,n:usize)->*mut u8; fn hci_uart_register_proto(p:*const hci_uart_proto)->c_int; fn hci_uart_unregister_proto(p:*const hci_uart_proto)->c_int; }

static mut BCM_DEVICE_LOCK: mutex = mutex { _private: [] };
static mut BCM_DEVICE_LIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn host_set_baudrate(hu:*mut hci_uart, speed:u32) { if !(*hu).serdev.is_null() { serdev_device_set_baudrate((*hu).serdev,speed) } else { hci_uart_set_baudrate(hu,speed) } }
unsafe extern "C" fn bcm_set_baudrate(hu:*mut hci_uart,speed:u32)->c_int {
    let bcm=(*hu).priv_; let h=(*hu).hdev; let mut skb;
    if speed>3_000_000 && !(*(*bcm).dev).no_uart_clock_set { let clock=bcm_write_uart_clock_setting{type_: 1}; skb=__hci_cmd_sync(h,0xfc45,1,&clock as *const _ as *const c_void,0); if skb as isize < 0 { return skb as c_int; } kfree_skb(skb); }
    let param=bcm_update_uart_baud_rate{zero:0,baud_rate:speed.to_le()}; skb=__hci_cmd_sync(h,0xfc18,core::mem::size_of::<bcm_update_uart_baud_rate>(),&param as *const _ as *const c_void,0); if skb as isize < 0 { return skb as c_int; } kfree_skb(skb); 0
}
unsafe extern "C" fn bcm_set_diag(hdev:*mut hci_dev,enable:bool)->c_int { let hu=hci_get_drvdata(hdev); let bcm=(*hu).priv_; let skb=bt_skb_alloc(3,0); if skb.is_null(){return -12}; skb_put_u8(skb,BCM_LM_DIAG_PKT); skb_put_u8(skb,0xf0); skb_put_u8(skb,enable as u8); skb_queue_tail(&mut (*bcm).txq,skb); hci_uart_tx_wakeup(hu); 0 }
unsafe extern "C" fn bcm_open(hu:*mut hci_uart)->c_int { if !hci_uart_has_flow_control(hu){return -95}; let bcm=Box::into_raw(Box::new(core::mem::zeroed::<bcm_data>())); (*hu).priv_=bcm; 0 }
unsafe extern "C" fn bcm_close(hu:*mut hci_uart)->c_int { let bcm=(*hu).priv_; if !bcm.is_null(){skb_queue_purge(&mut (*bcm).txq); kfree_skb((*bcm).rx_skb); drop(Box::from_raw(bcm)); (*hu).priv_=core::ptr::null_mut();} 0 }
unsafe extern "C" fn bcm_flush(hu:*mut hci_uart)->c_int { skb_queue_purge(&mut (*(*hu).priv_).txq); 0 }
unsafe extern "C" fn bcm_setup(hu:*mut hci_uart)->c_int { let bcm=(*hu).priv_; (*(*hu).hdev).set_diag=Some(bcm_set_diag); (*(*hu).hdev).set_bdaddr=btbcm_set_bdaddr as *mut c_void; let mut done=false; let auto_=!bcm.is_null() && !(*bcm).dev.is_null() && (*(*bcm).dev).use_autobaud_mode; let e=btbcm_initialize((*hu).hdev,&mut done,auto_); if e!=0{return e}; if !done{return 0}; if (*hu).oper_speed!=0 { let s=(*hu).oper_speed; let e=bcm_set_baudrate(hu,s); if e==0{host_set_baudrate(hu,s)} }; btbcm_finalize((*hu).hdev,&mut done,auto_) }
unsafe extern "C" fn bcm_recv(hu:*mut hci_uart,data:*const c_void,count:c_int)->c_int { let bcm=(*hu).priv_; (*bcm).rx_skb=h4_recv_buf(hu,(*bcm).rx_skb,data,count,core::ptr::null(),0); count }
unsafe extern "C" fn bcm_enqueue(hu:*mut hci_uart,skb:*mut sk_buff)->c_int { skb_queue_tail(&mut (*(*hu).priv_).txq,skb); 0 }
unsafe extern "C" fn bcm_dequeue(hu:*mut hci_uart)->*mut sk_buff { skb_dequeue(&mut (*(*hu).priv_).txq) }

static BCM_PROTO:hci_uart_proto=hci_uart_proto{id:0,name:b"Broadcom\0".as_ptr() as _,manufacturer:15,init_speed:115200,open:Some(bcm_open),close:Some(bcm_close),flush:Some(bcm_flush),setup:Some(bcm_setup),set_baudrate:Some(bcm_set_baudrate),recv:Some(bcm_recv),enqueue:Some(bcm_enqueue),dequeue:Some(bcm_dequeue)};
#[no_mangle] pub unsafe extern "C" fn bcm_init()->c_int { hci_uart_register_proto(&BCM_PROTO) }
#[no_mangle] pub unsafe extern "C" fn bcm_deinit()->c_int { hci_uart_unregister_proto(&BCM_PROTO) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
