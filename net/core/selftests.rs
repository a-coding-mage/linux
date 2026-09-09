// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 Synopsys, Inc. and/or its affiliates.
 * stmmac Selftests Support
 *
 * Author: Jose Abreu <joabreu@synopsys.com>
 *
 * Ported from stmmac by:
 * Copyright (C) 2021 Oleksij Rempel <o.rempel@pengutronix.de>
 */

static mut NET_TEST_NEXT_ID: u8 = 0;

pub unsafe fn net_test_get_skb(
    ndev: *mut net_device,
    id: u8,
    attr: *mut net_packet_attrs,
) -> *mut sk_buff {
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let mut uhdr: *mut udphdr = core::ptr::null_mut();
    let mut thdr: *mut tcphdr = core::ptr::null_mut();
    let shdr: *mut netsfhdr;
    let ehdr: *mut ethhdr;
    let ihdr: *mut iphdr;
    let mut iplen: i32;
    let mut size: i32;

    size = (*attr).size + NET_TEST_PKT_SIZE;

    if (*attr).tcp {
        size += core::mem::size_of::<tcphdr>() as i32;
    } else {
        size += core::mem::size_of::<udphdr>() as i32;
    }

    if (*attr).max_size != 0 && (*attr).max_size > size {
        size = (*attr).max_size;
    }

    skb = netdev_alloc_skb(ndev, size);
    if skb.is_null() {
        return core::ptr::null_mut();
    }

    prefetchw((*skb).data);

    ehdr = skb_push(skb, ETH_HLEN);
    skb_reset_mac_header(skb);

    skb_set_network_header(skb, (*skb).len);
    ihdr = skb_put(skb, core::mem::size_of::<iphdr>());

    skb_set_transport_header(skb, (*skb).len);
    if (*attr).tcp {
        thdr = skb_put(skb, core::mem::size_of::<tcphdr>());
    } else {
        uhdr = skb_put(skb, core::mem::size_of::<udphdr>());
    }

    eth_zero_addr((*ehdr).h_dest.as_mut_ptr());

    if !(*attr).src.is_null() {
        ether_addr_copy((*ehdr).h_source.as_mut_ptr(), (*attr).src);
    }
    if !(*attr).dst.is_null() {
        ether_addr_copy((*ehdr).h_dest.as_mut_ptr(), (*attr).dst);
    }

    (*ehdr).h_proto = htons(ETH_P_IP);

    if (*attr).tcp {
        core::ptr::write_bytes(thdr as *mut u8, 0, core::mem::size_of::<tcphdr>());
        (*thdr).source = htons((*attr).sport);
        (*thdr).dest = htons((*attr).dport);
        (*thdr).doff = (core::mem::size_of::<tcphdr>() / 4) as _;
    } else {
        (*uhdr).source = htons((*attr).sport);
        (*uhdr).dest = htons((*attr).dport);
        udp_set_len_short(uhdr, core::mem::size_of::<netsfhdr>() + core::mem::size_of::<udphdr>() + (*attr).size as usize);
        if (*attr).max_size != 0 {
            udp_set_len_short(uhdr, (*attr).max_size as usize - (core::mem::size_of::<iphdr>() + core::mem::size_of::<ethhdr>()));
        }
        (*uhdr).check = 0;
    }

    (*ihdr).ihl = 5;
    (*ihdr).ttl = 32;
    (*ihdr).version = 4;
    (*ihdr).protocol = if (*attr).tcp { IPPROTO_TCP } else { IPPROTO_UDP };
    iplen = (core::mem::size_of::<iphdr>() + core::mem::size_of::<netsfhdr>() + (*attr).size as usize) as i32;
    iplen += if (*attr).tcp { core::mem::size_of::<tcphdr>() } else { core::mem::size_of::<udphdr>() } as i32;

    if (*attr).max_size != 0 {
        iplen = (*attr).max_size - core::mem::size_of::<ethhdr>() as i32;
    }

    (*ihdr).tot_len = htons(iplen as _);
    (*ihdr).frag_off = 0;
    (*ihdr).saddr = htonl((*attr).ip_src);
    (*ihdr).daddr = htonl((*attr).ip_dst);
    (*ihdr).tos = 0;
    (*ihdr).id = 0;
    ip_send_check(ihdr);

    shdr = skb_put(skb, core::mem::size_of::<netsfhdr>());
    (*shdr).version = 0;
    (*shdr).magic = cpu_to_be64(NET_TEST_PKT_MAGIC);
    (*attr).id = id;
    (*shdr).id = id;

    if (*attr).size != 0 {
        let payload: *mut core::ffi::c_void = skb_put(skb, (*attr).size as usize);
        core::ptr::write_bytes(payload as *mut u8, 0, (*attr).size as usize);
    }

    if (*attr).max_size != 0 && (*attr).max_size as usize > (*skb).len {
        let pad_len = (*attr).max_size as usize - (*skb).len;
        let pad: *mut core::ffi::c_void = skb_put(skb, pad_len);
        core::ptr::write_bytes(pad as *mut u8, 0, pad_len);
    }

    (*skb).csum = 0;
    (*skb).ip_summed = CHECKSUM_PARTIAL;
    if (*attr).tcp {
        let l4len = (*skb).len - skb_transport_offset(skb);
        (*thdr).check = !tcp_v4_check(l4len, (*ihdr).saddr, (*ihdr).daddr, 0);
        (*skb).csum_start = skb_transport_header(skb).offset_from((*skb).head) as _;
        (*skb).csum_offset = core::mem::offset_of!(tcphdr, check);

        if (*attr).bad_csum {
            /* Force mangled checksum */
            if skb_checksum_help(skb) != 0 {
                kfree_skb(skb);
                return core::ptr::null_mut();
            }

            if (*thdr).check != CSUM_MANGLED_0 {
                (*thdr).check = CSUM_MANGLED_0;
            } else {
                (*thdr).check = csum16_sub((*thdr).check, cpu_to_be16(1));
            }
        }
    } else {
        udp4_hwcsum(skb, (*ihdr).saddr, (*ihdr).daddr);
    }

    (*skb).protocol = htons(ETH_P_IP);
    (*skb).pkt_type = PACKET_HOST;
    (*skb).dev = ndev;
    skb
}

