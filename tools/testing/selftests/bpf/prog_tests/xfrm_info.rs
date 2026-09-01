// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

/*
 * Topology:
 * ---------
 *   NS0 namespace         |   NS1 namespace        | NS2 namespace
 *                         |                        |
 *   +---------------+     |   +---------------+    |
 *   |    ipsec0     |---------|    ipsec0     |    |
 *   | 192.168.1.100 |     |   | 192.168.1.200 |    |
 *   | if_id: bpf    |     |   +---------------+    |
 *   +---------------+     |                        |
 *           |             |                        |   +---------------+
 *           |             |                        |   |    ipsec0     |
 *           \------------------------------------------| 192.168.1.200 |
 *                         |                        |   +---------------+
 *                         |                        |
 *                         |                        | (overlay network)
 *      ------------------------------------------------------
 *                         |                        | (underlay network)
 *   +--------------+      |   +--------------+     |
 *   |    veth01    |----------|    veth10    |     |
 *   | 172.16.1.100 |      |   | 172.16.1.200 |     |
 *   ---------------+      |   +--------------+     |
 *                         |                        |
 *   +--------------+      |                        |   +--------------+
 *   |    veth02    |-----------------------------------|    veth20    |
 *   | 172.16.2.100 |      |                        |   | 172.16.2.200 |
 *   +--------------+      |                        |   +--------------+
 *
 *
 * Test Packet flow
 * -----------
 *  The tests perform 'ping 192.168.1.200' from the NS0 namespace:
 *  1) request is routed to NS0 ipsec0
 *  2) NS0 ipsec0 tc egress BPF program is triggered and sets the if_id based
 *     on the requested value. This makes the ipsec0 device in external mode
 *     select the destination tunnel
 *  3) ping reaches the other namespace (NS1 or NS2 based on which if_id was
 *     used) and response is sent
 *  4) response is received on NS0 ipsec0, tc ingress program is triggered and
 *     records the response if_id
 *  5) requested if_id is compared with received if_id
 */

// C includes translated as external dependencies:
// <net/if.h>, <linux/rtnetlink.h>, <linux/if_link.h>
// "test_progs.h", "network_helpers.h", "xfrm_info.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void};
use core::mem;
use core::ptr;

const NS0: &[u8] = b"xfrm_test_ns0\0";
const NS1: &[u8] = b"xfrm_test_ns1\0";
const NS2: &[u8] = b"xfrm_test_ns2\0";

const IF_ID_0_TO_1: c_int = 1;
const IF_ID_0_TO_2: c_int = 2;
const IF_ID_1: c_int = 3;
const IF_ID_2: c_int = 4;

const IP4_ADDR_VETH01: &[u8] = b"172.16.1.100\0";
const IP4_ADDR_VETH10: &[u8] = b"172.16.1.200\0";
const IP4_ADDR_VETH02: &[u8] = b"172.16.2.100\0";
const IP4_ADDR_VETH20: &[u8] = b"172.16.2.200\0";

const ESP_DUMMY_PARAMS: &str = "proto esp aead 'rfc4106(gcm(aes))' \
0xe4d8f4b4da1df18a3510b3781496daa82488b713 128 mode tunnel ";

const BPF_TC_INGRESS: c_uint = 1;
const BPF_TC_EGRESS: c_uint = 2;
const AF_NETLINK: c_int = 16;
const SOCK_RAW: c_int = 3;
const SOCK_CLOEXEC: c_int = 0o2000000;
const NETLINK_ROUTE: c_int = 0;
const RTM_NEWLINK: c_ushort = 16;
const NLM_F_REQUEST: c_ushort = 0x01;
const NLM_F_CREATE: c_ushort = 0x400;
const IFLA_IFNAME: c_ushort = 3;
const IFLA_LINKINFO: c_ushort = 18;
const IFLA_INFO_KIND: c_ushort = 1;
const IFLA_INFO_DATA: c_ushort = 2;
const IFLA_XFRM_COLLECT_METADATA: c_ushort = 2;

#[repr(C)]
pub struct bpf_tc_hook {
    pub sz: usize,
    pub ifindex: c_int,
    pub attach_point: c_uint,
    pub parent: u32,
}

