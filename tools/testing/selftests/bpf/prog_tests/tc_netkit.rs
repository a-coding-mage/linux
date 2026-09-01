// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */
// Dependencies from the original C file:
// <uapi/linux/if_link.h>, <net/if.h>, <test_progs.h>,
// "test_tc_link.skel.h", "netlink_helpers.h", "tc_helpers.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

const netkit_peer: &[u8] = b"nk0\0";
const netkit_name: &[u8] = b"nk1\0";

const ping_addr_neigh: __u32 = 0x0a000002; /* 10.0.0.2 */
const ping_addr_noneigh: __u32 = 0x0a000003; /* 10.0.0.3 */

const NETKIT_HEADROOM: c_int = 32;
const NETKIT_TAILROOM: c_int = 8;

const MARK: c_int = 42;
const PRIO: c_int = 0xeb9f;
const ICMP_ECHO: __u8 = 8;

const FLAG_ADJUST_ROOM: __u32 = 1 << 0;
const FLAG_SAME_NETNS: __u32 = 1 << 1;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __be16 = u16;
type __sum16 = u16;

#[repr(C)]
struct icmphdr_echo {
    id: __be16,
    sequence: __be16,
}

#[repr(C)]
struct icmphdr {
    type_: __u8,
    code: __u8,
    checksum: __sum16,
    echo: icmphdr_echo,
}

#[repr(C)]
struct iplink_req {
    n: nlmsghdr,
    i: ifinfomsg,
    buf: [c_char; 1024],
}

unsafe fn create_netkit(
    mode: c_int,
    policy: c_int,
    peer_policy: c_int,
    ifindex: *mut c_int,
    scrub: c_int,
    peer_scrub: c_int,
    flags: __u32,
) -> c_int {
    let mut rth: rtnl_handle = zeroed();
    rth.fd = -1;
    let mut req: iplink_req = zeroed();
    let mut linkinfo: *mut rtattr;
    let mut data: *mut rtattr;
    let type_: *const c_char = c"netkit".as_ptr();
    let mut err: c_int;

    err = rtnl_open(&mut rth, 0);
    if !ASSERT_OK(err, c"open_rtnetlink".as_ptr()) {
        return err;
    }

    req = zeroed();
    req.n.nlmsg_len = NLMSG_LENGTH(size_of::<ifinfomsg>()) as _;
    req.n.nlmsg_flags = (NLM_F_REQUEST | NLM_F_CREATE | NLM_F_EXCL) as _;
    req.n.nlmsg_type = RTM_NEWLINK as _;
    req.i.ifi_family = AF_UNSPEC as _;

    addattr_l(
        &mut req.n,
        size_of::<iplink_req>(),
        IFLA_IFNAME,
        netkit_name.as_ptr() as *const c_void,
        strlen(netkit_name.as_ptr() as *const c_char),
    );
    linkinfo = addattr_nest(&mut req.n, size_of::<iplink_req>(), IFLA_LINKINFO);
    addattr_l(
        &mut req.n,
        size_of::<iplink_req>(),
        IFLA_INFO_KIND,
        type_ as *const c_void,
        strlen(type_),
    );
    data = addattr_nest(&mut req.n, size_of::<iplink_req>(), IFLA_INFO_DATA);
    addattr32(&mut req.n, size_of::<iplink_req>(), IFLA_NETKIT_POLICY, policy as __u32);
    addattr32(
        &mut req.n,
        size_of::<iplink_req>(),
        IFLA_NETKIT_PEER_POLICY,
        peer_policy as __u32,
    );
    addattr32(&mut req.n, size_of::<iplink_req>(), IFLA_NETKIT_SCRUB, scrub as __u32);
    addattr32(
        &mut req.n,
        size_of::<iplink_req>(),
        IFLA_NETKIT_PEER_SCRUB,
        peer_scrub as __u32,
    );
    addattr32(&mut req.n, size_of::<iplink_req>(), IFLA_NETKIT_MODE, mode as __u32);
    if flags & FLAG_ADJUST_ROOM != 0 {
        addattr16(
            &mut req.n,
            size_of::<iplink_req>(),
            IFLA_NETKIT_HEADROOM,
            NETKIT_HEADROOM as __u16,
        );
        addattr16(
            &mut req.n,
            size_of::<iplink_req>(),
            IFLA_NETKIT_TAILROOM,
            NETKIT_TAILROOM as __u16,
        );
    }
    addattr_nest_end(&mut req.n, data);
    addattr_nest_end(&mut req.n, linkinfo);

    err = rtnl_talk(&mut rth, &mut req.n, null_mut());
    ASSERT_OK(err, c"talk_rtnetlink".as_ptr());
    rtnl_close(&mut rth);
    *ifindex = if_nametoindex(netkit_name.as_ptr() as *const c_char) as c_int;

    ASSERT_GT(*ifindex, 0, c"retrieve_ifindex".as_ptr());
    ASSERT_OK(system(c"ip netns add foo".as_ptr()), c"create netns".as_ptr());
    ASSERT_OK(system(c"ip link set dev nk1 up".as_ptr()), c"up primary".as_ptr());
    ASSERT_OK(
        system(c"ip addr add dev nk1 10.0.0.1/24".as_ptr()),
        c"addr primary".as_ptr(),
    );

    if mode == NETKIT_L3 {
        ASSERT_EQ(
            system(c"ip link set dev nk1 addr ee:ff:bb:cc:aa:dd 2> /dev/null".as_ptr()),
            512,
            c"set hwaddress".as_ptr(),
        );
    } else {
        ASSERT_OK(
            system(c"ip link set dev nk1 addr ee:ff:bb:cc:aa:dd".as_ptr()),
            c"set hwaddress".as_ptr(),
        );
    }
    if flags & FLAG_SAME_NETNS != 0 {
        ASSERT_OK(system(c"ip link set dev nk0 up".as_ptr()), c"up peer".as_ptr());
        ASSERT_OK(
            system(c"ip addr add dev nk0 10.0.0.2/24".as_ptr()),
            c"addr peer".as_ptr(),
        );
    } else {
        ASSERT_OK(system(c"ip link set nk0 netns foo".as_ptr()), c"move peer".as_ptr());
        ASSERT_OK(
            system(c"ip netns exec foo ip link set dev nk0 up".as_ptr()),
            c"up peer".as_ptr(),
        );
        ASSERT_OK(
            system(c"ip netns exec foo ip addr add dev nk0 10.0.0.2/24".as_ptr()),
            c"addr peer".as_ptr(),
        );
    }
    err
}

