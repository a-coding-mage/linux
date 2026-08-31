// SPDX-License-Identifier: GPL-2.0
// Translated from C. Original dependencies:
// <test_progs.h>, <network_helpers.h>, <net/if.h>, <linux/if_ether.h>,
// <linux/if_packet.h>, <linux/if_link.h>, <linux/ipv6.h>, <linux/in6.h>,
// <netinet/udp.h>, <bpf/bpf_endian.h>, <uapi/linux/netdev.h>,
// "test_xdp_do_redirect.skel.h", "xdp_dummy.skel.h"

use core::mem::{offset_of, size_of};
use core::ptr;

#[repr(C, packed)]
pub struct udp_packet {
    pub eth: ethhdr,
    pub iph: ipv6hdr,
    pub udp: udphdr,
    pub payload: [__u8; 64 - size_of::<udphdr>() - size_of::<ethhdr>() - size_of::<ipv6hdr>()],
}

static mut pkt_udp: udp_packet = udp_packet {
    eth: ethhdr {
        h_proto: __bpf_constant_htons(ETH_P_IPV6),
        h_dest: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
        h_source: [0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb],
    },
    iph: ipv6hdr {
        version: 6,
        nexthdr: IPPROTO_UDP,
        payload_len: bpf_htons((size_of::<udp_packet>() - offset_of!(udp_packet, udp)) as _),
        hop_limit: 2,
        saddr: in6_addr {
            s6_addr16: [bpf_htons(0xfc00), 0, 0, 0, 0, 0, 0, bpf_htons(1)],
        },
        daddr: in6_addr {
            s6_addr16: [bpf_htons(0xfc00), 0, 0, 0, 0, 0, 0, bpf_htons(2)],
        },
    },
    udp: udphdr {
        source: bpf_htons(1),
        dest: bpf_htons(1),
        len: bpf_htons((size_of::<udp_packet>() - offset_of!(udp_packet, udp)) as _),
    },
    payload: {
        let mut payload = [0u8; 64 - size_of::<udphdr>() - size_of::<ethhdr>() - size_of::<ipv6hdr>()];
        payload[0] = 0x42; /* receiver XDP program matches on this */
        payload
    },
};

unsafe fn attach_tc_prog(hook: *mut bpf_tc_hook, fd: i32) -> i32 {
    let mut opts = bpf_tc_opts {
        handle: 1,
        priority: 1,
        prog_fd: fd,
        ..Default::default()
    };
    let mut ret: i32;

    ret = bpf_tc_hook_create(hook);
    if !ASSERT_OK(ret, c"create tc hook".as_ptr()) {
        return ret;
    }

    ret = bpf_tc_attach(hook, &mut opts);
    if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
        bpf_tc_hook_destroy(hook);
        return ret;
    }

    0
}

/* The maximum permissible size is: PAGE_SIZE - sizeof(struct xdp_page_head) -
 * SKB_DATA_ALIGN(sizeof(struct skb_shared_info)) - XDP_PACKET_HEADROOM =
 * 3408 bytes for 64-byte cacheline and 3216 for 256-byte one.
 */
// C used #if defined(__s390x__) for this architecture-specific value.
#[cfg(target_arch = "s390x")]
const MAX_PKT_SIZE: usize = 3216;
#[cfg(not(target_arch = "s390x"))]
const MAX_PKT_SIZE: usize = 3408;

const PAGE_SIZE_4K: usize = 4096;
const PAGE_SIZE_64K: usize = 65536;

unsafe fn test_max_pkt_size(fd: i32) {
    let mut data = [0i8; PAGE_SIZE_64K + 1];
    let mut err: i32;
    let mut opts = bpf_test_run_opts {
        data_in: data.as_mut_ptr() as *mut _,
        flags: BPF_F_TEST_XDP_LIVE_FRAMES,
        repeat: 1,
        ..Default::default()
    };

    if getpagesize() as usize == PAGE_SIZE_64K {
        opts.data_size_in = (MAX_PKT_SIZE + PAGE_SIZE_64K - PAGE_SIZE_4K) as _;
    } else {
        opts.data_size_in = MAX_PKT_SIZE as _;
    }

    err = bpf_prog_test_run_opts(fd, &mut opts);
    ASSERT_OK(err, c"prog_run_max_size".as_ptr());

    opts.data_size_in += 1;
    err = bpf_prog_test_run_opts(fd, &mut opts);
    ASSERT_EQ(err, -EINVAL, c"prog_run_too_big".as_ptr());
}

