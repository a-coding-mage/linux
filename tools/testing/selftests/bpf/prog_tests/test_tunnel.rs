// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

/*
 * End-to-end eBPF tunnel test suite
 *   The file tests BPF network tunnel implementation.
 *
 * Topology:
 * ---------
 *     root namespace   |     at_ns0 namespace
 *                       |
 *       -----------     |     -----------
 *       | tnl dev |     |     | tnl dev |  (overlay network)
 *       -----------     |     -----------
 *       metadata-mode   |     metadata-mode
 *        with bpf       |       with bpf
 *                       |
 *       ----------      |     ----------
 *       |  veth1  | --------- |  veth0  |  (underlay network)
 *       ----------    peer    ----------
 *
 *
 *  Device Configuration
 *  --------------------
 *  root namespace with metadata-mode tunnel + BPF
 *  Device names and addresses:
 *	veth1 IP 1: 172.16.1.200, IPv6: 00::22 (underlay)
 *		IP 2: 172.16.1.20, IPv6: 00::bb (underlay)
 *	tunnel dev <type>11, ex: gre11, IPv4: 10.1.1.200, IPv6: 1::22 (overlay)
 *
 *  Namespace at_ns0 with native tunnel
 *  Device names and addresses:
 *	veth0 IPv4: 172.16.1.100, IPv6: 00::11 (underlay)
 *	tunnel dev <type>00, ex: gre00, IPv4: 10.1.1.100, IPv6: 1::11 (overlay)
 *
 *
 * End-to-end ping packet flow
 *  ---------------------------
 *  Most of the tests start by namespace creation, device configuration,
 *  then ping the underlay and overlay network.  When doing 'ping 10.1.1.100'
 *  from root namespace, the following operations happen:
 *  1) Route lookup shows 10.1.1.100/24 belongs to tnl dev, fwd to tnl dev.
 *  2) Tnl device's egress BPF program is triggered and set the tunnel metadata,
 *     with local_ip=172.16.1.200, remote_ip=172.16.1.100. BPF program choose
 *     the primary or secondary ip of veth1 as the local ip of tunnel. The
 *     choice is made based on the value of bpf map local_ip_map.
 *  3) Outer tunnel header is prepended and route the packet to veth1's egress.
 *  4) veth0's ingress queue receive the tunneled packet at namespace at_ns0.
 *  5) Tunnel protocol handler, ex: vxlan_rcv, decap the packet.
 *  6) Forward the packet to the overlay tnl dev.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const IP4_ADDR_VETH0: *const c_char = b"172.16.1.100\0".as_ptr() as *const c_char;
const IP4_ADDR1_VETH1: *const c_char = b"172.16.1.200\0".as_ptr() as *const c_char;
const IP4_ADDR2_VETH1: *const c_char = b"172.16.1.20\0".as_ptr() as *const c_char;
const IP4_ADDR_TUNL_DEV0: *const c_char = b"10.1.1.100\0".as_ptr() as *const c_char;
const IP4_ADDR_TUNL_DEV1: *const c_char = b"10.1.1.200\0".as_ptr() as *const c_char;
const IP6_ADDR_TUNL_DEV0: *const c_char = b"fc80::100\0".as_ptr() as *const c_char;
const IP6_ADDR_TUNL_DEV1: *const c_char = b"fc80::200\0".as_ptr() as *const c_char;

const IP6_ADDR_VETH0: *const c_char = b"::11\0".as_ptr() as *const c_char;
const IP6_ADDR1_VETH1: *const c_char = b"::22\0".as_ptr() as *const c_char;
const IP6_ADDR2_VETH1: *const c_char = b"::bb\0".as_ptr() as *const c_char;

const IP4_ADDR1_HEX_VETH1: c_uint = 0xac1001c8;
const IP4_ADDR2_HEX_VETH1: c_uint = 0xac100114;
const IP6_ADDR1_HEX_VETH1: c_uint = 0x22;
const IP6_ADDR2_HEX_VETH1: c_uint = 0xbb;

const MAC_TUNL_DEV0: *const c_char = b"52:54:00:d9:01:00\0".as_ptr() as *const c_char;
const MAC_TUNL_DEV1: *const c_char = b"52:54:00:d9:02:00\0".as_ptr() as *const c_char;
const MAC_VETH1: *const c_char = b"52:54:00:d9:03:00\0".as_ptr() as *const c_char;

const VXLAN_TUNL_DEV0: *const c_char = b"vxlan00\0".as_ptr() as *const c_char;
const VXLAN_TUNL_DEV1: *const c_char = b"vxlan11\0".as_ptr() as *const c_char;
const IP6VXLAN_TUNL_DEV0: *const c_char = b"ip6vxlan00\0".as_ptr() as *const c_char;
const IP6VXLAN_TUNL_DEV1: *const c_char = b"ip6vxlan11\0".as_ptr() as *const c_char;

const IPIP_TUNL_DEV0: *const c_char = b"ipip00\0".as_ptr() as *const c_char;
const IPIP_TUNL_DEV1: *const c_char = b"ipip11\0".as_ptr() as *const c_char;

const XFRM_AUTH: *const c_char = b"0x1111111111111111111111111111111111111111\0".as_ptr() as *const c_char;
const XFRM_ENC: *const c_char = b"0x22222222222222222222222222222222\0".as_ptr() as *const c_char;
const XFRM_SPI_IN_TO_OUT: c_int = 0x1;
const XFRM_SPI_OUT_TO_IN: c_int = 0x2;

const GRE_TUNL_DEV0: *const c_char = b"gre00\0".as_ptr() as *const c_char;
const GRE_TUNL_DEV1: *const c_char = b"gre11\0".as_ptr() as *const c_char;

const IP6GRE_TUNL_DEV0: *const c_char = b"ip6gre00\0".as_ptr() as *const c_char;
const IP6GRE_TUNL_DEV1: *const c_char = b"ip6gre11\0".as_ptr() as *const c_char;

const ERSPAN_TUNL_DEV0: *const c_char = b"erspan00\0".as_ptr() as *const c_char;
const ERSPAN_TUNL_DEV1: *const c_char = b"erspan11\0".as_ptr() as *const c_char;

const IP6ERSPAN_TUNL_DEV0: *const c_char = b"ip6erspan00\0".as_ptr() as *const c_char;
const IP6ERSPAN_TUNL_DEV1: *const c_char = b"ip6erspan11\0".as_ptr() as *const c_char;

const GENEVE_TUNL_DEV0: *const c_char = b"geneve00\0".as_ptr() as *const c_char;
const GENEVE_TUNL_DEV1: *const c_char = b"geneve11\0".as_ptr() as *const c_char;

const IP6GENEVE_TUNL_DEV0: *const c_char = b"ip6geneve00\0".as_ptr() as *const c_char;
const IP6GENEVE_TUNL_DEV1: *const c_char = b"ip6geneve11\0".as_ptr() as *const c_char;

const IP6TNL_TUNL_DEV0: *const c_char = b"ip6tnl00\0".as_ptr() as *const c_char;
const IP6TNL_TUNL_DEV1: *const c_char = b"ip6tnl11\0".as_ptr() as *const c_char;

const PING_ARGS: *const c_char = b"-i 0.01 -c 3 -w 10 -q\0".as_ptr() as *const c_char;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const BPF_ANY: c_uint = 0;
const XDP_FLAGS_REPLACE: c_uint = 1 << 4;

#[repr(C)]
struct nstoken {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
struct test_tunnel_kern {
	progs: test_tunnel_kern_progs,
	maps: test_tunnel_kern_maps,
	bss: *mut test_tunnel_kern_bss,
}

#[repr(C)]
struct test_tunnel_kern_progs {
	vxlan_get_tunnel_src: *mut bpf_program,
	vxlan_set_tunnel_src: *mut bpf_program,
	veth_set_outer_dst: *mut bpf_program,
	vxlan_set_tunnel_dst: *mut bpf_program,
	ip6vxlan_get_tunnel_src: *mut bpf_program,
	ip6vxlan_set_tunnel_src: *mut bpf_program,
	ip6vxlan_set_tunnel_dst: *mut bpf_program,
	ipip_encap_get_tunnel: *mut bpf_program,
	ipip_fou_set_tunnel: *mut bpf_program,
	ipip_gue_set_tunnel: *mut bpf_program,
	ipip_get_tunnel: *mut bpf_program,
	ipip_set_tunnel: *mut bpf_program,
	xfrm_get_state: *mut bpf_program,
	xfrm_get_state_xdp: *mut bpf_program,
	gre_set_tunnel_no_key: *mut bpf_program,
	gre_get_tunnel: *mut bpf_program,
	gre_set_tunnel: *mut bpf_program,
	ip6gretap_set_tunnel: *mut bpf_program,
	ip6gretap_get_tunnel: *mut bpf_program,
	erspan_set_tunnel: *mut bpf_program,
	erspan_get_tunnel: *mut bpf_program,
	ip4ip6erspan_set_tunnel: *mut bpf_program,
	ip4ip6erspan_get_tunnel: *mut bpf_program,
	geneve_set_tunnel: *mut bpf_program,
	geneve_get_tunnel: *mut bpf_program,
	ip6geneve_set_tunnel: *mut bpf_program,
	ip6geneve_get_tunnel: *mut bpf_program,
	ipip6_set_tunnel: *mut bpf_program,
	ipip6_get_tunnel: *mut bpf_program,
	ip6ip6_set_tunnel: *mut bpf_program,
	ip6ip6_get_tunnel: *mut bpf_program,
}

#[repr(C)]
struct test_tunnel_kern_maps {
	local_ip_map: *mut bpf_map,
}

#[repr(C)]
struct test_tunnel_kern_bss {
	xfrm_reqid: c_int,
	xfrm_spi: c_int,
	xfrm_remote_ip: c_uint,
	xfrm_replay_window: c_int,
}

#[repr(C)]
struct bpf_xdp_attach_opts {
	sz: usize,
	old_prog_fd: c_int,
}

type pthread_t = usize;

unsafe extern "C" {
	fn sys(fmt: *const c_char, ...) -> c_int;
	fn sys_nofail(fmt: *const c_char, ...);
	fn assert_ok(err: c_int, name: *const c_char) -> bool;
	fn assert_ok_ptr(ptr: *mut c_void, name: *const c_char) -> bool;
	fn assert_ge(left: c_int, right: c_int, name: *const c_char) -> bool;
	fn assert_neq(left: c_uint, right: c_uint, name: *const c_char) -> bool;
	fn assert_eq(left: c_uint, right: c_uint, name: *const c_char) -> bool;
	fn test__start_subtest(name: *const c_char) -> bool;
	fn ping_command(family: c_int) -> *const c_char;
	fn open_netns(name: *const c_char) -> *mut nstoken;
	fn close_netns(token: *mut nstoken);
	fn test_tunnel_kern__open_and_load() -> *mut test_tunnel_kern;
	fn test_tunnel_kern__destroy(skel: *mut test_tunnel_kern);
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: c_uint) -> c_int;
	fn tc_prog_attach(dev: *const c_char, ingress_fd: c_int, egress_fd: c_int) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn if_nametoindex(ifname: *const c_char) -> c_uint;
	fn bpf_xdp_attach(ifindex: c_uint, prog_fd: c_int, flags: c_uint, opts: *const bpf_xdp_attach_opts) -> c_int;
	fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
}

macro_rules! c {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *const c_char
	};
}

macro_rules! SYS {
	($fmt:literal $(, $arg:expr)* $(,)?) => {
		if sys(c!($fmt) $(, $arg)*) != 0 {
			return -1;
		}
	};
}

macro_rules! SYS_NOFAIL {
	($fmt:literal $(, $arg:expr)* $(,)?) => {
		sys_nofail(c!($fmt) $(, $arg)*);
	};
}

unsafe fn config_device() -> c_int {
	SYS!("ip netns add at_ns0");
	SYS!("ip link add veth0 address 52:54:00:d9:03:00 type veth peer name veth1");
	SYS!("ip link set veth0 netns at_ns0");
	SYS!("ip addr add 172.16.1.200/24 dev veth1");
	SYS!("ip link set dev veth1 up mtu 1500");
	SYS!("ip netns exec at_ns0 ip addr add 172.16.1.100/24 dev veth0");
	SYS!("ip netns exec at_ns0 ip link set dev veth0 up mtu 1500");

	0
}

unsafe fn cleanup() {
	SYS_NOFAIL!("test -f /var/run/netns/at_ns0 && ip netns delete at_ns0");
	SYS_NOFAIL!("ip link del veth1");
	SYS_NOFAIL!("ip link del %s", VXLAN_TUNL_DEV1);
	SYS_NOFAIL!("ip link del %s", IP6VXLAN_TUNL_DEV1);
}

unsafe fn add_vxlan_tunnel() -> c_int {
	/* at_ns0 namespace */
	SYS!("ip netns exec at_ns0 ip link add dev %s type vxlan external gbp dstport 4789", VXLAN_TUNL_DEV0);
	SYS!("ip netns exec at_ns0 ip link set dev %s address %s up", VXLAN_TUNL_DEV0, MAC_TUNL_DEV0);
	SYS!("ip netns exec at_ns0 ip addr add dev %s %s/24", VXLAN_TUNL_DEV0, IP4_ADDR_TUNL_DEV0);
	SYS!("ip netns exec at_ns0 ip neigh add %s lladdr %s dev %s", IP4_ADDR_TUNL_DEV1, MAC_TUNL_DEV1, VXLAN_TUNL_DEV0);
	SYS!("ip netns exec at_ns0 ip neigh add %s lladdr %s dev veth0", IP4_ADDR2_VETH1, MAC_VETH1);

	/* root namespace */
	SYS!("ip link add dev %s type vxlan external gbp dstport 4789", VXLAN_TUNL_DEV1);
	SYS!("ip link set dev %s address %s up", VXLAN_TUNL_DEV1, MAC_TUNL_DEV1);
	SYS!("ip addr add dev %s %s/24", VXLAN_TUNL_DEV1, IP4_ADDR_TUNL_DEV1);
	SYS!("ip neigh add %s lladdr %s dev %s", IP4_ADDR_TUNL_DEV0, MAC_TUNL_DEV0, VXLAN_TUNL_DEV1);

	0
}

