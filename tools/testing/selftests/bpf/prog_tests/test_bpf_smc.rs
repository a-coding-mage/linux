// SPDX-License-Identifier: GPL-2.0
// Translated from C source:
//   #include <test_progs.h>
//   #include <linux/genetlink.h>
//   #include "network_helpers.h"
//   #include "bpf_smc.skel.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const IPPROTO_SMC: c_int = 256;

const CLIENT_IP: *const c_char = b"127.0.0.1\0".as_ptr() as *const c_char;
const SERVER_IP: *const c_char = b"127.0.1.0\0".as_ptr() as *const c_char;
const SERVER_IP_VIA_RISK_PATH: *const c_char = b"127.0.2.0\0".as_ptr() as *const c_char;

const SERVICE_1: c_int = 80;
const SERVICE_2: c_int = 443;
const SERVICE_3: c_int = 8443;

const TEST_NS: *const c_char = b"bpf_smc_netns\0".as_ptr() as *const c_char;

static mut test_netns: *mut netns_obj = ptr::null_mut();

#[repr(C)]
struct smc_policy_ip_key {
    sip: __u32,
    dip: __u32,
}

#[repr(C)]
struct smc_policy_ip_value {
    mode: __u8,
}

#[cfg(target_arch = "s390x")]
unsafe fn setup_ueid() -> bool {
    true
}

#[cfg(target_arch = "s390x")]
unsafe fn cleanup_ueid() {}

#[cfg(not(target_arch = "s390x"))]
const SMC_NETLINK_ADD_UEID: c_int = 10;
#[cfg(not(target_arch = "s390x"))]
const SMC_NETLINK_REMOVE_UEID: c_int = SMC_NETLINK_ADD_UEID + 1;

#[cfg(not(target_arch = "s390x"))]
const SMC_NLA_EID_TABLE_UNSPEC: c_int = 0;
#[cfg(not(target_arch = "s390x"))]
const SMC_NLA_EID_TABLE_ENTRY: c_int = 1; /* string */

#[cfg(not(target_arch = "s390x"))]
#[repr(C)]
struct msgtemplate {
    n: nlmsghdr,
    g: genlmsghdr,
    buf: [c_char; 1024],
}

#[cfg(not(target_arch = "s390x"))]
const SMC_GENL_FAMILY_NAME: *const c_char = b"SMC_GEN_NETLINK\0".as_ptr() as *const c_char;
#[cfg(not(target_arch = "s390x"))]
const SMC_BPFTEST_UEID: *const c_char = b"SMC-BPFTEST-UEID\0".as_ptr() as *const c_char;

#[cfg(not(target_arch = "s390x"))]
static mut smc_nl_family_id: u16 = -1i16 as u16;

