// SPDX-License-Identifier: GPL-2.0

/* Create 3 namespaces with 3 veth peers, and forward packets in-between using
 * native XDP
 *
 * Network topology:
 *  ----------        ----------       ----------
 *  |  NS1   |        |  NS2   |       |  NS3   |
 *  | veth11 |        | veth22 |       | veth33 |
 *  ----|-----        -----|----       -----|----
 *      |                  |                |
 *  ----|------------------|----------------|----
 *  | veth1              veth2            veth3 |
 *  |                                           |
 *  |                     NSO                   |
 *  ---------------------------------------------
 *
 * Test cases:
 *  - [test_xdp_veth_redirect] : ping veth33 from veth11
 *
 *    veth11             veth22              veth33
 *  (XDP_PASS)          (XDP_TX)           (XDP_PASS)
 *       |                  |                  |
 *       |                  |                  |
 *     veth1             veth2              veth3
 * (XDP_REDIRECT)     (XDP_REDIRECT)     (XDP_REDIRECT)
 *      ^ |                ^ |                ^ |
 *      | |                | |                | |
 *      | ------------------ ------------------ |
 *      -----------------------------------------
 *
 * - [test_xdp_veth_broadcast_redirect]: broadcast from veth11
 *     - IPv4 ping : BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS
 *          -> echo request received by all except veth11
 *     - IPv4 ping : BPF_F_BROADCAST
 *          -> echo request received by all veth
 * - [test_xdp_veth_egress]:
 *     - all src mac should be the magic mac
 *
 *    veth11             veth22              veth33
 *  (XDP_PASS)         (XDP_PASS)          (XDP_PASS)
 *       |                  |                  |
 *       |                  |                  |
 *     veth1		  veth2              veth3
 * (XDP_REDIRECT)     (XDP_REDIRECT)     (XDP_REDIRECT)
 *      |                   ^                  ^
 *      |                   |                  |
 *      ----------------------------------------
 *
 */

// C dependencies: <net/if.h>, "test_progs.h", "network_helpers.h",
// "xdp_dummy.skel.h", "xdp_redirect_map.skel.h",
// "xdp_redirect_multi_kern.skel.h", "xdp_tx.skel.h",
// <uapi/linux/if_link.h>.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem;
use core::ptr;

const VETH_PAIRS_COUNT: usize = 3;
const VETH_NAME_MAX_LEN: usize = 32;
const IP_MAX_LEN: usize = 16;
const IP_SRC: &[u8] = b"10.1.1.11\0";
const IP_DST: &[u8] = b"10.1.1.33\0";
const IP_NEIGH: &[u8] = b"10.1.1.253\0";
const PROG_NAME_MAX_LEN: usize = 128;
const NS_NAME_MAX_LEN: usize = 32;

type u16_t = u16;
type u32_t = u32;
type u64_t = u64;
type __be64 = u64;

const ETH_ALEN: usize = 6;
const ETH_P_IP: u16_t = 0x0800;
const BPF_ANY: u64_t = 0;
const BPF_NOEXIST: u64_t = 1;
const BPF_F_BROADCAST: u64_t = 1 << 3;
const BPF_F_EXCLUDE_INGRESS: u64_t = 1 << 4;
const XDP_FLAGS_DRV_MODE: u32_t = 1 << 2;
const XDP_FLAGS_SKB_MODE: u32_t = 1 << 1;

#[repr(C)]
struct bpf_object;
#[repr(C)]
struct bpf_program;
#[repr(C)]
struct bpf_map;
#[repr(C)]
struct nstoken;

#[repr(C)]
struct bpf_devmap_val_bpf_prog {
    fd: c_int,
}

#[repr(C)]
struct bpf_devmap_val {
    ifindex: u32_t,
    bpf_prog: bpf_devmap_val_bpf_prog,
}

#[repr(C)]
struct xdp_dummy {
    obj: *mut bpf_object,
}

#[repr(C)]
struct xdp_tx {
    obj: *mut bpf_object,
}

#[repr(C)]
struct xdp_redirect_map_maps {
    tx_port: *mut bpf_map,
    rxcnt: *mut bpf_map,
    rx_mac: *mut bpf_map,
}

#[repr(C)]
struct xdp_redirect_map {
    obj: *mut bpf_object,
    maps: xdp_redirect_map_maps,
}

#[repr(C)]
struct xdp_redirect_multi_kern_maps {
    map_all: *mut bpf_map,
    redirect_flags: *mut bpf_map,
    mac_map: *mut bpf_map,
    map_egress: *mut bpf_map,
}

#[repr(C)]
struct xdp_redirect_multi_kern_progs {
    xdp_devmap_prog: *mut bpf_program,
}

