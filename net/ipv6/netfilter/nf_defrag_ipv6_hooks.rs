// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// Dependencies supplied by the surrounding kernel translation.
// CONFIG_NF_CONNTRACK conditionals from the original source are preserved here.

static mut defrag6_mutex: mutex = DEFINE_MUTEX!();

unsafe fn nf_ct6_defrag_user(hooknum: c_uint, skb: *mut sk_buff) -> ip6_defrag_users {
    let mut zone_id: u16 = NF_CT_DEFAULT_ZONE_ID;
    // #if IS_ENABLED(CONFIG_NF_CONNTRACK)
    if skb_nfct(skb) != 0 {
        let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
        let ct: *const nf_conn = nf_ct_get(skb, &mut ctinfo);
        zone_id = nf_ct_zone_id(nf_ct_zone(ct), CTINFO2DIR(ctinfo));
    }
    // #endif
    if nf_bridge_in_prerouting(skb) {
        return IP6_DEFRAG_CONNTRACK_BRIDGE_IN + zone_id as ip6_defrag_users;
    }

    if hooknum == NF_INET_PRE_ROUTING {
        IP6_DEFRAG_CONNTRACK_IN + zone_id as ip6_defrag_users
    } else {
        IP6_DEFRAG_CONNTRACK_OUT + zone_id as ip6_defrag_users
    }
}

unsafe fn ipv6_defrag(
    _priv: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> c_uint {
    let err: c_int;

    // #if IS_ENABLED(CONFIG_NF_CONNTRACK)
    // Previously seen (loopback)?
    if skb_nfct(skb) != 0 && !nf_ct_is_template(skb_nfct(skb) as *mut nf_conn) {
        return NF_ACCEPT;
    }

    if (*skb)._nfct == IP_CT_UNTRACKED {
        return NF_ACCEPT;
    }
    // #endif

    err = nf_ct_frag6_gather(
        (*state).net,
        skb,
        nf_ct6_defrag_user((*state).hook, skb),
    );
    // queued
    if err == -EINPROGRESS {
        return NF_STOLEN;
    }

    if err == 0 { NF_ACCEPT } else { NF_DROP }
}

static ipv6_defrag_ops: [nf_hook_ops; 2] = [
    nf_hook_ops {
        hook: Some(ipv6_defrag),
        pf: NFPROTO_IPV6,
        hooknum: NF_INET_PRE_ROUTING,
        priority: NF_IP6_PRI_CONNTRACK_DEFRAG,
    },
    nf_hook_ops {
        hook: Some(ipv6_defrag),
        pf: NFPROTO_IPV6,
        hooknum: NF_INET_LOCAL_OUT,
        priority: NF_IP6_PRI_CONNTRACK_DEFRAG,
    },
];

unsafe fn defrag6_net_exit(net: *mut net) {
    if (*net).nf.defrag_ipv6_users != 0 {
        nf_unregister_net_hooks(
            net,
            ipv6_defrag_ops.as_ptr(),
            ipv6_defrag_ops.len(),
        );
        (*net).nf.defrag_ipv6_users = 0;
    }
}

static defrag_hook: nf_defrag_hook = nf_defrag_hook {
    owner: THIS_MODULE,
    enable: Some(nf_defrag_ipv6_enable),
    disable: Some(nf_defrag_ipv6_disable),
};

static mut defrag6_net_ops: pernet_operations = pernet_operations {
    exit: Some(defrag6_net_exit),
};

unsafe fn nf_defrag_init() -> c_int {
    let mut ret: c_int = 0;

    ret = nf_ct_frag6_init();
    if ret < 0 {
        pr_err!("nf_defrag_ipv6: can't initialize frag6.\n");
        return ret;
    }
    ret = register_pernet_subsys(&mut defrag6_net_ops);
    if ret < 0 {
        pr_err!("nf_defrag_ipv6: can't register pernet ops\n");
        nf_ct_frag6_cleanup();
        return ret;
    }

    rcu_assign_pointer!(nf_defrag_v6_hook, &defrag_hook);
    ret
}

unsafe fn nf_defrag_fini() {
    rcu_assign_pointer!(nf_defrag_v6_hook, core::ptr::null());
    unregister_pernet_subsys(&mut defrag6_net_ops);
    nf_ct_frag6_cleanup();
}

pub unsafe fn nf_defrag_ipv6_enable(net: *mut net) -> c_int {
    let mut err: c_int = 0;

    mutex_lock(&mut defrag6_mutex);
    if (*net).nf.defrag_ipv6_users == UINT_MAX {
        err = -EOVERFLOW;
        mutex_unlock(&mut defrag6_mutex);
        return err;
    }

    if (*net).nf.defrag_ipv6_users != 0 {
        (*net).nf.defrag_ipv6_users += 1;
        mutex_unlock(&mut defrag6_mutex);
        return err;
    }

    err = nf_register_net_hooks(net, ipv6_defrag_ops.as_ptr(), ipv6_defrag_ops.len());
    if err == 0 {
        (*net).nf.defrag_ipv6_users = 1;
    }

    mutex_unlock(&mut defrag6_mutex);
    err
}

unsafe fn nf_defrag_ipv6_disable(net: *mut net) {
    mutex_lock(&mut defrag6_mutex);
    if (*net).nf.defrag_ipv6_users != 0 {
        (*net).nf.defrag_ipv6_users -= 1;
        if (*net).nf.defrag_ipv6_users == 0 {
            nf_unregister_net_hooks(net, ipv6_defrag_ops.as_ptr(), ipv6_defrag_ops.len());
        }
    }
    mutex_unlock(&mut defrag6_mutex);
}

module_init!(nf_defrag_init);
module_exit!(nf_defrag_fini);

module_license!("GPL");
module_description!("IPv6 defragmentation support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
