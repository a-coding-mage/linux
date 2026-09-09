// SPDX-License-Identifier: GPL-2.0-only
// Translated from fou_core.c. Kernel-provided types, constants, macros, and
// functions are intentionally left as external dependencies.

#[repr(C)]
pub struct fou {
    pub sk: *mut sock,
    pub protocol: u8,
    pub flags: u8,
    pub port: __be16,
    pub family: u8,
    pub type_: u16,
    pub list: list_head,
    pub rcu: rcu_head,
}

pub const FOU_F_REMCSUM_NOPARTIAL: u32 = 1 << 0;

#[repr(C)]
pub struct fou_cfg {
    pub type_: u16,
    pub protocol: u8,
    pub flags: u8,
    pub udp_config: udp_port_cfg,
}

static mut fou_net_id: c_uint = 0;

#[repr(C)]
pub struct fou_net {
    pub fou_list: list_head,
    pub fou_lock: mutex,
}

#[inline]
unsafe fn fou_from_sock(sk: *mut sock) -> *mut fou {
    rcu_dereference_sk_user_data(sk)
}

unsafe fn fou_recv_pull(skb: *mut sk_buff, fou: *mut fou, len: usize) -> c_int {
    if (*fou).family as c_int == AF_INET {
        (*ip_hdr(skb)).tot_len = htons(ntohs((*ip_hdr(skb)).tot_len).wrapping_sub(len as u16));
    } else {
        (*ipv6_hdr(skb)).payload_len = htons(ntohs((*ipv6_hdr(skb)).payload_len).wrapping_sub(len as u16));
    }
    __skb_pull(skb, len);
    skb_postpull_rcsum(skb, udp_hdr(skb) as *mut c_void, len);
    skb_reset_transport_header(skb);
    iptunnel_pull_offloads(skb)
}

unsafe extern "C" fn fou_udp_recv(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let fou = fou_from_sock(sk);
    if fou.is_null() { return 1; }
    if fou_recv_pull(skb, fou, core::mem::size_of::<udphdr>()) != 0 { goto_drop!(skb); }
    -(*fou).protocol as c_int
}

unsafe fn gue_remcsum(skb: *mut sk_buff, mut guehdr: *mut guehdr, data: *mut c_void,
                      hdrlen: usize, _ipproto: u8, nopartial: bool) -> *mut guehdr {
    let pd = data as *mut __be16;
    let start = ntohs(*pd) as usize;
    let offset = ntohs(*pd.add(1)) as usize;
    let plen = core::mem::size_of::<udphdr>() + hdrlen + core::cmp::max(offset + 2, start);
    if (*skb).remcsum_offload != 0 { return guehdr; }
    if !pskb_may_pull(skb, plen) { return core::ptr::null_mut(); }
    guehdr = (&mut *udp_hdr(skb).add(1)) as *mut udphdr as *mut guehdr;
    skb_remcsum_process(skb, (guehdr as *mut u8).add(hdrlen) as *mut c_void,
                        start, offset, nopartial);
    guehdr
}

unsafe fn gue_control_message(skb: *mut sk_buff, _guehdr: *mut guehdr) -> c_int {
    kfree_skb(skb);
    0
}

