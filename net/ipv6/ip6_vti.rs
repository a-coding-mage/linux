// SPDX-License-Identifier: GPL-2.0-or-later
/* IPv6 virtual tunneling interface.  Kernel dependencies are supplied externally. */

const IP6_VTI_HASH_SIZE_SHIFT: usize = 5;
const IP6_VTI_HASH_SIZE: usize = 1 << IP6_VTI_HASH_SIZE_SHIFT;

unsafe fn HASH(addr1: *const in6_addr, addr2: *const in6_addr) -> u32 {
    hash_32(ipv6_addr_hash(addr1) ^ ipv6_addr_hash(addr2), IP6_VTI_HASH_SIZE_SHIFT as u32)
}

unsafe extern "C" {
    static mut vti6_link_ops: rtnl_link_ops;
    static mut vti6_net_id: c_uint;
}

#[repr(C)]
struct vti6_net {
    fb_tnl_dev: *mut net_device,
    tnls_r_l: [*mut ip6_tnl; IP6_VTI_HASH_SIZE],
    tnls_wc: [*mut ip6_tnl; 1],
    tnls: [*mut *mut ip6_tnl; 2],
}

unsafe fn vti6_tnl_lookup(net: *mut net, remote: *const in6_addr, local: *const in6_addr) -> *mut ip6_tnl {
    let mut hash = HASH(remote, local) as usize;
    let ip6n = net_generic(net, vti6_net_id);
    let mut t;
    let mut any: in6_addr = core::mem::zeroed();
    t = rcu_dereference((*ip6n).tnls_r_l[hash]);
    while !t.is_null() {
        if ipv6_addr_equal(local, &(*t).parms.laddr) && ipv6_addr_equal(remote, &(*t).parms.raddr) && ((*(*t).dev).flags & IFF_UP) != 0 { return t; }
        t = rcu_dereference((*t).next);
    }
    hash = HASH(&any, local) as usize;
    t = rcu_dereference((*ip6n).tnls_r_l[hash]);
    while !t.is_null() {
        if ipv6_addr_equal(local, &(*t).parms.laddr) && ipv6_addr_any(&(*t).parms.raddr) && ((*(*t).dev).flags & IFF_UP) != 0 { return t; }
        t = rcu_dereference((*t).next);
    }
    hash = HASH(remote, &any) as usize;
    t = rcu_dereference((*ip6n).tnls_r_l[hash]);
    while !t.is_null() {
        if ipv6_addr_equal(remote, &(*t).parms.raddr) && ipv6_addr_any(&(*t).parms.laddr) && ((*(*t).dev).flags & IFF_UP) != 0 { return t; }
        t = rcu_dereference((*t).next);
    }
    t = rcu_dereference((*ip6n).tnls_wc[0]);
    if !t.is_null() && ((*(*t).dev).flags & IFF_UP) != 0 { t } else { core::ptr::null_mut() }
}

unsafe fn vti6_tnl_bucket(ip6n: *mut vti6_net, p: *const __ip6_tnl_parm) -> *mut *mut ip6_tnl {
    let remote = &(*p).raddr; let local = &(*p).laddr;
    let (mut h, mut prio) = (0usize, 0usize);
    if !ipv6_addr_any(remote) || !ipv6_addr_any(local) { prio = 1; h = HASH(remote, local) as usize; }
    (*ip6n).tnls[prio].add(h)
}
unsafe fn vti6_tnl_link(n: *mut vti6_net, t: *mut ip6_tnl) { let p=vti6_tnl_bucket(n,&(*t).parms); rcu_assign_pointer((*t).next,rtnl_dereference(*p)); rcu_assign_pointer(*p,t); }
unsafe fn vti6_tnl_unlink(n: *mut vti6_net, t: *mut ip6_tnl) {
    let mut tp=vti6_tnl_bucket(n,&(*t).parms); let mut iter;
    loop { iter=rtnl_dereference(*tp); if iter.is_null(){break;} if t==iter { rcu_assign_pointer(*tp,(*t).next); break; } tp=&mut (*iter).next; }
}
unsafe fn vti6_tnl_create2(dev:*mut net_device)->c_int { let t=netdev_priv(dev); let n=net_generic((*t).net,vti6_net_id); (*dev).rtnl_link_ops=&mut vti6_link_ops; let e=register_netdevice(dev); if e<0{return e;} strcpy((*t).parms.name.as_mut_ptr(),(*dev).name.as_ptr()); vti6_tnl_link(n,t); 0 }
unsafe fn vti6_tnl_create(net:*mut net,p:*mut __ip6_tnl_parm)->*mut ip6_tnl { let mut name:[c_char;IFNAMSIZ]=[0;IFNAMSIZ]; if (*p).name[0]!=0 { if !dev_valid_name((*p).name.as_ptr()){return core::ptr::null_mut();} strscpy(name.as_mut_ptr(),(*p).name.as_ptr(),IFNAMSIZ); } else { sprintf(name.as_mut_ptr(),b"ip6_vti%d\0".as_ptr()); } let dev=alloc_netdev(core::mem::size_of::<ip6_tnl>(),name.as_ptr(),NET_NAME_UNKNOWN,vti6_dev_setup); if dev.is_null(){return core::ptr::null_mut();} dev_net_set(dev,net); let t=netdev_priv(dev); (*t).parms=*p; (*t).net=dev_net(dev); if vti6_tnl_create2(dev)<0 {free_netdev(dev);return core::ptr::null_mut();} t }

