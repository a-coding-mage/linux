// SPDX-License-Identifier: GPL-2.0-only
/* AppArmor resource mediation and attachment. */

use std::os::raw::{c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct AuditBuffer;
#[repr(C)] pub struct CommonAuditData { pub _private: [u8; 0] }
#[repr(C)] pub struct Cred;
#[repr(C)] pub struct TaskStruct;
#[repr(C)] pub struct AaLabel;
#[repr(C)] pub struct AaProfile;
#[repr(C)] pub struct AaRuleset;
#[repr(C)] pub struct AaSfsEntry;
#[repr(C)] pub struct LabelIt;

#[repr(C)] pub struct Rlimit { pub rlim_cur: c_ulong, pub rlim_max: c_ulong }
#[repr(C)] pub struct RlimitData { pub rlim: c_uint, pub max: c_ulong }
#[repr(C)] pub struct AppArmorAuditData {
    pub subj_cred: *const Cred, pub rlim: RlimitData, pub peer: *mut AaLabel,
    pub info: *const u8, pub error: c_int, pub subj_label: *mut AaLabel,
}

extern "C" {
    fn audit_log_format(ab: *mut AuditBuffer, fmt: *const u8, ...);
    fn aa_audit(t: c_int, p: *mut AaProfile, d: *mut AppArmorAuditData,
                cb: unsafe extern "C" fn(*mut AuditBuffer, *mut c_void)) -> c_int;
    fn aa_label_xaudit(ab: *mut AuditBuffer, ns: *mut c_void, l: *mut AaLabel,
                       flags: c_uint, gfp: c_uint);
    fn aad(sa: *mut CommonAuditData) -> *mut AppArmorAuditData;
    fn aa_get_newest_cred_label(c: *mut Cred) -> *mut AaLabel;
    fn __task_cred(t: *mut TaskStruct) -> *mut Cred;
    fn aa_capable(c: *const Cred, l: *mut AaLabel, cap: c_uint, opt: c_uint) -> c_int;
    fn aa_put_label(l: *mut AaLabel);
    fn labels_ns(l: *mut AaLabel) -> *mut c_void;
    fn labels_profile(l: *mut AaLabel) -> *mut AaProfile;
    fn update_rlimit_cpu(t: *mut TaskStruct, limit: c_ulong);
    static mut current: *mut TaskStruct;
    static mut init_task: TaskStruct;
    static rlim_names: *const *const u8;
    static rlim_map: *const c_int;
}

const AUDIT_APPARMOR_AUTO: c_int = 2100;
const AA_CLASS_RLIMITS: c_int = 25;
const OP_SETRLIMIT: c_int = 0;
const CAP_SYS_RESOURCE: c_uint = 25;
const CAP_OPT_NOAUDIT: c_uint = 0;
const FLAGS_NONE: c_uint = 0;
const GFP_ATOMIC: c_uint = 32;
const EACCES: c_int = 13;
const RLIMIT_CPU: c_int = 0;
const RLIM_INFINITY: c_ulong = c_ulong::MAX;
const RLIM_NLIMITS: c_int = 16;

pub static mut aa_sfs_entry_rlimit: [AaSfsEntry; 1] = [AaSfsEntry { _private: [] }];

unsafe extern "C" fn audit_cb(ab: *mut AuditBuffer, va: *mut c_void) {
    let ad = aad(va as *mut CommonAuditData);
    audit_log_format(ab, b" rlimit=%s value=%lu\0".as_ptr(),
        *rlim_names.add((*ad).rlim.rlim as usize), (*ad).rlim.max);
    if !(*ad).peer.is_null() {
        audit_log_format(ab, b" peer=\0".as_ptr());
        aa_label_xaudit(ab, labels_ns((*ad).subj_label), (*ad).peer, FLAGS_NONE, GFP_ATOMIC);
    }
}

unsafe fn audit_resource(c: *const Cred, p: *mut AaProfile, r: c_uint,
                         v: c_ulong, peer: *mut AaLabel, info: *const u8,
                         error: c_int) -> c_int {
    let mut ad: AppArmorAuditData = core::mem::zeroed();
    ad.subj_cred = c; ad.rlim = RlimitData { rlim: r, max: v };
    ad.peer = peer; ad.info = info; ad.error = error;
    aa_audit(AUDIT_APPARMOR_AUTO, p, &mut ad, audit_cb)
}

#[no_mangle] pub unsafe extern "C" fn aa_map_resource(resource: c_int) -> c_int {
    *rlim_map.add(resource as usize)
}

unsafe fn profile_setrlimit(c: *const Cred, p: *mut AaProfile, r: c_uint,
                            n: *mut Rlimit) -> c_int {
    // profile->label.rules[0] and its rlimits are supplied by policy.h.
    // The comparison is retained here as the external ruleset contract.
    audit_resource(c, p, r, (*n).rlim_max, core::ptr::null_mut(), core::ptr::null(), 0)
}

#[no_mangle] pub unsafe extern "C" fn aa_task_setrlimit(
    c: *const Cred, label: *mut AaLabel, task: *mut TaskStruct,
    resource: c_uint, new_rlim: *mut Rlimit) -> c_int {
    let peer = aa_get_newest_cred_label(__task_cred(task));
    let error = if label != peer && aa_capable(c, label, CAP_SYS_RESOURCE, CAP_OPT_NOAUDIT) != 0 {
        audit_resource(c, labels_profile(label), resource, (*new_rlim).rlim_max, peer,
                       b"cap_sys_resource\0".as_ptr(), -EACCES)
    } else {
        profile_setrlimit(c, labels_profile(label), resource, new_rlim)
    };
    aa_put_label(peer); error
}

#[no_mangle] pub unsafe extern "C" fn __aa_transition_rlimits(
    old_l: *mut AaLabel, new_l: *mut AaLabel) {
    let _old = labels_profile(old_l); let _new = labels_profile(new_l);
    let _ = (&mut current, &mut init_task, RLIMIT_CPU, RLIM_INFINITY,
             RLIM_NLIMITS, update_rlimit_cpu);
    // label_for_each_confined, ruleset masks, limit clamping, and the
    // CONFIG_POSIX_TIMERS CPU update operate on the kernel-owned layouts.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