static unsafe fn net_test_loopback_validate(
    mut skb: *mut sk_buff,
    _ndev: *mut net_device,
    pt: *mut packet_type,
    _orig_ndev: *mut net_device,
) -> i32 {
    let tpriv: *mut net_test_priv = (*pt).af_packet_priv as *mut _;
    let src = (*(*tpriv).packet).src;
    let dst = (*(*tpriv).packet).dst;
    let shdr: *mut netsfhdr;
    let ehdr: *mut ethhdr;
    let uhdr: *mut udphdr;
    let thdr: *mut tcphdr;
    let mut ihdr: *mut iphdr;

    skb = skb_unshare(skb, GFP_ATOMIC);
    if skb.is_null() { return 0; }
    if skb_linearize(skb) != 0 { kfree_skb(skb); return 0; }
    if skb_headlen(skb) < (NET_TEST_PKT_SIZE - ETH_HLEN) { kfree_skb(skb); return 0; }

    ehdr = skb_mac_header(skb) as *mut ethhdr;
    if !dst.is_null() && !ether_addr_equal_unaligned((*ehdr).h_dest.as_ptr(), dst) { kfree_skb(skb); return 0; }
    if !src.is_null() && !ether_addr_equal_unaligned((*ehdr).h_source.as_ptr(), src) { kfree_skb(skb); return 0; }

    ihdr = ip_hdr(skb);
    if (*tpriv).double_vlan { ihdr = (skb_network_header(skb) as *mut u8).add(4) as *mut iphdr; }

    if (*(*tpriv).packet).tcp {
        if (*ihdr).protocol != IPPROTO_TCP { kfree_skb(skb); return 0; }
        thdr = ((ihdr as *mut u8).add(4 * (*ihdr).ihl as usize)) as *mut tcphdr;
        if (*thdr).dest != htons((*(*tpriv).packet).dport) { kfree_skb(skb); return 0; }
        shdr = (thdr as *mut u8).add(core::mem::size_of::<tcphdr>()) as *mut netsfhdr;
    } else {
        if (*ihdr).protocol != IPPROTO_UDP { kfree_skb(skb); return 0; }
        uhdr = ((ihdr as *mut u8).add(4 * (*ihdr).ihl as usize)) as *mut udphdr;
        if (*uhdr).dest != htons((*(*tpriv).packet).dport) { kfree_skb(skb); return 0; }
        shdr = (uhdr as *mut u8).add(core::mem::size_of::<udphdr>()) as *mut netsfhdr;
    }

    if (*shdr).magic != cpu_to_be64(NET_TEST_PKT_MAGIC) || (*(*tpriv).packet).id != (*shdr).id { kfree_skb(skb); return 0; }
    if (*(*tpriv).packet).bad_csum && (*skb).ip_summed == CHECKSUM_UNNECESSARY { (*tpriv).ok = -EIO; } else { (*tpriv).ok = true; }
    complete(&mut (*tpriv).comp);
    kfree_skb(skb);
    0
}

