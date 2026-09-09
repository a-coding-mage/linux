// SPDX-License-Identifier: GPL-2.0-only
/* This is a module which is used to mark packets for tracing.
 */

// External kernel declarations supplied by the surrounding translation unit.
use core::ffi::c_char;

#[repr(C)]
pub struct xt_tgchk_param {
    pub family: u16,
}

#[repr(C)]
pub struct xt_tgdtor_param {
    pub family: u16,
}

#[repr(C)]
pub struct sk_buff {
    pub nf_trace: u8,
}

#[repr(C)]
pub struct xt_action_param {
    _private: [u8; 0],
}

type XtCheckEntry = unsafe extern "C" fn(*const xt_tgchk_param) -> i32;
type XtDestroy = unsafe extern "C" fn(*const xt_tgdtor_param);
type XtTargetFn = unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> u32;

#[repr(C)]
pub struct xt_target {
    pub name: *const c_char,
    pub revision: u8,
    pub family: u16,
    pub table: *const c_char,
    pub target: Option<XtTargetFn>,
    pub checkentry: Option<XtCheckEntry>,
    pub destroy: Option<XtDestroy>,
    pub me: *mut core::ffi::c_void,
}

unsafe extern "C" {
    fn nf_logger_find_get(family: u16, typ: u8) -> i32;
    fn nf_logger_put(family: u16, typ: u8);
    fn xt_register_targets(targets: *mut xt_target, count: usize) -> i32;
    fn xt_unregister_targets(targets: *mut xt_target, count: usize);
}

const NF_LOG_TYPE_LOG: u8 = 0;
const NFPROTO_IPV4: u16 = 2;
const NFPROTO_IPV6: u16 = 10;
const XT_CONTINUE: u32 = 0xFFFF_FFFF;

static mut TRACE_TG_REG: [xt_target; 2] = [
    xt_target {
        name: b"TRACE\0".as_ptr() as *const c_char,
        revision: 0,
        family: NFPROTO_IPV4,
        table: b"raw\0".as_ptr() as *const c_char,
        target: Some(trace_tg),
        checkentry: Some(trace_tg_check),
        destroy: Some(trace_tg_destroy),
        me: core::ptr::null_mut(),
    },
    // CONFIG_IP6_NF_IPTABLES / IS_ENABLED conditional from the C source.
    xt_target {
        name: b"TRACE\0".as_ptr() as *const c_char,
        revision: 0,
        family: NFPROTO_IPV6,
        table: b"raw\0".as_ptr() as *const c_char,
        target: Some(trace_tg),
        checkentry: Some(trace_tg_check),
        destroy: Some(trace_tg_destroy),
        me: core::ptr::null_mut(),
    },
];

unsafe extern "C" fn trace_tg_check(par: *const xt_tgchk_param) -> i32 {
    nf_logger_find_get((*par).family, NF_LOG_TYPE_LOG)
}

unsafe extern "C" fn trace_tg_destroy(par: *const xt_tgdtor_param) {
    nf_logger_put((*par).family, NF_LOG_TYPE_LOG);
}

unsafe extern "C" fn trace_tg(skb: *mut sk_buff, _par: *const xt_action_param) -> u32 {
    (*skb).nf_trace = 1;
    XT_CONTINUE
}

unsafe extern "C" fn trace_tg_init() -> i32 {
    xt_register_targets(TRACE_TG_REG.as_mut_ptr(), TRACE_TG_REG.len())
}

unsafe extern "C" fn trace_tg_exit() {
    xt_unregister_targets(TRACE_TG_REG.as_mut_ptr(), TRACE_TG_REG.len());
}

// MODULE_DESCRIPTION("Xtables: packet flow tracing");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ipt_TRACE");
// MODULE_ALIAS("ip6t_TRACE");
// module_init(trace_tg_init);
// module_exit(trace_tg_exit);
// MODULE_SOFTDEP("pre: nf_log_syslog");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
