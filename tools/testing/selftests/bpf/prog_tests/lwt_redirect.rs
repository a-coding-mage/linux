// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

/*
 * Test suite of lwt_xmit BPF programs that redirect packets
 *   The file tests focus not only if these programs work as expected normally,
 *   but also if they can handle abnormal situations gracefully.
 *
 * WARNING
 * -------
 *  This test suite may crash the kernel, thus should be run in a VM.
 *
 * Setup:
 * ---------
 *  All tests are performed in a single netns. Two lwt encap routes are setup for
 *  each subtest:
 *
 *    ip route add 10.0.0.0/24 encap bpf xmit <obj> sec "<ingress_sec>" dev link_err
 *    ip route add 20.0.0.0/24 encap bpf xmit <obj> sec "<egress_sec>" dev link_err
 *
 *  Here <obj> is statically defined to test_lwt_redirect.bpf.o, and each section
 *  of this object holds a program entry to test. The BPF object is built from
 *  progs/test_lwt_redirect.c. We didn't use generated BPF skeleton since the
 *  attachment for lwt programs are not supported by libbpf yet.
 *
 *  For testing, ping commands are run in the test netns:
 *
 *    ping 10.0.0.<ifindex> -c 1 -w 1 -s 100
 *    ping 20.0.0.<ifindex> -c 1 -w 1 -s 100
 *
 * Scenarios:
 * --------------------------------
 *  1. Redirect to a running tap/tun device
 *  2. Redirect to a down tap/tun device
 *  3. Redirect to a vlan device with lower layer down
 *
 *  Case 1, ping packets should be received by packet socket on target device
 *  when redirected to ingress, and by tun/tap fd when redirected to egress.
 *
 *  Case 2,3 are considered successful as long as they do not crash the kernel
 *  as a regression.
 *
 *  Case 1,2 use tap device to test redirect to device that requires MAC
 *  header, and tun device to test the case with no MAC header added.
 */

// C dependencies removed from executable Rust:
// <sys/socket.h>, <net/if.h>, <linux/if_ether.h>, <linux/if_packet.h>,
// <linux/if_tun.h>, <arpa/inet.h>, <unistd.h>, <errno.h>, <stdbool.h>,
// <stdlib.h>, "lwt_helpers.h", "test_progs.h", "network_helpers.h".

const NETNS: &str = "ns_lwt_redirect";
const BPF_OBJECT: &str = "test_lwt_redirect.bpf.o";
const LOCAL_SRC: &str = "10.0.0.1";
const CIDR_TO_INGRESS: &str = "10.0.0.0/24";
const CIDR_TO_EGRESS: &str = "20.0.0.0/24";

fn ingress_sec(need_mac: bool) -> &'static str {
    if need_mac {
        "redir_ingress"
    } else {
        "redir_ingress_nomac"
    }
}

fn egress_sec(need_mac: bool) -> &'static str {
    if need_mac {
        "redir_egress"
    } else {
        "redir_egress_nomac"
    }
}

/* ping to redirect toward given dev, with last byte of dest IP being the target
 * device index.
 *
 * Note: ping command inside BPF-CI is busybox version, so it does not have certain
 * function, such like -m option to set packet mark.
 */
unsafe fn ping_dev(dev: *const ::std::os::raw::c_char, is_ingress: bool) {
    let link_index: ::std::os::raw::c_int = if_nametoindex(dev) as ::std::os::raw::c_int;
    let mut ip = [0i8; 256];

    if !ASSERT_GE!(link_index, 0, "if_nametoindex") {
        return;
    }

    if is_ingress {
        snprintf(
            ip.as_mut_ptr(),
            ip.len(),
            b"10.0.0.%d\0".as_ptr() as *const ::std::os::raw::c_char,
            link_index,
        );
    } else {
        snprintf(
            ip.as_mut_ptr(),
            ip.len(),
            b"20.0.0.%d\0".as_ptr() as *const ::std::os::raw::c_char,
            link_index,
        );
    }

    /* We won't get a reply. Don't fail here */
    SYS_NOFAIL!(
        "ping %s -c1 -W1 -s %d",
        ip.as_ptr(),
        ICMP_PAYLOAD_SIZE
    );
}

