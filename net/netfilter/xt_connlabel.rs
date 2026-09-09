// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2013 Astaro GmbH & Co KG
 */

// External Linux kernel and netfilter declarations are supplied by other files.

use core::ffi::{c_char, c_int, c_uint, c_void};

type Bool = bool;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conn_labels {
    pub bits: [c_ulong; 0],
}

type c_ulong = usize;
type ip_conntrack_info = c_int;

#[repr(C)]
pub struct xt_connlabel_mtinfo {
    pub bit: c_uint,
    pub options: c_uint,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *mut c_void,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *mut c_void,
    pub net: *mut net,
    pub family: c_uint,
}

#[repr(C)]
pub struct xt_mtdtor_param {
    pub net: *mut net,
    pub family: c_uint,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const c_char,
    pub family: c_uint,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> c_int>,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> Bool>,
    pub matchsize: usize,
    pub destroy: Option<unsafe extern "C" fn(*const xt_mtdtor_param)>,
    pub me: *mut c_void,
}

const XT_CONNLABEL_OP_INVERT: c_uint = 1 << 0;
const XT_CONNLABEL_OP_SET: c_uint = 1 << 1;
const NFPROTO_UNSPEC: c_uint = 0;
const IPCT_LABEL: c_uint = 1;

extern "C" {
    static THIS_MODULE: c_void;

    fn nf_ct_get(skb: *const sk_buff, ctinfo: *mut ip_conntrack_info) -> *mut nf_conn;
    fn nf_ct_labels_find(ct: *mut nf_conn) -> *mut nf_conn_labels;
    fn test_bit(bit: c_uint, addr: *const c_ulong) -> c_int;
    fn test_and_set_bit(bit: c_uint, addr: *mut c_ulong) -> c_int;
    fn nf_conntrack_event_cache(event: c_uint, ct: *mut nf_conn);
    fn nf_ct_netns_get(net: *mut net, family: c_uint) -> c_int;
    fn nf_ct_netns_put(net: *mut net, family: c_uint);
    fn nf_connlabels_get(net: *mut net, bit: c_uint) -> c_int;
    fn nf_connlabels_put(net: *mut net);
    fn xt_register_match(m: *mut xt_match) -> c_int;
    fn xt_unregister_match(m: *mut xt_match);
}

unsafe extern "C" fn connlabel_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> Bool {
    let info = (*(par)).matchinfo as *const xt_connlabel_mtinfo;
    let mut ctinfo: ip_conntrack_info = 0;
    let labels: *mut nf_conn_labels;
    let ct: *mut nf_conn;
    let invert = ((*info).options & XT_CONNLABEL_OP_INVERT) != 0;

    ct = nf_ct_get(skb, &mut ctinfo);
    if ct.is_null() {
        return invert;
    }

    labels = nf_ct_labels_find(ct);
    if labels.is_null() {
        return invert;
    }

    if test_bit((*info).bit, (*labels).bits.as_ptr()) != 0 {
        return !invert;
    }

    if ((*info).options & XT_CONNLABEL_OP_SET) != 0 {
        if test_and_set_bit((*info).bit, (*labels).bits.as_mut_ptr()) == 0 {
            nf_conntrack_event_cache(IPCT_LABEL, ct);
        }

        return !invert;
    }

    invert
}

unsafe extern "C" fn connlabel_mt_check(par: *const xt_mtchk_param) -> c_int {
    let options = XT_CONNLABEL_OP_INVERT | XT_CONNLABEL_OP_SET;
    let info = (*par).matchinfo as *mut xt_connlabel_mtinfo;
    let mut ret: c_int;

    if ((*info).options & !options) != 0 {
        // pr_info_ratelimited("Unknown options in mask %x\\n", info->options);
        return -22;
    }

    ret = nf_ct_netns_get((*par).net, (*par).family);
    if ret < 0 {
        // pr_info_ratelimited("cannot load conntrack support for proto=%u\\n", par->family);
        return ret;
    }

    ret = nf_connlabels_get((*par).net, (*info).bit);
    if ret < 0 {
        nf_ct_netns_put((*par).net, (*par).family);
    }
    ret
}

unsafe extern "C" fn connlabel_mt_destroy(par: *const xt_mtdtor_param) {
    nf_connlabels_put((*par).net);
    nf_ct_netns_put((*par).net, (*par).family);
}

static mut connlabels_mt_reg: xt_match = xt_match {
    name: b"connlabel\0".as_ptr() as *const c_char,
    family: NFPROTO_UNSPEC,
    checkentry: Some(connlabel_mt_check),
    match_: Some(connlabel_mt),
    matchsize: core::mem::size_of::<xt_connlabel_mtinfo>(),
    destroy: Some(connlabel_mt_destroy),
    me: core::ptr::addr_of!(THIS_MODULE) as *mut c_void,
};

unsafe extern "C" fn connlabel_mt_init() -> c_int {
    xt_register_match(core::ptr::addr_of_mut!(connlabels_mt_reg))
}

unsafe extern "C" fn connlabel_mt_exit() {
    xt_unregister_match(core::ptr::addr_of_mut!(connlabels_mt_reg));
}

// module_init(connlabel_mt_init);
// module_exit(connlabel_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
