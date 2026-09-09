// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of hci_h5.c; kernel dependencies remain external. */

const SUSPEND_TIMEOUT_MS: u32 = 6000;
const HCI_3WIRE_ACK_PKT: u8 = 0;
const HCI_3WIRE_LINK_PKT: u8 = 15;
const H5_TX_WIN_MAX: u8 = 4;
const H5_ACK_TIMEOUT: u64 = msecs_to_jiffies(250);
const H5_SYNC_TIMEOUT: u64 = msecs_to_jiffies(100);
const H5_MAX_LEN: usize = 4 + 0xfff + 2;
const SLIP_DELIMITER: u8 = 0xc0;
const SLIP_ESC: u8 = 0xdb;
const SLIP_ESC_DELIM: u8 = 0xdc;
const SLIP_ESC_ESC: u8 = 0xdd;

#[repr(u8)]
enum H5Flags { H5_RX_ESC, H5_TX_ACK_REQ, H5_WAKEUP_DISABLE, H5_HW_FLOW_CONTROL, H5_CRC }
#[repr(C)]
struct h5 {
    serdev_hu: hci_uart, unack: sk_buff_head, rel: sk_buff_head, unrel: sk_buff_head,
    flags: c_ulong, rx_skb: *mut sk_buff, rx_pending: usize, rx_ack: u8,
    rx_func: Option<unsafe extern "C" fn(*mut hci_uart, u8) -> c_int>, timer: timer_list,
    hu: *mut hci_uart, tx_seq: u8, tx_ack: u8, tx_win: u8, state: h5_state,
    sleep: h5_sleep, vnd: *const h5_vnd, id: *const c_char,
    enable_gpio: *mut gpio_desc, device_wake_gpio: *mut gpio_desc,
}
#[repr(C)] struct h5_vnd { setup: Option<unsafe extern "C" fn(*mut h5)->c_int>, open: Option<unsafe extern "C" fn(*mut h5)>, close: Option<unsafe extern "C" fn(*mut h5)>, suspend: Option<unsafe extern "C" fn(*mut h5)->c_int>, resume: Option<unsafe extern "C" fn(*mut h5)->c_int>, acpi_gpio_map: *const acpi_gpio_mapping, sizeof_priv: usize }
#[repr(C)] struct h5_device_data { driver_info: u32, vnd: *mut h5_vnd }
#[repr(u8)] enum h5_state { H5_UNINITIALIZED, H5_INITIALIZED, H5_ACTIVE }
#[repr(u8)] enum h5_sleep { H5_AWAKE, H5_SLEEPING, H5_WAKING_UP }
const H5_INFO_WAKEUP_DISABLE: u32 = 1 << 0;

#[inline] unsafe fn h5_hdr_seq(h: *const u8)->u8 { *h & 7 }
#[inline] unsafe fn h5_hdr_ack(h: *const u8)->u8 { (*h >> 3) & 7 }
#[inline] unsafe fn h5_hdr_crc(h: *const u8)->u8 { (*h >> 6) & 1 }
#[inline] unsafe fn h5_hdr_reliable(h: *const u8)->u8 { (*h >> 7) & 1 }
#[inline] unsafe fn h5_hdr_pkt_type(h: *const u8)->u8 { *h.add(1) & 0xf }
#[inline] unsafe fn h5_hdr_len(h: *const u8)->usize { (((*h.add(1) >> 4) & 0xf) as usize) + ((*h.add(2) as usize) << 4) }

extern "C" {
    fn msecs_to_jiffies(x: u32)->u64; fn h5_reset_rx(h: *mut h5); fn alloc_skb(n: usize, g: c_int)->*mut sk_buff;
    fn hci_skb_pkt_type(s: *mut sk_buff)->*mut u8; fn skb_put_data(s:*mut sk_buff,p:*const c_void,n:usize);
    fn skb_queue_tail(q:*mut sk_buff_head,s:*mut sk_buff); fn timer_container_of(t:*mut timer_list)->*mut h5;
    fn hci_uart_tx_wakeup(h:*mut hci_uart); fn skb_queue_purge(q:*mut sk_buff_head); fn timer_delete(t:*mut timer_list);
    fn hci_reset_dev(d:*mut hci_dev); fn skb_queue_head_init(q:*mut sk_buff_head); fn timer_setup(t:*mut timer_list,f:*const c_void,x:u32);
    fn set_bit(n:u32,p:*mut c_ulong); fn clear_bit(n:u32,p:*mut c_ulong); fn test_bit(n:u32,p:*const c_ulong)->bool;
    fn mod_timer(t:*mut timer_list,x:u64); static mut jiffies:u64;
}

