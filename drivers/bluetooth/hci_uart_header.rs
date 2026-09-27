/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Bluetooth HCI UART driver declarations. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* #ifndef N_HCI / #define N_HCI 15 */
pub const N_HCI: ::kernel::ffi::c_int = 15;

/* Ioctl encodings depend on the platform _IOW/_IOR definitions. */
/* HCIUARTSETPROTO = _IOW('U', 200, int) */
/* HCIUARTGETPROTO = _IOR('U', 201, int) */
/* HCIUARTGETDEVICE = _IOR('U', 202, int) */
/* HCIUARTSETFLAGS = _IOW('U', 203, int) */
/* HCIUARTGETFLAGS = _IOR('U', 204, int) */

pub const HCI_UART_MAX_PROTO: ::kernel::ffi::c_uint = 13;
pub const HCI_UART_H4: ::kernel::ffi::c_uint = 0;
pub const HCI_UART_BCSP: ::kernel::ffi::c_uint = 1;
pub const HCI_UART_3WIRE: ::kernel::ffi::c_uint = 2;
pub const HCI_UART_H4DS: ::kernel::ffi::c_uint = 3;
pub const HCI_UART_LL: ::kernel::ffi::c_uint = 4;
pub const HCI_UART_ATH3K: ::kernel::ffi::c_uint = 5;
pub const HCI_UART_INTEL: ::kernel::ffi::c_uint = 6;
pub const HCI_UART_BCM: ::kernel::ffi::c_uint = 7;
pub const HCI_UART_QCA: ::kernel::ffi::c_uint = 8;
pub const HCI_UART_AG6XX: ::kernel::ffi::c_uint = 9;
pub const HCI_UART_NOKIA: ::kernel::ffi::c_uint = 10;
pub const HCI_UART_MRVL: ::kernel::ffi::c_uint = 11;
pub const HCI_UART_AML: ::kernel::ffi::c_uint = 12;

pub const HCI_UART_RAW_DEVICE: ::kernel::ffi::c_uint = 0;
pub const HCI_UART_RESET_ON_INIT: ::kernel::ffi::c_uint = 1;
pub const HCI_UART_INIT_PENDING: ::kernel::ffi::c_uint = 3;
pub const HCI_UART_EXT_CONFIG: ::kernel::ffi::c_uint = 4;
pub const HCI_UART_VND_DETECT: ::kernel::ffi::c_uint = 5;

