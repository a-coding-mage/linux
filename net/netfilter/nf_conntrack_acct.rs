// SPDX-License-Identifier: GPL-2.0-only
/* Accounting handling for netfilter. */

/*
 * (C) 2008 Krzysztof Piotr Oledzki <ole@ans.pl>
 */

// C dependency: pr_fmt(fmt) KBUILD_MODNAME ": " fmt

// C dependencies supplied by the surrounding kernel translation unit:
// linux/netfilter.h, linux/slab.h, linux/kernel.h, linux/moduleparam.h,
// linux/export.h, net/netfilter/nf_conntrack.h,
// net/netfilter/nf_conntrack_extend.h, and net/netfilter/nf_conntrack_acct.h

static mut nf_ct_acct: bool = false;

// C declaration: module_param_named(acct, nf_ct_acct, bool, 0644);
// C declaration: MODULE_PARM_DESC(acct, "Enable connection tracking flow accounting.");

pub unsafe fn nf_conntrack_acct_pernet_init(net: *mut net) {
    (*net).ct.sysctl_acct = nf_ct_acct;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
