// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Security-Enhanced Linux (SELinux) security module
 *
 *  This file contains the SELinux XFRM hook function implementations.
 *
 *  Authors:  Serge Hallyn <sergeh@us.ibm.com>
 *	      Trent Jaeger <jaegert@us.ibm.com>
 *
 *  Updated: Venkat Yekkirala <vyekkirala@TrustedCS.com>
 *
 *           Granular IPSec Associations for use in MLS environments.
 *
 *  Copyright (C) 2005 International Business Machines Corporation
 *  Copyright (C) 2006 Trusted Computer Solutions, Inc.
 */

/*
 * USAGE:
 * NOTES:
 *   1. Make sure to enable the following options in your kernel config:
 *	CONFIG_SECURITY=y
 *	CONFIG_SECURITY_NETWORK=y
 *	CONFIG_SECURITY_NETWORK_XFRM=y
 *	CONFIG_SECURITY_SELINUX=m/y
 * ISSUES:
 *   1. Caching packets, so they are not dropped during negotiation
 *   2. Emulating a reasonable SO_PEERSEC across machines
 *   3. Testing addition of sk_policy's with security context via setsockopt
 */

pub type c_int = i32;
pub type c_char = i8;
pub type c_void = core::ffi::c_void;
pub type u8 = u8;
pub type u32 = u32;
pub type gfp_t = u32;

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const ESRCH: c_int = 3;
pub const PAGE_SIZE: u32 = 4096;
pub const XFRM_SC_DOI_LSM: u8 = 1;
pub const XFRM_SC_ALG_SELINUX: u8 = 1;
pub const SECCLASS_ASSOCIATION: u16 = 0;
pub const ASSOCIATION__SETCONTEXT: u32 = 0;
pub const ASSOCIATION__POLMATCH: u32 = 0;
pub const ASSOCIATION__SENDTO: u32 = 0;
pub const ASSOCIATION__RECVFROM: u32 = 0;
pub const SECSID_NULL: u32 = 0;
pub const SECINITSID_UNLABELED: u32 = 0;
pub const GFP_ATOMIC: gfp_t = 0;
pub const GFP_KERNEL: gfp_t = 0;
pub const IPPROTO_AH: u8 = 51;
pub const IPPROTO_ESP: u8 = 50;
pub const IPPROTO_COMP: u8 = 108;

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

#[repr(C)]
pub struct xfrm_sec_ctx {
    pub ctx_doi: u8,
    pub ctx_alg: u8,
    pub ctx_len: u32,
    pub ctx_sid: u32,
    pub ctx_str: [c_char; 0],
}

#[repr(C)]
pub struct xfrm_user_sec_ctx {
    pub ctx_doi: u16,
    pub ctx_alg: u16,
    pub ctx_len: u16,
}

#[repr(C)]
pub struct xfrm_state {
    pub security: *mut xfrm_sec_ctx,
}

#[repr(C)]
pub struct xfrm_policy {
    pub security: *mut xfrm_sec_ctx,
}

