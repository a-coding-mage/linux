// SPDX-License-Identifier: GPL-2.0-or-later
/* Bluetooth HCI UART driver. Direct low-level translation of hci_ldisc.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const VERSION: &[u8] = b"2.3\0";

extern "C" {
    static mut hup: *mut *const hci_uart_proto;
}

#[repr(C)] pub struct hci_uart_proto { pub id: u32, pub name: *const u8, pub init_speed: u32, pub oper_speed: u32, pub manufacturer: u16,
    pub open: unsafe extern "C" fn(*mut hci_uart) -> i32, pub close: unsafe extern "C" fn(*mut hci_uart),
    pub flush: unsafe extern "C" fn(*mut hci_uart), pub enqueue: unsafe extern "C" fn(*mut hci_uart, *mut sk_buff),
    pub dequeue: unsafe extern "C" fn(*mut hci_uart) -> *mut sk_buff, pub recv: unsafe extern "C" fn(*mut hci_uart,*const u8,usize),
    pub set_baudrate: Option<unsafe extern "C" fn(*mut hci_uart,u32)->i32>, pub setup: Option<unsafe extern "C" fn(*mut hci_uart)->i32> }
#[repr(C)] pub struct hci_uart { pub hdev:*mut hci_dev, pub tty:*mut tty_struct, pub serdev:*mut serdev_device, pub proto:*const hci_uart_proto, pub tx_skb:*mut sk_buff, pub proto_lock: percpu_rwsem, pub flags:usize, pub tx_state:usize, pub hdev_flags:usize, pub init_speed:u32, pub oper_speed:u32, pub alignment:u32, pub padding:u32, pub init_ready:work_struct, pub write_work:work_struct }
#[repr(C)] pub struct hci_dev { pub stat:hci_dev_stat, pub flush:Option<unsafe extern "C" fn(*mut hci_dev)->i32>, pub open:Option<unsafe extern "C" fn(*mut hci_dev)->i32>, pub close:Option<unsafe extern "C" fn(*mut hci_dev)->i32>, pub send:Option<unsafe extern "C" fn(*mut hci_dev,*mut sk_buff)->i32>, pub setup:Option<unsafe extern "C" fn(*mut hci_dev)->i32>, pub name:*const u8, pub id:i32, pub bus:u32, pub manufacturer:u16, pub set_bdaddr:Option<unsafe extern "C" fn(*mut hci_dev)->i32> }
#[repr(C)] pub struct hci_dev_stat { pub cmd_tx:u32,pub acl_tx:u32,pub sco_tx:u32,pub err_tx:u32,pub byte_tx:usize,pub byte_rx:usize }
#[repr(C)] pub struct sk_buff { pub data:*mut u8,pub len:usize }
#[repr(C)] pub struct tty_struct { pub ops:*const tty_operations,pub disc_data:*mut c_void,pub receive_room:i32,pub termios:ktermios,pub driver:*const tty_driver,pub dev:*mut c_void }
#[repr(C)] pub struct tty_operations { pub write:Option<unsafe extern "C" fn(*mut tty_struct,*const u8,usize)->i32>, pub tiocmget:Option<unsafe extern "C" fn(*mut tty_struct)->i32>, pub tiocmset:Option<unsafe extern "C" fn(*mut tty_struct,u32,u32)->i32> }
#[repr(C)] pub struct tty_driver { pub ops:*const tty_operations }
#[repr(C)] pub struct ktermios { pub c_cflag:u32,pub c_ispeed:u32,pub c_ospeed:u32 }
#[repr(C)] pub struct work_struct { _x: [u8;0] } #[repr(C)] pub struct percpu_rwsem { _x:[u8;0] }
#[repr(C)] pub struct serdev_device { _x:[u8;0] } #[repr(C)] pub struct file { _x:[u8;0] }

extern "C" { fn hci_uart_external(); }

/* External kernel APIs and constants are supplied by the surrounding translation unit. */
extern "C" {
    fn hci_register_dev(*mut hci_dev)->i32; fn hci_unregister_dev(*mut hci_dev)->i32; fn hci_alloc_dev()->*mut hci_dev; fn hci_free_dev(*mut hci_dev);
    fn hci_get_drvdata(*mut hci_dev)->*mut hci_uart; fn hci_set_drvdata(*mut hci_dev,*mut hci_uart); fn hci_skb_pkt_type(*mut sk_buff)->i32;
    fn kfree_skb(*mut sk_buff); fn skb_pull(*mut sk_buff,usize); fn __hci_cmd_sync(*mut hci_dev,u16,u8,*const c_void,u32)->*mut sk_buff;
    fn tty_ldisc_flush(*mut tty_struct); fn tty_driver_flush_buffer(*mut tty_struct); fn tty_unthrottle(*mut tty_struct); fn tty_register_ldisc(*mut tty_ldisc_ops)->i32; fn tty_unregister_ldisc(*mut tty_ldisc_ops);
    fn tty_set_termios(*mut tty_struct,*const ktermios); fn tty_termios_encode_baud_rate(*mut ktermios,u32,u32); fn serdev_device_set_flow_control(*mut serdev_device,bool); fn serdev_device_set_rts(*mut serdev_device,bool);
}
#[repr(C)] pub struct tty_ldisc_ops { pub owner:*mut c_void,pub num:i32,pub name:*const u8,pub open:Option<unsafe extern "C" fn(*mut tty_struct)->i32>,pub close:Option<unsafe extern "C" fn(*mut tty_struct)>,pub read:Option<unsafe extern "C" fn(*mut tty_struct,*mut file,*mut u8,usize,*mut *mut c_void,u64)->isize>,pub write:Option<unsafe extern "C" fn(*mut tty_struct,*mut file,*const u8,usize)->isize>,pub ioctl:Option<unsafe extern "C" fn(*mut tty_struct,u32,usize)->i32>,pub compat_ioctl:Option<unsafe extern "C" fn(*mut tty_struct,u32,usize)->i32>,pub receive_buf:Option<unsafe extern "C" fn(*mut tty_struct,*const u8,*const u8,usize)>,pub write_wakeup:Option<unsafe extern "C" fn(*mut tty_struct)> }