unsafe fn move_netkit() {
    ASSERT_OK(system(c"ip link set nk0 netns foo".as_ptr()), c"move peer".as_ptr());
    ASSERT_OK(
        system(c"ip netns exec foo ip link set dev nk0 up".as_ptr()),
        c"up peer".as_ptr(),
    );
    ASSERT_OK(
        system(c"ip netns exec foo ip addr add dev nk0 10.0.0.2/24".as_ptr()),
        c"addr peer".as_ptr(),
    );
}

unsafe fn destroy_netkit() {
    ASSERT_OK(system(c"ip link del dev nk1".as_ptr()), c"del primary".as_ptr());
    ASSERT_OK(system(c"ip netns del foo".as_ptr()), c"delete netns".as_ptr());
    ASSERT_EQ(
        if_nametoindex(netkit_name.as_ptr() as *const c_char) as c_int,
        0,
        c"nk1_ifindex".as_ptr(),
    );
}

unsafe fn __send_icmp(dest: __u32) -> c_int {
    let mut sock: c_int;
    let mut ret: c_int;
    let mark: c_int = MARK;
    let prio: c_int = PRIO;
    let mut addr: sockaddr_in = zeroed();
    let mut icmp: icmphdr = zeroed();

    ret = write_sysctl(c"/proc/sys/net/ipv4/ping_group_range".as_ptr(), c"0 0".as_ptr());
    if !ASSERT_OK(ret, c"write_sysctl(net.ipv4.ping_group_range)".as_ptr()) {
        return ret;
    }

    sock = socket(AF_INET, SOCK_DGRAM, IPPROTO_ICMP);
    if !ASSERT_GE(sock, 0, c"icmp_socket".as_ptr()) {
        return -errno();
    }

    ret = setsockopt(
        sock,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        netkit_name.as_ptr() as *const c_void,
        strlen(netkit_name.as_ptr() as *const c_char) + 1,
    );
    if !ASSERT_OK(ret, c"setsockopt(SO_BINDTODEVICE)".as_ptr()) {
        close(sock);
        return ret;
    }

    ret = setsockopt(
        sock,
        SOL_SOCKET,
        SO_MARK,
        &mark as *const _ as *const c_void,
        size_of::<c_int>(),
    );
    if !ASSERT_OK(ret, c"setsockopt(SO_MARK)".as_ptr()) {
        close(sock);
        return ret;
    }

    ret = setsockopt(
        sock,
        SOL_SOCKET,
        SO_PRIORITY,
        &prio as *const _ as *const c_void,
        size_of::<c_int>(),
    );
    if !ASSERT_OK(ret, c"setsockopt(SO_PRIORITY)".as_ptr()) {
        close(sock);
        return ret;
    }

    addr = zeroed();
    addr.sin_family = AF_INET as _;
    addr.sin_addr.s_addr = htonl(dest);

    icmp = zeroed();
    icmp.type_ = ICMP_ECHO;
    icmp.echo.id = 1234;
    icmp.echo.sequence = 1;

    ret = sendto(
        sock,
        &icmp as *const _ as *const c_void,
        size_of::<icmphdr>(),
        0,
        &addr as *const _ as *const sockaddr,
        size_of::<sockaddr_in>(),
    );
    if !ASSERT_GE(ret, 0, c"icmp_sendto".as_ptr()) {
        ret = -errno();
    } else {
        ret = 0;
    }
    close(sock);
    ret
}

unsafe fn send_icmp() -> c_int {
    __send_icmp(ping_addr_neigh)
}

