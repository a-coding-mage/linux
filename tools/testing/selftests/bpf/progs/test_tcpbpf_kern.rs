// SPDX-License-Identifier: GPL-2.0
// Translated from test_tcpbpf_kern.c.
// Original dependencies:
//   "bpf_tracing_net.h"
//   <bpf/bpf_helpers.h>
//   <bpf/bpf_endian.h>
//   "test_tcpbpf.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::mem::size_of;

extern "C" {
    static mut global: tcpbpf_globals;

    fn bpf_skc_to_tcp_sock(sk: *mut bpf_sock) -> *mut tcp_sock;
    fn bpf_setsockopt(
        skops: *mut bpf_sock_ops,
        level: i32,
        optname: i32,
        optval: *const core::ffi::c_void,
        optlen: i32,
    ) -> i32;
    fn bpf_getsockopt(
        skops: *mut bpf_sock_ops,
        level: i32,
        optname: i32,
        optval: *mut core::ffi::c_void,
        optlen: i32,
    ) -> i32;
    fn bpf_sock_ops_cb_flags_set(skops: *mut bpf_sock_ops, argval: i32) -> i32;
}

// SOL_TCP is defined in <netinet/tcp.h> while TCP_SAVED_SYN is defined in
// already included <linux/tcp.h>. Keep the local fallback value from C.
pub const SOL_TCP: i32 = 6;

#[inline(always)]
unsafe fn get_tp_window_clamp(skops: *mut bpf_sock_ops) -> i32 {
    let sk: *mut bpf_sock;
    let tp: *mut tcp_sock;

    sk = (*skops).sk;
    if sk.is_null() {
        return -1;
    }
    tp = bpf_skc_to_tcp_sock(sk);
    if tp.is_null() {
        return -1;
    }
    (*tp).window_clamp as i32
}

// SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn bpf_testcb(skops: *mut bpf_sock_ops) -> i32 {
    let mut header = [0u8; size_of::<ipv6hdr>() + size_of::<tcphdr>()];
    let mut reuse: *mut bpf_sock_ops = skops;
    let mut thdr: *mut tcphdr;
    let mut window_clamp: i32 = 9216;
    let mut save_syn: i32 = 1;
    let mut rv: i32 = -1;
    let mut v: i32 = 0;
    let mut op: i32;

    // Test reading fields in bpf_sock_ops using single register.
    asm!(
        "{reuse:e} = *(u32 *)({reuse:e} +96)",
        reuse = inout(reg) reuse,
        options(nostack, preserves_flags),
    );

    asm!(
        "{op:e} = *(u32 *)({skops:e} +96)",
        op = out(reg) op,
        skops = in(reg) skops,
        options(nostack, preserves_flags),
    );

    asm!(
        "r9 = {skops};",
        "r8 = *(u32 *)(r9 +164);",
        "*(u32 *)(r9 +164) = r8;",
        skops = in(reg) skops,
        lateout("r9") _,
        lateout("r8") _,
        options(nostack, preserves_flags),
    );

    asm!(
        "r1 = {skops};",
        "r1 = *(u64 *)(r1 +184);",
        "if r1 == 0 goto +1;",
        "r1 = *(u32 *)(r1 +4);",
        skops = in(reg) skops,
        lateout("r1") _,
        options(nostack, preserves_flags),
    );

    asm!(
        "r9 = {skops};",
        "r9 = *(u64 *)(r9 +184);",
        "if r9 == 0 goto +1;",
        "r9 = *(u32 *)(r9 +4);",
        skops = in(reg) skops,
        lateout("r9") _,
        options(nostack, preserves_flags),
    );

    asm!(
        "r1 = {skops};",
        "r2 = *(u64 *)(r1 +184);",
        "if r2 == 0 goto +1;",
        "r2 = *(u32 *)(r2 +4);",
        skops = in(reg) skops,
        lateout("r1") _,
        lateout("r2") _,
        options(nostack, preserves_flags),
    );

    op = (*skops).op as i32;

    global.event_map |= 1 << op;

    match op {
        BPF_SOCK_OPS_TCP_CONNECT_CB => {
            rv = bpf_setsockopt(
                skops,
                SOL_TCP,
                TCP_WINDOW_CLAMP,
                (&mut window_clamp as *mut i32).cast(),
                size_of::<i32>() as i32,
            );
            global.window_clamp_client = get_tp_window_clamp(skops);
        }
        BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB => {
            // Test failure to set largest cb flag (assumes not defined).
            global.bad_cb_test_rv = bpf_sock_ops_cb_flags_set(skops, 0x80);
            // Set callback.
            global.good_cb_test_rv =
                bpf_sock_ops_cb_flags_set(skops, BPF_SOCK_OPS_STATE_CB_FLAG);
        }
        BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
            (*skops).sk_txhash = 0x12345f;
            v = 0xff;
            rv = bpf_setsockopt(
                skops,
                SOL_IPV6,
                IPV6_TCLASS,
                (&mut v as *mut i32).cast(),
                size_of::<i32>() as i32,
            );
            if (*skops).family == AF_INET6 {
                v = bpf_getsockopt(
                    skops,
                    IPPROTO_TCP,
                    TCP_SAVED_SYN,
                    header.as_mut_ptr().cast(),
                    (size_of::<ipv6hdr>() + size_of::<tcphdr>()) as i32,
                );
                if v == 0 {
                    let offset = size_of::<ipv6hdr>();

                    thdr = header.as_mut_ptr().add(offset).cast::<tcphdr>();
                    v = (*thdr).syn as i32;

                    global.tcp_saved_syn = v;
                }
            }
            rv = bpf_setsockopt(
                skops,
                SOL_TCP,
                TCP_WINDOW_CLAMP,
                (&mut window_clamp as *mut i32).cast(),
                size_of::<i32>() as i32,
            );

            global.window_clamp_server = get_tp_window_clamp(skops);
        }
        BPF_SOCK_OPS_RTO_CB => {}
        BPF_SOCK_OPS_RETRANS_CB => {}
        BPF_SOCK_OPS_STATE_CB => {
            if (*skops).args[1] == BPF_TCP_CLOSE {
                if (*skops).args[0] == BPF_TCP_LISTEN {
                    global.num_listen += 1;
                } else {
                    global.total_retrans = (*skops).total_retrans;
                    global.data_segs_in = (*skops).data_segs_in;
                    global.data_segs_out = (*skops).data_segs_out;
                    global.bytes_received = (*skops).bytes_received;
                    global.bytes_acked = (*skops).bytes_acked;
                }
                global.num_close_events += 1;
            }
        }
        BPF_SOCK_OPS_TCP_LISTEN_CB => {
            bpf_sock_ops_cb_flags_set(skops, BPF_SOCK_OPS_STATE_CB_FLAG);
            v = bpf_setsockopt(
                skops,
                IPPROTO_TCP,
                TCP_SAVE_SYN,
                (&mut save_syn as *mut i32).cast(),
                size_of::<i32>() as i32,
            );
            // Update global map w/ result of setsock opt.
            global.tcp_save_syn = v;
        }
        _ => {
            rv = -1;
        }
    }
    (*skops).reply = rv;
    1
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
