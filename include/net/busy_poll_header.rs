/* SPDX-License-Identifier: GPL-2.0-only */
/* net busy poll support; translated from the C header. */

// C dependencies: linux/netdevice.h, linux/sched/clock.h,
// linux/sched/signal.h, net/ip.h, and net/xdp.h.

pub const MIN_NAPI_ID: ::core::ffi::c_uint = NR_CPUS as ::core::ffi::c_uint + 1;

#[inline]
pub fn napi_id_valid(napi_id: ::core::ffi::c_uint) -> bool {
    napi_id >= MIN_NAPI_ID
}

pub const BUSY_POLL_BUDGET: u16 = 8;

// CONFIG_NET_RX_BUSY_POLL is a build-time configuration condition.
#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
pub struct napi_struct {
    pub gro: gro_node,
}

#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
extern "C" {
    pub static mut sysctl_net_busy_read: ::core::ffi::c_uint;
    pub static mut sysctl_net_busy_poll: ::core::ffi::c_uint;
}

#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
#[inline]
pub unsafe fn net_busy_loop_on() -> bool {
    core::ptr::read_volatile(&sysctl_net_busy_poll) != 0
}

#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
#[inline]
pub unsafe fn sk_can_busy_loop(sk: *const sock) -> bool {
    core::ptr::read_volatile(&(*sk).sk_ll_usec) != 0 && !signal_pending(current)
}

#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
extern "C" {
    pub fn sk_busy_loop_end(p: *mut ::core::ffi::c_void, start_time: ::core::ffi::c_ulong) -> bool;
    pub fn napi_busy_loop(
        napi_id: ::core::ffi::c_uint,
        loop_end: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_ulong) -> bool>,
        loop_end_arg: *mut ::core::ffi::c_void,
        prefer_busy_poll: bool,
        budget: u16,
    );
    pub fn napi_busy_loop_rcu(
        napi_id: ::core::ffi::c_uint,
        loop_end: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_ulong) -> bool>,
        loop_end_arg: *mut ::core::ffi::c_void,
        prefer_busy_poll: bool,
        budget: u16,
    );
    pub fn napi_suspend_irqs(napi_id: ::core::ffi::c_uint);
    pub fn napi_resume_irqs(napi_id: ::core::ffi::c_uint);
}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub fn net_busy_loop_on() -> ::core::ffi::c_ulong { 0 }

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
#[inline]
pub unsafe fn sk_can_busy_loop(_sk: *mut sock) -> bool { false }

#[inline]
pub unsafe fn busy_loop_current_time() -> ::core::ffi::c_ulong {
    #[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
    { (ktime_get_ns() >> 10) as ::core::ffi::c_ulong }
    #[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
    { 0 }
}

#[inline]
pub unsafe fn busy_loop_timeout(start_time: ::core::ffi::c_ulong) -> bool {
    #[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
    {
        let bp_usec = core::ptr::read_volatile(&sysctl_net_busy_poll) as ::core::ffi::c_ulong;
        if bp_usec != 0 {
            let end_time = start_time.wrapping_add(bp_usec);
            let now = busy_loop_current_time();
            return time_after(now, end_time);
        }
    }
    true
}

#[inline]
pub unsafe fn sk_busy_loop_timeout(sk: *mut sock, start_time: ::core::ffi::c_ulong) -> bool {
    #[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
    {
        let bp_usec = core::ptr::read_volatile(&(*sk).sk_ll_usec) as ::core::ffi::c_ulong;
        if bp_usec != 0 {
            let end_time = start_time.wrapping_add(bp_usec);
            let now = busy_loop_current_time();
            return time_after(now, end_time);
        }
    }
    true
}

#[inline]
pub unsafe fn sk_busy_loop(sk: *mut sock, nonblock: ::core::ffi::c_int) {
    #[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
    {
        let napi_id = core::ptr::read_volatile(&(*sk).sk_napi_id);
        if napi_id_valid(napi_id) {
            napi_busy_loop(napi_id, if nonblock != 0 { None } else { Some(sk_busy_loop_end) }, sk as *mut _, core::ptr::read_volatile(&(*sk).sk_prefer_busy_poll), {
                let budget = core::ptr::read_volatile(&(*sk).sk_busy_poll_budget);
                if budget != 0 { budget } else { BUSY_POLL_BUDGET }
            });
        }
    }
}

#[inline]
pub unsafe fn __skb_mark_napi_id(skb: *mut sk_buff, gro: *const gro_node) {
    #[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
    if !napi_id_valid((*skb).napi_id) { (*skb).napi_id = (*gro).cached_napi_id; }
}

#[inline]
pub unsafe fn skb_mark_napi_id(skb: *mut sk_buff, napi: *const napi_struct) { __skb_mark_napi_id(skb, &(*napi).gro); }

#[inline]
pub unsafe fn sk_mark_napi_id(sk: *mut sock, skb: *const sk_buff) {
    #[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
    if core::ptr::read_volatile(&(*sk).sk_napi_id) != (*skb).napi_id { core::ptr::write_volatile(&mut (*sk).sk_napi_id, (*skb).napi_id); }
    sk_rx_queue_update(sk, skb);
}

#[inline]
pub unsafe fn sk_mark_napi_id_set(sk: *mut sock, skb: *const sk_buff) {
    #[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
    core::ptr::write_volatile(&mut (*sk).sk_napi_id, (*skb).napi_id);
    sk_rx_queue_set(sk, skb);
}

#[inline]
pub unsafe fn __sk_mark_napi_id_once(sk: *mut sock, napi_id: ::core::ffi::c_uint) {
    #[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
    if core::ptr::read_volatile(&(*sk).sk_napi_id) == 0 { core::ptr::write_volatile(&mut (*sk).sk_napi_id, napi_id); }
}

#[inline]
pub unsafe fn sk_mark_napi_id_once(sk: *mut sock, skb: *const sk_buff) {
    #[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
    __sk_mark_napi_id_once(sk, (*skb).napi_id);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