pub unsafe fn serial_test_tc_netkit_basic() {
    let mut optq: bpf_prog_query_opts = zeroed();
    let mut optl: bpf_netkit_opts = zeroed();
    let mut prog_ids: [__u32; 2] = [0; 2];
    let mut link_ids: [__u32; 2] = [0; 2];
    let mut pid1: __u32;
    let mut pid2: __u32;
    let mut lid1: __u32;
    let mut lid2: __u32;
    let mut skel: *mut test_tc_link;
    let mut link: *mut bpf_link;
    let mut err: c_int;
    let mut ifindex: c_int = 0;

    err = create_netkit(
        NETKIT_L2,
        NETKIT_PASS,
        NETKIT_PASS,
        &mut ifindex,
        NETKIT_SCRUB_DEFAULT,
        NETKIT_SCRUB_DEFAULT,
        0,
    );
    if err != 0 {
        return;
    }

    skel = test_tc_link__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        destroy_netkit();
        return;
    }

    'cleanup: {
        ASSERT_EQ(
            bpf_program__set_expected_attach_type((*skel).progs.tc1, BPF_NETKIT_PRIMARY),
            0,
            c"tc1_attach_type".as_ptr(),
        );
        ASSERT_EQ(
            bpf_program__set_expected_attach_type((*skel).progs.tc2, BPF_NETKIT_PEER),
            0,
            c"tc2_attach_type".as_ptr(),
        );

        err = test_tc_link__load(skel);
        if !ASSERT_OK(err, c"skel_load".as_ptr()) {
            break 'cleanup;
        }

        pid1 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc1));
        pid2 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc2));
        ASSERT_NEQ(pid1, pid2, c"prog_ids_1_2".as_ptr());

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 0);
        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);

        ASSERT_EQ((*(*skel).bss).seen_tc1, false, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc2, false, c"seen_tc2".as_ptr());

        link = bpf_program__attach_netkit((*skel).progs.tc1, ifindex, &mut optl);
        if !ASSERT_OK_PTR(link as *const c_void, c"link_attach".as_ptr()) {
            break 'cleanup;
        }
        (*skel).links.tc1 = link;

        lid1 = id_from_link_fd(bpf_link__fd((*skel).links.tc1));
        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 1);
        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);

        optq.prog_ids = prog_ids.as_mut_ptr();
        optq.link_ids = link_ids.as_mut_ptr();
        prog_ids = [0; 2];
        link_ids = [0; 2];
        optq.count = prog_ids.len() as __u32;

        err = bpf_prog_query_opts(ifindex, BPF_NETKIT_PRIMARY, &mut optq);
        if !ASSERT_OK(err, c"prog_query".as_ptr()) {
            break 'cleanup;
        }

        ASSERT_EQ(optq.count, 1, c"count".as_ptr());
        ASSERT_EQ(optq.revision, 2, c"revision".as_ptr());
        ASSERT_EQ(prog_ids[0], pid1, c"prog_ids[0]".as_ptr());
        ASSERT_EQ(link_ids[0], lid1, c"link_ids[0]".as_ptr());
        ASSERT_EQ(prog_ids[1], 0, c"prog_ids[1]".as_ptr());
        ASSERT_EQ(link_ids[1], 0, c"link_ids[1]".as_ptr());

        tc_skel_reset_all_seen(skel);
        ASSERT_EQ(send_icmp(), 0, c"icmp_pkt".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc1, true, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc2, false, c"seen_tc2".as_ptr());

        link = bpf_program__attach_netkit((*skel).progs.tc2, ifindex, &mut optl);
        if !ASSERT_OK_PTR(link as *const c_void, c"link_attach".as_ptr()) {
            break 'cleanup;
        }
        (*skel).links.tc2 = link;

        lid2 = id_from_link_fd(bpf_link__fd((*skel).links.tc2));
        ASSERT_NEQ(lid1, lid2, c"link_ids_1_2".as_ptr());

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 1);
        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 1);

        prog_ids = [0; 2];
        link_ids = [0; 2];
        optq.count = prog_ids.len() as __u32;

        err = bpf_prog_query_opts(ifindex, BPF_NETKIT_PEER, &mut optq);
        if !ASSERT_OK(err, c"prog_query".as_ptr()) {
            break 'cleanup;
        }

        ASSERT_EQ(optq.count, 1, c"count".as_ptr());
        ASSERT_EQ(optq.revision, 2, c"revision".as_ptr());
        ASSERT_EQ(prog_ids[0], pid2, c"prog_ids[0]".as_ptr());
        ASSERT_EQ(link_ids[0], lid2, c"link_ids[0]".as_ptr());
        ASSERT_EQ(prog_ids[1], 0, c"prog_ids[1]".as_ptr());
        ASSERT_EQ(link_ids[1], 0, c"link_ids[1]".as_ptr());

        tc_skel_reset_all_seen(skel);
        ASSERT_EQ(send_icmp(), 0, c"icmp_pkt".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc1, true, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc2, true, c"seen_tc2".as_ptr());
    }
    test_tc_link__destroy(skel);

    assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 0);
    assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);
    destroy_netkit();
}

unsafe fn serial_test_tc_netkit_multi_links_target(mode: c_int, target: c_int) {
    let mut optq: bpf_prog_query_opts = zeroed();
    let mut optl: bpf_netkit_opts = zeroed();
    let mut prog_ids: [__u32; 3] = [0; 3];
    let mut link_ids: [__u32; 3] = [0; 3];
    let (mut pid1, mut pid2, mut lid1, mut lid2): (__u32, __u32, __u32, __u32);
    let mut skel: *mut test_tc_link;
    let mut link: *mut bpf_link;
    let mut err: c_int;
    let mut ifindex: c_int = 0;

    err = create_netkit(mode, NETKIT_PASS, NETKIT_PASS, &mut ifindex, NETKIT_SCRUB_DEFAULT, NETKIT_SCRUB_DEFAULT, 0);
    if err != 0 {
        return;
    }

    skel = test_tc_link__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        destroy_netkit();
        return;
    }

    'cleanup: {
        ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc1, target), 0, c"tc1_attach_type".as_ptr());
        ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc2, target), 0, c"tc2_attach_type".as_ptr());

        err = test_tc_link__load(skel);
        if !ASSERT_OK(err, c"skel_load".as_ptr()) {
            break 'cleanup;
        }

        pid1 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc1));
        pid2 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc2));
        ASSERT_NEQ(pid1, pid2, c"prog_ids_1_2".as_ptr());

        assert_mprog_count_ifindex(ifindex, target, 0);
        ASSERT_EQ((*(*skel).bss).seen_tc1, false, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_eth, false, c"seen_eth".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc2, false, c"seen_tc2".as_ptr());

        link = bpf_program__attach_netkit((*skel).progs.tc1, ifindex, &mut optl);
        if !ASSERT_OK_PTR(link as *const c_void, c"link_attach".as_ptr()) {
            break 'cleanup;
        }
        (*skel).links.tc1 = link;
        lid1 = id_from_link_fd(bpf_link__fd((*skel).links.tc1));

        assert_mprog_count_ifindex(ifindex, target, 1);
        optq.prog_ids = prog_ids.as_mut_ptr();
        optq.link_ids = link_ids.as_mut_ptr();
        prog_ids = [0; 3];
        link_ids = [0; 3];
        optq.count = prog_ids.len() as __u32;

        err = bpf_prog_query_opts(ifindex, target, &mut optq);
        if !ASSERT_OK(err, c"prog_query".as_ptr()) {
            break 'cleanup;
        }

        ASSERT_EQ(optq.count, 1, c"count".as_ptr());
        ASSERT_EQ(optq.revision, 2, c"revision".as_ptr());
        ASSERT_EQ(prog_ids[0], pid1, c"prog_ids[0]".as_ptr());
        ASSERT_EQ(link_ids[0], lid1, c"link_ids[0]".as_ptr());
        ASSERT_EQ(prog_ids[1], 0, c"prog_ids[1]".as_ptr());
        ASSERT_EQ(link_ids[1], 0, c"link_ids[1]".as_ptr());

        tc_skel_reset_all_seen(skel);
        ASSERT_EQ(send_icmp(), 0, c"icmp_pkt".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc1, true, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_eth, true, c"seen_eth".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc2, false, c"seen_tc2".as_ptr());

        optl = zeroed();
        optl.flags = BPF_F_BEFORE;
        optl.relative_fd = bpf_program__fd((*skel).progs.tc1);

        link = bpf_program__attach_netkit((*skel).progs.tc2, ifindex, &mut optl);
        if !ASSERT_OK_PTR(link as *const c_void, c"link_attach".as_ptr()) {
            break 'cleanup;
        }
        (*skel).links.tc2 = link;

        lid2 = id_from_link_fd(bpf_link__fd((*skel).links.tc2));
        ASSERT_NEQ(lid1, lid2, c"link_ids_1_2".as_ptr());

        assert_mprog_count_ifindex(ifindex, target, 2);
        prog_ids = [0; 3];
        link_ids = [0; 3];
        optq.count = prog_ids.len() as __u32;

        err = bpf_prog_query_opts(ifindex, target, &mut optq);
        if !ASSERT_OK(err, c"prog_query".as_ptr()) {
            break 'cleanup;
        }

        ASSERT_EQ(optq.count, 2, c"count".as_ptr());
        ASSERT_EQ(optq.revision, 3, c"revision".as_ptr());
        ASSERT_EQ(prog_ids[0], pid2, c"prog_ids[0]".as_ptr());
        ASSERT_EQ(link_ids[0], lid2, c"link_ids[0]".as_ptr());
        ASSERT_EQ(prog_ids[1], pid1, c"prog_ids[1]".as_ptr());
        ASSERT_EQ(link_ids[1], lid1, c"link_ids[1]".as_ptr());
        ASSERT_EQ(prog_ids[2], 0, c"prog_ids[2]".as_ptr());
        ASSERT_EQ(link_ids[2], 0, c"link_ids[2]".as_ptr());

        tc_skel_reset_all_seen(skel);
        ASSERT_EQ(send_icmp(), 0, c"icmp_pkt".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc1, true, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_eth, true, c"seen_eth".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc2, true, c"seen_tc2".as_ptr());
    }
    test_tc_link__destroy(skel);

    assert_mprog_count_ifindex(ifindex, target, 0);
    destroy_netkit();
}

