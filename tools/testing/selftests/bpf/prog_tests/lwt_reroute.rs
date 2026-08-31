// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

/*
 * Test suite of lwt BPF programs that reroutes packets
 *   The file tests focus not only if these programs work as expected normally,
 *   but also if they can handle abnormal situations gracefully. This test
 *   suite currently only covers lwt_xmit hook. lwt_in tests have not been
 *   implemented.
 *
 * WARNING
 * -------
 *  This test suite can crash the kernel, thus should be run in a VM.
 *
 * Setup:
 * ---------
 *  all tests are performed in a single netns. A lwt encap route is setup for
 *  each subtest:
 *
 *    ip route add 10.0.0.0/24 encap bpf xmit <obj> sec "<section_N>" dev link_err
 *
 *  Here <obj> is statically defined to test_lwt_reroute.bpf.o, and it contains
 *  a single test program entry. This program sets packet mark by last byte of
 *  the IPv4 daddr. For example, a packet going to 1.2.3.4 will receive a skb
 *  mark 4. A packet will only be marked once, and IP x.x.x.0 will be skipped
 *  to avoid route loop. We didn't use generated BPF skeleton since the
 *  attachment for lwt programs are not supported by libbpf yet.
 *
 *  The test program will bring up a tun device, and sets up the following
 *  routes:
 *
 *    ip rule add pref 100 from all fwmark <tun_index> lookup 100
 *    ip route add table 100 default dev tun0
 *
 *  For normal testing, a ping command is running in the test netns:
 *
 *    ping 10.0.0.<tun_index> -c 1 -w 1 -s 100
 *
 *  For abnormal testing, fq is used as the qdisc of the tun device. Then a UDP
 *  socket will try to overflow the fq queue and trigger qdisc drop error.
 *
 * Scenarios:
 * --------------------------------
 *  1. Reroute to a running tun device
 *  2. Reroute to a device where qdisc drop
 *
 *  For case 1, ping packets should be received by the tun device.
 *
 *  For case 2, force UDP packets to overflow fq limit. As long as kernel
 *  is not crashed, it is considered successful.
 */

const NETNS: &[u8] = b"ns_lwt_reroute\0";

// C dependencies: <netinet/in.h>, "lwt_helpers.h", "network_helpers.h",
// and <linux/net_tstamp.h>.

const BPF_OBJECT: &[u8] = b"test_lwt_reroute.bpf.o\0";
const LOCAL_SRC: &[u8] = b"10.0.0.1\0";
const TEST_CIDR: &[u8] = b"10.0.0.0/24\0";
const XMIT_HOOK: &[u8] = b"xmit\0";
const XMIT_SECTION: &[u8] = b"lwt_xmit\0";
const NSEC_PER_SEC: u64 = 1000000000;

