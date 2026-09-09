// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) 2011 Pablo Neira Ayuso <pablo@netfilter.org>
 * (C) 2011 Intra2net AG <https://www.intra2net.com>
 */

// Translated from xt_nfacct.c. Kernel/module declarations and types are
// supplied by the corresponding Linux netfilter dependencies.

// MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
// MODULE_DESCRIPTION("Xtables: match for the extended accounting infrastructure");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ipt_nfacct");
// MODULE_ALIAS("ip6t_nfacct");

unsafe fn nfacct_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let overquota: c_int;
    let info: *const xt_nfacct_match_info = unsafe { (*par).targinfo as *const xt_nfacct_match_info };

    unsafe {
        nfnl_acct_update(skb, (*info).nfacct);
        overquota = nfnl_acct_overquota(xt_net(par), (*info).nfacct);
    }

    overquota != NFACCT_UNDERQUOTA
}

unsafe fn nfacct_mt_checkentry(par: *const xt_mtchk_param) -> c_int {
    let info: *mut xt_nfacct_match_info = unsafe { (*par).matchinfo as *mut xt_nfacct_match_info };
    let nfacct: *mut nf_acct;

    unsafe {
        nfacct = nfnl_acct_find_get((*par).net, (*info).name);
        if nfacct.is_null() {
            pr_info_ratelimited!(
                "accounting object `%.*s' does not exist\\n",
                NFACCT_NAME_MAX,
                (*info).name,
            );
            return -ENOENT;
        }
        (*info).nfacct = nfacct;
    }
    0
}

unsafe fn nfacct_mt_destroy(par: *const xt_mtdtor_param) {
    let info: *const xt_nfacct_match_info = unsafe { (*par).matchinfo as *const xt_nfacct_match_info };

    unsafe {
        nfnl_acct_put((*info).nfacct);
    }
}

static mut nfacct_mt_reg: [xt_match; 2] = [
    xt_match {
        name: "nfacct",
        revision: 0,
        family: NFPROTO_UNSPEC,
        checkentry: Some(nfacct_mt_checkentry),
        match_: Some(nfacct_mt),
        destroy: Some(nfacct_mt_destroy),
        matchsize: core::mem::size_of::<xt_nfacct_match_info>(),
        usersize: core::mem::offset_of!(xt_nfacct_match_info, nfacct),
        me: THIS_MODULE,
    },
    xt_match {
        name: "nfacct",
        revision: 1,
        family: NFPROTO_UNSPEC,
        checkentry: Some(nfacct_mt_checkentry),
        match_: Some(nfacct_mt),
        destroy: Some(nfacct_mt_destroy),
        matchsize: core::mem::size_of::<xt_nfacct_match_info_v1>(),
        usersize: core::mem::offset_of!(xt_nfacct_match_info_v1, nfacct),
        me: THIS_MODULE,
    },
];

unsafe fn nfacct_mt_init() -> c_int {
    unsafe { xt_register_matches(nfacct_mt_reg.as_mut_ptr(), nfacct_mt_reg.len()) }
}

unsafe fn nfacct_mt_exit() {
    unsafe {
        xt_unregister_matches(nfacct_mt_reg.as_mut_ptr(), nfacct_mt_reg.len());
    }
}

// module_init(nfacct_mt_init);
// module_exit(nfacct_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
