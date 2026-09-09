// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// Translated from the Linux kernel C implementation. Required kernel types,
// constants, macros, and functions are supplied by external dependencies.

use core::ptr;

static mut DEFRAG4_MUTEX: /* DEFINE_MUTEX(defrag4_mutex) */ () = ();

unsafe fn nf_ct_ipv4_gather_frags(net: *mut net, skb: *mut sk_buff, user: u32) -> i32 {
    let err: i32;

    local_bh_disable();
    err = ip_defrag(net, skb, user);
    local_bh_enable();

    if err == 0 {
        (*skb).ignore_df = 1;
    }

    err
}

unsafe fn nf_ct_defrag_user(hooknum: u32, skb: *mut sk_buff) -> ip_defrag_users {
    let mut zone_id: u16 = NF_CT_DEFAULT_ZONE_ID;
    // #if IS_ENABLED(CONFIG_NF_CONNTRACK)
    if skb_nfct(skb) != ptr::null_mut() {
        let mut ctinfo: enum_ip_conntrack_info = 0;
        let ct: *const nf_conn = nf_ct_get(skb, &mut ctinfo);

        zone_id = nf_ct_zone_id(nf_ct_zone(ct), CTINFO2DIR(ctinfo));
    }
    // #endif
    if nf_bridge_in_prerouting(skb) {
        return IP_DEFRAG_CONNTRACK_BRIDGE_IN + zone_id as ip_defrag_users;
    }

    if hooknum == NF_INET_PRE_ROUTING {
        IP_DEFRAG_CONNTRACK_IN + zone_id as ip_defrag_users
    } else {
        IP_DEFRAG_CONNTRACK_OUT + zone_id as ip_defrag_users
    }
}

unsafe fn ipv4_conntrack_defrag(
    _priv: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> u32 {
    let sk: *mut sock = (*skb).sk;

    if !sk.is_null()
        && sk_fullsock(sk)
        && (*sk).sk_family == PF_INET
        && inet_test_bit(NODEFRAG, sk)
    {
        return NF_ACCEPT;
    }

    // #if IS_ENABLED(CONFIG_NF_CONNTRACK)
    // #if !IS_ENABLED(CONFIG_NF_NAT)
    /* Previously seen (loopback)?  Ignore.  Do this before
       fragment check. */
    if skb_nfct(skb) != ptr::null_mut()
        && !nf_ct_is_template(skb_nfct(skb) as *mut nf_conn)
    {
        return NF_ACCEPT;
    }
    // #endif
    if (*skb)._nfct == IP_CT_UNTRACKED {
        return NF_ACCEPT;
    }
    // #endif
    /* Gather fragments. */
    if ip_is_fragment(ip_hdr(skb)) {
        let user: ip_defrag_users = nf_ct_defrag_user((*state).hook, skb);

        if nf_ct_ipv4_gather_frags((*state).net, skb, user) != 0 {
            return NF_STOLEN;
        }
    }
    NF_ACCEPT
}

static mut IPV4_DEFRAG_OPS: [nf_hook_ops; 2] = [
    nf_hook_ops {
        hook: Some(ipv4_conntrack_defrag),
        pf: NFPROTO_IPV4,
        hooknum: NF_INET_PRE_ROUTING,
        priority: NF_IP_PRI_CONNTRACK_DEFRAG,
    },
    nf_hook_ops {
        hook: Some(ipv4_conntrack_defrag),
        pf: NFPROTO_IPV4,
        hooknum: NF_INET_LOCAL_OUT,
        priority: NF_IP_PRI_CONNTRACK_DEFRAG,
    },
];

unsafe fn defrag4_net_exit(net: *mut net) {
    if (*net).nf.defrag_ipv4_users != 0 {
        nf_unregister_net_hooks(net, IPV4_DEFRAG_OPS.as_ptr(), IPV4_DEFRAG_OPS.len());
        (*net).nf.defrag_ipv4_users = 0;
    }
}

static mut DEFRAG_HOOK: nf_defrag_hook = nf_defrag_hook {
    owner: THIS_MODULE,
    enable: Some(nf_defrag_ipv4_enable),
    disable: Some(nf_defrag_ipv4_disable),
};

static mut DEFRAG4_NET_OPS: pernet_operations = pernet_operations {
    exit: Some(defrag4_net_exit),
};

unsafe fn nf_defrag_init() -> i32 {
    let err: i32;

    err = register_pernet_subsys(&mut DEFRAG4_NET_OPS);
    if err != 0 {
        return err;
    }

    rcu_assign_pointer(nf_defrag_v4_hook, &DEFRAG_HOOK);
    err
}

unsafe fn nf_defrag_fini() {
    rcu_assign_pointer(nf_defrag_v4_hook, ptr::null());
    unregister_pernet_subsys(&mut DEFRAG4_NET_OPS);
}

pub unsafe fn nf_defrag_ipv4_enable(net: *mut net) -> i32 {
    let mut err: i32 = 0;

    mutex_lock(&mut DEFRAG4_MUTEX);
    if (*net).nf.defrag_ipv4_users == u32::MAX {
        err = -EOVERFLOW;
        mutex_unlock(&mut DEFRAG4_MUTEX);
        return err;
    }

    if (*net).nf.defrag_ipv4_users != 0 {
        (*net).nf.defrag_ipv4_users += 1;
        mutex_unlock(&mut DEFRAG4_MUTEX);
        return err;
    }

    err = nf_register_net_hooks(net, IPV4_DEFRAG_OPS.as_ptr(), IPV4_DEFRAG_OPS.len());
    if err == 0 {
        (*net).nf.defrag_ipv4_users = 1;
    }

    mutex_unlock(&mut DEFRAG4_MUTEX);
    err
}

pub unsafe fn nf_defrag_ipv4_disable(net: *mut net) {
    mutex_lock(&mut DEFRAG4_MUTEX);
    if (*net).nf.defrag_ipv4_users != 0 {
        (*net).nf.defrag_ipv4_users -= 1;
        if (*net).nf.defrag_ipv4_users == 0 {
            nf_unregister_net_hooks(net, IPV4_DEFRAG_OPS.as_ptr(), IPV4_DEFRAG_OPS.len());
        }
    }

    mutex_unlock(&mut DEFRAG4_MUTEX);
}

// module_init(nf_defrag_init);
// module_exit(nf_defrag_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("IPv4 defragmentation support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
