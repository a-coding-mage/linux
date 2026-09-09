/* SPDX-License-Identifier: GPL-2.0 */
/* Declarations of X.25 Packet Layer type objects. */

pub const X25_ADDR_LEN: usize = 16;
pub const X25_MAX_L2_LEN: usize = 18;
pub const X25_STD_MIN_LEN: usize = 3;
pub const X25_EXT_MIN_LEN: usize = 4;
pub const X25_GFI_SEQ_MASK: u8 = 0x30;
pub const X25_GFI_STDSEQ: u8 = 0x10;
pub const X25_GFI_EXTSEQ: u8 = 0x20;
pub const X25_Q_BIT: u8 = 0x80;
pub const X25_D_BIT: u8 = 0x40;
pub const X25_STD_M_BIT: u8 = 0x10;
pub const X25_EXT_M_BIT: u8 = 0x01;

pub const X25_CALL_REQUEST: u8 = 0x0b;
pub const X25_CALL_ACCEPTED: u8 = 0x0f;
pub const X25_CLEAR_REQUEST: u8 = 0x13;
pub const X25_CLEAR_CONFIRMATION: u8 = 0x17;
pub const X25_DATA: u8 = 0x00;
pub const X25_INTERRUPT: u8 = 0x23;
pub const X25_INTERRUPT_CONFIRMATION: u8 = 0x27;
pub const X25_RR: u8 = 0x01;
pub const X25_RNR: u8 = 0x05;
pub const X25_REJ: u8 = 0x09;
pub const X25_RESET_REQUEST: u8 = 0x1b;
pub const X25_RESET_CONFIRMATION: u8 = 0x1f;
pub const X25_REGISTRATION_REQUEST: u8 = 0xf3;
pub const X25_REGISTRATION_CONFIRMATION: u8 = 0xf7;
pub const X25_RESTART_REQUEST: u8 = 0xfb;
pub const X25_RESTART_CONFIRMATION: u8 = 0xff;
pub const X25_DIAGNOSTIC: u8 = 0xf1;
pub const X25_ILLEGAL: u8 = 0xfd;

pub const X25_COND_ACK_PENDING: u8 = 0x01;
pub const X25_COND_OWN_RX_BUSY: u8 = 0x02;
pub const X25_COND_PEER_RX_BUSY: u8 = 0x04;

pub const X25_STATE_0: u32 = 0;
pub const X25_STATE_1: u32 = 1;
pub const X25_STATE_2: u32 = 2;
pub const X25_STATE_3: u32 = 3;
pub const X25_STATE_4: u32 = 4;
pub const X25_STATE_5: u32 = 5;
pub const X25_LINK_STATE_0: u32 = 0;
pub const X25_LINK_STATE_1: u32 = 1;
pub const X25_LINK_STATE_2: u32 = 2;
pub const X25_LINK_STATE_3: u32 = 3;

pub const X25_DEFAULT_T20: u64 = 180 * HZ as u64;
pub const X25_DEFAULT_T21: u64 = 200 * HZ as u64;
pub const X25_DEFAULT_T22: u64 = 180 * HZ as u64;
pub const X25_DEFAULT_T23: u64 = 180 * HZ as u64;
pub const X25_DEFAULT_T2: u64 = 3 * HZ as u64;
pub const X25_DEFAULT_WINDOW_SIZE: u32 = 2;
pub const X25_DEFAULT_PACKET_SIZE: u32 = X25_PS128;
pub const X25_DEFAULT_THROUGHPUT: u8 = 0x0a;
pub const X25_DEFAULT_REVERSE: u8 = 0;
pub const X25_SMODULUS: u32 = 8;
pub const X25_EMODULUS: u32 = 128;

pub const X25_FAC_CLASS_MASK: u8 = 0xc0;
pub const X25_FAC_CLASS_A: u8 = 0x00;
pub const X25_FAC_CLASS_B: u8 = 0x40;
pub const X25_FAC_CLASS_C: u8 = 0x80;
pub const X25_FAC_CLASS_D: u8 = 0xc0;
pub const X25_FAC_REVERSE: u8 = 0x01;
pub const X25_FAC_THROUGHPUT: u8 = 0x02;
pub const X25_FAC_PACKET_SIZE: u8 = 0x42;
pub const X25_FAC_WINDOW_SIZE: u8 = 0x43;
pub const X25_MAX_FAC_LEN: usize = 60;
pub const X25_MAX_CUD_LEN: usize = 128;
pub const X25_FAC_CALLING_AE: u8 = 0xcb;
pub const X25_FAC_CALLED_AE: u8 = 0xc9;
pub const X25_MARKER: u8 = 0;
pub const X25_DTE_SERVICES: u8 = 0x0f;
pub const X25_MAX_AE_LEN: usize = 40;
pub const X25_MAX_DTE_FACIL_LEN: usize = 21;
pub const X25_Q_BIT_FLAG: u32 = 0;
pub const X25_INTERRUPT_FLAG: u32 = 1;
pub const X25_ACCPT_APPRV_FLAG: u32 = 2;

