/* SPDX-License-Identifier: GPL-2.0 */
// Copyright (c) 2010-2011 EIA Electronics,
//                         Kurt Van Dijck <kurt.van.dijck@eia.be>
// Copyright (c) 2017-2019 Pengutronix,
//                         Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (c) 2017-2019 Pengutronix,
//                         Oleksij Rempel <kernel@pengutronix.de>

// Dependencies supplied by the corresponding Linux CAN/network headers.

pub const J1939_XTP_ABORT_TIMEOUT_MS: u32 = 500;
pub const J1939_SIMPLE_ECHO_TIMEOUT_MS: u32 = 10 * 1000;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum j1939_sk_errqueue_type {
    J1939_ERRQUEUE_TX_ACK,
    J1939_ERRQUEUE_TX_SCHED,
    J1939_ERRQUEUE_TX_ABORT,
    J1939_ERRQUEUE_RX_RTS,
    J1939_ERRQUEUE_RX_DPO,
    J1939_ERRQUEUE_RX_ABORT,
}

#[repr(C)]
pub struct j1939_ecu {
    pub list: list_head,
    pub name: name_t,
    pub addr: u8,
    pub ac_timer: hrtimer,
    pub kref: kref,
    pub priv_: *mut j1939_priv,
    pub priv_dev_tracker: netdevice_tracker,
    pub nusers: i32,
}

#[repr(C)]
pub struct j1939_addr_ent {
    pub ecu: *mut j1939_ecu,
    pub nusers: i32,
}

#[repr(C)]
pub struct j1939_priv {
    pub ecus: list_head,
    pub lock: rwlock_t,
    pub ndev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub ents: [j1939_addr_ent; 256],
    pub kref: kref,
    pub active_session_list: list_head,
    pub active_session_list_lock: spinlock_t,
    pub tp_max_packet_size: u32,
    pub j1939_socks_lock: rwlock_t,
    pub j1939_socks: list_head,
    pub rx_kref: kref,
    pub rx_tskey: u32,
}

extern "C" {
    pub fn j1939_ecu_put(ecu: *mut j1939_ecu);
    pub fn j1939_local_ecu_get(priv_: *mut j1939_priv, name: name_t, sa: u8) -> i32;
    pub fn j1939_local_ecu_put(priv_: *mut j1939_priv, name: name_t, sa: u8);
}

#[inline]
pub fn j1939_address_is_unicast(addr: u8) -> bool { addr <= J1939_MAX_UNICAST_ADDR }
#[inline]
pub fn j1939_address_is_idle(addr: u8) -> bool { addr == J1939_IDLE_ADDR }
#[inline]
pub fn j1939_address_is_valid(addr: u8) -> bool { addr != J1939_NO_ADDR }
#[inline]
pub fn j1939_pgn_is_pdu1(pgn: pgn_t) -> bool { (pgn & 0xff00) < 0xf000 }

