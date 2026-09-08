/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Access vector cache interface for object managers.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/*
 * C header dependencies:
 * linux/stddef.h, linux/errno.h, linux/kernel.h, linux/kdev_t.h,
 * linux/spinlock.h, linux/init.h, linux/audit.h, linux/lsm_audit.h,
 * linux/in6.h, flask.h, av_permissions.h, security.h
 */

use core::ffi::{c_char, c_int};

/*
 * An entry in the AVC.
 */
#[repr(C)]
pub struct avc_entry {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _unused: [u8; 0],
}

/*
 * External dependency types supplied by included headers.
 */
#[repr(C)]
pub struct av_decision {
    pub allowed: u32,
    pub auditallow: u32,
    pub auditdeny: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct common_audit_data {
    _unused: [u8; 0],
}

/*
 * AVC statistics
 */
#[repr(C)]
pub struct avc_cache_stats {
    pub lookups: u32,
    pub misses: u32,
    pub allocations: u32,
    pub reclaims: u32,
    pub frees: u32,
}

/*
 * We only need this data after we have decided to send an audit message.
 */
/* __randomize_layout */
#[repr(C)]
pub struct selinux_audit_data {
    pub ssid: u32,
    pub tsid: u32,
    pub tclass: u16,
    pub requested: u32,
    pub audited: u32,
    pub denied: u32,
    pub result: c_int,
}

/*
 * AVC operations
 */

unsafe extern "C" {
    /* __init */
    pub fn avc_init();
}

pub unsafe fn avc_audit_required(
    requested: u32,
    avd: *mut av_decision,
    result: c_int,
    auditdeny: u32,
    deniedp: *mut u32,
) -> u32 {
    let denied: u32;
    let audited: u32;

    if unsafe { (*avd).flags } & AVD_FLAGS_NEVERAUDIT != 0 {
        return 0;
    }

    denied = requested & !unsafe { (*avd).allowed };
    if denied != 0 {
        audited = denied & unsafe { (*avd).auditdeny };
        /*
         * auditdeny is TRICKY!  Setting a bit in
         * this field means that ANY denials should NOT be audited if
         * the policy contains an explicit dontaudit rule for that
         * permission.  Take notice that this is unrelated to the
         * actual permissions that were denied.  As an example lets
         * assume:
         *
         * denied == READ
         * avd.auditdeny & ACCESS == 0 (not set means explicit rule)
         * auditdeny & ACCESS == 1
         *
         * We will NOT audit the denial even though the denied
         * permission was READ and the auditdeny checks were for
         * ACCESS
         */
        if auditdeny != 0 && (auditdeny & unsafe { (*avd).auditdeny }) == 0 {
            audited = 0;
        }
    } else if result != 0 {
        denied = requested;
        audited = requested;
    } else {
        audited = requested & unsafe { (*avd).auditallow };
    }
    unsafe {
        *deniedp = denied;
    }
    audited
}

unsafe extern "C" {
    pub fn slow_avc_audit(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        requested: u32,
        audited: u32,
        denied: u32,
        result: c_int,
        a: *mut common_audit_data,
    ) -> c_int;
}

/**
 * avc_audit - Audit the granting or denial of permissions.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 * @requested: requested permissions
 * @avd: access vector decisions
 * @result: result from avc_has_perm_noaudit
 * @a:  auxiliary audit data
 *
 * Audit the granting or denial of permissions in accordance
 * with the policy.  This function is typically called by
 * avc_has_perm() after a permission check, but can also be
 * called directly by callers who use avc_has_perm_noaudit()
 * in order to separate the permission check from the auditing.
 * For example, this separation is useful when the permission check must
 * be performed under a lock, to allow the lock to be released
 * before calling the auditing code.
 */
pub unsafe fn avc_audit(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    requested: u32,
    avd: *mut av_decision,
    result: c_int,
    a: *mut common_audit_data,
) -> c_int {
    let mut denied: u32 = 0;
    let audited: u32 = unsafe { avc_audit_required(requested, avd, result, 0, &mut denied) };
    if audited == 0 {
        return 0;
    }
    unsafe { slow_avc_audit(ssid, tsid, tclass, requested, audited, denied, result, a) }
}

pub const AVC_STRICT: u32 = 1; /* Ignore permissive mode. */
pub const AVC_EXTENDED_PERMS: u32 = 2; /* update extended permissions */

unsafe extern "C" {
    pub fn avc_has_perm_noaudit(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        requested: u32,
        flags: u32,
        avd: *mut av_decision,
    ) -> c_int;

    pub fn avc_has_perm(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        requested: u32,
        auditdata: *mut common_audit_data,
    ) -> c_int;
}

pub const AVC_EXT_IOCTL: u32 = 1 << 0; /* Cache entry for an ioctl extended permission */
pub const AVC_EXT_NLMSG: u32 = 1 << 1; /* Cache entry for an nlmsg extended permission */

unsafe extern "C" {
    pub fn avc_has_extended_perms(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        requested: u32,
        driver: u8,
        base_perm: u8,
        perm: u8,
        ad: *mut common_audit_data,
    ) -> c_int;

    pub fn avc_policy_seqno() -> u32;
}

pub const AVC_CALLBACK_GRANT: u32 = 1;
pub const AVC_CALLBACK_TRY_REVOKE: u32 = 2;
pub const AVC_CALLBACK_REVOKE: u32 = 4;
pub const AVC_CALLBACK_RESET: u32 = 8;
pub const AVC_CALLBACK_AUDITALLOW_ENABLE: u32 = 16;
pub const AVC_CALLBACK_AUDITALLOW_DISABLE: u32 = 32;
pub const AVC_CALLBACK_AUDITDENY_ENABLE: u32 = 64;
pub const AVC_CALLBACK_AUDITDENY_DISABLE: u32 = 128;
pub const AVC_CALLBACK_ADD_XPERMS: u32 = 256;

unsafe extern "C" {
    pub fn avc_add_callback(
        callback: Option<unsafe extern "C" fn(event: u32) -> c_int>,
        events: u32,
    ) -> c_int;

    /* Exported to selinuxfs */
    pub fn avc_get_hash_stats(page: *mut c_char) -> c_int;
    pub fn avc_get_cache_threshold() -> u32;
    pub fn avc_set_cache_threshold(cache_threshold: u32);
}

/* CONFIG_SECURITY_SELINUX_AVC_STATS */
#[cfg(CONFIG_SECURITY_SELINUX_AVC_STATS)]
unsafe extern "C" {
    pub static mut avc_cache_stats: avc_cache_stats;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
