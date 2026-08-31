// SPDX-License-Identifier: GPL-2.0

/*
 * Network topology:
 *  -----------        -----------
 *  |  NS1    |        |   NS2   |
 *  | veth0  -|--------|- veth0  |
 *  -----------        -----------
 *
 */

// C dependencies from:
// <net/if.h>, <uapi/linux/if_link.h>, "network_helpers.h",
// "test_progs.h", and "test_xdp_vlan.skel.h".

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

type u32 = c_uint;

const VETH_NAME: &[u8] = b"veth0\0";
const NS_MAX_SIZE: usize = 32;
const NS1_NAME: &[u8] = b"ns-xdp-vlan-1-\0";
const NS2_NAME: &[u8] = b"ns-xdp-vlan-2-\0";
const NS1_IP_ADDR: &[u8] = b"100.64.10.1\0";
const NS2_IP_ADDR: &[u8] = b"100.64.10.2\0";
const VLAN_ID: c_int = 4011;

// External constants supplied by Linux/libbpf headers.
extern "C" {
    static BPF_TC_EGRESS: c_int;
    static XDP_FLAGS_DRV_MODE: u32;
    static XDP_FLAGS_SKB_MODE: u32;
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_tc_hook {
    pub attach_point: c_int,
    pub ifindex: c_int,
}

#[repr(C)]
pub struct bpf_tc_opts {
    pub prog_fd: c_int,
    pub handle: c_int,
    pub priority: c_int,
}

#[repr(C)]
pub struct test_xdp_vlan {
    pub progs: test_xdp_vlan_progs,
}

#[repr(C)]
pub struct test_xdp_vlan_progs {
    pub xdp_vlan_change: *mut bpf_program,
    pub xdp_vlan_remove_outer2: *mut bpf_program,
    pub tc_vlan_push: *mut bpf_program,
}

extern "C" {
    fn append_tid(buf: *mut c_char, len: usize) -> c_int;
    fn ASSERT_OK(ret: c_int, msg: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut core::ffi::c_void, msg: *const c_char) -> bool;
    fn ASSERT_NEQ(left: c_int, right: c_int, msg: *const c_char) -> bool;
    fn SYS(label: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn SYS_NOFAIL(fmt: *const c_char, ...) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: u32, opts: *const core::ffi::c_void) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: u32, opts: *const core::ffi::c_void) -> c_int;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_tc_detach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn test_xdp_vlan__open_and_load() -> *mut test_xdp_vlan;
    fn test_xdp_vlan__destroy(skel: *mut test_xdp_vlan);
    fn test__start_subtest(name: *const c_char) -> bool;
}

fn init_name(name: &[u8]) -> [c_char; NS_MAX_SIZE] {
    let mut out = [0 as c_char; NS_MAX_SIZE];
    let mut i = 0;

    while i < name.len() && i < NS_MAX_SIZE {
        out[i] = name[i] as c_char;
        i += 1;
    }

    out
}