#[repr(C)]
pub struct x25_route { pub node: list_head, pub address: x25_address, pub sigdigits: c_uint, pub dev: *mut net_device, pub refcnt: refcount_t }
#[repr(C)]
pub struct x25_neigh { pub node: list_head, pub dev: *mut net_device, pub state: c_uint, pub extended: c_uint, pub queue: sk_buff_head, pub t20: c_ulong, pub t20timer: timer_list, pub global_facil_mask: c_ulong, pub refcnt: refcount_t }
#[repr(C)]
pub struct x25_sock { pub sk: sock, pub source_addr: x25_address, pub dest_addr: x25_address, pub neighbour: *mut x25_neigh, pub lci: c_uint, pub cudmatchlength: c_uint, pub state: u8, pub condition: u8, pub vs: c_ushort, pub vr: c_ushort, pub va: c_ushort, pub vl: c_ushort, pub t2: c_ulong, pub t21: c_ulong, pub t22: c_ulong, pub t23: c_ulong, pub fraglen: c_ushort, pub flags: c_ulong, pub ack_queue: sk_buff_head, pub fragment_queue: sk_buff_head, pub interrupt_in_queue: sk_buff_head, pub interrupt_out_queue: sk_buff_head, pub timer: timer_list, pub causediag: x25_causediag, pub facilities: x25_facilities, pub dte_facilities: x25_dte_facilities, pub calluserdata: x25_calluserdata, pub vc_facil_mask: c_ulong }
#[repr(C)]
pub struct x25_forward { pub node: list_head, pub lci: c_uint, pub dev1: *mut net_device, pub dev2: *mut net_device, pub refcnt: atomic_t }

/* container_of_const(ptr, struct x25_sock, sk) */
#[macro_export] macro_rules! x25_sk { ($ptr:expr) => { container_of_const!($ptr, x25_sock, sk) }; }

extern "C" {
    pub static mut sysctl_x25_restart_request_timeout: c_int;
    pub static mut sysctl_x25_call_request_timeout: c_int;
    pub static mut sysctl_x25_reset_request_timeout: c_int;
    pub static mut sysctl_x25_clear_request_timeout: c_int;
    pub static mut sysctl_x25_ack_holdback_timeout: c_int;
    pub static mut sysctl_x25_forward: c_int;
}

/* External declarations from the X.25 implementation. */
extern "C" {
    pub fn x25_parse_address_block(skb: *mut sk_buff, called_addr: *mut x25_address, calling_addr: *mut x25_address) -> c_int;
    pub fn x25_addr_ntoa(buf: *mut u8, called: *mut x25_address, calling: *mut x25_address) -> c_int;
    pub fn x25_addr_aton(buf: *mut u8, called: *mut x25_address, calling: *mut x25_address) -> c_int;
    pub fn x25_find_socket(lci: c_uint, nb: *mut x25_neigh) -> *mut sock;
    pub fn x25_destroy_socket_from_timer(sk: *mut sock);
    pub fn x25_rx_call_request(skb: *mut sk_buff, nb: *mut x25_neigh, lci: c_uint) -> c_int;
    pub fn x25_kill_by_neigh(nb: *mut x25_neigh);
    pub fn x25_send_frame(skb: *mut sk_buff, nb: *mut x25_neigh);
    pub fn x25_lapb_receive_frame(skb: *mut sk_buff, dev: *mut net_device, pt: *mut packet_type, orig_dev: *mut net_device) -> c_int;
    pub fn x25_establish_link(nb: *mut x25_neigh);
}

#[repr(C)] pub struct x25_skb_cb { pub flags: c_uint }
/* #define X25_SKB_CB(s) ((struct x25_skb_cb *) ((s)->cb)) */