#[repr(C)]
pub struct bpf_tc_opts {
    pub sz: usize,
    pub prog_fd: c_int,
    pub flags: u32,
    pub prog_id: u32,
    pub handle: u32,
    pub priority: u32,
}

#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: c_ushort,
    pub nlmsg_flags: c_ushort,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

#[repr(C)]
pub struct ifinfomsg {
    pub ifi_family: u8,
    pub __ifi_pad: u8,
    pub ifi_type: c_ushort,
    pub ifi_index: c_int,
    pub ifi_flags: c_uint,
    pub ifi_change: c_uint,
}

#[repr(C)]
pub struct rtattr {
    pub rta_len: c_ushort,
    pub rta_type: c_ushort,
}

#[repr(C)]
struct setup_xfrmi_external_dev_req {
    nh: nlmsghdr,
    info: ifinfomsg,
    data: [u8; 128],
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_info_bss {
    pub req_if_id: u32,
    pub resp_if_id: u32,
}

#[repr(C)]
pub struct xfrm_info_progs {
    pub set_xfrm_info: *mut bpf_program,
    pub get_xfrm_info: *mut bpf_program,
}

#[repr(C)]
pub struct xfrm_info {
    pub bss: *mut xfrm_info_bss,
    pub progs: xfrm_info_progs,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn xfrm_info__open_and_load() -> *mut xfrm_info;
    fn xfrm_info__destroy(obj: *mut xfrm_info);

    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_GE(left: c_int, right: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_int, right: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(left: c_uint, right: c_uint, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn SYS_NOFAIL(fmt: *const c_char, ...);
    fn SYS(label: *const c_char, fmt: *const c_char, ...) -> c_int;
}

const fn rta_align(len: u32) -> u32 {
    (len + 4 - 1) & !(4 - 1)
}

const fn rta_length(len: usize) -> c_ushort {
    (rta_align(mem::size_of::<rtattr>() as u32) + len as u32) as c_ushort
}

const fn nlmsg_align(len: u32) -> u32 {
    (len + 4 - 1) & !(4 - 1)
}

const fn nlmsg_length(len: usize) -> u32 {
    len as u32 + nlmsg_align(mem::size_of::<nlmsghdr>() as u32)
}

unsafe fn rta_data(rta: *mut rtattr) -> *mut c_void {
    (rta as *mut u8).add(rta_length(0) as usize) as *mut c_void
}

unsafe fn attach_tc_prog(hook: *mut bpf_tc_hook, igr_fd: c_int, egr_fd: c_int) -> c_int {
    let mut opts1 = bpf_tc_opts {
        sz: mem::size_of::<bpf_tc_opts>(),
        prog_fd: igr_fd,
        flags: 0,
        prog_id: 0,
        handle: 1,
        priority: 1,
    };
    let mut opts2 = bpf_tc_opts {
        sz: mem::size_of::<bpf_tc_opts>(),
        prog_fd: egr_fd,
        flags: 0,
        prog_id: 0,
        handle: 1,
        priority: 1,
    };
    let mut ret: c_int;

    ret = bpf_tc_hook_create(hook);
    if !ASSERT_OK(ret, c"create tc hook".as_ptr()) {
        return ret;
    }

    if igr_fd >= 0 {
        (*hook).attach_point = BPF_TC_INGRESS;
        ret = bpf_tc_attach(hook, &mut opts1);
        if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
            bpf_tc_hook_destroy(hook);
            return ret;
        }
    }

    if egr_fd >= 0 {
        (*hook).attach_point = BPF_TC_EGRESS;
        ret = bpf_tc_attach(hook, &mut opts2);
        if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
            bpf_tc_hook_destroy(hook);
            return ret;
        }
    }

