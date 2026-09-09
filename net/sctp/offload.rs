// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sctp_offload - GRO/GSO Offloading for SCTP
 *
 * Copyright (C) 2015, Marcelo Ricardo Leitner <marcelo.leitner@gmail.com>
 */

// Dependency headers from the original C translation unit are supplied by
// other parts of the kernel binding.

unsafe fn sctp_gso_make_checksum(skb: *mut sk_buff) -> __le32 {
    (*skb).ip_summed = CHECKSUM_NONE;
    (*skb).csum_not_inet = 0;
    /* csum and csum_start in GSO CB may be needed to do the UDP
     * checksum when it's a UDP tunneling packet.
     */
    (*SKB_GSO_CB(skb)).csum = !0 as __wsum;
    (*SKB_GSO_CB(skb)).csum_start = skb_headroom(skb) + (*skb).len;
    sctp_compute_cksum(skb, skb_transport_offset(skb))
}

unsafe fn sctp_gso_segment(
    mut skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let mut segs: *mut sk_buff = ERR_PTR(-EINVAL);
    let mut sh: *mut sctphdr;

    if !skb_is_gso_sctp(skb) {
        goto_out!(out);
    }

    sh = sctp_hdr(skb);
    if !pskb_may_pull(skb, core::mem::size_of::<sctphdr>()) {
        goto_out!(out);
    }

    __skb_pull(skb, core::mem::size_of::<sctphdr>());

    if skb_gso_ok(skb, features | NETIF_F_GSO_ROBUST) {
        /* Packet is from an untrusted source, reset gso_segs. */
        let pinfo: *mut skb_shared_info = skb_shinfo(skb);
        let mut frag_iter: *mut sk_buff;

        (*pinfo).gso_segs = 0;
        if (*skb).len != (*skb).data_len {
            /* Means we have chunks in here too */
            (*pinfo).gso_segs += 1;
        }

        skb_walk_frags!(skb, frag_iter, {
            (*pinfo).gso_segs += 1;
        });

        segs = core::ptr::null_mut();
        goto_out!(out);
    }

    segs = skb_segment(skb, (features | NETIF_F_HW_CSUM) & !NETIF_F_SG);
    if IS_ERR(segs) {
        goto_out!(out);
    }

    /* All that is left is update SCTP CRC if necessary */
    if (features & NETIF_F_SCTP_CRC) == 0 {
        let mut current = segs;
        while !current.is_null() {
            if (*current).ip_summed == CHECKSUM_PARTIAL {
                sh = sctp_hdr(current);
                (*sh).checksum = sctp_gso_make_checksum(current);
            }
            current = (*current).next;
        }
    }

out:
    segs
}

static sctp_offload: net_offload = net_offload {
    callbacks: net_offload_callbacks {
        gso_segment: Some(sctp_gso_segment),
    },
};

static sctp6_offload: net_offload = net_offload {
    callbacks: net_offload_callbacks {
        gso_segment: Some(sctp_gso_segment),
    },
};

pub unsafe fn sctp_offload_init() -> c_int {
    let mut ret: c_int;

    ret = inet_add_offload(&sctp_offload, IPPROTO_SCTP);
    if ret != 0 {
        goto_out!(out);
    }

    ret = inet6_add_offload(&sctp6_offload, IPPROTO_SCTP);
    if ret != 0 {
        goto_out!(ipv4);
    }

    return ret;

ipv4:
    inet_del_offload(&sctp_offload, IPPROTO_SCTP);
out:
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
