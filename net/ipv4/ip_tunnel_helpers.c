// SPDX-License-Identifier: GPL-2.0-only
/* Configured macro/header-inline/bitfield boundaries only. Tunnel policy,
 * iteration, ownership, errors and packet processing live in ip_tunnel.rs. */
#define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
#include "ip_tunnel_bindings.h"
void *rust_ipt_netdev_priv(const struct net_device *d) { return netdev_priv(d); }
void *rust_ipt_net_generic(const struct net *n, unsigned int id) { return net_generic(n, id); }
struct net *rust_ipt_dev_net(const struct net_device *d) { return dev_net(d); }
void rust_ipt_dev_net_set(struct net_device *d, struct net *n) { dev_net_set(d, n); }
bool rust_ipt_net_eq(const struct net *a, const struct net *b) { return net_eq(a, b); }
bool rust_ipt_has_fallback(const struct net *n) { return net_has_fallback_tunnels(n); }
unsigned int rust_ipt_hash32(u32 value, unsigned int bits) { return hash_32(value, bits); }
bool rust_ipt_test_bit(unsigned int bit, const unsigned long *f) { return test_bit(bit, f); }
struct hlist_node *rust_ipt_hlist_first(const struct hlist_head *h, bool rtnl) { __list_check_rcu(dummy, rtnl && lockdep_rtnl_is_held()); return rcu_dereference_raw(hlist_first_rcu(h)); }
struct hlist_node *rust_ipt_hlist_next(const struct hlist_node *n) { return rcu_dereference_raw(hlist_next_rcu(n)); }
void rust_ipt_hlist_add(struct hlist_node *n, struct hlist_head *h) { hlist_add_head_rcu(n, h); }
void rust_ipt_hlist_del(struct hlist_node *n) { hlist_del_init_rcu(n); }
struct ip_tunnel *rust_ipt_rcu_read(struct ip_tunnel * const *p) { return rcu_dereference(*p); }
struct ip_tunnel *rust_ipt_rtnl_read(struct ip_tunnel * const *p) { return rtnl_dereference(*p); }
void rust_ipt_rcu_assign(struct ip_tunnel **p, struct ip_tunnel *v) { rcu_assign_pointer(*p, v); }
int rust_ipt_read_int(const int *p) { return READ_ONCE(*p); }
void rust_ipt_write_int(int *p, int v) { WRITE_ONCE(*p, v); }
unsigned long rust_ipt_read_ulong(const unsigned long *p) { return READ_ONCE(*p); }
struct net *rust_ipt_read_net(struct net * const *p) { return READ_ONCE(*p); }
struct net_device *rust_ipt_read_dev(struct net_device * const *p) { return READ_ONCE(*p); }
void rust_ipt_write_dev(struct net_device **p, struct net_device *v) { WRITE_ONCE(*p, v); }
void rust_ipt_write_uint(unsigned int *p, unsigned int v) { WRITE_ONCE(*p, v); }
unsigned long rust_ipt_jiffies(void) { return jiffies; }
void rust_ipt_assert_rtnl(void) { ASSERT_RTNL(); }
void rust_ipt_assert_rtnl_net(struct net *n) { ASSERT_RTNL_NET(n); }
struct net_device *rust_ipt_alloc_netdev(unsigned int size, const char *name, void (*setup)(struct net_device *)) { return alloc_netdev(size, name, NET_NAME_UNKNOWN, setup); }
ssize_t rust_ipt_strscpy(char *d, const char *s, size_t n) { return strscpy(d, s, n); }
void rust_ipt_unregister_netdevice(struct net_device *d) { unregister_netdevice(d); }
bool rust_ipt_is_err(const void *p) { return IS_ERR(p); }
void rust_ipt_init_flow(struct flowi4 *f, int p, __be32 d, __be32 s, __be32 k, u8 tos, struct net *n, int oif, u32 m, u32 hash, u8 flags) { ip_tunnel_init_flow(f, p, d, s, k, tos, n, oif, m, hash, flags); }
struct rtable *rust_ipt_route_output(struct net *n, struct flowi4 *f) { return ip_route_output_key(n, f); }
void rust_ipt_rt_put(struct rtable *r) { ip_rt_put(r); }
struct net_device *rust_ipt_dst_dev(const struct dst_entry *d) { return d->dev; }
void rust_ipt_cache_reset(struct dst_cache *c) { dst_cache_reset(c); }
unsigned int rust_ipt_limit_headroom(unsigned int h) { return ip_tunnel_limit_headroom(h); }
void rust_ipt_adj_headroom(struct net_device *d, unsigned int h) { ip_tunnel_adj_headroom(d, h); }
struct iphdr *rust_ipt_ip_hdr(const struct sk_buff *s) { return ip_hdr(s); }
unsigned char *rust_ipt_network_header(const struct sk_buff *s) { return skb_network_header(s); }
unsigned char *rust_ipt_inner_network_header(const struct sk_buff *s) { return skb_inner_network_header(s); }
void rust_ipt_set_network_header(struct sk_buff *s, int o) { skb_set_network_header(s, o); }
bool rust_ipt_inet_may_pull(struct sk_buff *s) { return pskb_inet_may_pull(s); }
int rust_ipt_ecn_decap(const struct iphdr *h, struct sk_buff *s) { return IP_ECN_decapsulate(h, s); }
void rust_ipt_log_ecn(const struct iphdr *h) { net_info_ratelimited("non-ECT from %pI4 with TOS=%#x\n", &h->saddr, h->tos); }
void rust_ipt_stats_rx_add(struct net_device *d, unsigned int n) { dev_sw_netstats_rx_add(d, n); }
void rust_ipt_postpull_eth_rcsum(struct sk_buff *s) { skb_postpull_rcsum(s, eth_hdr(s), ETH_HLEN); }
void rust_ipt_skb_dst_set(struct sk_buff *s, struct dst_entry *d) { skb_dst_set(s, d); }
void rust_ipt_kfree_skb(struct sk_buff *s) { kfree_skb(s); }
void rust_ipt_skb_set_dev(struct sk_buff *s, struct net_device *d) { s->dev = d; }
void rust_ipt_skb_set_protocol(struct sk_buff *s, __be16 p) { s->protocol = p; }
__be16 rust_ipt_skb_protocol(const struct sk_buff *s) { return s->protocol; }
u32 rust_ipt_skb_mark(const struct sk_buff *s) { return s->mark; }
void rust_ipt_set_broadcast(struct sk_buff *s) { s->pkt_type = PACKET_BROADCAST; }
const struct ip_tunnel_encap_ops *rust_ipt_cmpxchg_encap(unsigned int n, const struct ip_tunnel_encap_ops *o, const struct ip_tunnel_encap_ops *v) { return cmpxchg((const struct ip_tunnel_encap_ops **)&iptun_encaps[n], o, v); }
int rust_ipt_encap_hlen(struct ip_tunnel_encap *e) { return ip_encap_hlen(e); }
int rust_ipt_encap(struct sk_buff *s, struct ip_tunnel_encap *e, u8 *p, struct flowi4 *f) { return ip_tunnel_encap(s, e, p, f); }
unsigned int rust_ipt_dst_mtu(const struct dst_entry *d) { return dst_mtu(d); }
bool rust_ipt_skb_valid_dst(const struct sk_buff *s) { return skb_valid_dst(s); }
struct dst_entry *rust_ipt_skb_dst(const struct sk_buff *s) { return skb_dst(s); }
void rust_ipt_update_pmtu(struct sk_buff *s, u32 mtu) { skb_dst_update_pmtu_no_confirm(s, mtu); }
bool rust_ipt_skb_is_gso(const struct sk_buff *s) { return skb_is_gso(s); }
struct ip_tunnel_info *rust_ipt_tunnel_info(struct sk_buff *s) { return skb_tunnel_info(s); }
unsigned short rust_ipt_info_af(const struct ip_tunnel_info *i) { return ip_tunnel_info_af(i); }
void rust_ipt_clear_ipcb_options(struct sk_buff *s) { memset(&IPCB(s)->opt, 0, sizeof(IPCB(s)->opt)); }
u8 rust_ipt_ipv6_dsfield(const struct ipv6hdr *h) { return ipv6_get_dsfield(h); }
__be32 rust_ipt_id_to_key(__be64 id) { return tunnel_id_to_key32(id); }
u32 rust_ipt_skb_hash(struct sk_buff *s) { return skb_get_hash(s); }
bool rust_ipt_cache_usable(const struct sk_buff *s, const struct ip_tunnel_info *i) { return ip_tunnel_dst_cache_usable(s, i); }
u8 rust_ipt_ecn_encap(u8 tos, const struct iphdr *h, const struct sk_buff *s) { return ip_tunnel_ecn_encap(tos, h, s); }
unsigned int rust_ipt_ll_reserved(const struct net_device *d) { return LL_RESERVED_SPACE(d); }
int rust_ipt_cow_head(struct sk_buff *s, unsigned int h) { return skb_cow_head(s, h); }
__be16 rust_ipt_payload_protocol(const struct sk_buff *s) { return skb_protocol(s, true); }
struct rtable *rust_ipt_skb_rtable(const struct sk_buff *s) { return skb_rtable(s); }
__be32 rust_ipt_rt_nexthop(const struct rtable *r, __be32 d) { return rt_nexthop(r, d); }
void rust_ipt_dst_link_failure(struct sk_buff *s) { dst_link_failure(s); }
void rust_ipt_dev_addr_set(struct net_device *d, const void *a, size_t n) { __dev_addr_set(d, a, n); }
void rust_ipt_flags_from_be16(unsigned long *f, __be16 v) { ip_tunnel_flags_from_be16(f, v); }
__be16 rust_ipt_flags_to_be16(const unsigned long *f) { return ip_tunnel_flags_to_be16(f); }
bool rust_ipt_flags_compat(const unsigned long *f) { return ip_tunnel_flags_is_be16_compat(f); }
unsigned long rust_ipt_copy_from_user(void *d, const void __user *s, unsigned long n) { return copy_from_user(d, s, n); }
unsigned long rust_ipt_copy_to_user(void __user *d, const void *s, unsigned long n) { return copy_to_user(d, s, n); }
void rust_ipt_set_netns_immutable(struct net_device *d) { d->netns_immutable = true; }
void rust_ipt_set_pcpu_stat_type(struct net_device *d) { d->pcpu_stat_type = NETDEV_PCPU_STAT_TSTATS; }
struct net_device *rust_ipt_first_netdev(struct net *n) { return first_net_device(n); }
struct net_device *rust_ipt_next_netdev(struct net_device *d) { return next_net_device(d); }
void rust_ipt_hw_addr_random(struct net_device *d) { eth_hw_addr_random(d); }
void rust_ipt_keep_dst(struct net_device *d) { netif_keep_dst(d); }
#if IS_ENABLED(CONFIG_IPV6)
struct rt6_info *rust_ipt_dst_rt6(struct dst_entry *d) { return dst_rt6_info(d); }
void rust_ipt_dst_metric_set(struct dst_entry *d, int metric, u32 value) { dst_metric_set(d, metric, value); }
struct neighbour *rust_ipt_dst_neigh(struct dst_entry *d, const void *p) { return dst_neigh_lookup(d, p); }
struct in6_addr *rust_ipt_ipv6_daddr(struct sk_buff *s) { return &ipv6_hdr(s)->daddr; }
struct in6_addr *rust_ipt_neigh_key(struct neighbour *n) { return (struct in6_addr *)&n->primary_key; }
int rust_ipt_ipv6_addr_type(const struct in6_addr *a) { return ipv6_addr_type(a); }
__be32 rust_ipt_ipv6_word3(const struct in6_addr *a) { return a->s6_addr32[3]; }
void rust_ipt_neigh_release(struct neighbour *n) { neigh_release(n); }
#endif
void rust_ipt_multicast(struct net_device *d) { DEV_STATS_INC(d, multicast); }
void rust_ipt_rx_crc_errors(struct net_device *d) { DEV_STATS_INC(d, rx_crc_errors); }
void rust_ipt_rx_errors(struct net_device *d) { DEV_STATS_INC(d, rx_errors); }
void rust_ipt_rx_fifo_errors(struct net_device *d) { DEV_STATS_INC(d, rx_fifo_errors); }
void rust_ipt_rx_length_errors(struct net_device *d) { DEV_STATS_INC(d, rx_length_errors); }
void rust_ipt_rx_frame_errors(struct net_device *d) { DEV_STATS_INC(d, rx_frame_errors); }
void rust_ipt_tx_carrier_errors(struct net_device *d) { DEV_STATS_INC(d, tx_carrier_errors); }
void rust_ipt_collisions(struct net_device *d) { DEV_STATS_INC(d, collisions); }
void rust_ipt_tx_errors(struct net_device *d) { DEV_STATS_INC(d, tx_errors); }
void rust_ipt_tx_dropped(struct net_device *d) { DEV_STATS_INC(d, tx_dropped); }
void rust_ipt_tx_fifo_errors(struct net_device *d) { DEV_STATS_INC(d, tx_fifo_errors); }
/* C emits the public symbol metadata; each symbol is defined by Rust. */
EXPORT_SYMBOL_GPL(ip_tunnel_lookup);
EXPORT_SYMBOL(ip_tunnel_md_udp_encap);
EXPORT_SYMBOL_GPL(ip_tunnel_rcv);
EXPORT_SYMBOL(ip_tunnel_encap_add_ops);
EXPORT_SYMBOL(ip_tunnel_encap_del_ops);
EXPORT_SYMBOL_GPL(ip_tunnel_encap_setup);
EXPORT_SYMBOL_GPL(ip_md_tunnel_xmit);
EXPORT_SYMBOL_GPL(ip_tunnel_xmit);
EXPORT_SYMBOL_GPL(ip_tunnel_ctl);
EXPORT_SYMBOL_GPL(ip_tunnel_parm_from_user);
EXPORT_SYMBOL_GPL(ip_tunnel_parm_to_user);
EXPORT_SYMBOL_GPL(ip_tunnel_siocdevprivate);
EXPORT_SYMBOL_GPL(ip_tunnel_change_mtu);
EXPORT_SYMBOL_GPL(ip_tunnel_dellink);
EXPORT_SYMBOL(ip_tunnel_get_link_net);
EXPORT_SYMBOL(ip_tunnel_get_iflink);
EXPORT_SYMBOL_GPL(ip_tunnel_init_net);
EXPORT_SYMBOL_GPL(ip_tunnel_delete_net);
EXPORT_SYMBOL_GPL(ip_tunnel_newlink);
EXPORT_SYMBOL_GPL(ip_tunnel_changelink);
EXPORT_SYMBOL_GPL(__ip_tunnel_init);
EXPORT_SYMBOL_GPL(ip_tunnel_uninit);
EXPORT_SYMBOL_GPL(ip_tunnel_setup);
MODULE_DESCRIPTION("IPv4 tunnel implementation library");
MODULE_LICENSE("GPL");
int rust_ipt_hoplimit(const struct dst_entry *d) { return ip4_dst_hoplimit(d); }
/* These are conditional header inlines when NF_NAT is disabled. */
void rust_ipt_icmp_send(struct sk_buff *s, int type, int code, __be32 info) { icmp_ndo_send(s, type, code, info); }
#if IS_ENABLED(CONFIG_IPV6)
void rust_ipt_icmpv6_send(struct sk_buff *s, u8 type, u8 code, u32 info) { icmpv6_ndo_send(s, type, code, info); }
#endif