    0
}

unsafe fn cleanup() {
    SYS_NOFAIL(c"test -f /var/run/netns/xfrm_test_ns0 && ip netns delete xfrm_test_ns0".as_ptr());
    SYS_NOFAIL(c"test -f /var/run/netns/xfrm_test_ns1 && ip netns delete xfrm_test_ns1".as_ptr());
    SYS_NOFAIL(c"test -f /var/run/netns/xfrm_test_ns2 && ip netns delete xfrm_test_ns2".as_ptr());
}

unsafe fn config_underlay() -> c_int {
    if SYS(c"fail".as_ptr(), c"ip netns add xfrm_test_ns0".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip netns add xfrm_test_ns1".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip netns add xfrm_test_ns2".as_ptr()) != 0 {
        return -1;
    }

    /* NS0 <-> NS1 [veth01 <-> veth10] */
    if SYS(c"fail".as_ptr(), c"ip link add veth01 netns xfrm_test_ns0 type veth peer name veth10 netns xfrm_test_ns1".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns0 addr add 172.16.1.100/24 dev veth01".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns0 link set dev veth01 up".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns1 addr add 172.16.1.200/24 dev veth10".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns1 link set dev veth10 up".as_ptr()) != 0 {
        return -1;
    }

    /* NS0 <-> NS2 [veth02 <-> veth20] */
    if SYS(c"fail".as_ptr(), c"ip link add veth02 netns xfrm_test_ns0 type veth peer name veth20 netns xfrm_test_ns2".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns0 addr add 172.16.2.100/24 dev veth02".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns0 link set dev veth02 up".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns2 addr add 172.16.2.200/24 dev veth20".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns2 link set dev veth20 up".as_ptr()) != 0 {
        return -1;
    }

    0
}

unsafe fn setup_xfrm_tunnel_ns(
    ns: *const c_char,
    ipv4_local: *const c_char,
    ipv4_remote: *const c_char,
    if_id: c_int,
) -> c_int {
    /* State: local -> remote */
    if SYS(c"fail".as_ptr(), c"ip -net %s xfrm state add src %s dst %s spi 1 proto esp aead 'rfc4106(gcm(aes))' 0xe4d8f4b4da1df18a3510b3781496daa82488b713 128 mode tunnel if_id %d".as_ptr(), ns, ipv4_local, ipv4_remote, if_id) != 0 {
        return -1;
    }

    /* State: local <- remote */
    if SYS(c"fail".as_ptr(), c"ip -net %s xfrm state add src %s dst %s spi 1 proto esp aead 'rfc4106(gcm(aes))' 0xe4d8f4b4da1df18a3510b3781496daa82488b713 128 mode tunnel if_id %d".as_ptr(), ns, ipv4_remote, ipv4_local, if_id) != 0 {
        return -1;
    }

    /* Policy: local -> remote */
    if SYS(c"fail".as_ptr(), c"ip -net %s xfrm policy add dir out src 0.0.0.0/0 dst 0.0.0.0/0 if_id %d tmpl src %s dst %s proto esp mode tunnel if_id %d".as_ptr(), ns, if_id, ipv4_local, ipv4_remote, if_id) != 0 {
        return -1;
    }

    /* Policy: local <- remote */
    if SYS(c"fail".as_ptr(), c"ip -net %s xfrm policy add dir in src 0.0.0.0/0 dst 0.0.0.0/0 if_id %d tmpl src %s dst %s proto esp mode tunnel if_id %d".as_ptr(), ns, if_id, ipv4_remote, ipv4_local, if_id) != 0 {
        return -1;
    }

    0
}

unsafe fn setup_xfrm_tunnel(
    ns_a: *const c_char,
    ns_b: *const c_char,
    ipv4_a: *const c_char,
    ipv4_b: *const c_char,
    if_id_a: c_int,
    if_id_b: c_int,
) -> c_int {
    if setup_xfrm_tunnel_ns(ns_a, ipv4_a, ipv4_b, if_id_a) != 0 {
        1
    } else if setup_xfrm_tunnel_ns(ns_b, ipv4_b, ipv4_a, if_id_b) != 0 {
        1
    } else {
        0
    }
}

unsafe fn rtattr_add(nh: *mut nlmsghdr, typ: c_ushort, len: c_ushort) -> *mut rtattr {
    let rta = ((nh as *mut u8).add(rta_align((*nh).nlmsg_len) as usize)) as *mut rtattr;
    (*rta).rta_type = typ;
    (*rta).rta_len = rta_length(len as usize);
    (*nh).nlmsg_len = rta_align((*nh).nlmsg_len) + rta_align((*rta).rta_len as u32);
    rta
}

unsafe fn rtattr_add_str(nh: *mut nlmsghdr, typ: c_ushort, s: *const c_char) -> *mut rtattr {
    let rta = rtattr_add(nh, typ, strlen(s) as c_ushort);

    memcpy(rta_data(rta), s as *const c_void, strlen(s));
    rta
}

unsafe fn rtattr_begin(nh: *mut nlmsghdr, typ: c_ushort) -> *mut rtattr {
    rtattr_add(nh, typ, 0)
}

unsafe fn rtattr_end(nh: *mut nlmsghdr, attr: *mut rtattr) {
    let end = (nh as *mut u8).add((*nh).nlmsg_len as usize);

    (*attr).rta_len = end.offset_from(attr as *mut u8) as c_ushort;
}

unsafe fn setup_xfrmi_external_dev(ns: *const c_char) -> c_int {
    let mut req: setup_xfrmi_external_dev_req = mem::zeroed();
    let mut link_info: *mut rtattr;
    let mut info_data: *mut rtattr;
    let mut nstoken: *mut nstoken;
    let mut ret: c_int = -1;
    let mut sock: c_int = -1;
    let nh: *mut nlmsghdr;

    memset(
        &mut req as *mut _ as *mut c_void,
        0,
        mem::size_of::<setup_xfrmi_external_dev_req>(),
    );
    nh = &mut req.nh;
    (*nh).nlmsg_len = nlmsg_length(mem::size_of::<ifinfomsg>());
    (*nh).nlmsg_type = RTM_NEWLINK;
    (*nh).nlmsg_flags |= NLM_F_CREATE | NLM_F_REQUEST;

    rtattr_add_str(nh, IFLA_IFNAME, c"ipsec0".as_ptr());
    link_info = rtattr_begin(nh, IFLA_LINKINFO);
    rtattr_add_str(nh, IFLA_INFO_KIND, c"xfrm".as_ptr());
    info_data = rtattr_begin(nh, IFLA_INFO_DATA);
    rtattr_add(nh, IFLA_XFRM_COLLECT_METADATA, 0);
    rtattr_end(nh, info_data);
    rtattr_end(nh, link_info);

    nstoken = open_netns(ns);
    if !ASSERT_OK_PTR(nstoken, c"setns".as_ptr()) {
        if sock != -1 {
            close(sock);
        }
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        return ret;
    }

    sock = socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, NETLINK_ROUTE);
    if !ASSERT_GE(sock, 0, c"netlink socket".as_ptr()) {
        if sock != -1 {
            close(sock);
        }
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        return ret;
    }
    ret = send(sock, nh as *const c_void, (*nh).nlmsg_len as usize, 0) as c_int;
    if !ASSERT_EQ(ret, (*nh).nlmsg_len as c_int, c"netlink send length".as_ptr()) {
        if sock != -1 {
            close(sock);
        }
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        return ret;
    }

