// SPDX-License-Identifier: GPL-2.0-or-later
/* Spanning tree protocol; generic parts. Linux ethernet bridge. */

// Dependencies supplied by the surrounding kernel translation.
const MESSAGE_AGE_INCR: u64 = (HZ / 256) + 1;

static BR_PORT_STATE_NAMES: [&str; 5] = [
    "disabled", "listening", "learning", "forwarding", "blocking",
];

pub unsafe fn br_set_state(p: *mut net_bridge_port, state: c_uint) {
    let attr = switchdev_attr { orig_dev: (*p).dev, id: SWITCHDEV_ATTR_ID_PORT_STP_STATE,
        flags: SWITCHDEV_F_DEFER, u: switchdev_attr_union { stp_state: state } };
    let mut err: c_int;
    if test_bit(BR_MRP_AWARE_BIT, &(*p).flags) != 0 { return; }
    (*p).state = state;
    if br_opt_get((*p).br, BROPT_MST_ENABLED) != 0 {
        err = br_mst_set_state(p, 0, state, core::ptr::null_mut());
        if err != 0 { br_warn((*p).br, "error setting MST state on port %u(%s)\n", (*p).port_no, netdev_name((*p).dev)); }
    }
    err = switchdev_port_attr_set((*p).dev, &attr, core::ptr::null_mut());
    if err != 0 && err != -EOPNOTSUPP { br_warn((*p).br, "error setting offload STP state on port %u(%s)\n", (*p).port_no, (*p).dev).name); }
    else { br_info((*p).br, "port %u(%s) entered %s state\n", (*p).port_no, (*p).dev).name, BR_PORT_STATE_NAMES[(*p).state as usize]); }
    if (*p).br.stp_enabled == BR_KERNEL_STP {
        match (*p).state { BR_STATE_BLOCKING => (*p).stp_xstats.transition_blk += 1, BR_STATE_FORWARDING => (*p).stp_xstats.transition_fwd += 1, _ => {} }
    }
}

pub unsafe fn br_port_get_stp_state(dev: *const net_device) -> u8 {
    ASSERT_RTNL();
    let p = br_port_get_rtnl(dev);
    if p.is_null() { return BR_STATE_DISABLED as u8; }
    (*p).state as u8
}

pub unsafe fn br_get_port(br: *mut net_bridge, port_no: u16) -> *mut net_bridge_port {
    let mut p: *mut net_bridge_port;
    list_for_each_entry_rcu!(p, &(*br).port_list, list, lockdep_is_held(&(*br).lock));
    if (*p).port_no == port_no { return p; }
    core::ptr::null_mut()
}

unsafe fn br_should_become_root_port(p: *const net_bridge_port, root_port: u16) -> c_int {
    let br = (*p).br;
    if (*p).state == BR_STATE_DISABLED || br_is_designated_port(p) != 0 { return 0; }
    if memcmp(&(*br).bridge_id, &(*p).designated_root, 8) <= 0 { return 0; }
    if root_port == 0 { return 1; }
    let rp = br_get_port(br, root_port);
    let t = memcmp(&(*p).designated_root, &(*rp).designated_root, 8);
    if t < 0 { return 1; } if t > 0 { return 0; }
    let pc = READ_ONCE!((*p).path_cost); let rpc = READ_ONCE!((*rp).path_cost);
    let pdc = READ_ONCE!((*p).designated_cost); let rpdc = READ_ONCE!((*rp).designated_cost);
    if pdc + pc < rpdc + rpc { return 1; } if pdc + pc > rpdc + rpc { return 0; }
    let t = memcmp(&(*p).designated_bridge, &(*rp).designated_bridge, 8);
    if t < 0 { return 1; } if t > 0 { return 0; }
    if (*p).designated_port < (*rp).designated_port { return 1; }
    if (*p).designated_port > (*rp).designated_port { return 0; }
    if (*p).port_id < (*rp).port_id { return 1; } 0
}

unsafe fn br_root_port_block(br: *const net_bridge, p: *mut net_bridge_port) {
    br_notice(br, "port %u(%s) tried to become root port (blocked)\n", (*p).port_no, (*p).dev).name);
    br_set_state(p, BR_STATE_LISTENING); br_ifinfo_notify(RTM_NEWLINK, core::ptr::null_mut(), p);
    if (*br).forward_delay > 0 { mod_timer(&(*p).forward_delay_timer, jiffies + (*br).forward_delay); }
}

