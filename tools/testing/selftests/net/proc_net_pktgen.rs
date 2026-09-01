// SPDX-License-Identifier: GPL-2.0
/*
 * proc_net_pktgen: kselftest for /proc/net/pktgen interface
 *
 * Copyright (c) 2025 Peter Seiderer <ps.report@gmx.net>
 *
 */

// C dependencies: errno.h, fcntl.h, stdlib.h, unistd.h, kselftest_harness.h.

type c_char = i8;
type c_int = i32;
type size_t = usize;
type ssize_t = isize;

const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const EOPNOTSUPP: c_int = 95;
const O_RDWR: c_int = 0o2;

unsafe extern "C" {
    fn system(command: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const core::ffi::c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;

    // errno is supplied by errno.h in C; this preserves the source-level dependency.
    static mut errno: c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! sizeof {
    ($name:ident) => {
        $name.len() + 1
    };
}

macro_rules! ptr {
    ($name:ident) => {
        $name.as_ptr() as *const core::ffi::c_void
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! TH_LOG {
    ($msg:literal) => {
        $msg
    };
}

static ctrl_cmd_stop: &[u8] = b"stop";
static ctrl_cmd_start: &[u8] = b"start";
static ctrl_cmd_reset: &[u8] = b"reset";

static wrong_ctrl_cmd: &[u8] = b"0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789";

static thr_cmd_add_loopback_0: &[u8] = b"add_device lo@0";
static thr_cmd_rm_loopback_0: &[u8] = b"rem_device_all";

static wrong_thr_cmd: &[u8] = b"forsureawrongcommand";
static legacy_thr_cmd: &[u8] = b"max_before_softirq";

static wrong_dev_cmd: &[u8] = b"forsurewrongcommand";
static dev_cmd_min_pkt_size_0: &[u8] = b"min_pkt_size";
static dev_cmd_min_pkt_size_1: &[u8] = b"min_pkt_size ";
static dev_cmd_min_pkt_size_2: &[u8] = b"min_pkt_size 0";
static dev_cmd_min_pkt_size_3: &[u8] = b"min_pkt_size 1";
static dev_cmd_min_pkt_size_4: &[u8] = b"min_pkt_size 100";
static dev_cmd_min_pkt_size_5: &[u8] = b"min_pkt_size=1001";
static dev_cmd_min_pkt_size_6: &[u8] = b"min_pkt_size =2002";
static dev_cmd_min_pkt_size_7: &[u8] = b"min_pkt_size= 3003";
static dev_cmd_min_pkt_size_8: &[u8] = b"min_pkt_size = 4004";
static dev_cmd_max_pkt_size_0: &[u8] = b"max_pkt_size 200";
static dev_cmd_pkt_size_0: &[u8] = b"pkt_size 300";
static dev_cmd_imix_weights_0: &[u8] = b"imix_weights 0,7 576,4 1500,1";
static dev_cmd_imix_weights_1: &[u8] = b"imix_weights 101,1 102,2 103,3 104,4 105,5 106,6 107,7 108,8 109,9 110,10 111,11 112,12 113,13 114,14 115,15 116,16 117,17 118,18 119,19 120,20";
static dev_cmd_imix_weights_2: &[u8] = b"imix_weights 100,1 102,2 103,3 104,4 105,5 106,6 107,7 108,8 109,9 110,10 111,11 112,12 113,13 114,14 115,15 116,16 117,17 118,18 119,19 120,20 121,21";
static dev_cmd_imix_weights_3: &[u8] = b"imix_weights";
static dev_cmd_imix_weights_4: &[u8] = b"imix_weights ";
static dev_cmd_imix_weights_5: &[u8] = b"imix_weights 0";
static dev_cmd_imix_weights_6: &[u8] = b"imix_weights 0,";
static dev_cmd_debug_0: &[u8] = b"debug 1";
static dev_cmd_debug_1: &[u8] = b"debug 0";
static dev_cmd_frags_0: &[u8] = b"frags 100";
static dev_cmd_delay_0: &[u8] = b"delay 100";
static dev_cmd_delay_1: &[u8] = b"delay 2147483647";
static dev_cmd_rate_0: &[u8] = b"rate 0";
static dev_cmd_rate_1: &[u8] = b"rate 100";
static dev_cmd_ratep_0: &[u8] = b"ratep 0";
static dev_cmd_ratep_1: &[u8] = b"ratep 200";
static dev_cmd_udp_src_min_0: &[u8] = b"udp_src_min 1";
static dev_cmd_udp_dst_min_0: &[u8] = b"udp_dst_min 2";
static dev_cmd_udp_src_max_0: &[u8] = b"udp_src_max 3";
static dev_cmd_udp_dst_max_0: &[u8] = b"udp_dst_max 4";
static dev_cmd_clone_skb_0: &[u8] = b"clone_skb 1";
static dev_cmd_clone_skb_1: &[u8] = b"clone_skb 0";
static dev_cmd_count_0: &[u8] = b"count 100";
static dev_cmd_src_mac_count_0: &[u8] = b"src_mac_count 100";
static dev_cmd_dst_mac_count_0: &[u8] = b"dst_mac_count 100";
static dev_cmd_burst_0: &[u8] = b"burst 0";
static dev_cmd_node_0: &[u8] = b"node 100";
static dev_cmd_xmit_mode_0: &[u8] = b"xmit_mode start_xmit";
static dev_cmd_xmit_mode_1: &[u8] = b"xmit_mode netif_receive";
static dev_cmd_xmit_mode_2: &[u8] = b"xmit_mode queue_xmit";
static dev_cmd_xmit_mode_3: &[u8] = b"xmit_mode nonsense";
static dev_cmd_flag_0: &[u8] = b"flag UDPCSUM";
static dev_cmd_flag_1: &[u8] = b"flag !UDPCSUM";
static dev_cmd_flag_2: &[u8] = b"flag nonsense";
static dev_cmd_dst_min_0: &[u8] = b"dst_min 101.102.103.104";
static dev_cmd_dst_0: &[u8] = b"dst 101.102.103.104";
static dev_cmd_dst_max_0: &[u8] = b"dst_max 201.202.203.204";
static dev_cmd_dst6_0: &[u8] = b"dst6 2001:db38:1234:0000:0000:0000:0000:0000";
static dev_cmd_dst6_min_0: &[u8] = b"dst6_min 2001:db8:1234:0000:0000:0000:0000:0000";
static dev_cmd_dst6_max_0: &[u8] = b"dst6_max 2001:db8:1234:0000:0000:0000:0000:0000";
static dev_cmd_src6_0: &[u8] = b"src6 2001:db38:1234:0000:0000:0000:0000:0000";
static dev_cmd_src_min_0: &[u8] = b"src_min 101.102.103.104";
static dev_cmd_src_max_0: &[u8] = b"src_max 201.202.203.204";
static dev_cmd_dst_mac_0: &[u8] = b"dst_mac 01:02:03:04:05:06";
static dev_cmd_src_mac_0: &[u8] = b"src_mac 11:12:13:14:15:16";
static dev_cmd_clear_counters_0: &[u8] = b"clear_counters";
static dev_cmd_flows_0: &[u8] = b"flows 100";
static dev_cmd_spi_0: &[u8] = b"spi 100";
static dev_cmd_flowlen_0: &[u8] = b"flowlen 100";
static dev_cmd_queue_map_min_0: &[u8] = b"queue_map_min 1";
static dev_cmd_queue_map_max_0: &[u8] = b"queue_map_max 2";
static dev_cmd_mpls_0: &[u8] = b"mpls 00000001";
static dev_cmd_mpls_1: &[u8] = b"mpls 00000001,000000f2";
static dev_cmd_mpls_2: &[u8] = b"mpls 00000f00,00000f01,00000f02,00000f03,00000f04,00000f05,00000f06,00000f07,00000f08,00000f09,00000f0a,00000f0b,00000f0c,00000f0d,00000f0e,00000f0f";
static dev_cmd_mpls_3: &[u8] = b"mpls 00000f00,00000f01,00000f02,00000f03,00000f04,00000f05,00000f06,00000f07,00000f08,00000f09,00000f0a,00000f0b,00000f0c,00000f0d,00000f0e,00000f0f,00000f10";
static dev_cmd_vlan_id_0: &[u8] = b"vlan_id 1";
static dev_cmd_vlan_p_0: &[u8] = b"vlan_p 1";
static dev_cmd_vlan_cfi_0: &[u8] = b"vlan_cfi 1";
static dev_cmd_vlan_id_1: &[u8] = b"vlan_id 4096";
static dev_cmd_svlan_id_0: &[u8] = b"svlan_id 1";
static dev_cmd_svlan_p_0: &[u8] = b"svlan_p 1";
static dev_cmd_svlan_cfi_0: &[u8] = b"svlan_cfi 1";
static dev_cmd_svlan_id_1: &[u8] = b"svlan_id 4096";
static dev_cmd_tos_0: &[u8] = b"tos 0";
static dev_cmd_tos_1: &[u8] = b"tos 0f";
static dev_cmd_tos_2: &[u8] = b"tos 0ff";
static dev_cmd_traffic_class_0: &[u8] = b"traffic_class f0";
static dev_cmd_skb_priority_0: &[u8] = b"skb_priority 999";

#[repr(C)]
struct proc_net_pktgen {
    ctrl_fd: c_int,
    thr_fd: c_int,
    dev_fd: c_int,
}

unsafe fn proc_net_pktgen_setup(self_: *mut proc_net_pktgen) {
    let r: c_int;
    let mut len: ssize_t;

    r = system(cstr!("modprobe pktgen"));
    ASSERT_EQ!(r, 0);
    TH_LOG!("CONFIG_NET_PKTGEN not enabled, module pktgen not loaded?");

    (*self_).ctrl_fd = open(cstr!("/proc/net/pktgen/pgctrl"), O_RDWR);
    ASSERT_GE!((*self_).ctrl_fd, 0);
    TH_LOG!("CONFIG_NET_PKTGEN not enabled, module pktgen not loaded?");

    (*self_).thr_fd = open(cstr!("/proc/net/pktgen/kpktgend_0"), O_RDWR);
    ASSERT_GE!((*self_).thr_fd, 0);
    TH_LOG!("CONFIG_NET_PKTGEN not enabled, module pktgen not loaded?");

    len = write((*self_).thr_fd, ptr!(thr_cmd_add_loopback_0), sizeof!(thr_cmd_add_loopback_0));
    ASSERT_EQ!(len, sizeof!(thr_cmd_add_loopback_0) as ssize_t);
    TH_LOG!("device lo@0 already registered?");

    (*self_).dev_fd = open(cstr!("/proc/net/pktgen/lo@0"), O_RDWR);
    ASSERT_GE!((*self_).dev_fd, 0);
    TH_LOG!("device entry for lo@0 missing?");
}

unsafe fn proc_net_pktgen_teardown(self_: *mut proc_net_pktgen) {
    let mut ret: c_int;
    let mut len: ssize_t;

    ret = close((*self_).dev_fd);
    EXPECT_EQ!(ret, 0);

    len = write((*self_).thr_fd, ptr!(thr_cmd_rm_loopback_0), sizeof!(thr_cmd_rm_loopback_0));
    EXPECT_EQ!(len, sizeof!(thr_cmd_rm_loopback_0) as ssize_t);

    ret = close((*self_).thr_fd);
    EXPECT_EQ!(ret, 0);

    ret = close((*self_).ctrl_fd);
    EXPECT_EQ!(ret, 0);
}

unsafe fn wrong_ctrl_cmd_test(self_: *mut proc_net_pktgen) {
    for i in 0..=sizeof!(wrong_ctrl_cmd) {
        let len: ssize_t;

        len = write((*self_).ctrl_fd, ptr!(wrong_ctrl_cmd), i);
        EXPECT_EQ!(len, -1);
        EXPECT_EQ!(errno, EINVAL);
    }
}

unsafe fn ctrl_cmd(self_: *mut proc_net_pktgen) {
    let mut len: ssize_t;

    len = write((*self_).ctrl_fd, ptr!(ctrl_cmd_stop), sizeof!(ctrl_cmd_stop));
    EXPECT_EQ!(len, sizeof!(ctrl_cmd_stop) as ssize_t);

    len = write((*self_).ctrl_fd, ptr!(ctrl_cmd_stop), sizeof!(ctrl_cmd_stop) - 1);
    EXPECT_EQ!(len, (sizeof!(ctrl_cmd_stop) - 1) as ssize_t);

    len = write((*self_).ctrl_fd, ptr!(ctrl_cmd_start), sizeof!(ctrl_cmd_start));
    EXPECT_EQ!(len, sizeof!(ctrl_cmd_start) as ssize_t);

    len = write((*self_).ctrl_fd, ptr!(ctrl_cmd_start), sizeof!(ctrl_cmd_start) - 1);
    EXPECT_EQ!(len, (sizeof!(ctrl_cmd_start) - 1) as ssize_t);

    len = write((*self_).ctrl_fd, ptr!(ctrl_cmd_reset), sizeof!(ctrl_cmd_reset));
    EXPECT_EQ!(len, sizeof!(ctrl_cmd_reset) as ssize_t);

    len = write((*self_).ctrl_fd, ptr!(ctrl_cmd_reset), sizeof!(ctrl_cmd_reset) - 1);
    EXPECT_EQ!(len, (sizeof!(ctrl_cmd_reset) - 1) as ssize_t);
}

unsafe fn wrong_thr_cmd_test(self_: *mut proc_net_pktgen) {
    for i in 0..=sizeof!(wrong_thr_cmd) {
        let len: ssize_t;

        len = write((*self_).thr_fd, ptr!(wrong_thr_cmd), i);
        EXPECT_EQ!(len, -1);
        EXPECT_EQ!(errno, EINVAL);
    }
}

unsafe fn legacy_thr_cmd_test(self_: *mut proc_net_pktgen) {
    for i in 0..=sizeof!(legacy_thr_cmd) {
        let len: ssize_t;

        len = write((*self_).thr_fd, ptr!(legacy_thr_cmd), i);
        if i < sizeof!(legacy_thr_cmd) - 1 {
            /* incomplete command string */
            EXPECT_EQ!(len, -1);
            EXPECT_EQ!(errno, EINVAL);
        } else {
            /* complete command string without/with trailing '\0' */
            EXPECT_EQ!(len, i as ssize_t);
        }
    }
}

unsafe fn wrong_dev_cmd_test(self_: *mut proc_net_pktgen) {
    for i in 0..=sizeof!(wrong_dev_cmd) {
        let len: ssize_t;

        len = write((*self_).dev_fd, ptr!(wrong_dev_cmd), i);
        EXPECT_EQ!(len, -1);
        EXPECT_EQ!(errno, EINVAL);
    }
}

macro_rules! expect_write_eq_size {
    ($self_:ident, $cmd:ident) => {{
        let len = write((*$self_).dev_fd, ptr!($cmd), sizeof!($cmd));
        EXPECT_EQ!(len, sizeof!($cmd) as ssize_t);
    }};
}

unsafe fn dev_cmd_min_pkt_size(self_: *mut proc_net_pktgen) {
    let mut len: ssize_t;

    /* with trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_min_pkt_size_0), sizeof!(dev_cmd_min_pkt_size_0));
    EXPECT_EQ!(len, sizeof!(dev_cmd_min_pkt_size_0) as ssize_t);

    /* without trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_min_pkt_size_0), sizeof!(dev_cmd_min_pkt_size_0) - 1);
    EXPECT_EQ!(len, (sizeof!(dev_cmd_min_pkt_size_0) - 1) as ssize_t);

    /* with trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_min_pkt_size_1), sizeof!(dev_cmd_min_pkt_size_1));
    EXPECT_EQ!(len, sizeof!(dev_cmd_min_pkt_size_1) as ssize_t);

    /* without trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_min_pkt_size_1), sizeof!(dev_cmd_min_pkt_size_1) - 1);
    EXPECT_EQ!(len, (sizeof!(dev_cmd_min_pkt_size_1) - 1) as ssize_t);

    /* with trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_min_pkt_size_2), sizeof!(dev_cmd_min_pkt_size_2));
    EXPECT_EQ!(len, sizeof!(dev_cmd_min_pkt_size_2) as ssize_t);

    /* without trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_min_pkt_size_2), sizeof!(dev_cmd_min_pkt_size_2) - 1);
    EXPECT_EQ!(len, (sizeof!(dev_cmd_min_pkt_size_2) - 1) as ssize_t);

    expect_write_eq_size!(self_, dev_cmd_min_pkt_size_3);
    expect_write_eq_size!(self_, dev_cmd_min_pkt_size_4);
    expect_write_eq_size!(self_, dev_cmd_min_pkt_size_5);
    expect_write_eq_size!(self_, dev_cmd_min_pkt_size_6);
    expect_write_eq_size!(self_, dev_cmd_min_pkt_size_7);
    expect_write_eq_size!(self_, dev_cmd_min_pkt_size_8);
}

unsafe fn dev_cmd_max_pkt_size(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_max_pkt_size_0); }
unsafe fn dev_cmd_pkt_size(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_pkt_size_0); }

unsafe fn dev_cmd_imix_weights(self_: *mut proc_net_pktgen) {
    let mut len: ssize_t;

    expect_write_eq_size!(self_, dev_cmd_imix_weights_0);
    expect_write_eq_size!(self_, dev_cmd_imix_weights_1);

    len = write((*self_).dev_fd, ptr!(dev_cmd_imix_weights_2), sizeof!(dev_cmd_imix_weights_2));
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, E2BIG);

    /* with trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_imix_weights_3), sizeof!(dev_cmd_imix_weights_3));
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);

    /* without trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_imix_weights_3), sizeof!(dev_cmd_imix_weights_3) - 1);
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);

    /* with trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_imix_weights_4), sizeof!(dev_cmd_imix_weights_4));
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);

    /* without trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_imix_weights_4), sizeof!(dev_cmd_imix_weights_4) - 1);
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);

    /* with trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_imix_weights_5), sizeof!(dev_cmd_imix_weights_5));
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);

    /* without trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_imix_weights_5), sizeof!(dev_cmd_imix_weights_5) - 1);
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);

    /* with trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_imix_weights_6), sizeof!(dev_cmd_imix_weights_6));
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);

    /* without trailing '\0' */
    len = write((*self_).dev_fd, ptr!(dev_cmd_imix_weights_6), sizeof!(dev_cmd_imix_weights_6) - 1);
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);
}

unsafe fn dev_cmd_debug(self_: *mut proc_net_pktgen) {
    /* debug on */
    expect_write_eq_size!(self_, dev_cmd_debug_0);
    /* debug off */
    expect_write_eq_size!(self_, dev_cmd_debug_1);
}

unsafe fn dev_cmd_frags(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_frags_0); }
unsafe fn dev_cmd_delay(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_delay_0); expect_write_eq_size!(self_, dev_cmd_delay_1); }

unsafe fn dev_cmd_rate(self_: *mut proc_net_pktgen) {
    let mut len: ssize_t;
    len = write((*self_).dev_fd, ptr!(dev_cmd_rate_0), sizeof!(dev_cmd_rate_0));
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);
    expect_write_eq_size!(self_, dev_cmd_rate_1);
}