extern "C" {
    static __expect_icmp_ipv4: libc::c_void;

    fn open_tuntap(dev: *const libc::c_char, need_mac: bool) -> libc::c_int;
    fn wait_for_packet(
        fd: libc::c_int,
        expect: *const libc::c_void,
        timeout: *mut libc::timeval,
    ) -> libc::c_int;
    fn netns_delete();
    fn log_err(fmt: *const libc::c_char, ...);
    fn if_nametoindex(ifname: *const libc::c_char) -> libc::c_uint;
    fn inet_pton(
        af: libc::c_int,
        src: *const libc::c_char,
        dst: *mut libc::c_void,
    ) -> libc::c_int;
    fn snprintf(
        s: *mut libc::c_char,
        n: libc::size_t,
        format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}

#[repr(C)]
struct sock_txtime {
    clockid: libc::clockid_t,
    flags: u32,
}

/* send a ping to be rerouted to the target device */
unsafe fn ping_once(ip: *const libc::c_char) {
    /* We won't get a reply. Don't fail here */
    SYS_NOFAIL!(
        b"ping %s -c1 -W1 -s %d\0".as_ptr() as *const libc::c_char,
        ip,
        ICMP_PAYLOAD_SIZE
    );
}

/* Send snd_target UDP packets to overflow the fq queue and trigger qdisc drop
 * error. This is done via TX tstamp to force buffering delayed packets.
 */
unsafe fn overflow_fq(mut snd_target: libc::c_int, target_ip: *const libc::c_char) -> libc::c_int {
    let mut addr: libc::sockaddr_in = std::mem::zeroed();
    addr.sin_family = libc::AF_INET as libc::sa_family_t;
    addr.sin_port = libc::htons(1234);

    let mut data_buf: [libc::c_char; 8] = [0; 8]; /* only #pkts matter, so use a random small buffer */
    let mut control_buf: [libc::c_char; libc::CMSG_SPACE(std::mem::size_of::<u64>() as libc::c_uint) as usize] =
        [0; libc::CMSG_SPACE(std::mem::size_of::<u64>() as libc::c_uint) as usize];
    let mut iov = libc::iovec {
        iov_base: data_buf.as_mut_ptr() as *mut libc::c_void,
        iov_len: std::mem::size_of_val(&data_buf),
    };
    let mut err: libc::c_int = -1;
    let mut s: libc::c_int = -1;
    let txtime_on = sock_txtime {
        clockid: libc::CLOCK_MONOTONIC,
        flags: 0,
    };
    let mut msg: libc::msghdr = std::mem::zeroed();
    msg.msg_name = &mut addr as *mut _ as *mut libc::c_void;
    msg.msg_namelen = std::mem::size_of_val(&addr) as libc::socklen_t;
    msg.msg_control = control_buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = std::mem::size_of_val(&control_buf);
    msg.msg_iovlen = 1;
    msg.msg_iov = &mut iov;
    let cmsg = libc::CMSG_FIRSTHDR(&msg as *const _ as *mut libc::msghdr);

    libc::memset(
        data_buf.as_mut_ptr() as *mut libc::c_void,
        0,
        std::mem::size_of_val(&data_buf),
    );

    'out: loop {
        s = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
        if !ASSERT_GE!(s, 0, b"socket\0".as_ptr() as *const libc::c_char) {
            break 'out;
        }

        err = libc::setsockopt(
            s,
            libc::SOL_SOCKET,
            SO_TXTIME,
            &txtime_on as *const _ as *const libc::c_void,
            std::mem::size_of_val(&txtime_on) as libc::socklen_t,
        );
        if !ASSERT_OK!(err, b"setsockopt(SO_TXTIME)\0".as_ptr() as *const libc::c_char) {
            break 'out;
        }

        err = inet_pton(
            libc::AF_INET,
            target_ip,
            &mut addr.sin_addr as *mut _ as *mut libc::c_void,
        );
        if !ASSERT_EQ!(err, 1, b"inet_pton\0".as_ptr() as *const libc::c_char) {
            break 'out;
        }

        while snd_target > 0 {
            let mut now: libc::timespec = std::mem::zeroed();

            libc::memset(
                control_buf.as_mut_ptr() as *mut libc::c_void,
                0,
                std::mem::size_of_val(&control_buf),
            );
            (*cmsg).cmsg_type = SCM_TXTIME;
            (*cmsg).cmsg_level = libc::SOL_SOCKET;
            (*cmsg).cmsg_len = libc::CMSG_LEN(std::mem::size_of::<u64>() as libc::c_uint) as _;

            err = libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut now);
            if !ASSERT_OK!(
            err,
            b"clock_gettime(CLOCK_MONOTONIC)\0".as_ptr() as *const libc::c_char
            ) {
                err = -1;
                break 'out;
            }

            *(libc::CMSG_DATA(cmsg) as *mut u64) =
                (now.tv_nsec as u64 + 1).wrapping_mul(NSEC_PER_SEC) + now.tv_nsec as u64;

            /* we will intentionally send more than fq limit, so ignore
             * the error here.
             */
            libc::sendmsg(s, &msg, libc::MSG_NOSIGNAL);
            snd_target -= 1;
        }

        /* no kernel crash so far is considered success */
        err = 0;
        break 'out;
    }

    if s >= 0 {
        libc::close(s);
    }

    return err;
}

unsafe fn setup(tun_dev: *const libc::c_char) -> libc::c_int {
    let mut target_index: libc::c_int = -1;
    let mut tap_fd: libc::c_int = -1;

    tap_fd = open_tuntap(tun_dev, false);
    if !ASSERT_GE!(tap_fd, 0, b"open_tun\0".as_ptr() as *const libc::c_char) {
        return -1;
    }

    target_index = if_nametoindex(tun_dev) as libc::c_int;
    if !ASSERT_GE!(
        target_index,
        0,
        b"if_nametoindex\0".as_ptr() as *const libc::c_char
    ) {
        return -1;
    }

    let setup_ret = 'fail: loop {
        SYS!(fail, b"ip link add link_err type dummy\0".as_ptr() as *const libc::c_char);
        SYS!(fail, b"ip link set lo up\0".as_ptr() as *const libc::c_char);
        SYS!(
            fail,
            b"ip addr add dev lo 10.0.0.1/32\0".as_ptr() as *const libc::c_char
        );
        SYS!(fail, b"ip link set link_err up\0".as_ptr() as *const libc::c_char);
        SYS!(
            fail,
            b"ip link set %s up\0".as_ptr() as *const libc::c_char,
            tun_dev
        );

        SYS!(
            fail,
            b"ip route add %s dev link_err encap bpf xmit obj %s sec lwt_xmit\0".as_ptr()
                as *const libc::c_char,
            TEST_CIDR.as_ptr() as *const libc::c_char,
            BPF_OBJECT.as_ptr() as *const libc::c_char
        );

        SYS!(
            fail,
            b"ip rule add pref 100 from all fwmark %d lookup 100\0".as_ptr() as *const libc::c_char,
            target_index
        );
        SYS!(
            fail,
            b"ip route add t 100 default dev %s\0".as_ptr() as *const libc::c_char,
            tun_dev
        );

        break 'fail tap_fd;
    };
    if setup_ret < 0 {
        if tap_fd >= 0 {
            libc::close(tap_fd);
        }
    }
    return setup_ret;
}