unsafe fn br_root_selection(br: *mut net_bridge) {
    let mut root_port: u16 = 0; let mut p: *mut net_bridge_port;
    list_for_each_entry!(p, &(*br).port_list, list) {
        if br_should_become_root_port(p, root_port) == 0 { continue; }
        if test_bit(BR_ROOT_BLOCK_BIT, &(*p).flags) != 0 { br_root_port_block(br, p); } else { root_port = (*p).port_no; }
    }
    (*br).root_port = root_port;
    if root_port == 0 { (*br).designated_root = (*br).bridge_id; (*br).root_path_cost = 0; }
    else { p = br_get_port(br, root_port); (*br).designated_root = (*p).designated_root; (*br).root_path_cost = READ_ONCE!((*p).designated_cost) + READ_ONCE!((*p).path_cost); }
}

pub unsafe fn br_become_root_bridge(br: *mut net_bridge) {
    (*br).max_age = (*br).bridge_max_age; (*br).hello_time = (*br).bridge_hello_time; (*br).forward_delay = (*br).bridge_forward_delay;
    br_topology_change_detection(br); timer_delete(&(*br).tcn_timer);
    if (*br).dev.flags & IFF_UP != 0 { br_config_bpdu_generation(br); mod_timer(&(*br).hello_timer, jiffies + (*br).hello_time); }
}

pub unsafe fn br_transmit_config(p: *mut net_bridge_port) {
    if timer_pending(&(*p).hold_timer) { WRITE_ONCE!((*p).config_pending, 1); return; }
    let br = (*p).br; let mut bpdu: br_config_bpdu = core::mem::zeroed();
    bpdu.topology_change = (*br).topology_change; bpdu.topology_change_ack = (*p).topology_change_ack; bpdu.root = (*br).designated_root; bpdu.root_path_cost = (*br).root_path_cost; bpdu.bridge_id = (*br).bridge_id; bpdu.port_id = (*p).port_id;
    if br_is_root_bridge(br) != 0 { bpdu.message_age = 0; } else { let root = br_get_port(br, (*br).root_port); bpdu.message_age = jiffies - (*root).designated_age + MESSAGE_AGE_INCR; }
    bpdu.max_age = (*br).max_age; bpdu.hello_time = (*br).hello_time; bpdu.forward_delay = (*br).forward_delay;
    if bpdu.message_age < (*br).max_age { br_send_config_bpdu(p, &bpdu); (*p).topology_change_ack = 0; WRITE_ONCE!((*p).config_pending, 0); if (*p).br.stp_enabled == BR_KERNEL_STP { mod_timer(&(*p).hold_timer, round_jiffies(jiffies + BR_HOLD_TIME)); } }
}

unsafe fn br_record_config_information(p: *mut net_bridge_port, b: *const br_config_bpdu) { (*p).designated_root=(*b).root; WRITE_ONCE!((*p).designated_cost,(*b).root_path_cost); (*p).designated_bridge=(*b).bridge_id; WRITE_ONCE!((*p).designated_port,(*b).port_id); (*p).designated_age=jiffies-(*b).message_age; mod_timer(&(*p).message_age_timer,jiffies+((*b).max_age-(*b).message_age)); }
unsafe fn br_record_config_timeout_values(br:*mut net_bridge,b:*const br_config_bpdu){(*br).max_age=(*b).max_age;(*br).hello_time=(*b).hello_time;(*br).forward_delay=(*b).forward_delay;__br_set_topology_change(br,(*b).topology_change);}
pub unsafe fn br_transmit_tcn(br:*mut net_bridge){let p=br_get_port(br,(*br).root_port);if !p.is_null(){br_send_tcn_bpdu(p)}else{br_notice(br,"root port %u not found for topology notice\n",(*br).root_port);}}

// The remaining functions preserve the original control flow and use the surrounding kernel declarations.
pub unsafe fn br_configuration_update(br:*mut net_bridge){br_root_selection(br);br_designated_port_selection(br);}
pub unsafe fn br_become_designated_port(p:*mut net_bridge_port){let br=(*p).br;(*p).designated_root=(*br).designated_root;WRITE_ONCE!((*p).designated_cost,(*br).root_path_cost);(*p).designated_bridge=(*br).bridge_id;WRITE_ONCE!((*p).designated_port,(*p).port_id);}

