/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_IP_TUNNEL_BINDINGS_H
#define LUPOS_IP_TUNNEL_BINDINGS_H
#include <linux/capability.h>
#include <linux/module.h>
#include <linux/types.h>
#include <linux/kernel.h>
#include <linux/slab.h>
#include <linux/uaccess.h>
#include <linux/skbuff.h>
#include <linux/netdevice.h>
#include <linux/in.h>
#include <linux/tcp.h>
#include <linux/udp.h>
#include <linux/if_arp.h>
#include <linux/init.h>
#include <linux/in6.h>
#include <linux/inetdevice.h>
#include <linux/igmp.h>
#include <linux/netfilter_ipv4.h>
#include <linux/etherdevice.h>
#include <linux/if_ether.h>
#include <linux/if_vlan.h>
#include <linux/rculist.h>
#include <linux/err.h>

#include <net/sock.h>
#include <net/ip.h>
#include <net/icmp.h>
#include <net/protocol.h>
#include <net/ip_tunnels.h>
#include <net/arp.h>
#include <net/checksum.h>
#include <net/dsfield.h>
#include <net/inet_ecn.h>
#include <net/xfrm.h>
#include <net/net_namespace.h>
#include <net/netns/generic.h>
#include <net/netdev_lock.h>
#include <net/rtnetlink.h>
#include <net/udp.h>
#include <net/dst_metadata.h>
#include <net/inet_dscp.h>

#if IS_ENABLED(CONFIG_IPV6)
#include <net/ipv6.h>
#include <net/ip6_fib.h>
#include <net/ip6_route.h>
#endif

