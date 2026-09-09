/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux networking headers:
// `__be16`, `sk_buff`, `net_device`, `PACKET_HOST`, `ETH_P_X25`, `htons`, and
// `skb_reset_mac_header`.

#[inline]
pub unsafe fn x25_type_trans(skb: *mut sk_buff, dev: *mut net_device) -> __be16 {
    (*skb).dev = dev;
    skb_reset_mac_header(skb);
    (*skb).pkt_type = PACKET_HOST;

    htons(ETH_P_X25)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