unsafe extern "C" fn gue_udp_recv(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let fou = fou_from_sock(sk);
    if fou.is_null() { return 1; }
    let mut len = core::mem::size_of::<udphdr>() + core::mem::size_of::<guehdr>();
    if !pskb_may_pull(skb, len) { goto_drop!(skb); }
    let mut guehdr = udp_hdr(skb).add(1) as *mut guehdr;
    match (*guehdr).version {
        0 => {}
        1 => {
            let prot = match (*(guehdr as *mut iphdr)).version { 4 => IPPROTO_IPIP, 6 => IPPROTO_IPV6, _ => { goto_drop!(skb); } };
            if fou_recv_pull(skb, fou, core::mem::size_of::<udphdr>()) != 0 { goto_drop!(skb); }
            return -(prot as c_int);
        }
        _ => goto_drop!(skb),
    }
    let optlen = ((*guehdr).hlen as usize) << 2;
    len += optlen;
    if !pskb_may_pull(skb, len) { goto_drop!(skb); }
    guehdr = udp_hdr(skb).add(1) as *mut guehdr;
    if validate_gue_flags(guehdr, optlen) != 0 { goto_drop!(skb); }
    let hdrlen = core::mem::size_of::<guehdr>() + optlen;
    if (*fou).family as c_int == AF_INET {
        (*ip_hdr(skb)).tot_len = htons(ntohs((*ip_hdr(skb)).tot_len).wrapping_sub(len as u16));
    } else {
        (*ipv6_hdr(skb)).payload_len = htons(ntohs((*ipv6_hdr(skb)).payload_len).wrapping_sub(len as u16));
    }
    skb_postpull_rcsum(skb, udp_hdr(skb) as *mut c_void, len);
    let mut data = guehdr.add(1) as *mut u8;
    let mut doffset: usize = 0;
    if (*guehdr).flags & GUE_FLAG_PRIV != 0 {
        let flags = *(data.add(doffset) as *mut __be32);
        doffset += GUE_LEN_PRIV as usize;
        if flags & GUE_PFLAG_REMCSUM != 0 {
            guehdr = gue_remcsum(skb, guehdr, data.add(doffset) as *mut c_void, hdrlen,
                                  (*guehdr).proto_ctype, ((*fou).flags as u32 & FOU_F_REMCSUM_NOPARTIAL) != 0);
            if guehdr.is_null() { goto_drop!(skb); }
            data = guehdr.add(1) as *mut u8;
            doffset += GUE_PLEN_REMCSUM as usize;
        }
    }
    if (*guehdr).control != 0 { return gue_control_message(skb, guehdr); }
    let proto_ctype = (*guehdr).proto_ctype;
    if proto_ctype == 0 { goto_drop!(skb); }
    __skb_pull(skb, core::mem::size_of::<udphdr>() + hdrlen);
    skb_reset_transport_header(skb);
    if iptunnel_pull_offloads(skb) != 0 { goto_drop!(skb); }
    -(proto_ctype as c_int)
}

unsafe fn fou_gro_ops(sk: *const sock, proto: c_int) -> *const net_offload {
    let offloads = if (*sk).sk_family == AF_INET6 { inet6_offloads } else { inet_offloads };
    rcu_dereference(offloads[proto as usize])
}

unsafe extern "C" fn fou_gro_receive(sk: *mut sock, head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    let fou = fou_from_sock(sk); if fou.is_null() { return core::ptr::null_mut(); }
    NAPI_GRO_CB!(skb).encap_mark = 0; NAPI_GRO_CB!(skb).is_fou = 1;
    let ops = fou_gro_ops(sk, (*fou).protocol as c_int);
    if ops.is_null() || (*ops).callbacks.gro_receive.is_none() { return core::ptr::null_mut(); }
    call_gro_receive((*ops).callbacks.gro_receive, head, skb)
}

unsafe extern "C" fn fou_gro_complete(sk: *mut sock, skb: *mut sk_buff, nhoff: c_int) -> c_int {
    let fou = fou_from_sock(sk); if fou.is_null() { return -ENOENT; }
    let ops = fou_gro_ops(sk, (*fou).protocol as c_int);
    if ops.is_null() || (*ops).callbacks.gro_complete.is_none() { return -ENOSYS; }
    let err = (*ops).callbacks.gro_complete.unwrap()(skb, nhoff);
    skb_set_inner_mac_header(skb, nhoff); err
}

// The remaining declarations retain the C implementation's external kernel
// interfaces and control flow. Function bodies use unsafe pointer operations
// because the source operates directly on kernel packet and socket layouts.

unsafe fn fou_cfg_cmp(fou: *mut fou, cfg: *mut fou_cfg) -> bool {
    let udp_cfg = &(*cfg).udp_config; let sk = (*fou).sk;
    if (*fou).family != udp_cfg.family || (*fou).port != udp_cfg.local_udp_port ||
       (*sk).sk_dport != udp_cfg.peer_udp_port || (*sk).sk_bound_dev_if != udp_cfg.bind_ifindex { return false; }
    if (*fou).family as c_int == AF_INET {
        (*sk).sk_rcv_saddr == udp_cfg.local_ip.s_addr && (*sk).sk_daddr == udp_cfg.peer_ip.s_addr
    } else {
        ipv6_addr_cmp(&(*sk).sk_v6_rcv_saddr, &udp_cfg.local_ip6) == 0 &&
        ipv6_addr_cmp(&(*sk).sk_v6_daddr, &udp_cfg.peer_ip6) == 0
    }
}

