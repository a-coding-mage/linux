/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_IP6_GRE_BINDINGS_H
#define LUPOS_IP6_GRE_BINDINGS_H
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
#include <linux/hash.h>
#include <linux/if_tunnel.h>
#include <linux/ip6_tunnel.h>

#include <net/sock.h>
#include <net/ip.h>
#include <net/ip_tunnels.h>
#include <net/icmp.h>
#include <net/protocol.h>
#include <net/addrconf.h>
#include <net/arp.h>
#include <net/checksum.h>
#include <net/dsfield.h>
#include <net/inet_ecn.h>
#include <net/xfrm.h>
#include <net/net_namespace.h>
#include <net/netns/generic.h>
#include <net/netdev_lock.h>
#include <net/rtnetlink.h>

#include <net/ipv6.h>
#include <net/ip6_fib.h>
#include <net/ip6_route.h>
#include <net/ip6_tunnel.h>
#include <net/gre.h>
#include <net/erspan.h>
#include <net/dst_metadata.h>
#define IP6_GRE_HASH_SIZE_SHIFT 5
#define IP6_GRE_HASH_SIZE (1 << IP6_GRE_HASH_SIZE_SHIFT)
struct ip6gre_net {
 struct ip6_tnl __rcu *tunnels[4][IP6_GRE_HASH_SIZE];
 struct ip6_tnl __rcu *collect_md_tun;
 struct ip6_tnl __rcu *collect_md_tun_erspan;
 struct net_device *fb_tunnel_dev;
};
enum { RUST_IP6GRE_FLAG_WORDS = BITS_TO_LONGS(__IP_TUNNEL_FLAG_NUM) };
struct net *rust_ip6gre_dev_net(const struct net_device *dev);
void rust_ip6gre_dev_net_set(struct net_device *dev, struct net *net);
void *rust_ip6gre_net_generic(const struct net *net, unsigned int id);
void *rust_ip6gre_netdev_priv(const struct net_device *dev);
struct net_device *rust_ip6gre_skb_dev(const struct sk_buff *skb);
struct ip6_tnl *rust_ip6gre_rcu_read(struct ip6_tnl * const *p);
struct ip6_tnl *rust_ip6gre_rtnl_read(struct ip6_tnl * const *p);
void rust_ip6gre_rcu_assign(struct ip6_tnl **p, struct ip6_tnl *v);
struct net_device *rust_ip6gre_read_dev(struct net_device * const *p);
void rust_ip6gre_write_dev(struct net_device **p, struct net_device *v);
u32 rust_ip6gre_hash_addr(const struct in6_addr *a);
bool rust_ip6gre_addr_equal(const struct in6_addr *a, const struct in6_addr *b);
bool rust_ip6gre_addr_any(const struct in6_addr *a);
bool rust_ip6gre_addr_multicast(const struct in6_addr *a);
struct net_device *rust_ip6gre_alloc_netdev(const char *name, void (*setup)(struct net_device *));
ssize_t rust_ip6gre_strscpy(char *dst, const char *src, size_t size);
void rust_ip6gre_dst_cache_reset(struct dst_cache *cache);
void rust_ip6gre_netdev_put(struct net_device *dev, netdevice_tracker *tracker);
unsigned long rust_ip6gre_jiffies(void);
unsigned long rust_ip6gre_read_ulong(const unsigned long *p);
void rust_ip6gre_write_ulong(unsigned long *p, unsigned long v);
int rust_ip6gre_read_int(const int *p);
void rust_ip6gre_write_int(int *p, int v);
kuid_t rust_ip6gre_sock_net_uid(const struct net *net);
struct ipv6hdr *rust_ip6gre_ipv6_hdr(const struct sk_buff *skb);
struct iphdr *rust_ip6gre_ip_hdr(const struct sk_buff *skb);
u8 *rust_ip6gre_network_header(const struct sk_buff *skb);
u32 rust_ip6gre_network_header_len(const struct sk_buff *skb);
int rust_ip6gre_network_offset(const struct sk_buff *skb);
int rust_ip6gre_transport_offset(const struct sk_buff *skb);
bool rust_ip6gre_transport_header_was_set(const struct sk_buff *skb);
bool rust_ip6gre_may_pull(struct sk_buff *skb, unsigned int len);
bool rust_ip6gre_inet_may_pull(struct sk_buff *skb);
int rust_ip6gre_trim(struct sk_buff *skb, unsigned int len);
void rust_ip6gre_flags_copy(unsigned long *dst, const unsigned long *src);
void rust_ip6gre_flags_and(unsigned long *dst, const unsigned long *a, const unsigned long *b);
void rust_ip6gre_set_bit(unsigned int bit, unsigned long *flags);
bool rust_ip6gre_test_bit(unsigned int bit, const unsigned long *flags);
u64 rust_ip6gre_key_to_id(u32 key);
u32 rust_ip6gre_id_to_key(u64 id);
struct metadata_dst *rust_ip6gre_rx_dst(struct sk_buff *skb, const unsigned long *flags, u64 id, int md_size);
struct ip_tunnel_info *rust_ip6gre_md_info(struct metadata_dst *md);
void *rust_ip6gre_info_opts(struct ip_tunnel_info *info);
struct ip_tunnel_info *rust_ip6gre_skb_tunnel_info(struct sk_buff *skb);
u16 rust_ip6gre_info_af(const struct ip_tunnel_info *info);
int rust_ip6gre_erspan_hlen(int version);
u8 rust_ip6gre_erspan_ver(const struct erspan_base_hdr *hdr);
void rust_ip6gre_erspan_build(struct sk_buff *skb, u32 id, u32 index, bool truncate);
void rust_ip6gre_erspan_build_v2(struct sk_buff *skb, u32 id, u8 dir, u16 hwid, bool truncate);
u8 rust_ip6gre_md2_dir(const struct erspan_md2 *md);
u8 rust_ip6gre_md2_hwid(const struct erspan_md2 *md);
void *rust_ip6gre_md2(struct erspan_metadata *md);
u32 rust_ip6gre_md_index(const struct erspan_metadata *md);
int rust_ip6gre_pull_header(struct sk_buff *skb, int len, u16 proto);
void rust_ip6gre_icmp6_send(struct sk_buff *skb, u8 type, u8 code, u32 info);
void rust_ip6gre_kfree_skb(struct sk_buff *skb);
void rust_ip6gre_rx_dropped(struct net_device *dev);
void rust_ip6gre_tx_errors(struct net_device *dev);
void rust_ip6gre_tx_dropped(struct net_device *dev);
void rust_ip6gre_ipcb_clear_options(struct sk_buff *skb);
void rust_ip6gre_ipcb_clear_flags(struct sk_buff *skb);
u8 rust_ip6gre_ipv4_dsfield(const struct iphdr *hdr);
u8 rust_ip6gre_ipv6_dsfield(const struct ipv6hdr *hdr);
u8 rust_ip6gre_tclass(u32 flowinfo);
u32 rust_ip6gre_flowlabel(const struct ipv6hdr *hdr);
void rust_ip6gre_flow_mark(struct flowi6 *fl, u32 mark);
void rust_ip6gre_flow_uid(struct flowi6 *fl, kuid_t uid);
void rust_ip6gre_flow_proto(struct flowi6 *fl, u8 proto);
void rust_ip6gre_flow_key(struct flowi6 *fl, u32 key);
u16 rust_ip6gre_skb_protocol(const struct sk_buff *skb, bool skip_vlan);
int rust_ip6gre_cow_head(struct sk_buff *skb, unsigned int headroom);
int rust_ip6gre_gre_hlen(const unsigned long *flags);
void rust_ip6gre_gre_build(struct sk_buff *skb, int hlen, const unsigned long *flags, u16 proto, u32 key, u32 seq);
int rust_ip6gre_atomic_fetch_inc(atomic_t *v);
struct dst_entry *rust_ip6gre_skb_dst(const struct sk_buff *skb);
struct net_device *rust_ip6gre_dst_dev(const struct dst_entry *dst);
unsigned int rust_ip6gre_dst_mtu(const struct dst_entry *dst);
unsigned int rust_ip6gre_read_uint(const unsigned int *p);
bool rust_ip6gre_is_err(const void *p);
void rust_ip6gre_debug_error(int kind, const char *name);
u32 rust_ip6gre_skb_mark(const struct sk_buff *skb);
u16 rust_ip6gre_protocol(const struct sk_buff *skb);
struct in6_addr *rust_ip6gre_saddr(struct ipv6hdr *hdr);
struct in6_addr *rust_ip6gre_daddr(struct ipv6hdr *hdr);
enum { RUST_IP6GRE_IFLA_GRE_MAX = IFLA_GRE_MAX };
/* Control-plane macro, header-inline, and bitfield boundaries. */
enum {
 RUST_IP6GRE_GFP_KERNEL = GFP_KERNEL,
 RUST_IP6GRE_GFP_ATOMIC = GFP_ATOMIC,
 RUST_IP6GRE_LL_MAX_HEADER = LL_MAX_HEADER,
};
void rust_ip6gre_dev_addr_set(struct net_device *dev, const void *addr, size_t len);
void rust_ip6gre_flow_oif(struct flowi6 *flow, int oif);
int rust_ip6gre_addr_type(const struct in6_addr *addr);
unsigned int rust_ip6gre_limit_headroom(unsigned int headroom);
void rust_ip6gre_rt_put(struct rt6_info *rt);
void rust_ip6gre_write_uint(unsigned int *p, unsigned int value);
void rust_ip6gre_flags_from_gre(unsigned long *flags, __be16 gre);
__be16 rust_ip6gre_flags_to_gre(const unsigned long *flags);
__be16 rust_ip6gre_flags_to_be16(const unsigned long *flags);
unsigned long rust_ip6gre_copy_from_user(void *to, const void __user *from, unsigned long len);
unsigned long rust_ip6gre_copy_to_user(void __user *to, const void *from, unsigned long len);
void rust_ip6gre_unregister_netdevice(struct net_device *dev);
unsigned int rust_ip6gre_headroom(const struct sk_buff *skb);
int rust_ip6gre_hh_data_align(int len);
__be32 rust_ip6gre_make_flowlabel(const struct net *net, struct sk_buff *skb, __be32 label, struct flowi6 *flow);
void rust_ip6gre_flow_hdr(struct ipv6hdr *hdr, unsigned int tclass, __be32 label);
void rust_ip6gre_keep_dst(struct net_device *dev);
void rust_ip6gre_random_addr(u8 *addr);
void rust_ip6gre_hw_addr_random(struct net_device *dev);
void rust_ip6gre_set_lltx(struct net_device *dev, bool value);
void rust_ip6gre_set_pcpu_stat_type(struct net_device *dev, unsigned int value);
void rust_ip6gre_set_netns_immutable(struct net_device *dev, bool value);
unsigned long rust_ip6gre_priv_flags(const struct net_device *dev);
void rust_ip6gre_set_priv_flags(struct net_device *dev, unsigned long value);
netdev_features_t rust_ip6gre_gre6_features(void);
netdev_features_t rust_ip6gre_gso_features(void);
void rust_ip6gre_netdev_hold(struct net_device *dev, netdevice_tracker *tracker, gfp_t gfp);
void rust_ip6gre_lockdep_classes(struct net_device *dev);
struct net_device *rust_ip6gre_first_netdev(struct net *net);
struct net_device *rust_ip6gre_next_netdev(struct net_device *dev);
struct ip6_tnl *rust_ip6gre_rtnl_net_read(struct net *net, struct ip6_tnl * const *p);
bool rust_ip6gre_net_eq(const struct net *a, const struct net *b);
bool rust_ip6gre_has_fallback(const struct net *net);
int rust_ip6gre_nla_len(const struct nlattr *attr);
void *rust_ip6gre_nla_data(const struct nlattr *attr);
u8 rust_ip6gre_nla_u8(const struct nlattr *attr);
u16 rust_ip6gre_nla_u16(const struct nlattr *attr);
u32 rust_ip6gre_nla_u32(const struct nlattr *attr);
struct in6_addr rust_ip6gre_nla_in6(const struct nlattr *attr);
int rust_ip6gre_nla_total_size(int payload);
bool rust_ip6gre_valid_ether_addr(const u8 *addr);
void rust_ip6gre_log_driver(void);
void rust_ip6gre_log_add_protocol(void);
bool rust_ip6gre_log_ecn_error(void);
int rust_ip6gre_init(void);
void rust_ip6gre_fini(void);

#endif
