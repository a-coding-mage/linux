/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from testing/selftests/net/tuntap_helpers.h.
// C include dependencies are expected to provide the referenced C-compatible
// kernel/libc/YNL route, addr, neigh, and link types and constants.

pub const GENEVE_HLEN: i32 = 8;
pub const PKT_DATA: i32 = 0xCB;
pub const TUNTAP_DEFAULT_TTL: i32 = 8;
pub const TUNTAP_DEFAULT_IPID: i32 = 1337;

unsafe extern "C" {
    pub fn if_nametoindex(ifname: *const core::ffi::c_char) -> core::ffi::c_uint;
}

pub unsafe fn ip_addr_len(family: core::ffi::c_int) -> core::ffi::c_int {
    if family == AF_INET {
        core::mem::size_of::<in_addr>() as core::ffi::c_int
    } else {
        core::mem::size_of::<in6_addr>() as core::ffi::c_int
    }
}

pub unsafe fn fill_ifaddr_msg(
    ifam: *mut ifaddrmsg,
    family: core::ffi::c_int,
    prefix: core::ffi::c_int,
    flags: core::ffi::c_int,
    dev: *const core::ffi::c_char,
) {
    unsafe {
        (*ifam).ifa_family = family as _;
        (*ifam).ifa_prefixlen = prefix as _;
        (*ifam).ifa_index = if_nametoindex(dev) as _;
        (*ifam).ifa_flags = flags as _;
        (*ifam).ifa_scope = RT_SCOPE_UNIVERSE as _;
    }
}

pub unsafe fn ip_addr_add(
    dev: *const core::ffi::c_char,
    family: core::ffi::c_int,
    addr: *mut core::ffi::c_void,
    prefix: u8,
) -> core::ffi::c_int {
    let nl_flags = NLM_F_REQUEST | NLM_F_CREATE | NLM_F_EXCL;
    let ifa_flags = IFA_F_PERMANENT | IFA_F_NODAD;
    let mut ret = -1;
    let ipalen = unsafe { ip_addr_len(family) };
    let req: *mut rt_addr_newaddr_req;
    let ys: *mut ynl_sock;

    unsafe {
        ys = ynl_sock_create(&raw const ynl_rt_addr_family, core::ptr::null_mut());
        if ys.is_null() {
            return -1;
        }

        req = rt_addr_newaddr_req_alloc();
        if req.is_null() {
            ynl_sock_destroy(ys);
            return ret;
        }

        fill_ifaddr_msg(&raw mut (*req)._hdr, family, prefix as _, ifa_flags, dev);
        rt_addr_newaddr_req_set_nlflags(req, nl_flags);
        rt_addr_newaddr_req_set_local(req, addr, ipalen);

        ret = rt_addr_newaddr(ys, req);
        rt_addr_newaddr_req_free(req);
        ynl_sock_destroy(ys);
    }
    ret
}

pub unsafe fn fill_neigh_req_header(
    ndm: *mut ndmsg,
    family: core::ffi::c_int,
    state: core::ffi::c_int,
    dev: *const core::ffi::c_char,
) {
    unsafe {
        (*ndm).ndm_family = family as _;
        (*ndm).ndm_ifindex = if_nametoindex(dev) as _;
        (*ndm).ndm_state = state as _;
        (*ndm).ndm_flags = 0;
        (*ndm).ndm_type = RTN_UNICAST as _;
    }
}

pub unsafe fn ip_neigh_add(
    dev: *const core::ffi::c_char,
    family: core::ffi::c_int,
    addr: *mut core::ffi::c_void,
    lladdr: *mut u8,
) -> core::ffi::c_int {
    let nl_flags = NLM_F_REQUEST | NLM_F_CREATE | NLM_F_EXCL;
    let mut ret = -1;
    let ipalen = unsafe { ip_addr_len(family) };
    let req: *mut rt_neigh_newneigh_req;
    let ys: *mut ynl_sock;

    unsafe {
        ys = ynl_sock_create(&raw const ynl_rt_neigh_family, core::ptr::null_mut());
        if ys.is_null() {
            return -1;
        }

        req = rt_neigh_newneigh_req_alloc();
        if req.is_null() {
            ynl_sock_destroy(ys);
            return ret;
        }

        fill_neigh_req_header(&raw mut (*req)._hdr, family, NUD_PERMANENT, dev);
        rt_neigh_newneigh_req_set_nlflags(req, nl_flags);
        rt_neigh_newneigh_req_set_dst(req, addr, ipalen);
        rt_neigh_newneigh_req_set_lladdr(req, lladdr, ETH_ALEN);
        rt_neigh_newneigh_req_set_ifindex(req, if_nametoindex(dev));

        ret = rt_neigh_newneigh(ys, req);
        rt_neigh_newneigh_req_free(req);
        ynl_sock_destroy(ys);
    }
    ret
}

