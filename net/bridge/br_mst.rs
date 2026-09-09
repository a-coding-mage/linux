// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Bridge Multiple Spanning Tree Support
 *
 *	Authors:
 *	Tobias Waldekranz		<tobias@waldekranz.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut BR_MST_USED: StaticKey = StaticKey::default();

pub unsafe fn br_mst_enabled(dev: *const net_device) -> bool {
    if !netif_is_bridge_master(dev) {
        return false;
    }

    br_opt_get(netdev_priv(dev), BROPT_MST_ENABLED)
}

pub unsafe fn br_mst_uninit(br: *mut net_bridge) {
    if br_opt_get(br, BROPT_MST_ENABLED) {
        static_branch_dec(&mut BR_MST_USED);
    }
}

pub unsafe fn br_mst_get_info(dev: *const net_device, msti: u16, vids: *mut usize) -> i32 {
    let vg: *const net_bridge_vlan_group;
    let br: *const net_bridge;

    ASSERT_RTNL!();

    if !netif_is_bridge_master(dev) {
        return -EINVAL;
    }

    br = netdev_priv(dev);
    if !br_opt_get(br, BROPT_MST_ENABLED) {
        return -EINVAL;
    }

    vg = br_vlan_group(br);
    list_for_each_entry!(v, &(*vg).vlan_list, vlist, {
        if (*v).msti == msti {
            __set_bit((*v).vid, vids);
        }
    });

    0
}

pub unsafe fn br_mst_get_state(dev: *const net_device, msti: u16, state: *mut u8) -> i32 {
    let p: *const net_bridge_port;
    let vg: *const net_bridge_vlan_group;

    ASSERT_RTNL!();

    p = br_port_get_check_rtnl(dev);
    if p.is_null() || !br_opt_get((*p).br, BROPT_MST_ENABLED) {
        return -EINVAL;
    }

    vg = nbp_vlan_group(p);
    list_for_each_entry!(v, &(*vg).vlan_list, vlist, {
        if (*v).brvlan.msti == msti {
            *state = (*v).state;
            return 0;
        }
    });

    -ENOENT
}

unsafe fn br_mst_vlan_set_state(vg: *mut net_bridge_vlan_group, v: *mut net_bridge_vlan, state: u8) {
    if br_vlan_get_state(v) == state {
        return;
    }

    if (*v).vid == (*vg).pvid {
        br_vlan_set_pvid_state(vg, state);
    }

    br_vlan_set_state(v, state);
}

pub unsafe fn br_mst_set_state(p: *mut net_bridge_port, msti: u16, state: u8,
                               extack: *mut netlink_ext_ack) -> i32 {
    let mut attr = switchdev_attr {
        id: SWITCHDEV_ATTR_ID_PORT_MST_STATE,
        orig_dev: (*p).dev,
        u: switchdev_attr_union::mst_state { msti, state },
    };
    let vg: *mut net_bridge_vlan_group;
    let mut err: i32 = 0;

    rcu_read_lock();
    vg = nbp_vlan_group_rcu(p);
    if vg.is_null() {
        rcu_read_unlock();
        return err;
    }

    /* MSTI 0 (CST) state changes are notified via the regular
     * SWITCHDEV_ATTR_ID_PORT_STP_STATE.
     */
    if msti != 0 {
        err = switchdev_port_attr_set((*p).dev, &mut attr, extack);
        if err != 0 && err != -EOPNOTSUPP {
            rcu_read_unlock();
            return err;
        }
    }

    err = 0;
    list_for_each_entry_rcu!(v, &(*vg).vlan_list, vlist, {
        if (*v).brvlan.msti != msti { continue; }
        br_mst_vlan_set_state(vg, v, state);
    });

    rcu_read_unlock();
    err
}

unsafe fn br_mst_vlan_sync_state(pv: *mut net_bridge_vlan, msti: u16) {
    let vg = nbp_vlan_group((*pv).port);
    list_for_each_entry!(v, &(*vg).vlan_list, vlist, {
        /* If this port already has a defined state in this
         * MSTI (through some other VLAN membership), inherit
         * it.
         */
        if v != pv && (*v).brvlan.msti == msti {
            br_mst_vlan_set_state(vg, pv, (*v).state);
            return;
        }
    });

    /* Otherwise, start out in a new MSTI with all ports disabled. */
    br_mst_vlan_set_state(vg, pv, BR_STATE_DISABLED);
}

pub unsafe fn br_mst_vlan_set_msti(mv: *mut net_bridge_vlan, msti: u16) -> i32 {
    let mut attr = switchdev_attr {
        id: SWITCHDEV_ATTR_ID_VLAN_MSTI,
        orig_dev: (*(*mv).br).dev,
        u: switchdev_attr_union::vlan_msti { vid: (*mv).vid, msti },
    };
    let mut err: i32;

    if (*mv).msti == msti { return 0; }
    err = switchdev_port_attr_set((*(*mv).br).dev, &mut attr, core::ptr::null_mut());
    if err != 0 && err != -EOPNOTSUPP { return err; }

    (*mv).msti = msti;
    list_for_each_entry!((*mv).br, p, port_list, list, {
        let vg = nbp_vlan_group(p);
        let pv = br_vlan_find(vg, (*mv).vid);
        if !pv.is_null() { br_mst_vlan_sync_state(pv, msti); }
    });
    0
}

pub unsafe fn br_mst_vlan_init_state(v: *mut net_bridge_vlan) {
    /* VLANs always start out in MSTI 0 (CST) */
    (*v).msti = 0;
    if br_vlan_is_master(v) { (*v).state = BR_STATE_FORWARDING; }
    else { (*v).state = (*(*v).port).state; }
}

pub unsafe fn br_mst_set_enabled(br: *mut net_bridge, on: bool,
                                  extack: *mut netlink_ext_ack) -> i32 {
    let mut attr = switchdev_attr {
        id: SWITCHDEV_ATTR_ID_BRIDGE_MST,
        orig_dev: (*br).dev,
        u: switchdev_attr_union::mst { on },
    };

    list_for_each_entry!(br, p, port_list, list, {
        let vg = nbp_vlan_group(p);
        if (*vg).num_vlans == 0 { continue; }
        NL_SET_ERR_MSG!(extack, "MST mode can't be changed while VLANs exist");
        return -EBUSY;
    });

    if br_opt_get(br, BROPT_MST_ENABLED) == on { return 0; }
    let err = switchdev_port_attr_set((*br).dev, &mut attr, extack);
    if err != 0 && err != -EOPNOTSUPP { return err; }

    if on { static_branch_inc(&mut BR_MST_USED); }
    else { static_branch_dec(&mut BR_MST_USED); }
    br_opt_toggle(br, BROPT_MST_ENABLED, on);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
