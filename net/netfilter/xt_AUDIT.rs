// SPDX-License-Identifier: GPL-2.0-only
/*
 * Creates audit record for dropped/accepted packets
 *
 * (C) 2010-2011 Thomas Graf <tgraf@redhat.com>
 * (C) 2010-2011 Red Hat, Inc.
 */

// C dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut audit_enabled: c_int;
    fn audit_log_start(ctx: *mut c_void, gfp_mask: c_uint, typ: c_int) -> *mut audit_buffer;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_nf_skb(ab: *mut audit_buffer, skb: *mut sk_buff, family: c_uint);
    fn audit_log_end(ab: *mut audit_buffer);
    fn xt_family(par: *const xt_action_param) -> c_uint;
    fn xt_register_targets(targets: *mut xt_target, count: usize) -> c_int;
    fn xt_unregister_targets(targets: *mut xt_target, count: usize);
}

#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    pub mark: c_uint,
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_action_param {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_tgchk_param {
    pub targinfo: *const c_void,
}

#[repr(C)]
pub struct xt_audit_info {
    pub type_: c_uint,
}

type TargetFn = unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> c_uint;
type CheckEntryFn = unsafe extern "C" fn(*const xt_tgchk_param) -> c_int;

#[repr(C)]
pub struct xt_target {
    pub name: *const c_char,
    pub revision: u8,
    pub family: u16,
    pub target: Option<TargetFn>,
    pub targetsize: usize,
    pub checkentry: Option<CheckEntryFn>,
    pub me: *mut c_void,
}

const AUDIT_OFF: c_int = 0;
const AUDIT_NETFILTER_PKT: c_int = 1320;
const GFP_ATOMIC: c_uint = 0;
const XT_CONTINUE: c_uint = 0;
const EBT_CONTINUE: c_uint = 0;
const NFPROTO_UNSPEC: u16 = 0;
const NFPROTO_BRIDGE: u16 = 7;
const XT_AUDIT_TYPE_MAX: c_uint = 1;
const ERANGE: c_int = 34;

unsafe extern "C" fn audit_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let mut ab: *mut audit_buffer;

    if audit_enabled == AUDIT_OFF {
        return XT_CONTINUE;
    }
    ab = audit_log_start(core::ptr::null_mut(), GFP_ATOMIC, AUDIT_NETFILTER_PKT);
    if ab.is_null() {
        return XT_CONTINUE;
    }

    audit_log_format(ab, b"mark=%#x\0".as_ptr() as *const c_char, (*skb).mark);
    audit_log_nf_skb(ab, skb, xt_family(par));
    audit_log_end(ab);

    XT_CONTINUE
}

unsafe extern "C" fn audit_tg_ebt(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    audit_tg(skb, par);
    EBT_CONTINUE
}

unsafe extern "C" fn audit_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *const xt_audit_info;

    if (*info).type_ > XT_AUDIT_TYPE_MAX {
        // pr_info_ratelimited("Audit type out of range (valid range: 0..%u)\n",
        //                     XT_AUDIT_TYPE_MAX);
        return -ERANGE;
    }

    0
}

static mut audit_tg_reg: [xt_target; 2] = [
    xt_target {
        name: b"AUDIT\0".as_ptr() as *const c_char,
        revision: 0,
        family: NFPROTO_UNSPEC,
        target: Some(audit_tg),
        targetsize: core::mem::size_of::<xt_audit_info>(),
        checkentry: Some(audit_tg_check),
        me: core::ptr::null_mut(),
    },
    xt_target {
        name: b"AUDIT\0".as_ptr() as *const c_char,
        revision: 0,
        family: NFPROTO_BRIDGE,
        target: Some(audit_tg_ebt),
        targetsize: core::mem::size_of::<xt_audit_info>(),
        checkentry: Some(audit_tg_check),
        me: core::ptr::null_mut(),
    },
];

unsafe extern "C" fn audit_tg_init() -> c_int {
    xt_register_targets(audit_tg_reg.as_mut_ptr(), audit_tg_reg.len())
}

unsafe extern "C" fn audit_tg_exit() {
    xt_unregister_targets(audit_tg_reg.as_mut_ptr(), audit_tg_reg.len());
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