pub unsafe fn fill_route_req_header(
    rtm: *mut rtmsg,
    family: core::ffi::c_int,
    table: core::ffi::c_int,
) {
    unsafe {
        (*rtm).rtm_family = family as _;
        (*rtm).rtm_table = table as _;
    }
}

pub unsafe fn ip_route_get(
    dev: *const core::ffi::c_char,
    family: core::ffi::c_int,
    table: core::ffi::c_int,
    dst: *mut core::ffi::c_void,
    parse_rsp: Option<unsafe extern "C" fn(*mut rt_route_getroute_rsp, *mut core::ffi::c_void)>,
    out: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut ret = -1;
    let ipalen = unsafe { ip_addr_len(family) };
    let req: *mut rt_route_getroute_req;
    let rsp: *mut rt_route_getroute_rsp;
    let ys: *mut ynl_sock;

    unsafe {
        ys = ynl_sock_create(&raw const ynl_rt_route_family, core::ptr::null_mut());
        if ys.is_null() {
            return -1;
        }

        req = rt_route_getroute_req_alloc();
        if req.is_null() {
            ynl_sock_destroy(ys);
            return ret;
        }

        fill_route_req_header(&raw mut (*req)._hdr, family, table);
        rt_route_getroute_req_set_nlflags(req, NLM_F_REQUEST);
        rt_route_getroute_req_set_dst(req, dst, ipalen);
        rt_route_getroute_req_set_oif(req, if_nametoindex(dev));

        rsp = rt_route_getroute(ys, req);
        if !rsp.is_null() {
            ret = 0;
            if let Some(parse_rsp_fn) = parse_rsp {
                parse_rsp_fn(rsp, out);
            }
            rt_route_getroute_rsp_free(rsp);
        }

        rt_route_getroute_req_free(req);
        ynl_sock_destroy(ys);
    }
    ret
}

pub unsafe fn ip_link_add(
    dev: *const core::ffi::c_char,
    link_type: *mut core::ffi::c_char,
    fill_link_attr: Option<
        unsafe extern "C" fn(*mut rt_link_newlink_req, *mut core::ffi::c_void) -> core::ffi::c_int,
    >,
    data: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let nl_flags = NLM_F_REQUEST | NLM_F_CREATE | NLM_F_EXCL;
    let req: *mut rt_link_newlink_req;
    let ys: *mut ynl_sock;
    let mut ret = -1;

    unsafe {
        ys = ynl_sock_create(&raw const ynl_rt_link_family, core::ptr::null_mut());
        if ys.is_null() {
            return -1;
        }

        req = rt_link_newlink_req_alloc();
        if req.is_null() {
            ynl_sock_destroy(ys);
            return ret;
        }

        (*req)._hdr.ifi_flags = IFF_UP as _;
        rt_link_newlink_req_set_nlflags(req, nl_flags);
        rt_link_newlink_req_set_ifname(req, dev);
        rt_link_newlink_req_set_linkinfo_kind(req, link_type);

        if let Some(fill_link_attr_fn) = fill_link_attr {
            if fill_link_attr_fn(req, data) < 0 {
                rt_link_newlink_req_free(req);
                ynl_sock_destroy(ys);
                return ret;
            }
        }

        ret = rt_link_newlink(ys, req);
        rt_link_newlink_req_free(req);
        ynl_sock_destroy(ys);
    }
    ret
}

pub unsafe fn ip_link_del(dev: *const core::ffi::c_char) -> core::ffi::c_int {
    let req: *mut rt_link_dellink_req;
    let ys: *mut ynl_sock;
    let mut ret = -1;

    unsafe {
        ys = ynl_sock_create(&raw const ynl_rt_link_family, core::ptr::null_mut());
        if ys.is_null() {
            return -1;
        }

        req = rt_link_dellink_req_alloc();
        if req.is_null() {
            ynl_sock_destroy(ys);
            return ret;
        }

        rt_link_dellink_req_set_nlflags(req, NLM_F_REQUEST);
        rt_link_dellink_req_set_ifname(req, dev);

        ret = rt_link_dellink(ys, req);
        rt_link_dellink_req_free(req);
        ynl_sock_destroy(ys);
    }
    ret
}

