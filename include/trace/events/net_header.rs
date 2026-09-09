/* SPDX-License-Identifier: GPL-2.0 */
// Rust representation of the Linux net tracepoint header.
// The tracepoint DSL is supplied by the surrounding kernel trace infrastructure.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// C preprocessor guard: _TRACE_NET_H / TRACE_HEADER_MULTI_READ.
// C includes are external dependencies and intentionally remain unresolved.

trace_event!(net_dev_start_xmit,
    proto: (const struct sk_buff *skb, const struct net_device *dev),
    entry: {
        string name = dev->name;
        field u16 queue_mapping = skb->queue_mapping;
        field *const_void skbaddr = skb;
        field bool vlan_tagged = skb_vlan_tag_present(skb);
        field u16 vlan_proto = ntohs(skb->vlan_proto);
        field u16 vlan_tci = skb_vlan_tag_get(skb);
        field u16 protocol = ntohs(skb->protocol);
        field u8 ip_summed = skb->ip_summed;
        field uint len = skb->len;
        field uint data_len = skb->data_len;
        field int network_offset = skb_network_offset(skb);
        field bool transport_offset_valid = skb_transport_header_was_set(skb);
        field int transport_offset = if skb_transport_header_was_set(skb) { skb_transport_offset(skb) } else { 0 };
        field u8 tx_flags = skb_shinfo(skb)->tx_flags;
        field u16 gso_size = skb_shinfo(skb)->gso_size;
        field u16 gso_segs = skb_shinfo(skb)->gso_segs;
        field u16 gso_type = skb_shinfo(skb)->gso_type;
        field u64 net_cookie = dev_net(dev)->net_cookie;
    }
);

trace_event!(net_dev_xmit,
    proto: (struct sk_buff *skb, int rc, struct net_device *dev, uint skb_len),
    entry: { void_ptr skbaddr = skb; uint len = skb_len; int result = rc; string name = dev->name; u64 net_cookie = dev_net(dev)->net_cookie; }
);

trace_event!(net_dev_xmit_timeout,
    proto: (struct net_device *dev, int queue_index),
    entry: { string name = dev->name; string driver = netdev_drivername(dev); int queue_index; u64 net_cookie = dev_net(dev)->net_cookie; }
);

trace_event_class!(net_dev_template,
    proto: (struct sk_buff *skb),
    entry: { void_ptr skbaddr = skb; uint len = skb->len; string name = skb->dev->name; u64 net_cookie = dev_net(skb->dev)->net_cookie; }
);
trace_event!(net_dev_queue, use net_dev_template, (struct sk_buff *skb));
trace_event!(netif_receive_skb, use net_dev_template, (struct sk_buff *skb));
trace_event!(netif_rx, use net_dev_template, (struct sk_buff *skb));

trace_event_class!(net_dev_rx_verbose_template,
    proto: (const struct sk_buff *skb),
    entry: {
        string name = skb->dev->name; uint napi_id = if cfg!(feature = "CONFIG_NET_RX_BUSY_POLL") { if napi_id_valid(skb->napi_id) { skb->napi_id } else { 0 } } else { 0 };
        u16 queue_mapping = skb->queue_mapping; const_void skbaddr = skb;
        bool vlan_tagged = skb_vlan_tag_present(skb); u16 vlan_proto = ntohs(skb->vlan_proto); u16 vlan_tci = skb_vlan_tag_get(skb);
        u16 protocol = ntohs(skb->protocol); u8 ip_summed = skb->ip_summed; u32 hash = skb->hash; bool l4_hash = skb->l4_hash;
        uint len = skb->len; uint data_len = skb->data_len; uint truesize = skb->truesize;
        bool mac_header_valid = skb_mac_header_was_set(skb); int mac_header = skb_mac_header(skb) - skb->data;
        u8 nr_frags = skb_shinfo(skb)->nr_frags; u16 gso_size = skb_shinfo(skb)->gso_size; u16 gso_type = skb_shinfo(skb)->gso_type;
        u64 net_cookie = dev_net(skb->dev)->net_cookie;
    }
);
trace_event!(napi_gro_frags_entry, use net_dev_rx_verbose_template, (const struct sk_buff *skb));
trace_event!(napi_gro_receive_entry, use net_dev_rx_verbose_template, (const struct sk_buff *skb));
trace_event!(netif_receive_skb_entry, use net_dev_rx_verbose_template, (const struct sk_buff *skb));
trace_event!(netif_receive_skb_list_entry, use net_dev_rx_verbose_template, (const struct sk_buff *skb));
trace_event!(netif_rx_entry, use net_dev_rx_verbose_template, (const struct sk_buff *skb));

trace_event_class!(net_dev_rx_exit_template, proto: (int ret), entry: { int ret = ret; }, print: "ret=%d");
trace_event!(napi_gro_frags_exit, use net_dev_rx_exit_template, (int ret));
trace_event!(napi_gro_receive_exit, use net_dev_rx_exit_template, (int ret));
trace_event!(netif_receive_skb_exit, use net_dev_rx_exit_template, (int ret));
trace_event!(netif_rx_exit, use net_dev_rx_exit_template, (int ret));
trace_event!(netif_receive_skb_list_exit, use net_dev_rx_exit_template, (int ret));

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