unsafe fn delete_vxlan_tunnel() {
	SYS_NOFAIL!("ip netns exec at_ns0 ip link delete dev %s", VXLAN_TUNL_DEV0);
	SYS_NOFAIL!("ip link delete dev %s", VXLAN_TUNL_DEV1);
}

unsafe fn add_ip6vxlan_tunnel() -> c_int {
	SYS!("ip netns exec at_ns0 ip -6 addr add %s/96 dev veth0", IP6_ADDR_VETH0);
	SYS!("ip netns exec at_ns0 ip link set dev veth0 up");
	SYS!("ip -6 addr add %s/96 dev veth1", IP6_ADDR1_VETH1);
	SYS!("ip -6 addr add %s/96 dev veth1", IP6_ADDR2_VETH1);
	SYS!("ip link set dev veth1 up");

	/* at_ns0 namespace */
	SYS!("ip netns exec at_ns0 ip link add dev %s type vxlan external dstport 4789", IP6VXLAN_TUNL_DEV0);
	SYS!("ip netns exec at_ns0 ip addr add dev %s %s/24", IP6VXLAN_TUNL_DEV0, IP4_ADDR_TUNL_DEV0);
	SYS!("ip netns exec at_ns0 ip link set dev %s address %s up", IP6VXLAN_TUNL_DEV0, MAC_TUNL_DEV0);

	/* root namespace */
	SYS!("ip link add dev %s type vxlan external dstport 4789", IP6VXLAN_TUNL_DEV1);
	SYS!("ip addr add dev %s %s/24", IP6VXLAN_TUNL_DEV1, IP4_ADDR_TUNL_DEV1);
	SYS!("ip link set dev %s address %s up", IP6VXLAN_TUNL_DEV1, MAC_TUNL_DEV1);

	0
}