    ret = 0;

    if sock != -1 {
        close(sock);
    }
    if !nstoken.is_null() {
        close_netns(nstoken);
    }
    ret
}

unsafe fn config_overlay() -> c_int {
    if setup_xfrm_tunnel(
        NS0.as_ptr() as *const c_char,
        NS1.as_ptr() as *const c_char,
        IP4_ADDR_VETH01.as_ptr() as *const c_char,
        IP4_ADDR_VETH10.as_ptr() as *const c_char,
        IF_ID_0_TO_1,
        IF_ID_1,
    ) != 0
    {
        return -1;
    }
    if setup_xfrm_tunnel(
        NS0.as_ptr() as *const c_char,
        NS2.as_ptr() as *const c_char,
        IP4_ADDR_VETH02.as_ptr() as *const c_char,
        IP4_ADDR_VETH20.as_ptr() as *const c_char,
        IF_ID_0_TO_2,
        IF_ID_2,
    ) != 0
    {
        return -1;
    }

    /* Older iproute2 doesn't support this option */
    if !ASSERT_OK(setup_xfrmi_external_dev(NS0.as_ptr() as *const c_char), c"xfrmi".as_ptr()) {
        return -1;
    }

    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns0 addr add 192.168.1.100/24 dev ipsec0".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns0 link set dev ipsec0 up".as_ptr()) != 0 {
        return -1;
    }

    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns1 link add ipsec0 type xfrm if_id %d".as_ptr(), IF_ID_1) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns1 addr add 192.168.1.200/24 dev ipsec0".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns1 link set dev ipsec0 up".as_ptr()) != 0 {
        return -1;
    }

    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns2 link add ipsec0 type xfrm if_id %d".as_ptr(), IF_ID_2) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns2 addr add 192.168.1.200/24 dev ipsec0".as_ptr()) != 0 {
        return -1;
    }
    if SYS(c"fail".as_ptr(), c"ip -net xfrm_test_ns2 link set dev ipsec0 up".as_ptr()) != 0 {
        return -1;
    }

    0
}

