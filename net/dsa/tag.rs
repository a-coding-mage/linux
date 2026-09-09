// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DSA tagging protocol handling
 *
 * Copyright (c) 2008-2009 Marvell Semiconductor
 * Copyright (c) 2013 Florian Fainelli <florian@openwrt.org>
 * Copyright (c) 2016 Andrew Lunn <andrew@lunn.ch>
 */

// Dependencies are provided by the surrounding kernel translation.

static mut DSA_TAG_DRIVERS_LIST: ListHead = ListHead::new();
static mut DSA_TAG_DRIVERS_LOCK: Mutex = Mutex::new();

/* Determine if we should defer delivery of skb until we have a rx timestamp.
 *
 * Called from dsa_switch_rcv. For now, this will only work if tagging is
 * enabled on the switch. Normally the MAC driver would retrieve the hardware
 * timestamp when it reads the packet out of the hardware. However in a DSA
 * switch, the DSA driver owning the interface to which the packet is
 * delivered is never notified unless we do so here.
+ */
unsafe fn dsa_skb_defer_rx_timestamp(p: *mut DsaUserPriv, skb: *mut SkBuff) -> bool {
    let ds = (*(*p).dp).ds;
    let mut type_: u32;

    if (*(*ds).ops).port_rxtstamp.is_none() {
        return false;
    }

    if skb_headroom(skb) < ETH_HLEN {
        return false;
    }

    __skb_push(skb, ETH_HLEN);
    type_ = ptp_classify_raw(skb);
    __skb_pull(skb, ETH_HLEN);

    if type_ == PTP_CLASS_NONE {
        return false;
    }

    ((*(*ds).ops).port_rxtstamp.unwrap())(ds, (*(*p).dp).index, skb, type_)
}

unsafe extern "C" fn dsa_switch_rcv(
    mut skb: *mut SkBuff,
    dev: *mut NetDevice,
    _pt: *mut PacketType,
    _unused: *mut NetDevice,
) -> i32 {
    let md_dst = skb_metadata_dst(skb);
    let cpu_dp = (*dev).dsa_ptr;
    let mut nskb: *mut SkBuff = core::ptr::null_mut();
    let p: *mut DsaUserPriv;

    if cpu_dp.is_null() {
        kfree_skb(skb);
        return 0;
    }

    skb = skb_unshare(skb, GFP_ATOMIC);
    if skb.is_null() {
        return 0;
    }

    if !md_dst.is_null() && (*md_dst).type_ == METADATA_HW_PORT_MUX {
        let port = (*md_dst).u.port_info.port_id;

        skb_dst_drop(skb);
        if !skb_has_extensions(skb) {
            (*skb).slow_gro = 0;
        }

        (*skb).dev = dsa_conduit_find_user(dev, 0, port);
        if !(*skb).dev.is_null() {
            dsa_default_offload_fwd_mark(skb);
            nskb = skb;
        } else {
            /* Just drop the skb if we can't find the user */
            kfree_skb(skb);
        }
    } else {
        nskb = ((*cpu_dp).rcv.unwrap())(skb, dev);
    }

    if nskb.is_null() {
        return 0;
    }

    skb = nskb;
    skb_push(skb, ETH_HLEN);
    (*skb).pkt_type = PACKET_HOST;
    (*skb).protocol = eth_type_trans(skb, (*skb).dev);

    if !dsa_user_dev_check((*skb).dev) {
        /* Packet is to be injected directly on an upper
         * device, e.g. a team/bond, so skip all DSA-port
         * specific actions.
         */
        netif_rx(skb);
        return 0;
    }

    p = netdev_priv((*skb).dev) as *mut DsaUserPriv;

    if (*(*cpu_dp).ds).untag_bridge_pvid || (*(*cpu_dp).ds).untag_vlan_aware_bridge_pvid {
        /* dsa_software_vlan_untag() drops skb on failure */
        nskb = dsa_software_vlan_untag(skb);
        if nskb.is_null() {
            return 0;
        }
        skb = nskb;
    }

    dev_sw_netstats_rx_add((*skb).dev, (*skb).len + ETH_HLEN);

    if dsa_skb_defer_rx_timestamp(p, skb) {
        return 0;
    }

    gro_cells_receive(&mut (*p).gcells, skb);
    0
}

