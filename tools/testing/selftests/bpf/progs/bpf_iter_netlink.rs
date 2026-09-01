// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: <vmlinux.h>, "bpf_tracing_net.h", <bpf/bpf_helpers.h>

use core::ffi::{c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::addr_of_mut;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[inline(never)]
unsafe fn SOCK_INODE(socket: *mut socket) -> *mut inode {
    addr_of_mut!((*container_of!(socket, socket_alloc, socket)).vfs_inode)
}

#[no_mangle]
#[link_section = "iter/netlink"]
pub unsafe extern "C" fn dump_netlink(ctx: *mut bpf_iter__netlink) -> c_int {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let nlk: *mut netlink_sock = (*ctx).sk;
    let mut group: c_ulong;
    let mut ino: c_ulong;
    let mut inode: *mut inode;
    let mut sk: *mut socket;
    let mut s: *mut sock;

    if nlk == 0 as *mut c_void as *mut netlink_sock {
        return 0;
    }

    if (*(*ctx).meta).seq_num == 0 {
        BPF_SEQ_PRINTF!(
            seq,
            "sk               Eth Pid        Groups   Rmem     Wmem     Dump  Locks    Drops    Inode\n"
        );
    }

    s = addr_of_mut!((*nlk).sk);
    BPF_SEQ_PRINTF!(seq, "%pK %-3d ", s, (*s).sk_protocol);

    if (*nlk).groups.is_null() {
        group = 0;
    } else {
        /* FIXME: temporary use bpf_probe_read_kernel here, needs
         * verifier support to do direct access.
         */
        bpf_probe_read_kernel(
            addr_of_mut!(group) as *mut c_void,
            size_of::<c_ulong>() as u32,
            addr_of_mut!(*(*nlk).groups.add(0)) as *const c_void,
        );
    }
    BPF_SEQ_PRINTF!(
        seq,
        "%-10u %08x %-8d %-8d %-5d %-8d ",
        (*nlk).portid,
        group as u32,
        (*s).sk_rmem_alloc.counter,
        (*s).sk_wmem_alloc.refs.counter - 1,
        (*nlk).cb_running,
        (*s).sk_refcnt.refs.counter
    );

    sk = (*s).sk_socket;
    if sk.is_null() {
        ino = 0;
    } else {
        /* FIXME: container_of inside SOCK_INODE has a forced
         * type conversion, and direct access cannot be used
         * with current verifier.
         */
        inode = SOCK_INODE(sk);
        bpf_probe_read_kernel(
            addr_of_mut!(ino) as *mut c_void,
            size_of::<c_ulong>() as u32,
            addr_of_mut!((*inode).i_ino) as *const c_void,
        );
    }
    BPF_SEQ_PRINTF!(seq, "%-8u %-8lu\n", (*s).sk_drops.counter, ino);

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