/* The remaining prototypes are declarations supplied by the corresponding X.25 source units. */
extern "C" {
    pub fn x25_parse_facilities(skb: *mut sk_buff, facilities: *mut x25_facilities, dte: *mut x25_dte_facilities, mask: *mut c_ulong) -> c_int;
    pub fn x25_create_facilities(buf: *mut u8, facilities: *mut x25_facilities, dte: *mut x25_dte_facilities, mask: c_ulong) -> c_int;
    pub fn x25_negotiate_facilities(skb: *mut sk_buff, sk: *mut sock, facilities: *mut x25_facilities, dte: *mut x25_dte_facilities) -> c_int;
    pub fn x25_limit_facilities(facilities: *mut x25_facilities, nb: *mut x25_neigh);
    pub fn x25_clear_forward_by_lci(lci: c_uint);
    pub fn x25_clear_forward_by_dev(dev: *mut net_device);
    pub fn x25_forward_data(op: c_int, nb: *mut x25_neigh, skb: *mut sk_buff) -> c_int;
    pub fn x25_forward_call(addr: *mut x25_address, nb: *mut x25_neigh, skb: *mut sk_buff, op: c_int) -> c_int;
    pub fn x25_process_rx_frame(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    pub fn x25_backlog_rcv(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    pub fn x25_link_control(skb: *mut sk_buff, nb: *mut x25_neigh, command: c_ushort);
    pub fn x25_link_device_up(dev: *mut net_device);
    pub fn x25_link_device_down(dev: *mut net_device);
    pub fn x25_link_established(nb: *mut x25_neigh);
    pub fn x25_link_terminated(nb: *mut x25_neigh);
    pub fn x25_transmit_clear_request(nb: *mut x25_neigh, lci: c_uint, cause: u8);
    pub fn x25_transmit_link(skb: *mut sk_buff, nb: *mut x25_neigh);
    pub fn x25_subscr_ioctl(cmd: c_uint, arg: *mut c_void) -> c_int;
    pub fn x25_get_neigh(dev: *mut net_device) -> *mut x25_neigh;
    pub fn x25_link_free();
    pub fn x25_output(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    pub fn x25_kick(sk: *mut sock);
    pub fn x25_enquiry_response(sk: *mut sock);
    pub fn x25_get_route(addr: *mut x25_address) -> *mut x25_route;
    pub fn x25_dev_get(name: *mut c_char) -> *mut net_device;
    pub fn x25_route_device_down(dev: *mut net_device);
    pub fn x25_route_ioctl(cmd: c_uint, arg: *mut c_void) -> c_int;
    pub fn x25_route_free();
    pub fn x25_clear_queues(sk: *mut sock);
    pub fn x25_frames_acked(sk: *mut sock, nr: c_ushort);
    pub fn x25_requeue_frames(sk: *mut sock);
    pub fn x25_validate_nr(sk: *mut sock, nr: c_ushort) -> c_int;
    pub fn x25_write_internal(sk: *mut sock, frametype: c_int);
    pub fn x25_decode(sk: *mut sock, skb: *mut sk_buff, ns: *mut c_int, nr: *mut c_int, q: *mut c_int, d: *mut c_int, m: *mut c_int) -> c_int;
    pub fn x25_disconnect(sk: *mut sock, reason: c_int, cause: u8, diagnostic: u8);
    pub fn x25_init_timers(sk: *mut sock);
    pub fn x25_start_heartbeat(sk: *mut sock);
    pub fn x25_start_t2timer(sk: *mut sock);
    pub fn x25_start_t21timer(sk: *mut sock);
    pub fn x25_start_t22timer(sk: *mut sock);
    pub fn x25_start_t23timer(sk: *mut sock);
    pub fn x25_stop_heartbeat(sk: *mut sock);
    pub fn x25_stop_timer(sk: *mut sock);
    pub fn x25_display_timer(sk: *mut sock) -> c_ulong;
    pub fn x25_check_rbuf(sk: *mut sock);
    pub fn x25_proc_init() -> c_int;
    pub fn x25_proc_exit();
}

#[inline] pub unsafe fn x25_neigh_hold(nb: *mut x25_neigh) { refcount_inc(&mut (*nb).refcnt); }
#[inline] pub unsafe fn x25_neigh_put(nb: *mut x25_neigh) { if refcount_dec_and_test(&mut (*nb).refcnt) { kfree(nb as *mut c_void); } }
#[inline] pub unsafe fn x25_route_hold(rt: *mut x25_route) { refcount_inc(&mut (*rt).refcnt); }
#[inline] pub unsafe fn x25_route_put(rt: *mut x25_route) { if refcount_dec_and_test(&mut (*rt).refcnt) { kfree(rt as *mut c_void); } }

#[cfg(feature = "CONFIG_SYSCTL")]
extern "C" { pub fn x25_register_sysctl() -> c_int; pub fn x25_unregister_sysctl(); }
#[cfg(not(feature = "CONFIG_SYSCTL"))]
#[inline] pub fn x25_register_sysctl() -> c_int { 0 }
#[cfg(not(feature = "CONFIG_SYSCTL"))]
#[inline] pub fn x25_unregister_sysctl() {}

extern "C" {
    pub static mut x25_list: hlist_head;
    pub static mut x25_list_lock: rwlock_t;
    pub static mut x25_route_list: list_head;
    pub static mut x25_route_list_lock: rwlock_t;
    pub static mut x25_forward_list: list_head;
    pub static mut x25_forward_list_lock: rwlock_t;
    pub static mut x25_neigh_list: list_head;
    pub static mut x25_neigh_list_lock: rwlock_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
