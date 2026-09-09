// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2007-2012 Nicira, Inc.
 */

// Dependencies supplied by the surrounding Open vSwitch/kernel translation.

unsafe fn dp_detach_port_notify(vport: *mut vport) {
    let mut notify: *mut sk_buff;
    let dp: *mut datapath;

    dp = (*vport).dp;
    notify = ovs_vport_cmd_build_info(
        vport,
        ovs_dp_get_net(dp),
        0,
        0,
        OVS_VPORT_CMD_DEL,
    );
    ovs_dp_detach_port(vport);
    if IS_ERR(notify) {
        genl_set_err(
            &mut dp_vport_genl_family,
            ovs_dp_get_net(dp),
            0,
            0,
            PTR_ERR(notify),
        );
        return;
    }

    genlmsg_multicast_netns(
        &mut dp_vport_genl_family,
        ovs_dp_get_net(dp),
        notify,
        0,
        0,
        GFP_KERNEL,
    );
}

pub unsafe fn ovs_dp_notify_wq(work: *mut work_struct) {
    let ovs_net: *mut ovs_net = container_of!(work, ovs_net, dp_notify_work);
    let mut dp: *mut datapath;

    ovs_lock();
    list_for_each_entry!(dp, &mut (*ovs_net).dps, list_node, {
        let mut i: i32;

        for i in 0..DP_VPORT_HASH_BUCKETS {
            let mut vport: *mut vport;
            let mut n: *mut hlist_node;

            hlist_for_each_entry_safe!(vport, n, &mut (*dp).ports[i as usize], dp_hash_node, {
                if (*(*vport).ops).type_ == OVS_VPORT_TYPE_INTERNAL {
                    continue;
                }

                if !netif_is_ovs_port((*vport).dev) {
                    dp_detach_port_notify(vport);
                }
            });
        }
    });
    ovs_unlock();
}

unsafe fn dp_device_event(
    _unused: *mut notifier_block,
    event: c_ulong,
    ptr: *mut c_void,
) -> c_int {
    let mut ovs_net: *mut ovs_net;
    let dev: *mut net_device = netdev_notifier_info_to_dev(ptr);
    let mut vport: *mut vport = core::ptr::null_mut();

    if !ovs_is_internal_dev(dev) {
        vport = ovs_netdev_get_vport(dev);
    }

    if vport.is_null() {
        return NOTIFY_DONE;
    }

    if event == NETDEV_UNREGISTER {
        /* upper_dev_unlink and decrement promisc immediately */
        ovs_netdev_detach_dev(vport);

        /* schedule vport destroy, dev_put and genl notification */
        ovs_net = net_generic(dev_net(dev), ovs_net_id);
        queue_work(system_percpu_wq, &mut (*ovs_net).dp_notify_work);
    }

    NOTIFY_DONE
}

#[no_mangle]
pub static mut ovs_dp_device_notifier: notifier_block = notifier_block {
    notifier_call: Some(dp_device_event),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