#[repr(C)]
pub struct flowi_common {
    pub flowic_secid: u32,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dst_entry {
    pub xfrm: *mut xfrm_state,
}

#[repr(C)]
pub struct sec_path {
    pub len: c_int,
    pub xvec: [*mut xfrm_state; 0],
}

#[repr(C)]
pub struct common_audit_data {
    _private: [u8; 0],
}

// Labeled XFRM instance counter
#[no_mangle]
pub static mut selinux_xfrm_refcount: atomic_t = atomic_t { counter: 0 };

unsafe extern "C" {
    fn kmalloc_flex_xfrm_sec_ctx_ctx_str(count: usize, flags: gfp_t) -> *mut xfrm_sec_ctx;
    fn kmemdup(src: *const c_void, len: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn security_context_to_sid(
        scontext: *const c_char,
        scontext_len: u32,
        sid: *mut u32,
        gfp: gfp_t,
    ) -> c_int;
    fn security_sid_to_context(sid: u32, scontext: *mut *mut c_char, scontext_len: *mut u32)
        -> c_int;
    fn avc_has_perm(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        requested: u32,
        auditdata: *mut common_audit_data,
    ) -> c_int;
    fn current_sid() -> u32;
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_dec(v: *mut atomic_t);
    fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry;
    fn skb_sec_path(skb: *mut sk_buff) -> *mut sec_path;
    fn xfrm_dst_child(dst: *mut dst_entry) -> *mut dst_entry;
}

/*
 * Returns true if the context is an LSM/SELinux context.
 */
#[inline]
unsafe fn selinux_authorizable_ctx(ctx: *mut xfrm_sec_ctx) -> c_int {
    (!ctx.is_null()
        && (*ctx).ctx_doi as c_int == XFRM_SC_DOI_LSM as c_int
        && (*ctx).ctx_alg as c_int == XFRM_SC_ALG_SELINUX as c_int) as c_int
}

/*
 * Returns true if the xfrm contains a security blob for SELinux.
 */
#[inline]
unsafe fn selinux_authorizable_xfrm(x: *mut xfrm_state) -> c_int {
    selinux_authorizable_ctx((*x).security)
}

/*
 * Allocates a xfrm_sec_state and populates it using the supplied security
 * xfrm_user_sec_ctx context.
 */
unsafe fn selinux_xfrm_alloc_user(
    ctxp: *mut *mut xfrm_sec_ctx,
    uctx: *mut xfrm_user_sec_ctx,
    gfp: gfp_t,
) -> c_int {
    let mut rc: c_int;
    let mut ctx: *mut xfrm_sec_ctx = core::ptr::null_mut();
    let str_len: u32;

    if ctxp.is_null()
        || uctx.is_null()
        || (*uctx).ctx_doi as c_int != XFRM_SC_DOI_LSM as c_int
        || (*uctx).ctx_alg as c_int != XFRM_SC_ALG_SELINUX as c_int
    {
        return -EINVAL;
    }

    str_len = (*uctx).ctx_len as u32;
    if str_len >= PAGE_SIZE {
        return -ENOMEM;
    }

    ctx = kmalloc_flex_xfrm_sec_ctx_ctx_str(str_len.wrapping_add(1) as usize, gfp);
    if ctx.is_null() {
        return -ENOMEM;
    }

    (*ctx).ctx_doi = XFRM_SC_DOI_LSM;
    (*ctx).ctx_alg = XFRM_SC_ALG_SELINUX;
    (*ctx).ctx_len = str_len.wrapping_add(1);
    memcpy(
        (*ctx).ctx_str.as_mut_ptr() as *mut c_void,
        uctx.add(1) as *const c_void,
        str_len as usize,
    );
    *(*ctx).ctx_str.as_mut_ptr().add(str_len as usize) = b'\0' as c_char;
    rc = security_context_to_sid((*ctx).ctx_str.as_ptr(), str_len, &mut (*ctx).ctx_sid, gfp);
    if rc != 0 {
        kfree(ctx as *const c_void);
        return rc;
    }

    rc = avc_has_perm(
        current_sid(),
        (*ctx).ctx_sid,
        SECCLASS_ASSOCIATION,
        ASSOCIATION__SETCONTEXT,
        core::ptr::null_mut(),
    );
    if rc != 0 {
        kfree(ctx as *const c_void);
        return rc;
    }

    *ctxp = ctx;
    atomic_inc(&raw mut selinux_xfrm_refcount);
    0
}

/*
 * Free the xfrm_sec_ctx structure.
 */
unsafe fn selinux_xfrm_free(ctx: *mut xfrm_sec_ctx) {
    if ctx.is_null() {
        return;
    }

    atomic_dec(&raw mut selinux_xfrm_refcount);
    kfree(ctx as *const c_void);
}

/*
 * Authorize the deletion of a labeled SA or policy rule.
 */
unsafe fn selinux_xfrm_delete(ctx: *mut xfrm_sec_ctx) -> c_int {
    if ctx.is_null() {
        return 0;
    }

    avc_has_perm(
        current_sid(),
        (*ctx).ctx_sid,
        SECCLASS_ASSOCIATION,
        ASSOCIATION__SETCONTEXT,
        core::ptr::null_mut(),
    )
}

/*
 * LSM hook implementation that authorizes that a flow can use a xfrm policy
 * rule.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_policy_lookup(ctx: *mut xfrm_sec_ctx, fl_secid: u32) -> c_int {
    let rc: c_int;

    /* All flows should be treated as polmatch'ing an otherwise applicable
     * "non-labeled" policy. This would prevent inadvertent "leaks". */
    if ctx.is_null() {
        return 0;
    }

    /* Context sid is either set to label or ANY_ASSOC */
    if selinux_authorizable_ctx(ctx) == 0 {
        return -EINVAL;
    }

    rc = avc_has_perm(
        fl_secid,
        (*ctx).ctx_sid,
        SECCLASS_ASSOCIATION,
        ASSOCIATION__POLMATCH,
        core::ptr::null_mut(),
    );
    if rc == -EACCES { -ESRCH } else { rc }
}

/*
 * LSM hook implementation that authorizes that a state matches
 * the given policy, flow combo.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_state_pol_flow_match(
    x: *mut xfrm_state,
    xp: *mut xfrm_policy,
    flic: *const flowi_common,
) -> c_int {
    let state_sid: u32;
    let flic_sid: u32;

    if (*xp).security.is_null() {
        if !(*x).security.is_null() {
            /* unlabeled policy and labeled SA can't match */
            return 0;
        } else {
            /* unlabeled policy and unlabeled SA match all flows */
            return 1;
        }
    } else if (*x).security.is_null() {
        /* unlabeled SA and labeled policy can't match */
        return 0;
    } else if selinux_authorizable_xfrm(x) == 0 {
        /* Not a SELinux-labeled SA */
        return 0;
    }

    state_sid = (*(*x).security).ctx_sid;
    flic_sid = (*flic).flowic_secid;

    if flic_sid != state_sid {
        return 0;
    }

    /* We don't need a separate SA Vs. policy polmatch check since the SA
     * is now of the same label as the flow and a flow Vs. policy polmatch
     * check had already happened in selinux_xfrm_policy_lookup() above. */
    if avc_has_perm(
        flic_sid,
        state_sid,
        SECCLASS_ASSOCIATION,
        ASSOCIATION__SENDTO,
        core::ptr::null_mut(),
    ) != 0
    {
        0
    } else {
        1
    }
}