unsafe fn new_packet_sock(ifname: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    let mut err: ::std::os::raw::c_int = 0;
    let ignore_outgoing: ::std::os::raw::c_int = 1;
    let mut ifindex: ::std::os::raw::c_int = -1;
    let mut s: ::std::os::raw::c_int = -1;

    s = socket(AF_PACKET, SOCK_RAW, 0);
    if !ASSERT_GE!(s, 0, "socket(AF_PACKET)") {
        return -1;
    }

    ifindex = if_nametoindex(ifname) as ::std::os::raw::c_int;
    if !ASSERT_GE!(ifindex, 0, "if_nametoindex") {
        close(s);
        return -1;
    }

    let addr = sockaddr_ll {
        sll_family: AF_PACKET as _,
        sll_protocol: htons(ETH_P_IP as _),
        sll_ifindex: ifindex,
        ..::std::mem::zeroed()
    };

    err = bind(
        s,
        &addr as *const sockaddr_ll as *const sockaddr,
        ::std::mem::size_of_val(&addr) as socklen_t,
    );
    if !ASSERT_OK!(err, "bind(AF_PACKET)") {
        close(s);
        return -1;
    }

    /* Use packet socket to capture only the ingress, so we can distinguish
     * the case where a regression that actually redirects the packet to
     * the egress.
     */
    err = setsockopt(
        s,
        SOL_PACKET,
        PACKET_IGNORE_OUTGOING,
        &ignore_outgoing as *const ::std::os::raw::c_int as *const ::std::os::raw::c_void,
        ::std::mem::size_of_val(&ignore_outgoing) as socklen_t,
    );
    if !ASSERT_OK!(err, "setsockopt(PACKET_IGNORE_OUTGOING)") {
        close(s);
        return -1;
    }

    err = fcntl(s, F_SETFL, O_NONBLOCK);
    if !ASSERT_OK!(err, "fcntl(O_NONBLOCK)") {
        close(s);
        return -1;
    }

    s
}

unsafe fn expect_icmp(buf: *mut ::std::os::raw::c_char, len: ssize_t) -> ::std::os::raw::c_int {
    let eth = buf as *mut ethhdr;

    if len < ::std::mem::size_of::<ethhdr>() as ssize_t {
        return -1;
    }

    if (*eth).h_proto == htons(ETH_P_IP as _) {
        return __expect_icmp_ipv4(
            eth.add(1) as *mut ::std::os::raw::c_char,
            len - ::std::mem::size_of::<ethhdr>() as ssize_t,
        );
    }

    -1
}

unsafe fn expect_icmp_nomac(
    buf: *mut ::std::os::raw::c_char,
    len: ssize_t,
) -> ::std::os::raw::c_int {
    __expect_icmp_ipv4(buf, len)
}

unsafe fn send_and_capture_test_packets(
    test_name: *const ::std::os::raw::c_char,
    tap_fd: ::std::os::raw::c_int,
    target_dev: *const ::std::os::raw::c_char,
    need_mac: bool,
) {
    let mut psock: ::std::os::raw::c_int = -1;
    let mut timeo = timeval {
        tv_sec: 0,
        tv_usec: 250000,
    };
    let mut ret: ::std::os::raw::c_int = -1;

    let filter: filter_t = if need_mac {
        expect_icmp
    } else {
        expect_icmp_nomac
    };

    ping_dev(target_dev, false);

    ret = wait_for_packet(tap_fd, filter, &mut timeo);
    if !ASSERT_EQ!(ret, 1, "wait_for_epacket") {
        log_err!("%s egress test fails", test_name);
        if psock >= 0 {
            close(psock);
        }
        return;
    }

    psock = new_packet_sock(target_dev);
    ping_dev(target_dev, true);

    ret = wait_for_packet(psock, filter, &mut timeo);
    if !ASSERT_EQ!(ret, 1, "wait_for_ipacket") {
        log_err!("%s ingress test fails", test_name);
        if psock >= 0 {
            close(psock);
        }
        return;
    }

    if psock >= 0 {
        close(psock);
    }
}

unsafe fn setup_redirect_target(
    target_dev: *const ::std::os::raw::c_char,
    need_mac: bool,
) -> ::std::os::raw::c_int {
    let mut target_index: ::std::os::raw::c_int = -1;
    let mut tap_fd: ::std::os::raw::c_int = -1;

    tap_fd = open_tuntap(target_dev, need_mac);
    if !ASSERT_GE!(tap_fd, 0, "open_tuntap") {
        if tap_fd >= 0 {
            close(tap_fd);
        }
        return -1;
    }

    target_index = if_nametoindex(target_dev) as ::std::os::raw::c_int;
    if !ASSERT_GE!(target_index, 0, "if_nametoindex") {
        if tap_fd >= 0 {
            close(tap_fd);
        }
        return -1;
    }

    let mut failed = false;
    if !failed {
        SYS!(failed, "sysctl -w net.ipv6.conf.all.disable_ipv6=1");
    }
    if !failed {
        SYS!(failed, "ip link add link_err type dummy");
    }
    if !failed {
        SYS!(failed, "ip link set lo up");
    }
    if !failed {
        SYS!(failed, "ip addr add dev lo " LOCAL_SRC "/32");
    }
    if !failed {
        SYS!(failed, "ip link set link_err up");
    }
    if !failed {
        SYS!(failed, "ip link set %s up", target_dev);
    }

    if !failed {
        SYS!(
            failed,
            "ip route add %s dev link_err encap bpf xmit obj %s sec %s",
            CIDR_TO_INGRESS,
            BPF_OBJECT,
            ingress_sec(need_mac)
        );
    }

    if !failed {
        SYS!(
            failed,
            "ip route add %s dev link_err encap bpf xmit obj %s sec %s",
            CIDR_TO_EGRESS,
            BPF_OBJECT,
            egress_sec(need_mac)
        );
    }

    if !failed {
        return tap_fd;
    }

    if tap_fd >= 0 {
        close(tap_fd);
    }
    -1
}

