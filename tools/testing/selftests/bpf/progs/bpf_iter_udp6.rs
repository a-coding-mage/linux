// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: <vmlinux.h>, "bpf_tracing_net.h", <bpf/bpf_helpers.h>,
// and <bpf/bpf_endian.h>.

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

const IPV6_SEQ_DGRAM_HEADER: &[u8] =
    b"  sl  local_address                         remote_address                        st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops\n\0";

unsafe fn sock_i_ino(sk: *const sock) -> libc::c_long {
    let sk_socket: *const socket = (*sk).sk_socket;
    let inode: *const inode;
    let mut ino: libc::c_ulong = 0;

    if sk_socket.is_null() {
        return 0;
    }

    inode = &(*container_of_socket_alloc_socket(sk_socket)).vfs_inode;
    bpf_probe_read_kernel(
        &mut ino as *mut _ as *mut libc::c_void,
        core::mem::size_of_val(&ino) as __u32,
        &(*inode).i_ino as *const _ as *const libc::c_void,
    );
    ino as libc::c_long
}

#[no_mangle]
#[link_section = "iter/udp"]
pub unsafe extern "C" fn dump_udp6(ctx: *mut bpf_iter__udp) -> libc::c_int {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let udp_sk: *mut udp_sock = (*ctx).udp_sk;
    let dest: *const in6_addr;
    let src: *const in6_addr;
    let udp6_sk: *mut udp6_sock;
    let inet: *mut inet_sock;
    let srcp: __u16;
    let destp: __u16;
    let seq_num: __u32;
    let rqueue: libc::c_int;

    if udp_sk == 0 as *mut libc::c_void as *mut udp_sock {
        return 0;
    }

    seq_num = (*(*ctx).meta).seq_num;
    if seq_num == 0 {
        BPF_SEQ_PRINTF(seq, IPV6_SEQ_DGRAM_HEADER.as_ptr() as *const libc::c_char);
    }

    udp6_sk = bpf_skc_to_udp6_sock(udp_sk);
    if udp6_sk == 0 as *mut libc::c_void as *mut udp6_sock {
        return 0;
    }

    inet = &mut (*udp_sk).inet;
    srcp = bpf_ntohs((*inet).inet_sport);
    destp = bpf_ntohs((*inet).inet_dport);
    rqueue = (*inet).sk.sk_rmem_alloc.counter - (*udp_sk).forward_deficit;
    dest = &(*inet).sk.sk_v6_daddr;
    src = &(*inet).sk.sk_v6_rcv_saddr;

    BPF_SEQ_PRINTF(
        seq,
        b"%5d: %08X%08X%08X%08X:%04X %08X%08X%08X%08X:%04X \0".as_ptr()
            as *const libc::c_char,
        (*ctx).bucket,
        (*src).s6_addr32[0],
        (*src).s6_addr32[1],
        (*src).s6_addr32[2],
        (*src).s6_addr32[3],
        srcp,
        (*dest).s6_addr32[0],
        (*dest).s6_addr32[1],
        (*dest).s6_addr32[2],
        (*dest).s6_addr32[3],
        destp,
    );

    BPF_SEQ_PRINTF(
        seq,
        b"%02X %08X:%08X %02X:%08lX %08X %5u %8d %lu %d %pK %u\n\0".as_ptr()
            as *const libc::c_char,
        (*inet).sk.sk_state,
        (*inet).sk.sk_wmem_alloc.refs.counter - 1,
        rqueue,
        0,
        0 as libc::c_long,
        0,
        (*ctx).uid,
        0,
        sock_i_ino(&(*inet).sk),
        (*inet).sk.sk_refcnt.refs.counter,
        udp_sk,
        (*udp_sk).drop_counters.drops0.counter + (*udp_sk).drop_counters.drops1.counter,
    );
    0
}
