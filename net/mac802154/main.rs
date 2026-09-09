// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007-2012 Siemens AG
 *
 * Written by:
 * Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 */

// Kernel and subsystem dependencies are supplied by other translation units.

unsafe fn ieee802154_tasklet_handler(t: *mut tasklet_struct) {
    let local: *mut ieee802154_local = from_tasklet!(local, t, tasklet);
    let mut skb: *mut sk_buff;

    while {
        skb = skb_dequeue(unsafe { &mut (*local).skb_queue });
        !skb.is_null()
    } {
        match unsafe { (*skb).pkt_type } {
            IEEE802154_RX_MSG => {
                /* Clear skb->pkt_type in order to not confuse kernel
                 * netstack.
                 */
                unsafe { (*skb).pkt_type = 0; }
                ieee802154_rx(local, skb);
            }
            _ => {
                WARN!(1, "mac802154: Packet is of unknown type %d\n", unsafe {
                    (*skb).pkt_type
                });
                kfree_skb(skb);
            }
        }
    }
}

pub unsafe fn ieee802154_alloc_hw(
    priv_data_len: usize,
    ops: *const ieee802154_ops,
) -> *mut ieee802154_hw {
    let mut phy: *mut wpan_phy;
    let local: *mut ieee802154_local;
    let priv_size: usize;

    if WARN_ON!(ops.is_null()
        || ((*ops).xmit_async.is_none() && (*ops).xmit_sync.is_none())
        || (*ops).ed.is_none()
        || (*ops).start.is_none()
        || (*ops).stop.is_none()
        || (*ops).set_channel.is_none())
    {
        return core::ptr::null_mut();
    }

    /* Ensure 32-byte alignment of our private data and hw private data.
     * We use the wpan_phy priv data for both our ieee802154_local and for
     * the driver's private data
     *
     * in memory it'll be like this:
     *
     * +-------------------------+
     * | struct wpan_phy         |
     * +-------------------------+
     * | struct ieee802154_local |
     * +-------------------------+
     * | driver's private data   |
     * +-------------------------+
     *
     * Due to ieee802154 layer isn't aware of driver and MAC structures,
     * so lets align them here.
     */
    priv_size = ALIGN!(core::mem::size_of::<ieee802154_local>(), NETDEV_ALIGN)
        + priv_data_len;

    phy = wpan_phy_new(&mac802154_config_ops, priv_size);
    if phy.is_null() {
        pr_err!("failure to allocate master IEEE802.15.4 device\n");
        return core::ptr::null_mut();
    }

    (*phy).privid = mac802154_wpan_phy_privid;

    local = wpan_phy_priv(phy);
    (*local).phy = phy;
    (*local).hw.phy = (*local).phy;
    (*local).hw.priv = (local as *mut u8).add(
        ALIGN!(core::mem::size_of::<ieee802154_local>(), NETDEV_ALIGN),
    ) as *mut core::ffi::c_void;
    (*local).ops = ops;

    INIT_LIST_HEAD!(&mut (*local).interfaces);
    INIT_LIST_HEAD!(&mut (*local).rx_beacon_list);
    INIT_LIST_HEAD!(&mut (*local).rx_mac_cmd_list);
    mutex_init!(&mut (*local).iflist_mtx);
    tasklet_setup!(&mut (*local).tasklet, ieee802154_tasklet_handler);
    skb_queue_head_init!(&mut (*local).skb_queue);
    INIT_WORK!(&mut (*local).sync_tx_work, ieee802154_xmit_sync_worker);
    INIT_DELAYED_WORK!(&mut (*local).scan_work, mac802154_scan_worker);
    INIT_WORK!(&mut (*local).rx_beacon_work, mac802154_rx_beacon_worker);
    INIT_DELAYED_WORK!(&mut (*local).beacon_work, mac802154_beacon_worker);
    INIT_WORK!(&mut (*local).rx_mac_cmd_work, mac802154_rx_mac_cmd_worker);
    init_completion!(&mut (*local).assoc_done);

    /* init supported flags with 802.15.4 default ranges */
    (*phy).supported.max_minbe = 8;
    (*phy).supported.min_maxbe = 3;
    (*phy).supported.max_maxbe = 8;
    (*phy).supported.min_frame_retries = 0;
    (*phy).supported.max_frame_retries = 7;
    (*phy).supported.max_csma_backoffs = 5;
    (*phy).supported.lbt = NL802154_SUPPORTED_BOOL_FALSE;
    /* always supported */
    (*phy).supported.iftypes = BIT!(NL802154_IFTYPE_NODE) | BIT!(NL802154_IFTYPE_COORD);

    &mut (*local).hw
}

