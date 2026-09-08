// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Simplified MAC Kernel (smack) security module
 *
 *  This file contains the Smack netfilter implementation
 *
 *  Author:
 *	Casey Schaufler <casey@schaufler-ca.com>
 *
 *  Copyright (C) 2014 Casey Schaufler <casey@schaufler-ca.com>
 *  Copyright (C) 2014 Intel Corporation.
 */

/* Dependencies from:
 * <linux/netfilter_ipv4.h>
 * <linux/netfilter_ipv6.h>
 * <linux/netdevice.h>
 * <net/inet_sock.h>
 * <net/net_namespace.h>
 * "smack.h"
 */

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct sk_buff {
    pub secmark: c_uint,
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_hook_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct socket_smack {
    pub smk_out: *mut smack_known,
}

#[repr(C)]
pub struct smack_known {
    pub smk_secid: c_uint,
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

pub type NfHookFn = unsafe extern "C" fn(
    priv_: *mut c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> c_uint;

#[repr(C)]
pub struct nf_hook_ops {
    pub hook: Option<NfHookFn>,
    pub pf: c_uint,
    pub hooknum: c_uint,
    pub priority: c_int,
}

#[repr(C)]
pub struct pernet_operations {
    pub init: Option<unsafe extern "C" fn(net: *mut net) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(net: *mut net)>,
}

unsafe extern "C" {
    static mut smack_enabled: c_int;

    fn skb_to_full_sk(skb: *mut sk_buff) -> *mut sock;
    fn smack_sock(sk: *mut sock) -> *mut socket_smack;
    fn nf_register_net_hooks(
        net: *mut net,
        reg: *const nf_hook_ops,
        n: c_uint,
    ) -> c_int;
    fn nf_unregister_net_hooks(net: *mut net, reg: *const nf_hook_ops, n: c_uint);
    fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
    fn printk(fmt: *const u8, ...) -> c_int;
}

/* External kernel constants/macros supplied by included headers. */
const NF_ACCEPT: c_uint = 1;
const NFPROTO_IPV4: c_uint = 2;
const NF_INET_LOCAL_OUT: c_uint = 3;
const NF_IP_PRI_SELINUX_FIRST: c_int = 225;
const KERN_DEBUG: &[u8] = b"\x017";

/* CONFIG_IPV6 / IS_ENABLED(CONFIG_IPV6) is a build-time kernel condition. */
#[cfg(CONFIG_IPV6)]
const NFPROTO_IPV6: c_uint = 10;
#[cfg(CONFIG_IPV6)]
const NF_IP6_PRI_SELINUX_FIRST: c_int = 225;

unsafe extern "C" fn smack_ip_output(
    priv_: *mut c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> c_uint {
    let sk: *mut sock = unsafe { skb_to_full_sk(skb) };
    let mut ssp: *mut socket_smack;
    let mut skp: *mut smack_known;

    let _ = priv_;
    let _ = state;

    if !sk.is_null() {
        ssp = unsafe { smack_sock(sk) };
        skp = unsafe { (*ssp).smk_out };
        unsafe {
            (*skb).secmark = (*skp).smk_secid;
        }
    }

    NF_ACCEPT
}

static SMACK_NF_OPS: &[nf_hook_ops] = &[
    nf_hook_ops {
        hook: Some(smack_ip_output),
        pf: NFPROTO_IPV4,
        hooknum: NF_INET_LOCAL_OUT,
        priority: NF_IP_PRI_SELINUX_FIRST,
    },
    #[cfg(CONFIG_IPV6)]
    nf_hook_ops {
        hook: Some(smack_ip_output),
        pf: NFPROTO_IPV6,
        hooknum: NF_INET_LOCAL_OUT,
        priority: NF_IP6_PRI_SELINUX_FIRST,
    },
];

unsafe extern "C" fn smack_nf_register(net: *mut net) -> c_int {
    unsafe {
        nf_register_net_hooks(
            net,
            SMACK_NF_OPS.as_ptr(),
            SMACK_NF_OPS.len() as c_uint,
        )
    }
}

unsafe extern "C" fn smack_nf_unregister(net: *mut net) {
    unsafe {
        nf_unregister_net_hooks(
            net,
            SMACK_NF_OPS.as_ptr(),
            SMACK_NF_OPS.len() as c_uint,
        );
    }
}

static mut SMACK_NET_OPS: pernet_operations = pernet_operations {
    init: Some(smack_nf_register),
    exit: Some(smack_nf_unregister),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn smack_nf_ip_init() -> c_int {
    if unsafe { smack_enabled } == 0 {
        return 0;
    }

    unsafe {
        printk(
            concat!("\x017", "Smack: Registering netfilter hooks\n\0").as_ptr(),
        );
        register_pernet_subsys(&raw mut SMACK_NET_OPS)
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