#[repr(C)]
pub struct serdev_device {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct tty_struct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct hci_dev {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct percpu_rw_semaphore {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct hci_uart_proto {
    pub id: ::kernel::ffi::c_uint,
    pub name: *const ::kernel::ffi::c_char,
    pub manufacturer: ::kernel::ffi::c_uint,
    pub init_speed: ::kernel::ffi::c_uint,
    pub oper_speed: ::kernel::ffi::c_uint,
    pub open: Option<unsafe extern "C" fn(*mut hci_uart) -> ::kernel::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut hci_uart) -> ::kernel::ffi::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut hci_uart) -> ::kernel::ffi::c_int>,
    pub setup: Option<unsafe extern "C" fn(*mut hci_uart) -> ::kernel::ffi::c_int>,
    pub set_baudrate: Option<unsafe extern "C" fn(*mut hci_uart, ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int>,
    pub recv: Option<unsafe extern "C" fn(*mut hci_uart, *const ::kernel::ffi::c_void, ::kernel::ffi::c_int) -> ::kernel::ffi::c_int>,
    pub enqueue: Option<unsafe extern "C" fn(*mut hci_uart, *mut sk_buff) -> ::kernel::ffi::c_int>,
    pub dequeue: Option<unsafe extern "C" fn(*mut hci_uart) -> *mut sk_buff>,
}

#[repr(C)]
pub struct hci_uart {
    pub tty: *mut tty_struct,
    pub serdev: *mut serdev_device,
    pub hdev: *mut hci_dev,
    pub flags: ::kernel::ffi::c_ulong,
    pub hdev_flags: ::kernel::ffi::c_ulong,
    pub init_ready: work_struct,
    pub write_work: work_struct,
    pub proto: *const hci_uart_proto,
    pub proto_lock: percpu_rw_semaphore,
    pub priv_: *mut ::kernel::ffi::c_void,
    pub tx_skb: *mut sk_buff,
    pub tx_state: ::kernel::ffi::c_ulong,
    pub init_speed: ::kernel::ffi::c_uint,
    pub oper_speed: ::kernel::ffi::c_uint,
    pub alignment: u8,
    pub padding: u8,
}

pub const HCI_UART_PROTO_SET: ::kernel::ffi::c_uint = 0;
pub const HCI_UART_REGISTERED: ::kernel::ffi::c_uint = 1;
pub const HCI_UART_PROTO_READY: ::kernel::ffi::c_uint = 2;
pub const HCI_UART_NO_SUSPEND_NOTIFIER: ::kernel::ffi::c_uint = 3;
pub const HCI_UART_PROTO_INIT: ::kernel::ffi::c_uint = 4;
pub const HCI_UART_SENDING: ::kernel::ffi::c_uint = 1;
pub const HCI_UART_TX_WAKEUP: ::kernel::ffi::c_uint = 2;

extern "C" {
    pub fn hci_uart_register_proto(p: *const hci_uart_proto) -> ::kernel::ffi::c_int;
    pub fn hci_uart_unregister_proto(p: *const hci_uart_proto) -> ::kernel::ffi::c_int;
    pub fn hci_uart_register_device_priv(hu: *mut hci_uart, p: *const hci_uart_proto, sizeof_priv: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn hci_uart_unregister_device(hu: *mut hci_uart);
    pub fn hci_uart_tx_wakeup(hu: *mut hci_uart) -> ::kernel::ffi::c_int;
    pub fn hci_uart_wait_until_sent(hu: *mut hci_uart) -> ::kernel::ffi::c_int;
    pub fn hci_uart_init_ready(hu: *mut hci_uart) -> ::kernel::ffi::c_int;
    pub fn hci_uart_init_work(work: *mut work_struct);
    pub fn hci_uart_set_baudrate(hu: *mut hci_uart, speed: ::kernel::ffi::c_uint);
    pub fn hci_uart_has_flow_control(hu: *mut hci_uart) -> bool;
    pub fn hci_uart_set_flow_control(hu: *mut hci_uart, enable: bool);
    pub fn hci_uart_set_speeds(hu: *mut hci_uart, init_speed: ::kernel::ffi::c_uint, oper_speed: ::kernel::ffi::c_uint);
}

#[inline]
pub unsafe fn hci_uart_register_device(hu: *mut hci_uart, p: *const hci_uart_proto) -> ::kernel::ffi::c_int {
    hci_uart_register_device_priv(hu, p, 0)
}

#[repr(C)]
pub struct h4_recv_pkt {
    pub type_: u8,
    pub hlen: u8,
    pub loff: u8,
    pub lsize: u8,
    pub maxlen: u16,
    pub recv: Option<unsafe extern "C" fn(*mut hci_dev, *mut sk_buff) -> ::kernel::ffi::c_int>,
}

/* H4_RECV_ACL, H4_RECV_SCO, H4_RECV_EVENT, and H4_RECV_ISO are designated
 * initializers whose values depend on declarations from other headers. */

/* Configuration-gated declarations retained from the C header. */
#[cfg(CONFIG_BT_HCIUART_H4)]
extern "C" {
    pub fn h4_init() -> ::kernel::ffi::c_int;
    pub fn h4_deinit() -> ::kernel::ffi::c_int;
    pub fn h4_recv_buf(hu: *mut hci_uart, skb: *mut sk_buff, buffer: *const u8, count: ::kernel::ffi::c_int, pkts: *const h4_recv_pkt, pkts_count: ::kernel::ffi::c_int) -> *mut sk_buff;
}
#[cfg(CONFIG_BT_HCIUART_BCSP)] extern "C" { pub fn bcsp_init() -> ::kernel::ffi::c_int; pub fn bcsp_deinit() -> ::kernel::ffi::c_int; }
#[cfg(CONFIG_BT_HCIUART_LL)] extern "C" { pub fn ll_init() -> ::kernel::ffi::c_int; pub fn ll_deinit() -> ::kernel::ffi::c_int; }
#[cfg(CONFIG_BT_HCIUART_ATH3K)] extern "C" { pub fn ath_init() -> ::kernel::ffi::c_int; pub fn ath_deinit() -> ::kernel::ffi::c_int; }
#[cfg(CONFIG_BT_HCIUART_3WIRE)] extern "C" { pub fn h5_init() -> ::kernel::ffi::c_int; pub fn h5_deinit() -> ::kernel::ffi::c_int; }
#[cfg(CONFIG_BT_HCIUART_INTEL)] extern "C" { pub fn intel_init() -> ::kernel::ffi::c_int; pub fn intel_deinit() -> ::kernel::ffi::c_int; }
#[cfg(CONFIG_BT_HCIUART_BCM)] extern "C" { pub fn bcm_init() -> ::kernel::ffi::c_int; pub fn bcm_deinit() -> ::kernel::ffi::c_int; }
#[cfg(CONFIG_BT_HCIUART_QCA)] extern "C" { pub fn qca_init() -> ::kernel::ffi::c_int; pub fn qca_deinit() -> ::kernel::ffi::c_int; }
#[cfg(CONFIG_BT_HCIUART_AG6XX)] extern "C" { pub fn ag6xx_init() -> ::kernel::ffi::c_int; pub fn ag6xx_deinit() -> ::kernel::ffi::c_int; }
#[cfg(CONFIG_BT_HCIUART_MRVL)] extern "C" { pub fn mrvl_init() -> ::kernel::ffi::c_int; pub fn mrvl_deinit() -> ::kernel::ffi::c_int; }
#[cfg(CONFIG_BT_HCIUART_AML)] extern "C" { pub fn aml_init() -> ::kernel::ffi::c_int; pub fn aml_deinit() -> ::kernel::ffi::c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
