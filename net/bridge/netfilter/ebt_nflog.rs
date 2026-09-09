// SPDX-License-Identifier: GPL-2.0-only
/*
 * ebt_nflog
 *
 *	Author:
 *	Peter Warasin <peter@endian.com>
 *
 *  February, 2008
 *
 * Based on:
 *  xt_NFLOG.c, (C) 2006 by Patrick McHardy <kaber@trash.net>
 *  ebt_ulog.c, (C) 2004 by Bart De Schuymer <bdschuym@pandora.be>
 *
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn xt_net(par: *const xt_action_param) -> *mut net;
    fn xt_hooknum(par: *const xt_action_param) -> c_uint;
    fn xt_in(par: *const xt_action_param) -> *mut net_device;
    fn xt_out(par: *const xt_action_param) -> *mut net_device;
    fn nf_log_packet(
        net: *mut net,
        pf: c_int,
        hooknum: c_uint,
        skb: *mut sk_buff,
        in_dev: *mut net_device,
        out_dev: *mut net_device,
        li: *const nf_loginfo,
        fmt: *const c_char,
        ...,
    );
    fn nf_logger_find_get(family: c_uint, typ: c_uint) -> c_int;
    fn nf_logger_put(family: c_uint, typ: c_uint);
    fn request_module(fmt: *const c_char, ... ) -> c_int;
    fn xt_register_target(target: *mut xt_target) -> c_int;
    fn xt_unregister_target(target: *mut xt_target);
}

// Types and constants supplied by included kernel headers.
#[allow(non_camel_case_types)]
type sk_buff = c_void;
#[allow(non_camel_case_types)]
type net = c_void;
#[allow(non_camel_case_types)]
type net_device = c_void;

#[repr(C)]
pub struct xt_action_param {
    pub targinfo: *const c_void,
}

#[repr(C)]
pub struct xt_tgchk_param {
    pub targinfo: *mut c_void,
    pub family: c_uint,
    pub nft_compat: bool,
}

#[repr(C)]
pub struct xt_tgdtor_param {
    pub family: c_uint,
}

#[repr(C)]
pub union nf_loginfo_ulog {
    pub copy_len: u32,
    pub group: u16,
    pub qthreshold: u16,
    pub flags: u8,
}

#[repr(C)]
pub struct nf_loginfo {
    pub typ: u8,
    pub ulog: nf_loginfo_ulog_fields,
}

#[repr(C)]
pub struct nf_loginfo_ulog_fields {
    pub copy_len: u32,
    pub group: u16,
    pub qthreshold: u16,
    pub flags: u8,
}

#[repr(C)]
pub struct xt_target {
    pub name: *const c_char,
    pub revision: u8,
    pub family: u16,
    pub target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> c_uint>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_tgdtor_param)>,
    pub targetsize: usize,
    pub me: *mut c_void,
}

extern "C" {
    static mut THIS_MODULE: c_void;
}

const NF_LOG_TYPE_ULOG: u8 = 1;
const PF_BRIDGE: c_int = 7;
const NFPROTO_BRIDGE: u16 = 7;
const EBT_CONTINUE: c_uint = 0xFFFFFFFF;
const EBT_NFLOG_MASK: u8 = 0x03;
const EBT_NFLOG_PREFIX_SIZE: usize = 64;

#[repr(C)]
pub struct ebt_nflog_info {
    pub len: u32,
    pub group: u16,
    pub threshold: u16,
    pub flags: u8,
    pub prefix: [u8; EBT_NFLOG_PREFIX_SIZE],
}

unsafe extern "C" fn ebt_nflog_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info = (*par).targinfo as *const ebt_nflog_info;
    let net = xt_net(par);
    let mut li = nf_loginfo {
        typ: NF_LOG_TYPE_ULOG,
        ulog: nf_loginfo_ulog_fields {
            copy_len: (*info).len,
            group: (*info).group,
            qthreshold: (*info).threshold,
            flags: 0,
        },
    };

    nf_log_packet(
        net,
        PF_BRIDGE,
        xt_hooknum(par),
        skb,
        xt_in(par),
        xt_out(par),
        &li,
        b"%s\0".as_ptr() as *const c_char,
        (*info).prefix.as_ptr(),
    );
    EBT_CONTINUE
}

unsafe extern "C" fn ebt_nflog_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *mut ebt_nflog_info;
    let mut ret: c_int;

    if (*info).flags & !EBT_NFLOG_MASK != 0 {
        return -22;
    }
    (*info).prefix[EBT_NFLOG_PREFIX_SIZE - 1] = 0;

    ret = nf_logger_find_get((*par).family, NF_LOG_TYPE_ULOG as c_uint);
    if ret != 0 && !(*par).nft_compat {
        request_module(b"%s\0".as_ptr() as *const c_char, b"nfnetlink_log\0".as_ptr());
        ret = nf_logger_find_get((*par).family, NF_LOG_TYPE_ULOG as c_uint);
    }

    ret
}

unsafe extern "C" fn ebt_nflog_tg_destroy(par: *const xt_tgdtor_param) {
    nf_logger_put((*par).family, NF_LOG_TYPE_ULOG as c_uint);
}

static mut ebt_nflog_tg_reg: xt_target = xt_target {
    name: b"nflog\0".as_ptr() as *const c_char,
    revision: 0,
    family: NFPROTO_BRIDGE,
    target: Some(ebt_nflog_tg),
    checkentry: Some(ebt_nflog_tg_check),
    destroy: Some(ebt_nflog_tg_destroy),
    targetsize: core::mem::size_of::<ebt_nflog_info>(),
    me: core::ptr::addr_of_mut!(THIS_MODULE),
};

unsafe extern "C" fn ebt_nflog_init() -> c_int {
    xt_register_target(core::ptr::addr_of_mut!(ebt_nflog_tg_reg))
}

unsafe extern "C" fn ebt_nflog_fini() {
    xt_unregister_target(core::ptr::addr_of_mut!(ebt_nflog_tg_reg));
}

// module_init(ebt_nflog_init);
// module_exit(ebt_nflog_fini);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Peter Warasin <peter@endian.com>");
// MODULE_DESCRIPTION("ebtables NFLOG netfilter logging module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