unsafe fn delete_ip6vxlan_tunnel() {
	SYS_NOFAIL!("ip netns exec at_ns0 ip -6 addr delete %s/96 dev veth0", IP6_ADDR_VETH0);
	SYS_NOFAIL!("ip -6 addr delete %s/96 dev veth1", IP6_ADDR1_VETH1);
	SYS_NOFAIL!("ip -6 addr delete %s/96 dev veth1", IP6_ADDR2_VETH1);
	SYS_NOFAIL!("ip netns exec at_ns0 ip link delete dev %s", IP6VXLAN_TUNL_DEV0);
	SYS_NOFAIL!("ip link delete dev %s", IP6VXLAN_TUNL_DEV1);
}

#[repr(C)]
#[derive(Copy, Clone)]
enum ipip_encap {
	NONE = 0,
	FOU = 1,
	GUE = 2,
}

unsafe fn set_ipip_encap(ipproto: *const c_char, type_: *const c_char) -> c_int {
	SYS!("ip -n at_ns0 fou add port 5555 %s", ipproto);
	SYS!("ip -n at_ns0 link set dev %s type ipip encap %s", IPIP_TUNL_DEV0, type_);
	SYS!("ip -n at_ns0 link set dev %s type ipip encap-dport 5555", IPIP_TUNL_DEV0);

	0
}

unsafe fn set_ipv4_addr(dev0: *const c_char, dev1: *const c_char) -> c_int {
	if sys(c!("ip -n at_ns0 link set dev %s up"), dev0) != 0 {
		return 1;
	}
	if sys(c!("ip -n at_ns0 addr add dev %s %s/24"), dev0, IP4_ADDR_TUNL_DEV0) != 0 {
		return 1;
	}
	if sys(c!("ip link set dev %s up"), dev1) != 0 {
		return 1;
	}
	if sys(c!("ip addr add dev %s %s/24"), dev1, IP4_ADDR_TUNL_DEV1) != 0 {
		return 1;
	}

	0
}

unsafe fn add_ipip_tunnel(encap: ipip_encap) -> c_int {
	let err: c_int;
	let ipproto: *const c_char;
	let type_: *const c_char;

	match encap {
		ipip_encap::FOU => {
			ipproto = c!("ipproto 4");
			type_ = c!("fou");
		}
		ipip_encap::GUE => {
			ipproto = c!("gue");
			type_ = ipproto;
		}
		_ => {
			ipproto = ptr::null();
			type_ = ipproto;
		}
	}

	/* at_ns0 namespace */
	SYS!("ip -n at_ns0 link add dev %s type ipip local %s remote %s", IPIP_TUNL_DEV0, IP4_ADDR_VETH0, IP4_ADDR1_VETH1);

	if !type_.is_null() && !ipproto.is_null() {
		err = set_ipip_encap(ipproto, type_);
		if !assert_ok(err, c!("set_ipip_encap")) {
			return -1;
		}
	}

	SYS!("ip -n at_ns0 link set dev %s up", IPIP_TUNL_DEV0);
	SYS!("ip -n at_ns0 addr add dev %s %s/24", IPIP_TUNL_DEV0, IP4_ADDR_TUNL_DEV0);

	/* root namespace */
	if !type_.is_null() && !ipproto.is_null() {
		SYS!("ip fou add port 5555 %s", ipproto);
	}
	SYS!("ip link add dev %s type ipip external", IPIP_TUNL_DEV1);
	SYS!("ip link set dev %s up", IPIP_TUNL_DEV1);
	SYS!("ip addr add dev %s %s/24", IPIP_TUNL_DEV1, IP4_ADDR_TUNL_DEV1);

	0
}

unsafe fn delete_ipip_tunnel() {
	SYS_NOFAIL!("ip -n at_ns0 link delete dev %s", IPIP_TUNL_DEV0);
	SYS_NOFAIL!("ip -n at_ns0 fou del port 5555");
	SYS_NOFAIL!("ip link delete dev %s", IPIP_TUNL_DEV1);
	SYS_NOFAIL!("ip fou del port 5555");
}