unsafe fn test_xfrm_ping(skel: *mut xfrm_info, if_id: u32) -> c_int {
    (*(*skel).bss).req_if_id = if_id;

    if SYS(c"fail".as_ptr(), c"ping -i 0.01 -c 3 -w 10 -q 192.168.1.200 > /dev/null".as_ptr()) != 0 {
        return -1;
    }

    if !ASSERT_EQ((*(*skel).bss).resp_if_id as c_int, if_id as c_int, c"if_id".as_ptr()) {
        return -1;
    }

    0
}

unsafe fn _test_xfrm_info() {
    let mut tc_hook = bpf_tc_hook {
        sz: mem::size_of::<bpf_tc_hook>(),
        ifindex: 0,
        attach_point: BPF_TC_INGRESS,
        parent: 0,
    };
    let get_xfrm_info_prog_fd: c_int;
    let set_xfrm_info_prog_fd: c_int;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let skel: *mut xfrm_info;
    let ifindex: c_uint;

    /* load and attach bpf progs to ipsec dev tc hook point */
    skel = xfrm_info__open_and_load();
    if !ASSERT_OK_PTR(skel, c"xfrm_info__open_and_load".as_ptr()) {
        xfrm_info__destroy(skel);
        return;
    }
    nstoken = open_netns(NS0.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken, c"setns xfrm_test_ns0".as_ptr()) {
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        xfrm_info__destroy(skel);
        return;
    }
    ifindex = if_nametoindex(c"ipsec0".as_ptr());
    if !ASSERT_NEQ(ifindex, 0, c"ipsec0 ifindex".as_ptr()) {
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        xfrm_info__destroy(skel);
        return;
    }
    tc_hook.ifindex = ifindex as c_int;
    set_xfrm_info_prog_fd = bpf_program__fd((*skel).progs.set_xfrm_info);
    get_xfrm_info_prog_fd = bpf_program__fd((*skel).progs.get_xfrm_info);
    if !ASSERT_GE(set_xfrm_info_prog_fd, 0, c"bpf_program__fd".as_ptr()) {
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        xfrm_info__destroy(skel);
        return;
    }
    if !ASSERT_GE(get_xfrm_info_prog_fd, 0, c"bpf_program__fd".as_ptr()) {
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        xfrm_info__destroy(skel);
        return;
    }
    if attach_tc_prog(&mut tc_hook, get_xfrm_info_prog_fd, set_xfrm_info_prog_fd) != 0 {
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        xfrm_info__destroy(skel);
        return;
    }

    /* perform test */
    if !ASSERT_EQ(test_xfrm_ping(skel, IF_ID_0_TO_1 as u32), 0, c"ping xfrm_test_ns1".as_ptr()) {
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        xfrm_info__destroy(skel);
        return;
    }
    if !ASSERT_EQ(test_xfrm_ping(skel, IF_ID_0_TO_2 as u32), 0, c"ping xfrm_test_ns2".as_ptr()) {
        if !nstoken.is_null() {
            close_netns(nstoken);
        }
        xfrm_info__destroy(skel);
        return;
    }

    if !nstoken.is_null() {
        close_netns(nstoken);
    }
    xfrm_info__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_xfrm_info() {
    cleanup();

    if !ASSERT_OK(config_underlay(), c"config_underlay".as_ptr()) {
        cleanup();
        return;
    }
    if !ASSERT_OK(config_overlay(), c"config_overlay".as_ptr()) {
        cleanup();
        return;
    }

    if test__start_subtest(c"xfrm_info".as_ptr()) {
        _test_xfrm_info();
    }

    cleanup();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
