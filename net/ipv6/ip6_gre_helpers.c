// SPDX-License-Identifier: GPL-2.0-or-later
/* Macro/header-inline boundaries. All GRE policy and packet ownership is Rust. */
#define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
#include "ip6_gre_bindings.h"
struct net *rust_ip6gre_dev_net(const struct net_device *d) { return dev_net(d); }
void rust_ip6gre_dev_net_set(struct net_device *d, struct net *n) { dev_net_set(d, n); }
void *rust_ip6gre_net_generic(const struct net *n, unsigned int id) { return net_generic(n, id); }
void *rust_ip6gre_netdev_priv(const struct net_device *d) { return netdev_priv(d); }
struct net_device *rust_ip6gre_skb_dev(const struct sk_buff *s) { return s->dev; }
struct ip6_tnl *rust_ip6gre_rcu_read(struct ip6_tnl * const *p) { return rcu_dereference(*p); }
struct ip6_tnl *rust_ip6gre_rtnl_read(struct ip6_tnl * const *p) { return rtnl_dereference(*p); }
void rust_ip6gre_rcu_assign(struct ip6_tnl **p, struct ip6_tnl *v) { rcu_assign_pointer(*p, v); }
struct net_device *rust_ip6gre_read_dev(struct net_device * const *p) { return READ_ONCE(*p); }
void rust_ip6gre_write_dev(struct net_device **p, struct net_device *v) { WRITE_ONCE(*p, v); }
u32 rust_ip6gre_hash_addr(const struct in6_addr *a) { return hash_32(ipv6_addr_hash(a), IP6_GRE_HASH_SIZE_SHIFT); }
bool rust_ip6gre_addr_equal(const struct in6_addr *a, const struct in6_addr *b) { return ipv6_addr_equal(a, b); }
bool rust_ip6gre_addr_any(const struct in6_addr *a) { return ipv6_addr_any(a); }
bool rust_ip6gre_addr_multicast(const struct in6_addr *a) { return ipv6_addr_is_multicast(a); }
struct net_device *rust_ip6gre_alloc_netdev(const char *n, void (*s)(struct net_device *)) { return alloc_netdev(sizeof(struct ip6_tnl), n, NET_NAME_UNKNOWN, s); }
ssize_t rust_ip6gre_strscpy(char *d, const char *s, size_t n) { return strscpy(d, s, n); }
void rust_ip6gre_dst_cache_reset(struct dst_cache *c) { dst_cache_reset(c); }
void rust_ip6gre_netdev_put(struct net_device *d, netdevice_tracker *t) { netdev_put(d, t); }
unsigned long rust_ip6gre_jiffies(void) { return jiffies; }
unsigned long rust_ip6gre_read_ulong(const unsigned long *p) { return READ_ONCE(*p); }
void rust_ip6gre_write_ulong(unsigned long *p, unsigned long v) { WRITE_ONCE(*p, v); }
int rust_ip6gre_read_int(const int *p) { return READ_ONCE(*p); }
void rust_ip6gre_write_int(int *p, int v) { WRITE_ONCE(*p, v); }
kuid_t rust_ip6gre_sock_net_uid(const struct net *n) { return sock_net_uid(n, NULL); }
struct ipv6hdr *rust_ip6gre_ipv6_hdr(const struct sk_buff *s) { return ipv6_hdr(s); }
struct iphdr *rust_ip6gre_ip_hdr(const struct sk_buff *s) { return ip_hdr(s); }
u8 *rust_ip6gre_network_header(const struct sk_buff *s) { return skb_network_header(s); }
u32 rust_ip6gre_network_header_len(const struct sk_buff *s) { return skb_network_header_len(s); }
int rust_ip6gre_network_offset(const struct sk_buff *s) { return skb_network_offset(s); }
int rust_ip6gre_transport_offset(const struct sk_buff *s) { return skb_transport_offset(s); }
bool rust_ip6gre_transport_header_was_set(const struct sk_buff *s) { return skb_transport_header_was_set(s); }
bool rust_ip6gre_may_pull(struct sk_buff *s, unsigned int n) { return pskb_may_pull(s, n); }
bool rust_ip6gre_inet_may_pull(struct sk_buff *s) { return pskb_inet_may_pull(s); }
int rust_ip6gre_trim(struct sk_buff *s, unsigned int n) { return pskb_trim(s, n); }
void rust_ip6gre_flags_copy(unsigned long *d, const unsigned long *s) { ip_tunnel_flags_copy(d, s); }
void rust_ip6gre_flags_and(unsigned long *d, const unsigned long *a, const unsigned long *b) { ip_tunnel_flags_and(d, a, b); }
void rust_ip6gre_set_bit(unsigned int b, unsigned long *f) { __set_bit(b, f); }
bool rust_ip6gre_test_bit(unsigned int b, const unsigned long *f) { return test_bit(b, f); }
u64 rust_ip6gre_key_to_id(u32 k) { return key32_to_tunnel_id(k); }
u32 rust_ip6gre_id_to_key(u64 k) { return tunnel_id_to_key32(k); }
struct metadata_dst *rust_ip6gre_rx_dst(struct sk_buff *s, const unsigned long *f, u64 id, int n) { return ipv6_tun_rx_dst(s, f, id, n); }
struct ip_tunnel_info *rust_ip6gre_md_info(struct metadata_dst *m) { return &m->u.tun_info; }
void *rust_ip6gre_info_opts(struct ip_tunnel_info *i) { return ip_tunnel_info_opts(i); }
struct ip_tunnel_info *rust_ip6gre_skb_tunnel_info(struct sk_buff *s) { return skb_tunnel_info(s); }
u16 rust_ip6gre_info_af(const struct ip_tunnel_info *i) { return ip_tunnel_info_af(i); }
int rust_ip6gre_erspan_hlen(int v) { return erspan_hdr_len(v); }
u8 rust_ip6gre_erspan_ver(const struct erspan_base_hdr *h) { return h->ver; }
void rust_ip6gre_erspan_build(struct sk_buff *s, u32 id, u32 index, bool t) { erspan_build_header(s, id, index, t, false); }
void rust_ip6gre_erspan_build_v2(struct sk_buff *s, u32 id, u8 dir, u16 hwid, bool t) { erspan_build_header_v2(s, id, dir, hwid, t, false); }
u8 rust_ip6gre_md2_dir(const struct erspan_md2 *m) { return m->dir; }
u8 rust_ip6gre_md2_hwid(const struct erspan_md2 *m) { return get_hwid(m); }
void *rust_ip6gre_md2(struct erspan_metadata *m) { return &m->u.md2; }
u32 rust_ip6gre_md_index(const struct erspan_metadata *m) { return m->u.index; }
int rust_ip6gre_pull_header(struct sk_buff *s, int n, u16 p) { return iptunnel_pull_header(s, n, p, false); }
void rust_ip6gre_icmp6_send(struct sk_buff *s, u8 t, u8 c, u32 i) { icmpv6_send(s, t, c, i); }
void rust_ip6gre_kfree_skb(struct sk_buff *s) { kfree_skb(s); }
void rust_ip6gre_rx_dropped(struct net_device *d) { dev_core_stats_rx_dropped_inc(d); }
void rust_ip6gre_tx_errors(struct net_device *d) { DEV_STATS_INC(d, tx_errors); }
void rust_ip6gre_tx_dropped(struct net_device *d) { DEV_STATS_INC(d, tx_dropped); }
void rust_ip6gre_ipcb_clear_options(struct sk_buff *s) { memset(&IPCB(s)->opt, 0, sizeof(IPCB(s)->opt)); }
void rust_ip6gre_ipcb_clear_flags(struct sk_buff *s) { IPCB(s)->flags = 0; }
u8 rust_ip6gre_ipv4_dsfield(const struct iphdr *h) { return ipv4_get_dsfield(h); }
u8 rust_ip6gre_ipv6_dsfield(const struct ipv6hdr *h) { return ipv6_get_dsfield(h); }
u8 rust_ip6gre_tclass(u32 f) { return ip6_tclass(f); }
u32 rust_ip6gre_flowlabel(const struct ipv6hdr *h) { return ip6_flowlabel(h); }
void rust_ip6gre_flow_mark(struct flowi6 *f, u32 m) { f->flowi6_mark = m; }
void rust_ip6gre_flow_uid(struct flowi6 *f, kuid_t u) { f->flowi6_uid = u; }
void rust_ip6gre_flow_proto(struct flowi6 *f, u8 p) { f->flowi6_proto = p; }
void rust_ip6gre_flow_key(struct flowi6 *f, u32 k) { f->fl6_gre_key = k; }
u16 rust_ip6gre_skb_protocol(const struct sk_buff *s, bool skip) { return skb_protocol(s, skip); }
int rust_ip6gre_cow_head(struct sk_buff *s, unsigned int n) { return skb_cow_head(s, n); }
int rust_ip6gre_gre_hlen(const unsigned long *f) { return gre_calc_hlen(f); }
void rust_ip6gre_gre_build(struct sk_buff *s, int h, const unsigned long *f, u16 p, u32 k, u32 q) { gre_build_header(s, h, f, p, k, q); }
int rust_ip6gre_atomic_fetch_inc(atomic_t *v) { return atomic_fetch_inc(v); }
struct dst_entry *rust_ip6gre_skb_dst(const struct sk_buff *s) { return skb_dst(s); }
struct net_device *rust_ip6gre_dst_dev(const struct dst_entry *d) { return dst_dev(d); }
unsigned int rust_ip6gre_dst_mtu(const struct dst_entry *d) { return dst_mtu(d); }
unsigned int rust_ip6gre_read_uint(const unsigned int *p) { return READ_ONCE(*p); }
bool rust_ip6gre_is_err(const void *p) { return IS_ERR(p); }
void rust_ip6gre_debug_error(int k, const char *n)
{
 switch (k) {
 case 0: net_dbg_ratelimited("%s: Path to destination invalid or inactive!\n", n); break;
 case 1: net_dbg_ratelimited("%s: Too small hop limit or routing loop in tunnel!\n", n); break;
 case 2: net_dbg_ratelimited("%s: Too small encapsulation limit or routing loop in tunnel!\n", n); break;
 case 3: net_dbg_ratelimited("%s: Recipient unable to parse tunneled packet!\n", n); break;
 }
}
u32 rust_ip6gre_skb_mark(const struct sk_buff *s) { return s->mark; }
u16 rust_ip6gre_protocol(const struct sk_buff *s) { return s->protocol; }
struct in6_addr *rust_ip6gre_saddr(struct ipv6hdr *h) { return &h->saddr; }
struct in6_addr *rust_ip6gre_daddr(struct ipv6hdr *h) { return &h->daddr; }
/* Appended to ip6_gre_helpers.c; no independent policy or algorithms. */
void rust_ip6gre_dev_addr_set(struct net_device *d, const void *a, size_t n) { __dev_addr_set(d, a, n); }
void rust_ip6gre_flow_oif(struct flowi6 *f, int i) { f->flowi6_oif = i; }
int rust_ip6gre_addr_type(const struct in6_addr *a) { return ipv6_addr_type(a); }
unsigned int rust_ip6gre_limit_headroom(unsigned int h) { return ip_tunnel_limit_headroom(h); }
void rust_ip6gre_rt_put(struct rt6_info *r) { ip6_rt_put(r); }
void rust_ip6gre_write_uint(unsigned int *p, unsigned int v) { WRITE_ONCE(*p, v); }
void rust_ip6gre_flags_from_gre(unsigned long *f, __be16 g) { gre_flags_to_tnl_flags(f, g); }
__be16 rust_ip6gre_flags_to_gre(const unsigned long *f) { return gre_tnl_flags_to_gre_flags(f); }
__be16 rust_ip6gre_flags_to_be16(const unsigned long *f) { return ip_tunnel_flags_to_be16(f); }
unsigned long rust_ip6gre_copy_from_user(void *d, const void __user *s, unsigned long n) { return copy_from_user(d, s, n); }
unsigned long rust_ip6gre_copy_to_user(void __user *d, const void *s, unsigned long n) { return copy_to_user(d, s, n); }
void rust_ip6gre_unregister_netdevice(struct net_device *d) { unregister_netdevice(d); }
unsigned int rust_ip6gre_headroom(const struct sk_buff *s) { return skb_headroom(s); }
int rust_ip6gre_hh_data_align(int n) { return HH_DATA_ALIGN(n); }
__be32 rust_ip6gre_make_flowlabel(const struct net *n, struct sk_buff *s, __be32 l, struct flowi6 *f) { return ip6_make_flowlabel(n, s, l, true, f); }
void rust_ip6gre_flow_hdr(struct ipv6hdr *h, unsigned int c, __be32 l) { ip6_flow_hdr(h, c, l); }
void rust_ip6gre_keep_dst(struct net_device *d) { netif_keep_dst(d); }
void rust_ip6gre_random_addr(u8 *a) { eth_random_addr(a); }
void rust_ip6gre_hw_addr_random(struct net_device *d) { eth_hw_addr_random(d); }
void rust_ip6gre_set_lltx(struct net_device *d, bool v) { d->lltx = v; }
void rust_ip6gre_set_pcpu_stat_type(struct net_device *d, unsigned int v) { d->pcpu_stat_type = v; }
void rust_ip6gre_set_netns_immutable(struct net_device *d, bool v) { d->netns_immutable = v; }
unsigned long rust_ip6gre_priv_flags(const struct net_device *d) { return d->priv_flags; }
void rust_ip6gre_set_priv_flags(struct net_device *d, unsigned long v) { d->priv_flags = v; }
netdev_features_t rust_ip6gre_gre6_features(void) { return NETIF_F_SG | NETIF_F_FRAGLIST | NETIF_F_HIGHDMA | NETIF_F_HW_CSUM; }
netdev_features_t rust_ip6gre_gso_features(void) { return NETIF_F_GSO_SOFTWARE; }
void rust_ip6gre_netdev_hold(struct net_device *d, netdevice_tracker *t, gfp_t g) { netdev_hold(d, t, g); }
void rust_ip6gre_lockdep_classes(struct net_device *d) { netdev_lockdep_set_classes(d); }
struct net_device *rust_ip6gre_first_netdev(struct net *n) { return first_net_device(n); }
struct net_device *rust_ip6gre_next_netdev(struct net_device *d) { return next_net_device(d); }
struct ip6_tnl *rust_ip6gre_rtnl_net_read(struct net *n, struct ip6_tnl * const *p) { return rtnl_net_dereference(n, *p); }
bool rust_ip6gre_net_eq(const struct net *a, const struct net *b) { return net_eq(a, b); }
bool rust_ip6gre_has_fallback(const struct net *n) { return net_has_fallback_tunnels(n); }
int rust_ip6gre_nla_len(const struct nlattr *a) { return nla_len(a); }
void *rust_ip6gre_nla_data(const struct nlattr *a) { return nla_data(a); }
u8 rust_ip6gre_nla_u8(const struct nlattr *a) { return nla_get_u8(a); }
u16 rust_ip6gre_nla_u16(const struct nlattr *a) { return nla_get_u16(a); }
u32 rust_ip6gre_nla_u32(const struct nlattr *a) { return nla_get_u32(a); }
struct in6_addr rust_ip6gre_nla_in6(const struct nlattr *a) { return nla_get_in6_addr(a); }
int rust_ip6gre_nla_total_size(int n) { return nla_total_size(n); }
bool rust_ip6gre_valid_ether_addr(const u8 *a) { return is_valid_ether_addr(a); }
void rust_ip6gre_log_driver(void) { pr_info("GRE over IPv6 tunneling driver\n"); }
void rust_ip6gre_log_add_protocol(void) { pr_info("ip6gre_init: can't add protocol\n"); }

static bool log_ecn_error = true;
module_param(log_ecn_error, bool, 0644);
MODULE_PARM_DESC(log_ecn_error, "Log packets received with corrupted ECN");
bool rust_ip6gre_log_ecn_error(void) { return log_ecn_error; }
static int __init ip6gre_init(void) { return rust_ip6gre_init(); }
static void __exit ip6gre_fini(void) { rust_ip6gre_fini(); }
module_init(ip6gre_init);
module_exit(ip6gre_fini);
MODULE_LICENSE("GPL");
MODULE_AUTHOR("D. Kozlov <xeb@mail.ru>");
MODULE_DESCRIPTION("GRE over IPv6 tunneling device");
MODULE_ALIAS_RTNL_LINK("ip6gre");
MODULE_ALIAS_RTNL_LINK("ip6gretap");
MODULE_ALIAS_RTNL_LINK("ip6erspan");
MODULE_ALIAS_NETDEV("ip6gre0");