unsafe fn setup_network(ns1: *mut c_char, ns2: *mut c_char) -> c_int {
    if !ASSERT_OK(append_tid(ns1, NS_MAX_SIZE), b"create ns1 name\0".as_ptr() as *const c_char) {
        return -1;
    }
    if !ASSERT_OK(append_tid(ns2, NS_MAX_SIZE), b"create ns2 name\0".as_ptr() as *const c_char) {
        return -1;
    }

    if SYS(b"fail\0".as_ptr() as *const c_char, b"ip netns add %s\0".as_ptr() as *const c_char, ns1) != 0 {
        return -1;
    }
    if SYS(b"fail\0".as_ptr() as *const c_char, b"ip netns add %s\0".as_ptr() as *const c_char, ns2) != 0 {
        return -1;
    }
    if SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip -n %s link add %s type veth peer name %s netns %s\0".as_ptr() as *const c_char,
        ns1,
        VETH_NAME.as_ptr() as *const c_char,
        VETH_NAME.as_ptr() as *const c_char,
        ns2,
    ) != 0 {
        return -1;
    }

    /* NOTICE: XDP require VLAN header inside packet payload
     *  - Thus, disable VLAN offloading driver features
     */
    if SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip netns exec %s ethtool -K %s rxvlan off txvlan off\0".as_ptr() as *const c_char,
        ns1,
        VETH_NAME.as_ptr() as *const c_char,
    ) != 0 {
        return -1;
    }
    if SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip netns exec %s ethtool -K %s rxvlan off txvlan off\0".as_ptr() as *const c_char,
        ns2,
        VETH_NAME.as_ptr() as *const c_char,
    ) != 0 {
        return -1;
    }

    /* NS1 configuration */
    if SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip -n %s addr add %s/24 dev %s\0".as_ptr() as *const c_char,
        ns1,
        NS1_IP_ADDR.as_ptr() as *const c_char,
        VETH_NAME.as_ptr() as *const c_char,
    ) != 0 {
        return -1;
    }
    if SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip -n %s link set %s up\0".as_ptr() as *const c_char,
        ns1,
        VETH_NAME.as_ptr() as *const c_char,
    ) != 0 {
        return -1;
    }

    /* NS2 configuration */
    if SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip -n %s link add link %s name %s.%d type vlan id %d\0".as_ptr() as *const c_char,
        ns2,
        VETH_NAME.as_ptr() as *const c_char,
        VETH_NAME.as_ptr() as *const c_char,
        VLAN_ID,
        VLAN_ID,
    ) != 0 {
        return -1;
    }
    if SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip -n %s addr add %s/24 dev %s.%d\0".as_ptr() as *const c_char,
        ns2,
        NS2_IP_ADDR.as_ptr() as *const c_char,
        VETH_NAME.as_ptr() as *const c_char,
        VLAN_ID,
    ) != 0 {
        return -1;
    }
    if SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip -n %s link set %s up\0".as_ptr() as *const c_char,
        ns2,
        VETH_NAME.as_ptr() as *const c_char,
    ) != 0 {
        return -1;
    }
    if SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip -n %s link set %s.%d up\0".as_ptr() as *const c_char,
        ns2,
        VETH_NAME.as_ptr() as *const c_char,
        VLAN_ID,
    ) != 0 {
        return -1;
    }

    /* At this point ping should fail because VLAN tags are only used by NS2 */
    if SYS_NOFAIL(
        b"ip netns exec %s ping -W 1 -c1 %s\0".as_ptr() as *const c_char,
        ns2,
        NS1_IP_ADDR.as_ptr() as *const c_char,
    ) == 0 {
        1
    } else {
        0
    }
}

unsafe fn cleanup_network(ns1: *const c_char, ns2: *const c_char) {
    SYS_NOFAIL(b"ip netns del %s\0".as_ptr() as *const c_char, ns1);
    SYS_NOFAIL(b"ip netns del %s\0".as_ptr() as *const c_char, ns2);
}

unsafe fn xdp_vlan(xdp: *mut bpf_program, tc: *mut bpf_program, flags: u32) {
    let mut tc_hook = bpf_tc_hook {
        attach_point: BPF_TC_EGRESS,
        ifindex: 0,
    };
    let mut tc_opts = bpf_tc_opts {
        prog_fd: 0,
        handle: 1,
        priority: 1,
    };
    let mut ns1 = init_name(NS1_NAME);
    let mut ns2 = init_name(NS2_NAME);
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut interface: c_int = 0;
    let mut tc_attached = false;
    let mut xdp_attached = false;
    let mut ret: c_int;

    if !ASSERT_OK(setup_network(ns1.as_mut_ptr(), ns2.as_mut_ptr()), b"setup network\0".as_ptr() as *const c_char) {
        cleanup_network(ns1.as_ptr(), ns2.as_ptr());
        return;
    }

    nstoken = open_netns(ns1.as_ptr());
    if !ASSERT_OK_PTR(nstoken as *mut core::ffi::c_void, b"open NS1\0".as_ptr() as *const c_char) {
        close_netns(nstoken);
        cleanup_network(ns1.as_ptr(), ns2.as_ptr());
        return;
    }

    interface = if_nametoindex(VETH_NAME.as_ptr() as *const c_char) as c_int;
    if !ASSERT_NEQ(interface, 0, b"get interface index\0".as_ptr() as *const c_char) {
        close_netns(nstoken);
        cleanup_network(ns1.as_ptr(), ns2.as_ptr());
        return;
    }

    ret = bpf_xdp_attach(interface, bpf_program__fd(xdp), flags, ptr::null());
    if !ASSERT_OK(ret, b"attach xdp_vlan_change\0".as_ptr() as *const c_char) {
        close_netns(nstoken);
        cleanup_network(ns1.as_ptr(), ns2.as_ptr());
        return;
    }
    xdp_attached = true;

    tc_hook.ifindex = interface;
    ret = bpf_tc_hook_create(&mut tc_hook);
    if !ASSERT_OK(ret, b"bpf_tc_hook_create\0".as_ptr() as *const c_char) {
        bpf_xdp_detach(interface, flags, ptr::null());
        close_netns(nstoken);
        cleanup_network(ns1.as_ptr(), ns2.as_ptr());
        return;
    }

    /* Now we'll use BPF programs to pop/push the VLAN tags */
    tc_opts.prog_fd = bpf_program__fd(tc);
    ret = bpf_tc_attach(&mut tc_hook, &mut tc_opts);
    if !ASSERT_OK(ret, b"bpf_tc_attach\0".as_ptr() as *const c_char) {
        bpf_xdp_detach(interface, flags, ptr::null());
        close_netns(nstoken);
        cleanup_network(ns1.as_ptr(), ns2.as_ptr());
        return;
    }
    tc_attached = true;

    close_netns(nstoken);
    nstoken = ptr::null_mut();

    /* Now the namespaces can reach each-other, test with pings */
    if SYS(
        b"detach_tc\0".as_ptr() as *const c_char,
        b"ip netns exec %s ping -i 0.2 -W 2 -c 2 %s > /dev/null\0".as_ptr() as *const c_char,
        ns1.as_ptr(),
        NS2_IP_ADDR.as_ptr() as *const c_char,
    ) != 0 {
        if tc_attached {
            bpf_tc_detach(&mut tc_hook, &mut tc_opts);
        }
        if xdp_attached {
            bpf_xdp_detach(interface, flags, ptr::null());
        }
        close_netns(nstoken);
        cleanup_network(ns1.as_ptr(), ns2.as_ptr());
        return;
    }
    SYS(
        b"detach_tc\0".as_ptr() as *const c_char,
        b"ip netns exec %s ping -i 0.2 -W 2 -c 2 %s > /dev/null\0".as_ptr() as *const c_char,
        ns2.as_ptr(),
        NS1_IP_ADDR.as_ptr() as *const c_char,
    );

    if tc_attached {
        bpf_tc_detach(&mut tc_hook, &mut tc_opts);
    }
    if xdp_attached {
        bpf_xdp_detach(interface, flags, ptr::null());
    }
    close_netns(nstoken);
    cleanup_network(ns1.as_ptr(), ns2.as_ptr());
}

