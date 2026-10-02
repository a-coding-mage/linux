// SPDX-License-Identifier: GPL-2.0-or-later
/* Configured declarations, macros and header-inline boundaries only.
 * Socket option validation, dispatch, state changes and lifetimes live in Rust. */
#include "ipv6_sockglue_bindings.h"
DEFINE_RWLOCK(ip6_ra_lock);
DEFINE_STATIC_KEY_FALSE(ip6_min_hopcount);
EXPORT_SYMBOL(ipv6_setsockopt);
EXPORT_SYMBOL(ipv6_getsockopt);
struct ipv6_pinfo *rust_ip6_np(struct sock *sk) { return inet6_sk(sk); }
struct net *rust_ip6_net(struct sock *sk) { return sock_net(sk); }
struct user_namespace *rust_ip6_user_ns(struct net *net) { return net->user_ns; }
int rust_ip6_optmem_max(struct net *net) { return READ_ONCE(net->core.sysctl_optmem_max); }
int rust_ip6_default_hoplimit(struct net *net) { return READ_ONCE(net->ipv6.devconf_all->hop_limit); }
unsigned int rust_ip6_sk_type(struct sock *sk) { return sk->sk_type; }
unsigned int rust_ip6_sk_protocol(struct sock *sk) { return sk->sk_protocol; }
unsigned int rust_ip6_sk_state(struct sock *sk) { return sk->sk_state; }
unsigned int rust_ip6_sk_family(struct sock *sk) { return sk->sk_family; }
unsigned int rust_ip6_inet_num(struct sock *sk) { return inet_sk(sk)->inet_num; }
unsigned int rust_ip6_inet_daddr(struct sock *sk) { return inet_sk(sk)->inet_daddr; }
int rust_ip6_bound_dev(struct sock *sk) { return READ_ONCE(sk->sk_bound_dev_if); }
u32 rust_ip6_mark(struct sock *sk) { return sk->sk_mark; }
struct in6_addr *rust_ip6_daddr(struct sock *sk) { return &sk->sk_v6_daddr; }
bool rust_ip6_only(struct sock *sk) { return ipv6_only_sock(sk); }
void rust_ip6_set_only(struct sock *sk, bool val) { sk->sk_ipv6only = val; }
struct proto *rust_ip6_prot(struct sock *sk) { return sk->sk_prot; }
void rust_ip6_set_prot(struct sock *sk, struct proto *p) { WRITE_ONCE(sk->sk_prot, p); }
void rust_ip6_set_ops(struct sock *sk, const struct proto_ops *p) { WRITE_ONCE(sk->sk_socket->ops, p); }
void rust_ip6_set_family(struct sock *sk, unsigned short family) { WRITE_ONCE(sk->sk_family, family); }
int rust_ip6_udp_pending(struct sock *sk) { return udp_sk(sk)->pending; }
u32 rust_ip6_pmtu_cookie(struct sock *sk) { return inet_csk(sk)->icsk_pmtu_cookie; }
void rust_ip6_set_ext_hdr_len(struct sock *sk, u16 len) { inet_csk(sk)->icsk_ext_hdr_len = len; }
void rust_ip6_sync_mss(struct sock *sk, u32 cookie) { inet_csk(sk)->icsk_sync_mss(sk, cookie); }
void rust_ip6_set_af_ops(struct sock *sk, const struct inet_connection_sock_af_ops *p) { WRITE_ONCE(inet_csk(sk)->icsk_af_ops, p); }
u32 rust_ip6_psp_overhead(struct sock *sk) { return psp_sk_overhead(sk); }
bool rust_ip6_test_bit(struct sock *sk, int bit) { return test_bit(bit, &inet_sk(sk)->inet_flags); }
void rust_ip6_assign_bit(struct sock *sk, int bit, bool val) { assign_bit(bit, &inet_sk(sk)->inet_flags, val); }
void rust_ip6_ra_lock(void) { write_lock_bh(&ip6_ra_lock); }
void rust_ip6_ra_unlock(void) { write_unlock_bh(&ip6_ra_lock); }
void rust_ip6_sock_hold(struct sock *sk) { sock_hold(sk); }
void rust_ip6_sock_put(struct sock *sk) { sock_put(sk); }
void rust_ip6_rcu_lock(void) { rcu_read_lock(); }
void rust_ip6_rcu_unlock(void) { rcu_read_unlock(); }
void rust_ip6_lock_sock(struct sock *sk) { sockopt_lock_sock(sk); }
void rust_ip6_release_sock(struct sock *sk) { sockopt_release_sock(sk); }
void rust_ip6_dst_reset(struct sock *sk) { sk_dst_reset(sk); }
struct dst_entry *rust_ip6_dst_get(struct sock *sk) { return __sk_dst_get(sk); }
u32 rust_ip6_dst_mtu(struct dst_entry *dst) { return dst6_mtu(dst); }
struct ipv6_txoptions *rust_ip6_opt_get_locked(struct sock *sk) { return rcu_dereference_protected(inet6_sk(sk)->opt, lockdep_sock_is_held(sk)); }
struct ipv6_txoptions *rust_ip6_opt_xchg(struct sock *sk, struct ipv6_txoptions *opt) { return unrcu_pointer(xchg(&inet6_sk(sk)->opt, RCU_INITIALIZER(opt))); }
void rust_ip6_omem_sub(struct sock *sk, int len) { atomic_sub(len, &sk->sk_omem_alloc); }
void rust_ip6_txopt_put(struct ipv6_txoptions *opt) { txopt_put(opt); }
void rust_ip6_refcount_set(refcount_t *ref, int val) { refcount_set(ref, val); }
void *rust_ip6_kmalloc(size_t size) { return kmalloc(size, GFP_KERNEL); }
void *rust_ip6_sock_kmalloc(struct sock *sk, size_t size) { return sock_kmalloc(sk, size, GFP_KERNEL); }
void *rust_ip6_memdup(sockptr_t p, size_t len) { return memdup_sockptr(p, len); }
int rust_ip6_copy_from(void *dst, sockptr_t src, size_t len) { return copy_from_sockptr(dst, src, len); }
int rust_ip6_copy_to(sockptr_t dst, const void *src, size_t len) { return copy_to_sockptr(dst, src, len); }
int rust_ip6_copy_to_offset(sockptr_t dst, size_t off, const void *src, size_t len) { return copy_to_sockptr_offset(dst, off, src, len); }
sockptr_t rust_ip6_user_sockptr(void __user *p) { return USER_SOCKPTR(p); }
bool rust_ip6_sockptr_null(sockptr_t p) { return sockptr_is_null(p); }
bool rust_ip6_compat(void) { return in_compat_syscall(); }
bool rust_ip6_mroute_opt(int optname) { return ip6_mroute_opt(optname); }
int rust_ip6_mroute_set(struct sock *sk, int optname, sockptr_t optval, unsigned int optlen) { return ip6_mroute_setsockopt(sk, optname, optval, optlen); }
int rust_ip6_mroute_get(struct sock *sk, int optname, sockptr_t optval, sockptr_t optlen) { return ip6_mroute_getsockopt(sk, optname, optval, optlen); }
void rust_ip6_min_hopcount_enable(void) { static_branch_enable(&ip6_min_hopcount); }
void rust_ip6_errqueue_purge(struct sock *sk) { skb_errqueue_purge(&sk->sk_error_queue); }
int rust_ip6_l3master(struct net_device *dev) { return l3mdev_master_ifindex_rcu(dev); }
void rust_ip6_dev_put(struct net_device *dev) { dev_put(dev); }
bool rust_ip6_dev_equal(struct sock *sk, int ifindex) { return sk_dev_equal_l3scope(sk, ifindex); }
bool rust_ip6_v4mapped(const struct in6_addr *addr) { return ipv6_addr_v4mapped(addr); }
bool rust_ip6_autoflowlabel(struct net *net, struct sock *sk) { return ip6_autoflowlabel(net, sk); }
void rust_ip6_module_put(void) { module_put(THIS_MODULE); }
int rust_ip6_xfrm_policy(struct sock *sk, int optname, sockptr_t optval, int optlen) { return xfrm_user_policy(sk, optname, optval, optlen); }
int rust_ip6_get_user_int(int __user *p, int *v) { return get_user(*v, p); }
int rust_ip6_put_user_int(int __user *p, int v) { return put_user(v, p); }
void rust_ip6_prot_inuse_add(struct net *net, struct proto *prot, int val) { sock_prot_inuse_add(net, prot, val); }
int rust_ip6_set_addr_preferences(struct sock *sk, int val) { return ip6_sock_set_addr_preferences(sk, val); }