#[no_mangle]
pub static mut dsa_pack_type: PacketType = PacketType {
    type_: cpu_to_be16(ETH_P_XDSA),
    func: Some(dsa_switch_rcv),
};

unsafe fn dsa_tag_driver_register(dsa_tag_driver: *mut DsaTagDriver, owner: *mut Module) {
    (*dsa_tag_driver).owner = owner;
    mutex_lock(&mut DSA_TAG_DRIVERS_LOCK);
    list_add_tail(&mut (*dsa_tag_driver).list, &mut DSA_TAG_DRIVERS_LIST);
    mutex_unlock(&mut DSA_TAG_DRIVERS_LOCK);
}

pub unsafe fn dsa_tag_drivers_register(
    dsa_tag_driver_array: *mut *mut DsaTagDriver,
    count: u32,
    owner: *mut Module,
) {
    for i in 0..count {
        dsa_tag_driver_register(*dsa_tag_driver_array.add(i as usize), owner);
    }
}

unsafe fn dsa_tag_driver_unregister(dsa_tag_driver: *mut DsaTagDriver) {
    mutex_lock(&mut DSA_TAG_DRIVERS_LOCK);
    list_del(&mut (*dsa_tag_driver).list);
    mutex_unlock(&mut DSA_TAG_DRIVERS_LOCK);
}

pub unsafe fn dsa_tag_drivers_unregister(
    dsa_tag_driver_array: *mut *mut DsaTagDriver,
    count: u32,
) {
    for i in 0..count {
        dsa_tag_driver_unregister(*dsa_tag_driver_array.add(i as usize));
    }
}

pub unsafe fn dsa_tag_protocol_to_str(ops: *const DsaDeviceOps) -> *const i8 {
    (*ops).name
}

/* Function takes a reference on the module owning the tagger,
 * so dsa_tag_driver_put must be called afterwards.
 */
pub unsafe fn dsa_tag_driver_get_by_name(name: *const i8) -> *const DsaDeviceOps {
    let mut ops = ERR_PTR(-ENOPROTOOPT);
    request_module(DSA_TAG_DRIVER_ALIAS, name);

    mutex_lock(&mut DSA_TAG_DRIVERS_LOCK);
    let mut driver = list_first_entry(&DSA_TAG_DRIVERS_LIST);
    while !driver.is_null() {
        let tmp = (*driver).ops;
        if strcmp(name, (*tmp).name) != 0 {
            driver = list_next_entry(driver);
            continue;
        }
        if !try_module_get((*driver).owner) {
            break;
        }
        ops = tmp;
        break;
    }
    mutex_unlock(&mut DSA_TAG_DRIVERS_LOCK);
    ops
}

pub unsafe fn dsa_tag_driver_get_by_id(tag_protocol: i32) -> *const DsaDeviceOps {
    let mut driver: *mut DsaTagDriver;
    let mut ops: *const DsaDeviceOps;
    let mut found = false;

    request_module_id(DSA_TAG_DRIVER_ALIAS, tag_protocol);
    mutex_lock(&mut DSA_TAG_DRIVERS_LOCK);
    driver = list_first_entry(&DSA_TAG_DRIVERS_LIST);
    while !driver.is_null() {
        ops = (*driver).ops;
        if (*ops).proto == tag_protocol {
            found = true;
            break;
        }
        driver = list_next_entry(driver);
    }
    if found {
        if !try_module_get((*driver).owner) {
            ops = ERR_PTR(-ENOPROTOOPT);
        }
    } else {
        ops = ERR_PTR(-ENOPROTOOPT);
    }
    mutex_unlock(&mut DSA_TAG_DRIVERS_LOCK);
    ops
}

pub unsafe fn dsa_tag_driver_put(ops: *const DsaDeviceOps) {
    mutex_lock(&mut DSA_TAG_DRIVERS_LOCK);
    let mut driver = list_first_entry(&DSA_TAG_DRIVERS_LIST);
    while !driver.is_null() {
        if (*driver).ops == ops {
            module_put((*driver).owner);
            break;
        }
        driver = list_next_entry(driver);
    }
    mutex_unlock(&mut DSA_TAG_DRIVERS_LOCK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
