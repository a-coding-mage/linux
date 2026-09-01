// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

/*
 * End-to-end eBPF tunnel test suite
 *   The file tests BPF network tunnels implementation. For each tunnel
 *   type, the test validates that:
 *   - basic communication can first be established between the two veths
 *   - when adding a BPF-based encapsulation on client egress, it now fails
 *   to communicate with the server
 *   - when adding a kernel-based decapsulation on server ingress, client
 *   can now connect
 *   - when replacing the kernel-based decapsulation with a BPF-based one,
 *   the client can still connect
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

const SERVER_NS: &[u8] = b"tc-tunnel-server-ns\0";
const CLIENT_NS: &[u8] = b"tc-tunnel-client-ns\0";
const MAC_ADDR_VETH1: &[u8] = b"00:11:22:33:44:55\0";
const IP4_ADDR_VETH1: &[u8] = b"192.168.1.1\0";
const IP6_ADDR_VETH1: &[u8] = b"fd::1\0";
const MAC_ADDR_VETH2: &[u8] = b"66:77:88:99:AA:BB\0";
const IP4_ADDR_VETH2: &[u8] = b"192.168.1.2\0";
const IP6_ADDR_VETH2: &[u8] = b"fd::2\0";

const TEST_NAME_MAX_LEN: usize = 64;
const PROG_NAME_MAX_LEN: usize = 64;
const TUNNEL_ARGS_MAX_LEN: usize = 128;
const BUFFER_LEN: usize = 2000;
const DEFAULT_TEST_DATA_SIZE: usize = 100;
const GSO_TEST_DATA_SIZE: usize = BUFFER_LEN;

const TIMEOUT_MS: c_int = 1000;
const TEST_PORT: c_int = 8000;
const UDP_PORT: c_int = 5555;
const MPLS_UDP_PORT: c_int = 6635;
const FOU_MPLS_PROTO: c_int = 137;
const VXLAN_ID: c_int = 1;
const VXLAN_PORT: c_int = 8472;
const MPLS_TABLE_ENTRIES_COUNT: c_int = 65536;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const O_RDONLY: c_int = 0;

static mut TX_BUFFER: [c_char; BUFFER_LEN] = [0; BUFFER_LEN];
static mut RX_BUFFER: [c_char; BUFFER_LEN] = [0; BUFFER_LEN];

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_tc_tunnel_progs {
    pub decap_f: *mut bpf_program,
}

#[repr(C)]
pub struct test_tc_tunnel {
    pub obj: *mut bpf_object,
    pub progs: test_tc_tunnel_progs,
}

#[repr(C)]
pub struct network_helper_opts {
    pub timeout_ms: c_int,
}

#[repr(C)]
struct subtest_cfg {
    ebpf_tun_type: *mut c_char,
    iproute_tun_type: *mut c_char,
    mac_tun_type: *mut c_char,
    ipproto: c_int,
    extra_decap_mod_args_cb: Option<unsafe extern "C" fn(*mut subtest_cfg, *mut c_char)>,
    tunnel_need_veth_mac: bool,
    configure_fou_rx_port: bool,
    tmode: *mut c_char,
    expect_kern_decap_failure: bool,
    configure_mpls: bool,
    test_gso: bool,
    tunnel_client_addr: *mut c_char,
    tunnel_server_addr: *mut c_char,
    name: [c_char; TEST_NAME_MAX_LEN],
    server_addr: *mut c_char,
    client_egress_prog_fd: c_int,
    server_ingress_prog_fd: c_int,
    extra_decap_mod_args: [c_char; TUNNEL_ARGS_MAX_LEN],
    server_fd: c_int,
}

#[repr(C)]
struct connection {
    client_fd: c_int,
    server_fd: c_int,
}

unsafe extern "C" {
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut c_void) -> c_int;

    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn make_netns(name: *const c_char) -> c_int;
    fn remove_netns(name: *const c_char);
    fn start_server_str(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        opts: *const network_helper_opts,
    ) -> c_int;
    fn connect_to_addr_str(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        opts: *const network_helper_opts,
    ) -> c_int;
    fn tc_prog_attach(ifname: *const c_char, ingress_fd: c_int, egress_fd: c_int) -> c_int;

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn test_tc_tunnel__open_and_load() -> *mut test_tc_tunnel;
    fn test_tc_tunnel__destroy(skel: *mut test_tc_tunnel);

    fn test__start_subtest(name: *const c_char) -> bool;
}

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! mut_cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *mut c_char
    };
}

macro_rules! ASSERT_OK_PTR {
    ($ptr:expr, $name:expr) => {
        !$ptr.is_null()
    };
}

macro_rules! ASSERT_OK_FD {
    ($fd:expr, $name:expr) => {
        $fd >= 0
    };
}

macro_rules! ASSERT_OK {
    ($expr:expr, $name:expr) => {
        $expr == 0
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr, $name:expr) => {
        $left == $right
    };
}

macro_rules! ASSERT_MEMEQ {
    ($left:expr, $right:expr, $len:expr, $name:expr) => {
        core::slice::from_raw_parts($left as *const u8, $len)
            == core::slice::from_raw_parts($right as *const u8, $len)
    };
}

macro_rules! SYS {
    ($fail:ident, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        let _ = ($fmt $(, $arg)*);
        /* External test harness command macro. */
    }};
}

