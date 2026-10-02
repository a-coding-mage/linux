/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_IPV6_SOCKGLUE_BINDINGS_H
#define LUPOS_IPV6_SOCKGLUE_BINDINGS_H
#include <linux/module.h>
#include <linux/capability.h>
#include <linux/errno.h>
#include <linux/socket.h>
#include <linux/mroute6.h>
#include <linux/netdevice.h>
#include <linux/netfilter.h>
#include <linux/slab.h>
#include <net/sock.h>
#include <net/ipv6.h>
#include <net/ndisc.h>
#include <net/protocol.h>
#include <net/transp_v6.h>
#include <net/ip6_route.h>
#include <net/addrconf.h>
#include <net/inet_common.h>
#include <net/tcp.h>
#include <net/udp.h>
#include <net/xfrm.h>
#include <net/compat.h>
#include <net/seg6.h>
#include <net/psp.h>
#include <linux/uaccess.h>

enum {
 RUST_IP6_GFP_KERNEL = GFP_KERNEL,
 RUST_IP6_SOCK_RAW = SOCK_RAW,
 RUST_IP6_SOCK_STREAM = SOCK_STREAM,
 RUST_IP6_LOOPBACK4_IPV6 = LOOPBACK4_IPV6,
 RUST_IP6_GROUP_FILTER_SIZE0 = GROUP_FILTER_SIZE(0),
 RUST_IP6_GF_FLEX_OFFSET = offsetof(struct group_filter, gf_slist_flex),
 RUST_IP6_COMPAT_GF_FLEX_OFFSET = offsetof(struct compat_group_filter, gf_slist_flex),
 RUST_IP6_COMPAT_GF_MODE_OFFSET = offsetof(struct compat_group_filter, gf_fmode),
 RUST_IP6_COMPAT_GF_NUMSRC_OFFSET = offsetof(struct compat_group_filter, gf_numsrc),
};
struct ipv6_pinfo *rust_ip6_np(struct sock *sk);
struct net *rust_ip6_net(struct sock *sk);
struct user_namespace *rust_ip6_user_ns(struct net *net);
int rust_ip6_optmem_max(struct net *net);
int rust_ip6_default_hoplimit(struct net *net);
unsigned int rust_ip6_sk_type(struct sock *sk);
unsigned int rust_ip6_sk_protocol(struct sock *sk);
unsigned int rust_ip6_sk_state(struct sock *sk);
unsigned int rust_ip6_sk_family(struct sock *sk);
unsigned int rust_ip6_inet_num(struct sock *sk);
unsigned int rust_ip6_inet_daddr(struct sock *sk);
int rust_ip6_bound_dev(struct sock *sk);
u32 rust_ip6_mark(struct sock *sk);
struct in6_addr *rust_ip6_daddr(struct sock *sk);
bool rust_ip6_only(struct sock *sk);
void rust_ip6_set_only(struct sock *sk, bool val);
struct proto *rust_ip6_prot(struct sock *sk);
void rust_ip6_set_prot(struct sock *sk, struct proto *p);
void rust_ip6_set_ops(struct sock *sk, const struct proto_ops *p);
void rust_ip6_set_family(struct sock *sk, unsigned short family);
int rust_ip6_udp_pending(struct sock *sk);
u32 rust_ip6_pmtu_cookie(struct sock *sk);
void rust_ip6_set_ext_hdr_len(struct sock *sk, u16 len);
void rust_ip6_sync_mss(struct sock *sk, u32 cookie);
void rust_ip6_set_af_ops(struct sock *sk, const struct inet_connection_sock_af_ops *p);
u32 rust_ip6_psp_overhead(struct sock *sk);
bool rust_ip6_test_bit(struct sock *sk, int bit);
void rust_ip6_assign_bit(struct sock *sk, int bit, bool val);
void rust_ip6_ra_lock(void);
void rust_ip6_ra_unlock(void);
void rust_ip6_sock_hold(struct sock *sk);
void rust_ip6_sock_put(struct sock *sk);
void rust_ip6_rcu_lock(void);
void rust_ip6_rcu_unlock(void);
void rust_ip6_lock_sock(struct sock *sk);
void rust_ip6_release_sock(struct sock *sk);
void rust_ip6_dst_reset(struct sock *sk);
struct dst_entry *rust_ip6_dst_get(struct sock *sk);
u32 rust_ip6_dst_mtu(struct dst_entry *dst);
struct ipv6_txoptions *rust_ip6_opt_get_locked(struct sock *sk);
struct ipv6_txoptions *rust_ip6_opt_xchg(struct sock *sk, struct ipv6_txoptions *opt);
void rust_ip6_omem_sub(struct sock *sk, int len);
void rust_ip6_txopt_put(struct ipv6_txoptions *opt);
void rust_ip6_refcount_set(refcount_t *ref, int val);
void *rust_ip6_kmalloc(size_t size);
void *rust_ip6_sock_kmalloc(struct sock *sk, size_t size);
void *rust_ip6_memdup(sockptr_t p, size_t len);
int rust_ip6_copy_from(void *dst, sockptr_t src, size_t len);
int rust_ip6_copy_to(sockptr_t dst, const void *src, size_t len);
int rust_ip6_copy_to_offset(sockptr_t dst, size_t off, const void *src, size_t len);
sockptr_t rust_ip6_user_sockptr(void __user *p);
bool rust_ip6_sockptr_null(sockptr_t p);
bool rust_ip6_compat(void);
bool rust_ip6_mroute_opt(int optname);
int rust_ip6_mroute_set(struct sock *sk, int optname, sockptr_t optval, unsigned int optlen);
int rust_ip6_mroute_get(struct sock *sk, int optname, sockptr_t optval, sockptr_t optlen);
void rust_ip6_min_hopcount_enable(void);
void rust_ip6_errqueue_purge(struct sock *sk);
int rust_ip6_l3master(struct net_device *dev);
void rust_ip6_dev_put(struct net_device *dev);
bool rust_ip6_dev_equal(struct sock *sk, int ifindex);
bool rust_ip6_v4mapped(const struct in6_addr *addr);
bool rust_ip6_autoflowlabel(struct net *net, struct sock *sk);
void rust_ip6_module_put(void);
int rust_ip6_set_addr_preferences(struct sock *sk, int val);
void rust_ip6_prot_inuse_add(struct net *net, struct proto *prot, int val);
int rust_ip6_xfrm_policy(struct sock *sk, int optname, sockptr_t optval, int optlen);
int rust_ip6_get_user_int(int __user *p, int *v);
int rust_ip6_put_user_int(int __user *p, int v);
#endif
