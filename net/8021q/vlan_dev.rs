// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of vlan_dev.c. */

unsafe fn vlan_dev_hard_header(skb: *mut sk_buff, dev: *mut net_device, mut ty: u16,
                               daddr: *const c_void, mut saddr: *const c_void,
                               len: c_uint) -> c_int {
    let vlan = vlan_dev_priv(dev);
    let mut vhdrlen: c_uint = 0;
    let mut vlan_tci: u16 = 0;
    if ((*vlan).flags & VLAN_FLAG_REORDER_HDR) == 0 {
        let vhdr = skb_push(skb, VLAN_HLEN);
        vlan_tci = (*vlan).vlan_id | vlan_dev_get_egress_qos_mask(dev, (*skb).priority);
        (*vhdr).h_vlan_TCI = htons(vlan_tci);
        (*vhdr).h_vlan_encapsulated_proto = htons(if ty != ETH_P_802_3 && ty != ETH_P_802_2 { ty } else { len as u16 });
        (*skb).protocol = (*vlan).vlan_proto;
        ty = ntohs((*vlan).vlan_proto);
        vhdrlen = VLAN_HLEN;
    }
    if saddr.is_null() { saddr = (*dev).dev_addr; }
    let rc = dev_hard_header(skb, (*vlan).real_dev, ty, daddr, saddr, len + vhdrlen);
    if rc > 0 { rc + vhdrlen as c_int } else { rc }
}

unsafe fn vlan_netpoll_send_skb(vlan: *mut vlan_dev_priv, skb: *mut sk_buff) -> netdev_tx_t {
    #[cfg(CONFIG_NET_POLL_CONTROLLER)]
    { return netpoll_send_skb((*vlan).netpoll, skb); }
    #[cfg(not(CONFIG_NET_POLL_CONTROLLER))]
    { BUG(); NETDEV_TX_OK }
}

unsafe fn vlan_dev_hard_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    let vlan = vlan_dev_priv(dev);
    let veth = (*skb).data as *mut vlan_ethhdr;
    if ((*vlan).flags & VLAN_FLAG_REORDER_HDR) != 0 || (*veth).h_vlan_proto != (*vlan).vlan_proto {
        let tci = (*vlan).vlan_id | vlan_dev_get_egress_qos_mask(dev, (*skb).priority);
        __vlan_hwaccel_put_tag(skb, (*vlan).vlan_proto, tci);
    }
    (*skb).dev = (*vlan).real_dev;
    let len = (*skb).len;
    if netpoll_tx_running(dev) { return vlan_netpoll_send_skb(vlan, skb); }
    let ret = dev_queue_xmit(skb);
    if ret == NET_XMIT_SUCCESS || ret == NET_XMIT_CN {
        let stats = this_cpu_ptr((*vlan).vlan_pcpu_stats);
        u64_stats_update_begin(&mut (*stats).syncp);
        u64_stats_inc(&mut (*stats).tx_packets);
        u64_stats_add(&mut (*stats).tx_bytes, len);
        u64_stats_update_end(&mut (*stats).syncp);
    } else { this_cpu_inc(&mut (*(*vlan).vlan_pcpu_stats).tx_dropped); }
    ret
}

unsafe fn vlan_dev_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    let real = (*vlan_dev_priv(dev)).real_dev;
    let mut max = (*real).mtu;
    if netif_reduces_vlan_mtu(real) { max -= VLAN_HLEN as c_int; }
    if max < new_mtu { return -ERANGE; }
    WRITE_ONCE(&mut (*dev).mtu, new_mtu); 0
}

pub unsafe fn vlan_dev_set_ingress_priority(dev: *const net_device, skb_prio: u32, vlan_prio: u16) {
    let vlan = vlan_dev_priv(dev as *mut net_device); let i = (vlan_prio & 7) as usize;
    if (*vlan).ingress_priority_map[i] != 0 && skb_prio == 0 { (*vlan).nr_ingress_mappings -= 1; }
    else if (*vlan).ingress_priority_map[i] == 0 && skb_prio != 0 { (*vlan).nr_ingress_mappings += 1; }
    (*vlan).ingress_priority_map[i] = skb_prio;
}

pub unsafe fn vlan_dev_set_egress_priority(dev: *const net_device, skb_prio: u32, vlan_prio: u16) -> c_int {
    let vlan = vlan_dev_priv(dev as *mut net_device); let bucket = (skb_prio & 0xf) as usize;
    let qos = ((vlan_prio as u32) << VLAN_PRIO_SHIFT) & VLAN_PRIO_MASK;
    let mut mpp = &mut (*vlan).egress_priority_map[bucket] as *mut _;
    let mut mp = rtnl_dereference(*mpp);
    while !mp.is_null() {
        if (*mp).priority == skb_prio {
            if qos == 0 { rcu_assign_pointer(mpp, rtnl_dereference((*mp).next)); (*vlan).nr_egress_mappings -= 1; kfree_rcu(mp); }
            else { WRITE_ONCE(&mut (*mp).vlan_qos, qos); }
            return 0;
        }
        mpp = &mut (*mp).next as *mut _; mp = rtnl_dereference(*mpp);
    }
    if qos == 0 { return 0; }
    let np = kmalloc_obj::<vlan_priority_tci_mapping>(); if np.is_null() { return -ENOBUFS; }
    (*np).priority = skb_prio; (*np).vlan_qos = qos;
    RCU_INIT_POINTER((*np).next, rtnl_dereference((*vlan).egress_priority_map[bucket]));
    rcu_assign_pointer((*vlan).egress_priority_map[bucket], np); (*vlan).nr_egress_mappings += 1; 0
}