pub unsafe fn serial_test_tc_netkit_multi_links() {
    serial_test_tc_netkit_multi_links_target(NETKIT_L2, BPF_NETKIT_PRIMARY);
    serial_test_tc_netkit_multi_links_target(NETKIT_L3, BPF_NETKIT_PRIMARY);
    serial_test_tc_netkit_multi_links_target(NETKIT_L2, BPF_NETKIT_PEER);
    serial_test_tc_netkit_multi_links_target(NETKIT_L3, BPF_NETKIT_PEER);
}

unsafe fn serial_test_tc_netkit_multi_opts_target(mode: c_int, target: c_int) {
    let mut opta: bpf_prog_attach_opts = zeroed();
    let mut optd: bpf_prog_detach_opts = zeroed();
    let mut optq: bpf_prog_query_opts = zeroed();
    let (mut pid1, mut pid2, mut fd1, mut fd2): (__u32, __u32, __u32, __u32);
    let mut prog_ids: [__u32; 3] = [0; 3];
    let mut skel: *mut test_tc_link;
    let mut err: c_int;
    let mut ifindex: c_int = 0;

    err = create_netkit(mode, NETKIT_PASS, NETKIT_PASS, &mut ifindex, NETKIT_SCRUB_DEFAULT, NETKIT_SCRUB_DEFAULT, 0);
    if err != 0 {
        return;
    }

    skel = test_tc_link__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_load".as_ptr()) {
        destroy_netkit();
        return;
    }

    fd1 = bpf_program__fd((*skel).progs.tc1) as __u32;
    fd2 = bpf_program__fd((*skel).progs.tc2) as __u32;
    pid1 = id_from_prog_fd(fd1 as c_int);
    pid2 = id_from_prog_fd(fd2 as c_int);
    ASSERT_NEQ(pid1, pid2, c"prog_ids_1_2".as_ptr());

    'cleanup: {
        assert_mprog_count_ifindex(ifindex, target, 0);
        ASSERT_EQ((*(*skel).bss).seen_tc1, false, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_eth, false, c"seen_eth".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc2, false, c"seen_tc2".as_ptr());

        err = bpf_prog_attach_opts(fd1 as c_int, ifindex, target, &mut opta);
        if !ASSERT_EQ(err, 0, c"prog_attach".as_ptr()) {
            break 'cleanup;
        }

        'cleanup_fd1: {
            assert_mprog_count_ifindex(ifindex, target, 1);
            optq.prog_ids = prog_ids.as_mut_ptr();
            prog_ids = [0; 3];
            optq.count = prog_ids.len() as __u32;

            err = bpf_prog_query_opts(ifindex, target, &mut optq);
            if !ASSERT_OK(err, c"prog_query".as_ptr()) {
                break 'cleanup_fd1;
            }

            ASSERT_EQ(optq.count, 1, c"count".as_ptr());
            ASSERT_EQ(optq.revision, 2, c"revision".as_ptr());
            ASSERT_EQ(prog_ids[0], pid1, c"prog_ids[0]".as_ptr());
            ASSERT_EQ(prog_ids[1], 0, c"prog_ids[1]".as_ptr());

            tc_skel_reset_all_seen(skel);
            ASSERT_EQ(send_icmp(), 0, c"icmp_pkt".as_ptr());
            ASSERT_EQ((*(*skel).bss).seen_tc1, true, c"seen_tc1".as_ptr());
            ASSERT_EQ((*(*skel).bss).seen_eth, true, c"seen_eth".as_ptr());
            ASSERT_EQ((*(*skel).bss).seen_tc2, false, c"seen_tc2".as_ptr());

            opta = zeroed();
            opta.flags = BPF_F_BEFORE;
            opta.relative_fd = fd1 as c_int;

            err = bpf_prog_attach_opts(fd2 as c_int, ifindex, target, &mut opta);
            if !ASSERT_EQ(err, 0, c"prog_attach".as_ptr()) {
                break 'cleanup_fd1;
            }

            'cleanup_fd2: {
                assert_mprog_count_ifindex(ifindex, target, 2);
                prog_ids = [0; 3];
                optq.count = prog_ids.len() as __u32;

                err = bpf_prog_query_opts(ifindex, target, &mut optq);
                if !ASSERT_OK(err, c"prog_query".as_ptr()) {
                    break 'cleanup_fd2;
                }

                ASSERT_EQ(optq.count, 2, c"count".as_ptr());
                ASSERT_EQ(optq.revision, 3, c"revision".as_ptr());
                ASSERT_EQ(prog_ids[0], pid2, c"prog_ids[0]".as_ptr());
                ASSERT_EQ(prog_ids[1], pid1, c"prog_ids[1]".as_ptr());
                ASSERT_EQ(prog_ids[2], 0, c"prog_ids[2]".as_ptr());

                tc_skel_reset_all_seen(skel);
                ASSERT_EQ(send_icmp(), 0, c"icmp_pkt".as_ptr());
                ASSERT_EQ((*(*skel).bss).seen_tc1, true, c"seen_tc1".as_ptr());
                ASSERT_EQ((*(*skel).bss).seen_eth, true, c"seen_eth".as_ptr());
                ASSERT_EQ((*(*skel).bss).seen_tc2, true, c"seen_tc2".as_ptr());
            }
            err = bpf_prog_detach_opts(fd2 as c_int, ifindex, target, &mut optd);
            ASSERT_OK(err, c"prog_detach".as_ptr());
            assert_mprog_count_ifindex(ifindex, target, 1);
        }
        err = bpf_prog_detach_opts(fd1 as c_int, ifindex, target, &mut optd);
        ASSERT_OK(err, c"prog_detach".as_ptr());
        assert_mprog_count_ifindex(ifindex, target, 0);
    }
    test_tc_link__destroy(skel);

    assert_mprog_count_ifindex(ifindex, target, 0);
    destroy_netkit();
}