const NUM_PKTS: u32 = 10000;

pub unsafe extern "C" fn test_xdp_do_redirect() {
    let mut err: i32;
    let mut xdp_prog_fd: i32;
    let mut tc_prog_fd: i32;
    let mut ifindex_src: i32;
    let mut ifindex_dst: i32;
    let mut data = [0i8; size_of::<udp_packet>() + size_of::<__u64>()];
    let mut skel: *mut test_xdp_do_redirect = ptr::null_mut();
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut link: *mut bpf_link;
    let mut query_opts = bpf_xdp_query_opts::default();
    let mut ctx_in = xdp_md {
        data: size_of::<__u64>() as _,
        data_end: size_of_val(&data) as _,
        ..Default::default()
    };
    let mut opts = bpf_test_run_opts {
        data_in: data.as_mut_ptr() as *mut _,
        data_size_in: size_of_val(&data) as _,
        ctx_in: &mut ctx_in as *mut _ as *mut _,
        ctx_size_in: size_of::<xdp_md>() as _,
        flags: BPF_F_TEST_XDP_LIVE_FRAMES,
        repeat: NUM_PKTS,
        batch_size: 64,
        ..Default::default()
    };
    let mut tc_hook = bpf_tc_hook {
        attach_point: BPF_TC_INGRESS,
        ..Default::default()
    };

    ptr::copy_nonoverlapping(
        &raw const pkt_udp as *const _ as *const u8,
        data.as_mut_ptr().add(size_of::<__u64>()) as *mut u8,
        size_of::<udp_packet>(),
    );
    *(data.as_mut_ptr() as *mut __u32).add(0) = 0x42; /* metadata test value */
    *(data.as_mut_ptr() as *mut __u32).add(1) = 0;

    skel = test_xdp_do_redirect__open();
    if !ASSERT_OK_PTR(skel as *const _, c"skel".as_ptr()) {
        return;
    }

    /* The XDP program we run with bpf_prog_run() will cycle through all
     * three xmit (PASS/TX/REDIRECT) return codes starting from above, and
     * ending up with PASS, so we should end up with two packets on the dst
     * iface and NUM_PKTS-2 in the TC hook. We match the packets on the UDP
     * payload.
     */
    'out: loop {
        SYS!(out, c"ip netns add testns".as_ptr());
        nstoken = open_netns(c"testns".as_ptr());
        if !ASSERT_OK_PTR(nstoken as *const _, c"setns".as_ptr()) {
            break 'out;
        }

        SYS!(out, c"ip link add veth_src type veth peer name veth_dst".as_ptr());
        SYS!(out, c"ip link set dev veth_src address 00:11:22:33:44:55".as_ptr());
        SYS!(out, c"ip link set dev veth_dst address 66:77:88:99:aa:bb".as_ptr());
        SYS!(out, c"ip link set dev veth_src up".as_ptr());
        SYS!(out, c"ip link set dev veth_dst up".as_ptr());
        SYS!(out, c"ip addr add dev veth_src fc00::1/64".as_ptr());
        SYS!(out, c"ip addr add dev veth_dst fc00::2/64".as_ptr());
        SYS!(out, c"ip neigh add fc00::2 dev veth_src lladdr 66:77:88:99:aa:bb".as_ptr());

        /* We enable forwarding in the test namespace because that will cause
         * the packets that go through the kernel stack (with XDP_PASS) to be
         * forwarded back out the same interface (because of the packet dst
         * combined with the interface addresses). When this happens, the
         * regular forwarding path will end up going through the same
         * veth_xdp_xmit() call as the XDP_REDIRECT code, which can cause a
         * deadlock if it happens on the same CPU. There's a local_bh_disable()
         * in the test_run code to prevent this, but an earlier version of the
         * code didn't have this, so we keep the test behaviour to make sure the
         * bug doesn't resurface.
         */
        SYS!(out, c"sysctl -qw net.ipv6.conf.all.forwarding=1".as_ptr());

        ifindex_src = if_nametoindex(c"veth_src".as_ptr()) as i32;
        ifindex_dst = if_nametoindex(c"veth_dst".as_ptr()) as i32;
        if !ASSERT_NEQ(ifindex_src, 0, c"ifindex_src".as_ptr())
            || !ASSERT_NEQ(ifindex_dst, 0, c"ifindex_dst".as_ptr())
        {
            break 'out;
        }

        /* Check xdp features supported by veth driver */
        err = bpf_xdp_query(ifindex_src, XDP_FLAGS_DRV_MODE, &mut query_opts);
        if !ASSERT_OK(err, c"veth_src bpf_xdp_query".as_ptr()) {
            break 'out;
        }

        if !ASSERT_EQ(
            query_opts.feature_flags,
            NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT | NETDEV_XDP_ACT_RX_SG,
            c"veth_src query_opts.feature_flags".as_ptr(),
        ) {
            break 'out;
        }

        err = bpf_xdp_query(ifindex_dst, XDP_FLAGS_DRV_MODE, &mut query_opts);
        if !ASSERT_OK(err, c"veth_dst bpf_xdp_query".as_ptr()) {
            break 'out;
        }

        if !ASSERT_EQ(
            query_opts.feature_flags,
            NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT | NETDEV_XDP_ACT_RX_SG,
            c"veth_dst query_opts.feature_flags".as_ptr(),
        ) {
            break 'out;
        }

        /* Enable GRO */
        SYS!(out, c"ethtool -K veth_src gro on".as_ptr());
        SYS!(out, c"ethtool -K veth_dst gro on".as_ptr());

        err = bpf_xdp_query(ifindex_src, XDP_FLAGS_DRV_MODE, &mut query_opts);
        if !ASSERT_OK(err, c"veth_src bpf_xdp_query gro on".as_ptr()) {
            break 'out;
        }

        if !ASSERT_EQ(
            query_opts.feature_flags,
            NETDEV_XDP_ACT_BASIC
                | NETDEV_XDP_ACT_REDIRECT
                | NETDEV_XDP_ACT_NDO_XMIT
                | NETDEV_XDP_ACT_RX_SG
                | NETDEV_XDP_ACT_NDO_XMIT_SG,
            c"veth_src query_opts.feature_flags gro on".as_ptr(),
        ) {
            break 'out;
        }

        err = bpf_xdp_query(ifindex_dst, XDP_FLAGS_DRV_MODE, &mut query_opts);
        if !ASSERT_OK(err, c"veth_dst bpf_xdp_query gro on".as_ptr()) {
            break 'out;
        }

        if !ASSERT_EQ(
            query_opts.feature_flags,
            NETDEV_XDP_ACT_BASIC
                | NETDEV_XDP_ACT_REDIRECT
                | NETDEV_XDP_ACT_NDO_XMIT
                | NETDEV_XDP_ACT_RX_SG
                | NETDEV_XDP_ACT_NDO_XMIT_SG,
            c"veth_dst query_opts.feature_flags gro on".as_ptr(),
        ) {
            break 'out;
        }

        ptr::copy_nonoverlapping(
            &raw const pkt_udp.eth.h_dest as *const _ as *const u8,
            (*(*skel).rodata).expect_dst.as_mut_ptr(),
            ETH_ALEN,
        );
        (*(*skel).rodata).ifindex_out = ifindex_src; /* redirect back to the same iface */
        (*(*skel).rodata).ifindex_in = ifindex_src;
        ctx_in.ingress_ifindex = ifindex_src;
        tc_hook.ifindex = ifindex_src;

        if !ASSERT_OK(test_xdp_do_redirect__load(skel), c"load".as_ptr()) {
            break 'out;
        }

        link = bpf_program__attach_xdp((*skel).progs.xdp_count_pkts, ifindex_dst);
        if !ASSERT_OK_PTR(link as *const _, c"prog_attach".as_ptr()) {
            break 'out;
        }
        (*skel).links.xdp_count_pkts = link;

        tc_prog_fd = bpf_program__fd((*skel).progs.tc_count_pkts);
        if attach_tc_prog(&mut tc_hook, tc_prog_fd) != 0 {
            break 'out;
        }

        'out_tc: loop {
            xdp_prog_fd = bpf_program__fd((*skel).progs.xdp_redirect);
            err = bpf_prog_test_run_opts(xdp_prog_fd, &mut opts);
            if !ASSERT_OK(err, c"prog_run".as_ptr()) {
                break 'out_tc;
            }

            /* wait for the packets to be flushed */
            kern_sync_rcu();

            /* There will be one packet sent through XDP_REDIRECT and one through
             * XDP_TX; these will show up on the XDP counting program, while the
             * rest will be counted at the TC ingress hook (and the counting program
             * resets the packet payload so they don't get counted twice even though
             * they are re-xmited out the veth device
             */
            ASSERT_EQ((*(*skel).bss).pkts_seen_xdp, 2, c"pkt_count_xdp".as_ptr());
            ASSERT_EQ((*(*skel).bss).pkts_seen_zero, 2, c"pkt_count_zero".as_ptr());
            ASSERT_EQ(
                (*(*skel).bss).pkts_seen_tc,
                NUM_PKTS - 2,
                c"pkt_count_tc".as_ptr(),
            );

            test_max_pkt_size(bpf_program__fd((*skel).progs.xdp_count_pkts));
            break 'out_tc;
        }

        bpf_tc_hook_destroy(&mut tc_hook);
        break 'out;
    }

    if !nstoken.is_null() {
        close_netns(nstoken);
    }
    SYS_NOFAIL!(c"ip netns del testns".as_ptr());
    test_xdp_do_redirect__destroy(skel);
}

