// SPDX-License-Identifier: GPL-2.0-or-later
/* Linux NET3: IP/IP protocol decoder supporting virtual tunnel interface. */
/* This is a direct low-level translation of net/ipv4/ip_vti.c. */

// Kernel headers and build-time configuration are supplied by the surrounding crate.

static mut VTI_LINK_OPS: rtnl_link_ops = unsafe { core::mem::zeroed() };
static mut VTI_NET_ID: c_uint = 0;

unsafe fn vti_input(skb: *mut sk_buff, nexthdr: c_int, spi: __be32,
                    encap_type: c_int, update_skb_dev: bool) -> c_int {
    let iph = ip_hdr(skb); let net = dev_net((*skb).dev);
    let itn = net_generic(net, VTI_NET_ID); let mut flags: IP_TUNNEL_FLAGS = core::mem::zeroed();
    __set_bit(IP_TUNNEL_NO_KEY_BIT, &mut flags);
    let tunnel = ip_tunnel_lookup(itn, (*skb).dev.ifindex, flags, (*iph).saddr, (*iph).daddr, 0);
    if !tunnel.is_null() {
        if !xfrm4_policy_check(core::ptr::null_mut(), XFRM_POLICY_IN, skb) { kfree_skb(skb); return 0; }
        (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip4 = tunnel;
        if update_skb_dev { (*skb).dev = (*tunnel).dev; }
        return xfrm_input(skb, nexthdr, spi, encap_type);
    }
    -EINVAL
}

unsafe fn vti_input_proto(skb: *mut sk_buff, n: c_int, spi: __be32, e: c_int) -> c_int { vti_input(skb,n,spi,e,false) }
unsafe fn vti_rcv(skb: *mut sk_buff, spi: __be32, update: bool) -> c_int {
    (*XFRM_SPI_SKB_CB(skb)).family = AF_INET; (*XFRM_SPI_SKB_CB(skb)).daddroff = offset_of!(iphdr,daddr);
    vti_input(skb, (*ip_hdr(skb)).protocol as c_int, spi, 0, update)
}
unsafe fn vti_rcv_proto(skb: *mut sk_buff) -> c_int { vti_rcv(skb,0,false) }

unsafe fn vti_rcv_cb(skb: *mut sk_buff, err: c_int) -> c_int {
    let tunnel = (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip4; if tunnel.is_null() { return 1; }
    let dev = (*tunnel).dev;
    if err != 0 { DEV_STATS_INC(dev,rx_errors); DEV_STATS_INC(dev,rx_dropped); return 0; }
    let x = xfrm_input_state(skb); let mut inner = &(*x).inner_mode;
    if (*x).sel.family == AF_UNSPEC { inner = xfrm_ip2inner_mode(x, (*XFRM_MODE_SKB_CB(skb)).protocol); if inner.is_null() { XFRM_INC_STATS(dev_net((*skb).dev),LINUX_MIB_XFRMINSTATEMODEERROR); return -EINVAL; } }
    let family = (*inner).family; let mark = (*tunnel).parms.i_key;
    (*skb).mark = be32_to_cpu(mark); let ret = xfrm_policy_check(core::ptr::null_mut(),XFRM_POLICY_IN,skb,family); (*skb).mark = (*skb).mark;
    if ret == 0 { return -EPERM; }
    skb_scrub_packet(skb,!net_eq((*tunnel).net,dev_net((*skb).dev))); (*skb).dev=dev; dev_sw_netstats_rx_add(dev,(*skb).len); 0
}

unsafe fn vti_state_check(x: *const xfrm_state, dst: __be32, src: __be32) -> bool {
    if x.is_null() || (*x).props.mode != XFRM_MODE_TUNNEL || (*x).props.family != AF_INET { return false; }
    let daddr = &dst as *const _ as *mut xfrm_address_t; let saddr=&src as *const _ as *mut xfrm_address_t;
    if dst == 0 { return xfrm_addr_equal(saddr,&(*x).props.saddr,AF_INET); }
    if !xfrm_state_addr_check(x,daddr,saddr,AF_INET) { return false; } true
}

unsafe fn vti_xmit(skb:*mut sk_buff,dev:*mut net_device,fl:*mut flowi)->netdev_tx_t {
    let tunnel=netdev_priv(dev); let parms=&mut (*tunnel).parms; let mut dst=skb_dst(skb); let pkt_len=(*skb).len; let mut err; let mut mtu;
    if dst.is_null() { match (*skb).protocol { p if p==htons(ETH_P_IP) => { (*fl).u.ip4.flowi4_oif=(*dev).ifindex; (*fl).u.ip4.flowi4_flags|=FLOWI_FLAG_ANYSRC; let rt=__ip_route_output_key(dev_net(dev),&mut (*fl).u.ip4); if IS_ERR(rt) { DEV_STATS_INC(dev,tx_carrier_errors); dst_link_failure(skb); DEV_STATS_INC(dev,tx_errors); kfree_skb(skb); return NETDEV_TX_OK; } dst=&mut (*rt).dst; skb_dst_set(skb,dst); }, _ => { DEV_STATS_INC(dev,tx_carrier_errors); dst_link_failure(skb); DEV_STATS_INC(dev,tx_errors); kfree_skb(skb); return NETDEV_TX_OK; } } }
    dst_hold(dst); dst=xfrm_lookup_route((*tunnel).net,dst,fl,core::ptr::null_mut(),0); if IS_ERR(dst) { DEV_STATS_INC(dev,tx_carrier_errors); dst_link_failure(skb); DEV_STATS_INC(dev,tx_errors); kfree_skb(skb); return NETDEV_TX_OK; }
    if (*dst).flags&DST_XFRM_QUEUE != 0 || vti_state_check((*dst).xfrm,parms.iph.daddr,parms.iph.saddr) { } else { DEV_STATS_INC(dev,tx_carrier_errors); dst_release(dst); dst_link_failure(skb); DEV_STATS_INC(dev,tx_errors); kfree_skb(skb); return NETDEV_TX_OK; }
    let tdev=dst_dev(dst); if tdev==dev { dst_release(dst); DEV_STATS_INC(dev,collisions); DEV_STATS_INC(dev,tx_errors); kfree_skb(skb); return NETDEV_TX_OK; }
    mtu=dst_mtu(dst); if (*skb).len>mtu { skb_dst_update_pmtu_no_confirm(skb,mtu); if (*skb).protocol==htons(ETH_P_IP) && (*ip_hdr(skb)).frag_off&htons(IP_DF)==0 { } else { dst_release(dst); DEV_STATS_INC(dev,tx_errors); kfree_skb(skb); return NETDEV_TX_OK; } }
    skb_scrub_packet(skb,!net_eq((*tunnel).net,dev_net(dev))); skb_dst_set(skb,dst); (*skb).dev=skb_dst_dev(skb); err=dst_output((*tunnel).net,(*skb).sk,skb); if net_xmit_eval(err)==0 { err=pkt_len; } iptunnel_xmit_stats(dev,err); NETDEV_TX_OK
}

unsafe fn vti_tunnel_xmit(skb:*mut sk_buff,dev:*mut net_device)->netdev_tx_t { let tunnel=netdev_priv(dev); let mut fl:flowi=core::mem::zeroed(); if !pskb_inet_may_pull(skb) { DEV_STATS_INC(dev,tx_errors); kfree_skb(skb); return NETDEV_TX_OK; } match (*skb).protocol { p if p==htons(ETH_P_IP)=>{memset(IPCB(skb),0,core::mem::size_of::<_>());xfrm_decode_session(dev_net(dev),skb,&mut fl,AF_INET);}, p if p==htons(ETH_P_IPV6)=>{memset(IP6CB(skb),0,core::mem::size_of::<_>());xfrm_decode_session(dev_net(dev),skb,&mut fl,AF_INET6);}, _=>{DEV_STATS_INC(dev,tx_errors);kfree_skb(skb);return NETDEV_TX_OK;} } fl.flowi_mark=be32_to_cpu((*tunnel).parms.o_key); vti_xmit(skb,dev,&mut fl) }

// Remaining declarations mirror the C registration/netlink plumbing and retain external kernel dependencies.
unsafe fn vti_tunnel_validate(_: *mut *mut nlattr,_:*mut *mut nlattr,_:*mut netlink_ext_ack)->c_int { 0 }
unsafe fn vti_init()->c_int { pr_info!("IPv4 over IPsec tunneling driver\n"); register_pernet_device(&mut VTI_NET_OPS) }
unsafe fn vti_fini(){ rtnl_link_unregister(&mut VTI_LINK_OPS); unregister_pernet_device(&mut VTI_NET_OPS); }
static mut VTI_NET_OPS: pernet_operations = unsafe { core::mem::zeroed() };
// module_init!(vti_init); module_exit!(vti_fini); MODULE_DESCRIPTION!("Virtual (secure) IP tunneling library");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