pub unsafe fn serial_test_tc_netkit_multi_opts() {
    serial_test_tc_netkit_multi_opts_target(NETKIT_L2, BPF_NETKIT_PRIMARY);
    serial_test_tc_netkit_multi_opts_target(NETKIT_L3, BPF_NETKIT_PRIMARY);
    serial_test_tc_netkit_multi_opts_target(NETKIT_L2, BPF_NETKIT_PEER);
    serial_test_tc_netkit_multi_opts_target(NETKIT_L3, BPF_NETKIT_PEER);
}

pub unsafe fn serial_test_tc_netkit_device() {
    let mut optq: bpf_prog_query_opts = zeroed();
    let mut optl: bpf_netkit_opts = zeroed();
    let mut prog_ids: [__u32; 2] = [0; 2];
    let mut link_ids: [__u32; 2] = [0; 2];
    let (mut pid1, mut pid2, mut lid1): (__u32, __u32, __u32);
    let mut skel: *mut test_tc_link;
    let mut link: *mut bpf_link;
    let mut err: c_int;
    let mut ifindex: c_int = 0;
    let mut ifindex2: c_int;

    err = create_netkit(NETKIT_L3, NETKIT_PASS, NETKIT_PASS, &mut ifindex, NETKIT_SCRUB_DEFAULT, NETKIT_SCRUB_DEFAULT, FLAG_SAME_NETNS);
    if err != 0 {
        return;
    }

    ifindex2 = if_nametoindex(netkit_peer.as_ptr() as *const c_char) as c_int;
    ASSERT_NEQ(ifindex, ifindex2, c"ifindex_1_2".as_ptr());

    skel = test_tc_link__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        destroy_netkit();
        return;
    }

    'cleanup: {
        ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc1, BPF_NETKIT_PRIMARY), 0, c"tc1_attach_type".as_ptr());
        ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc2, BPF_NETKIT_PEER), 0, c"tc2_attach_type".as_ptr());
        ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc3, BPF_NETKIT_PRIMARY), 0, c"tc3_attach_type".as_ptr());

        err = test_tc_link__load(skel);
        if !ASSERT_OK(err, c"skel_load".as_ptr()) {
            break 'cleanup;
        }

        pid1 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc1));
        pid2 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc2));
        ASSERT_NEQ(pid1, pid2, c"prog_ids_1_2".as_ptr());

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 0);
        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);
        ASSERT_EQ((*(*skel).bss).seen_tc1, false, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc2, false, c"seen_tc2".as_ptr());

        link = bpf_program__attach_netkit((*skel).progs.tc1, ifindex, &mut optl);
        if !ASSERT_OK_PTR(link as *const c_void, c"link_attach".as_ptr()) {
            break 'cleanup;
        }
        (*skel).links.tc1 = link;
        lid1 = id_from_link_fd(bpf_link__fd((*skel).links.tc1));

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 1);
        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);

        optq.prog_ids = prog_ids.as_mut_ptr();
        optq.link_ids = link_ids.as_mut_ptr();
        prog_ids = [0; 2];
        link_ids = [0; 2];
        optq.count = prog_ids.len() as __u32;

        err = bpf_prog_query_opts(ifindex, BPF_NETKIT_PRIMARY, &mut optq);
        if !ASSERT_OK(err, c"prog_query".as_ptr()) {
            break 'cleanup;
        }

        ASSERT_EQ(optq.count, 1, c"count".as_ptr());
        ASSERT_EQ(optq.revision, 2, c"revision".as_ptr());
        ASSERT_EQ(prog_ids[0], pid1, c"prog_ids[0]".as_ptr());
        ASSERT_EQ(link_ids[0], lid1, c"link_ids[0]".as_ptr());
        ASSERT_EQ(prog_ids[1], 0, c"prog_ids[1]".as_ptr());
        ASSERT_EQ(link_ids[1], 0, c"link_ids[1]".as_ptr());

        tc_skel_reset_all_seen(skel);
        ASSERT_EQ(send_icmp(), 0, c"icmp_pkt".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc1, true, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc2, false, c"seen_tc2".as_ptr());

        prog_ids = [0; 2];
        link_ids = [0; 2];
        optq.count = prog_ids.len() as __u32;

        err = bpf_prog_query_opts(ifindex2, BPF_NETKIT_PRIMARY, &mut optq);
        ASSERT_EQ(err, -EACCES, c"prog_query_should_fail".as_ptr());

        err = bpf_prog_query_opts(ifindex2, BPF_NETKIT_PEER, &mut optq);
        ASSERT_EQ(err, -EACCES, c"prog_query_should_fail".as_ptr());

        link = bpf_program__attach_netkit((*skel).progs.tc2, ifindex2, &mut optl);
        if !ASSERT_ERR_PTR(link as *const c_void, c"link_attach_should_fail".as_ptr()) {
            bpf_link__destroy(link);
            break 'cleanup;
        }

        link = bpf_program__attach_netkit((*skel).progs.tc3, ifindex2, &mut optl);
        if !ASSERT_ERR_PTR(link as *const c_void, c"link_attach_should_fail".as_ptr()) {
            bpf_link__destroy(link);
            break 'cleanup;
        }

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 1);
        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);
    }
    test_tc_link__destroy(skel);

    assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 0);
    assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);
    destroy_netkit();
}