#[cfg(not(target_arch = "s390x"))]
unsafe fn GENLMSG_DATA(glh: *mut msgtemplate) -> *mut c_void {
    (NLMSG_DATA(&mut (*glh).n) as *mut c_char).add(GENL_HDRLEN as usize) as *mut c_void
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn GENLMSG_PAYLOAD(glh: *mut nlmsghdr) -> c_int {
    NLMSG_PAYLOAD(glh, 0) - GENL_HDRLEN
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn NLA_DATA(na: *mut nlattr) -> *mut c_void {
    (na as *mut c_char).add(NLA_HDRLEN as usize) as *mut c_void
}

#[cfg(not(target_arch = "s390x"))]
fn NLA_PAYLOAD(len: c_int) -> c_int {
    len - NLA_HDRLEN
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn send_cmd(
    fd: c_int,
    nlmsg_type: __u16,
    nlmsg_pid: __u32,
    nlmsg_flags: __u16,
    genl_cmd: __u8,
    nla_type: __u16,
    nla_data: *mut c_void,
    nla_len: c_int,
) -> c_int {
    let mut na: *mut nlattr;
    let mut nladdr: sockaddr_nl = zeroed();
    let mut r: c_int;
    let mut buflen: c_int;
    let mut buf: *mut c_char;

    let mut msg: msgtemplate = zeroed();

    msg.n.nlmsg_len = NLMSG_LENGTH(GENL_HDRLEN);
    msg.n.nlmsg_type = nlmsg_type;
    msg.n.nlmsg_flags = nlmsg_flags;
    msg.n.nlmsg_seq = 0;
    msg.n.nlmsg_pid = nlmsg_pid;
    msg.g.cmd = genl_cmd;
    msg.g.version = 1;
    na = GENLMSG_DATA(&mut msg) as *mut nlattr;
    (*na).nla_type = nla_type;
    (*na).nla_len = (nla_len + 1 + NLA_HDRLEN) as __u16;
    memcpy(NLA_DATA(na), nla_data, nla_len as usize);
    msg.n.nlmsg_len += NLMSG_ALIGN((*na).nla_len as c_int) as __u32;

    buf = &mut msg as *mut msgtemplate as *mut c_char;
    buflen = msg.n.nlmsg_len as c_int;
    memset(
        &mut nladdr as *mut sockaddr_nl as *mut c_void,
        0,
        size_of::<sockaddr_nl>(),
    );
    nladdr.nl_family = AF_NETLINK as __u16;

    loop {
        r = sendto(
            fd,
            buf as *const c_void,
            buflen as usize,
            0,
            &mut nladdr as *mut sockaddr_nl as *mut sockaddr,
            size_of::<sockaddr_nl>() as socklen_t,
        ) as c_int;
        if r >= buflen {
            break;
        }
        if r > 0 {
            buf = buf.add(r as usize);
            buflen -= r;
        } else if *__errno_location() != EAGAIN {
            return -1;
        }
    }
    0
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn get_smc_nl_family_id() -> bool {
    let mut nl_src: sockaddr_nl = zeroed();
    let mut msg: msgtemplate = zeroed();
    let mut nl: *mut nlattr;
    let fd: c_int;
    let mut ret: c_int;
    let pid: pid_t;

    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    if !ASSERT_OK_FD(fd, b"nl_family socket\0".as_ptr() as *const c_char) {
        return false;
    }

    pid = getpid();

    memset(
        &mut nl_src as *mut sockaddr_nl as *mut c_void,
        0,
        size_of::<sockaddr_nl>(),
    );
    nl_src.nl_family = AF_NETLINK as __u16;
    nl_src.nl_pid = pid as __u32;

    ret = bind(
        fd,
        &mut nl_src as *mut sockaddr_nl as *mut sockaddr,
        size_of::<sockaddr_nl>() as socklen_t,
    );
    if !ASSERT_OK(ret, b"nl_family bind\0".as_ptr() as *const c_char) {
        close(fd);
        return false;
    }

    ret = send_cmd(
        fd,
        GENL_ID_CTRL as __u16,
        pid as __u32,
        NLM_F_REQUEST as __u16,
        CTRL_CMD_GETFAMILY as __u8,
        CTRL_ATTR_FAMILY_NAME as __u16,
        SMC_GENL_FAMILY_NAME as *mut c_void,
        strlen(SMC_GENL_FAMILY_NAME) as c_int,
    );
    if !ASSERT_OK(ret, b"nl_family query\0".as_ptr() as *const c_char) {
        close(fd);
        return false;
    }

    ret = recv(
        fd,
        &mut msg as *mut msgtemplate as *mut c_void,
        size_of::<msgtemplate>(),
        0,
    ) as c_int;
    if msg.n.nlmsg_type == NLMSG_ERROR as __u16 {
        close(fd);
        return false;
    }
    if !ASSERT_FALSE(
        ret < 0 || !NLMSG_OK(&mut msg.n, ret),
        b"nl_family response\0".as_ptr() as *const c_char,
    ) {
        close(fd);
        return false;
    }

    nl = GENLMSG_DATA(&mut msg) as *mut nlattr;
    nl = (nl as *mut c_char).add(NLA_ALIGN((*nl).nla_len as c_int) as usize) as *mut nlattr;
    if !ASSERT_EQ(
        (*nl).nla_type as c_longlong,
        CTRL_ATTR_FAMILY_ID as c_longlong,
        b"nl_family nla type\0".as_ptr() as *const c_char,
    ) {
        close(fd);
        return false;
    }

    smc_nl_family_id = *(NLA_DATA(nl) as *mut u16);
    close(fd);
    true
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn smc_ueid(op: c_int) -> bool {
    let mut nl_src: sockaddr_nl = zeroed();
    let mut msg: msgtemplate = zeroed();
    let mut err: *mut nlmsgerr;
    let mut test_ueid: [c_char; 32] = [0; 32];
    let fd: c_int;
    let mut ret: c_int;
    let pid: pid_t;

    /* UEID required */
    memset(
        test_ueid.as_mut_ptr() as *mut c_void,
        0x20,
        size_of::<[c_char; 32]>(),
    );
    memcpy(
        test_ueid.as_mut_ptr() as *mut c_void,
        SMC_BPFTEST_UEID as *const c_void,
        strlen(SMC_BPFTEST_UEID),
    );
    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    if !ASSERT_OK_FD(fd, b"ueid socket\0".as_ptr() as *const c_char) {
        return false;
    }

    pid = getpid();
    memset(
        &mut nl_src as *mut sockaddr_nl as *mut c_void,
        0,
        size_of::<sockaddr_nl>(),
    );
    nl_src.nl_family = AF_NETLINK as __u16;
    nl_src.nl_pid = pid as __u32;

    ret = bind(
        fd,
        &mut nl_src as *mut sockaddr_nl as *mut sockaddr,
        size_of::<sockaddr_nl>() as socklen_t,
    );
    if !ASSERT_OK(ret, b"ueid bind\0".as_ptr() as *const c_char) {
        close(fd);
        return false;
    }

    ret = send_cmd(
        fd,
        smc_nl_family_id,
        pid as __u32,
        (NLM_F_REQUEST | NLM_F_ACK) as __u16,
        op as __u8,
        SMC_NLA_EID_TABLE_ENTRY as __u16,
        test_ueid.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 32]>() as c_int,
    );
    if !ASSERT_OK(ret, b"ueid cmd\0".as_ptr() as *const c_char) {
        close(fd);
        return false;
    }

    ret = recv(
        fd,
        &mut msg as *mut msgtemplate as *mut c_void,
        size_of::<msgtemplate>(),
        0,
    ) as c_int;
    if !ASSERT_FALSE(
        ret < 0 || !NLMSG_OK(&mut msg.n, ret),
        b"ueid response\0".as_ptr() as *const c_char,
    ) {
        close(fd);
        return false;
    }

    if msg.n.nlmsg_type == NLMSG_ERROR as __u16 {
        err = NLMSG_DATA(&mut msg.n) as *mut nlmsgerr;
        match op {
            SMC_NETLINK_REMOVE_UEID => {
                if !ASSERT_FALSE(
                    (*err).error != 0 && (*err).error != -ENOENT,
                    b"ueid remove\0".as_ptr() as *const c_char,
                ) {
                    close(fd);
                    return false;
                }
            }
            SMC_NETLINK_ADD_UEID => {
                if !ASSERT_OK((*err).error, b"ueid add\0".as_ptr() as *const c_char) {
                    close(fd);
                    return false;
                }
            }
            _ => {}
        }
    }
    close(fd);
    true
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn setup_ueid() -> bool {
    /* get smc nl id */
    if !get_smc_nl_family_id() {
        return false;
    }
    /* clear old ueid for bpftest */
    smc_ueid(SMC_NETLINK_REMOVE_UEID);
    /* smc-loopback required ueid */
    smc_ueid(SMC_NETLINK_ADD_UEID)
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn cleanup_ueid() {
    smc_ueid(SMC_NETLINK_REMOVE_UEID);
}

unsafe fn setup_netns() -> bool {
    test_netns = netns_new(TEST_NS, true);
    if !ASSERT_OK_PTR(test_netns as *const c_void, b"open net namespace\0".as_ptr() as *const c_char) {
        return false;
    }

    if SYS(b"ip addr add 127.0.1.0/8 dev lo\0".as_ptr() as *const c_char) != 0 {
        netns_free(test_netns);
        return false;
    }
    if SYS(b"ip addr add 127.0.2.0/8 dev lo\0".as_ptr() as *const c_char) != 0 {
        netns_free(test_netns);
        return false;
    }

    true
}

unsafe fn cleanup_netns() {
    netns_free(test_netns);
}

unsafe fn setup_smc() -> bool {
    if !setup_ueid() {
        return false;
    }

    if !setup_netns() {
        cleanup_ueid();
        return false;
    }

    true
}

unsafe extern "C" fn set_client_addr_cb(fd: c_int, opts: *mut c_void) -> c_int {
    let src: *const c_char = opts as *const c_char;
    let mut localaddr: sockaddr_in = zeroed();

    localaddr.sin_family = AF_INET as __u16;
    localaddr.sin_port = htons(0);
    localaddr.sin_addr.s_addr = inet_addr(src);
    (!ASSERT_OK(
        bind(
            fd,
            &mut localaddr as *mut sockaddr_in as *mut sockaddr,
            size_of::<sockaddr_in>() as socklen_t,
        ),
        b"client bind\0".as_ptr() as *const c_char,
    )) as c_int
}

unsafe fn run_link(src: *const c_char, dst: *const c_char, port: c_int) {
    let mut opts: network_helper_opts = zeroed();
    let server: c_int;
    let client: c_int;

    server = start_server_str(AF_INET, SOCK_STREAM, dst, port, ptr::null_mut());
    if !ASSERT_OK_FD(server, b"start service_1\0".as_ptr() as *const c_char) {
        return;
    }

    opts.proto = IPPROTO_TCP;
    opts.post_socket_cb = Some(set_client_addr_cb);
    opts.cb_opts = src as *mut c_void;

    client = connect_to_fd_opts(server, &mut opts);
    if !ASSERT_OK_FD(client, b"start connect\0".as_ptr() as *const c_char) {
        close(server);
        return;
    }

    close(client);
    close(server);
}

unsafe fn block_link(map_fd: c_int, src: *const c_char, dst: *const c_char) {
    let mut val = smc_policy_ip_value {
        mode: 0, /* block */
    };
    let mut key = smc_policy_ip_key {
        sip: inet_addr(src),
        dip: inet_addr(dst),
    };

    bpf_map_update_elem(
        map_fd,
        &mut key as *mut smc_policy_ip_key as *const c_void,
        &mut val as *mut smc_policy_ip_value as *const c_void,
        BPF_ANY,
    );
}

/*
 * This test describes a real-life service topology as follows:
 *
 *                             +-------------> service_1
 *            link 1           |                     |
 *   +--------------------> server                   |  link 2
 *   |                         |                     V
 *   |                         +-------------> service_2
 *   |        link 3
 *  client -------------------> server_via_unsafe_path -> service_3
 *
 * Among them,
 * 1. link-1 is very suitable for using SMC.
 * 2. link-2 is not suitable for using SMC, because the mode of this link is
 *    kind of short-link services.
 * 3. link-3 is also not suitable for using SMC, because the RDMA link is
 *    unavailable and needs to go through a long timeout before it can fallback
 *    to TCP.
 * To achieve this goal, we use a customized SMC ip strategy via smc_hs_ctrl.
 */
unsafe fn test_topo() {
    let skel: *mut bpf_smc;
    let mut rc: c_int;
    let map_fd: c_int;

    skel = bpf_smc__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"bpf_smc__open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    rc = bpf_smc__attach(skel);
    if !ASSERT_OK(rc, b"bpf_smc__attach\0".as_ptr() as *const c_char) {
        bpf_smc__destroy(skel);
        return;
    }

    map_fd = bpf_map__fd((*(*skel).maps).smc_policy_ip);
    if !ASSERT_OK_FD(map_fd, b"bpf_map__fd\0".as_ptr() as *const c_char) {
        bpf_smc__destroy(skel);
        return;
    }

    /* Mock the process of transparent replacement, since we will modify
     * protocol to ipproto_smc accropding to it via
     * fmod_ret/update_socket_protocol.
     */
    write_sysctl(
        b"/proc/sys/net/smc/hs_ctrl\0".as_ptr() as *const c_char,
        b"linkcheck\0".as_ptr() as *const c_char,
    );

    /* Configure ip strat */
    block_link(map_fd, CLIENT_IP, SERVER_IP_VIA_RISK_PATH);
    block_link(map_fd, SERVER_IP, SERVER_IP);

    /* should go with smc */
    run_link(CLIENT_IP, SERVER_IP, SERVICE_1);
    /* should go with smc fallback */
    run_link(SERVER_IP, SERVER_IP, SERVICE_2);

    ASSERT_EQ((*(*skel).bss).smc_cnt as c_longlong, 2, b"smc count\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        (*(*skel).bss).fallback_cnt as c_longlong,
        1,
        b"fallback count\0".as_ptr() as *const c_char,
    );

    /* should go with smc */
    run_link(CLIENT_IP, SERVER_IP, SERVICE_2);

    ASSERT_EQ((*(*skel).bss).smc_cnt as c_longlong, 3, b"smc count\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        (*(*skel).bss).fallback_cnt as c_longlong,
        1,
        b"fallback count\0".as_ptr() as *const c_char,
    );

    /* should go with smc fallback */
    run_link(CLIENT_IP, SERVER_IP_VIA_RISK_PATH, SERVICE_3);

    ASSERT_EQ((*(*skel).bss).smc_cnt as c_longlong, 4, b"smc count\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        (*(*skel).bss).fallback_cnt as c_longlong,
        2,
        b"fallback count\0".as_ptr() as *const c_char,
    );

    bpf_smc__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_bpf_smc() {
    if !setup_smc() {
        printf(b"setup for smc test failed, test SKIP:\n\0".as_ptr() as *const c_char);
        test__skip();
        return;
    }

    if test__start_subtest(b"topo\0".as_ptr() as *const c_char) {
        test_topo();
    }

    cleanup_ueid();
    cleanup_netns();
}

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type c_longlong = i64;
type pid_t = c_int;
type socklen_t = u32;

#[repr(C)]
struct netns_obj {
    _private: [u8; 0],
}

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: __u32,
    nlmsg_type: __u16,
    nlmsg_flags: __u16,
    nlmsg_seq: __u32,
    nlmsg_pid: __u32,
}

#[repr(C)]
struct genlmsghdr {
    cmd: __u8,
    version: __u8,
    reserved: __u16,
}

#[repr(C)]
struct nlattr {
    nla_len: __u16,
    nla_type: __u16,
}

#[repr(C)]
struct nlmsgerr {
    error: c_int,
    msg: nlmsghdr,
}

#[repr(C)]
struct sockaddr {
    sa_family: __u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_nl {
    nl_family: __u16,
    nl_pad: __u16,
    nl_pid: __u32,
    nl_groups: __u32,
}

#[repr(C)]
struct in_addr {
    s_addr: __u32,
}

#[repr(C)]
struct sockaddr_in {
    sin_family: __u16,
    sin_port: __u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct network_helper_opts {
    proto: c_int,
    post_socket_cb: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    cb_opts: *mut c_void,
}

#[repr(C)]
struct bpf_smc {
    maps: *mut bpf_smc_maps,
    bss: *mut bpf_smc_bss,
}

#[repr(C)]
struct bpf_smc_maps {
    smc_policy_ip: *mut c_void,
}

#[repr(C)]
struct bpf_smc_bss {
    smc_cnt: __u32,
    fallback_cnt: __u32,
}

const AF_INET: c_int = 2;
const AF_NETLINK: c_int = 16;
const SOCK_STREAM: c_int = 1;
const SOCK_RAW: c_int = 3;
const IPPROTO_TCP: c_int = 6;
const NETLINK_GENERIC: c_int = 16;
const GENL_ID_CTRL: c_int = 16;
const NLM_F_REQUEST: c_int = 0x01;
const NLM_F_ACK: c_int = 0x04;
const CTRL_CMD_GETFAMILY: c_int = 3;
const CTRL_ATTR_FAMILY_ID: c_int = 1;
const CTRL_ATTR_FAMILY_NAME: c_int = 2;
const NLMSG_ERROR: c_int = 0x2;
const GENL_HDRLEN: c_int = 4;
const NLA_HDRLEN: c_int = 4;
const EAGAIN: c_int = 11;
const ENOENT: c_int = 2;
const BPF_ANY: u64 = 0;

fn NLMSG_ALIGN(len: c_int) -> c_int {
    (len + 3) & !3
}

fn NLA_ALIGN(len: c_int) -> c_int {
    (len + 3) & !3
}

fn NLMSG_LENGTH(len: c_int) -> __u32 {
    (NLMSG_ALIGN(size_of::<nlmsghdr>() as c_int) + len) as __u32
}

unsafe fn NLMSG_DATA(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as *mut c_char).add(NLMSG_ALIGN(size_of::<nlmsghdr>() as c_int) as usize) as *mut c_void
}

fn NLMSG_PAYLOAD(nlh: *mut nlmsghdr, len: c_int) -> c_int {
    unsafe { (*nlh).nlmsg_len as c_int - NLMSG_ALIGN(size_of::<nlmsghdr>() as c_int) - len }
}

fn NLMSG_OK(nlh: *mut nlmsghdr, len: c_int) -> bool {
    len >= size_of::<nlmsghdr>() as c_int
        && unsafe { (*nlh).nlmsg_len as c_int >= size_of::<nlmsghdr>() as c_int }
        && unsafe { (*nlh).nlmsg_len as c_int <= len }
}

unsafe extern "C" {
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_FALSE(condition: bool, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_longlong, right: c_longlong, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn SYS(command: *const c_char) -> c_int;

    fn netns_new(name: *const c_char, attach: bool) -> *mut netns_obj;
    fn netns_free(ns: *mut netns_obj);
    fn start_server_str(
        family: c_int,
        socktype: c_int,
        addr: *const c_char,
        port: c_int,
        opts: *mut c_void,
    ) -> c_int;
    fn connect_to_fd_opts(server_fd: c_int, opts: *mut network_helper_opts) -> c_int;
    fn write_sysctl(path: *const c_char, value: *const c_char);
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn bpf_smc__open_and_load() -> *mut bpf_smc;
    fn bpf_smc__attach(skel: *mut bpf_smc) -> c_int;
    fn bpf_smc__destroy(skel: *mut bpf_smc);
    fn bpf_map__fd(map: *mut c_void) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *mut sockaddr, addrlen: socklen_t) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *mut sockaddr,
        addrlen: socklen_t,
    ) -> isize;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn htons(hostshort: __u16) -> __u16;
    fn inet_addr(cp: *const c_char) -> __u32;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