#[repr(C)]
struct xdp_redirect_multi_kern {
    obj: *mut bpf_object,
    maps: xdp_redirect_multi_kern_maps,
    progs: xdp_redirect_multi_kern_progs,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct veth_configuration {
    local_veth: [c_char; VETH_NAME_MAX_LEN], /* Interface in main namespace */
    remote_veth: [c_char; VETH_NAME_MAX_LEN], /* Peer interface in dedicated namespace*/
    namespace: [c_char; NS_NAME_MAX_LEN], /* Namespace for the remote veth */
    next_veth: c_int, /* Local interface to redirect traffic to */
    remote_addr: [c_char; IP_MAX_LEN], /* IP address of the remote veth */
}

#[repr(C)]
#[derive(Copy, Clone)]
struct net_configuration {
    ns0_name: [c_char; NS_NAME_MAX_LEN],
    veth_cfg: [veth_configuration; VETH_PAIRS_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct prog_configuration {
    local_name: [c_char; PROG_NAME_MAX_LEN], /* BPF prog to attach to local_veth */
    remote_name: [c_char; PROG_NAME_MAX_LEN], /* BPF prog to attach to remote_veth */
    local_flags: u32_t, /* XDP flags to use on local_veth */
    remote_flags: u32_t, /* XDP flags to use on remote_veth */
}

const fn cstr<const N: usize>(s: &[u8]) -> [c_char; N] {
    let mut out = [0 as c_char; N];
    let mut i = 0;
    while i < s.len() && i < N {
        out[i] = s[i] as c_char;
        i += 1;
    }
    out
}

static default_config: net_configuration = net_configuration {
    ns0_name: cstr::<NS_NAME_MAX_LEN>(b"ns0-\0"),
    veth_cfg: [
        veth_configuration {
            local_veth: cstr::<VETH_NAME_MAX_LEN>(b"veth1-\0"),
            remote_veth: cstr::<VETH_NAME_MAX_LEN>(b"veth11\0"),
            next_veth: 1,
            remote_addr: cstr::<IP_MAX_LEN>(IP_SRC),
            namespace: cstr::<NS_NAME_MAX_LEN>(b"ns-veth11-\0"),
        },
        veth_configuration {
            local_veth: cstr::<VETH_NAME_MAX_LEN>(b"veth2-\0"),
            remote_veth: cstr::<VETH_NAME_MAX_LEN>(b"veth22\0"),
            next_veth: 2,
            remote_addr: cstr::<IP_MAX_LEN>(b"\0"),
            namespace: cstr::<NS_NAME_MAX_LEN>(b"ns-veth22-\0"),
        },
        veth_configuration {
            local_veth: cstr::<VETH_NAME_MAX_LEN>(b"veth3-\0"),
            remote_veth: cstr::<VETH_NAME_MAX_LEN>(b"veth33\0"),
            next_veth: 0,
            remote_addr: cstr::<IP_MAX_LEN>(IP_DST),
            namespace: cstr::<NS_NAME_MAX_LEN>(b"ns-veth33-\0"),
        },
    ],
};

extern "C" {
    fn append_tid(name: *mut c_char, max_len: usize) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: u32_t, opts: *const c_void) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64_t) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn SYS(label: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn SYS_NOFAIL(fmt: *const c_char, ...) -> c_int;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_NEQ<T: PartialEq>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_EQ<T: PartialEq>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn xdp_dummy__open_and_load() -> *mut xdp_dummy;
    fn xdp_dummy__destroy(obj: *mut xdp_dummy);
    fn xdp_tx__open_and_load() -> *mut xdp_tx;
    fn xdp_tx__destroy(obj: *mut xdp_tx);
    fn xdp_redirect_map__open_and_load() -> *mut xdp_redirect_map;
    fn xdp_redirect_map__destroy(obj: *mut xdp_redirect_map);
    fn xdp_redirect_multi_kern__open_and_load() -> *mut xdp_redirect_multi_kern;
    fn xdp_redirect_multi_kern__destroy(obj: *mut xdp_redirect_multi_kern);
}

unsafe fn attach_programs_to_veth_pair(
    objs: *mut *mut bpf_object,
    nb_obj: usize,
    net_config: *mut net_configuration,
    prog: *mut prog_configuration,
    index: c_int,
) -> c_int {
    let mut local_prog: *mut bpf_program = ptr::null_mut();
    let mut remote_prog: *mut bpf_program = ptr::null_mut();
    let mut nstoken: *mut nstoken;
    let mut interface: c_int;
    let mut ret: c_int;
    let mut i: usize;

    i = 0;
    while i < nb_obj {
        local_prog = bpf_object__find_program_by_name(*objs.add(i), (*prog.add(index as usize)).local_name.as_ptr());
        if !local_prog.is_null() {
            break;
        }
        i += 1;
    }
    if !ASSERT_OK_PTR(local_prog, b"find local program\0".as_ptr() as *const c_char) {
        return -1;
    }

    i = 0;
    while i < nb_obj {
        remote_prog = bpf_object__find_program_by_name(*objs.add(i), (*prog.add(index as usize)).remote_name.as_ptr());
        if !remote_prog.is_null() {
            break;
        }
        i += 1;
    }
    if !ASSERT_OK_PTR(remote_prog, b"find remote program\0".as_ptr() as *const c_char) {
        return -1;
    }

    interface = if_nametoindex((*net_config).veth_cfg[index as usize].local_veth.as_ptr()) as c_int;
    if !ASSERT_NEQ(interface, 0, b"non zero interface index\0".as_ptr() as *const c_char) {
        return -1;
    }

    ret = bpf_xdp_attach(
        interface,
        bpf_program__fd(local_prog),
        (*prog.add(index as usize)).local_flags,
        ptr::null(),
    );
    if !ASSERT_OK(ret, b"attach xdp program to local veth\0".as_ptr() as *const c_char) {
        return -1;
    }

    nstoken = open_netns((*net_config).veth_cfg[index as usize].namespace.as_ptr());
    if !ASSERT_OK_PTR(nstoken, b"switch to remote veth namespace\0".as_ptr() as *const c_char) {
        return -1;
    }

    interface = if_nametoindex((*net_config).veth_cfg[index as usize].remote_veth.as_ptr()) as c_int;
    if !ASSERT_NEQ(interface, 0, b"non zero interface index\0".as_ptr() as *const c_char) {
        close_netns(nstoken);
        return -1;
    }

    ret = bpf_xdp_attach(
        interface,
        bpf_program__fd(remote_prog),
        (*prog.add(index as usize)).remote_flags,
        ptr::null(),
    );
    if !ASSERT_OK(ret, b"attach xdp program to remote veth\0".as_ptr() as *const c_char) {
        close_netns(nstoken);
        return -1;
    }

    close_netns(nstoken);
    0
}

unsafe fn create_network(net_config: *mut net_configuration) -> c_int {
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut i: usize;
    let mut err: c_int;

    ptr::copy_nonoverlapping(&default_config, net_config, 1);

    /* Create unique namespaces */
    err = append_tid((*net_config).ns0_name.as_mut_ptr(), NS_NAME_MAX_LEN);
    if !ASSERT_OK(err, b"append TID to ns0 name\0".as_ptr() as *const c_char) {
        close_netns(nstoken);
        return -1;
    }
    if SYS(b"fail\0".as_ptr() as *const c_char, b"ip netns add %s\0".as_ptr() as *const c_char, (*net_config).ns0_name.as_ptr()) != 0 {
        close_netns(nstoken);
        return -1;
    }

    i = 0;
    while i < VETH_PAIRS_COUNT {
        err = append_tid((*net_config).veth_cfg[i].namespace.as_mut_ptr(), NS_NAME_MAX_LEN);
        if !ASSERT_OK(err, b"append TID to ns name\0".as_ptr() as *const c_char) {
            close_netns(nstoken);
            return -1;
        }
        if SYS(b"fail\0".as_ptr() as *const c_char, b"ip netns add %s\0".as_ptr() as *const c_char, (*net_config).veth_cfg[i].namespace.as_ptr()) != 0 {
            close_netns(nstoken);
            return -1;
        }
        i += 1;
    }

    /* Create interfaces */
    nstoken = open_netns((*net_config).ns0_name.as_ptr());
    if nstoken.is_null() {
        close_netns(nstoken);
        return -1;
    }

    i = 0;
    while i < VETH_PAIRS_COUNT {
        if SYS(
            b"fail\0".as_ptr() as *const c_char,
            b"ip link add %s type veth peer name %s netns %s\0".as_ptr() as *const c_char,
            (*net_config).veth_cfg[i].local_veth.as_ptr(),
            (*net_config).veth_cfg[i].remote_veth.as_ptr(),
            (*net_config).veth_cfg[i].namespace.as_ptr(),
        ) != 0 {
            close_netns(nstoken);
            return -1;
        }
        if SYS(
            b"fail\0".as_ptr() as *const c_char,
            b"ip link set dev %s up\0".as_ptr() as *const c_char,
            (*net_config).veth_cfg[i].local_veth.as_ptr(),
        ) != 0 {
            close_netns(nstoken);
            return -1;
        }
        if (*net_config).veth_cfg[i].remote_addr[0] != 0 {
            if SYS(
                b"fail\0".as_ptr() as *const c_char,
                b"ip -n %s addr add %s/24 dev %s\0".as_ptr() as *const c_char,
                (*net_config).veth_cfg[i].namespace.as_ptr(),
                (*net_config).veth_cfg[i].remote_addr.as_ptr(),
                (*net_config).veth_cfg[i].remote_veth.as_ptr(),
            ) != 0 {
                close_netns(nstoken);
                return -1;
            }
        }
        if SYS(
            b"fail\0".as_ptr() as *const c_char,
            b"ip -n %s link set dev %s up\0".as_ptr() as *const c_char,
            (*net_config).veth_cfg[i].namespace.as_ptr(),
            (*net_config).veth_cfg[i].remote_veth.as_ptr(),
        ) != 0 {
            close_netns(nstoken);
            return -1;
        }
        i += 1;
    }

    close_netns(nstoken);
    0
}

unsafe fn cleanup_network(net_config: *mut net_configuration) {
    let mut i: usize;

    SYS_NOFAIL(b"ip netns del %s\0".as_ptr() as *const c_char, (*net_config).ns0_name.as_ptr());
    i = 0;
    while i < VETH_PAIRS_COUNT {
        SYS_NOFAIL(b"ip netns del %s\0".as_ptr() as *const c_char, (*net_config).veth_cfg[i].namespace.as_ptr());
        i += 1;
    }
}

const VETH_REDIRECT_SKEL_NB: usize = 3;
unsafe fn xdp_veth_redirect(flags: u32_t) {
    let mut ping_config: [prog_configuration; VETH_PAIRS_COUNT] = [
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_0\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_dummy_prog\0"),
            local_flags: flags,
            remote_flags: flags,
        },
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_1\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_tx\0"),
            local_flags: flags,
            remote_flags: flags,
        },
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_2\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_dummy_prog\0"),
            local_flags: flags,
            remote_flags: flags,
        },
    ];
    let mut bpf_objs: [*mut bpf_object; VETH_REDIRECT_SKEL_NB] = [ptr::null_mut(); VETH_REDIRECT_SKEL_NB];
    let mut xdp_redirect_map: *mut xdp_redirect_map;
    let mut net_config: net_configuration = mem::zeroed();
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut xdp_dummy: *mut xdp_dummy;
    let mut xdp_tx: *mut xdp_tx;
    let mut map_fd: c_int;
    let mut i: usize;

    xdp_dummy = xdp_dummy__open_and_load();
    if !ASSERT_OK_PTR(xdp_dummy, b"xdp_dummy__open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    xdp_tx = xdp_tx__open_and_load();
    if !ASSERT_OK_PTR(xdp_tx, b"xdp_tx__open_and_load\0".as_ptr() as *const c_char) {
        xdp_dummy__destroy(xdp_dummy);
        cleanup_network(&mut net_config);
        return;
    }

    xdp_redirect_map = xdp_redirect_map__open_and_load();
    if !ASSERT_OK_PTR(xdp_redirect_map, b"xdp_redirect_map__open_and_load\0".as_ptr() as *const c_char) {
        xdp_tx__destroy(xdp_tx);
        xdp_dummy__destroy(xdp_dummy);
        cleanup_network(&mut net_config);
        return;
    }

    if ASSERT_OK(create_network(&mut net_config), b"create network\0".as_ptr() as *const c_char) {
        /* Then configure the redirect map and attach programs to interfaces */
        map_fd = bpf_map__fd((*xdp_redirect_map).maps.tx_port);
        if ASSERT_OK_FD(map_fd, b"open redirect map\0".as_ptr() as *const c_char) {
            bpf_objs[0] = (*xdp_dummy).obj;
            bpf_objs[1] = (*xdp_tx).obj;
            bpf_objs[2] = (*xdp_redirect_map).obj;

            nstoken = open_netns(net_config.ns0_name.as_ptr());
            if ASSERT_OK_PTR(nstoken, b"open NS0\0".as_ptr() as *const c_char) {
                i = 0;
                while i < VETH_PAIRS_COUNT {
                    let next_veth: c_int = net_config.veth_cfg[i].next_veth;
                    let interface_id: c_int;
                    let err: c_int;

                    interface_id = if_nametoindex(net_config.veth_cfg[next_veth as usize].local_veth.as_ptr()) as c_int;
                    if !ASSERT_NEQ(interface_id, 0, b"non zero interface index\0".as_ptr() as *const c_char) {
                        break;
                    }
                    err = bpf_map_update_elem(map_fd, &i as *const _ as *const c_void, &interface_id as *const _ as *const c_void, BPF_ANY);
                    if !ASSERT_OK(err, b"configure interface redirection through map\0".as_ptr() as *const c_char) {
                        break;
                    }
                    if attach_programs_to_veth_pair(bpf_objs.as_mut_ptr(), VETH_REDIRECT_SKEL_NB, &mut net_config, ping_config.as_mut_ptr(), i as c_int) != 0 {
                        break;
                    }
                    i += 1;
                }

                if i == VETH_PAIRS_COUNT {
                    /* Test: if all interfaces are properly configured, we must be able to ping
                     * veth33 from veth11
                     */
                    ASSERT_OK(
                        SYS_NOFAIL(
                            b"ip netns exec %s ping -c 1 -W 1 %s > /dev/null\0".as_ptr() as *const c_char,
                            net_config.veth_cfg[0].namespace.as_ptr(),
                            IP_DST.as_ptr() as *const c_char,
                        ),
                        b"ping\0".as_ptr() as *const c_char,
                    );
                }
            }
        }
    }

    close_netns(nstoken);
    xdp_redirect_map__destroy(xdp_redirect_map);
    xdp_tx__destroy(xdp_tx);
    xdp_dummy__destroy(xdp_dummy);

    cleanup_network(&mut net_config);
}