unsafe fn serial_test_tc_netkit_neigh_links_target(mode: c_int, target: c_int) {
    let mut optq: bpf_prog_query_opts = zeroed();
    let mut optl: bpf_netkit_opts = zeroed();
    let mut prog_ids: [__u32; 2] = [0; 2];
    let mut link_ids: [__u32; 2] = [0; 2];
    let (mut pid1, mut lid1): (__u32, __u32);
    let mut skel: *mut test_tc_link;
    let mut link: *mut bpf_link;
    let mut err: c_int;
    let mut ifindex: c_int = 0;

    err = create_netkit(mode, NETKIT_PASS, NETKIT_PASS, &mut ifindex, NETKIT_SCRUB_DEFAULT, NETKIT_SCRUB_DEFAULT, 0);
    if err != 0 {
        return;
    }

    skel = test_tc_link__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        destroy_netkit();
        return;
    }

    'cleanup: {
        ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc1, BPF_NETKIT_PRIMARY), 0, c"tc1_attach_type".as_ptr());

        err = test_tc_link__load(skel);
        if !ASSERT_OK(err, c"skel_load".as_ptr()) {
            break 'cleanup;
        }

        pid1 = id_from_prog_fd(bpf_program__fd((*skel).progs.tc1));
        assert_mprog_count_ifindex(ifindex, target, 0);

        ASSERT_EQ((*(*skel).bss).seen_tc1, false, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_eth, false, c"seen_eth".as_ptr());

        link = bpf_program__attach_netkit((*skel).progs.tc1, ifindex, &mut optl);
        if !ASSERT_OK_PTR(link as *const c_void, c"link_attach".as_ptr()) {
            break 'cleanup;
        }
        (*skel).links.tc1 = link;
        lid1 = id_from_link_fd(bpf_link__fd((*skel).links.tc1));

        assert_mprog_count_ifindex(ifindex, target, 1);
        optq.prog_ids = prog_ids.as_mut_ptr();
        optq.link_ids = link_ids.as_mut_ptr();
        prog_ids = [0; 2];
        link_ids = [0; 2];
        optq.count = prog_ids.len() as __u32;

        err = bpf_prog_query_opts(ifindex, target, &mut optq);
        if !ASSERT_OK(err, c"prog_query".as_ptr()) {
            break 'cleanup;
        }

        ASSERT_EQ(optq.count, 1, c"count".as_ptr());
        ASSERT_EQ(optq.revision, 2, c"revision".as_ptr());
        ASSERT_EQ(prog_ids[0], pid1, c"prog_ids[0]".as_ptr());
        ASSERT_EQ(link_ids[0], lid1, c"link_ids[0]".as_ptr());
        ASSERT_EQ(prog_ids[1], 0, c"prog_ids[1]".as_ptr());
        ASSERT_EQ(link_ids[1], 0, c"link_ids[1]".as_ptr());

        tc_skel_reset_all_seen(skel);
        ASSERT_EQ(__send_icmp(ping_addr_noneigh), 0, c"icmp_pkt".as_ptr());

        ASSERT_EQ((*(*skel).bss).seen_tc1, true, c"seen_tc1".as_ptr()); /* L2: ARP */
        ASSERT_EQ((*(*skel).bss).seen_eth, mode == NETKIT_L3, c"seen_eth".as_ptr());
    }
    test_tc_link__destroy(skel);

    assert_mprog_count_ifindex(ifindex, target, 0);
    destroy_netkit();
}

pub unsafe fn serial_test_tc_netkit_neigh_links() {
    serial_test_tc_netkit_neigh_links_target(NETKIT_L2, BPF_NETKIT_PRIMARY);
    serial_test_tc_netkit_neigh_links_target(NETKIT_L3, BPF_NETKIT_PRIMARY);
}

