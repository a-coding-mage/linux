// SPDX-License-Identifier: GPL-2.0
/* Copyright 2011-2014 Autronica Fire and Security AS
 *
 * Author(s):
 *	2011-2014 Arvid Brodin, arvid.brodin@alten.se
 *
 * Routines for handling Netlink messages for HSR and PRP.
 */

// Dependencies supplied by the kernel and the other HSR translation units.

static HSR_POLICY: [nla_policy; IFLA_HSR_MAX as usize + 1] = [
    nla_policy { type_: NLA_U32, ..nla_policy::default() },
    nla_policy { type_: NLA_U32, ..nla_policy::default() },
    nla_policy { type_: NLA_U8, ..nla_policy::default() },
    nla_policy { type_: NLA_U8, ..nla_policy::default() },
    nla_policy { len: ETH_ALEN, ..nla_policy::default() },
    nla_policy { type_: NLA_U16, ..nla_policy::default() },
    nla_policy { type_: NLA_U8, ..nla_policy::default() },
    nla_policy { type_: NLA_U32, ..nla_policy::default() },
];

/* Here, it seems a netdevice has already been allocated for us, and the
 * hsr_dev_setup routine has been executed. Nice!
 */
unsafe fn hsr_newlink(
    dev: *mut net_device,
    params: *mut rtnl_newlink_params,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let link_net = rtnl_newlink_link_net(params);
    let mut link: [*mut net_device; 2] = [core::ptr::null_mut(); 2];
    let mut interlink: *mut net_device = core::ptr::null_mut();
    let data = (*params).data;
    let mut proto_version: hsr_version;
    let multicast_spec: u8;
    let mut proto: u8 = HSR_PROTOCOL_HSR;

    if !net_eq(link_net, dev_net(dev)) {
        NL_SET_ERR_MSG_MOD(extack, "HSR slaves/interlink must be on the same net namespace than HSR link");
        return -EINVAL;
    }
    if data.is_null() { NL_SET_ERR_MSG_MOD(extack, "No slave devices specified"); return -EINVAL; }
    if (*data.add(IFLA_HSR_SLAVE1 as usize)).is_null() { NL_SET_ERR_MSG_MOD(extack, "Slave1 device not specified"); return -EINVAL; }
    link[0] = __dev_get_by_index(link_net, nla_get_u32(*data.add(IFLA_HSR_SLAVE1 as usize)));
    if link[0].is_null() { NL_SET_ERR_MSG_MOD(extack, "Slave1 does not exist"); return -EINVAL; }
    if (*data.add(IFLA_HSR_SLAVE2 as usize)).is_null() { NL_SET_ERR_MSG_MOD(extack, "Slave2 device not specified"); return -EINVAL; }
    link[1] = __dev_get_by_index(link_net, nla_get_u32(*data.add(IFLA_HSR_SLAVE2 as usize)));
    if link[1].is_null() { NL_SET_ERR_MSG_MOD(extack, "Slave2 does not exist"); return -EINVAL; }
    if link[0] == link[1] { NL_SET_ERR_MSG_MOD(extack, "Slave1 and Slave2 are same"); return -EINVAL; }
    if !(*data.add(IFLA_HSR_INTERLINK as usize)).is_null() {
        interlink = __dev_get_by_index(link_net, nla_get_u32(*data.add(IFLA_HSR_INTERLINK as usize)));
        if interlink.is_null() { NL_SET_ERR_MSG_MOD(extack, "Interlink does not exist"); return -EINVAL; }
    }
    if !interlink.is_null() && interlink == link[0] { NL_SET_ERR_MSG_MOD(extack, "Interlink and Slave1 are the same"); return -EINVAL; }
    if !interlink.is_null() && interlink == link[1] { NL_SET_ERR_MSG_MOD(extack, "Interlink and Slave2 are the same"); return -EINVAL; }
    multicast_spec = nla_get_u8_default(*data.add(IFLA_HSR_MULTICAST_SPEC as usize), 0);
    if !(*data.add(IFLA_HSR_PROTOCOL as usize)).is_null() { proto = nla_get_u8(*data.add(IFLA_HSR_PROTOCOL as usize)); }
    if proto >= HSR_PROTOCOL_MAX { NL_SET_ERR_MSG_MOD(extack, "Unsupported protocol"); return -EINVAL; }
    if (*data.add(IFLA_HSR_VERSION as usize)).is_null() {
        proto_version = HSR_V0;
    } else {
        if proto == HSR_PROTOCOL_PRP { NL_SET_ERR_MSG_MOD(extack, "PRP version unsupported"); return -EINVAL; }
        proto_version = nla_get_u8(*data.add(IFLA_HSR_VERSION as usize)) as hsr_version;
        if proto_version > HSR_V1 { NL_SET_ERR_MSG_MOD(extack, "Only HSR version 0/1 supported"); return -EINVAL; }
    }
    if proto == HSR_PROTOCOL_PRP { proto_version = PRP_V1; }
    hsr_dev_finalize(dev, link.as_mut_ptr(), interlink, multicast_spec, proto_version, extack)
}