extern "C" {
    pub fn j1939_ecu_unmap_locked(ecu: *mut j1939_ecu);
    pub fn j1939_ecu_unmap(ecu: *mut j1939_ecu);
    pub fn j1939_name_to_addr(priv_: *mut j1939_priv, name: name_t) -> u8;
    pub fn j1939_ecu_find_by_addr_locked(priv_: *mut j1939_priv, addr: u8) -> *mut j1939_ecu;
    pub fn j1939_ecu_get_by_addr(priv_: *mut j1939_priv, addr: u8) -> *mut j1939_ecu;
    pub fn j1939_ecu_get_by_addr_locked(priv_: *mut j1939_priv, addr: u8) -> *mut j1939_ecu;
    pub fn j1939_ecu_get_by_name(priv_: *mut j1939_priv, name: name_t) -> *mut j1939_ecu;
    pub fn j1939_ecu_get_by_name_locked(priv_: *mut j1939_priv, name: name_t) -> *mut j1939_ecu;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum j1939_transfer_type { J1939_TP, J1939_ETP, J1939_SIMPLE }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct j1939_addr { pub src_name: name_t, pub dst_name: name_t, pub pgn: pgn_t, pub sa: u8, pub da: u8, pub type_: u8 }

pub const J1939_ECU_LOCAL_SRC: u8 = BIT(0);
pub const J1939_ECU_LOCAL_DST: u8 = BIT(1);

#[repr(C)]
pub struct j1939_sk_buff_cb {
    pub offset: u32,
    pub msg_flags: u32,
    pub tskey: u32,
    pub addr: j1939_addr,
    pub flags: u8,
    pub priority: priority_t,
}

#[inline]
pub unsafe fn j1939_skb_to_cb(skb: *const sk_buff) -> *mut j1939_sk_buff_cb {
    (*skb).cb.as_mut_ptr() as *mut j1939_sk_buff_cb
}

extern "C" {
    pub fn j1939_send_one(priv_: *mut j1939_priv, skb: *mut sk_buff) -> i32;
    pub fn j1939_sk_recv(priv_: *mut j1939_priv, skb: *mut sk_buff);
    pub fn j1939_sk_recv_match(priv_: *mut j1939_priv, skcb: *mut j1939_sk_buff_cb) -> bool;
    pub fn j1939_sk_send_loop_abort(sk: *mut sock, err: i32);
    pub fn j1939_sk_errqueue(session: *mut j1939_session, type_: j1939_sk_errqueue_type);
    pub fn j1939_sk_queue_activate_next(session: *mut j1939_session);
    pub fn j1939_tp_send(priv_: *mut j1939_priv, skb: *mut sk_buff, size: usize) -> *mut j1939_session;
    pub fn j1939_tp_recv(priv_: *mut j1939_priv, skb: *mut sk_buff) -> i32;
    pub fn j1939_ac_fixup(priv_: *mut j1939_priv, skb: *mut sk_buff) -> i32;
    pub fn j1939_ac_recv(priv_: *mut j1939_priv, skb: *mut sk_buff);
    pub fn j1939_simple_recv(priv_: *mut j1939_priv, skb: *mut sk_buff);
    pub fn j1939_ecu_create_locked(priv_: *mut j1939_priv, name: name_t) -> *mut j1939_ecu;
    pub fn j1939_ecu_timer_start(ecu: *mut j1939_ecu);
    pub fn j1939_ecu_timer_cancel(ecu: *mut j1939_ecu);
    pub fn j1939_ecu_unmap_all(priv_: *mut j1939_priv);
    pub fn j1939_netdev_start(ndev: *mut net_device) -> *mut j1939_priv;
    pub fn j1939_netdev_stop(priv_: *mut j1939_priv);
    pub fn j1939_priv_put(priv_: *mut j1939_priv);
    pub fn j1939_priv_get(priv_: *mut j1939_priv);
    pub fn j1939_sk_netdev_event_netdown(priv_: *mut j1939_priv);
    pub fn j1939_sk_netdev_event_unregister(priv_: *mut j1939_priv);
    pub fn j1939_cancel_active_session(priv_: *mut j1939_priv, sk: *mut sock) -> i32;
    pub fn j1939_tp_init(priv_: *mut j1939_priv);
    pub fn j1939_sock_pending_del(sk: *mut sock);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum j1939_session_state { J1939_SESSION_NEW, J1939_SESSION_ACTIVE, J1939_SESSION_WAITING_ABORT, J1939_SESSION_ACTIVE_MAX, J1939_SESSION_DONE }

#[repr(C)]
pub struct j1939_session {
    pub priv_: *mut j1939_priv,
    pub priv_dev_tracker: netdevice_tracker,
    pub active_session_list_entry: list_head,
    pub sk_session_queue_entry: list_head,
    pub kref: kref,
    pub sk: *mut sock,
    pub skcb: j1939_sk_buff_cb,
    pub skb_queue: sk_buff_head,
    pub last_cmd: u8,
    pub last_txcmd: u8,
    pub transmission: bool,
    pub extd: bool,
    pub total_message_size: u32,
    pub total_queued_size: u32,
    pub tx_retry: u32,
    pub err: i32,
    pub tskey: u32,
    pub state: j1939_session_state,
    pub pkt: j1939_session_pkt,
    pub txtimer: hrtimer,
    pub rxtimer: hrtimer,
}

#[repr(C)]
pub struct j1939_session_pkt {
    pub total: u32,
    pub last: u32,
    pub tx: u32,
    pub tx_acked: u32,
    pub rx: u32,
    pub block: u32,
    pub dpo: u32,
}

#[repr(C)]
pub struct j1939_sock {
    pub sk: sock,
    pub priv_: *mut j1939_priv,
    pub list: list_head,
    pub state: i32,
    pub ifindex: i32,
    pub addr: j1939_addr,
    pub filters_lock: spinlock_t,
    pub filters: *mut j1939_filter,
    pub nfilters: i32,
    pub pgn_rx_filter: pgn_t,
    pub skb_pending: atomic_t,
    pub waitq: wait_queue_head_t,
    pub sk_session_queue_lock: spinlock_t,
    pub sk_session_queue: list_head,
}

pub const J1939_SOCK_BOUND: i32 = BIT(0);
pub const J1939_SOCK_CONNECTED: i32 = BIT(1);
pub const J1939_SOCK_PROMISC: i32 = BIT(2);
pub const J1939_SOCK_ERRQUEUE: i32 = BIT(3);

#[inline]
pub unsafe fn j1939_sk(sk: *const sock) -> *mut j1939_sock {
    (sk as *const u8).sub(std::mem::offset_of!(j1939_sock, sk)) as *mut j1939_sock
}

extern "C" {
    pub fn j1939_session_get(session: *mut j1939_session);
    pub fn j1939_session_put(session: *mut j1939_session);
    pub fn j1939_session_skb_queue(session: *mut j1939_session, skb: *mut sk_buff);
    pub fn j1939_session_activate(session: *mut j1939_session) -> i32;
    pub fn j1939_tp_schedule_txtimer(session: *mut j1939_session, msec: i32);
    pub fn j1939_session_timers_cancel(session: *mut j1939_session);
}

pub const J1939_MIN_TP_PACKET_SIZE: u32 = 9;
pub const J1939_MAX_TP_PACKET_SIZE: u32 = 7 * 0xff;
pub const J1939_MAX_ETP_PACKET_SIZE: u32 = 7 * 0x00ffffff;
pub const J1939_REGULAR: u32 = 0;
pub const J1939_EXTENDED: u32 = 1;

extern "C" { pub static j1939_can_proto: can_proto; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