pub unsafe fn ieee802154_configure_durations(
    phy: *mut wpan_phy,
    page: u32,
    channel: u32,
) {
    let mut duration: u32 = 0;

    match page {
        0 => {
            if BIT!(channel) & 0x1 != 0 {
                /* 868 MHz BPSK 802.15.4-2003: 20 ksym/s */
                duration = 50 * NSEC_PER_USEC;
            } else if BIT!(channel) & 0x7FE != 0 {
                /* 915 MHz BPSK 802.15.4-2003: 40 ksym/s */
                duration = 25 * NSEC_PER_USEC;
            } else if BIT!(channel) & 0x7FFF800 != 0 {
                /* 2400 MHz O-QPSK 802.15.4-2006: 62.5 ksym/s */
                duration = 16 * NSEC_PER_USEC;
            }
        }
        2 => {
            if BIT!(channel) & 0x1 != 0 {
                /* 868 MHz O-QPSK 802.15.4-2006: 25 ksym/s */
                duration = 40 * NSEC_PER_USEC;
            } else if BIT!(channel) & 0x7FE != 0 {
                /* 915 MHz O-QPSK 802.15.4-2006: 62.5 ksym/s */
                duration = 16 * NSEC_PER_USEC;
            }
        }
        3 => {
            if BIT!(channel) & 0x3FFF != 0 {
                /* 2.4 GHz CSS 802.15.4a-2007: 1/6 Msym/s */
                duration = 6 * NSEC_PER_USEC;
            }
        }
        _ => {}
    }

    if duration == 0 {
        pr_debug!("Unknown PHY symbol duration\n");
        return;
    }

    (*phy).symbol_duration = duration;
    (*phy).lifs_period = (IEEE802154_LIFS_PERIOD * (*phy).symbol_duration) / NSEC_PER_USEC;
    (*phy).sifs_period = (IEEE802154_SIFS_PERIOD * (*phy).symbol_duration) / NSEC_PER_USEC;
}

pub unsafe fn ieee802154_free_hw(hw: *mut ieee802154_hw) {
    let local = hw_to_local(hw);
    BUG_ON!(!list_empty!(&(*local).interfaces));
    mutex_destroy!(&mut (*local).iflist_mtx);
    wpan_phy_free((*local).phy);
}

unsafe fn ieee802154_setup_wpan_phy_pib(wpan_phy: *mut wpan_phy) {
    /* TODO warn on empty symbol_duration
     * Should be done when all drivers sets this value.
     */
    (*wpan_phy).lifs_period =
        (IEEE802154_LIFS_PERIOD * (*wpan_phy).symbol_duration) / NSEC_PER_USEC;
    (*wpan_phy).sifs_period =
        (IEEE802154_SIFS_PERIOD * (*wpan_phy).symbol_duration) / NSEC_PER_USEC;
}

pub unsafe fn ieee802154_register_hw(hw: *mut ieee802154_hw) -> i32 {
    let local = hw_to_local(hw);
    let mut mac_wq_name = [0u8; IFNAMSIZ + 10];
    let mut dev: *mut net_device;
    let mut rc: i32 = -ENOSYS;

    (*local).workqueue = create_singlethread_workqueue(wpan_phy_name((*local).phy));
    if (*local).workqueue.is_null() { rc = -ENOMEM; goto!(out); }
    snprintf!(mac_wq_name.as_mut_ptr(), IFNAMSIZ + 10, "%s-mac-cmds", wpan_phy_name((*local).phy));
    (*local).mac_wq = create_singlethread_workqueue(mac_wq_name.as_mut_ptr() as *const i8);
    if (*local).mac_wq.is_null() { rc = -ENOMEM; goto!(out_wq); }
    hrtimer_setup!(&mut (*local).ifs_timer, ieee802154_xmit_ifs_timer, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    wpan_phy_set_dev((*local).phy, (*local).hw.parent);
    ieee802154_setup_wpan_phy_pib((*local).phy);
    ieee802154_configure_durations((*local).phy, (*local).phy.current_page, (*local).phy.current_channel);
    if (*hw).flags & IEEE802154_HW_CSMA_PARAMS == 0 {
        (*local).phy.supported.min_csma_backoffs = 4;
        (*local).phy.supported.max_csma_backoffs = 4;
        (*local).phy.supported.min_maxbe = 5;
        (*local).phy.supported.max_maxbe = 5;
        (*local).phy.supported.min_minbe = 3;
        (*local).phy.supported.max_minbe = 3;
    }
    if (*hw).flags & IEEE802154_HW_FRAME_RETRIES == 0 {
        (*local).phy.supported.min_frame_retries = 3;
        (*local).phy.supported.max_frame_retries = 3;
    }
    if (*hw).flags & IEEE802154_HW_PROMISCUOUS != 0 {
        (*local).phy.supported.iftypes |= BIT!(NL802154_IFTYPE_MONITOR);
    }
    rc = wpan_phy_register((*local).phy);
    if rc < 0 { goto!(out_mac_wq); }
    rtnl_lock();
    dev = ieee802154_if_add(local, "wpan%d", NET_NAME_ENUM, NL802154_IFTYPE_NODE, cpu_to_le64!(0));
    if IS_ERR!(dev) {
        rtnl_unlock(); rc = PTR_ERR!(dev); goto!(out_phy);
    }
    rtnl_unlock();
    return 0;
out_phy: wpan_phy_unregister((*local).phy);
out_mac_wq: destroy_workqueue((*local).mac_wq);
out_wq: destroy_workqueue((*local).workqueue);
out: rc
}

pub unsafe fn ieee802154_unregister_hw(hw: *mut ieee802154_hw) {
    let local = hw_to_local(hw);
    tasklet_kill!(&mut (*local).tasklet);
    flush_workqueue((*local).workqueue);
    rtnl_lock();
    ieee802154_remove_interfaces(local);
    rtnl_unlock();
    destroy_workqueue((*local).mac_wq);
    destroy_workqueue((*local).workqueue);
    wpan_phy_unregister((*local).phy);
}

unsafe fn ieee802154_init() -> i32 { ieee802154_iface_init() }
unsafe fn ieee802154_exit() { ieee802154_iface_exit(); rcu_barrier(); }

// subsys_initcall(ieee802154_init);
// module_exit(ieee802154_exit);
// MODULE_DESCRIPTION("IEEE 802.15.4 subsystem");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
