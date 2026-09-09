// SPDX-License-Identifier: GPL-2.0-only
/*
 * This module is used to copy security markings from packets
 * to connections, and restore security markings from connections
 * back to packets.  This would normally be performed in conjunction
 * with the SECMARK target and state match.
 *
 * Based somewhat on CONNMARK:
 *   Copyright (C) 2002,2004 MARA Systems AB <https://www.marasystems.com>
 *    by Henrik Nordstrom <hno@marasystems.com>
 *
 * (C) 2006,2008 Red Hat, Inc., James Morris <jmorris@redhat.com>
 */

// C headers supplied by the surrounding kernel translation are intentionally
// omitted; their symbols remain external dependencies of this translation.

use core::ffi::{c_char, c_int, c_ushort, c_uint, c_void};

extern "C" {
    fn nf_ct_get(skb: *const sk_buff, ctinfo: *mut ip_conntrack_info) -> *mut nf_conn;
    fn nf_conntrack_event_cache(event: c_int, ct: *mut nf_conn);
    fn nf_ct_netns_get(net: *mut net, family: u8) -> c_int;
    fn nf_ct_netns_put(net: *mut net, family: u8);
    fn xt_register_targets(targets: *mut xt_target, count: usize) -> c_int;
    fn xt_unregister_targets(targets: *mut xt_target, count: usize);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn pr_info_ratelimited(fmt: *const c_char, ...);
    fn bug() -> !;
}

#[repr(C)]
pub struct sk_buff {
    pub secmark: u32,
}

#[repr(C)]
pub struct nf_conn {
    pub secmark: u32,
}

#[repr(C)]
pub struct xt_action_param {
    pub targinfo: *const c_void,
}

#[repr(C)]
pub struct xt_tgchk_param {
    pub targinfo: *const c_void,
    pub table: *const c_char,
    pub net: *mut net,
    pub family: u8,
}

#[repr(C)]
pub struct xt_tgdtor_param {
    pub net: *mut net,
    pub family: u8,
}

#[repr(C)]
pub struct xt_connsecmark_target_info {
    pub mode: c_ushort,
}

#[repr(C)]
pub struct xt_target {
    pub name: *const c_char,
    pub revision: u8,
    pub family: u8,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_tgchk_param) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*const xt_tgdtor_param)>,
    pub target: Option<unsafe extern "C" fn(*mut sk_buff, *const xt_action_param) -> c_uint>,
    pub targetsize: usize,
    pub me: *mut c_void,
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_conntrack_info {
    _private: [u8; 0],
}

const CONNSECMARK_SAVE: c_ushort = 1;
const CONNSECMARK_RESTORE: c_ushort = 2;
const IPCT_SECMARK: c_int = 0;
const XT_CONTINUE: c_uint = 0;
const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;

/*
 * If the packet has a security mark and the connection does not, copy
 * the security mark from the packet to the connection.
 */
unsafe extern "C" fn secmark_save(skb: *const sk_buff) {
    if (*skb).secmark != 0 {
        let mut ctinfo = core::mem::MaybeUninit::<ip_conntrack_info>::uninit();
        let ct = nf_ct_get(skb, ctinfo.as_mut_ptr());
        if !ct.is_null() && (*ct).secmark == 0 {
            (*ct).secmark = (*skb).secmark;
            nf_conntrack_event_cache(IPCT_SECMARK, ct);
        }
    }
}

/*
 * If packet has no security mark, and the connection does, restore the
 * security mark from the connection to the packet.
 */
unsafe extern "C" fn secmark_restore(skb: *mut sk_buff) {
    if (*skb).secmark == 0 {
        let mut ctinfo = core::mem::MaybeUninit::<ip_conntrack_info>::uninit();
        let ct = nf_ct_get(skb, ctinfo.as_mut_ptr());
        if !ct.is_null() && (*ct).secmark != 0 {
            (*skb).secmark = (*ct).secmark;
        }
    }
}

unsafe extern "C" fn connsecmark_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info = (*par).targinfo as *const xt_connsecmark_target_info;

    match (*info).mode {
        CONNSECMARK_SAVE => secmark_save(skb),
        CONNSECMARK_RESTORE => secmark_restore(skb),
        _ => bug(),
    }

    XT_CONTINUE
}

unsafe extern "C" fn connsecmark_tg_check(par: *const xt_tgchk_param) -> c_int {
    let info = (*par).targinfo as *const xt_connsecmark_target_info;
    let ret: c_int;

    // Only valid in the `mangle` or `security` table.
    if strcmp((*par).table, b"mangle\0".as_ptr() as *const c_char) != 0
        && strcmp((*par).table, b"security\0".as_ptr() as *const c_char) != 0
    {
        return -22; // -EINVAL
    }

    match (*info).mode {
        CONNSECMARK_SAVE | CONNSECMARK_RESTORE => {}
        _ => return -22, // -EINVAL
    }

    ret = nf_ct_netns_get((*par).net, (*par).family);
    ret
}

unsafe extern "C" fn connsecmark_tg_destroy(par: *const xt_tgdtor_param) {
    nf_ct_netns_put((*par).net, (*par).family);
}

static mut connsecmark_tg_reg: [xt_target; 2] = [xt_target {
    name: b"CONNSECMARK\0".as_ptr() as *const c_char,
    revision: 0,
    family: NFPROTO_IPV4,
    checkentry: Some(connsecmark_tg_check),
    destroy: Some(connsecmark_tg_destroy),
    target: Some(connsecmark_tg),
    targetsize: core::mem::size_of::<xt_connsecmark_target_info>(),
    me: core::ptr::null_mut(),
},
// #if IS_ENABLED(CONFIG_IP6_NF_IPTABLES)
xt_target {
    name: b"CONNSECMARK\0".as_ptr() as *const c_char,
    revision: 0,
    family: NFPROTO_IPV6,
    checkentry: Some(connsecmark_tg_check),
    destroy: Some(connsecmark_tg_destroy),
    target: Some(connsecmark_tg),
    targetsize: core::mem::size_of::<xt_connsecmark_target_info>(),
    me: core::ptr::null_mut(),
},
// #endif
];

unsafe extern "C" fn connsecmark_tg_init() -> c_int {
    xt_register_targets(connsecmark_tg_reg.as_mut_ptr(), connsecmark_tg_reg.len())
}

unsafe extern "C" fn connsecmark_tg_exit() {
    xt_unregister_targets(connsecmark_tg_reg.as_mut_ptr(), connsecmark_tg_reg.len());
}

// module_init(connsecmark_tg_init);
// module_exit(connsecmark_tg_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