unsafe fn br_should_become_designated_port(p:*const net_bridge_port)->c_int{let br=(*p).br;if br_is_designated_port(p)!=0{return 1;}if memcmp(&(*p).designated_root,&(*br).designated_root,8)!=0{return 1;}let c=READ_ONCE!((*p).designated_cost);if (*br).root_path_cost<c{return 1;}if (*br).root_path_cost>c{return 0;}let t=memcmp(&(*br).bridge_id,&(*p).designated_bridge,8);if t<0{return 1;}if t>0{return 0;}if (*p).port_id<(*p).designated_port{return 1;}0}
unsafe fn br_designated_port_selection(br:*mut net_bridge){let mut p:*mut net_bridge_port;list_for_each_entry!(p,&(*br).port_list,list){if (*p).state!=BR_STATE_DISABLED&&br_should_become_designated_port(p)!=0{br_become_designated_port(p);}}}
unsafe fn br_supersedes_port_info(p:*const net_bridge_port,b:*const br_config_bpdu)->c_int{let mut t=memcmp(&(*b).root,&(*p).designated_root,8);if t<0{return 1}if t>0{return 0}let c=READ_ONCE!((*p).designated_cost);if (*b).root_path_cost<c{return 1}if (*b).root_path_cost>c{return 0}t=memcmp(&(*b).bridge_id,&(*p).designated_bridge,8);if t<0{return 1}if t>0{return 0}if memcmp(&(*b).bridge_id,&(*p).br.bridge_id,8)!=0{return 1}if (*b).port_id<=(*p).designated_port{return 1}0}
unsafe fn br_topology_change_acknowledged(br:*mut net_bridge){(*br).topology_change_detected=0;timer_delete(&(*br).tcn_timer);}
pub unsafe fn br_topology_change_detection(br:*mut net_bridge){let root=br_is_root_bridge(br)!=0;if (*br).stp_enabled!=BR_KERNEL_STP||(*br).dev.flags&IFF_UP==0{return}br_info(br,"topology change detected, %s\n",if root{"propagating"}else{"sending tcn bpdu"});if root{__br_set_topology_change(br,1);mod_timer(&(*br).topology_change_timer,jiffies+(*br).bridge_forward_delay+(*br).bridge_max_age);}else if (*br).topology_change_detected==0{br_transmit_tcn(br);mod_timer(&(*br).tcn_timer,jiffies+(*br).bridge_hello_time);}(*br).topology_change_detected=1;}
pub unsafe fn br_config_bpdu_generation(br:*mut net_bridge){let mut p:*mut net_bridge_port;list_for_each_entry!(p,&(*br).port_list,list){if (*p).state!=BR_STATE_DISABLED&&br_is_designated_port(p)!=0{br_transmit_config(p);}}}
unsafe fn br_reply(p:*mut net_bridge_port){br_transmit_config(p)}
unsafe fn br_make_blocking(p:*mut net_bridge_port){if (*p).state!=BR_STATE_DISABLED&&(*p).state!=BR_STATE_BLOCKING{if (*p).state==BR_STATE_FORWARDING||(*p).state==BR_STATE_LEARNING{br_topology_change_detection((*p).br)}br_set_state(p,BR_STATE_BLOCKING);br_ifinfo_notify(RTM_NEWLINK,core::ptr::null_mut(),p);timer_delete(&(*p).forward_delay_timer);}}
unsafe fn br_make_forwarding(p:*mut net_bridge_port){let br=(*p).br;if (*p).state!=BR_STATE_BLOCKING{return}if (*br).stp_enabled==BR_NO_STP||(*br).forward_delay==0{br_set_state(p,BR_STATE_FORWARDING);br_topology_change_detection(br);timer_delete(&(*p).forward_delay_timer)}else if (*br).stp_enabled==BR_KERNEL_STP{br_set_state(p,BR_STATE_LISTENING)}else{br_set_state(p,BR_STATE_LEARNING)}br_ifinfo_notify(RTM_NEWLINK,core::ptr::null_mut(),p);if (*br).forward_delay!=0{mod_timer(&(*p).forward_delay_timer,jiffies+(*br).forward_delay);}}
pub unsafe fn br_port_state_selection(br:*mut net_bridge){let mut p:*mut net_bridge_port;let mut live=0;list_for_each_entry!(p,&(*br).port_list,list){if (*p).state==BR_STATE_DISABLED{continue}if (*br).stp_enabled!=BR_USER_STP{if (*p).port_no==(*br).root_port{WRITE_ONCE!((*p).config_pending,0);(*p).topology_change_ack=0;br_make_forwarding(p)}else if br_is_designated_port(p)!=0{timer_delete(&(*p).message_age_timer);br_make_forwarding(p)}else{WRITE_ONCE!((*p).config_pending,0);(*p).topology_change_ack=0;br_make_blocking(p)}}if (*p).state!=BR_STATE_BLOCKING{br_multicast_enable_port(p)}if (*p).state==BR_STATE_FORWARDING{live+=1}}if live==0{netif_carrier_off((*br).dev)}else{netif_carrier_on((*br).dev)}}
unsafe fn br_topology_change_acknowledge(p:*mut net_bridge_port){(*p).topology_change_ack=1;br_transmit_config(p)}
pub unsafe fn br_received_config_bpdu(p:*mut net_bridge_port,b:*const br_config_bpdu){(*p).stp_xstats.rx_bpdu+=1;let br=(*p).br;let was=br_is_root_bridge(br);if br_supersedes_port_info(p,b)!=0{br_record_config_information(p,b);br_configuration_update(br);br_port_state_selection(br);if br_is_root_bridge(br)==0&&was!=0{timer_delete(&(*br).hello_timer);if (*br).topology_change_detected!=0{timer_delete(&(*br).topology_change_timer);br_transmit_tcn(br);mod_timer(&(*br).tcn_timer,jiffies+(*br).bridge_hello_time)}}if (*p).port_no==(*br).root_port{br_record_config_timeout_values(br,b);br_config_bpdu_generation(br);if (*b).topology_change_ack!=0{br_topology_change_acknowledged(br)}}}else if br_is_designated_port(p)!=0{br_reply(p)}}
pub unsafe fn br_received_tcn_bpdu(p:*mut net_bridge_port){(*p).stp_xstats.rx_tcn+=1;if br_is_designated_port(p)!=0{br_info((*p).br,"port %u(%s) received tcn bpdu\n",(*p).port_no,(*p).dev).name);br_topology_change_detection((*p).br);br_topology_change_acknowledge(p)}}
pub unsafe fn br_set_hello_time(br:*mut net_bridge,val: c_ulong)->c_int{let t=clock_t_to_jiffies(val);if t<BR_MIN_HELLO_TIME||t>BR_MAX_HELLO_TIME{return -ERANGE}spin_lock_bh(&(*br).lock);(*br).bridge_hello_time=t;if br_is_root_bridge(br)!=0{(*br).hello_time=(*br).bridge_hello_time}spin_unlock_bh(&(*br).lock);0}
pub unsafe fn br_set_max_age(br:*mut net_bridge,val:c_ulong)->c_int{let t=clock_t_to_jiffies(val);if t<BR_MIN_MAX_AGE||t>BR_MAX_MAX_AGE{return -ERANGE}spin_lock_bh(&(*br).lock);(*br).bridge_max_age=t;if br_is_root_bridge(br)!=0{(*br).max_age=(*br).bridge_max_age}spin_unlock_bh(&(*br).lock);0}
pub unsafe fn __set_ageing_time(dev:*mut net_device,t:c_ulong)->c_int{let attr=switchdev_attr{orig_dev:dev,id:SWITCHDEV_ATTR_ID_BRIDGE_AGEING_TIME,flags:SWITCHDEV_F_SKIP_EOPNOTSUPP|SWITCHDEV_F_DEFER,u:switchdev_attr_union{ageing_time:jiffies_to_clock_t(t)}};let err=switchdev_port_attr_set(dev,&attr,core::ptr::null_mut());if err!=0&&err!=-EOPNOTSUPP{return err}0}
pub unsafe fn br_set_ageing_time(br:*mut net_bridge,ageing_time:clock_t)->c_int{let t=clock_t_to_jiffies(ageing_time);let err=__set_ageing_time((*br).dev,t);if err!=0{return err}spin_lock_bh(&(*br).lock);(*br).bridge_ageing_time=t;(*br).ageing_time=t;spin_unlock_bh(&(*br).lock);mod_delayed_work(system_long_wq,&mut (*br).gc_work,0);0}
pub unsafe fn br_get_ageing_time(dev:*const net_device)->clock_t{if netif_is_bridge_master(dev)==0{return 0}let br=netdev_priv(dev);jiffies_to_clock_t((*br).ageing_time)}
pub unsafe fn __br_set_topology_change(br:*mut net_bridge,val:u8){if (*br).stp_enabled==BR_KERNEL_STP&&(*br).topology_change!=val{let t=if val!=0{2*(*br).forward_delay}else{(*br).bridge_ageing_time};if __set_ageing_time((*br).dev,t)!=0{br_warn(br,"error offloading ageing time\n")}else{(*br).ageing_time=t}}(*br).topology_change=val}
pub unsafe fn __br_set_forward_delay(br:*mut net_bridge,t:c_ulong){(*br).bridge_forward_delay=t;if br_is_root_bridge(br)!=0{(*br).forward_delay=(*br).bridge_forward_delay}}
pub unsafe fn br_set_forward_delay(br:*mut net_bridge,val:c_ulong)->c_int{let t=clock_t_to_jiffies(val);let mut err=-ERANGE;spin_lock_bh(&(*br).lock);if (*br).stp_enabled!=BR_NO_STP&&(t<BR_MIN_FORWARD_DELAY||t>BR_MAX_FORWARD_DELAY){spin_unlock_bh(&(*br).lock);return err}__br_set_forward_delay(br,t);err=0;spin_unlock_bh(&(*br).lock);err}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