pub unsafe fn vlan_dev_change_flags(dev: *const net_device, flags: u32, mask: u32) -> c_int {
    let vlan = vlan_dev_priv(dev as *mut net_device); let old = (*vlan).flags;
    if mask & !(VLAN_FLAG_REORDER_HDR | VLAN_FLAG_GVRP | VLAN_FLAG_LOOSE_BINDING | VLAN_FLAG_MVRP | VLAN_FLAG_BRIDGE_BINDING) != 0 { return -EINVAL; }
    (*vlan).flags = (old & !mask) | (flags & mask); 0
}

pub unsafe fn vlan_dev_get_realdev_name(dev: *const net_device, result: *mut c_char, size: usize) { strscpy_pad(result, (*(*vlan_dev_priv(dev as *mut net_device)).real_dev).name, size); }
pub unsafe fn vlan_dev_inherit_address(dev: *mut net_device, real: *mut net_device) -> bool {
    if (*dev).addr_assign_type != NET_ADDR_STOLEN { return false; }
    eth_hw_addr_set(dev, (*real).dev_addr); call_netdevice_notifiers(NETDEV_CHANGEADDR, dev); true
}

// Remaining operations retain the kernel ABI and delegation shape.
pub unsafe fn vlan_dev_free_egress_priority(dev: *const net_device) {
    let vlan = vlan_dev_priv(dev as *mut net_device);
    for i in 0..ARRAY_SIZE((*vlan).egress_priority_map) { let mut p = rtnl_dereference((*vlan).egress_priority_map[i]); RCU_INIT_POINTER((*vlan).egress_priority_map[i], core::ptr::null_mut()); while !p.is_null() { let n = rtnl_dereference((*p).next); kfree_rcu(p); p = n; } }
    (*vlan).nr_egress_mappings = 0;
}

pub unsafe fn vlan_dev_uninit(dev: *mut net_device) { vlan_dev_free_egress_priority(dev); }
pub unsafe fn vlan_dev_free(dev: *mut net_device) { let v=vlan_dev_priv(dev); free_percpu((*v).vlan_pcpu_stats); (*v).vlan_pcpu_stats=core::ptr::null_mut(); netdev_put((*v).real_dev, &mut (*v).dev_tracker); }
pub unsafe fn vlan_setup(dev: *mut net_device) { ether_setup(dev); (*dev).priv_flags |= IFF_802_1Q_VLAN | IFF_NO_QUEUE | IFF_UNICAST_FLT; (*dev).priv_flags &= !IFF_TX_SKB_SHARING; netif_keep_dst(dev); (*dev).needs_free_netdev=true; (*dev).priv_destructor=Some(vlan_dev_free); (*dev).min_mtu=0; (*dev).max_mtu=ETH_MAX_MTU; eth_zero_addr((*dev).broadcast); }

// Kernel callbacks and feature-conditional callbacks from the implementation.
pub unsafe fn vlan_dev_open(dev: *mut net_device) -> c_int { let v=vlan_dev_priv(dev); let r=(*v).real_dev; if ((*r).flags & IFF_UP)==0 && ((*v).flags & VLAN_FLAG_LOOSE_BINDING)==0 { return -ENETDOWN; } if !ether_addr_equal((*dev).dev_addr,(*r).dev_addr) && !vlan_dev_inherit_address(dev,r) { let e=dev_uc_add(r,(*dev).dev_addr); if e<0{return e;} } ether_addr_copy((*v).real_dev_addr,(*r).dev_addr); if netif_carrier_ok(r) && ((*v).flags & VLAN_FLAG_BRIDGE_BINDING)==0 { netif_carrier_on(dev); } 0 }
pub unsafe fn vlan_dev_stop(dev: *mut net_device) -> c_int { let v=vlan_dev_priv(dev); let r=(*v).real_dev; dev_mc_unsync(r,dev); dev_uc_unsync(r,dev); if !ether_addr_equal((*dev).dev_addr,(*r).dev_addr){dev_uc_del(r,(*dev).dev_addr);} if ((*v).flags & VLAN_FLAG_BRIDGE_BINDING)==0{netif_carrier_off(dev);} 0 }
pub unsafe fn vlan_dev_set_mac_address(dev:*mut net_device,p:*mut c_void)->c_int{let a=p as *mut sockaddr;if !is_valid_ether_addr((*a).sa_data){return -EADDRNOTAVAIL;} eth_hw_addr_set(dev,(*a).sa_data);0}
pub unsafe fn vlan_parse_protocol(skb:*const sk_buff)->__be16{let v=(*skb).data as *mut vlan_ethhdr;__vlan_get_protocol(skb,v as *mut _,core::ptr::null_mut())}
pub unsafe fn vlan_dev_get_iflink(dev:*const net_device)->c_int{READ_ONCE(&(*(*vlan_dev_priv(dev as *mut _)).real_dev).ifindex)}
pub unsafe fn vlan_dev_fix_features(_dev:*mut net_device, features:netdev_features_t)->netdev_features_t{features}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