unsafe fn selinux_xfrm_skb_sid_egress(skb: *mut sk_buff) -> u32 {
    let dst: *mut dst_entry = skb_dst(skb);
    let x: *mut xfrm_state;

    if dst.is_null() {
        return SECSID_NULL;
    }
    x = (*dst).xfrm;
    if x.is_null() || selinux_authorizable_xfrm(x) == 0 {
        return SECSID_NULL;
    }

    (*(*x).security).ctx_sid
}

unsafe fn selinux_xfrm_skb_sid_ingress(skb: *mut sk_buff, sid: *mut u32, ckall: c_int) -> c_int {
    let mut sid_session: u32 = SECSID_NULL;
    let sp: *mut sec_path = skb_sec_path(skb);

    if !sp.is_null() {
        let mut i: c_int;

        i = (*sp).len - 1;
        while i >= 0 {
            let x: *mut xfrm_state = *(*sp).xvec.as_ptr().add(i as usize);
            if selinux_authorizable_xfrm(x) != 0 {
                let ctx: *mut xfrm_sec_ctx = (*x).security;

                if sid_session == SECSID_NULL {
                    sid_session = (*ctx).ctx_sid;
                    if ckall == 0 {
                        break;
                    }
                } else if sid_session != (*ctx).ctx_sid {
                    *sid = SECSID_NULL;
                    return -EINVAL;
                }
            }
            i -= 1;
        }
    }

    *sid = sid_session;
    0
}