unsafe fn serial_test_tc_netkit_pkt_type_mode(mode: c_int) {
    let mut optl_nk: bpf_netkit_opts = zeroed();
    let mut optl_tcx: bpf_tcx_opts = zeroed();
    let mut err: c_int;
    let mut ifindex: c_int = 0;
    let mut ifindex2: c_int;
    let mut skel: *mut test_tc_link;
    let mut link: *mut bpf_link;

    err = create_netkit(mode, NETKIT_PASS, NETKIT_PASS, &mut ifindex, NETKIT_SCRUB_DEFAULT, NETKIT_SCRUB_DEFAULT, FLAG_SAME_NETNS);
    if err != 0 {
        return;
    }

    ifindex2 = if_nametoindex(netkit_peer.as_ptr() as *const c_char) as c_int;
    ASSERT_NEQ(ifindex, ifindex2, c"ifindex_1_2".as_ptr());

    skel = test_tc_link__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        destroy_netkit();
        return;
    }

    'cleanup: {
        ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc1, BPF_NETKIT_PRIMARY), 0, c"tc1_attach_type".as_ptr());
        ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc7, BPF_TCX_INGRESS), 0, c"tc7_attach_type".as_ptr());

        err = test_tc_link__load(skel);
        if !ASSERT_OK(err, c"skel_load".as_ptr()) {
            break 'cleanup;
        }

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 0);
        assert_mprog_count_ifindex(ifindex2, BPF_TCX_INGRESS, 0);

        link = bpf_program__attach_netkit((*skel).progs.tc1, ifindex, &mut optl_nk);
        if !ASSERT_OK_PTR(link as *const c_void, c"link_attach".as_ptr()) {
            break 'cleanup;
        }
        (*skel).links.tc1 = link;

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 1);
        assert_mprog_count_ifindex(ifindex2, BPF_TCX_INGRESS, 0);

        link = bpf_program__attach_tcx((*skel).progs.tc7, ifindex2, &mut optl_tcx);
        if !ASSERT_OK_PTR(link as *const c_void, c"link_attach".as_ptr()) {
            break 'cleanup;
        }
        (*skel).links.tc7 = link;

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 1);
        assert_mprog_count_ifindex(ifindex2, BPF_TCX_INGRESS, 1);

        move_netkit();

        tc_skel_reset_all_seen(skel);
        (*(*skel).bss).set_type = true;
        ASSERT_EQ(send_icmp(), 0, c"icmp_pkt".as_ptr());

        ASSERT_EQ((*(*skel).bss).seen_tc1, true, c"seen_tc1".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_tc7, true, c"seen_tc7".as_ptr());

        ASSERT_EQ((*(*skel).bss).seen_host, true, c"seen_host".as_ptr());
        ASSERT_EQ((*(*skel).bss).seen_mcast, true, c"seen_mcast".as_ptr());
    }
    test_tc_link__destroy(skel);

    assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 0);
    destroy_netkit();
}

pub unsafe fn serial_test_tc_netkit_pkt_type() {
    serial_test_tc_netkit_pkt_type_mode(NETKIT_L2);
    serial_test_tc_netkit_pkt_type_mode(NETKIT_L3);
}

unsafe fn serial_test_tc_netkit_scrub_type(scrub: c_int, room: bool) {
    let mut optl: bpf_netkit_opts = zeroed();
    let mut skel: *mut test_tc_link;
    let mut link: *mut bpf_link;
    let mut err: c_int;
    let mut ifindex: c_int = 0;

    err = create_netkit(
        NETKIT_L2,
        NETKIT_PASS,
        NETKIT_PASS,
        &mut ifindex,
        scrub,
        scrub,
        if room { FLAG_ADJUST_ROOM } else { 0 },
    );
    if err != 0 {
        return;
    }

    skel = test_tc_link__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        destroy_netkit();
        return;
    }

    'cleanup: {
        ASSERT_EQ(bpf_program__set_expected_attach_type((*skel).progs.tc8, BPF_NETKIT_PRIMARY), 0, c"tc8_attach_type".as_ptr());

        err = test_tc_link__load(skel);
        if !ASSERT_OK(err, c"skel_load".as_ptr()) {
            break 'cleanup;
        }

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 0);
        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);

        ASSERT_EQ((*(*skel).bss).seen_tc8, false, c"seen_tc8".as_ptr());

        link = bpf_program__attach_netkit((*skel).progs.tc8, ifindex, &mut optl);
        if !ASSERT_OK_PTR(link as *const c_void, c"link_attach".as_ptr()) {
            break 'cleanup;
        }
        (*skel).links.tc8 = link;

        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 1);
        assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);

        tc_skel_reset_all_seen(skel);
        ASSERT_EQ(send_icmp(), 0, c"icmp_pkt".as_ptr());

        ASSERT_EQ((*(*skel).bss).seen_tc8, true, c"seen_tc8".as_ptr());
        ASSERT_EQ((*(*skel).bss).mark, if scrub == NETKIT_SCRUB_NONE { MARK } else { 0 }, c"mark".as_ptr());
        ASSERT_EQ((*(*skel).bss).prio, if scrub == NETKIT_SCRUB_NONE { PRIO } else { 0 }, c"prio".as_ptr());
        ASSERT_EQ((*(*skel).bss).headroom, if room { NETKIT_HEADROOM } else { 0 }, c"headroom".as_ptr());
        ASSERT_EQ((*(*skel).bss).tailroom, if room { NETKIT_TAILROOM } else { 0 }, c"tailroom".as_ptr());
    }
    test_tc_link__destroy(skel);

    assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PRIMARY, 0);
    assert_mprog_count_ifindex(ifindex, BPF_NETKIT_PEER, 0);
    destroy_netkit();
}

pub unsafe fn serial_test_tc_netkit_scrub() {
    serial_test_tc_netkit_scrub_type(NETKIT_SCRUB_DEFAULT, false);
    serial_test_tc_netkit_scrub_type(NETKIT_SCRUB_NONE, true);
}