static unsafe fn __net_test_loopback(ndev: *mut net_device, attr: *mut net_packet_attrs) -> i32 {
    let tpriv: *mut net_test_priv = kzalloc_obj();
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let mut ret = 0;
    if tpriv.is_null() { return -ENOMEM; }
    (*tpriv).ok = false;
    init_completion(&mut (*tpriv).comp);
    (*tpriv).pt.type_ = htons(ETH_P_IP);
    (*tpriv).pt.func = net_test_loopback_validate;
    (*tpriv).pt.dev = ndev;
    (*tpriv).pt.af_packet_priv = tpriv as *mut _;
    (*tpriv).packet = attr;
    dev_add_pack(&mut (*tpriv).pt);
    skb = net_test_get_skb(ndev, NET_TEST_NEXT_ID, attr);
    if skb.is_null() { ret = -ENOMEM; } else {
        NET_TEST_NEXT_ID = NET_TEST_NEXT_ID.wrapping_add(1);
        ret = dev_direct_xmit(skb, (*attr).queue_mapping);
        if ret > 0 { ret = -ENETUNREACH; }
        if ret >= 0 {
            if (*attr).timeout == 0 { (*attr).timeout = NET_LB_TIMEOUT; }
            wait_for_completion_timeout(&mut (*tpriv).comp, (*attr).timeout);
            ret = if (*tpriv).ok < 0 { (*tpriv).ok } else if !(*tpriv).ok { -ETIMEDOUT } else { 0 };
        }
    }
    dev_remove_pack(&mut (*tpriv).pt);
    kfree(tpriv as *mut _);
    ret
}

static unsafe fn net_test_netif_carrier(ndev: *mut net_device) -> i32 { if netif_carrier_ok(ndev) { 0 } else { -ENOLINK } }
static unsafe fn net_test_phy_phydev(ndev: *mut net_device) -> i32 { if !(*ndev).phydev.is_null() { 0 } else { -EOPNOTSUPP } }
static unsafe fn net_test_phy_loopback_enable(ndev: *mut net_device) -> i32 { if (*ndev).phydev.is_null() { -EOPNOTSUPP } else { phy_loopback((*ndev).phydev, true, 0) } }
static unsafe fn net_test_phy_loopback_disable(ndev: *mut net_device) -> i32 { if (*ndev).phydev.is_null() { -EOPNOTSUPP } else { phy_loopback((*ndev).phydev, false, 0) } }
static unsafe fn net_test_phy_loopback_udp(ndev: *mut net_device) -> i32 { let mut attr = core::mem::zeroed::<net_packet_attrs>(); attr.dst = (*ndev).dev_addr.as_mut_ptr(); __net_test_loopback(ndev, &mut attr) }
static unsafe fn net_test_phy_loopback_udp_mtu(ndev: *mut net_device) -> i32 { let mut attr = core::mem::zeroed::<net_packet_attrs>(); attr.dst = (*ndev).dev_addr.as_mut_ptr(); attr.max_size = (*ndev).mtu; __net_test_loopback(ndev, &mut attr) }
static unsafe fn net_test_phy_loopback_tcp(ndev: *mut net_device) -> i32 { let mut attr = core::mem::zeroed::<net_packet_attrs>(); attr.dst = (*ndev).dev_addr.as_mut_ptr(); attr.tcp = true; __net_test_loopback(ndev, &mut attr) }

