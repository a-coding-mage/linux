/* SPDX-License-Identifier: GPL-2.0 */
/*
 * SELinux support for the XFRM LSM hooks
 *
 * Author : Trent Jaeger, <jaegert@us.ibm.com>
 * Updated : Venkat Yekkirala, <vyekkirala@TrustedCS.com>
 */

/* Dependencies from the original C header:
 * #include <linux/lsm_audit.h>
 * #include <net/flow.h>
 * #include <net/xfrm.h>
 */

extern "C" {
    pub fn selinux_xfrm_policy_alloc(
        ctxp: *mut *mut xfrm_sec_ctx,
        uctx: *mut xfrm_user_sec_ctx,
        gfp: gfp_t,
    ) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_policy_clone(
        old_ctx: *mut xfrm_sec_ctx,
        new_ctxp: *mut *mut xfrm_sec_ctx,
    ) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_policy_free(ctx: *mut xfrm_sec_ctx);
    pub fn selinux_xfrm_policy_delete(ctx: *mut xfrm_sec_ctx) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_state_alloc(
        x: *mut xfrm_state,
        uctx: *mut xfrm_user_sec_ctx,
    ) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_state_alloc_acquire(
        x: *mut xfrm_state,
        polsec: *mut xfrm_sec_ctx,
        secid: u32,
    ) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_state_free(x: *mut xfrm_state);
    pub fn selinux_xfrm_state_delete(x: *mut xfrm_state) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_policy_lookup(
        ctx: *mut xfrm_sec_ctx,
        fl_secid: u32,
    ) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_state_pol_flow_match(
        x: *mut xfrm_state,
        xp: *mut xfrm_policy,
        flic: *const flowi_common,
    ) -> ::std::os::raw::c_int;
}

#[cfg(CONFIG_SECURITY_NETWORK_XFRM)]
extern "C" {
    pub static mut selinux_xfrm_refcount: atomic_t;

    pub fn atomic_read(v: *const atomic_t) -> ::std::os::raw::c_int;
    pub fn down_read(sem: *mut rw_semaphore);
    pub fn up_read(sem: *mut rw_semaphore);
    pub static mut net_rwsem: rw_semaphore;
    pub fn rt_genid_bump_all(net: *mut net);

    pub fn selinux_xfrm_sock_rcv_skb(
        sk_sid: u32,
        skb: *mut sk_buff,
        ad: *mut common_audit_data,
    ) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_postroute_last(
        sk_sid: u32,
        skb: *mut sk_buff,
        ad: *mut common_audit_data,
        proto: u8,
    ) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_decode_session(
        skb: *mut sk_buff,
        sid: *mut u32,
        ckall: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn selinux_xfrm_skb_sid(
        skb: *mut sk_buff,
        sid: *mut u32,
    ) -> ::std::os::raw::c_int;
}

#[cfg(CONFIG_SECURITY_NETWORK_XFRM)]
#[inline]
pub unsafe fn selinux_xfrm_enabled() -> ::std::os::raw::c_int {
    (atomic_read(&selinux_xfrm_refcount as *const atomic_t) > 0) as ::std::os::raw::c_int
}

#[cfg(CONFIG_SECURITY_NETWORK_XFRM)]
#[inline]
pub unsafe fn selinux_xfrm_notify_policyload() {
    let mut net: *mut net;

    down_read(&mut net_rwsem as *mut rw_semaphore);
    /* Original C uses for_each_net(net) to iterate every network namespace:
     *     rt_genid_bump_all(net);
     * The macro expansion and iteration state are external to this isolated
     * header, so the loop cannot be represented file-locally without those
     * dependencies.
     */
    let _ = &mut net;
    up_read(&mut net_rwsem as *mut rw_semaphore);
}

#[cfg(not(CONFIG_SECURITY_NETWORK_XFRM))]
#[inline]
pub unsafe fn selinux_xfrm_enabled() -> ::std::os::raw::c_int {
    0
}

#[cfg(not(CONFIG_SECURITY_NETWORK_XFRM))]
#[inline]
pub unsafe fn selinux_xfrm_sock_rcv_skb(
    sk_sid: u32,
    skb: *mut sk_buff,
    ad: *mut common_audit_data,
) -> ::std::os::raw::c_int {
    let _ = sk_sid;
    let _ = skb;
    let _ = ad;
    0
}

#[cfg(not(CONFIG_SECURITY_NETWORK_XFRM))]
#[inline]
pub unsafe fn selinux_xfrm_postroute_last(
    sk_sid: u32,
    skb: *mut sk_buff,
    ad: *mut common_audit_data,
    proto: u8,
) -> ::std::os::raw::c_int {
    let _ = sk_sid;
    let _ = skb;
    let _ = ad;
    let _ = proto;
    0
}

#[cfg(not(CONFIG_SECURITY_NETWORK_XFRM))]
#[inline]
pub unsafe fn selinux_xfrm_decode_session(
    skb: *mut sk_buff,
    sid: *mut u32,
    ckall: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let _ = skb;
    let _ = ckall;
    *sid = SECSID_NULL;
    0
}

#[cfg(not(CONFIG_SECURITY_NETWORK_XFRM))]
#[inline]
pub unsafe fn selinux_xfrm_notify_policyload() {}

#[cfg(not(CONFIG_SECURITY_NETWORK_XFRM))]
#[inline]
pub unsafe fn selinux_xfrm_skb_sid(
    skb: *mut sk_buff,
    sid: *mut u32,
) -> ::std::os::raw::c_int {
    let _ = skb;
    *sid = SECSID_NULL;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
