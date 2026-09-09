// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) 2010 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
//
// C dependencies supplied by the surrounding kernel translation:
// #include <linux/netfilter.h>
// #include <linux/slab.h>
// #include <linux/kernel.h>
// #include <linux/moduleparam.h>
// #include <net/netfilter/nf_conntrack.h>
// #include <net/netfilter/nf_conntrack_extend.h>
// #include <net/netfilter/nf_conntrack_timestamp.h>

// `__read_mostly` is a linker-placement annotation in the C source.
#[no_mangle]
pub static mut nf_ct_tstamp: bool = false;

// module_param_named(tstamp, nf_ct_tstamp, bool, 0644);
// MODULE_PARM_DESC(tstamp, "Enable connection tracking flow timestamping.");

// Opaque declarations corresponding to the externally supplied C structures.
#[repr(C)]
pub struct net__ct {
    pub sysctl_tstamp: bool,
}

#[repr(C)]
pub struct net {
    pub ct: net__ct,
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_tstamp_pernet_init(net: *mut net) {
    (*net).ct.sysctl_tstamp = nf_ct_tstamp;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