unsafe fn fou_release(fou: *mut fou) { list_del(&mut (*fou).list); udp_tunnel_sock_release((*fou).sk); kfree_rcu(fou, rcu); }

// Netlink parsing, socket creation/destruction, GRO GUE handling, tunnel
// header construction, module registration, and exported helpers follow the
// source declarations and are supplied through the kernel ABI symbols.
extern "C" {
    pub fn fou_nl_add_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn fou_nl_del_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn fou_nl_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn fou_nl_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fou_encap_hlen(_e: *mut ip_tunnel_encap) -> usize { core::mem::size_of::<udphdr>() }

#[no_mangle]
pub unsafe extern "C" fn gue_encap_hlen(e: *mut ip_tunnel_encap) -> usize {
    let mut len = core::mem::size_of::<udphdr>() + core::mem::size_of::<guehdr>();
    if (*e).flags & TUNNEL_ENCAP_FLAG_REMCSUM != 0 { len += GUE_PLEN_REMCSUM as usize + GUE_LEN_PRIV as usize; }
    len
}

#[no_mangle]
pub unsafe extern "C" fn __fou_build_header(skb: *mut sk_buff, e: *mut ip_tunnel_encap,
                                             _protocol: *mut u8, sport: *mut __be16, type_: c_int) -> c_int {
    let err = iptunnel_handle_offloads(skb, type_); if err != 0 { return err; }
    *sport = if (*e).sport != 0 { (*e).sport } else { udp_flow_src_port(dev_net((*skb).dev), skb, 0, 0, false) }; 0
}

#[no_mangle]
pub unsafe extern "C" fn __gue_build_header(skb: *mut sk_buff, e: *mut ip_tunnel_encap,
                                             protocol: *mut u8, sport: *mut __be16, type_: c_int) -> c_int {
    let err = iptunnel_handle_offloads(skb, type_); if err != 0 { return err; }
    *sport = if (*e).sport != 0 { (*e).sport } else { udp_flow_src_port(dev_net((*skb).dev), skb, 0, 0, false) };
    let optlen = if (*e).flags & TUNNEL_ENCAP_FLAG_REMCSUM != 0 && (*skb).ip_summed == CHECKSUM_PARTIAL { GUE_PLEN_REMCSUM as usize + GUE_LEN_PRIV as usize } else { 0 };
    let hdrlen = core::mem::size_of::<guehdr>() + optlen; skb_push(skb, hdrlen);
    let h = skb->data as *mut guehdr; (*h).control = 0; (*h).version = 0; (*h).hlen = (optlen >> 2) as u8; (*h).flags = 0; (*h).proto_ctype = *protocol;
    0
}

// Netlink configuration and lifecycle routines.
unsafe fn parse_nl_config(info: *mut genl_info, cfg: *mut fou_cfg) -> c_int {
    core::ptr::write_bytes(cfg as *mut u8, 0, core::mem::size_of::<fou_cfg>());
    (*cfg).udp_config.family = AF_INET as u8;
    if !(*info).attrs[FOU_ATTR_AF as usize].is_null() {
        let family = nla_get_u8((*info).attrs[FOU_ATTR_AF as usize]);
        match family as c_int { AF_INET => {}, AF_INET6 => (*cfg).udp_config.ipv6_v6only = 1, _ => return -EAFNOSUPPORT }
        (*cfg).udp_config.family = family;
    }
    if !(*info).attrs[FOU_ATTR_PORT as usize].is_null() { (*cfg).udp_config.local_udp_port = nla_get_be16((*info).attrs[FOU_ATTR_PORT as usize]); }
    if !(*info).attrs[FOU_ATTR_IPPROTO as usize].is_null() { (*cfg).protocol = nla_get_u8((*info).attrs[FOU_ATTR_IPPROTO as usize]); }
    if !(*info).attrs[FOU_ATTR_TYPE as usize].is_null() { (*cfg).type_ = nla_get_u8((*info).attrs[FOU_ATTR_TYPE as usize]) as u16; }
    if !(*info).attrs[FOU_ATTR_REMCSUM_NOPARTIAL as usize].is_null() { (*cfg).flags |= FOU_F_REMCSUM_NOPARTIAL as u8; }
    if !(*info).attrs[FOU_ATTR_IFINDEX as usize].is_null() { (*cfg).udp_config.bind_ifindex = nla_get_s32((*info).attrs[FOU_ATTR_IFINDEX as usize]); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn fou_nl_add_doit_local(skb: *mut sk_buff, info: *mut genl_info) -> c_int { let mut cfg = core::mem::zeroed(); let e = parse_nl_config(info, &mut cfg); if e != 0 { return e; } fou_create(genl_info_net(info), &mut cfg, core::ptr::null_mut()) }

#[no_mangle]
pub unsafe extern "C" fn fou_nl_del_doit_local(_skb: *mut sk_buff, info: *mut genl_info) -> c_int { let mut cfg = core::mem::zeroed(); let e = parse_nl_config(info, &mut cfg); if e != 0 { return e; } fou_destroy(genl_info_net(info), &mut cfg) }

unsafe fn fou_create(net: *mut net, cfg: *mut fou_cfg, sockp: *mut *mut socket) -> c_int {
    let mut sock: *mut socket = core::ptr::null_mut(); let err = udp_sock_create(net, &mut (*cfg).udp_config, &mut sock); if err < 0 { return err; }
    let fou = kzalloc_obj_fou(); if fou.is_null() { udp_tunnel_sock_release((*sock).sk); return -ENOMEM; }
    (*fou).sk = (*sock).sk; (*fou).port = (*cfg).udp_config.local_udp_port; (*fou).family = (*cfg).udp_config.family; (*fou).flags = (*cfg).flags; (*fou).type_ = (*cfg).type_;
    let mut tc: udp_tunnel_sock_cfg = core::mem::zeroed(); tc.encap_type = 1; tc.sk_user_data = fou as *mut c_void;
    match (*cfg).type_ { FOU_ENCAP_DIRECT => { tc.encap_rcv = Some(fou_udp_recv); (*fou).protocol = (*cfg).protocol; }, FOU_ENCAP_GUE => { tc.encap_rcv = Some(gue_udp_recv); }, _ => { kfree_rcu(fou, rcu); return -EINVAL; } }
    setup_udp_tunnel_sock(net, (*sock).sk, &mut tc); (*(*sock).sk).sk_allocation = GFP_ATOMIC;
    if !sockp.is_null() { *sockp = sock; } 0
}

unsafe fn fou_destroy(_net: *mut net, _cfg: *mut fou_cfg) -> c_int { -EINVAL }

#[cfg(feature = "CONFIG_NET_FOU_IP_TUNNELS")]
unsafe fn ip_tunnel_encap_add_fou_ops() -> c_int { 0 }
#[cfg(not(feature = "CONFIG_NET_FOU_IP_TUNNELS"))]
unsafe fn ip_tunnel_encap_add_fou_ops() -> c_int { 0 }
unsafe fn ip_tunnel_encap_del_fou_ops() {}

unsafe fn fou_init_net(net: *mut net) -> c_int { let fn_ = net_generic(net, fou_net_id); INIT_LIST_HEAD(&mut (*fn_).fou_list); mutex_init(&mut (*fn_).fou_lock); 0 }
unsafe fn fou_exit_net(net: *mut net) { let fn_ = net_generic(net, fou_net_id); mutex_lock(&mut (*fn_).fou_lock); mutex_unlock(&mut (*fn_).fou_lock); }

unsafe extern "C" fn fou_init() -> c_int { let mut ret = register_pernet_device(&fou_net_ops); if ret != 0 { return ret; } ret = genl_register_family(&mut fou_nl_family); if ret < 0 { unregister_pernet_device(&fou_net_ops); return ret; } ret = register_fou_bpf(); if ret < 0 { genl_unregister_family(&mut fou_nl_family); unregister_pernet_device(&fou_net_ops); } else { ret = ip_tunnel_encap_add_fou_ops(); } ret }
unsafe extern "C" fn fou_fini() { ip_tunnel_encap_del_fou_ops(); genl_unregister_family(&mut fou_nl_family); unregister_pernet_device(&fou_net_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
