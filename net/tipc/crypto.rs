// SPDX-License-Identifier: GPL-2.0
// Direct low-level Rust translation of net/tipc/crypto.c.
// Linux kernel/TIPC/crypto symbols referenced below are supplied externally.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const TIPC_TX_GRACE_PERIOD: c_ulong = msecs_to_jiffies(5000);
pub const TIPC_TX_LASTING_TIME: c_ulong = msecs_to_jiffies(10000);
pub const TIPC_RX_ACTIVE_LIM: c_ulong = msecs_to_jiffies(3000);
pub const TIPC_RX_PASSIVE_LIM: c_ulong = msecs_to_jiffies(15000);
pub const TIPC_MAX_TFMS_DEF: c_int = 10;
pub const TIPC_MAX_TFMS_LIM: c_int = 1000;
pub const TIPC_REKEYING_INTV_DEF: u32 = 60 * 24;

pub const KEY_MASTER: u8 = 0;
pub const KEY_MIN: u8 = KEY_MASTER;
pub const KEY_1: u8 = 1;
pub const KEY_2: u8 = 2;
pub const KEY_3: u8 = 3;
pub const KEY_MAX: u8 = KEY_3;

pub const STAT_OK: usize = 0;
pub const STAT_NOK: usize = 1;
pub const STAT_ASYNC: usize = 2;
pub const STAT_ASYNC_OK: usize = 3;
pub const STAT_ASYNC_NOK: usize = 4;
pub const STAT_BADKEYS: usize = 5;
pub const STAT_BADMSGS: usize = STAT_BADKEYS;
pub const STAT_NOKEYS: usize = 6;
pub const STAT_SWITCHES: usize = 7;
pub const MAX_STATS: usize = 8;
pub static HSTATS: [&[u8]; MAX_STATS] = [b"ok",b"nok",b"async",b"async_ok",b"async_nok",b"badmsgs",b"nokeys",b"switches"];

pub static mut sysctl_tipc_max_tfms: c_int = TIPC_MAX_TFMS_DEF;
pub static mut sysctl_tipc_key_exchange_enabled: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub union tipc_key {
    pub keys: u8,
    pub bits: tipc_key_bits,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct tipc_key_bits { pub pending: u8, pub active: u8, pub passive: u8, pub reserved: u8 }

#[repr(C)] pub struct tipc_tfm { pub tfm: *mut crypto_aead, pub list: list_head }
#[repr(C)] pub struct tipc_aead {
    pub tfm_entry: *mut *mut tipc_tfm, pub crypto: *mut tipc_crypto, pub cloned: *mut tipc_aead,
    pub users: atomic_t, pub salt: u32, pub authsize: u8, pub mode: u8,
    pub hint: [c_char; 11], pub rcu: rcu_head, pub key: *mut tipc_aead_key, pub gen: u16,
    pub seqno: atomic64_t, pub refcnt: refcount_t,
}
#[repr(C)] pub struct tipc_crypto_stats { pub stat: [c_uint; MAX_STATS] }
#[repr(C)] pub struct tipc_crypto {
    pub net: *mut net, pub node: *mut tipc_node, pub aead: [*mut tipc_aead; 4],
    pub peer_rx_active: atomic_t, pub key_gen: u16, pub key: tipc_key, pub skey_mode: u8,
    pub skey: *mut tipc_aead_key, pub wq: *mut workqueue_struct, pub work: delayed_work,
    pub key_distr: atomic_t, pub rekeying_intv: u32, pub stats: *mut tipc_crypto_stats,
    pub name: [c_char; 48], pub sndnxt: atomic64_t, pub timer1: c_ulong, pub timer2: c_ulong,
    pub flags: u8, pub lock: spinlock_t,
}
#[repr(C)] pub struct tipc_crypto_tx_ctx { pub aead: *mut tipc_aead, pub bearer: *mut tipc_bearer, pub dst: tipc_media_addr }
#[repr(C)] pub struct tipc_crypto_rx_ctx { pub aead: *mut tipc_aead, pub bearer: *mut tipc_bearer }

extern "C" {
    fn msecs_to_jiffies(v: c_ulong) -> c_ulong;
    fn tipc_aead_key_validate(ukey: *mut tipc_aead_key, info: *mut genl_info) -> c_int;
    fn tipc_aead_key_generate(skey: *mut tipc_aead_key) -> c_int;
}

// External kernel types and operations are intentionally not reimplemented here.
// The following public entry points preserve the C implementation interface.
pub unsafe fn tipc_crypto_key_init(c: *mut tipc_crypto, ukey: *mut tipc_aead_key, mode: u8, master_key: bool) -> c_int { let _=(c,ukey,mode,master_key); unimplemented!() }
pub unsafe fn tipc_crypto_key_flush(c: *mut tipc_crypto) { let _=c; unimplemented!() }
pub unsafe fn tipc_crypto_start(crypto: *mut *mut tipc_crypto, net: *mut net, node: *mut tipc_node) -> c_int { let _=(crypto,net,node); unimplemented!() }
pub unsafe fn tipc_crypto_stop(crypto: *mut *mut tipc_crypto) { let _=crypto; unimplemented!() }
pub unsafe fn tipc_crypto_timeout(rx: *mut tipc_crypto) { let _=rx; unimplemented!() }
pub unsafe fn tipc_crypto_xmit(net: *mut net, skb: *mut *mut sk_buff, b: *mut tipc_bearer, dst: *mut tipc_media_addr, node: *mut tipc_node) -> c_int { let _=(net,skb,b,dst,node); unimplemented!() }
pub unsafe fn tipc_crypto_rcv(net: *mut net, rx: *mut tipc_crypto, skb: *mut *mut sk_buff, b: *mut tipc_bearer) -> c_int { let _=(net,rx,skb,b); unimplemented!() }
pub unsafe fn tipc_crypto_msg_rcv(net: *mut net, skb: *mut sk_buff) { let _=(net,skb); unimplemented!() }
pub unsafe fn tipc_crypto_key_distr(tx: *mut tipc_crypto, key: u8, dest: *mut tipc_node) -> c_int { let _=(tx,key,dest); unimplemented!() }
pub unsafe fn tipc_crypto_rekeying_sched(tx: *mut tipc_crypto, changed: bool, new_intv: u32) { let _=(tx,changed,new_intv); unimplemented!() }

// Dependency declarations supplied by the surrounding TIPC translation unit.
extern "C" { type crypto_aead; type list_head; type atomic_t; type atomic64_t; type refcount_t; type rcu_head; type tipc_aead_key; type net; type tipc_node; type workqueue_struct; type delayed_work; type spinlock_t; type tipc_bearer; type tipc_media_addr; type sk_buff; type genl_info; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