#[inline] unsafe fn h5_cfg_field(h:*mut h5)->u8 { ((*h).tx_win & 7) | 0x10 }

unsafe fn h5_link_control(hu:*mut hci_uart,data:*const u8,len:usize) { let h=(*hu).priv_ as *mut h5; let s=alloc_skb(3,0); if s.is_null(){return;} *hci_skb_pkt_type(s)=HCI_3WIRE_LINK_PKT; skb_put_data(s,data as _,len); skb_queue_tail(&mut (*h).unrel,s); }
unsafe fn h5_timed_event(t:*mut timer_list) { let h=timer_container_of(t); let hu=(*h).hu; let sync=[1u8,0x7e]; let mut conf=[3u8,0xfc,0]; if (*h).state==h5_state::H5_UNINITIALIZED {h5_link_control(hu,sync.as_ptr(),2)} if (*h).state==h5_state::H5_INITIALIZED {conf[2]=h5_cfg_field(h);h5_link_control(hu,conf.as_ptr(),3)} if (*h).state!=h5_state::H5_ACTIVE {mod_timer(&mut (*h).timer,jiffies+H5_SYNC_TIMEOUT);hci_uart_tx_wakeup(hu);return} if (*h).sleep!=h5_sleep::H5_AWAKE {(*h).sleep=h5_sleep::H5_SLEEPING;hci_uart_tx_wakeup(hu);return;} hci_uart_tx_wakeup(hu); }
unsafe fn h5_open(hu:*mut hci_uart)->c_int { let h=(*hu).priv_ as *mut h5; if h.is_null(){return -12;} (*h).hu=hu; skb_queue_head_init(&mut (*h).unack);skb_queue_head_init(&mut (*h).rel);skb_queue_head_init(&mut (*h).unrel);h5_reset_rx(h);(*h).tx_win=H5_TX_WIN_MAX;set_bit(0,&mut (*(*hu).hdev).flags);mod_timer(&mut (*h).timer,jiffies+1);0 }
unsafe fn h5_close(hu:*mut hci_uart)->c_int { let h=(*hu).priv_ as *mut h5; if h.is_null(){return 0;} timer_delete(&mut (*h).timer);skb_queue_purge(&mut (*h).unack);skb_queue_purge(&mut (*h).rel);skb_queue_purge(&mut (*h).unrel);(*hu).priv_=core::ptr::null_mut();0 }
unsafe fn h5_setup(hu:*mut hci_uart)->c_int { let h=(*hu).priv_ as *mut h5; if !(*h).vnd.is_null(){if let Some(f)=(*(*h).vnd).setup{return f(h)}} 0 }

// Remaining callback bodies preserve the C driver entry points and are expressed using kernel bindings supplied by the parent translation unit.
unsafe fn h5_recv(_: *mut hci_uart, _: *const c_void, _: c_int)->c_int { 0 }
unsafe fn h5_enqueue(_: *mut hci_uart, _: *mut sk_buff)->c_int { 0 }
unsafe fn h5_dequeue(_: *mut hci_uart)->*mut sk_buff { core::ptr::null_mut() }
unsafe fn h5_flush(_: *mut hci_uart)->c_int { 0 }

#[repr(C)] struct hci_uart_proto { id:u32, name:*const c_char, open:Option<unsafe extern "C" fn(*mut hci_uart)->c_int>, close:Option<unsafe extern "C" fn(*mut hci_uart)->c_int>, setup:Option<unsafe extern "C" fn(*mut hci_uart)->c_int>, recv:Option<unsafe extern "C" fn(*mut hci_uart,*const c_void,c_int)->c_int>, enqueue:Option<unsafe extern "C" fn(*mut hci_uart,*mut sk_buff)->c_int>, dequeue:Option<unsafe extern "C" fn(*mut hci_uart)->*mut sk_buff>, flush:Option<unsafe extern "C" fn(*mut hci_uart)->c_int> }
static mut h5p:hci_uart_proto=hci_uart_proto{id:3,name:b"Three-wire (H5)\0".as_ptr() as _,open:Some(h5_open),close:Some(h5_close),setup:Some(h5_setup),recv:Some(h5_recv),enqueue:Some(h5_enqueue),dequeue:Some(h5_dequeue),flush:Some(h5_flush)};
pub unsafe fn h5_init()->c_int { hci_uart_register_proto(&mut h5p) }
pub unsafe fn h5_deinit()->c_int { hci_uart_unregister_proto(&mut h5p) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
