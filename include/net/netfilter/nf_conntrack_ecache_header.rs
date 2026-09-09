/* SPDX-License-Identifier: GPL-2.0 */
/* Connection tracking event cache. */

/* C header dependencies are supplied by other translated units. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_ct_ecache_state {
    NFCT_ECACHE_DESTROY_FAIL,
    NFCT_ECACHE_DESTROY_SENT,
}

#[repr(C)]
pub struct nf_conntrack_ecache {
    pub cache: ::core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP")]
    pub timestamp: local64_t,
    pub ctmask: u16,
    pub expmask: u16,
    pub missed: u32,
    pub portid: u32,
}

#[inline]
pub unsafe fn nf_ct_ecache_find(ct: *const nf_conn) -> *mut nf_conntrack_ecache {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
    {
        return nf_ct_ext_find(ct, NF_CT_EXT_ECACHE) as *mut nf_conntrack_ecache;
    }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
    { ::core::ptr::null_mut() }
}

#[inline]
pub unsafe fn nf_ct_ecache_exist(ct: *const nf_conn) -> bool {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
    { nf_ct_ext_exist(ct, NF_CT_EXT_ECACHE) }
    #[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
    { false }
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
#[repr(C)]
pub struct nf_ct_event {
    pub ct: *mut nf_conn,
    pub portid: u32,
    pub report: ::core::ffi::c_int,
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
#[repr(C)]
pub struct nf_exp_event {
    pub exp: *mut nf_conntrack_expect,
    pub portid: u32,
    pub report: ::core::ffi::c_int,
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
#[repr(C)]
pub struct nf_ct_event_notifier {
    pub ct_event: Option<unsafe extern "C" fn(u32, *const nf_ct_event) -> ::core::ffi::c_int>,
    pub exp_event: Option<unsafe extern "C" fn(u32, *const nf_exp_event) -> ::core::ffi::c_int>,
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
extern "C" {
    pub fn nf_conntrack_register_notifier(net: *mut net, nb: *const nf_ct_event_notifier);
    pub fn nf_conntrack_unregister_notifier(net: *mut net);
    pub fn nf_ct_deliver_cached_events(ct: *mut nf_conn);
    pub fn nf_conntrack_eventmask_report(eventmask: u32, ct: *mut nf_conn, portid: u32, report: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn nf_ct_ecache_ext_add(ct: *mut nf_conn, ctmask: u16, expmask: u16, gfp: gfp_t) -> bool;
}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
#[inline]
pub unsafe fn nf_ct_deliver_cached_events(_ct: *const nf_conn) {}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
#[inline]
pub unsafe fn nf_conntrack_eventmask_report(_eventmask: u32, _ct: *mut nf_conn, _portid: u32, _report: ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
#[inline]
pub unsafe fn nf_ct_ecache_ext_add(_ct: *mut nf_conn, _ctmask: u16, _expmask: u16, _gfp: gfp_t) -> bool { false }

#[inline]
pub unsafe fn nf_conntrack_event_cache(event: ip_conntrack_events, ct: *mut nf_conn) {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
    {
        let net = nf_ct_net(ct);
        let e = nf_ct_ecache_find(ct);
        if e.is_null() { return; }
        #[cfg(feature = "CONFIG_NF_CONNTRACK_TIMESTAMP")]
        {
            /* Renew only for the first cached event. */
            if local64_read(&(*e).timestamp) != 0 && READ_ONCE((*e).cache) == 0 {
                local64_set(&mut (*e).timestamp, ktime_get_real_ns());
            }
        }
        set_bit(event as usize, &mut (*e).cache);
    }
}

#[inline]
pub unsafe fn nf_conntrack_event_report(event: ip_conntrack_events, ct: *mut nf_conn, portid: u32, report: ::core::ffi::c_int) -> ::core::ffi::c_int {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
    { if nf_ct_ecache_exist(ct) { return nf_conntrack_eventmask_report(1u32.wrapping_shl(event as u32), ct, portid, report); } }
    0
}

#[inline]
pub unsafe fn nf_conntrack_event(event: ip_conntrack_events, ct: *mut nf_conn) -> ::core::ffi::c_int {
    #[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
    { if nf_ct_ecache_exist(ct) { return nf_conntrack_eventmask_report(1u32.wrapping_shl(event as u32), ct, 0, 0); } }
    0
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
extern "C" {
    pub fn nf_ct_expect_event_report(event: ip_conntrack_expect_events, exp: *mut nf_conntrack_expect, portid: u32, report: ::core::ffi::c_int);
    pub fn nf_conntrack_ecache_work(net: *mut net, state: nf_ct_ecache_state);
    pub fn nf_conntrack_ecache_pernet_init(net: *mut net);
    pub fn nf_conntrack_ecache_pernet_fini(net: *mut net);
    pub fn nf_conn_pernet_ecache(net: *const net) -> *mut nf_conntrack_net_ecache;
}

#[cfg(feature = "CONFIG_NF_CONNTRACK_EVENTS")]
#[inline]
pub unsafe fn nf_conntrack_ecache_dwork_pending(net: *const net) -> bool { (*net).ct.ecache_dwork_pending }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
#[inline]
pub unsafe fn nf_ct_expect_event_report(_e: ip_conntrack_expect_events, _exp: *mut nf_conntrack_expect, _portid: u32, _report: ::core::ffi::c_int) {}
#[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
#[inline]
pub unsafe fn nf_conntrack_ecache_work(_net: *mut net, _s: nf_ct_ecache_state) {}
#[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
#[inline]
pub unsafe fn nf_conntrack_ecache_pernet_init(_net: *mut net) {}
#[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
#[inline]
pub unsafe fn nf_conntrack_ecache_pernet_fini(_net: *mut net) {}
#[cfg(not(feature = "CONFIG_NF_CONNTRACK_EVENTS"))]
#[inline]
pub unsafe fn nf_conntrack_ecache_dwork_pending(_net: *const net) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