const NS_NB: usize = 3;
const NS0: *const i8 = c"NS0".as_ptr();
const NS1: *const i8 = c"NS1".as_ptr();
const NS2: *const i8 = c"NS2".as_ptr();
const IPV4_NETWORK: *const i8 = c"10.1.1".as_ptr();
const VETH1_INDEX: i32 = 111;
const VETH2_INDEX: i32 = 222;

#[repr(C)]
pub struct test_data {
    pub ns: [*mut netns_obj; NS_NB],
    pub xdp_flags: u32,
}

unsafe fn cleanup(data: *mut test_data) {
    let mut i: i32;

    i = 0;
    while i < NS_NB as i32 {
        netns_free((*data).ns[i as usize]);
        i += 1;
    }
}

/**
 * ping_setup -
 * Create two veth peers and forward packets in-between using XDP
 *
 *    ------------           ------------
 *    |    NS1   |           |    NS2   |
 *    |   veth0  |           |   veth0  |
 *    | 10.1.1.1 |           | 10.1.1.2 |
 *    -----|------           ------|-----
 *         |                       |
 *         |                       |
 *    -----|-----------------------|-------
 *    |  veth1                   veth2    |
 *    | (id:111)                (id:222)  |
 *    |    |                        |     |
 *    |    ----- xdp forwarding -----     |
 *    |                                   |
 *    |               NS0                 |
 *    -------------------------------------
 */
