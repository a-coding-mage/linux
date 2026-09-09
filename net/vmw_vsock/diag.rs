// SPDX-License-Identifier: GPL-2.0-only
/*
 * vsock sock_diag(7) module
 *
 * Copyright (C) 2017 Red Hat, Inc.
 * Author: Stefan Hajnoczi <stefanha@redhat.com>
 */

// Dependencies supplied by the Linux kernel and other translation units.

unsafe fn sk_diag_fill(
    sk: *mut sock,
    skb: *mut sk_buff,
    portid: u32,
    seq: u32,
    flags: u32,
) -> i32 {
    let vsk: *mut vsock_sock = vsock_sk(sk);
    let mut rep: *mut vsock_diag_msg;
    let nlh: *mut nlmsghdr;

    nlh = nlmsg_put(
        skb,
        portid,
        seq,
        SOCK_DIAG_BY_FAMILY,
        core::mem::size_of::<vsock_diag_msg>() as u32,
        flags,
    );
    if nlh.is_null() {
        return -EMSGSIZE;
    }

    rep = nlmsg_data(nlh);
    (*rep).vdiag_family = AF_VSOCK;

    /* Lock order dictates that sk_lock is acquired before
     * vsock_table_lock, so we cannot lock here.  Simply don't take
     * sk_lock; sk is guaranteed to stay alive since vsock_table_lock is
     * held.
     */
    (*rep).vdiag_type = (*sk).sk_type;
    (*rep).vdiag_state = (*sk).sk_state;
    (*rep).vdiag_shutdown = (*sk).sk_shutdown;
    (*rep).vdiag_src_cid = (*vsk).local_addr.svm_cid;
    (*rep).vdiag_src_port = (*vsk).local_addr.svm_port;
    (*rep).vdiag_dst_cid = (*vsk).remote_addr.svm_cid;
    (*rep).vdiag_dst_port = (*vsk).remote_addr.svm_port;
    (*rep).vdiag_ino = sock_i_ino(sk);

    sock_diag_save_cookie(sk, (*rep).vdiag_cookie.as_mut_ptr());

    0
}

unsafe fn vsock_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let req: *mut vsock_diag_req;
    let mut vsk: *mut vsock_sock;
    let mut bucket: u32;
    let mut last_i: u32;
    let mut table: u32;
    let net: *mut net;
    let mut i: u32;

    req = nlmsg_data((*cb).nlh);
    net = sock_net((*skb).sk);

    /* State saved between calls: */
    table = (*cb).args[0];
    bucket = (*cb).args[1];
    i = (*cb).args[2];
    last_i = i;

    /* TODO VMCI pending sockets? */

    spin_lock_bh(&mut vsock_table_lock);

    /* Bind table (locally created sockets) */
    if table == 0 {
        while (bucket < ARRAY_SIZE(vsock_bind_table)) {
            let head: *mut list_head = &mut vsock_bind_table[bucket as usize];

            i = 0;
            list_for_each_entry!(vsk, head, bound_table, {
                let sk: *mut sock = sk_vsock(vsk);

                if !net_eq(sock_net(sk), net) {
                    continue;
                }
                if i < last_i {
                    goto_next_bind!();
                }
                if ((*req).vdiag_states & (1u32 << (*sk).sk_state)) == 0 {
                    goto_next_bind!();
                }
                if sk_diag_fill(
                    sk,
                    skb,
                    NETLINK_CB!((*cb).skb).portid,
                    (*(*cb).nlh).nlmsg_seq,
                    NLM_F_MULTI,
                ) < 0 {
                    goto_done!();
                }
                goto_next_bind!();
            });
            last_i = 0;
            bucket += 1;
        }

        table += 1;
        bucket = 0;
    }

    /* Connected table (accepted connections) */
    while bucket < ARRAY_SIZE(vsock_connected_table) {
        let head: *mut list_head = &mut vsock_connected_table[bucket as usize];

        i = 0;
        list_for_each_entry!(vsk, head, connected_table, {
            let sk: *mut sock = sk_vsock(vsk);

            /* Skip sockets we've already seen above */
            if __vsock_in_bound_table(vsk) {
                continue;
            }

            if !net_eq(sock_net(sk), net) {
                continue;
            }
            if i < last_i {
                goto_next_connected!();
            }
            if ((*req).vdiag_states & (1u32 << (*sk).sk_state)) == 0 {
                goto_next_connected!();
            }
            if sk_diag_fill(
                sk,
                skb,
                NETLINK_CB!((*cb).skb).portid,
                (*(*cb).nlh).nlmsg_seq,
                NLM_F_MULTI,
            ) < 0 {
                goto_done!();
            }
            goto_next_connected!();
        });
        last_i = 0;
        bucket += 1;
    }

    spin_unlock_bh(&mut vsock_table_lock);

    (*cb).args[0] = table;
    (*cb).args[1] = bucket;
    (*cb).args[2] = i;

    (*skb).len as i32
}

unsafe fn vsock_diag_handler_dump(skb: *mut sk_buff, h: *mut nlmsghdr) -> i32 {
    let hdrlen = core::mem::size_of::<vsock_diag_req>() as u32;
    let net: *mut net = sock_net((*skb).sk);

    if nlmsg_len(h) < hdrlen {
        return -EINVAL;
    }

    if (*h).nlmsg_flags & NLM_F_DUMP != 0 {
        let c = netlink_dump_control {
            dump: Some(vsock_diag_dump),
        };
        return netlink_dump_start((*net).diag_nlsk, skb, h, &c);
    }

    -EOPNOTSUPP
}

static vsock_diag_handler: sock_diag_handler = sock_diag_handler {
    owner: THIS_MODULE,
    family: AF_VSOCK,
    dump: Some(vsock_diag_handler_dump),
};

unsafe fn vsock_diag_init() -> i32 {
    sock_diag_register(&vsock_diag_handler)
}

unsafe fn vsock_diag_exit() {
    sock_diag_unregister(&vsock_diag_handler);
}

// module_init(vsock_diag_init);
// module_exit(vsock_diag_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("VMware Virtual Sockets monitoring via SOCK_DIAG");
// MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG,
//                                40 /* AF_VSOCK */);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
