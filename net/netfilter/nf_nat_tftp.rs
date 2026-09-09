// SPDX-License-Identifier: GPL-2.0-only
/* (C) 2001-2002 Magnus Boden <mb@ozaba.mine.nu>
 */

// Translated from C. Kernel and netfilter declarations are supplied by other
// translation units or bindings.

const NAT_HELPER_NAME: &str = "tftp";

// MODULE_AUTHOR("Magnus Boden <mb@ozaba.mine.nu>");
// MODULE_DESCRIPTION("TFTP NAT helper");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_NF_NAT_HELPER(NAT_HELPER_NAME);

static mut nat_helper_tftp: nf_conntrack_nat_helper =
    NF_CT_NAT_HELPER_INIT!(NAT_HELPER_NAME);

unsafe fn help(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    exp: *mut nf_conntrack_expect,
) -> u32 {
    (*exp).saved_proto.udp.port =
        (*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple.src.u.udp.port;
    (*exp).dir = IP_CT_DIR_REPLY;
    (*exp).expectfn = Some(nf_nat_follow_master);
    if nf_ct_expect_related(exp, 0) != 0 {
        nf_ct_helper_log(skb, ct, "cannot add expectation");
        return NF_DROP;
    }
    NF_ACCEPT
}

unsafe fn nf_nat_tftp_fini() {
    nf_nat_helper_unregister(&raw mut nat_helper_tftp);
    RCU_INIT_POINTER!(nf_nat_tftp_hook, core::ptr::null_mut());
    synchronize_rcu();
}

unsafe fn nf_nat_tftp_init() -> i32 {
    BUG_ON!(nf_nat_tftp_hook != core::ptr::null_mut());
    nf_nat_helper_register(&raw mut nat_helper_tftp);
    RCU_INIT_POINTER!(nf_nat_tftp_hook, Some(help));
    0
}

// module_init(nf_nat_tftp_init);
// module_exit(nf_nat_tftp_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
