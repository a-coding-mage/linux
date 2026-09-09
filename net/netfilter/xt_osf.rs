// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2003+ Evgeniy Polyakov <zbr@ioremap.net>
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Linux kernel and netfilter dependencies are supplied by the surrounding build.

unsafe extern "C" {
    fn nf_osf_match(
        skb: *const sk_buff,
        family: i32,
        hooknum: u32,
        in_: *const net_device,
        out: *const net_device,
        matchinfo: *const core::ffi::c_void,
        net: *const net,
        fingers: *const core::ffi::c_void,
    ) -> bool;

    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
}

unsafe extern "C" {
    static mut nf_osf_fingers: *const core::ffi::c_void;
}

unsafe fn xt_osf_match_packet(
    skb: *const sk_buff,
    p: *mut xt_action_param,
) -> bool {
    if (*p).fragoff != 0 {
        return false;
    }

    nf_osf_match(
        skb,
        xt_family(p),
        xt_hooknum(p),
        xt_in(p),
        xt_out(p),
        (*p).matchinfo,
        xt_net(p),
        nf_osf_fingers,
    )
}

static mut xt_osf_match: xt_match = xt_match {
    name: b"osf\0".as_ptr() as *const core::ffi::c_char,
    revision: 0,
    family: NFPROTO_IPV4,
    proto: IPPROTO_TCP,
    hooks: (1u32 << NF_INET_LOCAL_IN)
        | (1u32 << NF_INET_PRE_ROUTING)
        | (1u32 << NF_INET_FORWARD),
    match_: Some(xt_osf_match_packet),
    matchsize: core::mem::size_of::<xt_osf_info>(),
    me: THIS_MODULE,
};

unsafe extern "C" fn xt_osf_init() -> i32 {
    let err: i32;

    err = xt_register_match(&raw mut xt_osf_match);
    if err != 0 {
        // pr_err("Failed to register OS fingerprint matching module (%d)\n", err);
        return err;
    }

    0
}

unsafe extern "C" fn xt_osf_fini() {
    xt_unregister_match(&raw mut xt_osf_match);
}

// module_init(xt_osf_init);
// module_exit(xt_osf_fini);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Evgeniy Polyakov <zbr@ioremap.net>");
// MODULE_DESCRIPTION("Passive OS fingerprint matching.");
// MODULE_ALIAS("ipt_osf");
// MODULE_ALIAS("ip6t_osf");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