pub unsafe fn build_eth(
    buf: *mut u8,
    proto: u16,
    src: *mut u8,
    dest: *mut u8,
) -> usize {
    let eth = buf as *mut ethhdr;

    unsafe {
        (*eth).h_proto = htons(proto);
        core::ptr::copy_nonoverlapping(src, (*eth).h_source.as_mut_ptr(), ETH_ALEN);
        core::ptr::copy_nonoverlapping(dest, (*eth).h_dest.as_mut_ptr(), ETH_ALEN);
    }

    ETH_HLEN
}

pub unsafe fn add_csum(buf: *const u8, mut len: core::ffi::c_int) -> u32 {
    let mut sbuf = buf as *const u16;
    let mut sum: u32 = 0;

    unsafe {
        while len > 1 {
            sum = sum.wrapping_add(*sbuf as u32);
            sbuf = sbuf.add(1);
            len -= 2;
        }

        if len != 0 {
            sum = sum.wrapping_add(*(sbuf as *const u8) as u32);
        }
    }

    sum
}

pub unsafe fn finish_ip_csum(mut sum: u32) -> u16 {
    while (sum >> 16) != 0 {
        sum = (sum & 0xffff).wrapping_add(sum >> 16);
    }
    !(sum as u16)
}

pub unsafe fn build_ip_csum(buf: *const u8, len: core::ffi::c_int, mut sum: u32) -> u16 {
    sum = sum.wrapping_add(unsafe { add_csum(buf, len) });
    unsafe { finish_ip_csum(sum) }
}

pub unsafe fn build_ipv4_header(
    buf: *mut u8,
    proto: u8,
    payload_len: core::ffi::c_int,
    src: *mut in_addr,
    dst: *mut in_addr,
) -> core::ffi::c_int {
    let iph = buf as *mut iphdr;

    unsafe {
        (*iph).ihl = 5;
        (*iph).version = 4;
        (*iph).ttl = TUNTAP_DEFAULT_TTL as _;
        (*iph).tot_len = htons((core::mem::size_of_val(&*iph) as core::ffi::c_int + payload_len) as u16);
        (*iph).id = htons(TUNTAP_DEFAULT_IPID as u16);
        (*iph).protocol = proto;
        (*iph).saddr = (*src).s_addr;
        (*iph).daddr = (*dst).s_addr;
        (*iph).check = build_ip_csum(buf, ((*iph).ihl as core::ffi::c_int) << 2, 0);

        ((*iph).ihl as core::ffi::c_int) << 2
    }
}

pub unsafe fn ipv6_set_dsfield(ip6h: *mut ipv6hdr, dsfield: u8) {
    let ptr = ip6h as *mut u16;

    unsafe {
        let mut val = ntohs(*ptr);
        val &= 0xF00F;
        val |= (dsfield as u16) << 4;
        *ptr = htons(val);
    }
}

pub unsafe fn build_ipv6_header(
    buf: *mut u8,
    proto: u8,
    dsfield: u8,
    payload_len: core::ffi::c_int,
    src: *mut in6_addr,
    dst: *mut in6_addr,
) -> core::ffi::c_int {
    let ip6h = buf as *mut ipv6hdr;

    unsafe {
        (*ip6h).version = 6;
        (*ip6h).payload_len = htons(payload_len as u16);
        (*ip6h).nexthdr = proto;
        (*ip6h).hop_limit = TUNTAP_DEFAULT_TTL as _;
        ipv6_set_dsfield(ip6h, dsfield);
        core::ptr::copy_nonoverlapping(
            src as *const u8,
            &raw mut (*ip6h).saddr as *mut u8,
            core::mem::size_of_val(&(*ip6h).saddr),
        );
        core::ptr::copy_nonoverlapping(
            dst as *const u8,
            &raw mut (*ip6h).daddr as *mut u8,
            core::mem::size_of_val(&(*ip6h).daddr),
        );

        core::mem::size_of::<ipv6hdr>() as core::ffi::c_int
    }
}

pub unsafe fn build_geneve_header(buf: *mut u8, vni: u32) -> core::ffi::c_int {
    let protocol = unsafe { htons(ETH_P_TEB as u16) };
    let geneve_vni = unsafe { htonl((vni << 8) & 0xffffff00) };

    unsafe {
        core::ptr::copy_nonoverlapping(
            &protocol as *const u16 as *const u8,
            buf.add(2),
            2,
        );
        core::ptr::copy_nonoverlapping(
            &geneve_vni as *const u32 as *const u8,
            buf.add(4),
            4,
        );
    }
    GENEVE_HLEN
}