unsafe fn ping_setup(data: *mut test_data) -> i32 {
    let mut i: i32;

    (*data).ns[0] = netns_new(NS0, false);
    if !ASSERT_OK_PTR((*data).ns[0] as *const _, c"create ns".as_ptr()) {
        return -1;
    }

    i = 1;
    while i < NS_NB as i32 {
        let mut ns_name = [0i8; 4];

        snprintf(ns_name.as_mut_ptr(), 4, c"NS%d".as_ptr(), i);
        (*data).ns[i as usize] = netns_new(ns_name.as_ptr(), false);
        if !ASSERT_OK_PTR((*data).ns[i as usize] as *const _, c"create ns".as_ptr()) {
            cleanup(data);
            return -1;
        }

        SYS!(
            fail,
            c"ip -n %s link add veth%d index %d%d%d type veth peer name veth0 netns %s".as_ptr(),
            NS0,
            i,
            i,
            i,
            i,
            ns_name.as_ptr()
        );
        SYS!(fail, c"ip -n %s link set veth%d up".as_ptr(), NS0, i);

        SYS!(
            fail,
            c"ip -n %s addr add %s.%d/24 dev veth0".as_ptr(),
            ns_name.as_ptr(),
            IPV4_NETWORK,
            i
        );
        SYS!(fail, c"ip -n %s link set veth0 up".as_ptr(), ns_name.as_ptr());

        i += 1;
    }

    return 0;

    #[allow(unreachable_code)]
    {
        cleanup(data);
        -1
    }
}

