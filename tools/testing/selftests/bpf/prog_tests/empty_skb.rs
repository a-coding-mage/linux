// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external Rust dependencies:
// <test_progs.h>, <network_helpers.h>, <net/if.h>, "empty_skb.skel.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

type __u32 = u32;
type __be16 = u16;

const ETH_P_IP: c_int = 0x0800;
const ETH_P_IPV6: c_int = 0x86DD;
const EINVAL: c_int = 22;
const ERANGE: c_int = 34;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: __u32,
    pub data_size_out: __u32,
}

#[repr(C)]
pub struct empty_skb_bss {
    pub ifindex: c_int,
    pub ret: c_int,
}

#[repr(C)]
pub struct empty_skb {
    pub obj: *mut bpf_object,
    pub bss: *mut empty_skb_bss,
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
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: __be16,
}

#[repr(C)]
struct Test {
    msg: *const c_char,
    data_in: *const c_void,
    data_size_in: __u32,
    ifindex: *mut c_int,
    err: c_int,
    ret: c_int,
    lwt_egress_ret: c_int, /* expected retval at lwt/egress */
    h_proto: __be16,
    success_on_tc: bool,
    adjust_room: bool,
}

extern "C" {
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(tok: *mut nstoken);
    fn if_nametoindex(ifname: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn htons(hostshort: u16) -> u16;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;

    fn empty_skb__open_and_load() -> *mut empty_skb;
    fn empty_skb__destroy(obj: *mut empty_skb);
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_program__section_name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn SYS(label: *const c_char, cmd: *const c_char);
    fn SYS_NOFAIL(cmd: *const c_char);
}

// Rust translation placeholder for libbpf's bpf_object__for_each_program macro.
extern "C" {
    fn bpf_object__next_program(obj: *mut bpf_object, prev: *mut bpf_program) -> *mut bpf_program;
}

pub unsafe fn test_empty_skb() {
    // LIBBPF_OPTS(bpf_test_run_opts, tattr);
    let mut tattr: bpf_test_run_opts = mem::zeroed();
    let mut bpf_obj: *mut empty_skb = ptr::null_mut();
    let mut tok: *mut nstoken = ptr::null_mut();
    let mut prog: *mut bpf_program;
    let mut eth_hlen: ethhdr = mem::zeroed();
    let mut eth_hlen_pp: [c_char; 15] = [0; 15];
    let mut veth_ifindex: c_int = 0;
    let mut ipip_ifindex: c_int = 0;
    let mut err: c_int;
    let mut i: c_int;

    let mut tests: [Test; 8] = [
        /* Empty packets are always rejected. */
        Test {
            /* BPF_PROG_RUN ETH_HLEN size check */
            msg: b"veth empty ingress packet\0".as_ptr() as *const c_char,
            data_in: ptr::null(),
            data_size_in: 0,
            ifindex: &mut veth_ifindex,
            err: -EINVAL,
            ret: 0,
            lwt_egress_ret: 0,
            h_proto: 0,
            success_on_tc: false,
            adjust_room: false,
        },
        Test {
            /* BPF_PROG_RUN ETH_HLEN size check */
            msg: b"ipip empty ingress packet\0".as_ptr() as *const c_char,
            data_in: ptr::null(),
            data_size_in: 0,
            ifindex: &mut ipip_ifindex,
            err: -EINVAL,
            ret: 0,
            lwt_egress_ret: 0,
            h_proto: 0,
            success_on_tc: false,
            adjust_room: false,
        },
        /* ETH_HLEN-sized packets with IPv4/IPv6 EtherType but
         * no L3 header are rejected.
         */
        Test {
            msg: b"veth short IPv4 ingress packet\0".as_ptr() as *const c_char,
            data_in: &eth_hlen as *const ethhdr as *const c_void,
            data_size_in: mem::size_of::<ethhdr>() as __u32,
            ifindex: &mut veth_ifindex,
            err: -EINVAL,
            ret: 0,
            lwt_egress_ret: 0,
            h_proto: htons(ETH_P_IP as u16),
            success_on_tc: false,
            adjust_room: true,
        },
        Test {
            msg: b"veth short IPv6 ingress packet\0".as_ptr() as *const c_char,
            data_in: &eth_hlen as *const ethhdr as *const c_void,
            data_size_in: mem::size_of::<ethhdr>() as __u32,
            ifindex: &mut veth_ifindex,
            err: -EINVAL,
            ret: 0,
            lwt_egress_ret: 0,
            h_proto: htons(ETH_P_IPV6 as u16),
            success_on_tc: false,
            adjust_room: true,
        },
        /* ETH_HLEN-sized packets:
         * - can not be redirected at LWT_XMIT
         * - can be redirected at TC to non-tunneling dest
         */
        Test {
            /* __bpf_redirect_common */
            msg: b"veth ETH_HLEN packet ingress\0".as_ptr() as *const c_char,
            data_in: &eth_hlen as *const ethhdr as *const c_void,
            data_size_in: mem::size_of::<ethhdr>() as __u32,
            ifindex: &mut veth_ifindex,
            err: 0,
            ret: -ERANGE,
            lwt_egress_ret: -ERANGE,
            h_proto: 0,
            success_on_tc: true,
            adjust_room: false,
        },
        Test {
            /* __bpf_redirect_no_mac
             *
             * lwt: skb->len=0 <= skb_network_offset=0
             * tc: skb->len=14 <= skb_network_offset=14
             */
            msg: b"ipip ETH_HLEN packet ingress\0".as_ptr() as *const c_char,
            data_in: &eth_hlen as *const ethhdr as *const c_void,
            data_size_in: mem::size_of::<ethhdr>() as __u32,
            ifindex: &mut ipip_ifindex,
            err: 0,
            ret: -ERANGE,
            lwt_egress_ret: -ERANGE,
            h_proto: 0,
            success_on_tc: false,
            adjust_room: false,
        },
        /* ETH_HLEN+1-sized packet should be redirected. */
        Test {
            msg: b"veth ETH_HLEN+1 packet ingress\0".as_ptr() as *const c_char,
            data_in: eth_hlen_pp.as_ptr() as *const c_void,
            data_size_in: mem::size_of_val(&eth_hlen_pp) as __u32,
            ifindex: &mut veth_ifindex,
            err: 0,
            ret: 0,
            lwt_egress_ret: 1, /* veth_xmit NET_XMIT_DROP */
            h_proto: 0,
            success_on_tc: false,
            adjust_room: false,
        },
        Test {
            msg: b"ipip ETH_HLEN+1 packet ingress\0".as_ptr() as *const c_char,
            data_in: eth_hlen_pp.as_ptr() as *const c_void,
            data_size_in: mem::size_of_val(&eth_hlen_pp) as __u32,
            ifindex: &mut ipip_ifindex,
            err: 0,
            ret: 0,
            lwt_egress_ret: 0,
            h_proto: 0,
            success_on_tc: false,
            adjust_room: false,
        },
    ];

    SYS(b"out\0".as_ptr() as *const c_char, b"ip netns add empty_skb\0".as_ptr() as *const c_char);
    tok = open_netns(b"empty_skb\0".as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(tok as *const c_void, b"setns\0".as_ptr() as *const c_char) {
        goto_out(bpf_obj, tok);
        return;
    }
    SYS(b"out\0".as_ptr() as *const c_char, b"ip link add veth0 type veth peer veth1\0".as_ptr() as *const c_char);
    SYS(b"out\0".as_ptr() as *const c_char, b"ip link set dev veth0 up\0".as_ptr() as *const c_char);
    SYS(b"out\0".as_ptr() as *const c_char, b"ip link set dev veth1 up\0".as_ptr() as *const c_char);
    SYS(b"out\0".as_ptr() as *const c_char, b"ip addr add 10.0.0.1/8 dev veth0\0".as_ptr() as *const c_char);
    SYS(b"out\0".as_ptr() as *const c_char, b"ip addr add 10.0.0.2/8 dev veth1\0".as_ptr() as *const c_char);
    veth_ifindex = if_nametoindex(b"veth0\0".as_ptr() as *const c_char);

    SYS(b"out\0".as_ptr() as *const c_char, b"ip link add ipip0 type ipip local 10.0.0.1 remote 10.0.0.2\0".as_ptr() as *const c_char);
    SYS(b"out\0".as_ptr() as *const c_char, b"ip link set ipip0 up\0".as_ptr() as *const c_char);
    SYS(b"out\0".as_ptr() as *const c_char, b"ip addr add 192.168.1.1/16 dev ipip0\0".as_ptr() as *const c_char);
    ipip_ifindex = if_nametoindex(b"ipip0\0".as_ptr() as *const c_char);

    memset(
        eth_hlen_pp.as_mut_ptr() as *mut c_void,
        0,
        mem::size_of_val(&eth_hlen_pp),
    );
    memset(
        &mut eth_hlen as *mut ethhdr as *mut c_void,
        0,
        mem::size_of_val(&eth_hlen),
    );

    bpf_obj = empty_skb__open_and_load();
    if !ASSERT_OK_PTR(bpf_obj as *const c_void, b"open skeleton\0".as_ptr() as *const c_char) {
        goto_out(bpf_obj, tok);
        return;
    }

    i = 0;
    while i < tests.len() as c_int {
        if tests[i as usize].data_in == (&eth_hlen as *const ethhdr as *const c_void) {
            eth_hlen.h_proto = tests[i as usize].h_proto;
        }

        // bpf_object__for_each_program(prog, bpf_obj->obj)
        prog = ptr::null_mut();
        loop {
            prog = bpf_object__next_program((*bpf_obj).obj, prog);
            if prog.is_null() {
                break;
            }

            let at_egress: bool =
                !strstr(bpf_program__name(prog), b"egress\0".as_ptr() as *const c_char).is_null();
            let at_tc: bool =
                strncmp(bpf_program__section_name(prog), b"tc\0".as_ptr() as *const c_char, 2) == 0;
            let is_adjust_room: bool =
                strcmp(bpf_program__name(prog), b"tc_adjust_room\0".as_ptr() as *const c_char) == 0;
            let expected_ret: c_int;
            let mut buf: [c_char; 128] = [0; 128];

            if tests[i as usize].adjust_room != is_adjust_room {
                continue;
            }

            expected_ret = if at_egress && !at_tc {
                tests[i as usize].lwt_egress_ret
            } else {
                tests[i as usize].ret
            };

            tattr.data_in = tests[i as usize].data_in;
            tattr.data_size_in = tests[i as usize].data_size_in;

            tattr.data_size_out = 0;
            (*(*bpf_obj).bss).ifindex = *tests[i as usize].ifindex;
            (*(*bpf_obj).bss).ret = 0;
            err = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut tattr);
            sprintf(
                buf.as_mut_ptr(),
                b"err: %s [%s]\0".as_ptr() as *const c_char,
                tests[i as usize].msg,
                bpf_program__name(prog),
            );

            if at_tc && tests[i as usize].success_on_tc {
                ASSERT_GE(err, 0, buf.as_ptr());
            } else {
                ASSERT_EQ(err, tests[i as usize].err, buf.as_ptr());
            }
            sprintf(
                buf.as_mut_ptr(),
                b"ret: %s [%s]\0".as_ptr() as *const c_char,
                tests[i as usize].msg,
                bpf_program__name(prog),
            );
            if at_tc && tests[i as usize].success_on_tc {
                ASSERT_GE((*(*bpf_obj).bss).ret, 0, buf.as_ptr());
            } else {
                ASSERT_EQ((*(*bpf_obj).bss).ret, expected_ret, buf.as_ptr());
            }
        }

        i += 1;
    }

    goto_out(bpf_obj, tok);
}

unsafe fn goto_out(bpf_obj: *mut empty_skb, tok: *mut nstoken) {
    if !bpf_obj.is_null() {
        empty_skb__destroy(bpf_obj);
    }
    if !tok.is_null() {
        close_netns(tok);
    }
    SYS_NOFAIL(b"ip netns del empty_skb\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