pub unsafe fn build_udp_header(
    buf: *mut u8,
    sport: u16,
    dport: u16,
    payload_len: core::ffi::c_int,
) -> core::ffi::c_int {
    let udph = buf as *mut udphdr;

    unsafe {
        (*udph).source = htons(sport);
        (*udph).dest = htons(dport);
        (*udph).len = htons((core::mem::size_of_val(&*udph) as core::ffi::c_int + payload_len) as u16);
        core::mem::size_of_val(&*udph) as core::ffi::c_int
    }
}

pub unsafe fn build_udp_packet_csum(buf: *mut u8, family: core::ffi::c_int, csum_off: bool) {
    let udph = buf as *mut udphdr;
    let ipalen = unsafe { ip_addr_len(family) } as usize;
    let mut sum: u32;

    unsafe {
        /* No extension IPv4 and IPv6 headers addresses are the last fields */
        sum = add_csum(buf.sub(2 * ipalen), (2 * ipalen) as core::ffi::c_int);
        sum = sum.wrapping_add(htons(IPPROTO_UDP as u16) as u32 + (*udph).len as u32);

        if !csum_off {
            sum = sum.wrapping_add(add_csum(buf, (*udph).len as core::ffi::c_int));
        }

        (*udph).check = finish_ip_csum(sum);
    }
}

pub unsafe fn build_udp_packet(
    buf: *mut u8,
    sport: u16,
    dport: u16,
    payload_len: core::ffi::c_int,
    family: core::ffi::c_int,
    csum_off: bool,
) -> core::ffi::c_int {
    let udph = buf as *mut udphdr;

    unsafe {
        build_udp_header(buf, sport, dport, payload_len);
        core::ptr::write_bytes(
            buf.add(core::mem::size_of_val(&*udph)),
            PKT_DATA as u8,
            payload_len as usize,
        );
        build_udp_packet_csum(buf, family, csum_off);

        core::mem::size_of_val(&*udph) as core::ffi::c_int + payload_len
    }
}

pub unsafe fn build_virtio_net_hdr_v1_hash_tunnel(
    buf: *mut u8,
    is_tap: bool,
    hdr_len: core::ffi::c_int,
    gso_size: core::ffi::c_int,
    outer_family: core::ffi::c_int,
    inner_family: core::ffi::c_int,
) -> core::ffi::c_int {
    let vh_tunnel = buf as *mut virtio_net_hdr_v1_hash_tunnel;
    let vh: *mut virtio_net_hdr_v1;
    let outer_iphlen: core::ffi::c_int;
    let inner_iphlen: core::ffi::c_int;
    let eth_hlen: core::ffi::c_int;
    let gso_type: core::ffi::c_int;

    unsafe {
        vh = &raw mut (*vh_tunnel).hash_hdr.hdr;

        eth_hlen = if is_tap { ETH_HLEN as core::ffi::c_int } else { 0 };
        outer_iphlen = if outer_family == AF_INET {
            core::mem::size_of::<iphdr>() as core::ffi::c_int
        } else {
            core::mem::size_of::<ipv6hdr>() as core::ffi::c_int
        };
        inner_iphlen = if inner_family == AF_INET {
            core::mem::size_of::<iphdr>() as core::ffi::c_int
        } else {
            core::mem::size_of::<ipv6hdr>() as core::ffi::c_int
        };

        (*vh_tunnel).outer_th_offset = (eth_hlen + outer_iphlen) as _;
        (*vh_tunnel).inner_nh_offset = ((*vh_tunnel).outer_th_offset as core::ffi::c_int
            + ETH_HLEN as core::ffi::c_int
            + GENEVE_HLEN
            + core::mem::size_of::<udphdr>() as core::ffi::c_int) as _;

        (*vh).csum_start = ((*vh_tunnel).inner_nh_offset as core::ffi::c_int + inner_iphlen) as _;
        (*vh).csum_offset = core::mem::offset_of!(udphdr, check) as _;
        (*vh).flags = VIRTIO_NET_HDR_F_NEEDS_CSUM as _;
        (*vh).hdr_len = hdr_len as _;
        (*vh).gso_size = gso_size as _;

        if gso_size != 0 {
            gso_type = if outer_family == AF_INET {
                VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV4
            } else {
                VIRTIO_NET_HDR_GSO_UDP_TUNNEL_IPV6
            };
            (*vh).gso_type = (VIRTIO_NET_HDR_GSO_UDP_L4 | gso_type) as _;
        }

        core::mem::size_of::<virtio_net_hdr_v1_hash_tunnel>() as core::ffi::c_int
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