unsafe fn ping_test(data: *mut test_data) {
    let mut skel: *mut test_xdp_do_redirect = ptr::null_mut();
    let mut skel_dummy: *mut xdp_dummy = ptr::null_mut();
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut i: i32;
    let mut ret: i32;

    skel_dummy = xdp_dummy__open_and_load();
    if !ASSERT_OK_PTR(skel_dummy as *const _, c"open and load xdp_dummy skeleton".as_ptr()) {
        goto_close(
            &mut nstoken,
            skel_dummy,
            skel,
        );
        return;
    }

    i = 1;
    while i < NS_NB as i32 {
        let mut ns_name = [0i8; 4];

        snprintf(ns_name.as_mut_ptr(), 4, c"NS%d".as_ptr(), i);
        nstoken = open_netns(ns_name.as_ptr());
        if !ASSERT_OK_PTR(nstoken as *const _, c"open ns".as_ptr()) {
            goto_close(&mut nstoken, skel_dummy, skel);
            return;
        }

        ret = bpf_xdp_attach(
            if_nametoindex(c"veth0".as_ptr()) as i32,
            bpf_program__fd((*(*skel_dummy).progs).xdp_dummy_prog),
            (*data).xdp_flags,
            ptr::null_mut(),
        );
        if !ASSERT_GE(ret, 0, c"bpf_xdp_attach dummy_prog".as_ptr()) {
            goto_close(&mut nstoken, skel_dummy, skel);
            return;
        }

        close_netns(nstoken);
        nstoken = ptr::null_mut();
        i += 1;
    }

    skel = test_xdp_do_redirect__open_and_load();
    if !ASSERT_OK_PTR(skel as *const _, c"open and load skeleton".as_ptr()) {
        goto_close(&mut nstoken, skel_dummy, skel);
        return;
    }

    nstoken = open_netns(NS0);
    if !ASSERT_OK_PTR(nstoken as *const _, c"open NS0".as_ptr()) {
        goto_close(&mut nstoken, skel_dummy, skel);
        return;
    }

    ret = bpf_xdp_attach(
        VETH2_INDEX,
        bpf_program__fd((*skel).progs.xdp_redirect_to_111),
        (*data).xdp_flags,
        ptr::null_mut(),
    );
    if !ASSERT_GE(ret, 0, c"bpf_xdp_attach".as_ptr()) {
        goto_close(&mut nstoken, skel_dummy, skel);
        return;
    }

    ret = bpf_xdp_attach(
        VETH1_INDEX,
        bpf_program__fd((*skel).progs.xdp_redirect_to_222),
        (*data).xdp_flags,
        ptr::null_mut(),
    );
    if !ASSERT_GE(ret, 0, c"bpf_xdp_attach".as_ptr()) {
        goto_close(&mut nstoken, skel_dummy, skel);
        return;
    }

    close_netns(nstoken);
    nstoken = ptr::null_mut();

    nstoken = open_netns(NS1);
    if !ASSERT_OK_PTR(nstoken as *const _, c"open NS1".as_ptr()) {
        goto_close(&mut nstoken, skel_dummy, skel);
        return;
    }

    SYS!(close, c"ping -c 1 %s.2 > /dev/null".as_ptr(), IPV4_NETWORK);

    goto_close(&mut nstoken, skel_dummy, skel);
}

unsafe fn goto_close(
    nstoken: *mut *mut nstoken,
    skel_dummy: *mut xdp_dummy,
    skel: *mut test_xdp_do_redirect,
) {
    close_netns(*nstoken);
    xdp_dummy__destroy(skel_dummy);
    test_xdp_do_redirect__destroy(skel);
}

unsafe fn xdp_redirect_ping(xdp_flags: u32) {
    let mut data = test_data {
        ns: [ptr::null_mut(); NS_NB],
        xdp_flags: 0,
    };

    if ping_setup(&mut data) < 0 {
        return;
    }

    data.xdp_flags = xdp_flags;
    ping_test(&mut data);
    cleanup(&mut data);
}

pub unsafe extern "C" fn test_xdp_index_redirect() {
    if test__start_subtest(c"noflag".as_ptr()) {
        xdp_redirect_ping(0);
    }

    if test__start_subtest(c"drvflag".as_ptr()) {
        xdp_redirect_ping(XDP_FLAGS_DRV_MODE);
    }

    if test__start_subtest(c"skbflag".as_ptr()) {
        xdp_redirect_ping(XDP_FLAGS_SKB_MODE);
    }
}