unsafe fn dev_cmd_ratep(self_: *mut proc_net_pktgen) {
    let mut len: ssize_t;
    len = write((*self_).dev_fd, ptr!(dev_cmd_ratep_0), sizeof!(dev_cmd_ratep_0));
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EINVAL);
    expect_write_eq_size!(self_, dev_cmd_ratep_1);
}

unsafe fn dev_cmd_udp_src_min(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_udp_src_min_0); }
unsafe fn dev_cmd_udp_dst_min(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_udp_dst_min_0); }
unsafe fn dev_cmd_udp_src_max(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_udp_src_max_0); }
unsafe fn dev_cmd_udp_dst_max(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_udp_dst_max_0); }

unsafe fn dev_cmd_clone_skb(self_: *mut proc_net_pktgen) {
    let mut len: ssize_t;

    /* clone_skb on (gives EOPNOTSUPP on lo device) */
    len = write((*self_).dev_fd, ptr!(dev_cmd_clone_skb_0), sizeof!(dev_cmd_clone_skb_0));
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, EOPNOTSUPP);

    /* clone_skb off */
    expect_write_eq_size!(self_, dev_cmd_clone_skb_1);
}

unsafe fn dev_cmd_count(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_count_0); }
unsafe fn dev_cmd_src_mac_count(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_src_mac_count_0); }
unsafe fn dev_cmd_dst_mac_count(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_dst_mac_count_0); }
unsafe fn dev_cmd_burst(self_: *mut proc_net_pktgen) { /* burst off */ expect_write_eq_size!(self_, dev_cmd_burst_0); }
unsafe fn dev_cmd_node(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_node_0); }

