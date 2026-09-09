// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * "TEE" target extension for Xtables
 * Copyright © Sebastian Claßen, 2007
 * Jan Engelhardt, 2007-2010
 *
 * based on ipt_ROUTE.c from Cédric de Launois
 * <delaunois@info.ucl.be>
 */
// Linux kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
struct xt_tee_priv {
    list: list_head,
    tginfo: *mut xt_tee_tginfo,
    oif: i32,
}

static mut tee_net_id: u32 = 0;
static mut tee_zero_address: nf_inet_addr = unsafe { core::mem::zeroed() };

#[repr(C)]
struct tee_net {
    priv_list: list_head,
    // lock protects the priv_list
    lock: mutex,
}

unsafe extern "C" fn tee_tg4(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let info = (*par).targinfo as *const xt_tee_tginfo;
    let oif = if !(*info).priv_.is_null() {
        (*(*info).priv_).oif
    } else {
        0
    };

    nf_dup_ipv4(
        xt_net(par),
        skb,
        xt_hooknum(par),
        &(*info).gw.in_,
        oif,
    );

    XT_CONTINUE
}

#[cfg(CONFIG_IP6_NF_IPTABLES)]
unsafe extern "C" fn tee_tg6(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> u32 {
    let info = (*par).targinfo as *const xt_tee_tginfo;
    let oif = if !(*info).priv_.is_null() {
        (*(*info).priv_).oif
    } else {
        0
    };

    nf_dup_ipv6(
        xt_net(par),
        skb,
        xt_hooknum(par),
        &(*info).gw.in6,
        oif,
    );

    XT_CONTINUE
}

unsafe extern "C" fn tee_netdev_event(
    this: *mut notifier_block,
    event: libc::c_ulong,
    ptr: *mut libc::c_void,
) -> i32 {
    let dev = netdev_notifier_info_to_dev(ptr);
    let net = dev_net(dev);
    let tn = net_generic(net, tee_net_id);

    mutex_lock(&mut (*tn).lock);
    let mut pos = (*tn).priv_list.next;
    while pos != &mut (*tn).priv_list as *mut list_head {
        let priv_ = container_of!(pos, xt_tee_priv, list);
        match event {
            NETDEV_REGISTER => {
                if strcmp((*dev).name.as_ptr(), (*(*priv_).tginfo).oif.as_ptr()) == 0 {
                    (*priv_).oif = (*dev).ifindex;
                }
            }
            NETDEV_UNREGISTER => {
                if (*dev).ifindex == (*priv_).oif {
                    (*priv_).oif = -1;
                }
            }
            NETDEV_CHANGENAME => {
                if strcmp((*dev).name.as_ptr(), (*(*priv_).tginfo).oif.as_ptr()) == 0 {
                    (*priv_).oif = (*dev).ifindex;
                } else if (*dev).ifindex == (*priv_).oif {
                    (*priv_).oif = -1;
                }
            }
            _ => {}
        }
        pos = (*pos).next;
    }
    mutex_unlock(&mut (*tn).lock);

    NOTIFY_DONE
}

unsafe extern "C" fn tee_tg_check(par: *const xt_tgchk_param) -> i32 {
    let tn = net_generic((*par).net, tee_net_id);
    let info = (*par).targinfo as *mut xt_tee_tginfo;

    // 0.0.0.0 and :: not allowed
    if memcmp(
        &(*info).gw as *const _ as *const libc::c_void,
        &tee_zero_address as *const _ as *const libc::c_void,
        core::mem::size_of::<nf_inet_addr>(),
    ) == 0 {
        return -EINVAL;
    }

    if (*info).oif[0] != 0 {
        if (*info).oif[core::mem::size_of_val(&(*info).oif) - 1] != 0 {
            return -EINVAL;
        }

        let priv_ = kzalloc::<xt_tee_priv>();
        if priv_.is_null() {
            return -ENOMEM;
        }

        (*priv_).tginfo = info;
        (*priv_).oif = -1;
        (*info).priv_ = priv_;

        let dev = dev_get_by_name((*par).net, (*info).oif.as_ptr());
        if !dev.is_null() {
            (*priv_).oif = (*dev).ifindex;
            dev_put(dev);
        }
        mutex_lock(&mut (*tn).lock);
        list_add(&mut (*priv_).list, &mut (*tn).priv_list);
        mutex_unlock(&mut (*tn).lock);
    } else {
        (*info).priv_ = core::ptr::null_mut();
    }

    static_key_slow_inc(&mut xt_tee_enabled);
    0
}

unsafe extern "C" fn tee_tg_destroy(par: *const xt_tgdtor_param) {
    let tn = net_generic((*par).net, tee_net_id);
    let info = (*par).targinfo as *mut xt_tee_tginfo;

    if !(*info).priv_.is_null() {
        mutex_lock(&mut (*tn).lock);
        list_del(&mut (*(*info).priv_).list);
        mutex_unlock(&mut (*tn).lock);
        kfree((*info).priv_);
    }
    static_key_slow_dec(&mut xt_tee_enabled);
}

static mut tee_tg_reg: [xt_target; 2] = [
    xt_target {
        name: *b"TEE\0",
        revision: 1,
        family: NFPROTO_IPV4,
        target: Some(tee_tg4),
        targetsize: core::mem::size_of::<xt_tee_tginfo>(),
        usersize: core::mem::offset_of!(xt_tee_tginfo, priv_),
        checkentry: Some(tee_tg_check),
        destroy: Some(tee_tg_destroy),
        me: THIS_MODULE,
    },
    #[cfg(CONFIG_IP6_NF_IPTABLES)]
    xt_target {
        name: *b"TEE\0",
        revision: 1,
        family: NFPROTO_IPV6,
        target: Some(tee_tg6),
        targetsize: core::mem::size_of::<xt_tee_tginfo>(),
        usersize: core::mem::offset_of!(xt_tee_tginfo, priv_),
        checkentry: Some(tee_tg_check),
        destroy: Some(tee_tg_destroy),
        me: THIS_MODULE,
    },
];

unsafe extern "C" fn tee_net_init(net: *mut net) -> i32 {
    let tn = net_generic(net, tee_net_id);
    INIT_LIST_HEAD(&mut (*tn).priv_list);
    mutex_init(&mut (*tn).lock);
    0
}

static mut tee_net_ops: pernet_operations = pernet_operations {
    init: Some(tee_net_init),
    id: &mut tee_net_id,
    size: core::mem::size_of::<tee_net>(),
};

static mut tee_netdev_notifier: notifier_block = notifier_block {
    notifier_call: Some(tee_netdev_event),
};

unsafe extern "C" fn tee_tg_init() -> i32 {
    let mut ret = register_pernet_subsys(&mut tee_net_ops);
    if ret < 0 {
        return ret;
    }

    ret = xt_register_targets(tee_tg_reg.as_mut_ptr(), tee_tg_reg.len());
    if ret < 0 {
        unregister_pernet_subsys(&mut tee_net_ops);
        return ret;
    }

    ret = register_netdevice_notifier(&mut tee_netdev_notifier);
    if ret < 0 {
        xt_unregister_targets(tee_tg_reg.as_mut_ptr(), tee_tg_reg.len());
        unregister_pernet_subsys(&mut tee_net_ops);
        return ret;
    }

    0
}

unsafe extern "C" fn tee_tg_exit() {
    unregister_netdevice_notifier(&mut tee_netdev_notifier);
    xt_unregister_targets(tee_tg_reg.as_mut_ptr(), tee_tg_reg.len());
    unregister_pernet_subsys(&mut tee_net_ops);
}

module_init!(tee_tg_init);
module_exit!(tee_tg_exit);
module_author!("Sebastian Claßen <sebastian.classen@freenet.ag>");
module_author!("Jan Engelhardt <jengelh@medozas.de>");
module_description!("Xtables: Reroute packet copy");
module_license!("GPL");
module_alias!("ipt_TEE");
module_alias!("ip6t_TEE");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
