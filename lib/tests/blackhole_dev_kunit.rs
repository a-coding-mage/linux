// SPDX-License-Identifier: GPL-2.0
/*
 * This tests the blackhole_dev that is created during the
 * net subsystem initialization. The test this module performs is
 * by injecting an skb into the stack with skb->dev as the
 * blackhole_dev and expects kernel to behave in a sane manner
 * (in other words, *not crash*)!
 *
 * Copyright (c) 2018, Mahesh Bandewar <maheshb@google.com>
 */

// Dependencies supplied by the kernel KUnit, module, skbuff, netdevice, UDP,
// IPv6, and destination headers are external to this translation unit.

const SKB_SIZE: usize = 256;
const HEAD_SIZE: usize = 14 + 40 + 8; // Ether + IPv6 + UDP
const TAIL_SIZE: usize = 32; // random tail-room
const UDP_PORT: u16 = 1234;

unsafe fn test_blackholedev(test: *mut kunit) {
    let mut ip6h: *mut ipv6hdr;
    let mut skb: *mut sk_buff;
    let mut uh: *mut udphdr;
    let data_len: usize;

    skb = alloc_skb(SKB_SIZE, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, skb);

    /* Reserve head-room for the headers */
    skb_reserve(skb, HEAD_SIZE);

    /* Add data to the skb */
    data_len = SKB_SIZE - (HEAD_SIZE + TAIL_SIZE);
    memset(__skb_put(skb, data_len), 0xf, data_len);

    /* Add protocol data */
    /* (Transport) UDP */
    uh = skb_push(skb, core::mem::size_of::<udphdr>()) as *mut udphdr;
    skb_set_transport_header(skb, 0);
    (*uh).source = htons(UDP_PORT);
    (*uh).dest = htons(UDP_PORT);
    udp_set_len_short(uh, data_len);
    (*uh).check = 0;
    /* (Network) IPv6 */
    ip6h = skb_push(skb, core::mem::size_of::<ipv6hdr>()) as *mut ipv6hdr;
    skb_set_network_header(skb, 0);
    (*ip6h).hop_limit = 32;
    (*ip6h).payload_len = htons((data_len + core::mem::size_of::<udphdr>()) as u16);
    (*ip6h).nexthdr = IPPROTO_UDP;
    (*ip6h).saddr = in6addr_loopback;
    (*ip6h).daddr = in6addr_loopback;
    /* Ether */
    skb_push(skb, core::mem::size_of::<ethhdr>());
    skb_set_mac_header(skb, 0);

    (*skb).protocol = htons(ETH_P_IPV6);
    (*skb).pkt_type = PACKET_HOST;
    (*skb).dev = blackhole_netdev;

    /* Now attempt to send the packet */
    KUNIT_EXPECT_EQ(test, dev_queue_xmit(skb), NET_XMIT_SUCCESS);
}

static mut BLACKHOLEDEV_CASES: [kunit_case; 2] = [
    KUNIT_CASE!(test_blackholedev),
    KUNIT_CASE_EMPTY!(),
];

static mut BLACKHOLEDEV_SUITE: kunit_suite = kunit_suite {
    name: "blackholedev",
    test_cases: BLACKHOLEDEV_CASES.as_mut_ptr(),
};

// kunit_test_suite(blackholedev_suite);

// MODULE_AUTHOR("Mahesh Bandewar <maheshb@google.com>");
// MODULE_DESCRIPTION("module test of the blackhole_dev");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
