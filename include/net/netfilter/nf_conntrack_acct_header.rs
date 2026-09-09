/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * (C) 2008 Krzysztof Piotr Oledzki <ole@ans.pl>
 */

/* C header dependencies:
 * net/net_namespace.h
 * linux/netfilter/nf_conntrack_common.h
 * linux/netfilter/nf_conntrack_tuple_common.h
 * net/netfilter/nf_conntrack.h
 * net/netfilter/nf_conntrack_extend.h
 */

#[repr(C)]
pub struct nf_conn_counter {
    pub packets: atomic64_t,
    pub bytes: atomic64_t,
}

#[repr(C)]
pub struct nf_conn_acct {
    pub counter: [nf_conn_counter; IP_CT_DIR_MAX as usize],
}

pub unsafe fn nf_conn_acct_find(ct: *const nf_conn) -> *mut nf_conn_acct {
    nf_ct_ext_find(ct, NF_CT_EXT_ACCT)
}

pub unsafe fn nf_ct_acct_ext_add(ct: *mut nf_conn, gfp: gfp_t) -> *mut nf_conn_acct {
    /* #if IS_ENABLED(CONFIG_NF_CONNTRACK): configuration-dependent code. */
    let net: *mut net = nf_ct_net(ct);
    let mut acct: *mut nf_conn_acct;

    if !(*net).ct.sysctl_acct {
        return core::ptr::null_mut();
    }

    acct = nf_ct_ext_add(ct, NF_CT_EXT_ACCT, gfp);
    if acct.is_null() {
        pr_debug!("failed to add accounting extension area");
    }

    acct
    /* #else: return NULL; */
}

/* Check if connection tracking accounting is enabled */
pub unsafe fn nf_ct_acct_enabled(net: *mut net) -> bool {
    /* #if IS_ENABLED(CONFIG_NF_CONNTRACK) */
    (*net).ct.sysctl_acct != 0
    /* #else: false */
}

/* Enable/disable connection tracking accounting */
pub unsafe fn nf_ct_set_acct(net: *mut net, enable: bool) {
    /* #if IS_ENABLED(CONFIG_NF_CONNTRACK) */
    (*net).ct.sysctl_acct = enable;
    /* #endif */
}

extern "C" {
    pub fn nf_ct_acct_add(
        ct: *mut nf_conn,
        dir: u32,
        packets: core::ffi::c_uint,
        bytes: core::ffi::c_uint,
    );
}

pub unsafe fn nf_ct_acct_update(ct: *mut nf_conn, dir: u32, bytes: core::ffi::c_uint) {
    /* #if IS_ENABLED(CONFIG_NF_CONNTRACK) */
    nf_ct_acct_add(ct, dir, 1, bytes);
    /* #endif */
}

extern "C" {
    pub fn nf_conntrack_acct_pernet_init(net: *mut net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