unsafe fn dev_cmd_xmit_mode(self_: *mut proc_net_pktgen) {
    expect_write_eq_size!(self_, dev_cmd_xmit_mode_0);
    expect_write_eq_size!(self_, dev_cmd_xmit_mode_1);
    expect_write_eq_size!(self_, dev_cmd_xmit_mode_2);
    expect_write_eq_size!(self_, dev_cmd_xmit_mode_3);
}

unsafe fn dev_cmd_flag(self_: *mut proc_net_pktgen) {
    /* flag UDPCSUM on */
    expect_write_eq_size!(self_, dev_cmd_flag_0);
    /* flag UDPCSUM off */
    expect_write_eq_size!(self_, dev_cmd_flag_1);
    /* flag invalid */
    expect_write_eq_size!(self_, dev_cmd_flag_2);
}

unsafe fn dev_cmd_dst_min(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_dst_min_0); }
unsafe fn dev_cmd_dst(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_dst_0); }
unsafe fn dev_cmd_dst_max(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_dst_max_0); }
unsafe fn dev_cmd_dst6(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_dst6_0); }
unsafe fn dev_cmd_dst6_min(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_dst6_min_0); }
unsafe fn dev_cmd_dst6_max(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_dst6_max_0); }
unsafe fn dev_cmd_src6(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_src6_0); }
unsafe fn dev_cmd_src_min(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_src_min_0); }
unsafe fn dev_cmd_src_max(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_src_max_0); }
unsafe fn dev_cmd_dst_mac(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_dst_mac_0); }
unsafe fn dev_cmd_src_mac(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_src_mac_0); }
unsafe fn dev_cmd_clear_counters(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_clear_counters_0); }
unsafe fn dev_cmd_flows(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_flows_0); }