unsafe fn add_xfrm_tunnel() -> c_int {
	/* at_ns0 namespace
	 * at_ns0 -> root
	 */
	SYS!("ip netns exec at_ns0 ip xfrm state add src %s dst %s proto esp spi %d reqid 1 mode tunnel replay-window 42 auth-trunc 'hmac(sha1)' %s 96 enc 'cbc(aes)' %s", IP4_ADDR_VETH0, IP4_ADDR1_VETH1, XFRM_SPI_IN_TO_OUT, XFRM_AUTH, XFRM_ENC);
	SYS!("ip netns exec at_ns0 ip xfrm policy add src %s/32 dst %s/32 dir out tmpl src %s dst %s proto esp reqid 1 mode tunnel", IP4_ADDR_TUNL_DEV0, IP4_ADDR_TUNL_DEV1, IP4_ADDR_VETH0, IP4_ADDR1_VETH1);

	/* root -> at_ns0 */
	SYS!("ip netns exec at_ns0 ip xfrm state add src %s dst %s proto esp spi %d reqid 2 mode tunnel auth-trunc 'hmac(sha1)' %s 96 enc 'cbc(aes)' %s", IP4_ADDR1_VETH1, IP4_ADDR_VETH0, XFRM_SPI_OUT_TO_IN, XFRM_AUTH, XFRM_ENC);
	SYS!("ip netns exec at_ns0 ip xfrm policy add src %s/32 dst %s/32 dir in tmpl src %s dst %s proto esp reqid 2 mode tunnel", IP4_ADDR_TUNL_DEV1, IP4_ADDR_TUNL_DEV0, IP4_ADDR1_VETH1, IP4_ADDR_VETH0);

	/* address & route */
	SYS!("ip netns exec at_ns0 ip addr add dev veth0 %s/32", IP4_ADDR_TUNL_DEV0);
	SYS!("ip netns exec at_ns0 ip route add %s dev veth0 via %s src %s", IP4_ADDR_TUNL_DEV1, IP4_ADDR1_VETH1, IP4_ADDR_TUNL_DEV0);

	/* root namespace
	 * at_ns0 -> root
	 */
	SYS!("ip xfrm state add src %s dst %s proto esp spi %d reqid 1 mode tunnel replay-window 42 auth-trunc 'hmac(sha1)' %s 96  enc 'cbc(aes)' %s", IP4_ADDR_VETH0, IP4_ADDR1_VETH1, XFRM_SPI_IN_TO_OUT, XFRM_AUTH, XFRM_ENC);
	SYS!("ip xfrm policy add src %s/32 dst %s/32 dir in tmpl src %s dst %s proto esp reqid 1 mode tunnel", IP4_ADDR_TUNL_DEV0, IP4_ADDR_TUNL_DEV1, IP4_ADDR_VETH0, IP4_ADDR1_VETH1);

	/* root -> at_ns0 */
	SYS!("ip xfrm state add src %s dst %s proto esp spi %d reqid 2 mode tunnel auth-trunc 'hmac(sha1)' %s 96  enc 'cbc(aes)' %s", IP4_ADDR1_VETH1, IP4_ADDR_VETH0, XFRM_SPI_OUT_TO_IN, XFRM_AUTH, XFRM_ENC);
	SYS!("ip xfrm policy add src %s/32 dst %s/32 dir out tmpl src %s dst %s proto esp reqid 2 mode tunnel", IP4_ADDR_TUNL_DEV1, IP4_ADDR_TUNL_DEV0, IP4_ADDR1_VETH1, IP4_ADDR_VETH0);

	/* address & route */
	SYS!("ip addr add dev veth1 %s/32", IP4_ADDR_TUNL_DEV1);
	SYS!("ip route add %s dev veth1 via %s src %s", IP4_ADDR_TUNL_DEV0, IP4_ADDR_VETH0, IP4_ADDR_TUNL_DEV1);

	0
}

unsafe fn delete_xfrm_tunnel() {
	SYS_NOFAIL!("ip xfrm policy delete dir out src %s/32 dst %s/32", IP4_ADDR_TUNL_DEV1, IP4_ADDR_TUNL_DEV0);
	SYS_NOFAIL!("ip xfrm policy delete dir in src %s/32 dst %s/32", IP4_ADDR_TUNL_DEV0, IP4_ADDR_TUNL_DEV1);
	SYS_NOFAIL!("ip xfrm state delete src %s dst %s proto esp spi %d", IP4_ADDR_VETH0, IP4_ADDR1_VETH1, XFRM_SPI_IN_TO_OUT);
	SYS_NOFAIL!("ip xfrm state delete src %s dst %s proto esp spi %d", IP4_ADDR1_VETH1, IP4_ADDR_VETH0, XFRM_SPI_OUT_TO_IN);
}

unsafe fn add_ipv4_tunnel(dev0: *const c_char, dev1: *const c_char, type_: *const c_char, opt: *const c_char) -> c_int {
	if type_.is_null() || opt.is_null() || dev0.is_null() || dev1.is_null() {
		return -1;
	}

	SYS!("ip -n at_ns0 link add dev %s type %s %s local %s remote %s", dev0, type_, opt, IP4_ADDR_VETH0, IP4_ADDR1_VETH1);
	SYS!("ip link add dev %s type %s external", dev1, type_);

	set_ipv4_addr(dev0, dev1)
}

unsafe fn delete_tunnel(dev0: *const c_char, dev1: *const c_char) {
	if dev0.is_null() || dev1.is_null() {
		return;
	}

	SYS_NOFAIL!("ip netns exec at_ns0 ip link delete dev %s", dev0);
	SYS_NOFAIL!("ip link delete dev %s", dev1);
}

unsafe fn set_ipv6_addr(dev0: *const c_char, dev1: *const c_char) -> c_int {
	/* disable IPv6 DAD because it might take too long and fail tests */
	if sys(c!("ip -n at_ns0 addr add %s/96 dev veth0 nodad"), IP6_ADDR_VETH0) != 0 { return 1; }
	if sys(c!("ip -n at_ns0 link set dev veth0 up")) != 0 { return 1; }
	if sys(c!("ip addr add %s/96 dev veth1 nodad"), IP6_ADDR1_VETH1) != 0 { return 1; }
	if sys(c!("ip link set dev veth1 up")) != 0 { return 1; }

	if sys(c!("ip -n at_ns0 addr add dev %s %s/24"), dev0, IP4_ADDR_TUNL_DEV0) != 0 { return 1; }
	if sys(c!("ip -n at_ns0 addr add dev %s %s/96 nodad"), dev0, IP6_ADDR_TUNL_DEV0) != 0 { return 1; }
	if sys(c!("ip -n at_ns0 link set dev %s up"), dev0) != 0 { return 1; }

	if sys(c!("ip addr add dev %s %s/24"), dev1, IP4_ADDR_TUNL_DEV1) != 0 { return 1; }
	if sys(c!("ip addr add dev %s %s/96 nodad"), dev1, IP6_ADDR_TUNL_DEV1) != 0 { return 1; }
	if sys(c!("ip link set dev %s up"), dev1) != 0 { return 1; }
	0
}