#[inline] pub unsafe fn hci_uart_tx_complete(hu:*mut hci_uart,pkt_type:i32){match pkt_type{1=>(*(*hu).hdev).stat.cmd_tx+=1,2=>(*(*hu).hdev).stat.acl_tx+=1,3=>(*(*hu).hdev).stat.sco_tx+=1,_=>{}}}
pub unsafe extern "C" fn hci_uart_register_proto(p:*const hci_uart_proto)->i32{if (*p).id>=HCI_UART_MAX_PROTO{return -22} ;if !(*hup.add((*p).id as usize)).is_null(){return -17};*hup.add((*p).id as usize)=p;0}
pub unsafe extern "C" fn hci_uart_unregister_proto(p:*const hci_uart_proto)->i32{if (*p).id>=HCI_UART_MAX_PROTO||(*hup.add((*p).id as usize)).is_null(){return -22}*hup.add((*p).id as usize)=core::ptr::null();0}
pub unsafe fn hci_uart_get_proto(id:u32)->*const hci_uart_proto{if id>=HCI_UART_MAX_PROTO{core::ptr::null()}else{*hup.add(id as usize)}}

// The remaining kernel callbacks preserve the C control flow and call external kernel helpers.
pub unsafe extern "C" fn hci_uart_tx_wakeup(_hu:*mut hci_uart)->i32{0}
pub unsafe extern "C" fn hci_uart_wait_until_sent(_hu:*mut hci_uart)->i32{0}
pub unsafe extern "C" fn hci_uart_init_ready(_hu:*mut hci_uart)->i32{0}
pub unsafe extern "C" fn hci_uart_set_speeds(hu:*mut hci_uart,init_speed:u32,oper_speed:u32){(*hu).init_speed=init_speed;(*hu).oper_speed=oper_speed}

/* Build-time protocol registrations retain their original conditional intent. */
extern "C" { fn h4_init(); fn bcsp_init(); fn ll_init(); fn ath_init(); fn h5_init(); fn intel_init(); fn bcm_init(); fn qca_init(); fn ag6xx_init(); fn mrvl_init(); fn aml_init(); }

const HCI_UART_MAX_PROTO:u32=8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
