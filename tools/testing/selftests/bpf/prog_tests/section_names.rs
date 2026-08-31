// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook
// Depends on declarations originally provided by <test_progs.h>.

use core::ffi::{c_char, c_int};

static mut duration: c_int = 0;

#[repr(C)]
struct sec_name_test {
    sec_name: [c_char; 32],
    expected_load: sec_name_test_expected_load,
    expected_attach: sec_name_test_expected_attach,
}

#[repr(C)]
struct sec_name_test_expected_load {
    rc: c_int,
    prog_type: bpf_prog_type,
    expected_attach_type: bpf_attach_type,
}

#[repr(C)]
struct sec_name_test_expected_attach {
    rc: c_int,
    attach_type: bpf_attach_type,
}

const fn sec_name<const N: usize>(s: &[u8; N]) -> [c_char; 32] {
    let mut out = [0 as c_char; 32];
    let mut i = 0;

    while i < N {
        out[i] = s[i] as c_char;
        i += 1;
    }

    out
}

static mut tests: [sec_name_test; 52] = [
    sec_name_test {
        sec_name: sec_name(b"InvAliD"),
        expected_load: sec_name_test_expected_load {
            rc: -ESRCH,
            prog_type: 0 as bpf_prog_type,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup"),
        expected_load: sec_name_test_expected_load {
            rc: -ESRCH,
            prog_type: 0 as bpf_prog_type,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"socket"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_SOCKET_FILTER,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"kprobe/"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_KPROBE,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"uprobe/"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_KPROBE,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"kretprobe/"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_KPROBE,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"uretprobe/"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_KPROBE,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"classifier"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_SCHED_CLS,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"action"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_SCHED_ACT,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"tracepoint/"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_TRACEPOINT,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"tp/"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_TRACEPOINT,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"raw_tracepoint/"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_RAW_TRACEPOINT,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"raw_tp/"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_RAW_TRACEPOINT,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"xdp"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_XDP,
            expected_attach_type: BPF_XDP,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_XDP,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"perf_event"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_PERF_EVENT,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"lwt_in"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_LWT_IN,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"lwt_out"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_LWT_OUT,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"lwt_xmit"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_LWT_XMIT,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"lwt_seg6local"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_LWT_SEG6LOCAL,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup_skb/ingress"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SKB,
            expected_attach_type: BPF_CGROUP_INET_INGRESS,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET_INGRESS,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup_skb/egress"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SKB,
            expected_attach_type: BPF_CGROUP_INET_EGRESS,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET_EGRESS,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/skb"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SKB,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/sock"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK,
            expected_attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET_SOCK_CREATE,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/post_bind4"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK,
            expected_attach_type: BPF_CGROUP_INET4_POST_BIND,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET4_POST_BIND,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/post_bind6"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK,
            expected_attach_type: BPF_CGROUP_INET6_POST_BIND,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET6_POST_BIND,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/dev"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_DEVICE,
            expected_attach_type: BPF_CGROUP_DEVICE,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_DEVICE,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"sockops"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_SOCK_OPS,
            expected_attach_type: BPF_CGROUP_SOCK_OPS,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_SOCK_OPS,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"sk_skb/stream_parser"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_SK_SKB,
            expected_attach_type: BPF_SK_SKB_STREAM_PARSER,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_SK_SKB_STREAM_PARSER,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"sk_skb/stream_verdict"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_SK_SKB,
            expected_attach_type: BPF_SK_SKB_STREAM_VERDICT,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_SK_SKB_STREAM_VERDICT,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"sk_skb"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_SK_SKB,
            expected_attach_type: 0 as bpf_attach_type,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: -EINVAL,
            attach_type: 0 as bpf_attach_type,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"sk_msg"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_SK_MSG,
            expected_attach_type: BPF_SK_MSG_VERDICT,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_SK_MSG_VERDICT,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"lirc_mode2"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_LIRC_MODE2,
            expected_attach_type: BPF_LIRC_MODE2,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_LIRC_MODE2,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"flow_dissector"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_FLOW_DISSECTOR,
            expected_attach_type: BPF_FLOW_DISSECTOR,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_FLOW_DISSECTOR,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/bind4"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_INET4_BIND,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET4_BIND,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/bind6"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_INET6_BIND,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET6_BIND,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/connect4"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_INET4_CONNECT,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET4_CONNECT,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/connect6"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_INET6_CONNECT,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET6_CONNECT,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/connect_unix"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_UNIX_CONNECT,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_UNIX_CONNECT,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/sendmsg4"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_UDP4_SENDMSG,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_UDP4_SENDMSG,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/sendmsg6"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_UDP6_SENDMSG,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_UDP6_SENDMSG,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/sendmsg_unix"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_UNIX_SENDMSG,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_UNIX_SENDMSG,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/recvmsg4"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_UDP4_RECVMSG,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_UDP4_RECVMSG,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/recvmsg6"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_UDP6_RECVMSG,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_UDP6_RECVMSG,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/recvmsg_unix"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_UNIX_RECVMSG,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_UNIX_RECVMSG,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/sysctl"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SYSCTL,
            expected_attach_type: BPF_CGROUP_SYSCTL,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_SYSCTL,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/getsockopt"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCKOPT,
            expected_attach_type: BPF_CGROUP_GETSOCKOPT,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_GETSOCKOPT,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/setsockopt"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCKOPT,
            expected_attach_type: BPF_CGROUP_SETSOCKOPT,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_SETSOCKOPT,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/getpeername4"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_INET4_GETPEERNAME,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET4_GETPEERNAME,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/getpeername6"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_INET6_GETPEERNAME,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET6_GETPEERNAME,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/getpeername_unix"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_UNIX_GETPEERNAME,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_UNIX_GETPEERNAME,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/getsockname4"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_INET4_GETSOCKNAME,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET4_GETSOCKNAME,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/getsockname6"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_INET6_GETSOCKNAME,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_INET6_GETSOCKNAME,
        },
    },
    sec_name_test {
        sec_name: sec_name(b"cgroup/getsockname_unix"),
        expected_load: sec_name_test_expected_load {
            rc: 0,
            prog_type: BPF_PROG_TYPE_CGROUP_SOCK_ADDR,
            expected_attach_type: BPF_CGROUP_UNIX_GETSOCKNAME,
        },
        expected_attach: sec_name_test_expected_attach {
            rc: 0,
            attach_type: BPF_CGROUP_UNIX_GETSOCKNAME,
        },
    },
];

unsafe fn test_prog_type_by_name(test: *const sec_name_test) {
    let mut expected_attach_type: bpf_attach_type = 0 as bpf_attach_type;
    let mut prog_type: bpf_prog_type = 0 as bpf_prog_type;
    let mut rc: c_int;

    rc = libbpf_prog_type_by_name(
        (*test).sec_name.as_ptr(),
        &mut prog_type,
        &mut expected_attach_type,
    );

    CHECK!(
        rc != (*test).expected_load.rc,
        "check_code",
        "prog: unexpected rc=%d for %s\n",
        rc,
        (*test).sec_name.as_ptr()
    );

    if rc != 0 {
        return;
    }

    CHECK!(
        prog_type != (*test).expected_load.prog_type,
        "check_prog_type",
        "prog: unexpected prog_type=%d for %s\n",
        prog_type,
        (*test).sec_name.as_ptr()
    );

    CHECK!(
        expected_attach_type != (*test).expected_load.expected_attach_type,
        "check_attach_type",
        "prog: unexpected expected_attach_type=%d for %s\n",
        expected_attach_type,
        (*test).sec_name.as_ptr()
    );
}

unsafe fn test_attach_type_by_name(test: *const sec_name_test) {
    let mut attach_type: bpf_attach_type = 0 as bpf_attach_type;
    let mut rc: c_int;

    rc = libbpf_attach_type_by_name((*test).sec_name.as_ptr(), &mut attach_type);

    CHECK!(
        rc != (*test).expected_attach.rc,
        "check_ret",
        "attach: unexpected rc=%d for %s\n",
        rc,
        (*test).sec_name.as_ptr()
    );

    if rc != 0 {
        return;
    }

    CHECK!(
        attach_type != (*test).expected_attach.attach_type,
        "check_attach_type",
        "attach: unexpected attach_type=%d for %s\n",
        attach_type,
        (*test).sec_name.as_ptr()
    );
}

pub unsafe extern "C" fn test_section_names() {
    let mut i: c_int;

    i = 0;
    while (i as usize) < tests.len() {
        let test: *mut sec_name_test = tests.as_mut_ptr().add(i as usize);

        test_prog_type_by_name(test);
        test_attach_type_by_name(test);

        i += 1;
    }
}