unsafe fn add_ipv6_tunnel(dev0: *const c_char, dev1: *const c_char, type_: *const c_char, opt: *const c_char) -> c_int {
	if type_.is_null() || opt.is_null() || dev0.is_null() || dev1.is_null() {
		return -1;
	}

	SYS!("ip -n at_ns0 link add dev %s type %s %s local %s remote %s", dev0, type_, opt, IP6_ADDR_VETH0, IP6_ADDR1_VETH1);
	SYS!("ip link add dev %s type %s external", dev1, type_);

	set_ipv6_addr(dev0, dev1)
}

unsafe fn add_geneve_tunnel(dev0: *const c_char, dev1: *const c_char, type_: *const c_char, opt: *const c_char) -> c_int {
	if type_.is_null() || opt.is_null() || dev0.is_null() || dev1.is_null() {
		return -1;
	}

	SYS!("ip -n at_ns0 link add dev %s type %s id 2 %s remote %s", dev0, type_, opt, IP4_ADDR1_VETH1);
	SYS!("ip link add dev %s type %s %s external", dev1, type_, opt);

	set_ipv4_addr(dev0, dev1)
}

unsafe fn add_ip6geneve_tunnel(dev0: *const c_char, dev1: *const c_char, type_: *const c_char, opt: *const c_char) -> c_int {
	if type_.is_null() || opt.is_null() || dev0.is_null() || dev1.is_null() {
		return -1;
	}

	SYS!("ip -n at_ns0 link add dev %s type %s id 22 %s remote %s", dev0, type_, opt, IP6_ADDR1_VETH1);
	SYS!("ip link add dev %s type %s %s external", dev1, type_, opt);

	set_ipv6_addr(dev0, dev1)
}

unsafe fn test_ping(family: c_int, addr: *const c_char) -> c_int {
	SYS!("%s %s %s > /dev/null", ping_command(family), PING_ARGS, addr);
	0
}

unsafe fn ping_dev0() {
	/* ping from root namespace test */
	test_ping(AF_INET, IP4_ADDR_TUNL_DEV0);
}

unsafe fn ping_dev1() {
	let nstoken: *mut nstoken;

	/* ping from at_ns0 namespace test */
	nstoken = open_netns(c!("at_ns0"));
	if !assert_ok_ptr(nstoken as *mut c_void, c!("setns")) {
		return;
	}

	test_ping(AF_INET, IP4_ADDR_TUNL_DEV1);
	close_netns(nstoken);
}

unsafe fn ping6_veth0() {
	test_ping(AF_INET6, IP6_ADDR_VETH0);
}

unsafe fn ping6_dev0() {
	test_ping(AF_INET6, IP6_ADDR_TUNL_DEV0);
}

unsafe fn ping6_dev1() {
	let nstoken: *mut nstoken;

	/* ping from at_ns0 namespace test */
	nstoken = open_netns(c!("at_ns0"));
	if !assert_ok_ptr(nstoken as *mut c_void, c!("setns")) {
		return;
	}

	test_ping(AF_INET, IP6_ADDR_TUNL_DEV1);
	close_netns(nstoken);
}

unsafe fn test_vxlan_tunnel() {
	let mut skel: *mut test_tunnel_kern = ptr::null_mut();
	let nstoken: *mut nstoken;
	let mut local_ip_map_fd: c_int = -1;
	let set_src_prog_fd: c_int;
	let get_src_prog_fd: c_int;
	let mut set_dst_prog_fd: c_int;
	let key: c_int = 0;
	let mut local_ip: c_uint;
	let mut err: c_int;

	err = add_vxlan_tunnel();
	if !assert_ok(err, c!("add vxlan tunnel")) {
		goto_done_vxlan(skel, local_ip_map_fd);
		return;
	}

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) {
		goto_done_vxlan(skel, local_ip_map_fd);
		return;
	}
	get_src_prog_fd = bpf_program__fd((*skel).progs.vxlan_get_tunnel_src);
	set_src_prog_fd = bpf_program__fd((*skel).progs.vxlan_set_tunnel_src);
	if tc_prog_attach(VXLAN_TUNL_DEV1, get_src_prog_fd, set_src_prog_fd) != 0 {
		goto_done_vxlan(skel, local_ip_map_fd);
		return;
	}

	set_dst_prog_fd = bpf_program__fd((*skel).progs.veth_set_outer_dst);
	if tc_prog_attach(c!("veth1"), set_dst_prog_fd, -1) != 0 {
		goto_done_vxlan(skel, local_ip_map_fd);
		return;
	}

	nstoken = open_netns(c!("at_ns0"));
	if !assert_ok_ptr(nstoken as *mut c_void, c!("setns src")) {
		goto_done_vxlan(skel, local_ip_map_fd);
		return;
	}
	set_dst_prog_fd = bpf_program__fd((*skel).progs.vxlan_set_tunnel_dst);
	if tc_prog_attach(VXLAN_TUNL_DEV0, -1, set_dst_prog_fd) != 0 {
		goto_done_vxlan(skel, local_ip_map_fd);
		return;
	}
	close_netns(nstoken);

	local_ip_map_fd = bpf_map__fd((*skel).maps.local_ip_map);
	if !assert_ge(local_ip_map_fd, 0, c!("bpf_map__fd")) {
		goto_done_vxlan(skel, local_ip_map_fd);
		return;
	}
	local_ip = IP4_ADDR2_HEX_VETH1;
	err = bpf_map_update_elem(local_ip_map_fd, &key as *const _ as *const c_void, &local_ip as *const _ as *const c_void, BPF_ANY);
	if !assert_ok(err, c!("update bpf local_ip_map")) {
		goto_done_vxlan(skel, local_ip_map_fd);
		return;
	}

	ping_dev0();
	goto_done_vxlan(skel, local_ip_map_fd);
}

unsafe fn goto_done_vxlan(skel: *mut test_tunnel_kern, local_ip_map_fd: c_int) {
	delete_vxlan_tunnel();
	if local_ip_map_fd >= 0 {
		close(local_ip_map_fd);
	}
	if !skel.is_null() {
		test_tunnel_kern__destroy(skel);
	}
}