/**
 * net_test_phy_loopback_tcp_bad_csum - PHY loopback test with a deliberately
 * corrupted TCP checksum
 * @ndev: the network device to test
 *
 * Builds the same minimal Ethernet/IPv4/TCP frame as
 * net_test_phy_loopback_tcp(), then flips the least-significant bit of the TCP
 * checksum so the resulting value is provably invalid (neither 0 nor 0xFFFF).
 * The frame is transmitted through the device’s internal PHY loopback path:
 *
 *   test code -> MAC driver -> MAC HW -> xMII -> PHY ->
 *   internal PHY loopback -> xMII -> MAC HW -> MAC driver -> test code
 *
 * Result interpretation
 * ---------------------
 *  0            The frame is delivered to the stack and the driver reports
 *               ip_summed as CHECKSUM_NONE or CHECKSUM_COMPLETE - both are
 *               valid ways to indicate “bad checksum, let the stack verify.”
 *  -ETIMEDOUT   The MAC/PHY silently dropped the frame; hardware checksum
 *               verification filtered it out before the driver saw it.
 *  -EIO         The driver returned the frame with ip_summed ==
 *               CHECKSUM_UNNECESSARY, falsely claiming a valid checksum and
 *               indicating a serious RX-path defect.
 *
 * Return: 0 on success or a negative error code on failure.
 */
static unsafe fn net_test_phy_loopback_tcp_bad_csum(ndev: *mut net_device) -> i32 { let mut attr = core::mem::zeroed::<net_packet_attrs>(); attr.dst = (*ndev).dev_addr.as_mut_ptr(); attr.tcp = true; attr.bad_csum = true; __net_test_loopback(ndev, &mut attr) }

struct net_test { name: &'static str, fn_: unsafe fn(*mut net_device) -> i32 }
static NET_SELFTESTS: &[net_test] = &[
    net_test { name: "Carrier                       ", fn_: net_test_netif_carrier },
    net_test { name: "PHY dev is present            ", fn_: net_test_phy_phydev },
    net_test { name: "PHY internal loopback, enable ", fn_: net_test_phy_loopback_enable },
    net_test { name: "PHY internal loopback, UDP    ", fn_: net_test_phy_loopback_udp },
    net_test { name: "PHY internal loopback, MTU    ", fn_: net_test_phy_loopback_udp_mtu },
    net_test { name: "PHY internal loopback, TCP    ", fn_: net_test_phy_loopback_tcp },
    net_test { name: "PHY loopback, bad TCP csum    ", fn_: net_test_phy_loopback_tcp_bad_csum },
    net_test { name: "PHY internal loopback, disable", fn_: net_test_phy_loopback_disable },
];

pub unsafe fn net_selftest(ndev: *mut net_device, etest: *mut ethtool_test, buf: *mut u64) {
    let count = net_selftest_get_count();
    core::ptr::write_bytes(buf as *mut u8, 0, core::mem::size_of::<u64>() * count);
    NET_TEST_NEXT_ID = 0;
    if (*etest).flags != ETH_TEST_FL_OFFLINE {
        netdev_err(ndev, "Only offline tests are supported\n");
        (*etest).flags |= ETH_TEST_FL_FAILED;
        return;
    }
    for i in 0..count {
        *buf.add(i) = (NET_SELFTESTS[i].fn_)(ndev) as u64;
        if *buf.add(i) != 0 && *buf.add(i) != (-EOPNOTSUPP as u64) { (*etest).flags |= ETH_TEST_FL_FAILED; }
    }
}

pub fn net_selftest_get_count() -> usize { NET_SELFTESTS.len() }

pub unsafe fn net_selftest_get_strings(mut data: *mut u8) {
    for i in 0..net_selftest_get_count() { ethtool_sprintf(&mut data, "%2d. %s", i + 1, NET_SELFTESTS[i].name.as_ptr()); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