enum { RUST_IPT_GFP_KERNEL = GFP_KERNEL, RUST_IPT_LL_MAX_HEADER = LL_MAX_HEADER };
void *rust_ipt_netdev_priv(const struct net_device *d);
void *rust_ipt_net_generic(const struct net *n, unsigned int id);
struct net *rust_ipt_dev_net(const struct net_device *d);
void rust_ipt_dev_net_set(struct net_device *d, struct net *n);
bool rust_ipt_net_eq(const struct net *a, const struct net *b);
bool rust_ipt_has_fallback(const struct net *n);
unsigned int rust_ipt_hash32(u32 value, unsigned int bits);
bool rust_ipt_test_bit(unsigned int bit, const unsigned long *f);
struct hlist_node *rust_ipt_hlist_first(const struct hlist_head *h, bool rtnl);
struct hlist_node *rust_ipt_hlist_next(const struct hlist_node *n);
void rust_ipt_hlist_add(struct hlist_node *n, struct hlist_head *h);
void rust_ipt_hlist_del(struct hlist_node *n);
struct ip_tunnel *rust_ipt_rcu_read(struct ip_tunnel * const *p);
struct ip_tunnel *rust_ipt_rtnl_read(struct ip_tunnel * const *p);
void rust_ipt_rcu_assign(struct ip_tunnel **p, struct ip_tunnel *v);
int rust_ipt_read_int(const int *p);
void rust_ipt_write_int(int *p, int v);
unsigned long rust_ipt_read_ulong(const unsigned long *p);
struct net *rust_ipt_read_net(struct net * const *p);
struct net_device *rust_ipt_read_dev(struct net_device * const *p);
void rust_ipt_write_dev(struct net_device **p, struct net_device *v);
void rust_ipt_write_uint(unsigned int *p, unsigned int v);
unsigned long rust_ipt_jiffies(void);
void rust_ipt_assert_rtnl(void);
void rust_ipt_assert_rtnl_net(struct net *n);
struct net_device *rust_ipt_alloc_netdev(unsigned int size, const char *name, void (*setup)(struct net_device *));
ssize_t rust_ipt_strscpy(char *d, const char *s, size_t n);
void rust_ipt_unregister_netdevice(struct net_device *d);
bool rust_ipt_is_err(const void *p);
void rust_ipt_init_flow(struct flowi4 *f, int p, __be32 d, __be32 s, __be32 k, u8 tos, struct net *n, int oif, u32 m, u32 hash, u8 flags);
struct rtable *rust_ipt_route_output(struct net *n, struct flowi4 *f);
void rust_ipt_rt_put(struct rtable *r);
struct net_device *rust_ipt_dst_dev(const struct dst_entry *d);
void rust_ipt_cache_reset(struct dst_cache *c);
unsigned int rust_ipt_limit_headroom(unsigned int h);
void rust_ipt_adj_headroom(struct net_device *d, unsigned int h);
struct iphdr *rust_ipt_ip_hdr(const struct sk_buff *s);
unsigned char *rust_ipt_network_header(const struct sk_buff *s);
unsigned char *rust_ipt_inner_network_header(const struct sk_buff *s);
void rust_ipt_set_network_header(struct sk_buff *s, int o);
bool rust_ipt_inet_may_pull(struct sk_buff *s);
int rust_ipt_ecn_decap(const struct iphdr *h, struct sk_buff *s);
void rust_ipt_log_ecn(const struct iphdr *h);
void rust_ipt_stats_rx_add(struct net_device *d, unsigned int n);
void rust_ipt_postpull_eth_rcsum(struct sk_buff *s);
void rust_ipt_skb_dst_set(struct sk_buff *s, struct dst_entry *d);
void rust_ipt_kfree_skb(struct sk_buff *s);
void rust_ipt_skb_set_dev(struct sk_buff *s, struct net_device *d);
void rust_ipt_skb_set_protocol(struct sk_buff *s, __be16 p);
__be16 rust_ipt_skb_protocol(const struct sk_buff *s);
u32 rust_ipt_skb_mark(const struct sk_buff *s);
void rust_ipt_set_broadcast(struct sk_buff *s);
const struct ip_tunnel_encap_ops *rust_ipt_cmpxchg_encap(unsigned int n, const struct ip_tunnel_encap_ops *o, const struct ip_tunnel_encap_ops *v);
int rust_ipt_encap_hlen(struct ip_tunnel_encap *e);
int rust_ipt_encap(struct sk_buff *s, struct ip_tunnel_encap *e, u8 *p, struct flowi4 *f);
unsigned int rust_ipt_dst_mtu(const struct dst_entry *d);
bool rust_ipt_skb_valid_dst(const struct sk_buff *s);
struct dst_entry *rust_ipt_skb_dst(const struct sk_buff *s);
void rust_ipt_update_pmtu(struct sk_buff *s, u32 mtu);
bool rust_ipt_skb_is_gso(const struct sk_buff *s);
struct ip_tunnel_info *rust_ipt_tunnel_info(struct sk_buff *s);
unsigned short rust_ipt_info_af(const struct ip_tunnel_info *i);
void rust_ipt_clear_ipcb_options(struct sk_buff *s);
u8 rust_ipt_ipv6_dsfield(const struct ipv6hdr *h);
__be32 rust_ipt_id_to_key(__be64 id);
u32 rust_ipt_skb_hash(struct sk_buff *s);
bool rust_ipt_cache_usable(const struct sk_buff *s, const struct ip_tunnel_info *i);
u8 rust_ipt_ecn_encap(u8 tos, const struct iphdr *h, const struct sk_buff *s);
unsigned int rust_ipt_ll_reserved(const struct net_device *d);
int rust_ipt_cow_head(struct sk_buff *s, unsigned int h);
__be16 rust_ipt_payload_protocol(const struct sk_buff *s);
struct rtable *rust_ipt_skb_rtable(const struct sk_buff *s);
__be32 rust_ipt_rt_nexthop(const struct rtable *r, __be32 d);
void rust_ipt_dst_link_failure(struct sk_buff *s);
void rust_ipt_dev_addr_set(struct net_device *d, const void *a, size_t n);
void rust_ipt_flags_from_be16(unsigned long *f, __be16 v);
__be16 rust_ipt_flags_to_be16(const unsigned long *f);
bool rust_ipt_flags_compat(const unsigned long *f);
unsigned long rust_ipt_copy_from_user(void *d, const void __user *s, unsigned long n);
unsigned long rust_ipt_copy_to_user(void __user *d, const void *s, unsigned long n);
void rust_ipt_set_netns_immutable(struct net_device *d);
void rust_ipt_set_pcpu_stat_type(struct net_device *d);
struct net_device *rust_ipt_first_netdev(struct net *n);
struct net_device *rust_ipt_next_netdev(struct net_device *d);
void rust_ipt_hw_addr_random(struct net_device *d);
void rust_ipt_keep_dst(struct net_device *d);
#if IS_ENABLED(CONFIG_IPV6)
struct rt6_info *rust_ipt_dst_rt6(struct dst_entry *d);
void rust_ipt_dst_metric_set(struct dst_entry *d, int metric, u32 value);
struct neighbour *rust_ipt_dst_neigh(struct dst_entry *d, const void *p);
struct in6_addr *rust_ipt_ipv6_daddr(struct sk_buff *s);
struct in6_addr *rust_ipt_neigh_key(struct neighbour *n);
int rust_ipt_ipv6_addr_type(const struct in6_addr *a);
__be32 rust_ipt_ipv6_word3(const struct in6_addr *a);
void rust_ipt_neigh_release(struct neighbour *n);
#endif
void rust_ipt_multicast(struct net_device *d);
void rust_ipt_rx_crc_errors(struct net_device *d);
void rust_ipt_rx_errors(struct net_device *d);
void rust_ipt_rx_fifo_errors(struct net_device *d);
void rust_ipt_rx_length_errors(struct net_device *d);
void rust_ipt_rx_frame_errors(struct net_device *d);
void rust_ipt_tx_carrier_errors(struct net_device *d);
void rust_ipt_collisions(struct net_device *d);
void rust_ipt_tx_errors(struct net_device *d);
void rust_ipt_tx_dropped(struct net_device *d);
void rust_ipt_tx_fifo_errors(struct net_device *d);
int rust_ipt_hoplimit(const struct dst_entry *d);
void rust_ipt_icmp_send(struct sk_buff *s, int type, int code, __be32 info);
#if IS_ENABLED(CONFIG_IPV6)
void rust_ipt_icmpv6_send(struct sk_buff *s, u8 type, u8 code, u32 info);
#endif
#endif
