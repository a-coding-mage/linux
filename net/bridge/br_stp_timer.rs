// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Spanning tree protocol; timer-related code
 *	Linux ethernet bridge
 *
 *	Authors:
 *	Lennert Buytenhek		<buytenh@gnu.org>
 */

// Dependencies supplied by the surrounding bridge implementation are intentionally
// referenced here rather than reimplemented in this translation unit.

/* called under bridge lock */
unsafe fn br_is_designated_for_some_port(br: *const net_bridge) -> libc::c_int {
    let mut p: *mut net_bridge_port;

    // Equivalent of list_for_each_entry(p, &br->port_list, list).
    list_for_each_entry!(p, &(*br).port_list, list) {
        if (*p).state != BR_STATE_DISABLED
            && libc::memcmp(
                &(*p).designated_bridge as *const _ as *const libc::c_void,
                &(*br).bridge_id as *const _ as *const libc::c_void,
                8,
            ) == 0
        {
            return 1;
        }
    }

    0
}

unsafe fn br_hello_timer_expired(t: *mut timer_list) {
    let br: *mut net_bridge = timer_container_of!(br, t, hello_timer);

    br_debug!(br, "hello timer expired\n");
    spin_lock(&mut (*br).lock);
    if (*(*br).dev).flags & IFF_UP != 0 {
        br_config_bpdu_generation(br);

        if (*br).stp_enabled == BR_KERNEL_STP {
            mod_timer(
                &mut (*br).hello_timer,
                round_jiffies(jiffies + (*br).hello_time),
            );
        }
    }
    spin_unlock(&mut (*br).lock);
}

unsafe fn br_message_age_timer_expired(t: *mut timer_list) {
    let p: *mut net_bridge_port = timer_container_of!(p, t, message_age_timer);
    let br: *mut net_bridge = (*p).br;
    let id: *const bridge_id = &(*p).designated_bridge;
    let mut was_root: libc::c_int;

    if (*p).state == BR_STATE_DISABLED {
        return;
    }

    br_info!(
        br,
        "port %u(%s) neighbor %.2x%.2x.%pM lost\n",
        (*p).port_no as libc::c_uint,
        (*(*p).dev).name,
        (*id).prio[0],
        (*id).prio[1],
        &(*id).addr,
    );

    /*
     * According to the spec, the message age timer cannot be
     * running when we are the root bridge. So..  this was_root
     * check is redundant. I'm leaving it in for now, though.
     */
    spin_lock(&mut (*br).lock);
    if (*p).state == BR_STATE_DISABLED {
        goto_unlock!(unlock);
    }
    was_root = br_is_root_bridge(br);

    br_become_designated_port(p);
    br_configuration_update(br);
    br_port_state_selection(br);
    if br_is_root_bridge(br) != 0 && was_root == 0 {
        br_become_root_bridge(br);
    }
unlock:
    spin_unlock(&mut (*br).lock);
}

unsafe fn br_forward_delay_timer_expired(t: *mut timer_list) {
    let p: *mut net_bridge_port = timer_container_of!(p, t, forward_delay_timer);
    let br: *mut net_bridge = (*p).br;

    br_debug!(
        br,
        "port %u(%s) forward delay timer\n",
        (*p).port_no as libc::c_uint,
        (*(*p).dev).name,
    );
    spin_lock(&mut (*br).lock);
    if (*p).state == BR_STATE_LISTENING {
        br_set_state(p, BR_STATE_LEARNING);
        mod_timer(
            &mut (*p).forward_delay_timer,
            jiffies + (*br).forward_delay,
        );
    } else if (*p).state == BR_STATE_LEARNING {
        br_set_state(p, BR_STATE_FORWARDING);
        if br_is_designated_for_some_port(br) != 0 {
            br_topology_change_detection(br);
        }
        netif_carrier_on((*br).dev);
    }
    rcu_read_lock();
    br_ifinfo_notify(RTM_NEWLINK, core::ptr::null_mut(), p);
    rcu_read_unlock();
    spin_unlock(&mut (*br).lock);
}

unsafe fn br_tcn_timer_expired(t: *mut timer_list) {
    let br: *mut net_bridge = timer_container_of!(br, t, tcn_timer);

    br_debug!(br, "tcn timer expired\n");
    spin_lock(&mut (*br).lock);
    if br_is_root_bridge(br) == 0 && (*(*br).dev).flags & IFF_UP != 0 {
        br_transmit_tcn(br);
        mod_timer(&mut (*br).tcn_timer, jiffies + (*br).bridge_hello_time);
    }
    spin_unlock(&mut (*br).lock);
}

unsafe fn br_topology_change_timer_expired(t: *mut timer_list) {
    let br: *mut net_bridge = timer_container_of!(br, t, topology_change_timer);

    br_debug!(br, "topo change timer expired\n");
    spin_lock(&mut (*br).lock);
    (*br).topology_change_detected = 0;
    __br_set_topology_change(br, 0);
    spin_unlock(&mut (*br).lock);
}

unsafe fn br_hold_timer_expired(t: *mut timer_list) {
    let p: *mut net_bridge_port = timer_container_of!(p, t, hold_timer);

    br_debug!(
        (*p).br,
        "port %u(%s) hold timer expired\n",
        (*p).port_no as libc::c_uint,
        (*(*p).dev).name,
    );

    spin_lock(&mut (*(*p).br).lock);
    if (*p).config_pending {
        br_transmit_config(p);
    }
    spin_unlock(&mut (*(*p).br).lock);
}

pub unsafe fn br_stp_timer_init(br: *mut net_bridge) {
    timer_setup!(&mut (*br).hello_timer, br_hello_timer_expired, 0);
    timer_setup!(&mut (*br).tcn_timer, br_tcn_timer_expired, 0);
    timer_setup!(
        &mut (*br).topology_change_timer,
        br_topology_change_timer_expired,
        0,
    );
}

pub unsafe fn br_stp_port_timer_init(p: *mut net_bridge_port) {
    timer_setup!(&mut (*p).message_age_timer, br_message_age_timer_expired, 0);
    timer_setup!(&mut (*p).forward_delay_timer, br_forward_delay_timer_expired, 0);
    timer_setup!(&mut (*p).hold_timer, br_hold_timer_expired, 0);
}

/* Report ticks left (in USER_HZ) used for API */
pub unsafe fn br_timer_value(timer: *const timer_list) -> libc::c_ulong {
    if timer_pending(timer) {
        jiffies_delta_to_clock_t((*timer).expires.wrapping_sub(jiffies))
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