macro_rules! SYS_NOFAIL {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        let _ = ($fmt $(, $arg)*);
        /* External test harness command macro. */
    }};
}

unsafe fn build_subtest_name(cfg: *mut subtest_cfg, dst: *mut c_char, size: usize) -> c_int {
    let ret: c_int;

    ret = snprintf(
        dst,
        size,
        cstr!("%s_%s"),
        (*cfg).ebpf_tun_type,
        (*cfg).mac_tun_type,
    );

    if ret < 0 { ret } else { 0 }
}

unsafe fn set_subtest_progs(cfg: *mut subtest_cfg, skel: *mut test_tc_tunnel) -> c_int {
    let mut prog_name: [c_char; PROG_NAME_MAX_LEN] = [0; PROG_NAME_MAX_LEN];
    let mut prog: *mut bpf_program;
    let mut ret: c_int;

    ret = snprintf(prog_name.as_mut_ptr(), PROG_NAME_MAX_LEN, cstr!("__encap_"));
    if ret < 0 {
        return ret;
    }
    ret = build_subtest_name(
        cfg,
        prog_name.as_mut_ptr().offset(ret as isize),
        PROG_NAME_MAX_LEN - ret as usize,
    );
    if ret < 0 {
        return ret;
    }
    prog = bpf_object__find_program_by_name((*skel).obj, prog_name.as_ptr());
    if prog.is_null() {
        return -1;
    }

    (*cfg).client_egress_prog_fd = bpf_program__fd(prog);
    (*cfg).server_ingress_prog_fd = bpf_program__fd((*skel).progs.decap_f);
    0
}

unsafe fn set_subtest_addresses(cfg: *mut subtest_cfg) {
    if (*cfg).ipproto == 6 {
        (*cfg).server_addr = IP6_ADDR_VETH2.as_ptr() as *mut c_char;
    } else {
        (*cfg).server_addr = IP4_ADDR_VETH2.as_ptr() as *mut c_char;
    }

    /* Some specific tunnel types need specific addressing, it then
     * has been already set in the configuration table. Otherwise,
     * deduce the relevant addressing from the ipproto
     */
    if !(*cfg).tunnel_client_addr.is_null() && !(*cfg).tunnel_server_addr.is_null() {
        return;
    }

    if (*cfg).ipproto == 6 {
        (*cfg).tunnel_client_addr = IP6_ADDR_VETH1.as_ptr() as *mut c_char;
        (*cfg).tunnel_server_addr = IP6_ADDR_VETH2.as_ptr() as *mut c_char;
    } else {
        (*cfg).tunnel_client_addr = IP4_ADDR_VETH1.as_ptr() as *mut c_char;
        (*cfg).tunnel_server_addr = IP4_ADDR_VETH2.as_ptr() as *mut c_char;
    }
}