/*
 * LSM hook implementation that checks and/or returns the xfrm sid for the
 * incoming packet.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_decode_session(
    skb: *mut sk_buff,
    sid: *mut u32,
    ckall: c_int,
) -> c_int {
    if skb.is_null() {
        *sid = SECSID_NULL;
        return 0;
    }
    selinux_xfrm_skb_sid_ingress(skb, sid, ckall)
}

#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_skb_sid(skb: *mut sk_buff, sid: *mut u32) -> c_int {
    let rc: c_int;

    rc = selinux_xfrm_skb_sid_ingress(skb, sid, 0);
    if rc == 0 && *sid == SECSID_NULL {
        *sid = selinux_xfrm_skb_sid_egress(skb);
    }

    rc
}

/*
 * LSM hook implementation that allocs and transfers uctx spec to xfrm_policy.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_policy_alloc(
    ctxp: *mut *mut xfrm_sec_ctx,
    uctx: *mut xfrm_user_sec_ctx,
    gfp: gfp_t,
) -> c_int {
    selinux_xfrm_alloc_user(ctxp, uctx, gfp)
}

/*
 * LSM hook implementation that copies security data structure from old to new
 * for policy cloning.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_policy_clone(
    old_ctx: *mut xfrm_sec_ctx,
    new_ctxp: *mut *mut xfrm_sec_ctx,
) -> c_int {
    let new_ctx: *mut xfrm_sec_ctx;

    if old_ctx.is_null() {
        return 0;
    }

    new_ctx = kmemdup(
        old_ctx as *const c_void,
        core::mem::size_of::<xfrm_sec_ctx>() + (*old_ctx).ctx_len as usize,
        GFP_ATOMIC,
    ) as *mut xfrm_sec_ctx;
    if new_ctx.is_null() {
        return -ENOMEM;
    }
    atomic_inc(&raw mut selinux_xfrm_refcount);
    *new_ctxp = new_ctx;

    0
}

/*
 * LSM hook implementation that frees xfrm_sec_ctx security information.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_policy_free(ctx: *mut xfrm_sec_ctx) {
    selinux_xfrm_free(ctx);
}

/*
 * LSM hook implementation that authorizes deletion of labeled policies.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_policy_delete(ctx: *mut xfrm_sec_ctx) -> c_int {
    selinux_xfrm_delete(ctx)
}

/*
 * LSM hook implementation that allocates a xfrm_sec_state, populates it using
 * the supplied security context, and assigns it to the xfrm_state.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_state_alloc(
    x: *mut xfrm_state,
    uctx: *mut xfrm_user_sec_ctx,
) -> c_int {
    selinux_xfrm_alloc_user(&mut (*x).security, uctx, GFP_KERNEL)
}

/*
 * LSM hook implementation that allocates a xfrm_sec_state and populates based
 * on a secid.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_state_alloc_acquire(
    x: *mut xfrm_state,
    polsec: *mut xfrm_sec_ctx,
    secid: u32,
) -> c_int {
    let mut rc: c_int;
    let ctx: *mut xfrm_sec_ctx;
    let mut ctx_str: *mut c_char = core::ptr::null_mut();
    let mut str_len: u32 = 0;

    if polsec.is_null() {
        return 0;
    }

    if secid == 0 {
        return -EINVAL;
    }

    rc = security_sid_to_context(secid, &mut ctx_str, &mut str_len);
    if rc != 0 {
        return rc;
    }

    ctx = kmalloc_flex_xfrm_sec_ctx_ctx_str(str_len as usize, GFP_ATOMIC);
    if ctx.is_null() {
        rc = -ENOMEM;
        kfree(ctx_str as *const c_void);
        return rc;
    }

    (*ctx).ctx_doi = XFRM_SC_DOI_LSM;
    (*ctx).ctx_alg = XFRM_SC_ALG_SELINUX;
    (*ctx).ctx_sid = secid;
    (*ctx).ctx_len = str_len;
    memcpy(
        (*ctx).ctx_str.as_mut_ptr() as *mut c_void,
        ctx_str as *const c_void,
        str_len as usize,
    );

    (*x).security = ctx;
    atomic_inc(&raw mut selinux_xfrm_refcount);
    kfree(ctx_str as *const c_void);
    rc
}

/*
 * LSM hook implementation that frees xfrm_state security information.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_state_free(x: *mut xfrm_state) {
    selinux_xfrm_free((*x).security);
}

/*
 * LSM hook implementation that authorizes deletion of labeled SAs.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_state_delete(x: *mut xfrm_state) -> c_int {
    selinux_xfrm_delete((*x).security)
}

/*
 * LSM hook that controls access to unlabelled packets.  If
 * a xfrm_state is authorizable (defined by macro) then it was
 * already authorized by the IPSec process.  If not, then
 * we need to check for unlabelled access since this may not have
 * gone thru the IPSec process.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_sock_rcv_skb(
    sk_sid: u32,
    skb: *mut sk_buff,
    ad: *mut common_audit_data,
) -> c_int {
    let mut i: c_int;
    let sp: *mut sec_path = skb_sec_path(skb);
    let mut peer_sid: u32 = SECINITSID_UNLABELED;

    if !sp.is_null() {
        i = 0;
        while i < (*sp).len {
            let x: *mut xfrm_state = *(*sp).xvec.as_ptr().add(i as usize);

            if !x.is_null() && selinux_authorizable_xfrm(x) != 0 {
                let ctx: *mut xfrm_sec_ctx = (*x).security;
                peer_sid = (*ctx).ctx_sid;
                break;
            }
            i += 1;
        }
    }

    /* This check even when there's no association involved is intended,
     * according to Trent Jaeger, to make sure a process can't engage in
     * non-IPsec communication unless explicitly allowed by policy. */
    avc_has_perm(
        sk_sid,
        peer_sid,
        SECCLASS_ASSOCIATION,
        ASSOCIATION__RECVFROM,
        ad,
    )
}

