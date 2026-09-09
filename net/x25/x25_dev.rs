// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * X.25 Packet Layer release 002
 *
 * This is ALPHA test software. This code may break your machine, randomly fail to work with new
 * releases, misbehave and/or generally screw up. It might even work.
 *
 * This code REQUIRES 2.1.15 or higher
 *
 * History
 * X.25 001 Jonathan Naylor Started coding.
 * 2000-09-04 Henner Eisen Prevent freeing a dangling skb.
 */

// pr_fmt(fmt) expands to "X25: " followed by fmt.

use core::ptr;

unsafe fn x25_receive_data(skb: *mut sk_buff, nb: *mut x25_neigh) -> i32 {
    let mut sk: *mut sock;
    let frametype: u16;
    let lci: u32;

    if !pskb_may_pull(skb, X25_STD_MIN_LEN) {
        return 0;
    }

    frametype = (*skb).data[2] as u16;
    lci = (((*skb).data[0] as u32) << 8 & 0xF00)
        + (((*skb).data[1] as u32) << 0 & 0x0FF);

    /*
     * LCI of zero is always for us, and its always a link control
     * frame.
     */
    if lci == 0 {
        x25_link_control(skb, nb, frametype);
        return 0;
    }

    /* Find an existing socket. */
    sk = x25_find_socket(lci, nb);
    if !sk.is_null() {
        let mut queued: i32 = 1;

        skb_reset_transport_header(skb);
        bh_lock_sock(sk);
        if !sock_owned_by_user(sk) {
            queued = x25_process_rx_frame(sk, skb);
        } else {
            queued = (!sk_add_backlog(sk, skb, READ_ONCE((*sk).sk_rcvbuf))) as i32;
        }
        bh_unlock_sock(sk);
        sock_put(sk);
        return queued;
    }

    /* Is is a Call Request ? if so process it. */
    if frametype == X25_CALL_REQUEST {
        return x25_rx_call_request(skb, nb, lci);
    }

    /* Its not a Call Request, nor is it a control frame. Can we forward it? */
    if x25_forward_data(lci, nb, skb) {
        if frametype == X25_CLEAR_CONFIRMATION {
            x25_clear_forward_by_lci(lci);
        }
        kfree_skb(skb);
        return 1;
    }

    /* x25_transmit_clear_request(nb, lci, 0x0D); */

    if frametype != X25_CLEAR_CONFIRMATION {
        pr_debug!("x25_receive_data(): unknown frame type %2x\n", frametype);
    }

    0
}

pub unsafe fn x25_lapb_receive_frame(
    mut skb: *mut sk_buff,
    dev: *mut net_device,
    _ptype: *mut packet_type,
    _orig_dev: *mut net_device,
) -> i32 {
    let nskb: *mut sk_buff;
    let nb: *mut x25_neigh;

    if !net_eq(dev_net((*dev).net), &init_net) {
        kfree_skb(skb);
        return 0;
    }

    nskb = skb_copy(skb, GFP_ATOMIC);
    if nskb.is_null() {
        kfree_skb(skb);
        return 0;
    }
    kfree_skb(skb);
    skb = nskb;

    /* Packet received from unrecognised device, throw it away. */
    nb = x25_get_neigh(dev);
    if nb.is_null() {
        pr_debug!("unknown neighbour - %s\n", (*dev).name);
        kfree_skb(skb);
        return 0;
    }

    if !pskb_may_pull(skb, 1) {
        x25_neigh_put(nb);
        kfree_skb(skb);
        return 0;
    }

    match (*skb).data[0] {
        X25_IFACE_DATA => {
            skb_pull(skb, 1);
            if x25_receive_data(skb, nb) != 0 {
                x25_neigh_put(nb);
                return 0;
            }
        }
        X25_IFACE_CONNECT => x25_link_established(nb),
        X25_IFACE_DISCONNECT => x25_link_terminated(nb),
        _ => {}
    }
    x25_neigh_put(nb);
    kfree_skb(skb);
    0
}

pub unsafe fn x25_establish_link(nb: *mut x25_neigh) {
    let skb: *mut sk_buff;
    let ptr: *mut u8;

    match (*(*nb).dev).type_ {
        ARPHRD_X25 => {
            skb = alloc_skb(1, GFP_ATOMIC);
            if skb.is_null() {
                pr_err!("x25_dev: out of memory\n");
                return;
            }
            ptr = skb_put(skb, 1);
            *ptr = X25_IFACE_CONNECT;
        }
        _ => return,
    }

    (*skb).protocol = htons(ETH_P_X25);
    (*skb).dev = (*nb).dev;
    dev_queue_xmit(skb);
}

pub unsafe fn x25_send_frame(skb: *mut sk_buff, nb: *mut x25_neigh) {
    let dptr: *mut u8;

    skb_reset_network_header(skb);

    match (*(*nb).dev).type_ {
        ARPHRD_X25 => {
            dptr = skb_push(skb, 1);
            *dptr = X25_IFACE_DATA;
        }
        _ => {
            kfree_skb(skb);
            return;
        }
    }

    (*skb).protocol = htons(ETH_P_X25);
    (*skb).dev = (*nb).dev;
    dev_queue_xmit(skb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