unsafe fn hsr_dellink(dev: *mut net_device, head: *mut list_head) {
    let hsr = netdev_priv(dev);
    timer_delete_sync(&mut (*hsr).prune_timer); timer_delete_sync(&mut (*hsr).prune_proxy_timer);
    timer_delete_sync(&mut (*hsr).announce_timer); timer_delete_sync(&mut (*hsr).announce_proxy_timer);
    hsr_debugfs_term(hsr); hsr_del_ports(hsr); hsr_del_self_node(hsr);
    hsr_del_nodes(&mut (*hsr).node_db); hsr_del_nodes(&mut (*hsr).proxy_node_db);
    unregister_netdevice_queue(dev, head);
}

unsafe fn hsr_fill_info(skb: *mut sk_buff, dev: *const net_device) -> c_int {
    let hsr = netdev_priv(dev as *mut net_device); let mut proto = HSR_PROTOCOL_HSR;
    let mut port = hsr_port_get_hsr(hsr, HSR_PT_SLAVE_A);
    if !port.is_null() && nla_put_u32(skb, IFLA_HSR_SLAVE1, (*(*port).dev).ifindex) != 0 { return -EMSGSIZE; }
    port = hsr_port_get_hsr(hsr, HSR_PT_SLAVE_B);
    if !port.is_null() && nla_put_u32(skb, IFLA_HSR_SLAVE2, (*(*port).dev).ifindex) != 0 { return -EMSGSIZE; }
    port = hsr_port_get_hsr(hsr, HSR_PT_INTERLINK);
    if !port.is_null() && nla_put_u32(skb, IFLA_HSR_INTERLINK, (*(*port).dev).ifindex) != 0 { return -EMSGSIZE; }
    if nla_put(skb, IFLA_HSR_SUPERVISION_ADDR, ETH_ALEN, (*hsr).sup_multicast_addr.as_ptr() as *const c_void) != 0 || nla_put_u16(skb, IFLA_HSR_SEQ_NR, (*hsr).sequence_nr) != 0 { return -EMSGSIZE; }
    if (*hsr).prot_version == PRP_V1 { proto = HSR_PROTOCOL_PRP; } else if nla_put_u8(skb, IFLA_HSR_VERSION, (*hsr).prot_version as u8) != 0 { return -EMSGSIZE; }
    if nla_put_u8(skb, IFLA_HSR_PROTOCOL, proto) != 0 { return -EMSGSIZE; } 0
}

static HSR_GENL_POLICY: [nla_policy; HSR_A_MAX as usize + 1] = [
    nla_policy { len: ETH_ALEN, ..nla_policy::default() }, nla_policy { len: ETH_ALEN, ..nla_policy::default() },
    nla_policy { type_: NLA_U32, ..nla_policy::default() }, nla_policy { type_: NLA_U32, ..nla_policy::default() },
    nla_policy { type_: NLA_U32, ..nla_policy::default() }, nla_policy { type_: NLA_U16, ..nla_policy::default() },
    nla_policy { type_: NLA_U16, ..nla_policy::default() },
];
static HSR_LINK_OPS: rtnl_link_ops = rtnl_link_ops {
    kind: "hsr\0".as_ptr() as *const c_char,
    maxtype: IFLA_HSR_MAX,
    policy: HSR_POLICY.as_ptr(),
    priv_size: core::mem::size_of::<hsr_priv>(),
    setup: Some(hsr_dev_setup), newlink: Some(hsr_newlink), dellink: Some(hsr_dellink),
    fill_info: Some(hsr_fill_info), ..rtnl_link_ops::default()
};
static mut HSR_GENL_FAMILY: genl_family = genl_family::default();
static HSR_MCGRPS: [genl_multicast_group; 1] = [genl_multicast_group { name: "hsr-network\0".as_ptr() as *const c_char }];

/* This is called if for some node with MAC address addr, we only get frames
 * over one of the slave interfaces. This would indicate an open network ring
 * (i.e. a link has failed somewhere).
 */
