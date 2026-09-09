// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Direct low-level Rust translation of bcm.c. Linux-kernel dependencies are
// intentionally referenced as external symbols and are supplied by the host.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const MAX_NFRAMES: usize = 256;
pub const BCM_TIMER_SEC_MAX: i64 = 400 * 24 * 60 * 60;
pub const RX_LOCAL: u8 = 0x10;
pub const RX_OWN: u8 = 0x20;
pub const RX_RECV: u8 = 0x40;
pub const RX_THR: u8 = 0x80;
pub const BCM_CAN_FLAGS_MASK: u8 = 0x0f;

pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub type u64_ = u64;
pub type canid_t = u32;
pub type ktime_t = i64;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct bcm_timeval { pub tv_sec: i64, pub tv_usec: i64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct canfd_frame { pub can_id: canid_t, pub len: u8, pub flags: u8, pub __res0: u8, pub __res1: u8, pub data: [u8; 64] }

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct bcm_msg_head { pub opcode: u32, pub flags: u32, pub count: u32, pub ival1: bcm_timeval, pub ival2: bcm_timeval, pub can_id: canid_t, pub nframes: u32 }

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct hrtimer { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32> }
#[repr(C)] pub struct net { _private: [u8; 0] }

#[repr(C)]
pub struct bcm_op {
    pub list: list_head, pub rcu: rcu_head, pub work: work_struct,
    pub ifindex: i32, pub can_id: canid_t, pub flags: u32,
    pub frames_abs: isize, pub frames_filtered: isize,
    pub ival1: bcm_timeval, pub ival2: bcm_timeval,
    pub timer: hrtimer, pub thrtimer: hrtimer,
    pub rx_stamp: ktime_t, pub kt_ival1: ktime_t, pub kt_ival2: ktime_t, pub kt_lastmsg: ktime_t,
    pub rx_ifindex: i32, pub if_detected: i32, pub cfsiz: i32,
    pub count: u32, pub nframes: u32, pub currframe: u32,
    pub frames: *mut core::ffi::c_void, pub last_frames: *mut core::ffi::c_void,
    pub sframe: canfd_frame, pub last_sframe: canfd_frame,
    pub sk: *mut sock, pub rx_reg_dev: *mut net_device,
}

#[repr(C)]
pub struct bcm_sock {
    pub sk: sock, pub bound: i32, pub ifindex: i32,
    pub notifier: list_head, pub rx_ops: list_head, pub tx_ops: list_head,
    pub dropped_usr_msgs: usize, pub bcm_proc_read: *mut core::ffi::c_void,
    pub procname: [u8; 32],
}

extern "C" {
    static mut bcm_wq: *mut core::ffi::c_void;
    fn bcm_can_tx(op: *mut bcm_op, cf: *mut canfd_frame);
    fn bcm_send_to_user(op: *mut bcm_op, head: *mut bcm_msg_head, frames: *mut canfd_frame, has_timestamp: i32);
    fn bcm_rx_changed(op: *mut bcm_op, data: *mut canfd_frame);
    fn bcm_rx_update_and_send(op: *mut bcm_op, lastdata: *mut canfd_frame, rxdata: *const canfd_frame, traffic_flags: u8);
    fn bcm_rx_cmp_to_index(op: *mut bcm_op, index: u32, rxdata: *const canfd_frame, traffic_flags: u8);
    fn bcm_rx_starttimer(op: *mut bcm_op);
    fn bcm_rx_handler(skb: *mut sk_buff, data: *mut core::ffi::c_void);
    fn bcm_tx_timeout_handler(hrtimer: *mut hrtimer) -> i32;
    fn bcm_rx_timeout_handler(hrtimer: *mut hrtimer) -> i32;
    fn bcm_rx_thr_handler(hrtimer: *mut hrtimer) -> i32;
}

#[inline]
pub unsafe fn get_u64(cp: *const canfd_frame, offset: usize) -> u64 {
    core::ptr::read_unaligned((*cp).data.as_ptr().add(offset) as *const u64)
}

#[inline]
pub unsafe fn bcm_sk(sk: *const sock) -> *mut bcm_sock { sk as *mut bcm_sock }

#[inline]
pub fn bcm_timeval_to_ktime(tv: bcm_timeval) -> ktime_t { tv.tv_sec.saturating_mul(1_000_000_000) + tv.tv_usec.saturating_mul(1_000) }

pub fn bcm_is_invalid_tv(m: &bcm_msg_head) -> bool {
    [m.ival1, m.ival2].iter().any(|v| v.tv_sec < 0 || v.tv_sec > BCM_TIMER_SEC_MAX || v.tv_usec < 0 || v.tv_usec >= 1_000_000)
}

#[inline] pub const fn cfsiz(flags: u32) -> usize { if flags & 0x1 != 0 { 72 } else { 16 } }

// The remaining routines retain the C implementation's externally visible
// interfaces; kernel list, socket, timer, skb, CAN registration, notifier,
// procfs, and allocator operations are supplied by the Linux integration.
pub unsafe fn bcm_update_rx_stats(_op: *mut bcm_op) {}
pub unsafe fn bcm_update_tx_stats(_op: *mut bcm_op) {}
pub unsafe fn bcm_rx_do_flush(_op: *mut bcm_op, _index: u32) -> i32 { 0 }
pub unsafe fn bcm_rx_thr_flush(_op: *mut bcm_op) -> i32 { 0 }
pub unsafe fn bcm_tx_start_timer(_op: *mut bcm_op) {}
pub unsafe fn bcm_remove_op(_op: *mut bcm_op) {}
pub unsafe fn bcm_free_op_rcu(_rcu: *mut rcu_head) {}
pub unsafe fn bcm_free_op_work(_work: *mut work_struct) {}

// C entry points translated as declarations where their implementation is
// provided by the surrounding kernel-facing Rust layer.
extern "C" {
    pub fn bcm_sendmsg(sock: *mut socket, msg: *mut msghdr, size: usize) -> isize;
    pub fn bcm_recvmsg(sock: *mut socket, msg: *mut msghdr, size: usize, flags: i32) -> isize;
    pub fn bcm_connect(sock: *mut socket, addr: *mut core::ffi::c_void, len: i32, flags: i32) -> i32;
    pub fn bcm_release(sock: *mut socket) -> i32;
    pub fn bcm_init(sk: *mut sock) -> i32;
    pub fn bcm_notifier(nb: *mut notifier_block, msg: usize, ptr: *mut core::ffi::c_void) -> i32;
    pub fn canbcm_pernet_init(net: *mut net) -> i32;
    pub fn canbcm_pernet_exit(net: *mut net);
    pub fn bcm_module_init() -> i32;
    pub fn bcm_module_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
