/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of `smc_tracepoint.h`.
//!
//! The original file is a Linux tracepoint header.  The tracepoint registration
//! and formatting macros are supplied by the kernel tracepoint subsystem; the
//! payload layouts and event assignment logic are represented here directly.

use core::ffi::c_void;

/* External kernel types referenced by the header. */
pub enum smc_sock {}
pub enum smc_link {}
pub enum smc_link_group {}
pub enum sock {}
pub enum net {}

#[repr(C)]
pub struct smc_switch_to_fallback_entry {
    pub sk: *const c_void,
    pub clcsk: *const c_void,
    pub net_cookie: u64,
    pub fallback_rsn: i32,
}

#[repr(C)]
pub struct smc_msg_event_entry {
    pub smc: *const c_void,
    pub net_cookie: u64,
    pub len: usize,
    /* __string(name, ...) is a dynamically sized tracepoint string. */
    pub name: *const u8,
}

#[repr(C)]
pub struct smcr_link_down_entry {
    pub lnk: *const c_void,
    pub lgr: *const c_void,
    pub net_cookie: u64,
    pub state: i32,
    /* __string(name, ...) is a dynamically sized tracepoint string. */
    pub name: *const u8,
    pub location: *mut c_void,
}

/*
 * TRACE_EVENT(smc_switch_to_fallback,
 *   TP_PROTO(const struct smc_sock *smc, int fallback_rsn),
 *   TP_ARGS(smc, fallback_rsn),
 *   TP_printk("sk=%p clcsk=%p net=%llu fallback_rsn=%d", ...))
 */
#[inline]
pub unsafe fn smc_switch_to_fallback_entry_from(
    sk: *const sock,
    clcsk: *const sock,
    net_cookie: u64,
    fallback_rsn: i32,
) -> smc_switch_to_fallback_entry {
    smc_switch_to_fallback_entry {
        sk: sk.cast(),
        clcsk: clcsk.cast(),
        net_cookie,
        fallback_rsn,
    }
}

/*
 * DECLARE_EVENT_CLASS(smc_msg_event,
 *   TP_PROTO(const struct smc_sock *smc, size_t len),
 *   TP_ARGS(smc, len),
 *   TP_printk("smc=%p net=%llu len=%zu dev=%s", ...))
 */
#[inline]
pub unsafe fn smc_msg_event_entry_from(
    smc: *const smc_sock,
    net_cookie: u64,
    len: usize,
    name: *const u8,
) -> smc_msg_event_entry {
    smc_msg_event_entry {
        smc: smc.cast(),
        net_cookie,
        len,
        name,
    }
}

/* DEFINE_EVENT(smc_msg_event, smc_tx_sendmsg, ...); */
#[inline]
pub unsafe fn smc_tx_sendmsg_entry_from(
    smc: *const smc_sock,
    net_cookie: u64,
    len: usize,
    name: *const u8,
) -> smc_msg_event_entry {
    smc_msg_event_entry_from(smc, net_cookie, len, name)
}

/* DEFINE_EVENT(smc_msg_event, smc_rx_recvmsg, ...); */
#[inline]
pub unsafe fn smc_rx_recvmsg_entry_from(
    smc: *const smc_sock,
    net_cookie: u64,
    len: usize,
    name: *const u8,
) -> smc_msg_event_entry {
    smc_msg_event_entry_from(smc, net_cookie, len, name)
}

/*
 * TRACE_EVENT(smcr_link_down,
 *   TP_PROTO(const struct smc_link *lnk, void *location),
 *   TP_ARGS(lnk, location),
 *   TP_printk("lnk=%p lgr=%p net=%llu state=%d dev=%s location=%pS", ...))
 */
#[inline]
pub unsafe fn smcr_link_down_entry_from(
    lnk: *const smc_link,
    lgr: *const smc_link_group,
    net_cookie: u64,
    state: i32,
    name: *const u8,
    location: *mut c_void,
) -> smcr_link_down_entry {
    smcr_link_down_entry {
        lnk: lnk.cast(),
        lgr: lgr.cast(),
        net_cookie,
        state,
        name,
        location,
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