unsafe fn hsr_nl_ringerror(hsr: *mut hsr_priv, addr: *mut u8, port: *mut hsr_port) {
    let skb = genlmsg_new(NLMSG_GOODSIZE, GFP_ATOMIC); if skb.is_null() { return; }
    let msg_head = genlmsg_put(skb, 0, 0, &HSR_GENL_FAMILY, 0, HSR_C_RING_ERROR); if msg_head.is_null() { kfree_skb(skb); return; }
    if nla_put(skb, HSR_A_NODE_ADDR, ETH_ALEN, addr as *const c_void) < 0 || nla_put_u32(skb, HSR_A_IFINDEX, (*(*port).dev).ifindex) < 0 { kfree_skb(skb); let master = hsr_port_get_hsr(hsr, HSR_PT_MASTER); netdev_warn((*master).dev, "Could not send HSR ring error message\n"); return; }
    genlmsg_end(skb, msg_head); genlmsg_multicast_netns(&HSR_GENL_FAMILY, dev_net((*port).dev), skb, 0, 0, GFP_ATOMIC);
}

/* This is called when we haven't heard from the node with MAC address addr for
 * some time (just before the node is removed from the node table/list).
 */
unsafe fn hsr_nl_nodedown(hsr: *mut hsr_priv, addr: *mut u8) {
    let skb = genlmsg_new(NLMSG_GOODSIZE, GFP_ATOMIC); if skb.is_null() { let m=hsr_port_get_hsr(hsr,HSR_PT_MASTER); netdev_warn((*m).dev,"Could not send HSR node down\n"); return; }
    let head=genlmsg_put(skb,0,0,&HSR_GENL_FAMILY,0,HSR_C_NODE_DOWN); if head.is_null() { kfree_skb(skb); return; }
    if nla_put(skb,HSR_A_NODE_ADDR,ETH_ALEN,addr as *const c_void)<0 { kfree_skb(skb); return; }
    let master=hsr_port_get_hsr(hsr,HSR_PT_MASTER); genlmsg_end(skb,head); genlmsg_multicast_netns(&HSR_GENL_FAMILY,dev_net((*master).dev),skb,0,0,GFP_ATOMIC);
}