unsafe fn test_lwt_redirect_normal() {
    let target_dev = b"tap0\0".as_ptr() as *const ::std::os::raw::c_char;
    let mut tap_fd: ::std::os::raw::c_int = -1;
    let need_mac = true;

    tap_fd = setup_redirect_target(target_dev, need_mac);
    if !ASSERT_GE!(tap_fd, 0, "setup_redirect_target") {
        return;
    }

    send_and_capture_test_packets(
        b"test_lwt_redirect_normal\0".as_ptr() as *const ::std::os::raw::c_char,
        tap_fd,
        target_dev,
        need_mac,
    );
    close(tap_fd);
}

unsafe fn test_lwt_redirect_normal_nomac() {
    let target_dev = b"tun0\0".as_ptr() as *const ::std::os::raw::c_char;
    let mut tap_fd: ::std::os::raw::c_int = -1;
    let need_mac = false;

    tap_fd = setup_redirect_target(target_dev, need_mac);
    if !ASSERT_GE!(tap_fd, 0, "setup_redirect_target") {
        return;
    }

    send_and_capture_test_packets(
        b"test_lwt_redirect_normal_nomac\0".as_ptr() as *const ::std::os::raw::c_char,
        tap_fd,
        target_dev,
        need_mac,
    );
    close(tap_fd);
}

/* This test aims to prevent regression of future. As long as the kernel does
 * not panic, it is considered as success.
 */
unsafe fn __test_lwt_redirect_dev_down(need_mac: bool) {
    let target_dev = b"tap0\0".as_ptr() as *const ::std::os::raw::c_char;
    let mut tap_fd: ::std::os::raw::c_int = -1;

    tap_fd = setup_redirect_target(target_dev, need_mac);
    if !ASSERT_GE!(tap_fd, 0, "setup_redirect_target") {
        return;
    }

    let mut failed = false;
    SYS!(failed, "ip link set %s down", target_dev);
    if !failed {
        ping_dev(target_dev, true);
        ping_dev(target_dev, false);
    }

    close(tap_fd);
}

unsafe fn test_lwt_redirect_dev_down() {
    __test_lwt_redirect_dev_down(true);
}

unsafe fn test_lwt_redirect_dev_down_nomac() {
    __test_lwt_redirect_dev_down(false);
}

/* This test aims to prevent regression of future. As long as the kernel does
 * not panic, it is considered as success.
 */
unsafe fn test_lwt_redirect_dev_carrier_down() {
    let lower_dev = b"tap0\0".as_ptr() as *const ::std::os::raw::c_char;
    let vlan_dev = b"vlan100\0".as_ptr() as *const ::std::os::raw::c_char;
    let mut tap_fd: ::std::os::raw::c_int = -1;

    tap_fd = setup_redirect_target(lower_dev, true);
    if !ASSERT_GE!(tap_fd, 0, "setup_redirect_target") {
        return;
    }

    let mut failed = false;
    SYS!(failed, "ip link add vlan100 link %s type vlan id 100", lower_dev);
    if !failed {
        SYS!(failed, "ip link set %s up", vlan_dev);
    }
    if !failed {
        SYS!(failed, "ip link set %s down", lower_dev);
    }
    if !failed {
        ping_dev(vlan_dev, true);
        ping_dev(vlan_dev, false);
    }

    close(tap_fd);
}

unsafe extern "C" fn test_lwt_redirect_run(
    _arg: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    netns_delete();
    RUN_TEST!(lwt_redirect_normal);
    RUN_TEST!(lwt_redirect_normal_nomac);
    RUN_TEST!(lwt_redirect_dev_down);
    RUN_TEST!(lwt_redirect_dev_down_nomac);
    RUN_TEST!(lwt_redirect_dev_carrier_down);
    ::std::ptr::null_mut()
}

pub unsafe fn test_lwt_redirect() {
    let mut test_thread: pthread_t = ::std::mem::zeroed();
    let mut err: ::std::os::raw::c_int;

    /* Run the tests in their own thread to isolate the namespace changes
     * so they do not affect the environment of other tests.
     * (specifically needed because of unshare(CLONE_NEWNS) in open_netns())
     */
    err = pthread_create(
        &mut test_thread,
        ::std::ptr::null(),
        Some(test_lwt_redirect_run),
        ::std::ptr::null_mut(),
    );
    if ASSERT_OK!(err, "pthread_create") {
        ASSERT_OK!(
            pthread_join(test_thread, ::std::ptr::null_mut()),
            "pthread_join"
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