unsafe fn test_ip6vxlan_tunnel() {
	let mut skel: *mut test_tunnel_kern = ptr::null_mut();
	let nstoken: *mut nstoken;
	let mut local_ip_map_fd: c_int = -1;
	let set_src_prog_fd: c_int;
	let get_src_prog_fd: c_int;
	let set_dst_prog_fd: c_int;
	let key: c_int = 0;
	let mut local_ip: c_uint;
	let mut err: c_int;

	err = add_ip6vxlan_tunnel();
	if !assert_ok(err, c!("add_ip6vxlan_tunnel")) { goto_done_ip6vxlan(skel, local_ip_map_fd); return; }

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { goto_done_ip6vxlan(skel, local_ip_map_fd); return; }
	get_src_prog_fd = bpf_program__fd((*skel).progs.ip6vxlan_get_tunnel_src);
	set_src_prog_fd = bpf_program__fd((*skel).progs.ip6vxlan_set_tunnel_src);
	if tc_prog_attach(IP6VXLAN_TUNL_DEV1, get_src_prog_fd, set_src_prog_fd) != 0 { goto_done_ip6vxlan(skel, local_ip_map_fd); return; }

	nstoken = open_netns(c!("at_ns0"));
	if !assert_ok_ptr(nstoken as *mut c_void, c!("setns src")) { goto_done_ip6vxlan(skel, local_ip_map_fd); return; }
	set_dst_prog_fd = bpf_program__fd((*skel).progs.ip6vxlan_set_tunnel_dst);
	if tc_prog_attach(IP6VXLAN_TUNL_DEV0, -1, set_dst_prog_fd) != 0 { goto_done_ip6vxlan(skel, local_ip_map_fd); return; }
	close_netns(nstoken);

	local_ip_map_fd = bpf_map__fd((*skel).maps.local_ip_map);
	if !assert_ge(local_ip_map_fd, 0, c!("get local_ip_map fd")) { goto_done_ip6vxlan(skel, local_ip_map_fd); return; }
	local_ip = IP6_ADDR2_HEX_VETH1;
	err = bpf_map_update_elem(local_ip_map_fd, &key as *const _ as *const c_void, &local_ip as *const _ as *const c_void, BPF_ANY);
	if !assert_ok(err, c!("update bpf local_ip_map")) { goto_done_ip6vxlan(skel, local_ip_map_fd); return; }

	ping_dev0();
	goto_done_ip6vxlan(skel, local_ip_map_fd);
}

unsafe fn goto_done_ip6vxlan(skel: *mut test_tunnel_kern, local_ip_map_fd: c_int) {
	delete_ip6vxlan_tunnel();
	if local_ip_map_fd >= 0 {
		close(local_ip_map_fd);
	}
	if !skel.is_null() {
		test_tunnel_kern__destroy(skel);
	}
}

unsafe fn test_ipip_tunnel(encap: ipip_encap) {
	let mut skel: *mut test_tunnel_kern = ptr::null_mut();
	let set_src_prog_fd: c_int;
	let get_src_prog_fd: c_int;
	let err: c_int;

	err = add_ipip_tunnel(encap);
	if !assert_ok(err, c!("add_ipip_tunnel")) { goto_done_ipip(skel); return; }

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { goto_done_ipip(skel); return; }

	match encap {
		ipip_encap::FOU => {
			get_src_prog_fd = bpf_program__fd((*skel).progs.ipip_encap_get_tunnel);
			set_src_prog_fd = bpf_program__fd((*skel).progs.ipip_fou_set_tunnel);
		}
		ipip_encap::GUE => {
			get_src_prog_fd = bpf_program__fd((*skel).progs.ipip_encap_get_tunnel);
			set_src_prog_fd = bpf_program__fd((*skel).progs.ipip_gue_set_tunnel);
		}
		_ => {
			get_src_prog_fd = bpf_program__fd((*skel).progs.ipip_get_tunnel);
			set_src_prog_fd = bpf_program__fd((*skel).progs.ipip_set_tunnel);
		}
	}

	if tc_prog_attach(IPIP_TUNL_DEV1, get_src_prog_fd, set_src_prog_fd) != 0 { goto_done_ipip(skel); return; }

	ping_dev0();
	ping_dev1();
	goto_done_ipip(skel);
}

unsafe fn goto_done_ipip(skel: *mut test_tunnel_kern) {
	delete_ipip_tunnel();
	if !skel.is_null() {
		test_tunnel_kern__destroy(skel);
	}
}

unsafe fn test_xfrm_tunnel() {
	let opts = bpf_xdp_attach_opts { sz: core::mem::size_of::<bpf_xdp_attach_opts>(), old_prog_fd: 0 };
	let mut skel: *mut test_tunnel_kern = ptr::null_mut();
	let xdp_prog_fd: c_int;
	let tc_prog_fd: c_int;
	let ifindex: c_uint;
	let mut err: c_int;

	err = add_xfrm_tunnel();
	if !assert_ok(err, c!("add_xfrm_tunnel")) {
		return;
	}

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { goto_done_xfrm(skel); return; }

	tc_prog_fd = bpf_program__fd((*skel).progs.xfrm_get_state);
	if tc_prog_attach(c!("veth1"), tc_prog_fd, -1) != 0 { goto_done_xfrm(skel); return; }

	ifindex = if_nametoindex(c!("veth1"));
	if !assert_neq(ifindex, 0, c!("veth1 ifindex")) { goto_done_xfrm(skel); return; }
	xdp_prog_fd = bpf_program__fd((*skel).progs.xfrm_get_state_xdp);
	if !assert_ge(xdp_prog_fd, 0, c!("bpf_program__fd")) { goto_done_xfrm(skel); return; }
	err = bpf_xdp_attach(ifindex, xdp_prog_fd, XDP_FLAGS_REPLACE, &opts);
	if !assert_ok(err, c!("bpf_xdp_attach")) { goto_done_xfrm(skel); return; }

	ping_dev1();

	if !assert_eq((*(*skel).bss).xfrm_reqid as c_uint, 1, c!("req_id")) { goto_done_xfrm(skel); return; }
	if !assert_eq((*(*skel).bss).xfrm_spi as c_uint, XFRM_SPI_IN_TO_OUT as c_uint, c!("spi")) { goto_done_xfrm(skel); return; }
	if !assert_eq((*(*skel).bss).xfrm_remote_ip, 0xac100164, c!("remote_ip")) { goto_done_xfrm(skel); return; }
	if !assert_eq((*(*skel).bss).xfrm_replay_window as c_uint, 42, c!("replay_window")) { goto_done_xfrm(skel); return; }

	goto_done_xfrm(skel);
}