unsafe fn test_lwt_reroute_normal_xmit() {
    let tun_dev = b"tun0\0".as_ptr() as *const libc::c_char;
    let mut tun_fd: libc::c_int = -1;
    let mut ifindex: libc::c_int = -1;
    let mut ip: [libc::c_char; 256] = [0; 256];
    let mut timeo = libc::timeval {
        tv_sec: 0,
        tv_usec: 250000,
    };

    tun_fd = setup(tun_dev);
    if !ASSERT_GE!(tun_fd, 0, b"setup_reroute\0".as_ptr() as *const libc::c_char) {
        return;
    }

    ifindex = if_nametoindex(tun_dev) as libc::c_int;
    if !ASSERT_GE!(ifindex, 0, b"if_nametoindex\0".as_ptr() as *const libc::c_char) {
        return;
    }

    snprintf(
        ip.as_mut_ptr(),
        256,
        b"10.0.0.%d\0".as_ptr() as *const libc::c_char,
        ifindex,
    );

    /* ping packets should be received by the tun device */
    ping_once(ip.as_ptr());

    if !ASSERT_EQ!(
        wait_for_packet(tun_fd, &__expect_icmp_ipv4 as *const _, &mut timeo),
        1,
        b"wait_for_packet\0".as_ptr() as *const libc::c_char
    ) {
        log_err(b"%s xmit\0".as_ptr() as *const libc::c_char, b"test_lwt_reroute_normal_xmit\0".as_ptr());
    }
}

/*
 * Test the failure case when the skb is dropped at the qdisc. This is a
 * regression prevention at the xmit hook only.
 */
unsafe fn test_lwt_reroute_qdisc_dropped() {
    let tun_dev = b"tun0\0".as_ptr() as *const libc::c_char;
    let mut tun_fd: libc::c_int = -1;
    let mut ifindex: libc::c_int = -1;
    let mut ip: [libc::c_char; 256] = [0; 256];

    'fail: loop {
        tun_fd = setup(tun_dev);
        if !ASSERT_GE!(tun_fd, 0, b"setup_reroute\0".as_ptr() as *const libc::c_char) {
            break 'fail;
        }

        SYS!(
            fail,
            b"tc qdisc replace dev %s root fq limit 5 flow_limit 5\0".as_ptr() as *const libc::c_char,
            tun_dev
        );

        ifindex = if_nametoindex(tun_dev) as libc::c_int;
        if !ASSERT_GE!(ifindex, 0, b"if_nametoindex\0".as_ptr() as *const libc::c_char) {
            return;
        }

        snprintf(
            ip.as_mut_ptr(),
            256,
            b"10.0.0.%d\0".as_ptr() as *const libc::c_char,
            ifindex,
        );
        ASSERT_EQ!(overflow_fq(10, ip.as_ptr()), 0, b"overflow_fq\0".as_ptr() as *const libc::c_char);
        break 'fail;
    }

    if tun_fd >= 0 {
        libc::close(tun_fd);
    }
}

unsafe extern "C" fn test_lwt_reroute_run(_arg: *mut libc::c_void) -> *mut libc::c_void {
    netns_delete();
    RUN_TEST!(lwt_reroute_normal_xmit);
    RUN_TEST!(lwt_reroute_qdisc_dropped);
    std::ptr::null_mut()
}

pub unsafe fn test_lwt_reroute() {
    let mut test_thread: libc::pthread_t = std::mem::zeroed();
    let mut err: libc::c_int;

    /* Run the tests in their own thread to isolate the namespace changes
     * so they do not affect the environment of other tests.
     * (specifically needed because of unshare(CLONE_NEWNS) in open_netns())
     */
    err = libc::pthread_create(
        &mut test_thread,
        std::ptr::null(),
        test_lwt_reroute_run,
        std::ptr::null_mut(),
    );
    if ASSERT_OK!(err, b"pthread_create\0".as_ptr() as *const libc::c_char) {
        ASSERT_OK!(
            libc::pthread_join(test_thread, std::ptr::null_mut()),
            b"pthread_join\0".as_ptr() as *const libc::c_char
        );
    }
}