unsafe extern "C" {
    static NETKIT_L2: c_int;
    static NETKIT_L3: c_int;
    static NETKIT_PASS: c_int;
    static NETKIT_SCRUB_DEFAULT: c_int;
    static NETKIT_SCRUB_NONE: c_int;
    static BPF_NETKIT_PRIMARY: c_int;
    static BPF_NETKIT_PEER: c_int;
    static BPF_TCX_INGRESS: c_int;
    static BPF_F_BEFORE: __u32;
    static EACCES: c_int;

    static NLM_F_REQUEST: c_int;
    static NLM_F_CREATE: c_int;
    static NLM_F_EXCL: c_int;
    static RTM_NEWLINK: c_int;
    static AF_UNSPEC: c_int;
    static AF_INET: c_int;
    static SOCK_DGRAM: c_int;
    static IPPROTO_ICMP: c_int;
    static SOL_SOCKET: c_int;
    static SO_BINDTODEVICE: c_int;
    static SO_MARK: c_int;
    static SO_PRIORITY: c_int;
    static IFLA_IFNAME: c_int;
    static IFLA_LINKINFO: c_int;
    static IFLA_INFO_KIND: c_int;
    static IFLA_INFO_DATA: c_int;
    static IFLA_NETKIT_POLICY: c_int;
    static IFLA_NETKIT_PEER_POLICY: c_int;
    static IFLA_NETKIT_SCRUB: c_int;
    static IFLA_NETKIT_PEER_SCRUB: c_int;
    static IFLA_NETKIT_MODE: c_int;
    static IFLA_NETKIT_HEADROOM: c_int;
    static IFLA_NETKIT_TAILROOM: c_int;

    fn NLMSG_LENGTH(len: usize) -> usize;
    fn strlen(s: *const c_char) -> usize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn system(command: *const c_char) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> u32;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: usize) -> c_int;
    fn sendto(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int, dest_addr: *const sockaddr, addrlen: usize) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn htonl(hostlong: __u32) -> __u32;
    fn errno() -> c_int;

    fn rtnl_open(rth: *mut rtnl_handle, subscriptions: c_int) -> c_int;
    fn rtnl_close(rth: *mut rtnl_handle);
    fn rtnl_talk(rth: *mut rtnl_handle, n: *mut nlmsghdr, answer: *mut *mut nlmsghdr) -> c_int;
    fn addattr_l(n: *mut nlmsghdr, maxlen: usize, type_: c_int, data: *const c_void, alen: usize) -> c_int;
    fn addattr_nest(n: *mut nlmsghdr, maxlen: usize, type_: c_int) -> *mut rtattr;
    fn addattr_nest_end(n: *mut nlmsghdr, nest: *mut rtattr) -> c_int;
    fn addattr32(n: *mut nlmsghdr, maxlen: usize, type_: c_int, data: __u32) -> c_int;
    fn addattr16(n: *mut nlmsghdr, maxlen: usize, type_: c_int, data: __u16) -> c_int;

    fn write_sysctl(path: *const c_char, value: *const c_char) -> c_int;
    fn id_from_prog_fd(fd: c_int) -> __u32;
    fn id_from_link_fd(fd: c_int) -> __u32;
    fn assert_mprog_count_ifindex(ifindex: c_int, attach_type: c_int, count: c_int);
    fn tc_skel_reset_all_seen(skel: *mut test_tc_link);

    fn test_tc_link__open() -> *mut test_tc_link;
    fn test_tc_link__open_and_load() -> *mut test_tc_link;
    fn test_tc_link__load(skel: *mut test_tc_link) -> c_int;
    fn test_tc_link__destroy(skel: *mut test_tc_link);
    fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, attach_type: c_int) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__attach_netkit(prog: *mut bpf_program, ifindex: c_int, opts: *mut bpf_netkit_opts) -> *mut bpf_link;
    fn bpf_program__attach_tcx(prog: *mut bpf_program, ifindex: c_int, opts: *mut bpf_tcx_opts) -> *mut bpf_link;
    fn bpf_prog_query_opts(ifindex: c_int, attach_type: c_int, opts: *mut bpf_prog_query_opts) -> c_int;
    fn bpf_prog_attach_opts(prog_fd: c_int, target_fd: c_int, attach_type: c_int, opts: *mut bpf_prog_attach_opts) -> c_int;
    fn bpf_prog_detach_opts(prog_fd: c_int, target_fd: c_int, attach_type: c_int, opts: *mut bpf_prog_detach_opts) -> c_int;

    fn ASSERT_OK<T>(actual: T, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_GT<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_NEQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
}

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}

#[repr(C)]
struct ifinfomsg {
    ifi_family: u8,
    __ifi_pad: u8,
    ifi_type: u16,
    ifi_index: i32,
    ifi_flags: u32,
    ifi_change: u32,
}

#[repr(C)]
struct rtattr {
    rta_len: u16,
    rta_type: u16,
}

#[repr(C)]
struct rtnl_handle {
    fd: c_int,
}

#[repr(C)]
struct in_addr {
    s_addr: __u32,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_prog_query_opts {
    prog_ids: *mut __u32,
    link_ids: *mut __u32,
    count: __u32,
    revision: __u32,
}

#[repr(C)]
struct bpf_netkit_opts {
    flags: __u32,
    relative_fd: c_int,
}

#[repr(C)]
struct bpf_tcx_opts {
    _opaque: [u8; 0],
}

#[repr(C)]
struct bpf_prog_attach_opts {
    flags: __u32,
    relative_fd: c_int,
}

#[repr(C)]
struct bpf_prog_detach_opts {
    _opaque: [u8; 0],
}

#[repr(C)]
struct test_tc_link {
    progs: test_tc_link_progs,
    links: test_tc_link_links,
    bss: *mut test_tc_link_bss,
}

#[repr(C)]
struct test_tc_link_progs {
    tc1: *mut bpf_program,
    tc2: *mut bpf_program,
    tc3: *mut bpf_program,
    tc7: *mut bpf_program,
    tc8: *mut bpf_program,
}

#[repr(C)]
struct test_tc_link_links {
    tc1: *mut bpf_link,
    tc2: *mut bpf_link,
    tc7: *mut bpf_link,
    tc8: *mut bpf_link,
}

#[repr(C)]
struct test_tc_link_bss {
    seen_tc1: bool,
    seen_tc2: bool,
    seen_tc7: bool,
    seen_tc8: bool,
    seen_eth: bool,
    seen_host: bool,
    seen_mcast: bool,
    set_type: bool,
    mark: c_int,
    prio: c_int,
    headroom: c_int,
    tailroom: c_int,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