/*
 * POSTROUTE_LAST hook's XFRM processing:
 * If we have no security association, then we need to determine
 * whether the socket is allowed to send to an unlabelled destination.
 * If we do have a authorizable security association, then it has already been
 * checked in the selinux_xfrm_state_pol_flow_match hook above.
 */
#[no_mangle]
pub unsafe extern "C" fn selinux_xfrm_postroute_last(
    sk_sid: u32,
    skb: *mut sk_buff,
    ad: *mut common_audit_data,
    proto: u8,
) -> c_int {
    let dst: *mut dst_entry;

    match proto {
        IPPROTO_AH | IPPROTO_ESP | IPPROTO_COMP => {
            /* We should have already seen this packet once before it
             * underwent xfrm(s). No need to subject it to the unlabeled
             * check. */
            return 0;
        }
        _ => {}
    }

    dst = skb_dst(skb);
    if !dst.is_null() {
        let mut iter: *mut dst_entry;

        iter = dst;
        while !iter.is_null() {
            let x: *mut xfrm_state = (*iter).xfrm;

            if !x.is_null() && selinux_authorizable_xfrm(x) != 0 {
                return 0;
            }
            iter = xfrm_dst_child(iter);
        }
    }

    /* This check even when there's no association involved is intended,
     * according to Trent Jaeger, to make sure a process can't engage in
     * non-IPsec communication unless explicitly allowed by policy. */
    avc_has_perm(
        sk_sid,
        SECINITSID_UNLABELED,
        SECCLASS_ASSOCIATION,
        ASSOCIATION__SENDTO,
        ad,
    )
}



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