const BROADCAST_REDIRECT_SKEL_NB: usize = 2;
unsafe fn xdp_veth_broadcast_redirect(attach_flags: u32_t, redirect_flags: u64_t) {
    let mut prog_cfg: [prog_configuration; VETH_PAIRS_COUNT] = [
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_multi_prog\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_count_0\0"),
            local_flags: attach_flags,
            remote_flags: attach_flags,
        },
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_multi_prog\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_count_1\0"),
            local_flags: attach_flags,
            remote_flags: attach_flags,
        },
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_multi_prog\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_count_2\0"),
            local_flags: attach_flags,
            remote_flags: attach_flags,
        },
    ];
    let mut bpf_objs: [*mut bpf_object; BROADCAST_REDIRECT_SKEL_NB] = [ptr::null_mut(); BROADCAST_REDIRECT_SKEL_NB];
    let mut xdp_redirect_multi_kern: *mut xdp_redirect_multi_kern;
    let mut xdp_redirect_map: *mut xdp_redirect_map;
    let mut devmap_val: bpf_devmap_val = mem::zeroed();
    let mut net_config: net_configuration = mem::zeroed();
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let protocol: u16_t = ETH_P_IP;
    let mut group_map: c_int;
    let mut flags_map: c_int;
    let mut cnt_map: c_int;
    let mut cnt: u64_t = 0;
    let mut i: usize;
    let mut err: c_int;

    xdp_redirect_multi_kern = xdp_redirect_multi_kern__open_and_load();
    if !ASSERT_OK_PTR(xdp_redirect_multi_kern, b"xdp_redirect_multi_kern__open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    xdp_redirect_map = xdp_redirect_map__open_and_load();
    if !ASSERT_OK_PTR(xdp_redirect_map, b"xdp_redirect_map__open_and_load\0".as_ptr() as *const c_char) {
        xdp_redirect_multi_kern__destroy(xdp_redirect_multi_kern);
        cleanup_network(&mut net_config);
        return;
    }

    if ASSERT_OK(create_network(&mut net_config), b"create network\0".as_ptr() as *const c_char) {
        group_map = bpf_map__fd((*xdp_redirect_multi_kern).maps.map_all);
        if ASSERT_OK_FD(group_map, b"open map_all\0".as_ptr() as *const c_char) {
            flags_map = bpf_map__fd((*xdp_redirect_multi_kern).maps.redirect_flags);
            if ASSERT_OK_FD(group_map, b"open map_all\0".as_ptr() as *const c_char) {
                err = bpf_map_update_elem(flags_map, &protocol as *const _ as *const c_void, &redirect_flags as *const _ as *const c_void, BPF_NOEXIST);
                if ASSERT_OK(err, b"init IP count\0".as_ptr() as *const c_char) {
                    cnt_map = bpf_map__fd((*xdp_redirect_map).maps.rxcnt);
                    if ASSERT_OK_FD(cnt_map, b"open rxcnt map\0".as_ptr() as *const c_char) {
                        bpf_objs[0] = (*xdp_redirect_multi_kern).obj;
                        bpf_objs[1] = (*xdp_redirect_map).obj;

                        nstoken = open_netns(net_config.ns0_name.as_ptr());
                        if ASSERT_OK_PTR(nstoken, b"open NS0\0".as_ptr() as *const c_char) {
                            i = 0;
                            while i < VETH_PAIRS_COUNT {
                                let ifindex: c_int = if_nametoindex(net_config.veth_cfg[i].local_veth.as_ptr()) as c_int;

                                if attach_programs_to_veth_pair(bpf_objs.as_mut_ptr(), BROADCAST_REDIRECT_SKEL_NB, &mut net_config, prog_cfg.as_mut_ptr(), i as c_int) != 0 {
                                    break;
                                }

                                if SYS(
                                    b"destroy_xdp_redirect_map\0".as_ptr() as *const c_char,
                                    b"ip -n %s neigh add %s lladdr 00:00:00:00:00:01 dev %s\0".as_ptr() as *const c_char,
                                    net_config.veth_cfg[i].namespace.as_ptr(),
                                    IP_NEIGH.as_ptr() as *const c_char,
                                    net_config.veth_cfg[i].remote_veth.as_ptr(),
                                ) != 0 {
                                    break;
                                }

                                devmap_val.ifindex = ifindex as u32_t;
                                err = bpf_map_update_elem(group_map, &ifindex as *const _ as *const c_void, &devmap_val as *const _ as *const c_void, 0);
                                if !ASSERT_OK(err, b"bpf_map_update_elem\0".as_ptr() as *const c_char) {
                                    break;
                                }
                                i += 1;
                            }

                            if i == VETH_PAIRS_COUNT {
                                SYS_NOFAIL(
                                    b"ip netns exec %s ping %s -i 0.1 -c 4 -W1 > /dev/null \0".as_ptr() as *const c_char,
                                    net_config.veth_cfg[0].namespace.as_ptr(),
                                    IP_NEIGH.as_ptr() as *const c_char,
                                );

                                i = 0;
                                while i < VETH_PAIRS_COUNT {
                                    err = bpf_map_lookup_elem(cnt_map, &i as *const _ as *const c_void, &mut cnt as *mut _ as *mut c_void);
                                    if !ASSERT_OK(err, b"get IP cnt\0".as_ptr() as *const c_char) {
                                        break;
                                    }

                                    if (redirect_flags & BPF_F_EXCLUDE_INGRESS) != 0 {
                                        /* veth11 shouldn't receive the ICMP requests;
                                         * others should
                                         */
                                        ASSERT_EQ(cnt, if i != 0 { 4 } else { 0 }, b"compare IP cnt\0".as_ptr() as *const c_char);
                                    } else {
                                        /* All remote veth should receive the ICMP requests */
                                        ASSERT_EQ(cnt, 4, b"compare IP cnt\0".as_ptr() as *const c_char);
                                    }
                                    i += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    close_netns(nstoken);
    xdp_redirect_map__destroy(xdp_redirect_map);
    xdp_redirect_multi_kern__destroy(xdp_redirect_multi_kern);

    cleanup_network(&mut net_config);
}

const VETH_EGRESS_SKEL_NB: usize = 3;
unsafe fn xdp_veth_egress(flags: u32_t) {
    let mut prog_cfg: [prog_configuration; VETH_PAIRS_COUNT] = [
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_all_prog\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_dummy_prog\0"),
            local_flags: flags,
            remote_flags: flags,
        },
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_all_prog\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"store_mac_1\0"),
            local_flags: flags,
            remote_flags: flags,
        },
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_all_prog\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"store_mac_2\0"),
            local_flags: flags,
            remote_flags: flags,
        },
    ];
    let egress_macs: [[u8; ETH_ALEN]; VETH_PAIRS_COUNT] = [
        [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x01],
        [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x02],
        [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x03],
    ];
    let mut xdp_redirect_multi_kern: *mut xdp_redirect_multi_kern;
    let mut bpf_objs: [*mut bpf_object; VETH_EGRESS_SKEL_NB] = [ptr::null_mut(); VETH_EGRESS_SKEL_NB];
    let mut xdp_redirect_map: *mut xdp_redirect_map;
    let mut devmap_val: bpf_devmap_val = mem::zeroed();
    let mut net_config: net_configuration = mem::zeroed();
    let mut mac_map: c_int;
    let mut egress_map: c_int;
    let mut res_map: c_int;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut xdp_dummy: *mut xdp_dummy;
    let mut err: c_int;
    let mut i: usize;

    xdp_dummy = xdp_dummy__open_and_load();
    if !ASSERT_OK_PTR(xdp_dummy, b"xdp_dummy__open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    xdp_redirect_multi_kern = xdp_redirect_multi_kern__open_and_load();
    if !ASSERT_OK_PTR(xdp_redirect_multi_kern, b"xdp_redirect_multi_kern__open_and_load\0".as_ptr() as *const c_char) {
        xdp_dummy__destroy(xdp_dummy);
        cleanup_network(&mut net_config);
        return;
    }

    xdp_redirect_map = xdp_redirect_map__open_and_load();
    if !ASSERT_OK_PTR(xdp_redirect_map, b"xdp_redirect_map__open_and_load\0".as_ptr() as *const c_char) {
        xdp_redirect_multi_kern__destroy(xdp_redirect_multi_kern);
        xdp_dummy__destroy(xdp_dummy);
        cleanup_network(&mut net_config);
        return;
    }

    if ASSERT_OK(create_network(&mut net_config), b"create network\0".as_ptr() as *const c_char) {
        mac_map = bpf_map__fd((*xdp_redirect_multi_kern).maps.mac_map);
        if ASSERT_OK_FD(mac_map, b"open mac_map\0".as_ptr() as *const c_char) {
            egress_map = bpf_map__fd((*xdp_redirect_multi_kern).maps.map_egress);
            if ASSERT_OK_FD(egress_map, b"open map_egress\0".as_ptr() as *const c_char) {
                devmap_val.bpf_prog.fd = bpf_program__fd((*xdp_redirect_multi_kern).progs.xdp_devmap_prog);

                bpf_objs[0] = (*xdp_dummy).obj;
                bpf_objs[1] = (*xdp_redirect_multi_kern).obj;
                bpf_objs[2] = (*xdp_redirect_map).obj;

                nstoken = open_netns(net_config.ns0_name.as_ptr());
                if ASSERT_OK_PTR(nstoken, b"open NS0\0".as_ptr() as *const c_char) {
                    i = 0;
                    while i < VETH_PAIRS_COUNT {
                        let ifindex: c_int = if_nametoindex(net_config.veth_cfg[i].local_veth.as_ptr()) as c_int;

                        if SYS(
                            b"destroy_xdp_redirect_map\0".as_ptr() as *const c_char,
                            b"ip -n %s neigh add %s lladdr 00:00:00:00:00:01 dev %s\0".as_ptr() as *const c_char,
                            net_config.veth_cfg[i].namespace.as_ptr(),
                            IP_NEIGH.as_ptr() as *const c_char,
                            net_config.veth_cfg[i].remote_veth.as_ptr(),
                        ) != 0 {
                            break;
                        }

                        if attach_programs_to_veth_pair(bpf_objs.as_mut_ptr(), VETH_REDIRECT_SKEL_NB, &mut net_config, prog_cfg.as_mut_ptr(), i as c_int) != 0 {
                            break;
                        }

                        {
                            let mut mac: __be64 = 0;

                            ptr::copy_nonoverlapping(egress_macs[i].as_ptr(), &mut mac as *mut _ as *mut u8, ETH_ALEN);
                            err = bpf_map_update_elem(mac_map, &ifindex as *const _ as *const c_void, &mac as *const _ as *const c_void, 0);
                        }

                        if !ASSERT_OK(err, b"bpf_map_update_elem\0".as_ptr() as *const c_char) {
                            break;
                        }

                        devmap_val.ifindex = ifindex as u32_t;
                        err = bpf_map_update_elem(egress_map, &ifindex as *const _ as *const c_void, &devmap_val as *const _ as *const c_void, 0);
                        if !ASSERT_OK(err, b"bpf_map_update_elem\0".as_ptr() as *const c_char) {
                            break;
                        }
                        i += 1;
                    }

                    if i == VETH_PAIRS_COUNT {
                        SYS_NOFAIL(
                            b"ip netns exec %s ping %s -i 0.1 -c 4 -W1 > /dev/null \0".as_ptr() as *const c_char,
                            net_config.veth_cfg[0].namespace.as_ptr(),
                            IP_NEIGH.as_ptr() as *const c_char,
                        );

                        res_map = bpf_map__fd((*xdp_redirect_map).maps.rx_mac);
                        if ASSERT_OK_FD(res_map, b"open rx_map\0".as_ptr() as *const c_char) {
                            i = 0;
                            while i < 2 {
                                let key: u32_t = i as u32_t;
                                let mut expected: __be64 = 0;
                                let mut res: u64_t = 0;

                                err = bpf_map_lookup_elem(res_map, &key as *const _ as *const c_void, &mut res as *mut _ as *mut c_void);
                                if !ASSERT_OK(err, b"get MAC res\0".as_ptr() as *const c_char) {
                                    break;
                                }

                                /* store_mac_1/2 run on the second/third remote veths. */
                                ptr::copy_nonoverlapping(egress_macs[i + 1].as_ptr(), &mut expected as *mut _ as *mut u8, ETH_ALEN);
                                ASSERT_EQ(res, expected, b"compare mac\0".as_ptr() as *const c_char);
                                i += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    close_netns(nstoken);
    xdp_redirect_map__destroy(xdp_redirect_map);
    xdp_redirect_multi_kern__destroy(xdp_redirect_multi_kern);
    xdp_dummy__destroy(xdp_dummy);

    cleanup_network(&mut net_config);
}

unsafe fn xdp_veth_egress_last_dst(flags: u32_t) {
    let mut prog_cfg: [prog_configuration; VETH_PAIRS_COUNT] = [
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_all_prog\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_dummy_prog\0"),
            local_flags: flags,
            remote_flags: flags,
        },
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_all_prog\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"store_mac_1\0"),
            local_flags: flags,
            remote_flags: flags,
        },
        prog_configuration {
            local_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_redirect_map_all_prog\0"),
            remote_name: cstr::<PROG_NAME_MAX_LEN>(b"xdp_dummy_prog\0"),
            local_flags: flags,
            remote_flags: flags,
        },
    ];
    let egress_macs: [[u8; ETH_ALEN]; VETH_PAIRS_COUNT] = [
        [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x01],
        [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x02],
        [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x03],
    ];
    let mut xdp_redirect_multi_kern: *mut xdp_redirect_multi_kern;
    let mut bpf_objs: [*mut bpf_object; VETH_EGRESS_SKEL_NB] = [ptr::null_mut(); VETH_EGRESS_SKEL_NB];
    let mut xdp_redirect_map: *mut xdp_redirect_map;
    let mut net_config: net_configuration = mem::zeroed();
    let mut mac_map: c_int;
    let mut egress_map: c_int;
    let mut res_map: c_int;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut xdp_dummy: *mut xdp_dummy;
    let mut sentinel_mac: __be64 = 0;
    let mut last_mac: __be64 = 0;
    let mut res: __be64 = 0;
    let mut key: u32_t;
    let mut err: c_int;
    let mut i: usize;

    xdp_dummy = xdp_dummy__open_and_load();
    if !ASSERT_OK_PTR(xdp_dummy, b"xdp_dummy__open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    xdp_redirect_multi_kern = xdp_redirect_multi_kern__open_and_load();
    if !ASSERT_OK_PTR(xdp_redirect_multi_kern, b"xdp_redirect_multi_kern__open_and_load\0".as_ptr() as *const c_char) {
        xdp_dummy__destroy(xdp_dummy);
        cleanup_network(&mut net_config);
        return;
    }

    xdp_redirect_map = xdp_redirect_map__open_and_load();
    if !ASSERT_OK_PTR(xdp_redirect_map, b"xdp_redirect_map__open_and_load\0".as_ptr() as *const c_char) {
        xdp_redirect_multi_kern__destroy(xdp_redirect_multi_kern);
        xdp_dummy__destroy(xdp_dummy);
        cleanup_network(&mut net_config);
        return;
    }

    if ASSERT_OK(create_network(&mut net_config), b"create network\0".as_ptr() as *const c_char) {
        mac_map = bpf_map__fd((*xdp_redirect_multi_kern).maps.mac_map);
        if ASSERT_OK_FD(mac_map, b"open mac_map\0".as_ptr() as *const c_char) {
            egress_map = bpf_map__fd((*xdp_redirect_multi_kern).maps.map_egress);
            if ASSERT_OK_FD(egress_map, b"open map_egress\0".as_ptr() as *const c_char) {
                bpf_objs[0] = (*xdp_dummy).obj;
                bpf_objs[1] = (*xdp_redirect_multi_kern).obj;
                bpf_objs[2] = (*xdp_redirect_map).obj;

                nstoken = open_netns(net_config.ns0_name.as_ptr());
                if ASSERT_OK_PTR(nstoken, b"open NS0\0".as_ptr() as *const c_char) {
                    i = 0;
                    while i < VETH_PAIRS_COUNT {
                        let mut devmap_val: bpf_devmap_val = mem::zeroed();
                        let ifindex: c_int = if_nametoindex(net_config.veth_cfg[i].local_veth.as_ptr()) as c_int;
                        let map_key: u32_t = i as u32_t;

                        if SYS(
                            b"destroy_xdp_redirect_map\0".as_ptr() as *const c_char,
                            b"ip -n %s neigh add %s lladdr 00:00:00:00:00:01 dev %s\0".as_ptr() as *const c_char,
                            net_config.veth_cfg[i].namespace.as_ptr(),
                            IP_NEIGH.as_ptr() as *const c_char,
                            net_config.veth_cfg[i].remote_veth.as_ptr(),
                        ) != 0 {
                            break;
                        }

                        if attach_programs_to_veth_pair(bpf_objs.as_mut_ptr(), VETH_EGRESS_SKEL_NB, &mut net_config, prog_cfg.as_mut_ptr(), i as c_int) != 0 {
                            break;
                        }

                        {
                            let mut mac: __be64 = 0;

                            ptr::copy_nonoverlapping(egress_macs[i].as_ptr(), &mut mac as *mut _ as *mut u8, ETH_ALEN);
                            err = bpf_map_update_elem(mac_map, &ifindex as *const _ as *const c_void, &mac as *const _ as *const c_void, 0);
                        }

                        if !ASSERT_OK(err, b"bpf_map_update_elem\0".as_ptr() as *const c_char) {
                            break;
                        }

                        devmap_val.ifindex = ifindex as u32_t;
                        devmap_val.bpf_prog.fd = -1;

                        if i == VETH_PAIRS_COUNT - 1 {
                            devmap_val.bpf_prog.fd = bpf_program__fd((*xdp_redirect_multi_kern).progs.xdp_devmap_prog);
                        }

                        err = bpf_map_update_elem(egress_map, &map_key as *const _ as *const c_void, &devmap_val as *const _ as *const c_void, 0);
                        if !ASSERT_OK(err, b"bpf_map_update_elem\0".as_ptr() as *const c_char) {
                            break;
                        }
                        i += 1;
                    }

                    if i == VETH_PAIRS_COUNT {
                        res_map = bpf_map__fd((*xdp_redirect_map).maps.rx_mac);
                        if ASSERT_OK_FD(res_map, b"open rx_map\0".as_ptr() as *const c_char) {
                            ptr::copy_nonoverlapping(egress_macs[VETH_PAIRS_COUNT - 1].as_ptr(), &mut sentinel_mac as *mut _ as *mut u8, ETH_ALEN);
                            ptr::copy_nonoverlapping(egress_macs[VETH_PAIRS_COUNT - 1].as_ptr(), &mut last_mac as *mut _ as *mut u8, ETH_ALEN);

                            key = 0;
                            err = bpf_map_update_elem(res_map, &key as *const _ as *const c_void, &sentinel_mac as *const _ as *const c_void, 0);
                            if ASSERT_OK(err, b"init rx mac\0".as_ptr() as *const c_char) {
                                SYS_NOFAIL(
                                    b"ip netns exec %s ping %s -i 0.1 -c 4 -W1 > /dev/null \0".as_ptr() as *const c_char,
                                    net_config.veth_cfg[0].namespace.as_ptr(),
                                    IP_NEIGH.as_ptr() as *const c_char,
                                );

                                err = bpf_map_lookup_elem(res_map, &key as *const _ as *const c_void, &mut res as *mut _ as *mut c_void);
                                if ASSERT_OK(err, b"get MAC res\0".as_ptr() as *const c_char) {
                                    if ASSERT_NEQ(res, sentinel_mac, b"rx_mac overwritten by store_mac_1\0".as_ptr() as *const c_char) {
                                        ASSERT_NEQ(res, last_mac, b"earlier dst not rewritten by last dst\0".as_ptr() as *const c_char);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    close_netns(nstoken);
    xdp_redirect_map__destroy(xdp_redirect_map);
    xdp_redirect_multi_kern__destroy(xdp_redirect_multi_kern);
    xdp_dummy__destroy(xdp_dummy);

    cleanup_network(&mut net_config);
}

#[no_mangle]
pub unsafe extern "C" fn test_xdp_veth_redirect() {
    if test__start_subtest(b"0\0".as_ptr() as *const c_char) {
        xdp_veth_redirect(0);
    }

    if test__start_subtest(b"DRV_MODE\0".as_ptr() as *const c_char) {
        xdp_veth_redirect(XDP_FLAGS_DRV_MODE);
    }

    if test__start_subtest(b"SKB_MODE\0".as_ptr() as *const c_char) {
        xdp_veth_redirect(XDP_FLAGS_SKB_MODE);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_xdp_veth_broadcast_redirect() {
    if test__start_subtest(b"0/BROADCAST\0".as_ptr() as *const c_char) {
        xdp_veth_broadcast_redirect(0, BPF_F_BROADCAST);
    }

    if test__start_subtest(b"0/(BROADCAST | EXCLUDE_INGRESS)\0".as_ptr() as *const c_char) {
        xdp_veth_broadcast_redirect(0, BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS);
    }

    if test__start_subtest(b"DRV_MODE/BROADCAST\0".as_ptr() as *const c_char) {
        xdp_veth_broadcast_redirect(XDP_FLAGS_DRV_MODE, BPF_F_BROADCAST);
    }

    if test__start_subtest(b"DRV_MODE/(BROADCAST | EXCLUDE_INGRESS)\0".as_ptr() as *const c_char) {
        xdp_veth_broadcast_redirect(XDP_FLAGS_DRV_MODE, BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS);
    }

    if test__start_subtest(b"SKB_MODE/BROADCAST\0".as_ptr() as *const c_char) {
        xdp_veth_broadcast_redirect(XDP_FLAGS_SKB_MODE, BPF_F_BROADCAST);
    }

    if test__start_subtest(b"SKB_MODE/(BROADCAST | EXCLUDE_INGRESS)\0".as_ptr() as *const c_char) {
        xdp_veth_broadcast_redirect(XDP_FLAGS_SKB_MODE, BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_xdp_veth_egress() {
    if test__start_subtest(b"0/egress\0".as_ptr() as *const c_char) {
        xdp_veth_egress(0);
    }

    if test__start_subtest(b"DRV_MODE/egress\0".as_ptr() as *const c_char) {
        xdp_veth_egress(XDP_FLAGS_DRV_MODE);
    }

    if test__start_subtest(b"SKB_MODE/egress\0".as_ptr() as *const c_char) {
        xdp_veth_egress(XDP_FLAGS_SKB_MODE);
    }

    if test__start_subtest(b"SKB_MODE/egress_last_dst\0".as_ptr() as *const c_char) {
        xdp_veth_egress_last_dst(XDP_FLAGS_SKB_MODE);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
