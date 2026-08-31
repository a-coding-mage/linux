// SPDX-License-Identifier: GPL-2.0
// Translated from C source. Original include dependencies:
// <test_progs.h>, <network_helpers.h>, <bpf/btf.h>, <linux/if_link.h>,
// <netinet/udp.h>, <net/if.h>, <unistd.h>, "xdp_flowtable.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

const TX_NETNS_NAME: &[u8] = b"ns0\0";
const RX_NETNS_NAME: &[u8] = b"ns1\0";

const TX_NAME: &[u8] = b"v0\0";
const FORWARD_NAME: &[u8] = b"v1\0";
const RX_NAME: &[u8] = b"d0\0";

const TX_MAC: &[u8] = b"00:00:00:00:00:01\0";
const FORWARD_MAC: &[u8] = b"00:00:00:00:00:02\0";
const RX_MAC: &[u8] = b"00:00:00:00:00:03\0";
const DST_MAC: &[u8] = b"00:00:00:00:00:04\0";

const TX_ADDR: &[u8] = b"10.0.0.1\0";
const FORWARD_ADDR: &[u8] = b"10.0.0.2\0";
const RX_ADDR: &[u8] = b"20.0.0.1\0";
const DST_ADDR: &[u8] = b"20.0.0.2\0";

const PREFIX_LEN: &[u8] = b"8\0";
const N_PACKETS: c_int = 10;
const UDP_PORT: c_int = 12345;
const UDP_PORT_STR: &[u8] = b"12345\0";

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const MSG_NOSIGNAL: c_int = 0x4000;
const MSG_CONFIRM: c_int = 0x800;
const EINVAL: c_int = 22;

type __u32 = u32;
type socklen_t = c_uint;

#[repr(C)]
pub struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: u64,
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xdp_flowtable_progs {
    pub xdp_flowtable_do_lookup: *mut bpf_program,
}

#[repr(C)]
pub struct xdp_flowtable_maps {
    pub stats: *mut bpf_map,
}

#[repr(C)]
pub struct xdp_flowtable {
    pub progs: xdp_flowtable_progs,
    pub maps: xdp_flowtable_maps,
}

