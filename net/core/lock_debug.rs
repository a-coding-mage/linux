// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright Amazon.com Inc. or its affiliates. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined in this file.

pub unsafe extern "C" fn netdev_debug_event(
    nb: *mut notifier_block,
    event: ::core::ffi::c_ulong,
    ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let dev: *mut net_device = netdev_notifier_info_to_dev(ptr);
    let net: *mut net = dev_net(dev);
    let cmd: netdev_cmd = event as netdev_cmd;

    // Keep enum and don't add default to trigger -Werror=switch
    match cmd {
        NETDEV_XDP_FEAT_CHANGE => {
            netdev_assert_locked(dev);
            // fallthrough
            netdev_assert_locked_ops_compat(dev);
        }
        NETDEV_CHANGE | NETDEV_REGISTER | NETDEV_UP | NETDEV_DOWN | NETDEV_GOING_DOWN => {
            netdev_assert_locked_ops_compat(dev);
        }
        NETDEV_REBOOT
        | NETDEV_UNREGISTER
        | NETDEV_CHANGEMTU
        | NETDEV_CHANGEADDR
        | NETDEV_PRE_CHANGEADDR
        | NETDEV_FEAT_CHANGE
        | NETDEV_BONDING_FAILOVER
        | NETDEV_PRE_UP
        | NETDEV_PRE_TYPE_CHANGE
        | NETDEV_POST_TYPE_CHANGE
        | NETDEV_POST_INIT
        | NETDEV_PRE_UNINIT
        | NETDEV_RELEASE
        | NETDEV_NOTIFY_PEERS
        | NETDEV_JOIN
        | NETDEV_CHANGEUPPER
        | NETDEV_RESEND_IGMP
        | NETDEV_PRECHANGEMTU
        | NETDEV_CHANGEINFODATA
        | NETDEV_BONDING_INFO
        | NETDEV_PRECHANGEUPPER
        | NETDEV_CHANGELOWERSTATE
        | NETDEV_UDP_TUNNEL_PUSH_INFO
        | NETDEV_UDP_TUNNEL_DROP_INFO
        | NETDEV_CHANGE_TX_QUEUE_LEN
        | NETDEV_CVLAN_FILTER_PUSH_INFO
        | NETDEV_CVLAN_FILTER_DROP_INFO
        | NETDEV_SVLAN_FILTER_PUSH_INFO
        | NETDEV_SVLAN_FILTER_DROP_INFO
        | NETDEV_OFFLOAD_XSTATS_ENABLE
        | NETDEV_OFFLOAD_XSTATS_DISABLE
        | NETDEV_OFFLOAD_XSTATS_REPORT_USED
        | NETDEV_OFFLOAD_XSTATS_REPORT_DELTA => {
            ASSERT_RTNL();
        }
        NETDEV_CHANGENAME => {
            netdev_assert_locked_ops(dev);
            ASSERT_RTNL_NET(net);
        }
    }

    NOTIFY_DONE
}

// EXPORT_SYMBOL_NS_GPL(netdev_debug_event, "NETDEV_INTERNAL");

static mut rtnl_net_debug_net_id: ::core::ffi::c_int = 0;

unsafe extern "C" fn rtnl_net_debug_net_init(net: *mut net) -> ::core::ffi::c_int {
    let nb: *mut notifier_block = net_generic(net, &raw mut rtnl_net_debug_net_id);
    (*nb).notifier_call = Some(netdev_debug_event);

    register_netdevice_notifier_net(net, nb)
}

unsafe extern "C" fn rtnl_net_debug_net_exit(net: *mut net) {
    let nb: *mut notifier_block = net_generic(net, &raw mut rtnl_net_debug_net_id);
    unregister_netdevice_notifier_net(net, nb);
}

static mut rtnl_net_debug_net_ops: pernet_operations = pernet_operations {
    init: Some(rtnl_net_debug_net_init),
    exit: Some(rtnl_net_debug_net_exit),
    id: &raw mut rtnl_net_debug_net_id,
    size: core::mem::size_of::<notifier_block>(),
};

static mut rtnl_net_debug_block: notifier_block = notifier_block {
    notifier_call: Some(netdev_debug_event),
};

unsafe extern "C" fn rtnl_net_debug_init() -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;

    ret = register_pernet_subsys(&raw mut rtnl_net_debug_net_ops);
    if ret != 0 {
        return ret;
    }

    ret = register_netdevice_notifier(&raw mut rtnl_net_debug_block);
    if ret != 0 {
        unregister_pernet_subsys(&raw mut rtnl_net_debug_net_ops);
    }

    ret
}

// subsys_initcall(rtnl_net_debug_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
