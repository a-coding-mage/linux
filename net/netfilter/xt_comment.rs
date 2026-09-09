// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implements a dummy match to allow attaching comments to rules
 *
 * 2003-05-13 Brad Fisher (brad@info-link.net)
 */

// Dependencies supplied by the Linux kernel and netfilter headers:
// linux/module.h, linux/skbuff.h, linux/netfilter/x_tables.h,
// linux/netfilter/xt_comment.h

// MODULE_AUTHOR("Brad Fisher <brad@info-link.net>");
// MODULE_DESCRIPTION("Xtables: No-op match which can be tagged with a comment");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("ipt_comment");
// MODULE_ALIAS("ip6t_comment");

unsafe fn comment_mt(
    _skb: *const crate::sk_buff,
    _par: *mut crate::xt_action_param,
) -> bool {
    /* We always match */
    true
}

static mut comment_mt_reg: crate::xt_match = crate::xt_match {
    name: *b"comment\0",
    revision: 0,
    family: crate::NFPROTO_UNSPEC,
    match_: Some(comment_mt),
    matchsize: core::mem::size_of::<crate::xt_comment_info>(),
    me: crate::THIS_MODULE,
};

unsafe fn comment_mt_init() -> core::ffi::c_int {
    crate::xt_register_match(&raw mut comment_mt_reg)
}

unsafe fn comment_mt_exit() {
    crate::xt_unregister_match(&raw mut comment_mt_reg);
}

// module_init(comment_mt_init);
// module_exit(comment_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