unsafe extern "C" {
    static mut stdout: *mut c_void;

    fn make_sockaddr(
        family: c_int,
        addr: *const c_char,
        port: c_int,
        sockaddr: *mut sockaddr_storage,
        socklen: *mut socklen_t,
    ) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn sendto(
        socket: c_int,
        message: *const c_void,
        length: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        dest_len: socklen_t,
    ) -> isize;
    fn close(fd: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn test__skip();
    fn system(command: *const c_char) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(tok: *mut nstoken);
    fn if_nametoindex(ifname: *const c_char) -> c_uint;

    fn xdp_flowtable__open_and_load() -> *mut xdp_flowtable;
    fn xdp_flowtable__destroy(obj: *mut xdp_flowtable);
    fn bpf_program__attach_xdp(prog: *mut bpf_program, ifindex: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(a: __u32, b: c_int, name: *const c_char) -> bool;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! SYS_NOFAIL {
    ($cmd:literal) => {{
        unsafe { system(cstr!($cmd)) }
    }};
}

macro_rules! SYS {
    ($label:lifetime, $cmd:expr) => {{
        if unsafe { system($cmd.as_ptr() as *const c_char) } != 0 {
            break $label;
        }
    }};
}

fn send_udp_traffic() -> c_int {
    unsafe {
        let mut addr: sockaddr_storage = core::mem::zeroed();
        let mut i: c_int;
        let sock: c_int;

        if make_sockaddr(
            AF_INET,
            DST_ADDR.as_ptr() as *const c_char,
            UDP_PORT,
            &mut addr,
            core::ptr::null_mut(),
        ) != 0
        {
            return -EINVAL;
        }

        sock = socket(AF_INET, SOCK_DGRAM, 0);
        if sock < 0 {
            return sock;
        }

        i = 0;
        while i < N_PACKETS {
            let buf: [u8; 3] = [0xaa, 0xbb, 0xcc];
            let n: c_int;

            n = sendto(
                sock,
                buf.as_ptr() as *const c_void,
                core::mem::size_of_val(&buf),
                MSG_NOSIGNAL | MSG_CONFIRM,
                &addr as *const sockaddr_storage as *const sockaddr,
                core::mem::size_of_val(&addr) as socklen_t,
            ) as c_int;
            if n != core::mem::size_of_val(&buf) as c_int {
                close(sock);
                return -EINVAL;
            }

            usleep(50000); /* 50ms */
            i += 1;
        }
        close(sock);

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_flowtable() {
    unsafe {
        let mut skel: *mut xdp_flowtable = core::ptr::null_mut();
        let mut tok: *mut nstoken = core::ptr::null_mut();
        let iifindex: c_int;
        let stats_fd: c_int;
        let mut value: __u32 = 0;
        let key: __u32 = 0;
        let mut link: *mut bpf_link = core::ptr::null_mut();

        'out: loop {
            if SYS_NOFAIL!("nft -v") != 0 {
                fprintf(stdout, cstr!("Missing required nft tool\n"));
                test__skip();
                return;
            }

            SYS!('out, b"ip netns add ns0\0");
            SYS!('out, b"ip netns add ns1\0");

            tok = open_netns(RX_NETNS_NAME.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(tok as *const c_void, cstr!("setns")) {
                break 'out;
            }

            SYS!('out, b"sysctl -qw net.ipv4.conf.all.forwarding=1\0");

            SYS!('out, b"ip link add v0 type veth peer v1\0");
            SYS!('out, b"ip link set v0 netns ns0\0");
            SYS!('out, b"ip link set dev v1 address 00:00:00:00:00:02\0");
            SYS!('out, b"ip addr add 10.0.0.2/8 dev v1\0");
            SYS!('out, b"ip link set dev v1 up\0");

            SYS!('out, b"ip link add d0 type dummy\0");
            SYS!('out, b"ip link set dev d0 address 00:00:00:00:00:03\0");
            SYS!('out, b"ip addr add 20.0.0.1/8 dev d0\0");
            SYS!('out, b"ip link set dev d0 up\0");

            /* configure the flowtable */
            SYS!('out, b"nft add table ip filter\0");
            SYS!(
                'out,
                b"nft add flowtable ip filter f { hook ingress priority 0\\; devices = { v1, d0 }\\; }\0"
            );
            SYS!(
                'out,
                b"nft add chain ip filter forward { type filter hook forward priority 0\\; }\0"
            );
            SYS!(
                'out,
                b"nft add rule ip filter forward ip protocol udp th dport 12345 flow add @f\0"
            );

            /* Avoid ARP calls */
            SYS!(
                'out,
                b"ip -4 neigh add 20.0.0.2 lladdr 00:00:00:00:00:04 dev d0\0"
            );

            close_netns(tok);
            tok = open_netns(TX_NETNS_NAME.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(tok as *const c_void, cstr!("setns")) {
                break 'out;
            }

            SYS!('out, b"ip addr add 10.0.0.1/8 dev v0\0");
            SYS!('out, b"ip link set dev v0 address 00:00:00:00:00:01\0");
            SYS!('out, b"ip link set dev v0 up\0");
            SYS!('out, b"ip route add default via 10.0.0.2\0");

            close_netns(tok);
            tok = open_netns(RX_NETNS_NAME.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(tok as *const c_void, cstr!("setns")) {
                break 'out;
            }

            iifindex = if_nametoindex(FORWARD_NAME.as_ptr() as *const c_char) as c_int;
            if !ASSERT_NEQ(iifindex, 0, cstr!("iifindex")) {
                break 'out;
            }

            skel = xdp_flowtable__open_and_load();
            if !ASSERT_OK_PTR(skel as *const c_void, cstr!("skel")) {
                break 'out;
            }

            link = bpf_program__attach_xdp((*skel).progs.xdp_flowtable_do_lookup, iifindex);
            if !ASSERT_OK_PTR(link as *const c_void, cstr!("prog_attach")) {
                break 'out;
            }

            close_netns(tok);
            tok = open_netns(TX_NETNS_NAME.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(tok as *const c_void, cstr!("setns")) {
                break 'out;
            }

            if !ASSERT_OK(send_udp_traffic(), cstr!("send udp")) {
                break 'out;
            }

            close_netns(tok);
            tok = open_netns(RX_NETNS_NAME.as_ptr() as *const c_char);
            if !ASSERT_OK_PTR(tok as *const c_void, cstr!("setns")) {
                break 'out;
            }

            stats_fd = bpf_map__fd((*skel).maps.stats);
            if !ASSERT_OK(
                bpf_map_lookup_elem(
                    stats_fd,
                    &key as *const __u32 as *const c_void,
                    &mut value as *mut __u32 as *mut c_void,
                ),
                cstr!("bpf_map_update_elem stats"),
            ) {
                break 'out;
            }

            ASSERT_GE(value, N_PACKETS - 2, cstr!("bpf_xdp_flow_lookup failed"));
            break 'out;
        }

        bpf_link__destroy(link);
        xdp_flowtable__destroy(skel);
        if !tok.is_null() {
            close_netns(tok);
        }
        SYS_NOFAIL!("ip netns del ns0");
        SYS_NOFAIL!("ip netns del ns1");
    }
}