/* First test: Remove VLAN by setting VLAN ID 0, using "xdp_vlan_change"
 * egress use TC to add back VLAN tag 4011
 */
#[no_mangle]
pub unsafe extern "C" fn test_xdp_vlan_change() {
    let skel: *mut test_xdp_vlan;

    skel = test_xdp_vlan__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut core::ffi::c_void, b"xdp_vlan__open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    if test__start_subtest(b"0\0".as_ptr() as *const c_char) {
        xdp_vlan((*skel).progs.xdp_vlan_change, (*skel).progs.tc_vlan_push, 0);
    }

    if test__start_subtest(b"DRV_MODE\0".as_ptr() as *const c_char) {
        xdp_vlan(
            (*skel).progs.xdp_vlan_change,
            (*skel).progs.tc_vlan_push,
            XDP_FLAGS_DRV_MODE,
        );
    }

    if test__start_subtest(b"SKB_MODE\0".as_ptr() as *const c_char) {
        xdp_vlan(
            (*skel).progs.xdp_vlan_change,
            (*skel).progs.tc_vlan_push,
            XDP_FLAGS_SKB_MODE,
        );
    }

    test_xdp_vlan__destroy(skel);
}

/* Second test: XDP prog fully remove vlan header
 *
 * Catch kernel bug for generic-XDP, that doesn't allow us to
 * remove a VLAN header, because skb->protocol still contain VLAN
 * ETH_P_8021Q indication, and this cause overwriting of our changes.
 */
#[no_mangle]
pub unsafe extern "C" fn test_xdp_vlan_remove() {
    let skel: *mut test_xdp_vlan;

    skel = test_xdp_vlan__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut core::ffi::c_void, b"xdp_vlan__open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    if test__start_subtest(b"0\0".as_ptr() as *const c_char) {
        xdp_vlan((*skel).progs.xdp_vlan_remove_outer2, (*skel).progs.tc_vlan_push, 0);
    }

    if test__start_subtest(b"DRV_MODE\0".as_ptr() as *const c_char) {
        xdp_vlan(
            (*skel).progs.xdp_vlan_remove_outer2,
            (*skel).progs.tc_vlan_push,
            XDP_FLAGS_DRV_MODE,
        );
    }

    if test__start_subtest(b"SKB_MODE\0".as_ptr() as *const c_char) {
        xdp_vlan(
            (*skel).progs.xdp_vlan_remove_outer2,
            (*skel).progs.tc_vlan_push,
            XDP_FLAGS_SKB_MODE,
        );
    }

    test_xdp_vlan__destroy(skel);
}