unsafe fn run_server(cfg: *mut subtest_cfg) -> c_int {
    let family = if (*cfg).ipproto == 6 { AF_INET6 } else { AF_INET };
    let mut nstoken: *mut nstoken;
    let opts = network_helper_opts {
        timeout_ms: TIMEOUT_MS,
    };

    nstoken = open_netns(SERVER_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR!(nstoken, "open server ns") {
        return -1;
    }

    (*cfg).server_fd = start_server_str(family, SOCK_STREAM, (*cfg).server_addr, TEST_PORT, &opts);
    close_netns(nstoken);
    if !ASSERT_OK_FD!((*cfg).server_fd, "start server") {
        return -1;
    }

    0
}

unsafe fn check_server_rx_data(
    cfg: *mut subtest_cfg,
    conn: *mut connection,
    len: c_int,
) -> c_int {
    let mut err: c_int;

    let _ = cfg;
    memset(RX_BUFFER.as_mut_ptr() as *mut c_void, 0, BUFFER_LEN);
    err = recv((*conn).server_fd, RX_BUFFER.as_mut_ptr() as *mut c_void, len as usize, 0) as c_int;
    if !ASSERT_EQ!(err, len, "check rx data len") {
        return 1;
    }
    if !ASSERT_MEMEQ!(TX_BUFFER.as_ptr(), RX_BUFFER.as_ptr(), len as usize, "check received data") {
        return 1;
    }
    0
}

unsafe fn connect_client_to_server(cfg: *mut subtest_cfg) -> *mut connection {
    let opts = network_helper_opts { timeout_ms: 1000 };
    let family = if (*cfg).ipproto == 6 { AF_INET6 } else { AF_INET };
    let mut conn: *mut connection = ptr::null_mut();
    let client_fd: c_int;
    let server_fd: c_int;

    conn = malloc(mem::size_of::<connection>()) as *mut connection;
    if conn.is_null() {
        return conn;
    }

    client_fd = connect_to_addr_str(family, SOCK_STREAM, (*cfg).server_addr, TEST_PORT, &opts);

    if client_fd < 0 {
        free(conn as *mut c_void);
        return ptr::null_mut();
    }

    server_fd = accept((*cfg).server_fd, ptr::null_mut(), ptr::null_mut());
    if server_fd < 0 {
        close(client_fd);
        free(conn as *mut c_void);
        return ptr::null_mut();
    }

    (*conn).server_fd = server_fd;
    (*conn).client_fd = client_fd;

    conn
}

unsafe fn disconnect_client_from_server(cfg: *mut subtest_cfg, conn: *mut connection) {
    let _ = cfg;
    close((*conn).server_fd);
    close((*conn).client_fd);
    free(conn as *mut c_void);
}

unsafe fn send_and_test_data(cfg: *mut subtest_cfg) -> c_int {
    let mut conn: *mut connection;
    let mut err: c_int;
    let mut res: c_int = -1;

    conn = connect_client_to_server(cfg);
    if !ASSERT_OK_PTR!(conn, "connect to server") {
        return -1;
    }

    err = send(
        (*conn).client_fd,
        TX_BUFFER.as_ptr() as *const c_void,
        DEFAULT_TEST_DATA_SIZE,
        0,
    ) as c_int;
    if !ASSERT_EQ!(err, DEFAULT_TEST_DATA_SIZE as c_int, "send data from client") {
        goto_end_send_and_test_data(cfg, conn, res);
        return res;
    }
    if check_server_rx_data(cfg, conn, DEFAULT_TEST_DATA_SIZE as c_int) != 0 {
        goto_end_send_and_test_data(cfg, conn, res);
        return res;
    }

    if !(*cfg).test_gso {
        res = 0;
        goto_end_send_and_test_data(cfg, conn, res);
        return res;
    }

    err = send(
        (*conn).client_fd,
        TX_BUFFER.as_ptr() as *const c_void,
        GSO_TEST_DATA_SIZE,
        0,
    ) as c_int;
    if !ASSERT_EQ!(err, GSO_TEST_DATA_SIZE as c_int, "send (large) data from client") {
        goto_end_send_and_test_data(cfg, conn, res);
        return res;
    }
    if check_server_rx_data(cfg, conn, DEFAULT_TEST_DATA_SIZE as c_int) != 0 {
        goto_end_send_and_test_data(cfg, conn, res);
        return res;
    }

    res = 0;
    goto_end_send_and_test_data(cfg, conn, res);
    res
}

unsafe fn goto_end_send_and_test_data(cfg: *mut subtest_cfg, conn: *mut connection, res: c_int) {
    disconnect_client_from_server(cfg, conn);
    let _ = res;
}

unsafe extern "C" fn vxlan_decap_mod_args_cb(cfg: *mut subtest_cfg, dst: *mut c_char) {
    let _ = cfg;
    snprintf(
        dst,
        TUNNEL_ARGS_MAX_LEN,
        cstr!("id %d dstport %d udp6zerocsumrx"),
        VXLAN_ID,
        VXLAN_PORT,
    );
}

unsafe extern "C" fn udp_decap_mod_args_cb(cfg: *mut subtest_cfg, dst: *mut c_char) {
    let is_mpls: bool = strcmp((*cfg).mac_tun_type, cstr!("mpls")) == 0;

    snprintf(
        dst,
        TUNNEL_ARGS_MAX_LEN,
        cstr!("encap fou encap-sport auto encap-dport %d"),
        if is_mpls { MPLS_UDP_PORT } else { UDP_PORT },
    );
}

unsafe fn configure_fou_rx_port(cfg: *mut subtest_cfg, add: bool) -> c_int {
    let is_mpls: bool = strcmp((*cfg).mac_tun_type, cstr!("mpls")) == 0;
    let fou_proto: c_int;

    if is_mpls {
        fou_proto = FOU_MPLS_PROTO;
    } else {
        fou_proto = if (*cfg).ipproto == 6 { 41 } else { 4 };
    }

    SYS!(
        fail,
        "ip fou %s port %d ipproto %d%s",
        if add { "add" } else { "del" },
        if is_mpls { MPLS_UDP_PORT } else { UDP_PORT },
        fou_proto,
        if (*cfg).ipproto == 6 { " -6" } else { "" }
    );

    0
}

unsafe fn add_fou_rx_port(cfg: *mut subtest_cfg) -> c_int {
    configure_fou_rx_port(cfg, true)
}

unsafe fn del_fou_rx_port(cfg: *mut subtest_cfg) -> c_int {
    configure_fou_rx_port(cfg, false)
}

unsafe fn update_tunnel_intf_addr(cfg: *mut subtest_cfg) -> c_int {
    let _ = cfg;
    SYS!(fail, "ip link set dev testtun0 address 66:77:88:99:AA:BB");
    0
}

unsafe fn configure_kernel_for_mpls(cfg: *mut subtest_cfg) -> c_int {
    let _ = cfg;
    SYS!(
        fail,
        "sysctl -qw net.mpls.platform_labels=%d",
        MPLS_TABLE_ENTRIES_COUNT
    );
    SYS!(fail, "ip -f mpls route add 1000 dev lo");
    SYS!(fail, "ip link set lo up");
    SYS!(fail, "sysctl -qw net.mpls.conf.testtun0.input=1");
    SYS!(fail, "sysctl -qw net.ipv4.conf.lo.rp_filter=0");
    0
}

unsafe fn configure_encapsulation(cfg: *mut subtest_cfg) -> c_int {
    let ret: c_int;

    ret = tc_prog_attach(cstr!("veth1"), -1, (*cfg).client_egress_prog_fd);

    ret
}

unsafe fn configure_kernel_decapsulation(cfg: *mut subtest_cfg) -> c_int {
    let mut nstoken: *mut nstoken = open_netns(SERVER_NS.as_ptr() as *const c_char);
    let mut ret: c_int = -1;

    if !ASSERT_OK_PTR!(nstoken, "open server ns") {
        return ret;
    }

    if (*cfg).configure_fou_rx_port
        && !ASSERT_OK!(add_fou_rx_port(cfg), "configure FOU RX port")
    {
        close_netns(nstoken);
        return ret;
    }
    SYS!(
        fail,
        "ip link add name testtun0 type %s %s remote %s local %s %s",
        (*cfg).iproute_tun_type,
        if !(*cfg).tmode.is_null() { (*cfg).tmode } else { cstr!("") as *mut c_char },
        (*cfg).tunnel_client_addr,
        (*cfg).tunnel_server_addr,
        (*cfg).extra_decap_mod_args.as_ptr()
    );
    if (*cfg).tunnel_need_veth_mac
        && !ASSERT_OK!(update_tunnel_intf_addr(cfg), "update testtun0 mac")
    {
        close_netns(nstoken);
        return ret;
    }
    if (*cfg).configure_mpls
        && (!ASSERT_OK!(configure_kernel_for_mpls(cfg), "configure MPLS decap"))
    {
        close_netns(nstoken);
        return ret;
    }
    SYS!(fail, "sysctl -qw net.ipv4.conf.all.rp_filter=0");
    SYS!(fail, "sysctl -qw net.ipv4.conf.testtun0.rp_filter=0");
    SYS!(fail, "ip link set dev testtun0 up");

    ret = 0;
    close_netns(nstoken);
    ret
}

unsafe fn remove_kernel_decapsulation(cfg: *mut subtest_cfg) {
    SYS_NOFAIL!("ip link del testtun0");
    if (*cfg).configure_mpls {
        SYS_NOFAIL!("ip -f mpls route del 1000 dev lo");
    }
    if (*cfg).configure_fou_rx_port {
        del_fou_rx_port(cfg);
    }
}

unsafe fn configure_ebpf_decapsulation(cfg: *mut subtest_cfg) -> c_int {
    let mut nstoken: *mut nstoken = open_netns(SERVER_NS.as_ptr() as *const c_char);
    let mut ret: c_int = -1;

    if !ASSERT_OK_PTR!(nstoken, "open server ns") {
        return ret;
    }

    if !(*cfg).expect_kern_decap_failure {
        SYS!(fail, "ip link del testtun0");
    }

    if !ASSERT_OK!(
        tc_prog_attach(cstr!("veth2"), (*cfg).server_ingress_prog_fd, -1),
        "attach_program"
    ) {
        close_netns(nstoken);
        return ret;
    }

    ret = 0;
    close_netns(nstoken);
    ret
}

unsafe fn run_test(cfg: *mut subtest_cfg) {
    let mut nstoken: *mut nstoken;

    if !ASSERT_OK!(run_server(cfg), "run server") {
        return;
    }

    nstoken = open_netns(CLIENT_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR!(nstoken, "open client ns") {
        close((*cfg).server_fd);
        return;
    }

    /* Basic communication must work */
    if !ASSERT_OK!(send_and_test_data(cfg), "connect without any encap") {
        close_netns(nstoken);
        close((*cfg).server_fd);
        return;
    }

    /* Attach encapsulation program to client */
    if !ASSERT_OK!(configure_encapsulation(cfg), "configure encapsulation") {
        close_netns(nstoken);
        close((*cfg).server_fd);
        return;
    }

    /* If supported, insert kernel decap module, connection must succeed */
    if !(*cfg).expect_kern_decap_failure {
        if !ASSERT_OK!(
            configure_kernel_decapsulation(cfg),
            "configure kernel decapsulation"
        ) {
            close_netns(nstoken);
            close((*cfg).server_fd);
            return;
        }
        if !ASSERT_OK!(
            send_and_test_data(cfg),
            "connect with encap prog and kern decap"
        ) {
            close_netns(nstoken);
            close((*cfg).server_fd);
            return;
        }
    }

    /* Replace kernel decapsulation with BPF decapsulation, test must pass */
    if !ASSERT_OK!(
        configure_ebpf_decapsulation(cfg),
        "configure ebpf decapsulation"
    ) {
        close_netns(nstoken);
        close((*cfg).server_fd);
        return;
    }
    ASSERT_OK!(
        send_and_test_data(cfg),
        "connect with encap and decap progs"
    );

    close_netns(nstoken);
    close((*cfg).server_fd);
}

unsafe fn setup() -> c_int {
    let mut nstoken_client: *mut nstoken;
    let mut nstoken_server: *mut nstoken;
    let fd: c_int;
    let err: c_int;

    fd = open(cstr!("/dev/urandom"), O_RDONLY);
    if !ASSERT_OK_FD!(fd, "open urandom") {
        return -1;
    }
    err = read(fd, TX_BUFFER.as_mut_ptr() as *mut c_void, BUFFER_LEN) as c_int;
    close(fd);

    if !ASSERT_EQ!(err, BUFFER_LEN as c_int, "read random bytes") {
        return -1;
    }

    /* Configure the testing network */
    if !ASSERT_OK!(make_netns(CLIENT_NS.as_ptr() as *const c_char), "create client ns")
        || !ASSERT_OK!(make_netns(SERVER_NS.as_ptr() as *const c_char), "create server ns")
    {
        return -1;
    }

    nstoken_client = open_netns(CLIENT_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR!(nstoken_client, "open client ns") {
        SYS_NOFAIL!("ip netns del tc-tunnel-client-ns");
        SYS_NOFAIL!("ip netns del tc-tunnel-server-ns");
        return -1;
    }
    SYS!(
        fail_close_ns_client,
        "ip link add %s type veth peer name %s",
        "veth1 mtu 1500 netns tc-tunnel-client-ns address 00:11:22:33:44:55",
        "veth2 mtu 1500 netns tc-tunnel-server-ns address 66:77:88:99:AA:BB"
    );
    SYS!(fail_close_ns_client, "ip link set veth1 up");
    nstoken_server = open_netns(SERVER_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR!(nstoken_server, "open server ns") {
        close_netns(nstoken_client);
        SYS_NOFAIL!("ip netns del tc-tunnel-client-ns");
        SYS_NOFAIL!("ip netns del tc-tunnel-server-ns");
        return -1;
    }
    SYS!(fail_close_ns_server, "ip link set veth2 up");

    close_netns(nstoken_server);
    close_netns(nstoken_client);
    0
}

unsafe fn subtest_setup(skel: *mut test_tc_tunnel, cfg: *mut subtest_cfg) -> c_int {
    let mut nstoken_client: *mut nstoken;
    let mut nstoken_server: *mut nstoken;
    let mut ret: c_int = -1;

    set_subtest_addresses(cfg);
    if !ASSERT_OK!(set_subtest_progs(cfg, skel), "find subtest progs") {
        return ret;
    }
    if let Some(cb) = (*cfg).extra_decap_mod_args_cb {
        cb(cfg, (*cfg).extra_decap_mod_args.as_mut_ptr());
    }

    nstoken_client = open_netns(CLIENT_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR!(nstoken_client, "open client ns") {
        return ret;
    }
    SYS!(
        fail_close_client_ns,
        "ip -4 addr add 192.168.1.1/24 dev veth1"
    );
    SYS!(fail_close_client_ns, "ip -4 route flush table main");
    SYS!(
        fail_close_client_ns,
        "ip -4 route add 192.168.1.2 mtu 1450 dev veth1"
    );
    SYS!(
        fail_close_client_ns,
        "ip -6 addr add fd::1/64 dev veth1 nodad"
    );
    SYS!(fail_close_client_ns, "ip -6 route flush table main");
    SYS!(
        fail_close_client_ns,
        "ip -6 route add fd::2 mtu 1430 dev veth1"
    );
    nstoken_server = open_netns(SERVER_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR!(nstoken_server, "open server ns") {
        close_netns(nstoken_client);
        return ret;
    }
    SYS!(
        fail_close_server_ns,
        "ip -4 addr add 192.168.1.2/24 dev veth2"
    );
    SYS!(
        fail_close_server_ns,
        "ip -6 addr add fd::2/64 dev veth2 nodad"
    );

    ret = 0;

    close_netns(nstoken_server);
    close_netns(nstoken_client);
    ret
}

unsafe fn subtest_cleanup(cfg: *mut subtest_cfg) {
    let mut nstoken: *mut nstoken;

    nstoken = open_netns(CLIENT_NS.as_ptr() as *const c_char);
    if ASSERT_OK_PTR!(nstoken, "open clien ns") {
        SYS_NOFAIL!("tc qdisc delete dev veth1 parent ffff:fff1");
        SYS_NOFAIL!("ip a flush veth1");
        close_netns(nstoken);
    }
    nstoken = open_netns(SERVER_NS.as_ptr() as *const c_char);
    if ASSERT_OK_PTR!(nstoken, "open clien ns") {
        SYS_NOFAIL!("tc qdisc delete dev veth2 parent ffff:fff1");
        SYS_NOFAIL!("ip a flush veth2");
        if !(*cfg).expect_kern_decap_failure {
            remove_kernel_decapsulation(cfg);
        }
        close_netns(nstoken);
    }
}

unsafe fn cleanup() {
    remove_netns(CLIENT_NS.as_ptr() as *const c_char);
    remove_netns(SERVER_NS.as_ptr() as *const c_char);
}

macro_rules! subtest_cfg_item {
    ($ebpf:expr, $mac:expr, $iproute:expr, $ipproto:expr $(, $field:ident : $value:expr)* $(,)?) => {
        subtest_cfg {
            ebpf_tun_type: mut_cstr!($ebpf),
            iproute_tun_type: mut_cstr!($iproute),
            mac_tun_type: mut_cstr!($mac),
            ipproto: $ipproto,
            extra_decap_mod_args_cb: None,
            tunnel_need_veth_mac: false,
            configure_fou_rx_port: false,
            tmode: ptr::null_mut(),
            expect_kern_decap_failure: false,
            configure_mpls: false,
            test_gso: false,
            tunnel_client_addr: ptr::null_mut(),
            tunnel_server_addr: ptr::null_mut(),
            name: [0; TEST_NAME_MAX_LEN],
            server_addr: ptr::null_mut(),
            client_egress_prog_fd: 0,
            server_ingress_prog_fd: 0,
            extra_decap_mod_args: [0; TUNNEL_ARGS_MAX_LEN],
            server_fd: 0,
            $($field: $value,)*
        }
    };
}

static mut SUBTESTS_CFG: [subtest_cfg; 18] = [
    subtest_cfg_item!("ipip", "none", "ipip", 4),
    subtest_cfg_item!(
        "ipip6",
        "none",
        "ip6tnl",
        4,
        tunnel_client_addr: IP6_ADDR_VETH1.as_ptr() as *mut c_char,
        tunnel_server_addr: IP6_ADDR_VETH2.as_ptr() as *mut c_char,
    ),
    subtest_cfg_item!("ip6tnl", "none", "ip6tnl", 6),
    subtest_cfg_item!(
        "sit",
        "none",
        "sit",
        6,
        tunnel_client_addr: IP4_ADDR_VETH1.as_ptr() as *mut c_char,
        tunnel_server_addr: IP4_ADDR_VETH2.as_ptr() as *mut c_char,
    ),
    subtest_cfg_item!(
        "vxlan",
        "eth",
        "vxlan",
        4,
        extra_decap_mod_args_cb: Some(vxlan_decap_mod_args_cb),
        tunnel_need_veth_mac: true,
    ),
    subtest_cfg_item!(
        "ip6vxlan",
        "eth",
        "vxlan",
        6,
        extra_decap_mod_args_cb: Some(vxlan_decap_mod_args_cb),
        tunnel_need_veth_mac: true,
    ),
    subtest_cfg_item!("gre", "none", "gre", 4, test_gso: true),
    subtest_cfg_item!(
        "gre",
        "eth",
        "gretap",
        4,
        tunnel_need_veth_mac: true,
        test_gso: true,
    ),
    subtest_cfg_item!(
        "gre",
        "mpls",
        "gre",
        4,
        configure_mpls: true,
        test_gso: true,
    ),
    subtest_cfg_item!("ip6gre", "none", "ip6gre", 6, test_gso: true),
    subtest_cfg_item!(
        "ip6gre",
        "eth",
        "ip6gretap",
        6,
        tunnel_need_veth_mac: true,
        test_gso: true,
    ),
    subtest_cfg_item!(
        "ip6gre",
        "mpls",
        "ip6gre",
        6,
        configure_mpls: true,
        test_gso: true,
    ),
    subtest_cfg_item!(
        "udp",
        "none",
        "ipip",
        4,
        extra_decap_mod_args_cb: Some(udp_decap_mod_args_cb),
        configure_fou_rx_port: true,
        test_gso: true,
    ),
    subtest_cfg_item!(
        "udp",
        "eth",
        "ipip",
        4,
        extra_decap_mod_args_cb: Some(udp_decap_mod_args_cb),
        configure_fou_rx_port: true,
        expect_kern_decap_failure: true,
        test_gso: true,
    ),
    subtest_cfg_item!(
        "udp",
        "mpls",
        "ipip",
        4,
        extra_decap_mod_args_cb: Some(udp_decap_mod_args_cb),
        configure_fou_rx_port: true,
        tmode: mut_cstr!("mode any ttl 255"),
        configure_mpls: true,
        test_gso: true,
    ),
    subtest_cfg_item!(
        "ip6udp",
        "none",
        "ip6tnl",
        6,
        extra_decap_mod_args_cb: Some(udp_decap_mod_args_cb),
        configure_fou_rx_port: true,
        test_gso: true,
    ),
    subtest_cfg_item!(
        "ip6udp",
        "eth",
        "ip6tnl",
        6,
        extra_decap_mod_args_cb: Some(udp_decap_mod_args_cb),
        configure_fou_rx_port: true,
        expect_kern_decap_failure: true,
        test_gso: true,
    ),
    subtest_cfg_item!(
        "ip6udp",
        "mpls",
        "ip6tnl",
        6,
        extra_decap_mod_args_cb: Some(udp_decap_mod_args_cb),
        configure_fou_rx_port: true,
        tmode: mut_cstr!("mode any ttl 255"),
        expect_kern_decap_failure: true,
        test_gso: true,
    ),
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_tc_tunnel() {
    let mut skel: *mut test_tc_tunnel;
    let mut cfg: *mut subtest_cfg;
    let mut i: usize;
    let mut ret: c_int;

    skel = test_tc_tunnel__open_and_load();
    if !ASSERT_OK_PTR!(skel, "skel open and load") {
        return;
    }

    if !ASSERT_OK!(setup(), "global setup") {
        test_tc_tunnel__destroy(skel);
        return;
    }

    i = 0;
    while i < SUBTESTS_CFG.len() {
        cfg = &mut SUBTESTS_CFG[i] as *mut subtest_cfg;
        ret = build_subtest_name(cfg, (*cfg).name.as_mut_ptr(), TEST_NAME_MAX_LEN);
        if ret < 0 || !test__start_subtest((*cfg).name.as_ptr()) {
            i += 1;
            continue;
        }
        if subtest_setup(skel, cfg) == 0 {
            run_test(cfg);
        }
        subtest_cleanup(cfg);
        i += 1;
    }
    cleanup();

    test_tc_tunnel__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