unsafe fn dev_cmd_spi(self_: *mut proc_net_pktgen) {
    let len: ssize_t;
    len = write((*self_).dev_fd, ptr!(dev_cmd_spi_0), sizeof!(dev_cmd_spi_0));
    EXPECT_EQ!(len, sizeof!(dev_cmd_spi_0) as ssize_t);
    TH_LOG!("CONFIG_XFRM not enabled?");
}

unsafe fn dev_cmd_flowlen(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_flowlen_0); }
unsafe fn dev_cmd_queue_map_min(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_queue_map_min_0); }
unsafe fn dev_cmd_queue_map_max(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_queue_map_max_0); }

unsafe fn dev_cmd_mpls(self_: *mut proc_net_pktgen) {
    let mut len: ssize_t;

    expect_write_eq_size!(self_, dev_cmd_mpls_0);
    expect_write_eq_size!(self_, dev_cmd_mpls_1);
    expect_write_eq_size!(self_, dev_cmd_mpls_2);

    len = write((*self_).dev_fd, ptr!(dev_cmd_mpls_3), sizeof!(dev_cmd_mpls_3));
    EXPECT_EQ!(len, -1);
    EXPECT_EQ!(errno, E2BIG);
}

unsafe fn dev_cmd_vlan_id(self_: *mut proc_net_pktgen) {
    expect_write_eq_size!(self_, dev_cmd_vlan_id_0);
    expect_write_eq_size!(self_, dev_cmd_vlan_p_0);
    expect_write_eq_size!(self_, dev_cmd_vlan_cfi_0);
    expect_write_eq_size!(self_, dev_cmd_vlan_id_1);
}

unsafe fn dev_cmd_svlan_id(self_: *mut proc_net_pktgen) {
    expect_write_eq_size!(self_, dev_cmd_svlan_id_0);
    expect_write_eq_size!(self_, dev_cmd_svlan_p_0);
    expect_write_eq_size!(self_, dev_cmd_svlan_cfi_0);
    expect_write_eq_size!(self_, dev_cmd_svlan_id_1);
}

unsafe fn dev_cmd_tos(self_: *mut proc_net_pktgen) {
    expect_write_eq_size!(self_, dev_cmd_tos_0);
    expect_write_eq_size!(self_, dev_cmd_tos_1);
    expect_write_eq_size!(self_, dev_cmd_tos_2);
}

unsafe fn dev_cmd_traffic_class(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_traffic_class_0); }
unsafe fn dev_cmd_skb_priority(self_: *mut proc_net_pktgen) { expect_write_eq_size!(self_, dev_cmd_skb_priority_0); }

fn main() {
    // TEST_HARNESS_MAIN
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
