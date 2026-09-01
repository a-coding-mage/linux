// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: <vmlinux.h>, "bpf_tracing_net.h", <bpf/bpf_helpers.h>,
// and <bpf/bpf_endian.h>.

use core::mem::{offset_of, size_of};
use core::ptr::{addr_of, null_mut};

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn BPF_SEQ_PRINTF(seq: *mut seq_file, fmt: *const core::ffi::c_char, ...);
    fn bpf_ntohs(x: __be16) -> __u16;
}

static unsafe fn sock_i_ino(sk: *const sock) -> i64 {
    let sk_socket: *const socket = unsafe { (*sk).sk_socket };
    let inode: *const inode;
    let mut ino: core::ffi::c_ulong;

    if sk_socket.is_null() {
        return 0;
    }

    inode = unsafe {
        (sk_socket as *const u8)
            .offset(-(offset_of!(socket_alloc, socket) as isize))
            .cast::<socket_alloc>()
            .cast::<u8>()
            .add(offset_of!(socket_alloc, vfs_inode))
            .cast::<inode>()
    };
    unsafe {
        bpf_probe_read_kernel(
            addr_of!(ino).cast_mut().cast::<core::ffi::c_void>(),
            size_of::<core::ffi::c_ulong>() as u32,
            addr_of!((*inode).i_ino).cast::<core::ffi::c_void>(),
        );
    }
    ino as i64
}

#[unsafe(link_section = "iter/udp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_udp4(ctx: *mut bpf_iter__udp) -> i32 {
    let seq: *mut seq_file = unsafe { (*(*ctx).meta).seq };
    let udp_sk: *mut udp_sock = unsafe { (*ctx).udp_sk };
    let mut inet: *mut inet_sock;
    let srcp: __u16;
    let destp: __u16;
    let dest: __be32;
    let src: __be32;
    let seq_num: __u32;
    let rqueue: i32;

    if udp_sk == null_mut() {
        return 0;
    }

    seq_num = unsafe { (*(*ctx).meta).seq_num };
    if seq_num == 0 {
        unsafe {
            BPF_SEQ_PRINTF(
                seq,
                c"  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops\n"
                    .as_ptr(),
            );
        }
    }

    /* filter out udp6 sockets */
    inet = unsafe { addr_of!((*udp_sk).inet).cast_mut() };
    if unsafe { (*addr_of!((*inet).sk)).sk_family } == AF_INET6 {
        return 0;
    }

    inet = unsafe { addr_of!((*udp_sk).inet).cast_mut() };
    dest = unsafe { (*inet).inet_daddr };
    src = unsafe { (*inet).inet_rcv_saddr };
    srcp = unsafe { bpf_ntohs((*inet).inet_sport) };
    destp = unsafe { bpf_ntohs((*inet).inet_dport) };
    rqueue = unsafe { (*inet).sk.sk_rmem_alloc.counter - (*udp_sk).forward_deficit };

    unsafe {
        BPF_SEQ_PRINTF(
            seq,
            c"%5d: %08X:%04X %08X:%04X ".as_ptr(),
            (*ctx).bucket,
            src,
            srcp,
            dest,
            destp,
        );
    }

    unsafe {
        BPF_SEQ_PRINTF(
            seq,
            c"%02X %08X:%08X %02X:%08lX %08X %5u %8d %lu %d %pK %u\n".as_ptr(),
            (*inet).sk.sk_state,
            (*inet).sk.sk_wmem_alloc.refs.counter - 1,
            rqueue,
            0,
            0i64,
            0,
            (*ctx).uid,
            0,
            sock_i_ino(addr_of!((*inet).sk)),
            (*inet).sk.sk_refcnt.refs.counter,
            udp_sk,
            (*udp_sk).drop_counters.drops0.counter + (*udp_sk).drop_counters.drops1.counter,
        );
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