/* HSR_C_GET_NODE_STATUS lets userspace query the internal HSR node table. */
unsafe fn hsr_get_node_status(skb_in: *mut sk_buff, info: *mut genl_info) -> c_int {
    if info.is_null() || (*info).attrs[HSR_A_IFINDEX as usize].is_null() || (*info).attrs[HSR_A_NODE_ADDR as usize].is_null() { netlink_ack(skb_in,nlmsg_hdr(skb_in),-EINVAL,core::ptr::null_mut()); return 0; }
    rcu_read_lock();
    let dev=dev_get_by_index_rcu(genl_info_net(info),nla_get_u32((*info).attrs[HSR_A_IFINDEX as usize]));
    if dev.is_null() || !is_hsr_master(dev) { rcu_read_unlock(); netlink_ack(skb_in,nlmsg_hdr(skb_in),-EINVAL,core::ptr::null_mut()); return 0; }
    let out=genlmsg_new(NLMSG_GOODSIZE,GFP_ATOMIC); if out.is_null(){rcu_read_unlock();return -ENOMEM;}
    let head=genlmsg_put(out,NETLINK_CB(skb_in).portid,(*info).snd_seq,&HSR_GENL_FAMILY,0,HSR_C_SET_NODE_STATUS); if head.is_null(){kfree_skb(out);rcu_read_unlock();return -ENOMEM;}
    let hsr=netdev_priv(dev); let mut b=[0u8;ETH_ALEN as usize]; let mut bi=0; let mut a1=0; let mut s1=0u16; let mut a2=0; let mut s2=0u16;
    let mut res=nla_put_u32(out,HSR_A_IFINDEX,(*dev).ifindex);
    if res>=0 {res=hsr_get_node_data(hsr,nla_data((*info).attrs[HSR_A_NODE_ADDR]) as *mut u8,b.as_mut_ptr(),&mut bi,&mut a1,&mut s1,&mut a2,&mut s2);}
    if res>=0 {res=nla_put(out,HSR_A_NODE_ADDR,ETH_ALEN,nla_data((*info).attrs[HSR_A_NODE_ADDR]));}
    if res>=0 && bi>-1 {res=nla_put(out,HSR_A_NODE_ADDR_B,ETH_ALEN,b.as_ptr() as *const c_void); if res>=0 {res=nla_put_u32(out,HSR_A_ADDR_B_IFINDEX,bi);}}
    if res>=0 {res=nla_put_u32(out,HSR_A_IF1_AGE,a1);}
    if res>=0 {res=nla_put_u16(out,HSR_A_IF1_SEQ,s1);}
    let p=hsr_port_get_hsr(hsr,HSR_PT_SLAVE_A); if res>=0 && !p.is_null(){res=nla_put_u32(out,HSR_A_IF1_IFINDEX,(*(*p).dev).ifindex);}
    if res>=0 {res=nla_put_u32(out,HSR_A_IF2_AGE,a2);}
    if res>=0 {res=nla_put_u16(out,HSR_A_IF2_SEQ,s2);}
    let p=hsr_port_get_hsr(hsr,HSR_PT_SLAVE_B); if res>=0 && !p.is_null(){res=nla_put_u32(out,HSR_A_IF2_IFINDEX,(*(*p).dev).ifindex);}
    if res<0 {kfree_skb(out);rcu_read_unlock();return res;} rcu_read_unlock(); genlmsg_end(out,head); genlmsg_unicast(genl_info_net(info),out,(*info).snd_portid); 0
}
/* Get a list of MacAddressA of all nodes known to this node (including self). */
unsafe fn hsr_get_node_list(skb_in: *mut sk_buff, info: *mut genl_info) -> c_int {
    if info.is_null() || (*info).attrs[HSR_A_IFINDEX as usize].is_null(){netlink_ack(skb_in,nlmsg_hdr(skb_in),-EINVAL,core::ptr::null_mut());return 0;}
    rcu_read_lock(); let dev=dev_get_by_index_rcu(genl_info_net(info),nla_get_u32((*info).attrs[HSR_A_IFINDEX as usize])); if dev.is_null()||!is_hsr_master(dev){rcu_read_unlock();return 0;}
    let hsr=netdev_priv(dev); let mut pos: *mut c_void=core::ptr::null_mut(); let mut addr=[0u8;ETH_ALEN as usize]; let mut first=true;
    loop { let out=genlmsg_new(GENLMSG_DEFAULT_SIZE,GFP_ATOMIC); if out.is_null(){rcu_read_unlock();return -ENOMEM;} let head=genlmsg_put(out,NETLINK_CB(skb_in).portid,(*info).snd_seq,&HSR_GENL_FAMILY,0,HSR_C_SET_NODE_LIST); if head.is_null(){nlmsg_free(out);rcu_read_unlock();return -ENOMEM;}
        if first {if nla_put_u32(out,HSR_A_IFINDEX,(*dev).ifindex)<0{nlmsg_free(out);rcu_read_unlock();return -EMSGSIZE;} first=false;} pos=hsr_get_next_node(hsr,pos,addr.as_mut_ptr()); if pos.is_null(){genlmsg_end(out,head);genlmsg_unicast(genl_info_net(info),out,(*info).snd_portid);break;} if nla_put(out,HSR_A_NODE_ADDR,ETH_ALEN,addr.as_ptr() as *const c_void)<0{genlmsg_end(out,head);genlmsg_unicast(genl_info_net(info),out,(*info).snd_portid);continue;} genlmsg_end(out,head);genlmsg_unicast(genl_info_net(info),out,(*info).snd_portid); }
    rcu_read_unlock();0
}

static HSR_OPS: [genl_small_ops; 2] = [
    genl_small_ops { cmd: HSR_C_GET_NODE_STATUS, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: 0, doit: Some(hsr_get_node_status), dumpit: None },
    genl_small_ops { cmd: HSR_C_GET_NODE_LIST, validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP, flags: 0, doit: Some(hsr_get_node_list), dumpit: None },
];

#[no_mangle]
pub unsafe extern "C" fn hsr_netlink_init() -> c_int {
    let mut rc = rtnl_link_register(&HSR_LINK_OPS); if rc != 0 { return rc; }
    rc = genl_register_family(&mut HSR_GENL_FAMILY); if rc != 0 { rtnl_link_unregister(&HSR_LINK_OPS); return rc; }
    hsr_debugfs_create_root(); 0
}

#[no_mangle]
pub unsafe extern "C" fn hsr_netlink_exit() { genl_unregister_family(&mut HSR_GENL_FAMILY); rtnl_link_unregister(&HSR_LINK_OPS); }

// MODULE_ALIAS_RTNL_LINK("hsr");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