unsafe fn goto_done_xfrm(skel: *mut test_tunnel_kern) {
	delete_xfrm_tunnel();
	if !skel.is_null() {
		test_tunnel_kern__destroy(skel);
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
enum gre_test {
	GRE,
	GRE_NOKEY,
	GRETAP,
	GRETAP_NOKEY,
}

unsafe fn test_gre_tunnel(test: gre_test) {
	let skel: *mut test_tunnel_kern;
	let set_fd: c_int;
	let get_fd: c_int;
	let err: c_int;

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { return; }

	match test {
		gre_test::GRE => {
			err = add_ipv4_tunnel(GRE_TUNL_DEV0, GRE_TUNL_DEV1, c!("gre"), c!("seq"));
			set_fd = bpf_program__fd((*skel).progs.gre_set_tunnel_no_key);
			get_fd = bpf_program__fd((*skel).progs.gre_get_tunnel);
		}
		gre_test::GRE_NOKEY => {
			err = add_ipv4_tunnel(GRE_TUNL_DEV0, GRE_TUNL_DEV1, c!("gre"), c!("seq key 2"));
			set_fd = bpf_program__fd((*skel).progs.gre_set_tunnel);
			get_fd = bpf_program__fd((*skel).progs.gre_get_tunnel);
		}
		gre_test::GRETAP => {
			err = add_ipv4_tunnel(GRE_TUNL_DEV0, GRE_TUNL_DEV1, c!("gretap"), c!("seq"));
			set_fd = bpf_program__fd((*skel).progs.gre_set_tunnel_no_key);
			get_fd = bpf_program__fd((*skel).progs.gre_get_tunnel);
		}
		gre_test::GRETAP_NOKEY => {
			err = add_ipv4_tunnel(GRE_TUNL_DEV0, GRE_TUNL_DEV1, c!("gretap"), c!("seq key 2"));
			set_fd = bpf_program__fd((*skel).progs.gre_set_tunnel);
			get_fd = bpf_program__fd((*skel).progs.gre_get_tunnel);
		}
	}
	if !assert_ok(err, c!("add tunnel")) { goto_done_gre(skel); return; }

	if tc_prog_attach(GRE_TUNL_DEV1, get_fd, set_fd) != 0 { goto_done_gre(skel); return; }

	ping_dev0();
	ping_dev1();
	goto_done_gre(skel);
}

unsafe fn goto_done_gre(skel: *mut test_tunnel_kern) {
	delete_tunnel(GRE_TUNL_DEV0, GRE_TUNL_DEV1);
	test_tunnel_kern__destroy(skel);
}

#[repr(C)]
#[derive(Copy, Clone)]
enum ip6gre_test {
	IP6GRE,
	IP6GRETAP,
}

unsafe fn test_ip6gre_tunnel(test: ip6gre_test) {
	let skel: *mut test_tunnel_kern;
	let set_fd: c_int;
	let get_fd: c_int;
	let err: c_int;

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { return; }

	match test {
		ip6gre_test::IP6GRE => {
			err = add_ipv6_tunnel(IP6GRE_TUNL_DEV0, IP6GRE_TUNL_DEV1, c!("ip6gre"), c!("flowlabel 0xbcdef key 2"));
		}
		ip6gre_test::IP6GRETAP => {
			err = add_ipv6_tunnel(IP6GRE_TUNL_DEV0, IP6GRE_TUNL_DEV1, c!("ip6gretap"), c!("flowlabel 0xbcdef key 2"));
		}
	}
	if !assert_ok(err, c!("add tunnel")) { goto_done_ip6gre(skel); return; }

	set_fd = bpf_program__fd((*skel).progs.ip6gretap_set_tunnel);
	get_fd = bpf_program__fd((*skel).progs.ip6gretap_get_tunnel);
	if tc_prog_attach(IP6GRE_TUNL_DEV1, get_fd, set_fd) != 0 { goto_done_ip6gre(skel); return; }

	ping6_veth0();
	ping6_dev1();
	ping_dev0();
	ping_dev1();
	goto_done_ip6gre(skel);
}

unsafe fn goto_done_ip6gre(skel: *mut test_tunnel_kern) {
	delete_tunnel(IP6GRE_TUNL_DEV0, IP6GRE_TUNL_DEV1);
	test_tunnel_kern__destroy(skel);
}

#[repr(C)]
#[derive(Copy, Clone)]
enum erspan_test {
	V1,
	V2,
}

unsafe fn test_erspan_tunnel(test: erspan_test) {
	let skel: *mut test_tunnel_kern;
	let set_fd: c_int;
	let get_fd: c_int;
	let err: c_int;

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { return; }

	match test {
		erspan_test::V1 => {
			err = add_ipv4_tunnel(ERSPAN_TUNL_DEV0, ERSPAN_TUNL_DEV1, c!("erspan"), c!("seq key 2 erspan_ver 1 erspan 123"));
		}
		erspan_test::V2 => {
			err = add_ipv4_tunnel(ERSPAN_TUNL_DEV0, ERSPAN_TUNL_DEV1, c!("erspan"), c!("seq key 2 erspan_ver 2 erspan_dir egress erspan_hwid 3"));
		}
	}
	if !assert_ok(err, c!("add tunnel")) { goto_done_erspan(skel); return; }

	set_fd = bpf_program__fd((*skel).progs.erspan_set_tunnel);
	get_fd = bpf_program__fd((*skel).progs.erspan_get_tunnel);
	if tc_prog_attach(ERSPAN_TUNL_DEV1, get_fd, set_fd) != 0 { goto_done_erspan(skel); return; }

	ping_dev0();
	ping_dev1();
	goto_done_erspan(skel);
}

unsafe fn goto_done_erspan(skel: *mut test_tunnel_kern) {
	delete_tunnel(ERSPAN_TUNL_DEV0, ERSPAN_TUNL_DEV1);
	test_tunnel_kern__destroy(skel);
}

unsafe fn test_ip6erspan_tunnel(test: erspan_test) {
	let skel: *mut test_tunnel_kern;
	let set_fd: c_int;
	let get_fd: c_int;
	let err: c_int;

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { return; }

	match test {
		erspan_test::V1 => {
			err = add_ipv6_tunnel(IP6ERSPAN_TUNL_DEV0, IP6ERSPAN_TUNL_DEV1, c!("ip6erspan"), c!("seq key 2 erspan_ver 1 erspan 123"));
		}
		erspan_test::V2 => {
			err = add_ipv6_tunnel(IP6ERSPAN_TUNL_DEV0, IP6ERSPAN_TUNL_DEV1, c!("ip6erspan"), c!("seq key 2 erspan_ver 2 erspan_dir egress erspan_hwid 7"));
		}
	}
	if !assert_ok(err, c!("add tunnel")) { goto_done_ip6erspan(skel); return; }

	set_fd = bpf_program__fd((*skel).progs.ip4ip6erspan_set_tunnel);
	get_fd = bpf_program__fd((*skel).progs.ip4ip6erspan_get_tunnel);
	if tc_prog_attach(IP6ERSPAN_TUNL_DEV1, get_fd, set_fd) != 0 { goto_done_ip6erspan(skel); return; }

	ping6_veth0();
	ping_dev1();
	goto_done_ip6erspan(skel);
}

unsafe fn goto_done_ip6erspan(skel: *mut test_tunnel_kern) {
	delete_tunnel(IP6ERSPAN_TUNL_DEV0, IP6ERSPAN_TUNL_DEV1);
	test_tunnel_kern__destroy(skel);
}

unsafe fn test_geneve_tunnel() {
	let skel: *mut test_tunnel_kern;
	let set_fd: c_int;
	let get_fd: c_int;
	let err: c_int;

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { return; }

	err = add_geneve_tunnel(GENEVE_TUNL_DEV0, GENEVE_TUNL_DEV1, c!("geneve"), c!("dstport 6081"));
	if !assert_ok(err, c!("add tunnel")) { goto_done_geneve(skel); return; }

	set_fd = bpf_program__fd((*skel).progs.geneve_set_tunnel);
	get_fd = bpf_program__fd((*skel).progs.geneve_get_tunnel);
	if tc_prog_attach(GENEVE_TUNL_DEV1, get_fd, set_fd) != 0 { goto_done_geneve(skel); return; }

	ping_dev0();
	ping_dev1();
	goto_done_geneve(skel);
}

unsafe fn goto_done_geneve(skel: *mut test_tunnel_kern) {
	delete_tunnel(GENEVE_TUNL_DEV0, GENEVE_TUNL_DEV1);
	test_tunnel_kern__destroy(skel);
}

unsafe fn test_ip6geneve_tunnel() {
	let skel: *mut test_tunnel_kern;
	let set_fd: c_int;
	let get_fd: c_int;
	let err: c_int;

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { return; }

	err = add_ip6geneve_tunnel(IP6GENEVE_TUNL_DEV0, IP6GENEVE_TUNL_DEV1, c!("geneve"), c!(""));
	if !assert_ok(err, c!("add tunnel")) { goto_done_ip6geneve(skel); return; }

	set_fd = bpf_program__fd((*skel).progs.ip6geneve_set_tunnel);
	get_fd = bpf_program__fd((*skel).progs.ip6geneve_get_tunnel);
	if tc_prog_attach(IP6GENEVE_TUNL_DEV1, get_fd, set_fd) != 0 { goto_done_ip6geneve(skel); return; }

	ping_dev0();
	ping_dev1();
	goto_done_ip6geneve(skel);
}

unsafe fn goto_done_ip6geneve(skel: *mut test_tunnel_kern) {
	delete_tunnel(IP6GENEVE_TUNL_DEV0, IP6GENEVE_TUNL_DEV1);
	test_tunnel_kern__destroy(skel);
}

#[repr(C)]
#[derive(Copy, Clone)]
enum ip6tnl_test {
	IPIP6,
	IP6IP6,
}

unsafe fn test_ip6tnl_tunnel(test: ip6tnl_test) {
	let skel: *mut test_tunnel_kern;
	let set_fd: c_int;
	let get_fd: c_int;
	let err: c_int;

	skel = test_tunnel_kern__open_and_load();
	if !assert_ok_ptr(skel as *mut c_void, c!("test_tunnel_kern__open_and_load")) { return; }

	err = add_ipv6_tunnel(IP6TNL_TUNL_DEV0, IP6TNL_TUNL_DEV1, c!("ip6tnl"), c!(""));
	if !assert_ok(err, c!("add tunnel")) { goto_done_ip6tnl(skel); return; }

	match test {
		ip6tnl_test::IPIP6 => {
			set_fd = bpf_program__fd((*skel).progs.ipip6_set_tunnel);
			get_fd = bpf_program__fd((*skel).progs.ipip6_get_tunnel);
		}
		ip6tnl_test::IP6IP6 => {
			set_fd = bpf_program__fd((*skel).progs.ip6ip6_set_tunnel);
			get_fd = bpf_program__fd((*skel).progs.ip6ip6_get_tunnel);
		}
	}
	if tc_prog_attach(IP6TNL_TUNL_DEV1, get_fd, set_fd) != 0 { goto_done_ip6tnl(skel); return; }

	ping6_veth0();
	match test {
		ip6tnl_test::IPIP6 => {
			ping_dev0();
			ping_dev1();
		}
		ip6tnl_test::IP6IP6 => {
			ping6_dev0();
			ping6_dev1();
		}
	}

	goto_done_ip6tnl(skel);
}

unsafe fn goto_done_ip6tnl(skel: *mut test_tunnel_kern) {
	delete_tunnel(IP6TNL_TUNL_DEV0, IP6TNL_TUNL_DEV1);
	test_tunnel_kern__destroy(skel);
}

macro_rules! RUN_TEST {
	($name:literal, $body:block) => {
		if test__start_subtest(c!($name)) {
			config_device();
			$body
			cleanup();
		}
	};
}

unsafe extern "C" fn test_tunnel_run_tests(_arg: *mut c_void) -> *mut c_void {
	RUN_TEST!("vxlan_tunnel", { test_vxlan_tunnel(); });
	RUN_TEST!("ip6vxlan_tunnel", { test_ip6vxlan_tunnel(); });
	RUN_TEST!("ipip_tunnel", { test_ipip_tunnel(ipip_encap::NONE); });
	RUN_TEST!("ipip_tunnel", { test_ipip_tunnel(ipip_encap::FOU); });
	RUN_TEST!("ipip_tunnel", { test_ipip_tunnel(ipip_encap::GUE); });
	RUN_TEST!("xfrm_tunnel", { test_xfrm_tunnel(); });
	RUN_TEST!("gre_tunnel", { test_gre_tunnel(gre_test::GRE); });
	RUN_TEST!("gre_tunnel", { test_gre_tunnel(gre_test::GRE_NOKEY); });
	RUN_TEST!("gre_tunnel", { test_gre_tunnel(gre_test::GRETAP); });
	RUN_TEST!("gre_tunnel", { test_gre_tunnel(gre_test::GRETAP_NOKEY); });
	RUN_TEST!("ip6gre_tunnel", { test_ip6gre_tunnel(ip6gre_test::IP6GRE); });
	RUN_TEST!("ip6gre_tunnel", { test_ip6gre_tunnel(ip6gre_test::IP6GRETAP); });
	RUN_TEST!("erspan_tunnel", { test_erspan_tunnel(erspan_test::V1); });
	RUN_TEST!("erspan_tunnel", { test_erspan_tunnel(erspan_test::V2); });
	RUN_TEST!("ip6erspan_tunnel", { test_ip6erspan_tunnel(erspan_test::V1); });
	RUN_TEST!("ip6erspan_tunnel", { test_ip6erspan_tunnel(erspan_test::V2); });
	RUN_TEST!("geneve_tunnel", { test_geneve_tunnel(); });
	RUN_TEST!("ip6geneve_tunnel", { test_ip6geneve_tunnel(); });
	RUN_TEST!("ip6tnl_tunnel", { test_ip6tnl_tunnel(ip6tnl_test::IPIP6); });
	RUN_TEST!("ip6tnl_tunnel", { test_ip6tnl_tunnel(ip6tnl_test::IP6IP6); });

	ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn test_tunnel() {
	let mut test_thread: pthread_t = 0;
	let mut err: c_int;

	/* Run the tests in their own thread to isolate the namespace changes
	 * so they do not affect the environment of other tests.
	 * (specifically needed because of unshare(CLONE_NEWNS) in open_netns())
	 */
	err = pthread_create(&mut test_thread, ptr::null(), test_tunnel_run_tests, ptr::null_mut());
	if assert_ok(err, c!("pthread_create")) {
		err = pthread_join(test_thread, ptr::null_mut());
		assert_ok(err, c!("pthread_join"));
	}
}