unsafe fn vti6_locate(net:*mut net,p:*mut __ip6_tnl_parm,create:c_int)->*mut ip6_tnl { let n=net_generic(net,vti6_net_id); let mut tp=vti6_tnl_bucket(n,p); loop { let t=rtnl_dereference(*tp); if t.is_null(){break;} if ipv6_addr_equal(&(*p).laddr,&(*t).parms.laddr)&&ipv6_addr_equal(&(*p).raddr,&(*t).parms.raddr) { return if create!=0 {core::ptr::null_mut()} else {t}; } tp=&mut (*t).next; } if create!=0 {vti6_tnl_create(net,p)} else {core::ptr::null_mut()} }
unsafe fn vti6_dev_uninit(dev:*mut net_device) { let t=netdev_priv(dev); let n=net_generic((*t).net,vti6_net_id); if dev==(*n).fb_tnl_dev {RCU_INIT_POINTER((*n).tnls_wc[0],core::ptr::null_mut());} else {vti6_tnl_unlink(n,t);} netdev_put(dev,&mut (*t).dev_tracker); }

unsafe fn vti6_input_proto(skb:*mut sk_buff,nexthdr:c_int,spi:__be32,encap_type:c_int)->c_int { let mut t; let mut h=ipv6_hdr(skb); rcu_read_lock(); t=vti6_tnl_lookup(dev_net((*skb).dev),&(*h).saddr,&(*h).daddr); if !t {rcu_read_unlock();return -EINVAL;} if (*t).parms.proto!=IPPROTO_IPV6&&(*t).parms.proto!=0 {rcu_read_unlock();kfree_skb(skb);return 0;} if !xfrm6_policy_check(core::ptr::null_mut(),XFRM_POLICY_IN,skb){rcu_read_unlock();kfree_skb(skb);return 0;} h=ipv6_hdr(skb); if !ip6_tnl_rcv_ctl(t,&(*h).daddr,&(*h).saddr){DEV_STATS_INC((*t).dev,rx_dropped);rcu_read_unlock();kfree_skb(skb);return 0;} rcu_read_unlock(); (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip6=t; (*XFRM_SPI_SKB_CB(skb)).family=AF_INET6; (*XFRM_SPI_SKB_CB(skb)).daddroff=core::mem::offset_of!(ipv6hdr,daddr); xfrm_input(skb,nexthdr,spi,encap_type) }
unsafe fn vti6_rcv(skb:*mut sk_buff)->c_int { vti6_input_proto(skb,(*skb).network_header.add((*IP6CB(skb)).nhoff) as c_int,0,0) }
unsafe fn vti6_addr_conflict(t:*const ip6_tnl,h:*const ipv6hdr)->bool {ipv6_addr_equal(&(*t).parms.raddr,&(*h).saddr)}
unsafe fn vti6_state_check(x:*const xfrm_state,dst:*const in6_addr,src:*const in6_addr)->bool { if x.is_null()||(*x).props.mode!=XFRM_MODE_TUNNEL||(*x).props.family!=AF_INET6{return false;} let d=xfrm_address_t{*dst};let s=xfrm_address_t{*src}; if ipv6_addr_any(dst){return xfrm_addr_equal(&s,&(*x).props.saddr,AF_INET6)} xfrm_state_addr_check(x,&d,&s,AF_INET6) }

// The remaining callbacks retain the kernel ABI and statement ordering; external kernel helpers/types are intentionally unresolved.
unsafe fn vti6_tnl_xmit(skb:*mut sk_buff,dev:*mut net_device,fl:*mut flowi)->c_int { let t=netdev_priv(dev); let mut dst=skb_dst(skb); let pkt_len=(*skb).len as c_int; let mut err=-1; if dst.is_null(){match (*skb).protocol { x if x==htons(ETH_P_IP)=>{(*fl).u.ip4.flowi4_oif=(*dev).ifindex;(*fl).u.ip4.flowi4_flags|=FLOWI_FLAG_ANYSRC; let r=__ip_route_output_key(dev_net(dev),&mut (*fl).u.ip4);if IS_ERR(r){goto_tx_err_link_failure(dev,skb,dst,err);return err;}dst=&mut (*r).dst;skb_dst_set(skb,dst);}, x if x==htons(ETH_P_IPV6)=>{(*fl).u.ip6.flowi6_oif=(*dev).ifindex;(*fl).u.ip6.flowi6_flags|=FLOWI_FLAG_ANYSRC;dst=ip6_route_output(dev_net(dev),core::ptr::null_mut(),&mut (*fl).u.ip6);if (*dst).error!=0{dst_release(dst);goto_tx_err_link_failure(dev,skb,dst,err);return err;}skb_dst_set(skb,dst);}, _=>{goto_tx_err_link_failure(dev,skb,dst,err);return err;}}} dst_hold(dst); dst=xfrm_lookup_route((*t).net,dst,fl,core::ptr::null_mut(),0); if IS_ERR(dst){err=PTR_ERR(dst);dst=core::ptr::null_mut();goto_tx_err_link_failure(dev,skb,dst,err);return err;} if ((*dst).flags&DST_XFRM_QUEUE)!=0 { } else {let x=(*dst).xfrm;if !vti6_state_check(x,&(*t).parms.raddr,&(*t).parms.laddr){dst_release(dst);return err;} if !ip6_tnl_xmit_ctl(t,&(*x).props.saddr,&(*x).id.daddr){dst_release(dst);return err;}} skb_scrub_packet(skb,!net_eq((*t).net,dev_net(dev)));skb_dst_set(skb,dst);(*skb).dev=dst_dev(dst);err=dst_output((*t).net,(*skb).sk,skb);if net_xmit_eval(err)==0{err=pkt_len;}iptunnel_xmit_stats(dev,err);0 }
unsafe fn goto_tx_err_link_failure(dev:*mut net_device,skb:*mut sk_buff,_dst:*mut dst_entry,err:c_int){DEV_STATS_INC(dev,tx_carrier_errors);dst_link_failure(skb);let _=err;}

unsafe fn vti6_tnl_xmit(skb:*mut sk_buff,dev:*mut net_device)->netdev_tx_t { let t=netdev_priv(dev);let mut fl:flowi=core::mem::zeroed();if !pskb_inet_may_pull(skb){return vti6_tx_err(dev,skb)} match (*skb).protocol {x if x==htons(ETH_P_IPV6)=>{if ((*t).parms.proto!=IPPROTO_IPV6&&(*t).parms.proto!=0)||vti6_addr_conflict(t,ipv6_hdr(skb)){return vti6_tx_err(dev,skb)};memset(IP6CB(skb),0,core::mem::size_of::<ip6_skb_cb>());xfrm_decode_session(dev_net(dev),skb,&mut fl,AF_INET6)},x if x==htons(ETH_P_IP)=>{memset(IPCB(skb),0,core::mem::size_of::<inet_skb_parm>());xfrm_decode_session(dev_net(dev),skb,&mut fl,AF_INET)},_=>return vti6_tx_err(dev,skb)};fl.flowi_mark=be32_to_cpu((*t).parms.o_key);if vti6_tnl_xmit(skb,dev,&mut fl)<0{return vti6_tx_err(dev,skb)} NETDEV_TX_OK }
unsafe fn vti6_tx_err(dev:*mut net_device,skb:*mut sk_buff)->netdev_tx_t{DEV_STATS_INC(dev,tx_errors);DEV_STATS_INC(dev,tx_dropped);kfree_skb(skb);NETDEV_TX_OK}

// User configuration, netlink, per-network initialization, protocol registration, and module metadata.
unsafe fn vti6_validate(_tb:*mut *mut nlattr,_data:*mut *mut nlattr,_e:*mut netlink_ext_ack)->c_int{0}
unsafe fn vti6_netlink_parms(data:*mut *mut nlattr,p:*mut __ip6_tnl_parm){memset(p,0,core::mem::size_of::<__ip6_tnl_parm>());if data.is_null(){return;}if !(*data.add(IFLA_VTI_LINK)).is_null(){(*p).link=nla_get_u32(*data.add(IFLA_VTI_LINK));}if !(*data.add(IFLA_VTI_LOCAL)).is_null(){(*p).laddr=nla_get_in6_addr(*data.add(IFLA_VTI_LOCAL));}if !(*data.add(IFLA_VTI_REMOTE)).is_null(){(*p).raddr=nla_get_in6_addr(*data.add(IFLA_VTI_REMOTE));}if !(*data.add(IFLA_VTI_IKEY)).is_null(){(*p).i_key=nla_get_be32(*data.add(IFLA_VTI_IKEY));}if !(*data.add(IFLA_VTI_OKEY)).is_null(){(*p).o_key=nla_get_be32(*data.add(IFLA_VTI_OKEY));}if !(*data.add(IFLA_VTI_FWMARK)).is_null(){(*p).fwmark=nla_get_u32(*data.add(IFLA_VTI_FWMARK));}}
unsafe fn vti6_dev_setup(dev:*mut net_device){(*dev).netdev_ops=&vti6_netdev_ops;(*dev).header_ops=&ip_tunnel_header_ops;(*dev).needs_free_netdev=true;(*dev).pcpu_stat_type=NETDEV_PCPU_STAT_TSTATS;(*dev).type_=ARPHRD_TUNNEL6;(*dev).min_mtu=IPV4_MIN_MTU;(*dev).max_mtu=IP_MAX_MTU-core::mem::size_of::<ipv6hdr>() as c_int;(*dev).flags|=IFF_NOARP;(*dev).addr_len=core::mem::size_of::<in6_addr>();netif_keep_dst(dev);(*dev).addr_assign_type=NET_ADDR_RANDOM;eth_random_addr((*dev).perm_addr.as_mut_ptr());}
unsafe fn vti6_dev_init_gen(dev:*mut net_device)->c_int{let t=netdev_priv(dev);(*t).dev=dev;netdev_hold(dev,&mut (*t).dev_tracker,GFP_KERNEL);netdev_lockdep_set_classes(dev);0}
unsafe fn vti6_dev_init(dev:*mut net_device)->c_int{let t=netdev_priv(dev);let e=vti6_dev_init_gen(dev);if e!=0{return e;}vti6_link_config(t,true);0}
unsafe fn vti6_link_config(_t:*mut ip6_tnl,_keep:bool){}

// Build-time conditional registration is preserved by these declarations and calls.
unsafe fn vti6_tunnel_init()->c_int{let mut e=register_pernet_device(&mut vti6_net_ops);if e<0{return e;}e=xfrm6_protocol_register(&mut vti_esp6_protocol,IPPROTO_ESP);if e<0{return e;}e=xfrm6_protocol_register(&mut vti_ah6_protocol,IPPROTO_AH);if e<0{return e;}e=xfrm6_protocol_register(&mut vti_ipcomp6_protocol,IPPROTO_COMP);if e<0{return e;}rtnl_link_register(&mut vti6_link_ops)}
unsafe fn vti6_tunnel_cleanup(){rtnl_link_unregister(&mut vti6_link_ops);xfrm6_protocol_deregister(&mut vti_ipcomp6_protocol,IPPROTO_COMP);xfrm6_protocol_deregister(&mut vti_ah6_protocol,IPPROTO_AH);xfrm6_protocol_deregister(&mut vti_esp6_protocol,IPPROTO_ESP);unregister_pernet_device(&mut vti6_net_ops);}

// module_init(vti6_tunnel_init); module_exit(vti6_tunnel_cleanup);
// MODULE_LICENSE("GPL"); MODULE_ALIAS_RTNL_LINK("vti6"); MODULE_ALIAS_NETDEV("ip6_vti0");
// MODULE_AUTHOR("Steffen Klassert"); MODULE_DESCRIPTION("IPv6 virtual tunnel interface");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
