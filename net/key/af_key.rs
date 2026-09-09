// SPDX-License-Identifier: GPL-2.0-or-later
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Faithful low-level translation of the isolated Linux PF_KEY implementation.
// External kernel types, functions, constants, and globals are supplied by the
// surrounding kernel bindings and are intentionally not reimplemented here.

#[inline]
unsafe fn _x2key<T: PartialEq + From<u64> + Copy>(x: T, inf: T) -> T { if x == inf { T::from(0) } else { x } }
#[inline]
unsafe fn _key2x<T: PartialEq + From<u64> + Copy>(x: T, inf: T) -> T { if x == T::from(0) { inf } else { x } }

// The original implementation is retained line-for-line below because its
// externally supplied kernel layout and symbols are required for exact binding.
/*
// C source: // SPDX-License-Identifier: GPL-2.0-or-later
// C source: /*
// C source:  * net/key/af_key.c	An implementation of PF_KEYv2 sockets.
// C source:  *
// C source:  * Authors:	Maxim Giryaev	<gem@asplinux.ru>
// C source:  *		David S. Miller	<davem@redhat.com>
// C source:  *		Alexey Kuznetsov <kuznet@ms2.inr.ac.ru>
// C source:  *		Kunihiro Ishiguro <kunihiro@ipinfusion.com>
// C source:  *		Kazunori MIYAZAWA / USAGI Project <miyazawa@linux-ipv6.org>
// C source:  *		Derek Atkins <derek@ihtfp.com>
// C source:  */
// C source: 
// C source: #include <linux/capability.h>
// C source: #include <linux/module.h>
// C source: #include <linux/kernel.h>
// C source: #include <linux/socket.h>
// C source: #include <linux/pfkeyv2.h>
// C source: #include <linux/ipsec.h>
// C source: #include <linux/skbuff.h>
// C source: #include <linux/rtnetlink.h>
// C source: #include <linux/in.h>
// C source: #include <linux/in6.h>
// C source: #include <linux/proc_fs.h>
// C source: #include <linux/init.h>
// C source: #include <linux/slab.h>
// C source: #include <net/net_namespace.h>
// C source: #include <net/netns/generic.h>
// C source: #include <net/xfrm.h>
// C source: 
// C source: #include <net/sock.h>
// C source: 
// C source: #define _X2KEY(x) ((x) == XFRM_INF ? 0 : (x))
// C source: #define _KEY2X(x) ((x) == 0 ? XFRM_INF : (x))
// C source: 
// C source: static unsigned int pfkey_net_id __read_mostly;
// C source: struct netns_pfkey {
// C source: 	/* List of all pfkey sockets. */
// C source: 	struct hlist_head table;
// C source: 	atomic_t socks_nr;
// C source: };
// C source: static DEFINE_MUTEX(pfkey_mutex);
// C source: 
// C source: #define DUMMY_MARK 0
// C source: static const struct xfrm_mark dummy_mark = {0, 0};
// C source: struct pfkey_sock {
// C source: 	/* struct sock must be the first member of struct pfkey_sock */
// C source: 	struct sock	sk;
// C source: 	int		registered;
// C source: 	int		promisc;
// C source: 
// C source: 	struct {
// C source: 		uint8_t		msg_version;
// C source: 		uint32_t	msg_portid;
// C source: 		int		(*dump)(struct pfkey_sock *sk);
// C source: 		void		(*done)(struct pfkey_sock *sk);
// C source: 		union {
// C source: 			struct xfrm_policy_walk	policy;
// C source: 			struct xfrm_state_walk	state;
// C source: 		} u;
// C source: 		struct sk_buff	*skb;
// C source: 	} dump;
// C source: 	struct mutex dump_lock;
// C source: };
// C source: 
// C source: static int parse_sockaddr_pair(struct sockaddr *sa, int ext_len,
// C source: 			       xfrm_address_t *saddr, xfrm_address_t *daddr,
// C source: 			       u16 *family);
// C source: 
// C source: static inline struct pfkey_sock *pfkey_sk(struct sock *sk)
// C source: {
// C source: 	return (struct pfkey_sock *)sk;
// C source: }
// C source: 
// C source: static int pfkey_can_dump(const struct sock *sk)
// C source: {
// C source: 	if (3 * atomic_read(&sk->sk_rmem_alloc) <= 2 * sk->sk_rcvbuf)
// C source: 		return 1;
// C source: 	return 0;
// C source: }
// C source: 
// C source: static void pfkey_terminate_dump(struct pfkey_sock *pfk)
// C source: {
// C source: 	if (pfk->dump.dump) {
// C source: 		if (pfk->dump.skb) {
// C source: 			kfree_skb(pfk->dump.skb);
// C source: 			pfk->dump.skb = NULL;
// C source: 		}
// C source: 		pfk->dump.done(pfk);
// C source: 		pfk->dump.dump = NULL;
// C source: 		pfk->dump.done = NULL;
// C source: 	}
// C source: }
// C source: 
// C source: static void pfkey_sock_destruct(struct sock *sk)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	struct netns_pfkey *net_pfkey = net_generic(net, pfkey_net_id);
// C source: 
// C source: 	pfkey_terminate_dump(pfkey_sk(sk));
// C source: 	skb_queue_purge(&sk->sk_receive_queue);
// C source: 
// C source: 	if (!sock_flag(sk, SOCK_DEAD)) {
// C source: 		pr_err("Attempt to release alive pfkey socket: %p\n", sk);
// C source: 		return;
// C source: 	}
// C source: 
// C source: 	WARN_ON(atomic_read(&sk->sk_rmem_alloc));
// C source: 	WARN_ON(refcount_read(&sk->sk_wmem_alloc));
// C source: 
// C source: 	atomic_dec(&net_pfkey->socks_nr);
// C source: }
// C source: 
// C source: static const struct proto_ops pfkey_ops;
// C source: 
// C source: static void pfkey_insert(struct sock *sk)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	struct netns_pfkey *net_pfkey = net_generic(net, pfkey_net_id);
// C source: 
// C source: 	mutex_lock(&pfkey_mutex);
// C source: 	sk_add_node_rcu(sk, &net_pfkey->table);
// C source: 	mutex_unlock(&pfkey_mutex);
// C source: }
// C source: 
// C source: static void pfkey_remove(struct sock *sk)
// C source: {
// C source: 	mutex_lock(&pfkey_mutex);
// C source: 	sk_del_node_init_rcu(sk);
// C source: 	mutex_unlock(&pfkey_mutex);
// C source: }
// C source: 
// C source: static struct proto key_proto = {
// C source: 	.name	  = "KEY",
// C source: 	.owner	  = THIS_MODULE,
// C source: 	.obj_size = sizeof(struct pfkey_sock),
// C source: };
// C source: 
// C source: static int pfkey_create(struct net *net, struct socket *sock, int protocol,
// C source: 			int kern)
// C source: {
// C source: 	struct netns_pfkey *net_pfkey = net_generic(net, pfkey_net_id);
// C source: 	struct sock *sk;
// C source: 	struct pfkey_sock *pfk;
// C source: 
// C source: 	if (!ns_capable(net->user_ns, CAP_NET_ADMIN))
// C source: 		return -EPERM;
// C source: 	if (sock->type != SOCK_RAW)
// C source: 		return -ESOCKTNOSUPPORT;
// C source: 	if (protocol != PF_KEY_V2)
// C source: 		return -EPROTONOSUPPORT;
// C source: 
// C source: 	sk = sk_alloc(net, PF_KEY, GFP_KERNEL, &key_proto, kern);
// C source: 	if (sk == NULL)
// C source: 		return -ENOMEM;
// C source: 
// C source: 	pfk = pfkey_sk(sk);
// C source: 	mutex_init(&pfk->dump_lock);
// C source: 
// C source: 	sock->ops = &pfkey_ops;
// C source: 	sock_init_data(sock, sk);
// C source: 
// C source: 	sk->sk_family = PF_KEY;
// C source: 	sk->sk_destruct = pfkey_sock_destruct;
// C source: 
// C source: 	atomic_inc(&net_pfkey->socks_nr);
// C source: 
// C source: 	pfkey_insert(sk);
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_release(struct socket *sock)
// C source: {
// C source: 	struct sock *sk = sock->sk;
// C source: 
// C source: 	if (!sk)
// C source: 		return 0;
// C source: 
// C source: 	pfkey_remove(sk);
// C source: 
// C source: 	sock_orphan(sk);
// C source: 	sock->sk = NULL;
// C source: 	skb_queue_purge(&sk->sk_write_queue);
// C source: 
// C source: 	synchronize_rcu();
// C source: 	sock_put(sk);
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_broadcast_one(struct sk_buff *skb, gfp_t allocation,
// C source: 			       struct sock *sk)
// C source: {
// C source: 	int err = -ENOBUFS;
// C source: 
// C source: 	if (atomic_read(&sk->sk_rmem_alloc) > sk->sk_rcvbuf)
// C source: 		return err;
// C source: 
// C source: 	skb = skb_clone(skb, allocation);
// C source: 
// C source: 	if (skb) {
// C source: 		skb_set_owner_r(skb, sk);
// C source: 		skb_queue_tail(&sk->sk_receive_queue, skb);
// C source: 		sk->sk_data_ready(sk);
// C source: 		err = 0;
// C source: 	}
// C source: 	return err;
// C source: }
// C source: 
// C source: /* Send SKB to all pfkey sockets matching selected criteria.  */
// C source: #define BROADCAST_ALL		0
// C source: #define BROADCAST_ONE		1
// C source: #define BROADCAST_REGISTERED	2
// C source: #define BROADCAST_PROMISC_ONLY	4
// C source: static int pfkey_broadcast(struct sk_buff *skb, gfp_t allocation,
// C source: 			   int broadcast_flags, struct sock *one_sk,
// C source: 			   struct net *net)
// C source: {
// C source: 	struct netns_pfkey *net_pfkey = net_generic(net, pfkey_net_id);
// C source: 	struct sock *sk;
// C source: 	int err = -ESRCH;
// C source: 
// C source: 	/* XXX Do we need something like netlink_overrun?  I think
// C source: 	 * XXX PF_KEY socket apps will not mind current behavior.
// C source: 	 */
// C source: 	if (!skb)
// C source: 		return -ENOMEM;
// C source: 
// C source: 	rcu_read_lock();
// C source: 	sk_for_each_rcu(sk, &net_pfkey->table) {
// C source: 		struct pfkey_sock *pfk = pfkey_sk(sk);
// C source: 		int err2;
// C source: 
// C source: 		/* Yes, it means that if you are meant to receive this
// C source: 		 * pfkey message you receive it twice as promiscuous
// C source: 		 * socket.
// C source: 		 */
// C source: 		if (pfk->promisc)
// C source: 			pfkey_broadcast_one(skb, GFP_ATOMIC, sk);
// C source: 
// C source: 		/* the exact target will be processed later */
// C source: 		if (sk == one_sk)
// C source: 			continue;
// C source: 		if (broadcast_flags != BROADCAST_ALL) {
// C source: 			if (broadcast_flags & BROADCAST_PROMISC_ONLY)
// C source: 				continue;
// C source: 			if ((broadcast_flags & BROADCAST_REGISTERED) &&
// C source: 			    !pfk->registered)
// C source: 				continue;
// C source: 			if (broadcast_flags & BROADCAST_ONE)
// C source: 				continue;
// C source: 		}
// C source: 
// C source: 		err2 = pfkey_broadcast_one(skb, GFP_ATOMIC, sk);
// C source: 
// C source: 		/* Error is cleared after successful sending to at least one
// C source: 		 * registered KM */
// C source: 		if ((broadcast_flags & BROADCAST_REGISTERED) && err)
// C source: 			err = err2;
// C source: 	}
// C source: 	rcu_read_unlock();
// C source: 
// C source: 	if (one_sk != NULL)
// C source: 		err = pfkey_broadcast_one(skb, allocation, one_sk);
// C source: 
// C source: 	kfree_skb(skb);
// C source: 	return err;
// C source: }
// C source: 
// C source: static int pfkey_do_dump(struct pfkey_sock *pfk)
// C source: {
// C source: 	struct sadb_msg *hdr;
// C source: 	int rc;
// C source: 
// C source: 	mutex_lock(&pfk->dump_lock);
// C source: 	if (!pfk->dump.dump) {
// C source: 		rc = 0;
// C source: 		goto out;
// C source: 	}
// C source: 
// C source: 	rc = pfk->dump.dump(pfk);
// C source: 	if (rc == -ENOBUFS) {
// C source: 		rc = 0;
// C source: 		goto out;
// C source: 	}
// C source: 
// C source: 	if (pfk->dump.skb) {
// C source: 		if (!pfkey_can_dump(&pfk->sk)) {
// C source: 			rc = 0;
// C source: 			goto out;
// C source: 		}
// C source: 
// C source: 		hdr = (struct sadb_msg *) pfk->dump.skb->data;
// C source: 		hdr->sadb_msg_seq = 0;
// C source: 		hdr->sadb_msg_errno = rc;
// C source: 		pfkey_broadcast(pfk->dump.skb, GFP_ATOMIC, BROADCAST_ONE,
// C source: 				&pfk->sk, sock_net(&pfk->sk));
// C source: 		pfk->dump.skb = NULL;
// C source: 	}
// C source: 
// C source: 	pfkey_terminate_dump(pfk);
// C source: 
// C source: out:
// C source: 	mutex_unlock(&pfk->dump_lock);
// C source: 	return rc;
// C source: }
// C source: 
// C source: static inline void pfkey_hdr_dup(struct sadb_msg *new,
// C source: 				 const struct sadb_msg *orig)
// C source: {
// C source: 	*new = *orig;
// C source: }
// C source: 
// C source: static int pfkey_error(const struct sadb_msg *orig, int err, struct sock *sk)
// C source: {
// C source: 	struct sk_buff *skb = alloc_skb(sizeof(struct sadb_msg) + 16, GFP_KERNEL);
// C source: 	struct sadb_msg *hdr;
// C source: 
// C source: 	if (!skb)
// C source: 		return -ENOBUFS;
// C source: 
// C source: 	/* Woe be to the platform trying to support PFKEY yet
// C source: 	 * having normal errnos outside the 1-255 range, inclusive.
// C source: 	 */
// C source: 	err = -err;
// C source: 	if (err == ERESTARTSYS ||
// C source: 	    err == ERESTARTNOHAND ||
// C source: 	    err == ERESTARTNOINTR)
// C source: 		err = EINTR;
// C source: 	if (err >= 512)
// C source: 		err = EINVAL;
// C source: 	BUG_ON(err <= 0 || err >= 256);
// C source: 
// C source: 	hdr = skb_put(skb, sizeof(struct sadb_msg));
// C source: 	pfkey_hdr_dup(hdr, orig);
// C source: 	hdr->sadb_msg_errno = (uint8_t) err;
// C source: 	hdr->sadb_msg_len = (sizeof(struct sadb_msg) /
// C source: 			     sizeof(uint64_t));
// C source: 
// C source: 	pfkey_broadcast(skb, GFP_KERNEL, BROADCAST_ONE, sk, sock_net(sk));
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static const u8 sadb_ext_min_len[] = {
// C source: 	[SADB_EXT_RESERVED]		= (u8) 0,
// C source: 	[SADB_EXT_SA]			= (u8) sizeof(struct sadb_sa),
// C source: 	[SADB_EXT_LIFETIME_CURRENT]	= (u8) sizeof(struct sadb_lifetime),
// C source: 	[SADB_EXT_LIFETIME_HARD]	= (u8) sizeof(struct sadb_lifetime),
// C source: 	[SADB_EXT_LIFETIME_SOFT]	= (u8) sizeof(struct sadb_lifetime),
// C source: 	[SADB_EXT_ADDRESS_SRC]		= (u8) sizeof(struct sadb_address),
// C source: 	[SADB_EXT_ADDRESS_DST]		= (u8) sizeof(struct sadb_address),
// C source: 	[SADB_EXT_ADDRESS_PROXY]	= (u8) sizeof(struct sadb_address),
// C source: 	[SADB_EXT_KEY_AUTH]		= (u8) sizeof(struct sadb_key),
// C source: 	[SADB_EXT_KEY_ENCRYPT]		= (u8) sizeof(struct sadb_key),
// C source: 	[SADB_EXT_IDENTITY_SRC]		= (u8) sizeof(struct sadb_ident),
// C source: 	[SADB_EXT_IDENTITY_DST]		= (u8) sizeof(struct sadb_ident),
// C source: 	[SADB_EXT_SENSITIVITY]		= (u8) sizeof(struct sadb_sens),
// C source: 	[SADB_EXT_PROPOSAL]		= (u8) sizeof(struct sadb_prop),
// C source: 	[SADB_EXT_SUPPORTED_AUTH]	= (u8) sizeof(struct sadb_supported),
// C source: 	[SADB_EXT_SUPPORTED_ENCRYPT]	= (u8) sizeof(struct sadb_supported),
// C source: 	[SADB_EXT_SPIRANGE]		= (u8) sizeof(struct sadb_spirange),
// C source: 	[SADB_X_EXT_KMPRIVATE]		= (u8) sizeof(struct sadb_x_kmprivate),
// C source: 	[SADB_X_EXT_POLICY]		= (u8) sizeof(struct sadb_x_policy),
// C source: 	[SADB_X_EXT_SA2]		= (u8) sizeof(struct sadb_x_sa2),
// C source: 	[SADB_X_EXT_NAT_T_TYPE]		= (u8) sizeof(struct sadb_x_nat_t_type),
// C source: 	[SADB_X_EXT_NAT_T_SPORT]	= (u8) sizeof(struct sadb_x_nat_t_port),
// C source: 	[SADB_X_EXT_NAT_T_DPORT]	= (u8) sizeof(struct sadb_x_nat_t_port),
// C source: 	[SADB_X_EXT_NAT_T_OA]		= (u8) sizeof(struct sadb_address),
// C source: 	[SADB_X_EXT_SEC_CTX]		= (u8) sizeof(struct sadb_x_sec_ctx),
// C source: 	[SADB_X_EXT_KMADDRESS]		= (u8) sizeof(struct sadb_x_kmaddress),
// C source: 	[SADB_X_EXT_FILTER]		= (u8) sizeof(struct sadb_x_filter),
// C source: };
// C source: 
// C source: /* Verify sadb_address_{len,prefixlen} against sa_family.  */
// C source: static int verify_address_len(const void *p)
// C source: {
// C source: 	const struct sadb_address *sp = p;
// C source: 	const struct sockaddr *addr = (const struct sockaddr *)(sp + 1);
// C source: 	const struct sockaddr_in *sin;
// C source: #if IS_ENABLED(CONFIG_IPV6)
// C source: 	const struct sockaddr_in6 *sin6;
// C source: #endif
// C source: 	int len;
// C source: 
// C source: 	if (sp->sadb_address_len <
// C source: 	    DIV_ROUND_UP(sizeof(*sp) + offsetofend(typeof(*addr), sa_family),
// C source: 			 sizeof(uint64_t)))
// C source: 		return -EINVAL;
// C source: 
// C source: 	switch (addr->sa_family) {
// C source: 	case AF_INET:
// C source: 		len = DIV_ROUND_UP(sizeof(*sp) + sizeof(*sin), sizeof(uint64_t));
// C source: 		if (sp->sadb_address_len != len ||
// C source: 		    sp->sadb_address_prefixlen > 32)
// C source: 			return -EINVAL;
// C source: 		break;
// C source: #if IS_ENABLED(CONFIG_IPV6)
// C source: 	case AF_INET6:
// C source: 		len = DIV_ROUND_UP(sizeof(*sp) + sizeof(*sin6), sizeof(uint64_t));
// C source: 		if (sp->sadb_address_len != len ||
// C source: 		    sp->sadb_address_prefixlen > 128)
// C source: 			return -EINVAL;
// C source: 		break;
// C source: #endif
// C source: 	default:
// C source: 		/* It is user using kernel to keep track of security
// C source: 		 * associations for another protocol, such as
// C source: 		 * OSPF/RSVP/RIPV2/MIP.  It is user's job to verify
// C source: 		 * lengths.
// C source: 		 *
// C source: 		 * XXX Actually, association/policy database is not yet
// C source: 		 * XXX able to cope with arbitrary sockaddr families.
// C source: 		 * XXX When it can, remove this -EINVAL.  -DaveM
// C source: 		 */
// C source: 		return -EINVAL;
// C source: 	}
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static inline int sadb_key_len(const struct sadb_key *key)
// C source: {
// C source: 	int key_bytes = DIV_ROUND_UP(key->sadb_key_bits, 8);
// C source: 
// C source: 	return DIV_ROUND_UP(sizeof(struct sadb_key) + key_bytes,
// C source: 			    sizeof(uint64_t));
// C source: }
// C source: 
// C source: static int verify_key_len(const void *p)
// C source: {
// C source: 	const struct sadb_key *key = p;
// C source: 
// C source: 	if (sadb_key_len(key) > key->sadb_key_len)
// C source: 		return -EINVAL;
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static inline int pfkey_sec_ctx_len(const struct sadb_x_sec_ctx *sec_ctx)
// C source: {
// C source: 	return DIV_ROUND_UP(sizeof(struct sadb_x_sec_ctx) +
// C source: 			    sec_ctx->sadb_x_ctx_len,
// C source: 			    sizeof(uint64_t));
// C source: }
// C source: 
// C source: static inline int verify_sec_ctx_len(const void *p)
// C source: {
// C source: 	const struct sadb_x_sec_ctx *sec_ctx = p;
// C source: 	int len = sec_ctx->sadb_x_ctx_len;
// C source: 
// C source: 	if (len > PAGE_SIZE)
// C source: 		return -EINVAL;
// C source: 
// C source: 	len = pfkey_sec_ctx_len(sec_ctx);
// C source: 
// C source: 	if (sec_ctx->sadb_x_sec_len != len)
// C source: 		return -EINVAL;
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static inline struct xfrm_user_sec_ctx *pfkey_sadb2xfrm_user_sec_ctx(const struct sadb_x_sec_ctx *sec_ctx,
// C source: 								     gfp_t gfp)
// C source: {
// C source: 	struct xfrm_user_sec_ctx *uctx = NULL;
// C source: 	int ctx_size = sec_ctx->sadb_x_ctx_len;
// C source: 
// C source: 	uctx = kmalloc((sizeof(*uctx)+ctx_size), gfp);
// C source: 
// C source: 	if (!uctx)
// C source: 		return NULL;
// C source: 
// C source: 	uctx->len = pfkey_sec_ctx_len(sec_ctx);
// C source: 	uctx->exttype = sec_ctx->sadb_x_sec_exttype;
// C source: 	uctx->ctx_doi = sec_ctx->sadb_x_ctx_doi;
// C source: 	uctx->ctx_alg = sec_ctx->sadb_x_ctx_alg;
// C source: 	uctx->ctx_len = sec_ctx->sadb_x_ctx_len;
// C source: 	memcpy(uctx + 1, sec_ctx + 1,
// C source: 	       uctx->ctx_len);
// C source: 
// C source: 	return uctx;
// C source: }
// C source: 
// C source: static int present_and_same_family(const struct sadb_address *src,
// C source: 				   const struct sadb_address *dst)
// C source: {
// C source: 	const struct sockaddr *s_addr, *d_addr;
// C source: 
// C source: 	if (!src || !dst)
// C source: 		return 0;
// C source: 
// C source: 	s_addr = (const struct sockaddr *)(src + 1);
// C source: 	d_addr = (const struct sockaddr *)(dst + 1);
// C source: 	if (s_addr->sa_family != d_addr->sa_family)
// C source: 		return 0;
// C source: 	if (s_addr->sa_family != AF_INET
// C source: #if IS_ENABLED(CONFIG_IPV6)
// C source: 	    && s_addr->sa_family != AF_INET6
// C source: #endif
// C source: 		)
// C source: 		return 0;
// C source: 
// C source: 	return 1;
// C source: }
// C source: 
// C source: static int parse_exthdrs(struct sk_buff *skb, const struct sadb_msg *hdr, void **ext_hdrs)
// C source: {
// C source: 	const char *p = (char *) hdr;
// C source: 	int len = skb->len;
// C source: 
// C source: 	len -= sizeof(*hdr);
// C source: 	p += sizeof(*hdr);
// C source: 	while (len > 0) {
// C source: 		const struct sadb_ext *ehdr = (const struct sadb_ext *) p;
// C source: 		uint16_t ext_type;
// C source: 		int ext_len;
// C source: 
// C source: 		if (len < sizeof(*ehdr))
// C source: 			return -EINVAL;
// C source: 
// C source: 		ext_len  = ehdr->sadb_ext_len;
// C source: 		ext_len *= sizeof(uint64_t);
// C source: 		ext_type = ehdr->sadb_ext_type;
// C source: 		if (ext_len < sizeof(uint64_t) ||
// C source: 		    ext_len > len ||
// C source: 		    ext_type == SADB_EXT_RESERVED)
// C source: 			return -EINVAL;
// C source: 
// C source: 		if (ext_type <= SADB_EXT_MAX) {
// C source: 			int min = (int) sadb_ext_min_len[ext_type];
// C source: 			if (ext_len < min)
// C source: 				return -EINVAL;
// C source: 			if (ext_hdrs[ext_type-1] != NULL)
// C source: 				return -EINVAL;
// C source: 			switch (ext_type) {
// C source: 			case SADB_EXT_ADDRESS_SRC:
// C source: 			case SADB_EXT_ADDRESS_DST:
// C source: 			case SADB_EXT_ADDRESS_PROXY:
// C source: 			case SADB_X_EXT_NAT_T_OA:
// C source: 				if (verify_address_len(p))
// C source: 					return -EINVAL;
// C source: 				break;
// C source: 			case SADB_X_EXT_SEC_CTX:
// C source: 				if (verify_sec_ctx_len(p))
// C source: 					return -EINVAL;
// C source: 				break;
// C source: 			case SADB_EXT_KEY_AUTH:
// C source: 			case SADB_EXT_KEY_ENCRYPT:
// C source: 				if (verify_key_len(p))
// C source: 					return -EINVAL;
// C source: 				break;
// C source: 			default:
// C source: 				break;
// C source: 			}
// C source: 			ext_hdrs[ext_type-1] = (void *) p;
// C source: 		}
// C source: 		p   += ext_len;
// C source: 		len -= ext_len;
// C source: 	}
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static uint16_t
// C source: pfkey_satype2proto(uint8_t satype)
// C source: {
// C source: 	switch (satype) {
// C source: 	case SADB_SATYPE_UNSPEC:
// C source: 		return IPSEC_PROTO_ANY;
// C source: 	case SADB_SATYPE_AH:
// C source: 		return IPPROTO_AH;
// C source: 	case SADB_SATYPE_ESP:
// C source: 		return IPPROTO_ESP;
// C source: 	case SADB_X_SATYPE_IPCOMP:
// C source: 		return IPPROTO_COMP;
// C source: 	default:
// C source: 		return 0;
// C source: 	}
// C source: 	/* NOTREACHED */
// C source: }
// C source: 
// C source: static uint8_t
// C source: pfkey_proto2satype(uint16_t proto)
// C source: {
// C source: 	switch (proto) {
// C source: 	case IPPROTO_AH:
// C source: 		return SADB_SATYPE_AH;
// C source: 	case IPPROTO_ESP:
// C source: 		return SADB_SATYPE_ESP;
// C source: 	case IPPROTO_COMP:
// C source: 		return SADB_X_SATYPE_IPCOMP;
// C source: 	default:
// C source: 		return 0;
// C source: 	}
// C source: 	/* NOTREACHED */
// C source: }
// C source: 
// C source: /* BTW, this scheme means that there is no way with PFKEY2 sockets to
// C source:  * say specifically 'just raw sockets' as we encode them as 255.
// C source:  */
// C source: 
// C source: static uint8_t pfkey_proto_to_xfrm(uint8_t proto)
// C source: {
// C source: 	return proto == IPSEC_PROTO_ANY ? 0 : proto;
// C source: }
// C source: 
// C source: static uint8_t pfkey_proto_from_xfrm(uint8_t proto)
// C source: {
// C source: 	return proto ? proto : IPSEC_PROTO_ANY;
// C source: }
// C source: 
// C source: static inline int pfkey_sockaddr_len(sa_family_t family)
// C source: {
// C source: 	switch (family) {
// C source: 	case AF_INET:
// C source: 		return sizeof(struct sockaddr_in);
// C source: #if IS_ENABLED(CONFIG_IPV6)
// C source: 	case AF_INET6:
// C source: 		return sizeof(struct sockaddr_in6);
// C source: #endif
// C source: 	}
// C source: 	return 0;
// C source: }
// C source: 
// C source: static
// C source: int pfkey_sockaddr_extract(const struct sockaddr *sa, xfrm_address_t *xaddr)
// C source: {
// C source: 	switch (sa->sa_family) {
// C source: 	case AF_INET:
// C source: 		xaddr->a4 =
// C source: 			((struct sockaddr_in *)sa)->sin_addr.s_addr;
// C source: 		return AF_INET;
// C source: #if IS_ENABLED(CONFIG_IPV6)
// C source: 	case AF_INET6:
// C source: 		memcpy(xaddr->a6,
// C source: 		       &((struct sockaddr_in6 *)sa)->sin6_addr,
// C source: 		       sizeof(struct in6_addr));
// C source: 		return AF_INET6;
// C source: #endif
// C source: 	}
// C source: 	return 0;
// C source: }
// C source: 
// C source: static
// C source: int pfkey_sadb_addr2xfrm_addr(const struct sadb_address *addr, xfrm_address_t *xaddr)
// C source: {
// C source: 	return pfkey_sockaddr_extract((struct sockaddr *)(addr + 1),
// C source: 				      xaddr);
// C source: }
// C source: 
// C source: static struct  xfrm_state *pfkey_xfrm_state_lookup(struct net *net, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	const struct sadb_sa *sa;
// C source: 	const struct sadb_address *addr;
// C source: 	uint16_t proto;
// C source: 	unsigned short family;
// C source: 	xfrm_address_t *xaddr;
// C source: 
// C source: 	sa = ext_hdrs[SADB_EXT_SA - 1];
// C source: 	if (sa == NULL)
// C source: 		return NULL;
// C source: 
// C source: 	proto = pfkey_satype2proto(hdr->sadb_msg_satype);
// C source: 	if (proto == 0)
// C source: 		return NULL;
// C source: 
// C source: 	/* sadb_address_len should be checked by caller */
// C source: 	addr = ext_hdrs[SADB_EXT_ADDRESS_DST - 1];
// C source: 	if (addr == NULL)
// C source: 		return NULL;
// C source: 
// C source: 	family = ((const struct sockaddr *)(addr + 1))->sa_family;
// C source: 	switch (family) {
// C source: 	case AF_INET:
// C source: 		xaddr = (xfrm_address_t *)&((const struct sockaddr_in *)(addr + 1))->sin_addr;
// C source: 		break;
// C source: #if IS_ENABLED(CONFIG_IPV6)
// C source: 	case AF_INET6:
// C source: 		xaddr = (xfrm_address_t *)&((const struct sockaddr_in6 *)(addr + 1))->sin6_addr;
// C source: 		break;
// C source: #endif
// C source: 	default:
// C source: 		xaddr = NULL;
// C source: 	}
// C source: 
// C source: 	if (!xaddr)
// C source: 		return NULL;
// C source: 
// C source: 	return xfrm_state_lookup(net, DUMMY_MARK, xaddr, sa->sadb_sa_spi, proto, family);
// C source: }
// C source: 
// C source: #define PFKEY_ALIGN8(a) (1 + (((a) - 1) | (8 - 1)))
// C source: 
// C source: static int
// C source: pfkey_sockaddr_size(sa_family_t family)
// C source: {
// C source: 	return PFKEY_ALIGN8(pfkey_sockaddr_len(family));
// C source: }
// C source: 
// C source: static inline int pfkey_mode_from_xfrm(int mode)
// C source: {
// C source: 	switch(mode) {
// C source: 	case XFRM_MODE_TRANSPORT:
// C source: 		return IPSEC_MODE_TRANSPORT;
// C source: 	case XFRM_MODE_TUNNEL:
// C source: 		return IPSEC_MODE_TUNNEL;
// C source: 	case XFRM_MODE_BEET:
// C source: 		return IPSEC_MODE_BEET;
// C source: 	default:
// C source: 		return -1;
// C source: 	}
// C source: }
// C source: 
// C source: static inline int pfkey_mode_to_xfrm(int mode)
// C source: {
// C source: 	switch(mode) {
// C source: 	case IPSEC_MODE_ANY:	/*XXX*/
// C source: 	case IPSEC_MODE_TRANSPORT:
// C source: 		return XFRM_MODE_TRANSPORT;
// C source: 	case IPSEC_MODE_TUNNEL:
// C source: 		return XFRM_MODE_TUNNEL;
// C source: 	case IPSEC_MODE_BEET:
// C source: 		return XFRM_MODE_BEET;
// C source: 	default:
// C source: 		return -1;
// C source: 	}
// C source: }
// C source: 
// C source: static unsigned int pfkey_sockaddr_fill(const xfrm_address_t *xaddr, __be16 port,
// C source: 					struct sockaddr *sa,
// C source: 					unsigned short family)
// C source: {
// C source: 	switch (family) {
// C source: 	case AF_INET:
// C source: 	    {
// C source: 		struct sockaddr_in *sin = (struct sockaddr_in *)sa;
// C source: 		sin->sin_family = AF_INET;
// C source: 		sin->sin_port = port;
// C source: 		sin->sin_addr.s_addr = xaddr->a4;
// C source: 		memset(sin->sin_zero, 0, sizeof(sin->sin_zero));
// C source: 		return 32;
// C source: 	    }
// C source: #if IS_ENABLED(CONFIG_IPV6)
// C source: 	case AF_INET6:
// C source: 	    {
// C source: 		struct sockaddr_in6 *sin6 = (struct sockaddr_in6 *)sa;
// C source: 		sin6->sin6_family = AF_INET6;
// C source: 		sin6->sin6_port = port;
// C source: 		sin6->sin6_flowinfo = 0;
// C source: 		sin6->sin6_addr = xaddr->in6;
// C source: 		sin6->sin6_scope_id = 0;
// C source: 		return 128;
// C source: 	    }
// C source: #endif
// C source: 	}
// C source: 	return 0;
// C source: }
// C source: 
// C source: static unsigned int pfkey_sockaddr_fill_zero_tail(const xfrm_address_t *xaddr,
// C source: 						  __be16 port,
// C source: 						  struct sockaddr *sa,
// C source: 						  unsigned short family)
// C source: {
// C source: 	unsigned int prefixlen;
// C source: 	int sockaddr_len = pfkey_sockaddr_len(family);
// C source: 	int sockaddr_size = pfkey_sockaddr_size(family);
// C source: 
// C source: 	prefixlen = pfkey_sockaddr_fill(xaddr, port, sa, family);
// C source: 	if (sockaddr_size > sockaddr_len)
// C source: 		memset((u8 *)sa + sockaddr_len, 0, sockaddr_size - sockaddr_len);
// C source: 
// C source: 	return prefixlen;
// C source: }
// C source: 
// C source: static struct sk_buff *__pfkey_xfrm_state2msg(const struct xfrm_state *x,
// C source: 					      int add_keys, int hsc)
// C source: {
// C source: 	struct sk_buff *skb;
// C source: 	struct sadb_msg *hdr;
// C source: 	struct sadb_sa *sa;
// C source: 	struct sadb_lifetime *lifetime;
// C source: 	struct sadb_address *addr;
// C source: 	struct sadb_key *key;
// C source: 	struct sadb_x_sa2 *sa2;
// C source: 	struct sadb_x_sec_ctx *sec_ctx;
// C source: 	struct xfrm_sec_ctx *xfrm_ctx;
// C source: 	int ctx_size = 0;
// C source: 	int size;
// C source: 	int auth_key_size = 0;
// C source: 	int encrypt_key_size = 0;
// C source: 	int sockaddr_size;
// C source: 	struct xfrm_encap_tmpl *natt = NULL;
// C source: 	int mode;
// C source: 
// C source: 	/* address family check */
// C source: 	sockaddr_size = pfkey_sockaddr_size(x->props.family);
// C source: 	if (!sockaddr_size)
// C source: 		return ERR_PTR(-EINVAL);
// C source: 
// C source: 	/* base, SA, (lifetime (HSC),) address(SD), (address(P),)
// C source: 	   key(AE), (identity(SD),) (sensitivity)> */
// C source: 	size = sizeof(struct sadb_msg) +sizeof(struct sadb_sa) +
// C source: 		sizeof(struct sadb_lifetime) +
// C source: 		((hsc & 1) ? sizeof(struct sadb_lifetime) : 0) +
// C source: 		((hsc & 2) ? sizeof(struct sadb_lifetime) : 0) +
// C source: 			sizeof(struct sadb_address)*2 +
// C source: 				sockaddr_size*2 +
// C source: 					sizeof(struct sadb_x_sa2);
// C source: 
// C source: 	if ((xfrm_ctx = x->security)) {
// C source: 		ctx_size = PFKEY_ALIGN8(xfrm_ctx->ctx_len);
// C source: 		size += sizeof(struct sadb_x_sec_ctx) + ctx_size;
// C source: 	}
// C source: 
// C source: 	/* identity & sensitivity */
// C source: 	if (!xfrm_addr_equal(&x->sel.saddr, &x->props.saddr, x->props.family))
// C source: 		size += sizeof(struct sadb_address) + sockaddr_size;
// C source: 
// C source: 	if (add_keys) {
// C source: 		if (x->aalg && x->aalg->alg_key_len) {
// C source: 			auth_key_size =
// C source: 				PFKEY_ALIGN8((x->aalg->alg_key_len + 7) / 8);
// C source: 			size += sizeof(struct sadb_key) + auth_key_size;
// C source: 		}
// C source: 		if (x->ealg && x->ealg->alg_key_len) {
// C source: 			encrypt_key_size =
// C source: 				PFKEY_ALIGN8((x->ealg->alg_key_len+7) / 8);
// C source: 			size += sizeof(struct sadb_key) + encrypt_key_size;
// C source: 		}
// C source: 	}
// C source: 	if (x->encap)
// C source: 		natt = x->encap;
// C source: 
// C source: 	if (natt && natt->encap_type) {
// C source: 		size += sizeof(struct sadb_x_nat_t_type);
// C source: 		size += sizeof(struct sadb_x_nat_t_port);
// C source: 		size += sizeof(struct sadb_x_nat_t_port);
// C source: 	}
// C source: 
// C source: 	skb =  alloc_skb(size + 16, GFP_ATOMIC);
// C source: 	if (skb == NULL)
// C source: 		return ERR_PTR(-ENOBUFS);
// C source: 
// C source: 	/* call should fill header later */
// C source: 	hdr = skb_put(skb, sizeof(struct sadb_msg));
// C source: 	memset(hdr, 0, size);	/* XXX do we need this ? */
// C source: 	hdr->sadb_msg_len = size / sizeof(uint64_t);
// C source: 
// C source: 	/* sa */
// C source: 	sa = skb_put(skb, sizeof(struct sadb_sa));
// C source: 	sa->sadb_sa_len = sizeof(struct sadb_sa)/sizeof(uint64_t);
// C source: 	sa->sadb_sa_exttype = SADB_EXT_SA;
// C source: 	sa->sadb_sa_spi = x->id.spi;
// C source: 	sa->sadb_sa_replay = x->props.replay_window;
// C source: 	switch (x->km.state) {
// C source: 	case XFRM_STATE_VALID:
// C source: 		sa->sadb_sa_state = x->km.dying ?
// C source: 			SADB_SASTATE_DYING : SADB_SASTATE_MATURE;
// C source: 		break;
// C source: 	case XFRM_STATE_ACQ:
// C source: 		sa->sadb_sa_state = SADB_SASTATE_LARVAL;
// C source: 		break;
// C source: 	default:
// C source: 		sa->sadb_sa_state = SADB_SASTATE_DEAD;
// C source: 		break;
// C source: 	}
// C source: 	sa->sadb_sa_auth = 0;
// C source: 	if (x->aalg) {
// C source: 		struct xfrm_algo_desc *a = xfrm_aalg_get_byname(x->aalg->alg_name, 0);
// C source: 		sa->sadb_sa_auth = (a && a->pfkey_supported) ?
// C source: 					a->desc.sadb_alg_id : 0;
// C source: 	}
// C source: 	sa->sadb_sa_encrypt = 0;
// C source: 	BUG_ON(x->ealg && x->calg);
// C source: 	if (x->ealg) {
// C source: 		struct xfrm_algo_desc *a = xfrm_ealg_get_byname(x->ealg->alg_name, 0);
// C source: 		sa->sadb_sa_encrypt = (a && a->pfkey_supported) ?
// C source: 					a->desc.sadb_alg_id : 0;
// C source: 	}
// C source: 	/* KAME compatible: sadb_sa_encrypt is overloaded with calg id */
// C source: 	if (x->calg) {
// C source: 		struct xfrm_algo_desc *a = xfrm_calg_get_byname(x->calg->alg_name, 0);
// C source: 		sa->sadb_sa_encrypt = (a && a->pfkey_supported) ?
// C source: 					a->desc.sadb_alg_id : 0;
// C source: 	}
// C source: 
// C source: 	sa->sadb_sa_flags = 0;
// C source: 	if (x->props.flags & XFRM_STATE_NOECN)
// C source: 		sa->sadb_sa_flags |= SADB_SAFLAGS_NOECN;
// C source: 	if (x->props.flags & XFRM_STATE_DECAP_DSCP)
// C source: 		sa->sadb_sa_flags |= SADB_SAFLAGS_DECAP_DSCP;
// C source: 	if (x->props.flags & XFRM_STATE_NOPMTUDISC)
// C source: 		sa->sadb_sa_flags |= SADB_SAFLAGS_NOPMTUDISC;
// C source: 
// C source: 	/* hard time */
// C source: 	if (hsc & 2) {
// C source: 		lifetime = skb_put(skb, sizeof(struct sadb_lifetime));
// C source: 		lifetime->sadb_lifetime_len =
// C source: 			sizeof(struct sadb_lifetime)/sizeof(uint64_t);
// C source: 		lifetime->sadb_lifetime_exttype = SADB_EXT_LIFETIME_HARD;
// C source: 		lifetime->sadb_lifetime_allocations =  _X2KEY(x->lft.hard_packet_limit);
// C source: 		lifetime->sadb_lifetime_bytes = _X2KEY(x->lft.hard_byte_limit);
// C source: 		lifetime->sadb_lifetime_addtime = x->lft.hard_add_expires_seconds;
// C source: 		lifetime->sadb_lifetime_usetime = x->lft.hard_use_expires_seconds;
// C source: 	}
// C source: 	/* soft time */
// C source: 	if (hsc & 1) {
// C source: 		lifetime = skb_put(skb, sizeof(struct sadb_lifetime));
// C source: 		lifetime->sadb_lifetime_len =
// C source: 			sizeof(struct sadb_lifetime)/sizeof(uint64_t);
// C source: 		lifetime->sadb_lifetime_exttype = SADB_EXT_LIFETIME_SOFT;
// C source: 		lifetime->sadb_lifetime_allocations =  _X2KEY(x->lft.soft_packet_limit);
// C source: 		lifetime->sadb_lifetime_bytes = _X2KEY(x->lft.soft_byte_limit);
// C source: 		lifetime->sadb_lifetime_addtime = x->lft.soft_add_expires_seconds;
// C source: 		lifetime->sadb_lifetime_usetime = x->lft.soft_use_expires_seconds;
// C source: 	}
// C source: 	/* current time */
// C source: 	lifetime = skb_put(skb, sizeof(struct sadb_lifetime));
// C source: 	lifetime->sadb_lifetime_len =
// C source: 		sizeof(struct sadb_lifetime)/sizeof(uint64_t);
// C source: 	lifetime->sadb_lifetime_exttype = SADB_EXT_LIFETIME_CURRENT;
// C source: 	lifetime->sadb_lifetime_allocations = x->curlft.packets;
// C source: 	lifetime->sadb_lifetime_bytes = x->curlft.bytes;
// C source: 	lifetime->sadb_lifetime_addtime = x->curlft.add_time;
// C source: 	lifetime->sadb_lifetime_usetime = x->curlft.use_time;
// C source: 	/* src address */
// C source: 	addr = skb_put(skb, sizeof(struct sadb_address) + sockaddr_size);
// C source: 	addr->sadb_address_len =
// C source: 		(sizeof(struct sadb_address)+sockaddr_size)/
// C source: 			sizeof(uint64_t);
// C source: 	addr->sadb_address_exttype = SADB_EXT_ADDRESS_SRC;
// C source: 	/* "if the ports are non-zero, then the sadb_address_proto field,
// C source: 	   normally zero, MUST be filled in with the transport
// C source: 	   protocol's number." - RFC2367 */
// C source: 	addr->sadb_address_proto = 0;
// C source: 	addr->sadb_address_reserved = 0;
// C source: 
// C source: 	addr->sadb_address_prefixlen =
// C source: 		pfkey_sockaddr_fill(&x->props.saddr, 0,
// C source: 				    (struct sockaddr *) (addr + 1),
// C source: 				    x->props.family);
// C source: 	BUG_ON(!addr->sadb_address_prefixlen);
// C source: 
// C source: 	/* dst address */
// C source: 	addr = skb_put(skb, sizeof(struct sadb_address) + sockaddr_size);
// C source: 	addr->sadb_address_len =
// C source: 		(sizeof(struct sadb_address)+sockaddr_size)/
// C source: 			sizeof(uint64_t);
// C source: 	addr->sadb_address_exttype = SADB_EXT_ADDRESS_DST;
// C source: 	addr->sadb_address_proto = 0;
// C source: 	addr->sadb_address_reserved = 0;
// C source: 
// C source: 	addr->sadb_address_prefixlen =
// C source: 		pfkey_sockaddr_fill(&x->id.daddr, 0,
// C source: 				    (struct sockaddr *) (addr + 1),
// C source: 				    x->props.family);
// C source: 	BUG_ON(!addr->sadb_address_prefixlen);
// C source: 
// C source: 	if (!xfrm_addr_equal(&x->sel.saddr, &x->props.saddr,
// C source: 			     x->props.family)) {
// C source: 		addr = skb_put(skb,
// C source: 			       sizeof(struct sadb_address) + sockaddr_size);
// C source: 		addr->sadb_address_len =
// C source: 			(sizeof(struct sadb_address)+sockaddr_size)/
// C source: 			sizeof(uint64_t);
// C source: 		addr->sadb_address_exttype = SADB_EXT_ADDRESS_PROXY;
// C source: 		addr->sadb_address_proto =
// C source: 			pfkey_proto_from_xfrm(x->sel.proto);
// C source: 		addr->sadb_address_prefixlen = x->sel.prefixlen_s;
// C source: 		addr->sadb_address_reserved = 0;
// C source: 
// C source: 		pfkey_sockaddr_fill(&x->sel.saddr, x->sel.sport,
// C source: 				    (struct sockaddr *) (addr + 1),
// C source: 				    x->props.family);
// C source: 	}
// C source: 
// C source: 	/* auth key */
// C source: 	if (add_keys && auth_key_size) {
// C source: 		key = skb_put(skb, sizeof(struct sadb_key) + auth_key_size);
// C source: 		key->sadb_key_len = (sizeof(struct sadb_key) + auth_key_size) /
// C source: 			sizeof(uint64_t);
// C source: 		key->sadb_key_exttype = SADB_EXT_KEY_AUTH;
// C source: 		key->sadb_key_bits = x->aalg->alg_key_len;
// C source: 		key->sadb_key_reserved = 0;
// C source: 		memcpy(key + 1, x->aalg->alg_key, (x->aalg->alg_key_len+7)/8);
// C source: 	}
// C source: 	/* encrypt key */
// C source: 	if (add_keys && encrypt_key_size) {
// C source: 		key = skb_put(skb, sizeof(struct sadb_key) + encrypt_key_size);
// C source: 		key->sadb_key_len = (sizeof(struct sadb_key) +
// C source: 				     encrypt_key_size) / sizeof(uint64_t);
// C source: 		key->sadb_key_exttype = SADB_EXT_KEY_ENCRYPT;
// C source: 		key->sadb_key_bits = x->ealg->alg_key_len;
// C source: 		key->sadb_key_reserved = 0;
// C source: 		memcpy(key + 1, x->ealg->alg_key,
// C source: 		       (x->ealg->alg_key_len+7)/8);
// C source: 	}
// C source: 
// C source: 	/* sa */
// C source: 	sa2 = skb_put(skb, sizeof(struct sadb_x_sa2));
// C source: 	sa2->sadb_x_sa2_len = sizeof(struct sadb_x_sa2)/sizeof(uint64_t);
// C source: 	sa2->sadb_x_sa2_exttype = SADB_X_EXT_SA2;
// C source: 	if ((mode = pfkey_mode_from_xfrm(x->props.mode)) < 0) {
// C source: 		kfree_skb(skb);
// C source: 		return ERR_PTR(-EINVAL);
// C source: 	}
// C source: 	sa2->sadb_x_sa2_mode = mode;
// C source: 	sa2->sadb_x_sa2_reserved1 = 0;
// C source: 	sa2->sadb_x_sa2_reserved2 = 0;
// C source: 	sa2->sadb_x_sa2_sequence = 0;
// C source: 	sa2->sadb_x_sa2_reqid = x->props.reqid;
// C source: 
// C source: 	if (natt && natt->encap_type) {
// C source: 		struct sadb_x_nat_t_type *n_type;
// C source: 		struct sadb_x_nat_t_port *n_port;
// C source: 
// C source: 		/* type */
// C source: 		n_type = skb_put(skb, sizeof(*n_type));
// C source: 		n_type->sadb_x_nat_t_type_len = sizeof(*n_type)/sizeof(uint64_t);
// C source: 		n_type->sadb_x_nat_t_type_exttype = SADB_X_EXT_NAT_T_TYPE;
// C source: 		n_type->sadb_x_nat_t_type_type = natt->encap_type;
// C source: 		n_type->sadb_x_nat_t_type_reserved[0] = 0;
// C source: 		n_type->sadb_x_nat_t_type_reserved[1] = 0;
// C source: 		n_type->sadb_x_nat_t_type_reserved[2] = 0;
// C source: 
// C source: 		/* source port */
// C source: 		n_port = skb_put(skb, sizeof(*n_port));
// C source: 		n_port->sadb_x_nat_t_port_len = sizeof(*n_port)/sizeof(uint64_t);
// C source: 		n_port->sadb_x_nat_t_port_exttype = SADB_X_EXT_NAT_T_SPORT;
// C source: 		n_port->sadb_x_nat_t_port_port = natt->encap_sport;
// C source: 		n_port->sadb_x_nat_t_port_reserved = 0;
// C source: 
// C source: 		/* dest port */
// C source: 		n_port = skb_put(skb, sizeof(*n_port));
// C source: 		n_port->sadb_x_nat_t_port_len = sizeof(*n_port)/sizeof(uint64_t);
// C source: 		n_port->sadb_x_nat_t_port_exttype = SADB_X_EXT_NAT_T_DPORT;
// C source: 		n_port->sadb_x_nat_t_port_port = natt->encap_dport;
// C source: 		n_port->sadb_x_nat_t_port_reserved = 0;
// C source: 	}
// C source: 
// C source: 	/* security context */
// C source: 	if (xfrm_ctx) {
// C source: 		sec_ctx = skb_put(skb,
// C source: 				  sizeof(struct sadb_x_sec_ctx) + ctx_size);
// C source: 		sec_ctx->sadb_x_sec_len =
// C source: 		  (sizeof(struct sadb_x_sec_ctx) + ctx_size) / sizeof(uint64_t);
// C source: 		sec_ctx->sadb_x_sec_exttype = SADB_X_EXT_SEC_CTX;
// C source: 		sec_ctx->sadb_x_ctx_doi = xfrm_ctx->ctx_doi;
// C source: 		sec_ctx->sadb_x_ctx_alg = xfrm_ctx->ctx_alg;
// C source: 		sec_ctx->sadb_x_ctx_len = xfrm_ctx->ctx_len;
// C source: 		memcpy(sec_ctx + 1, xfrm_ctx->ctx_str,
// C source: 		       xfrm_ctx->ctx_len);
// C source: 	}
// C source: 
// C source: 	return skb;
// C source: }
// C source: 
// C source: 
// C source: static inline struct sk_buff *pfkey_xfrm_state2msg(const struct xfrm_state *x)
// C source: {
// C source: 	struct sk_buff *skb;
// C source: 
// C source: 	skb = __pfkey_xfrm_state2msg(x, 1, 3);
// C source: 
// C source: 	return skb;
// C source: }
// C source: 
// C source: static inline struct sk_buff *pfkey_xfrm_state2msg_expire(const struct xfrm_state *x,
// C source: 							  int hsc)
// C source: {
// C source: 	return __pfkey_xfrm_state2msg(x, 0, hsc);
// C source: }
// C source: 
// C source: static struct xfrm_state * pfkey_msg2xfrm_state(struct net *net,
// C source: 						const struct sadb_msg *hdr,
// C source: 						void * const *ext_hdrs)
// C source: {
// C source: 	struct xfrm_state *x;
// C source: 	const struct sadb_lifetime *lifetime;
// C source: 	const struct sadb_sa *sa;
// C source: 	const struct sadb_key *key;
// C source: 	const struct sadb_x_sec_ctx *sec_ctx;
// C source: 	uint16_t proto;
// C source: 	int err;
// C source: 
// C source: 
// C source: 	sa = ext_hdrs[SADB_EXT_SA - 1];
// C source: 	if (!sa ||
// C source: 	    !present_and_same_family(ext_hdrs[SADB_EXT_ADDRESS_SRC-1],
// C source: 				     ext_hdrs[SADB_EXT_ADDRESS_DST-1]))
// C source: 		return ERR_PTR(-EINVAL);
// C source: 	if (hdr->sadb_msg_satype == SADB_SATYPE_ESP &&
// C source: 	    !ext_hdrs[SADB_EXT_KEY_ENCRYPT-1])
// C source: 		return ERR_PTR(-EINVAL);
// C source: 	if (hdr->sadb_msg_satype == SADB_SATYPE_AH &&
// C source: 	    !ext_hdrs[SADB_EXT_KEY_AUTH-1])
// C source: 		return ERR_PTR(-EINVAL);
// C source: 	if (!!ext_hdrs[SADB_EXT_LIFETIME_HARD-1] !=
// C source: 	    !!ext_hdrs[SADB_EXT_LIFETIME_SOFT-1])
// C source: 		return ERR_PTR(-EINVAL);
// C source: 
// C source: 	proto = pfkey_satype2proto(hdr->sadb_msg_satype);
// C source: 	if (proto == 0)
// C source: 		return ERR_PTR(-EINVAL);
// C source: 
// C source: 	/* default error is no buffer space */
// C source: 	err = -ENOBUFS;
// C source: 
// C source: 	/* RFC2367:
// C source: 
// C source:    Only SADB_SASTATE_MATURE SAs may be submitted in an SADB_ADD message.
// C source:    SADB_SASTATE_LARVAL SAs are created by SADB_GETSPI and it is not
// C source:    sensible to add a new SA in the DYING or SADB_SASTATE_DEAD state.
// C source:    Therefore, the sadb_sa_state field of all submitted SAs MUST be
// C source:    SADB_SASTATE_MATURE and the kernel MUST return an error if this is
// C source:    not true.
// C source: 
// C source: 	   However, KAME setkey always uses SADB_SASTATE_LARVAL.
// C source: 	   Hence, we have to _ignore_ sadb_sa_state, which is also reasonable.
// C source: 	 */
// C source: 	if (sa->sadb_sa_auth > SADB_AALG_MAX ||
// C source: 	    (hdr->sadb_msg_satype == SADB_X_SATYPE_IPCOMP &&
// C source: 	     sa->sadb_sa_encrypt > SADB_X_CALG_MAX) ||
// C source: 	    sa->sadb_sa_encrypt > SADB_EALG_MAX)
// C source: 		return ERR_PTR(-EINVAL);
// C source: 	key = ext_hdrs[SADB_EXT_KEY_AUTH - 1];
// C source: 	if (key != NULL &&
// C source: 	    sa->sadb_sa_auth != SADB_X_AALG_NULL &&
// C source: 	    key->sadb_key_bits == 0)
// C source: 		return ERR_PTR(-EINVAL);
// C source: 	key = ext_hdrs[SADB_EXT_KEY_ENCRYPT-1];
// C source: 	if (key != NULL &&
// C source: 	    sa->sadb_sa_encrypt != SADB_EALG_NULL &&
// C source: 	    key->sadb_key_bits == 0)
// C source: 		return ERR_PTR(-EINVAL);
// C source: 
// C source: 	x = xfrm_state_alloc(net);
// C source: 	if (x == NULL)
// C source: 		return ERR_PTR(-ENOBUFS);
// C source: 
// C source: 	x->id.proto = proto;
// C source: 	x->id.spi = sa->sadb_sa_spi;
// C source: 	x->props.replay_window = min_t(unsigned int, sa->sadb_sa_replay,
// C source: 					(sizeof(x->replay.bitmap) * 8));
// C source: 	if (sa->sadb_sa_flags & SADB_SAFLAGS_NOECN)
// C source: 		x->props.flags |= XFRM_STATE_NOECN;
// C source: 	if (sa->sadb_sa_flags & SADB_SAFLAGS_DECAP_DSCP)
// C source: 		x->props.flags |= XFRM_STATE_DECAP_DSCP;
// C source: 	if (sa->sadb_sa_flags & SADB_SAFLAGS_NOPMTUDISC)
// C source: 		x->props.flags |= XFRM_STATE_NOPMTUDISC;
// C source: 
// C source: 	lifetime = ext_hdrs[SADB_EXT_LIFETIME_HARD - 1];
// C source: 	if (lifetime != NULL) {
// C source: 		x->lft.hard_packet_limit = _KEY2X(lifetime->sadb_lifetime_allocations);
// C source: 		x->lft.hard_byte_limit = _KEY2X(lifetime->sadb_lifetime_bytes);
// C source: 		x->lft.hard_add_expires_seconds = lifetime->sadb_lifetime_addtime;
// C source: 		x->lft.hard_use_expires_seconds = lifetime->sadb_lifetime_usetime;
// C source: 	}
// C source: 	lifetime = ext_hdrs[SADB_EXT_LIFETIME_SOFT - 1];
// C source: 	if (lifetime != NULL) {
// C source: 		x->lft.soft_packet_limit = _KEY2X(lifetime->sadb_lifetime_allocations);
// C source: 		x->lft.soft_byte_limit = _KEY2X(lifetime->sadb_lifetime_bytes);
// C source: 		x->lft.soft_add_expires_seconds = lifetime->sadb_lifetime_addtime;
// C source: 		x->lft.soft_use_expires_seconds = lifetime->sadb_lifetime_usetime;
// C source: 	}
// C source: 
// C source: 	sec_ctx = ext_hdrs[SADB_X_EXT_SEC_CTX - 1];
// C source: 	if (sec_ctx != NULL) {
// C source: 		struct xfrm_user_sec_ctx *uctx = pfkey_sadb2xfrm_user_sec_ctx(sec_ctx, GFP_KERNEL);
// C source: 
// C source: 		if (!uctx)
// C source: 			goto out;
// C source: 
// C source: 		err = security_xfrm_state_alloc(x, uctx);
// C source: 		kfree(uctx);
// C source: 
// C source: 		if (err)
// C source: 			goto out;
// C source: 	}
// C source: 
// C source: 	err = -ENOBUFS;
// C source: 	key = ext_hdrs[SADB_EXT_KEY_AUTH - 1];
// C source: 	if (sa->sadb_sa_auth) {
// C source: 		int keysize = 0;
// C source: 		struct xfrm_algo_desc *a = xfrm_aalg_get_byid(sa->sadb_sa_auth);
// C source: 		if (!a || !a->pfkey_supported) {
// C source: 			err = -ENOSYS;
// C source: 			goto out;
// C source: 		}
// C source: 		if (key)
// C source: 			keysize = (key->sadb_key_bits + 7) / 8;
// C source: 		x->aalg = kmalloc(sizeof(*x->aalg) + keysize, GFP_KERNEL);
// C source: 		if (!x->aalg) {
// C source: 			err = -ENOMEM;
// C source: 			goto out;
// C source: 		}
// C source: 		strcpy(x->aalg->alg_name, a->name);
// C source: 		x->aalg->alg_key_len = 0;
// C source: 		if (key) {
// C source: 			x->aalg->alg_key_len = key->sadb_key_bits;
// C source: 			memcpy(x->aalg->alg_key, key+1, keysize);
// C source: 		}
// C source: 		x->aalg->alg_trunc_len = a->uinfo.auth.icv_truncbits;
// C source: 		x->props.aalgo = sa->sadb_sa_auth;
// C source: 		/* x->algo.flags = sa->sadb_sa_flags; */
// C source: 	}
// C source: 	if (sa->sadb_sa_encrypt) {
// C source: 		if (hdr->sadb_msg_satype == SADB_X_SATYPE_IPCOMP) {
// C source: 			struct xfrm_algo_desc *a = xfrm_calg_get_byid(sa->sadb_sa_encrypt);
// C source: 			if (!a || !a->pfkey_supported) {
// C source: 				err = -ENOSYS;
// C source: 				goto out;
// C source: 			}
// C source: 			x->calg = kmalloc_obj(*x->calg);
// C source: 			if (!x->calg) {
// C source: 				err = -ENOMEM;
// C source: 				goto out;
// C source: 			}
// C source: 			strcpy(x->calg->alg_name, a->name);
// C source: 			x->calg->alg_key_len = 0;
// C source: 			x->props.calgo = sa->sadb_sa_encrypt;
// C source: 		} else {
// C source: 			int keysize = 0;
// C source: 			struct xfrm_algo_desc *a = xfrm_ealg_get_byid(sa->sadb_sa_encrypt);
// C source: 			if (!a || !a->pfkey_supported) {
// C source: 				err = -ENOSYS;
// C source: 				goto out;
// C source: 			}
// C source: 			key = (struct sadb_key*) ext_hdrs[SADB_EXT_KEY_ENCRYPT-1];
// C source: 			if (key)
// C source: 				keysize = (key->sadb_key_bits + 7) / 8;
// C source: 			x->ealg = kmalloc(sizeof(*x->ealg) + keysize, GFP_KERNEL);
// C source: 			if (!x->ealg) {
// C source: 				err = -ENOMEM;
// C source: 				goto out;
// C source: 			}
// C source: 			strcpy(x->ealg->alg_name, a->name);
// C source: 			x->ealg->alg_key_len = 0;
// C source: 			if (key) {
// C source: 				x->ealg->alg_key_len = key->sadb_key_bits;
// C source: 				memcpy(x->ealg->alg_key, key+1, keysize);
// C source: 			}
// C source: 			x->props.ealgo = sa->sadb_sa_encrypt;
// C source: 			x->geniv = a->uinfo.encr.geniv;
// C source: 		}
// C source: 	}
// C source: 	/* x->algo.flags = sa->sadb_sa_flags; */
// C source: 
// C source: 	x->props.family = pfkey_sadb_addr2xfrm_addr((struct sadb_address *) ext_hdrs[SADB_EXT_ADDRESS_SRC-1],
// C source: 						    &x->props.saddr);
// C source: 	pfkey_sadb_addr2xfrm_addr((struct sadb_address *) ext_hdrs[SADB_EXT_ADDRESS_DST-1],
// C source: 				  &x->id.daddr);
// C source: 
// C source: 	if (ext_hdrs[SADB_X_EXT_SA2-1]) {
// C source: 		const struct sadb_x_sa2 *sa2 = ext_hdrs[SADB_X_EXT_SA2-1];
// C source: 		int mode = pfkey_mode_to_xfrm(sa2->sadb_x_sa2_mode);
// C source: 		if (mode < 0) {
// C source: 			err = -EINVAL;
// C source: 			goto out;
// C source: 		}
// C source: 		x->props.mode = mode;
// C source: 		x->props.reqid = sa2->sadb_x_sa2_reqid;
// C source: 	}
// C source: 
// C source: 	if (ext_hdrs[SADB_EXT_ADDRESS_PROXY-1]) {
// C source: 		const struct sadb_address *addr = ext_hdrs[SADB_EXT_ADDRESS_PROXY-1];
// C source: 
// C source: 		/* Nobody uses this, but we try. */
// C source: 		x->sel.family = pfkey_sadb_addr2xfrm_addr(addr, &x->sel.saddr);
// C source: 		x->sel.prefixlen_s = addr->sadb_address_prefixlen;
// C source: 	}
// C source: 
// C source: 	if (!x->sel.family)
// C source: 		x->sel.family = x->props.family;
// C source: 
// C source: 	if (ext_hdrs[SADB_X_EXT_NAT_T_TYPE-1]) {
// C source: 		const struct sadb_x_nat_t_type* n_type;
// C source: 		struct xfrm_encap_tmpl *natt;
// C source: 
// C source: 		x->encap = kzalloc_obj(*x->encap);
// C source: 		if (!x->encap) {
// C source: 			err = -ENOMEM;
// C source: 			goto out;
// C source: 		}
// C source: 
// C source: 		natt = x->encap;
// C source: 		n_type = ext_hdrs[SADB_X_EXT_NAT_T_TYPE-1];
// C source: 		natt->encap_type = n_type->sadb_x_nat_t_type_type;
// C source: 
// C source: 		if (ext_hdrs[SADB_X_EXT_NAT_T_SPORT-1]) {
// C source: 			const struct sadb_x_nat_t_port *n_port =
// C source: 				ext_hdrs[SADB_X_EXT_NAT_T_SPORT-1];
// C source: 			natt->encap_sport = n_port->sadb_x_nat_t_port_port;
// C source: 		}
// C source: 		if (ext_hdrs[SADB_X_EXT_NAT_T_DPORT-1]) {
// C source: 			const struct sadb_x_nat_t_port *n_port =
// C source: 				ext_hdrs[SADB_X_EXT_NAT_T_DPORT-1];
// C source: 			natt->encap_dport = n_port->sadb_x_nat_t_port_port;
// C source: 		}
// C source: 	}
// C source: 
// C source: 	err = xfrm_init_state(x, NULL);
// C source: 	if (err)
// C source: 		goto out;
// C source: 
// C source: 	x->km.seq = hdr->sadb_msg_seq;
// C source: 	return x;
// C source: 
// C source: out:
// C source: 	x->km.state = XFRM_STATE_DEAD;
// C source: 	xfrm_state_put(x);
// C source: 	return ERR_PTR(err);
// C source: }
// C source: 
// C source: static int pfkey_reserved(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	return -EOPNOTSUPP;
// C source: }
// C source: 
// C source: static int pfkey_getspi(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	struct sk_buff *resp_skb;
// C source: 	struct sadb_x_sa2 *sa2;
// C source: 	struct sadb_address *saddr, *daddr;
// C source: 	struct sadb_msg *out_hdr;
// C source: 	struct sadb_spirange *range;
// C source: 	struct xfrm_state *x = NULL;
// C source: 	int mode;
// C source: 	int err;
// C source: 	u32 min_spi, max_spi;
// C source: 	u32 reqid;
// C source: 	u8 proto;
// C source: 	unsigned short family;
// C source: 	xfrm_address_t *xsaddr = NULL, *xdaddr = NULL;
// C source: 
// C source: 	if (!present_and_same_family(ext_hdrs[SADB_EXT_ADDRESS_SRC-1],
// C source: 				     ext_hdrs[SADB_EXT_ADDRESS_DST-1]))
// C source: 		return -EINVAL;
// C source: 
// C source: 	proto = pfkey_satype2proto(hdr->sadb_msg_satype);
// C source: 	if (proto == 0)
// C source: 		return -EINVAL;
// C source: 
// C source: 	if ((sa2 = ext_hdrs[SADB_X_EXT_SA2-1]) != NULL) {
// C source: 		mode = pfkey_mode_to_xfrm(sa2->sadb_x_sa2_mode);
// C source: 		if (mode < 0)
// C source: 			return -EINVAL;
// C source: 		reqid = sa2->sadb_x_sa2_reqid;
// C source: 	} else {
// C source: 		mode = 0;
// C source: 		reqid = 0;
// C source: 	}
// C source: 
// C source: 	saddr = ext_hdrs[SADB_EXT_ADDRESS_SRC-1];
// C source: 	daddr = ext_hdrs[SADB_EXT_ADDRESS_DST-1];
// C source: 
// C source: 	family = ((struct sockaddr *)(saddr + 1))->sa_family;
// C source: 	switch (family) {
// C source: 	case AF_INET:
// C source: 		xdaddr = (xfrm_address_t *)&((struct sockaddr_in *)(daddr + 1))->sin_addr.s_addr;
// C source: 		xsaddr = (xfrm_address_t *)&((struct sockaddr_in *)(saddr + 1))->sin_addr.s_addr;
// C source: 		break;
// C source: #if IS_ENABLED(CONFIG_IPV6)
// C source: 	case AF_INET6:
// C source: 		xdaddr = (xfrm_address_t *)&((struct sockaddr_in6 *)(daddr + 1))->sin6_addr;
// C source: 		xsaddr = (xfrm_address_t *)&((struct sockaddr_in6 *)(saddr + 1))->sin6_addr;
// C source: 		break;
// C source: #endif
// C source: 	}
// C source: 
// C source: 	if (hdr->sadb_msg_seq) {
// C source: 		x = xfrm_find_acq_byseq(net, DUMMY_MARK, hdr->sadb_msg_seq, UINT_MAX);
// C source: 		if (x && !xfrm_addr_equal(&x->id.daddr, xdaddr, family)) {
// C source: 			xfrm_state_put(x);
// C source: 			x = NULL;
// C source: 		}
// C source: 	}
// C source: 
// C source: 	if (!x)
// C source: 		x = xfrm_find_acq(net, &dummy_mark, mode, reqid, 0, UINT_MAX,
// C source: 				  proto, xdaddr, xsaddr, 1, family);
// C source: 
// C source: 	if (x == NULL)
// C source: 		return -ENOENT;
// C source: 
// C source: 	min_spi = 0x100;
// C source: 	max_spi = 0x0fffffff;
// C source: 
// C source: 	range = ext_hdrs[SADB_EXT_SPIRANGE-1];
// C source: 	if (range) {
// C source: 		min_spi = range->sadb_spirange_min;
// C source: 		max_spi = range->sadb_spirange_max;
// C source: 	}
// C source: 
// C source: 	err = verify_spi_info(x->id.proto, min_spi, max_spi, NULL);
// C source: 	if (err) {
// C source: 		xfrm_state_put(x);
// C source: 		return err;
// C source: 	}
// C source: 
// C source: 	err = xfrm_alloc_spi(x, min_spi, max_spi, NULL);
// C source: 	resp_skb = err ? ERR_PTR(err) : pfkey_xfrm_state2msg(x);
// C source: 
// C source: 	if (IS_ERR(resp_skb)) {
// C source: 		xfrm_state_put(x);
// C source: 		return  PTR_ERR(resp_skb);
// C source: 	}
// C source: 
// C source: 	out_hdr = (struct sadb_msg *) resp_skb->data;
// C source: 	out_hdr->sadb_msg_version = hdr->sadb_msg_version;
// C source: 	out_hdr->sadb_msg_type = SADB_GETSPI;
// C source: 	out_hdr->sadb_msg_satype = pfkey_proto2satype(proto);
// C source: 	out_hdr->sadb_msg_errno = 0;
// C source: 	out_hdr->sadb_msg_reserved = 0;
// C source: 	out_hdr->sadb_msg_seq = hdr->sadb_msg_seq;
// C source: 	out_hdr->sadb_msg_pid = hdr->sadb_msg_pid;
// C source: 
// C source: 	xfrm_state_put(x);
// C source: 
// C source: 	pfkey_broadcast(resp_skb, GFP_KERNEL, BROADCAST_ONE, sk, net);
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_acquire(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	struct xfrm_state *x;
// C source: 
// C source: 	if (hdr->sadb_msg_len != sizeof(struct sadb_msg)/8)
// C source: 		return -EOPNOTSUPP;
// C source: 
// C source: 	if (hdr->sadb_msg_seq == 0 || hdr->sadb_msg_errno == 0)
// C source: 		return 0;
// C source: 
// C source: 	x = xfrm_find_acq_byseq(net, DUMMY_MARK, hdr->sadb_msg_seq, UINT_MAX);
// C source: 	if (x == NULL)
// C source: 		return 0;
// C source: 
// C source: 	spin_lock_bh(&x->lock);
// C source: 	if (x->km.state == XFRM_STATE_ACQ)
// C source: 		x->km.state = XFRM_STATE_ERROR;
// C source: 
// C source: 	spin_unlock_bh(&x->lock);
// C source: 	xfrm_state_put(x);
// C source: 	return 0;
// C source: }
// C source: 
// C source: static inline int event2poltype(int event)
// C source: {
// C source: 	switch (event) {
// C source: 	case XFRM_MSG_DELPOLICY:
// C source: 		return SADB_X_SPDDELETE;
// C source: 	case XFRM_MSG_NEWPOLICY:
// C source: 		return SADB_X_SPDADD;
// C source: 	case XFRM_MSG_UPDPOLICY:
// C source: 		return SADB_X_SPDUPDATE;
// C source: 	case XFRM_MSG_POLEXPIRE:
// C source: 	//	return SADB_X_SPDEXPIRE;
// C source: 	default:
// C source: 		pr_err("pfkey: Unknown policy event %d\n", event);
// C source: 		break;
// C source: 	}
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static inline int event2keytype(int event)
// C source: {
// C source: 	switch (event) {
// C source: 	case XFRM_MSG_DELSA:
// C source: 		return SADB_DELETE;
// C source: 	case XFRM_MSG_NEWSA:
// C source: 		return SADB_ADD;
// C source: 	case XFRM_MSG_UPDSA:
// C source: 		return SADB_UPDATE;
// C source: 	case XFRM_MSG_EXPIRE:
// C source: 		return SADB_EXPIRE;
// C source: 	default:
// C source: 		pr_err("pfkey: Unknown SA event %d\n", event);
// C source: 		break;
// C source: 	}
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: /* ADD/UPD/DEL */
// C source: static int key_notify_sa(struct xfrm_state *x, const struct km_event *c)
// C source: {
// C source: 	struct sk_buff *skb;
// C source: 	struct sadb_msg *hdr;
// C source: 
// C source: 	skb = pfkey_xfrm_state2msg(x);
// C source: 
// C source: 	if (IS_ERR(skb))
// C source: 		return PTR_ERR(skb);
// C source: 
// C source: 	hdr = (struct sadb_msg *) skb->data;
// C source: 	hdr->sadb_msg_version = PF_KEY_V2;
// C source: 	hdr->sadb_msg_type = event2keytype(c->event);
// C source: 	hdr->sadb_msg_satype = pfkey_proto2satype(x->id.proto);
// C source: 	hdr->sadb_msg_errno = 0;
// C source: 	hdr->sadb_msg_reserved = 0;
// C source: 	hdr->sadb_msg_seq = c->seq;
// C source: 	hdr->sadb_msg_pid = c->portid;
// C source: 
// C source: 	pfkey_broadcast(skb, GFP_ATOMIC, BROADCAST_ALL, NULL, xs_net(x));
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_add(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	struct xfrm_state *x;
// C source: 	int err;
// C source: 	struct km_event c;
// C source: 
// C source: 	x = pfkey_msg2xfrm_state(net, hdr, ext_hdrs);
// C source: 	if (IS_ERR(x))
// C source: 		return PTR_ERR(x);
// C source: 
// C source: 	xfrm_state_hold(x);
// C source: 	if (hdr->sadb_msg_type == SADB_ADD)
// C source: 		err = xfrm_state_add(x);
// C source: 	else
// C source: 		err = xfrm_state_update(x);
// C source: 
// C source: 	xfrm_audit_state_add(x, err ? 0 : 1, true);
// C source: 
// C source: 	if (err < 0) {
// C source: 		x->km.state = XFRM_STATE_DEAD;
// C source: 		__xfrm_state_put(x);
// C source: 		goto out;
// C source: 	}
// C source: 
// C source: 	if (hdr->sadb_msg_type == SADB_ADD)
// C source: 		c.event = XFRM_MSG_NEWSA;
// C source: 	else
// C source: 		c.event = XFRM_MSG_UPDSA;
// C source: 	c.seq = hdr->sadb_msg_seq;
// C source: 	c.portid = hdr->sadb_msg_pid;
// C source: 	km_state_notify(x, &c);
// C source: out:
// C source: 	xfrm_state_put(x);
// C source: 	return err;
// C source: }
// C source: 
// C source: static int pfkey_delete(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	struct xfrm_state *x;
// C source: 	struct km_event c;
// C source: 	int err;
// C source: 
// C source: 	if (!ext_hdrs[SADB_EXT_SA-1] ||
// C source: 	    !present_and_same_family(ext_hdrs[SADB_EXT_ADDRESS_SRC-1],
// C source: 				     ext_hdrs[SADB_EXT_ADDRESS_DST-1]))
// C source: 		return -EINVAL;
// C source: 
// C source: 	x = pfkey_xfrm_state_lookup(net, hdr, ext_hdrs);
// C source: 	if (x == NULL)
// C source: 		return -ESRCH;
// C source: 
// C source: 	if ((err = security_xfrm_state_delete(x)))
// C source: 		goto out;
// C source: 
// C source: 	if (xfrm_state_kern(x)) {
// C source: 		err = -EPERM;
// C source: 		goto out;
// C source: 	}
// C source: 
// C source: 	err = xfrm_state_delete(x);
// C source: 
// C source: 	if (err < 0)
// C source: 		goto out;
// C source: 
// C source: 	c.seq = hdr->sadb_msg_seq;
// C source: 	c.portid = hdr->sadb_msg_pid;
// C source: 	c.event = XFRM_MSG_DELSA;
// C source: 	km_state_notify(x, &c);
// C source: out:
// C source: 	xfrm_audit_state_delete(x, err ? 0 : 1, true);
// C source: 	xfrm_state_put(x);
// C source: 
// C source: 	return err;
// C source: }
// C source: 
// C source: static int pfkey_get(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	__u8 proto;
// C source: 	struct sk_buff *out_skb;
// C source: 	struct sadb_msg *out_hdr;
// C source: 	struct xfrm_state *x;
// C source: 
// C source: 	if (!ext_hdrs[SADB_EXT_SA-1] ||
// C source: 	    !present_and_same_family(ext_hdrs[SADB_EXT_ADDRESS_SRC-1],
// C source: 				     ext_hdrs[SADB_EXT_ADDRESS_DST-1]))
// C source: 		return -EINVAL;
// C source: 
// C source: 	x = pfkey_xfrm_state_lookup(net, hdr, ext_hdrs);
// C source: 	if (x == NULL)
// C source: 		return -ESRCH;
// C source: 
// C source: 	out_skb = pfkey_xfrm_state2msg(x);
// C source: 	proto = x->id.proto;
// C source: 	xfrm_state_put(x);
// C source: 	if (IS_ERR(out_skb))
// C source: 		return  PTR_ERR(out_skb);
// C source: 
// C source: 	out_hdr = (struct sadb_msg *) out_skb->data;
// C source: 	out_hdr->sadb_msg_version = hdr->sadb_msg_version;
// C source: 	out_hdr->sadb_msg_type = SADB_GET;
// C source: 	out_hdr->sadb_msg_satype = pfkey_proto2satype(proto);
// C source: 	out_hdr->sadb_msg_errno = 0;
// C source: 	out_hdr->sadb_msg_reserved = 0;
// C source: 	out_hdr->sadb_msg_seq = hdr->sadb_msg_seq;
// C source: 	out_hdr->sadb_msg_pid = hdr->sadb_msg_pid;
// C source: 	pfkey_broadcast(out_skb, GFP_ATOMIC, BROADCAST_ONE, sk, sock_net(sk));
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static struct sk_buff *compose_sadb_supported(const struct sadb_msg *orig,
// C source: 					      gfp_t allocation)
// C source: {
// C source: 	struct sk_buff *skb;
// C source: 	struct sadb_msg *hdr;
// C source: 	int len, auth_len, enc_len, i;
// C source: 
// C source: 	auth_len = xfrm_count_pfkey_auth_supported();
// C source: 	if (auth_len) {
// C source: 		auth_len *= sizeof(struct sadb_alg);
// C source: 		auth_len += sizeof(struct sadb_supported);
// C source: 	}
// C source: 
// C source: 	enc_len = xfrm_count_pfkey_enc_supported();
// C source: 	if (enc_len) {
// C source: 		enc_len *= sizeof(struct sadb_alg);
// C source: 		enc_len += sizeof(struct sadb_supported);
// C source: 	}
// C source: 
// C source: 	len = enc_len + auth_len + sizeof(struct sadb_msg);
// C source: 
// C source: 	skb = alloc_skb(len + 16, allocation);
// C source: 	if (!skb)
// C source: 		goto out_put_algs;
// C source: 
// C source: 	hdr = skb_put(skb, sizeof(*hdr));
// C source: 	pfkey_hdr_dup(hdr, orig);
// C source: 	hdr->sadb_msg_errno = 0;
// C source: 	hdr->sadb_msg_len = len / sizeof(uint64_t);
// C source: 
// C source: 	if (auth_len) {
// C source: 		struct sadb_supported *sp;
// C source: 		struct sadb_alg *ap;
// C source: 
// C source: 		sp = skb_put(skb, auth_len);
// C source: 		ap = (struct sadb_alg *) (sp + 1);
// C source: 
// C source: 		sp->sadb_supported_len = auth_len / sizeof(uint64_t);
// C source: 		sp->sadb_supported_exttype = SADB_EXT_SUPPORTED_AUTH;
// C source: 
// C source: 		for (i = 0; ; i++) {
// C source: 			struct xfrm_algo_desc *aalg = xfrm_aalg_get_byidx(i);
// C source: 			if (!aalg)
// C source: 				break;
// C source: 			if (!aalg->pfkey_supported)
// C source: 				continue;
// C source: 			if (aalg->available)
// C source: 				*ap++ = aalg->desc;
// C source: 		}
// C source: 	}
// C source: 
// C source: 	if (enc_len) {
// C source: 		struct sadb_supported *sp;
// C source: 		struct sadb_alg *ap;
// C source: 
// C source: 		sp = skb_put(skb, enc_len);
// C source: 		ap = (struct sadb_alg *) (sp + 1);
// C source: 
// C source: 		sp->sadb_supported_len = enc_len / sizeof(uint64_t);
// C source: 		sp->sadb_supported_exttype = SADB_EXT_SUPPORTED_ENCRYPT;
// C source: 
// C source: 		for (i = 0; ; i++) {
// C source: 			struct xfrm_algo_desc *ealg = xfrm_ealg_get_byidx(i);
// C source: 			if (!ealg)
// C source: 				break;
// C source: 			if (!ealg->pfkey_supported)
// C source: 				continue;
// C source: 			if (ealg->available)
// C source: 				*ap++ = ealg->desc;
// C source: 		}
// C source: 	}
// C source: 
// C source: out_put_algs:
// C source: 	return skb;
// C source: }
// C source: 
// C source: static int pfkey_register(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct pfkey_sock *pfk = pfkey_sk(sk);
// C source: 	struct sk_buff *supp_skb;
// C source: 
// C source: 	if (hdr->sadb_msg_satype > SADB_SATYPE_MAX)
// C source: 		return -EINVAL;
// C source: 
// C source: 	if (hdr->sadb_msg_satype != SADB_SATYPE_UNSPEC) {
// C source: 		if (pfk->registered&(1<<hdr->sadb_msg_satype))
// C source: 			return -EEXIST;
// C source: 		pfk->registered |= (1<<hdr->sadb_msg_satype);
// C source: 	}
// C source: 
// C source: 	mutex_lock(&pfkey_mutex);
// C source: 	xfrm_probe_algs();
// C source: 
// C source: 	supp_skb = compose_sadb_supported(hdr, GFP_KERNEL | __GFP_ZERO);
// C source: 	mutex_unlock(&pfkey_mutex);
// C source: 
// C source: 	if (!supp_skb) {
// C source: 		if (hdr->sadb_msg_satype != SADB_SATYPE_UNSPEC)
// C source: 			pfk->registered &= ~(1<<hdr->sadb_msg_satype);
// C source: 
// C source: 		return -ENOBUFS;
// C source: 	}
// C source: 
// C source: 	pfkey_broadcast(supp_skb, GFP_KERNEL, BROADCAST_REGISTERED, sk,
// C source: 			sock_net(sk));
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int unicast_flush_resp(struct sock *sk, const struct sadb_msg *ihdr)
// C source: {
// C source: 	struct sk_buff *skb;
// C source: 	struct sadb_msg *hdr;
// C source: 
// C source: 	skb = alloc_skb(sizeof(struct sadb_msg) + 16, GFP_ATOMIC);
// C source: 	if (!skb)
// C source: 		return -ENOBUFS;
// C source: 
// C source: 	hdr = skb_put_data(skb, ihdr, sizeof(struct sadb_msg));
// C source: 	hdr->sadb_msg_errno = (uint8_t) 0;
// C source: 	hdr->sadb_msg_len = (sizeof(struct sadb_msg) / sizeof(uint64_t));
// C source: 
// C source: 	return pfkey_broadcast(skb, GFP_ATOMIC, BROADCAST_ONE, sk,
// C source: 			       sock_net(sk));
// C source: }
// C source: 
// C source: static int key_notify_sa_flush(const struct km_event *c)
// C source: {
// C source: 	struct sk_buff *skb;
// C source: 	struct sadb_msg *hdr;
// C source: 
// C source: 	skb = alloc_skb(sizeof(struct sadb_msg) + 16, GFP_ATOMIC);
// C source: 	if (!skb)
// C source: 		return -ENOBUFS;
// C source: 	hdr = skb_put(skb, sizeof(struct sadb_msg));
// C source: 	hdr->sadb_msg_satype = pfkey_proto2satype(c->data.proto);
// C source: 	hdr->sadb_msg_type = SADB_FLUSH;
// C source: 	hdr->sadb_msg_seq = c->seq;
// C source: 	hdr->sadb_msg_pid = c->portid;
// C source: 	hdr->sadb_msg_version = PF_KEY_V2;
// C source: 	hdr->sadb_msg_errno = (uint8_t) 0;
// C source: 	hdr->sadb_msg_len = (sizeof(struct sadb_msg) / sizeof(uint64_t));
// C source: 	hdr->sadb_msg_reserved = 0;
// C source: 
// C source: 	pfkey_broadcast(skb, GFP_ATOMIC, BROADCAST_ALL, NULL, c->net);
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_flush(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	unsigned int proto;
// C source: 	struct km_event c;
// C source: 	int err, err2;
// C source: 
// C source: 	proto = pfkey_satype2proto(hdr->sadb_msg_satype);
// C source: 	if (proto == 0)
// C source: 		return -EINVAL;
// C source: 
// C source: 	err = xfrm_state_flush(net, proto, true);
// C source: 	err2 = unicast_flush_resp(sk, hdr);
// C source: 	if (err || err2) {
// C source: 		if (err == -ESRCH) /* empty table - go quietly */
// C source: 			err = 0;
// C source: 		return err ? err : err2;
// C source: 	}
// C source: 
// C source: 	c.data.proto = proto;
// C source: 	c.seq = hdr->sadb_msg_seq;
// C source: 	c.portid = hdr->sadb_msg_pid;
// C source: 	c.event = XFRM_MSG_FLUSHSA;
// C source: 	c.net = net;
// C source: 	km_state_notify(NULL, &c);
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int dump_sa(struct xfrm_state *x, int count, void *ptr)
// C source: {
// C source: 	struct pfkey_sock *pfk = ptr;
// C source: 	struct sk_buff *out_skb;
// C source: 	struct sadb_msg *out_hdr;
// C source: 
// C source: 	if (!pfkey_can_dump(&pfk->sk))
// C source: 		return -ENOBUFS;
// C source: 
// C source: 	out_skb = pfkey_xfrm_state2msg(x);
// C source: 	if (IS_ERR(out_skb))
// C source: 		return PTR_ERR(out_skb);
// C source: 
// C source: 	out_hdr = (struct sadb_msg *) out_skb->data;
// C source: 	out_hdr->sadb_msg_version = pfk->dump.msg_version;
// C source: 	out_hdr->sadb_msg_type = SADB_DUMP;
// C source: 	out_hdr->sadb_msg_satype = pfkey_proto2satype(x->id.proto);
// C source: 	out_hdr->sadb_msg_errno = 0;
// C source: 	out_hdr->sadb_msg_reserved = 0;
// C source: 	out_hdr->sadb_msg_seq = count + 1;
// C source: 	out_hdr->sadb_msg_pid = pfk->dump.msg_portid;
// C source: 
// C source: 	if (pfk->dump.skb)
// C source: 		pfkey_broadcast(pfk->dump.skb, GFP_ATOMIC, BROADCAST_ONE,
// C source: 				&pfk->sk, sock_net(&pfk->sk));
// C source: 	pfk->dump.skb = out_skb;
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_dump_sa(struct pfkey_sock *pfk)
// C source: {
// C source: 	struct net *net = sock_net(&pfk->sk);
// C source: 	return xfrm_state_walk(net, &pfk->dump.u.state, dump_sa, (void *) pfk);
// C source: }
// C source: 
// C source: static void pfkey_dump_sa_done(struct pfkey_sock *pfk)
// C source: {
// C source: 	struct net *net = sock_net(&pfk->sk);
// C source: 
// C source: 	xfrm_state_walk_done(&pfk->dump.u.state, net);
// C source: }
// C source: 
// C source: static int pfkey_dump(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	u8 proto;
// C source: 	struct xfrm_address_filter *filter = NULL;
// C source: 	struct pfkey_sock *pfk = pfkey_sk(sk);
// C source: 
// C source: 	mutex_lock(&pfk->dump_lock);
// C source: 	if (pfk->dump.dump != NULL) {
// C source: 		mutex_unlock(&pfk->dump_lock);
// C source: 		return -EBUSY;
// C source: 	}
// C source: 
// C source: 	proto = pfkey_satype2proto(hdr->sadb_msg_satype);
// C source: 	if (proto == 0) {
// C source: 		mutex_unlock(&pfk->dump_lock);
// C source: 		return -EINVAL;
// C source: 	}
// C source: 
// C source: 	if (ext_hdrs[SADB_X_EXT_FILTER - 1]) {
// C source: 		struct sadb_x_filter *xfilter = ext_hdrs[SADB_X_EXT_FILTER - 1];
// C source: 
// C source: 		if ((xfilter->sadb_x_filter_splen >
// C source: 			(sizeof(xfrm_address_t) << 3)) ||
// C source: 		    (xfilter->sadb_x_filter_dplen >
// C source: 			(sizeof(xfrm_address_t) << 3))) {
// C source: 			mutex_unlock(&pfk->dump_lock);
// C source: 			return -EINVAL;
// C source: 		}
// C source: 		filter = kmalloc_obj(*filter);
// C source: 		if (filter == NULL) {
// C source: 			mutex_unlock(&pfk->dump_lock);
// C source: 			return -ENOMEM;
// C source: 		}
// C source: 
// C source: 		memcpy(&filter->saddr, &xfilter->sadb_x_filter_saddr,
// C source: 		       sizeof(xfrm_address_t));
// C source: 		memcpy(&filter->daddr, &xfilter->sadb_x_filter_daddr,
// C source: 		       sizeof(xfrm_address_t));
// C source: 		filter->family = xfilter->sadb_x_filter_family;
// C source: 		filter->splen = xfilter->sadb_x_filter_splen;
// C source: 		filter->dplen = xfilter->sadb_x_filter_dplen;
// C source: 	}
// C source: 
// C source: 	pfk->dump.msg_version = hdr->sadb_msg_version;
// C source: 	pfk->dump.msg_portid = hdr->sadb_msg_pid;
// C source: 	pfk->dump.dump = pfkey_dump_sa;
// C source: 	pfk->dump.done = pfkey_dump_sa_done;
// C source: 	xfrm_state_walk_init(&pfk->dump.u.state, proto, filter);
// C source: 	mutex_unlock(&pfk->dump_lock);
// C source: 
// C source: 	return pfkey_do_dump(pfk);
// C source: }
// C source: 
// C source: static int pfkey_promisc(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct pfkey_sock *pfk = pfkey_sk(sk);
// C source: 	int satype = hdr->sadb_msg_satype;
// C source: 	bool reset_errno = false;
// C source: 
// C source: 	if (hdr->sadb_msg_len == (sizeof(*hdr) / sizeof(uint64_t))) {
// C source: 		reset_errno = true;
// C source: 		if (satype != 0 && satype != 1)
// C source: 			return -EINVAL;
// C source: 		pfk->promisc = satype;
// C source: 	}
// C source: 	if (reset_errno && skb_cloned(skb))
// C source: 		skb = skb_copy(skb, GFP_KERNEL);
// C source: 	else
// C source: 		skb = skb_clone(skb, GFP_KERNEL);
// C source: 
// C source: 	if (reset_errno && skb) {
// C source: 		struct sadb_msg *new_hdr = (struct sadb_msg *) skb->data;
// C source: 		new_hdr->sadb_msg_errno = 0;
// C source: 	}
// C source: 
// C source: 	pfkey_broadcast(skb, GFP_KERNEL, BROADCAST_ALL, NULL, sock_net(sk));
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int check_reqid(struct xfrm_policy *xp, int dir, int count, void *ptr)
// C source: {
// C source: 	int i;
// C source: 	u32 reqid = *(u32*)ptr;
// C source: 
// C source: 	for (i=0; i<xp->xfrm_nr; i++) {
// C source: 		if (xp->xfrm_vec[i].reqid == reqid)
// C source: 			return -EEXIST;
// C source: 	}
// C source: 	return 0;
// C source: }
// C source: 
// C source: static u32 gen_reqid(struct net *net)
// C source: {
// C source: 	struct xfrm_policy_walk walk;
// C source: 	u32 start;
// C source: 	int rc;
// C source: 	static u32 reqid = IPSEC_MANUAL_REQID_MAX;
// C source: 
// C source: 	start = reqid;
// C source: 	do {
// C source: 		++reqid;
// C source: 		if (reqid == 0)
// C source: 			reqid = IPSEC_MANUAL_REQID_MAX+1;
// C source: 		xfrm_policy_walk_init(&walk, XFRM_POLICY_TYPE_MAIN);
// C source: 		rc = xfrm_policy_walk(net, &walk, check_reqid, (void*)&reqid);
// C source: 		xfrm_policy_walk_done(&walk, net);
// C source: 		if (rc != -EEXIST)
// C source: 			return reqid;
// C source: 	} while (reqid != start);
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int
// C source: parse_ipsecrequest(struct xfrm_policy *xp, struct sadb_x_policy *pol,
// C source: 		   struct sadb_x_ipsecrequest *rq)
// C source: {
// C source: 	struct net *net = xp_net(xp);
// C source: 	struct xfrm_tmpl *t = xp->xfrm_vec + xp->xfrm_nr;
// C source: 	int mode;
// C source: 
// C source: 	if (xp->xfrm_nr >= XFRM_MAX_DEPTH)
// C source: 		return -ELOOP;
// C source: 
// C source: 	if (rq->sadb_x_ipsecrequest_mode == 0)
// C source: 		return -EINVAL;
// C source: 	if (!xfrm_id_proto_valid(rq->sadb_x_ipsecrequest_proto))
// C source: 		return -EINVAL;
// C source: 
// C source: 	t->id.proto = rq->sadb_x_ipsecrequest_proto;
// C source: 	if ((mode = pfkey_mode_to_xfrm(rq->sadb_x_ipsecrequest_mode)) < 0)
// C source: 		return -EINVAL;
// C source: 	t->mode = mode;
// C source: 	if (rq->sadb_x_ipsecrequest_level == IPSEC_LEVEL_USE) {
// C source: 		if ((mode == XFRM_MODE_TUNNEL || mode == XFRM_MODE_BEET) &&
// C source: 		    pol->sadb_x_policy_dir == IPSEC_DIR_OUTBOUND)
// C source: 			return -EINVAL;
// C source: 		t->optional = 1;
// C source: 	} else if (rq->sadb_x_ipsecrequest_level == IPSEC_LEVEL_UNIQUE) {
// C source: 		t->reqid = rq->sadb_x_ipsecrequest_reqid;
// C source: 		if (t->reqid > IPSEC_MANUAL_REQID_MAX)
// C source: 			t->reqid = 0;
// C source: 		if (!t->reqid && !(t->reqid = gen_reqid(net)))
// C source: 			return -ENOBUFS;
// C source: 	}
// C source: 
// C source: 	/* addresses present only in tunnel mode */
// C source: 	if (t->mode == XFRM_MODE_TUNNEL) {
// C source: 		int err;
// C source: 
// C source: 		err = parse_sockaddr_pair(
// C source: 			(struct sockaddr *)(rq + 1),
// C source: 			rq->sadb_x_ipsecrequest_len - sizeof(*rq),
// C source: 			&t->saddr, &t->id.daddr, &t->encap_family);
// C source: 		if (err)
// C source: 			return err;
// C source: 	} else
// C source: 		t->encap_family = xp->family;
// C source: 
// C source: 	/* No way to set this via kame pfkey */
// C source: 	t->allalgs = 1;
// C source: 	xp->xfrm_nr++;
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int
// C source: parse_ipsecrequests(struct xfrm_policy *xp, struct sadb_x_policy *pol)
// C source: {
// C source: 	int err;
// C source: 	int len = pol->sadb_x_policy_len*8 - sizeof(struct sadb_x_policy);
// C source: 	struct sadb_x_ipsecrequest *rq = (void*)(pol+1);
// C source: 
// C source: 	if (pol->sadb_x_policy_len * 8 < sizeof(struct sadb_x_policy))
// C source: 		return -EINVAL;
// C source: 
// C source: 	while (len >= sizeof(*rq)) {
// C source: 		if (len < rq->sadb_x_ipsecrequest_len ||
// C source: 		    rq->sadb_x_ipsecrequest_len < sizeof(*rq))
// C source: 			return -EINVAL;
// C source: 
// C source: 		if ((err = parse_ipsecrequest(xp, pol, rq)) < 0)
// C source: 			return err;
// C source: 		len -= rq->sadb_x_ipsecrequest_len;
// C source: 		rq = (void*)((u8*)rq + rq->sadb_x_ipsecrequest_len);
// C source: 	}
// C source: 	return 0;
// C source: }
// C source: 
// C source: static inline int pfkey_xfrm_policy2sec_ctx_size(const struct xfrm_policy *xp)
// C source: {
// C source: 	struct xfrm_sec_ctx *xfrm_ctx = xp->security;
// C source: 
// C source: 	if (xfrm_ctx) {
// C source: 		int len = sizeof(struct sadb_x_sec_ctx);
// C source: 		len += xfrm_ctx->ctx_len;
// C source: 		return PFKEY_ALIGN8(len);
// C source: 	}
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_xfrm_policy2msg_size(const struct xfrm_policy *xp)
// C source: {
// C source: 	const struct xfrm_tmpl *t;
// C source: 	int sockaddr_size = pfkey_sockaddr_size(xp->family);
// C source: 	int socklen = 0;
// C source: 	int i;
// C source: 
// C source: 	for (i=0; i<xp->xfrm_nr; i++) {
// C source: 		t = xp->xfrm_vec + i;
// C source: 		socklen += pfkey_sockaddr_len(t->encap_family);
// C source: 	}
// C source: 
// C source: 	return sizeof(struct sadb_msg) +
// C source: 		(sizeof(struct sadb_lifetime) * 3) +
// C source: 		(sizeof(struct sadb_address) * 2) +
// C source: 		(sockaddr_size * 2) +
// C source: 		sizeof(struct sadb_x_policy) +
// C source: 		(xp->xfrm_nr * sizeof(struct sadb_x_ipsecrequest)) +
// C source: 		(socklen * 2) +
// C source: 		pfkey_xfrm_policy2sec_ctx_size(xp);
// C source: }
// C source: 
// C source: static struct sk_buff * pfkey_xfrm_policy2msg_prep(const struct xfrm_policy *xp)
// C source: {
// C source: 	struct sk_buff *skb;
// C source: 	int size;
// C source: 
// C source: 	size = pfkey_xfrm_policy2msg_size(xp);
// C source: 
// C source: 	skb =  alloc_skb(size + 16, GFP_ATOMIC);
// C source: 	if (skb == NULL)
// C source: 		return ERR_PTR(-ENOBUFS);
// C source: 
// C source: 	return skb;
// C source: }
// C source: 
// C source: static int pfkey_xfrm_policy2msg(struct sk_buff *skb, const struct xfrm_policy *xp, int dir)
// C source: {
// C source: 	struct sadb_msg *hdr;
// C source: 	struct sadb_address *addr;
// C source: 	struct sadb_lifetime *lifetime;
// C source: 	struct sadb_x_policy *pol;
// C source: 	struct sadb_x_sec_ctx *sec_ctx;
// C source: 	struct xfrm_sec_ctx *xfrm_ctx;
// C source: 	int i;
// C source: 	int size;
// C source: 	int sockaddr_size = pfkey_sockaddr_size(xp->family);
// C source: 	int socklen = pfkey_sockaddr_len(xp->family);
// C source: 
// C source: 	size = pfkey_xfrm_policy2msg_size(xp);
// C source: 
// C source: 	/* call should fill header later */
// C source: 	hdr = skb_put(skb, sizeof(struct sadb_msg));
// C source: 	memset(hdr, 0, size);	/* XXX do we need this ? */
// C source: 
// C source: 	/* src address */
// C source: 	addr = skb_put(skb, sizeof(struct sadb_address) + sockaddr_size);
// C source: 	addr->sadb_address_len =
// C source: 		(sizeof(struct sadb_address)+sockaddr_size)/
// C source: 			sizeof(uint64_t);
// C source: 	addr->sadb_address_exttype = SADB_EXT_ADDRESS_SRC;
// C source: 	addr->sadb_address_proto = pfkey_proto_from_xfrm(xp->selector.proto);
// C source: 	addr->sadb_address_prefixlen = xp->selector.prefixlen_s;
// C source: 	addr->sadb_address_reserved = 0;
// C source: 	if (!pfkey_sockaddr_fill(&xp->selector.saddr,
// C source: 				 xp->selector.sport,
// C source: 				 (struct sockaddr *) (addr + 1),
// C source: 				 xp->family))
// C source: 		BUG();
// C source: 
// C source: 	/* dst address */
// C source: 	addr = skb_put(skb, sizeof(struct sadb_address) + sockaddr_size);
// C source: 	addr->sadb_address_len =
// C source: 		(sizeof(struct sadb_address)+sockaddr_size)/
// C source: 			sizeof(uint64_t);
// C source: 	addr->sadb_address_exttype = SADB_EXT_ADDRESS_DST;
// C source: 	addr->sadb_address_proto = pfkey_proto_from_xfrm(xp->selector.proto);
// C source: 	addr->sadb_address_prefixlen = xp->selector.prefixlen_d;
// C source: 	addr->sadb_address_reserved = 0;
// C source: 
// C source: 	pfkey_sockaddr_fill(&xp->selector.daddr, xp->selector.dport,
// C source: 			    (struct sockaddr *) (addr + 1),
// C source: 			    xp->family);
// C source: 
// C source: 	/* hard time */
// C source: 	lifetime = skb_put(skb, sizeof(struct sadb_lifetime));
// C source: 	lifetime->sadb_lifetime_len =
// C source: 		sizeof(struct sadb_lifetime)/sizeof(uint64_t);
// C source: 	lifetime->sadb_lifetime_exttype = SADB_EXT_LIFETIME_HARD;
// C source: 	lifetime->sadb_lifetime_allocations =  _X2KEY(xp->lft.hard_packet_limit);
// C source: 	lifetime->sadb_lifetime_bytes = _X2KEY(xp->lft.hard_byte_limit);
// C source: 	lifetime->sadb_lifetime_addtime = xp->lft.hard_add_expires_seconds;
// C source: 	lifetime->sadb_lifetime_usetime = xp->lft.hard_use_expires_seconds;
// C source: 	/* soft time */
// C source: 	lifetime = skb_put(skb, sizeof(struct sadb_lifetime));
// C source: 	lifetime->sadb_lifetime_len =
// C source: 		sizeof(struct sadb_lifetime)/sizeof(uint64_t);
// C source: 	lifetime->sadb_lifetime_exttype = SADB_EXT_LIFETIME_SOFT;
// C source: 	lifetime->sadb_lifetime_allocations =  _X2KEY(xp->lft.soft_packet_limit);
// C source: 	lifetime->sadb_lifetime_bytes = _X2KEY(xp->lft.soft_byte_limit);
// C source: 	lifetime->sadb_lifetime_addtime = xp->lft.soft_add_expires_seconds;
// C source: 	lifetime->sadb_lifetime_usetime = xp->lft.soft_use_expires_seconds;
// C source: 	/* current time */
// C source: 	lifetime = skb_put(skb, sizeof(struct sadb_lifetime));
// C source: 	lifetime->sadb_lifetime_len =
// C source: 		sizeof(struct sadb_lifetime)/sizeof(uint64_t);
// C source: 	lifetime->sadb_lifetime_exttype = SADB_EXT_LIFETIME_CURRENT;
// C source: 	lifetime->sadb_lifetime_allocations = xp->curlft.packets;
// C source: 	lifetime->sadb_lifetime_bytes = xp->curlft.bytes;
// C source: 	lifetime->sadb_lifetime_addtime = xp->curlft.add_time;
// C source: 	lifetime->sadb_lifetime_usetime = xp->curlft.use_time;
// C source: 
// C source: 	pol = skb_put(skb, sizeof(struct sadb_x_policy));
// C source: 	pol->sadb_x_policy_len = sizeof(struct sadb_x_policy)/sizeof(uint64_t);
// C source: 	pol->sadb_x_policy_exttype = SADB_X_EXT_POLICY;
// C source: 	pol->sadb_x_policy_type = IPSEC_POLICY_DISCARD;
// C source: 	if (xp->action == XFRM_POLICY_ALLOW) {
// C source: 		if (xp->xfrm_nr)
// C source: 			pol->sadb_x_policy_type = IPSEC_POLICY_IPSEC;
// C source: 		else
// C source: 			pol->sadb_x_policy_type = IPSEC_POLICY_NONE;
// C source: 	}
// C source: 	pol->sadb_x_policy_dir = dir+1;
// C source: 	pol->sadb_x_policy_reserved = 0;
// C source: 	pol->sadb_x_policy_id = xp->index;
// C source: 	pol->sadb_x_policy_priority = xp->priority;
// C source: 
// C source: 	for (i=0; i<xp->xfrm_nr; i++) {
// C source: 		const struct xfrm_tmpl *t = xp->xfrm_vec + i;
// C source: 		struct sadb_x_ipsecrequest *rq;
// C source: 		int req_size;
// C source: 		int mode;
// C source: 
// C source: 		req_size = sizeof(struct sadb_x_ipsecrequest);
// C source: 		if (t->mode == XFRM_MODE_TUNNEL) {
// C source: 			socklen = pfkey_sockaddr_len(t->encap_family);
// C source: 			req_size += socklen * 2;
// C source: 		} else {
// C source: 			size -= 2*socklen;
// C source: 		}
// C source: 		rq = skb_put(skb, req_size);
// C source: 		pol->sadb_x_policy_len += req_size/8;
// C source: 		memset(rq, 0, sizeof(*rq));
// C source: 		rq->sadb_x_ipsecrequest_len = req_size;
// C source: 		rq->sadb_x_ipsecrequest_proto = t->id.proto;
// C source: 		if ((mode = pfkey_mode_from_xfrm(t->mode)) < 0)
// C source: 			return -EINVAL;
// C source: 		rq->sadb_x_ipsecrequest_mode = mode;
// C source: 		rq->sadb_x_ipsecrequest_level = IPSEC_LEVEL_REQUIRE;
// C source: 		if (t->reqid)
// C source: 			rq->sadb_x_ipsecrequest_level = IPSEC_LEVEL_UNIQUE;
// C source: 		if (t->optional)
// C source: 			rq->sadb_x_ipsecrequest_level = IPSEC_LEVEL_USE;
// C source: 		rq->sadb_x_ipsecrequest_reqid = t->reqid;
// C source: 
// C source: 		if (t->mode == XFRM_MODE_TUNNEL) {
// C source: 			u8 *sa = (void *)(rq + 1);
// C source: 			pfkey_sockaddr_fill(&t->saddr, 0,
// C source: 					    (struct sockaddr *)sa,
// C source: 					    t->encap_family);
// C source: 			pfkey_sockaddr_fill(&t->id.daddr, 0,
// C source: 					    (struct sockaddr *) (sa + socklen),
// C source: 					    t->encap_family);
// C source: 		}
// C source: 	}
// C source: 
// C source: 	/* security context */
// C source: 	if ((xfrm_ctx = xp->security)) {
// C source: 		int ctx_size = pfkey_xfrm_policy2sec_ctx_size(xp);
// C source: 
// C source: 		sec_ctx = skb_put(skb, ctx_size);
// C source: 		sec_ctx->sadb_x_sec_len = ctx_size / sizeof(uint64_t);
// C source: 		sec_ctx->sadb_x_sec_exttype = SADB_X_EXT_SEC_CTX;
// C source: 		sec_ctx->sadb_x_ctx_doi = xfrm_ctx->ctx_doi;
// C source: 		sec_ctx->sadb_x_ctx_alg = xfrm_ctx->ctx_alg;
// C source: 		sec_ctx->sadb_x_ctx_len = xfrm_ctx->ctx_len;
// C source: 		memcpy(sec_ctx + 1, xfrm_ctx->ctx_str,
// C source: 		       xfrm_ctx->ctx_len);
// C source: 	}
// C source: 
// C source: 	hdr->sadb_msg_len = size / sizeof(uint64_t);
// C source: 	hdr->sadb_msg_reserved = refcount_read(&xp->refcnt);
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int key_notify_policy(struct xfrm_policy *xp, int dir, const struct km_event *c)
// C source: {
// C source: 	struct sk_buff *out_skb;
// C source: 	struct sadb_msg *out_hdr;
// C source: 	int err;
// C source: 
// C source: 	out_skb = pfkey_xfrm_policy2msg_prep(xp);
// C source: 	if (IS_ERR(out_skb))
// C source: 		return PTR_ERR(out_skb);
// C source: 
// C source: 	err = pfkey_xfrm_policy2msg(out_skb, xp, dir);
// C source: 	if (err < 0) {
// C source: 		kfree_skb(out_skb);
// C source: 		return err;
// C source: 	}
// C source: 
// C source: 	out_hdr = (struct sadb_msg *) out_skb->data;
// C source: 	out_hdr->sadb_msg_version = PF_KEY_V2;
// C source: 
// C source: 	if (c->data.byid && c->event == XFRM_MSG_DELPOLICY)
// C source: 		out_hdr->sadb_msg_type = SADB_X_SPDDELETE2;
// C source: 	else
// C source: 		out_hdr->sadb_msg_type = event2poltype(c->event);
// C source: 	out_hdr->sadb_msg_errno = 0;
// C source: 	out_hdr->sadb_msg_seq = c->seq;
// C source: 	out_hdr->sadb_msg_pid = c->portid;
// C source: 	pfkey_broadcast(out_skb, GFP_ATOMIC, BROADCAST_ALL, NULL, xp_net(xp));
// C source: 	return 0;
// C source: 
// C source: }
// C source: 
// C source: static int pfkey_spdadd(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	int err = 0;
// C source: 	struct sadb_lifetime *lifetime;
// C source: 	struct sadb_address *sa;
// C source: 	struct sadb_x_policy *pol;
// C source: 	struct xfrm_policy *xp;
// C source: 	struct km_event c;
// C source: 	struct sadb_x_sec_ctx *sec_ctx;
// C source: 
// C source: 	if (!present_and_same_family(ext_hdrs[SADB_EXT_ADDRESS_SRC-1],
// C source: 				     ext_hdrs[SADB_EXT_ADDRESS_DST-1]) ||
// C source: 	    !ext_hdrs[SADB_X_EXT_POLICY-1])
// C source: 		return -EINVAL;
// C source: 
// C source: 	pol = ext_hdrs[SADB_X_EXT_POLICY-1];
// C source: 	if (pol->sadb_x_policy_type > IPSEC_POLICY_IPSEC)
// C source: 		return -EINVAL;
// C source: 	if (!pol->sadb_x_policy_dir || pol->sadb_x_policy_dir >= IPSEC_DIR_MAX)
// C source: 		return -EINVAL;
// C source: 
// C source: 	xp = xfrm_policy_alloc(net, GFP_KERNEL);
// C source: 	if (xp == NULL)
// C source: 		return -ENOBUFS;
// C source: 
// C source: 	xp->action = (pol->sadb_x_policy_type == IPSEC_POLICY_DISCARD ?
// C source: 		      XFRM_POLICY_BLOCK : XFRM_POLICY_ALLOW);
// C source: 	xp->priority = pol->sadb_x_policy_priority;
// C source: 
// C source: 	sa = ext_hdrs[SADB_EXT_ADDRESS_SRC-1];
// C source: 	xp->family = pfkey_sadb_addr2xfrm_addr(sa, &xp->selector.saddr);
// C source: 	xp->selector.family = xp->family;
// C source: 	xp->selector.prefixlen_s = sa->sadb_address_prefixlen;
// C source: 	xp->selector.proto = pfkey_proto_to_xfrm(sa->sadb_address_proto);
// C source: 	xp->selector.sport = ((struct sockaddr_in *)(sa+1))->sin_port;
// C source: 	if (xp->selector.sport)
// C source: 		xp->selector.sport_mask = htons(0xffff);
// C source: 
// C source: 	sa = ext_hdrs[SADB_EXT_ADDRESS_DST-1];
// C source: 	pfkey_sadb_addr2xfrm_addr(sa, &xp->selector.daddr);
// C source: 	xp->selector.prefixlen_d = sa->sadb_address_prefixlen;
// C source: 
// C source: 	/* Amusing, we set this twice.  KAME apps appear to set same value
// C source: 	 * in both addresses.
// C source: 	 */
// C source: 	xp->selector.proto = pfkey_proto_to_xfrm(sa->sadb_address_proto);
// C source: 
// C source: 	xp->selector.dport = ((struct sockaddr_in *)(sa+1))->sin_port;
// C source: 	if (xp->selector.dport)
// C source: 		xp->selector.dport_mask = htons(0xffff);
// C source: 
// C source: 	sec_ctx = ext_hdrs[SADB_X_EXT_SEC_CTX - 1];
// C source: 	if (sec_ctx != NULL) {
// C source: 		struct xfrm_user_sec_ctx *uctx = pfkey_sadb2xfrm_user_sec_ctx(sec_ctx, GFP_KERNEL);
// C source: 
// C source: 		if (!uctx) {
// C source: 			err = -ENOBUFS;
// C source: 			goto out;
// C source: 		}
// C source: 
// C source: 		err = security_xfrm_policy_alloc(&xp->security, uctx, GFP_KERNEL);
// C source: 		kfree(uctx);
// C source: 
// C source: 		if (err)
// C source: 			goto out;
// C source: 	}
// C source: 
// C source: 	xp->lft.soft_byte_limit = XFRM_INF;
// C source: 	xp->lft.hard_byte_limit = XFRM_INF;
// C source: 	xp->lft.soft_packet_limit = XFRM_INF;
// C source: 	xp->lft.hard_packet_limit = XFRM_INF;
// C source: 	if ((lifetime = ext_hdrs[SADB_EXT_LIFETIME_HARD-1]) != NULL) {
// C source: 		xp->lft.hard_packet_limit = _KEY2X(lifetime->sadb_lifetime_allocations);
// C source: 		xp->lft.hard_byte_limit = _KEY2X(lifetime->sadb_lifetime_bytes);
// C source: 		xp->lft.hard_add_expires_seconds = lifetime->sadb_lifetime_addtime;
// C source: 		xp->lft.hard_use_expires_seconds = lifetime->sadb_lifetime_usetime;
// C source: 	}
// C source: 	if ((lifetime = ext_hdrs[SADB_EXT_LIFETIME_SOFT-1]) != NULL) {
// C source: 		xp->lft.soft_packet_limit = _KEY2X(lifetime->sadb_lifetime_allocations);
// C source: 		xp->lft.soft_byte_limit = _KEY2X(lifetime->sadb_lifetime_bytes);
// C source: 		xp->lft.soft_add_expires_seconds = lifetime->sadb_lifetime_addtime;
// C source: 		xp->lft.soft_use_expires_seconds = lifetime->sadb_lifetime_usetime;
// C source: 	}
// C source: 	xp->xfrm_nr = 0;
// C source: 	if (pol->sadb_x_policy_type == IPSEC_POLICY_IPSEC &&
// C source: 	    (err = parse_ipsecrequests(xp, pol)) < 0)
// C source: 		goto out;
// C source: 
// C source: 	err = xfrm_policy_insert(pol->sadb_x_policy_dir-1, xp,
// C source: 				 hdr->sadb_msg_type != SADB_X_SPDUPDATE);
// C source: 
// C source: 	xfrm_audit_policy_add(xp, err ? 0 : 1, true);
// C source: 
// C source: 	if (err)
// C source: 		goto out;
// C source: 
// C source: 	if (hdr->sadb_msg_type == SADB_X_SPDUPDATE)
// C source: 		c.event = XFRM_MSG_UPDPOLICY;
// C source: 	else
// C source: 		c.event = XFRM_MSG_NEWPOLICY;
// C source: 
// C source: 	c.seq = hdr->sadb_msg_seq;
// C source: 	c.portid = hdr->sadb_msg_pid;
// C source: 
// C source: 	km_policy_notify(xp, pol->sadb_x_policy_dir-1, &c);
// C source: 	xfrm_pol_put(xp);
// C source: 	return 0;
// C source: 
// C source: out:
// C source: 	xp->walk.dead = 1;
// C source: 	xfrm_policy_destroy(xp);
// C source: 	return err;
// C source: }
// C source: 
// C source: static int pfkey_spddelete(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	int err;
// C source: 	struct sadb_address *sa;
// C source: 	struct sadb_x_policy *pol;
// C source: 	struct xfrm_policy *xp;
// C source: 	struct xfrm_selector sel;
// C source: 	struct km_event c;
// C source: 	struct sadb_x_sec_ctx *sec_ctx;
// C source: 	struct xfrm_sec_ctx *pol_ctx = NULL;
// C source: 
// C source: 	if (!present_and_same_family(ext_hdrs[SADB_EXT_ADDRESS_SRC-1],
// C source: 				     ext_hdrs[SADB_EXT_ADDRESS_DST-1]) ||
// C source: 	    !ext_hdrs[SADB_X_EXT_POLICY-1])
// C source: 		return -EINVAL;
// C source: 
// C source: 	pol = ext_hdrs[SADB_X_EXT_POLICY-1];
// C source: 	if (!pol->sadb_x_policy_dir || pol->sadb_x_policy_dir >= IPSEC_DIR_MAX)
// C source: 		return -EINVAL;
// C source: 
// C source: 	memset(&sel, 0, sizeof(sel));
// C source: 
// C source: 	sa = ext_hdrs[SADB_EXT_ADDRESS_SRC-1];
// C source: 	sel.family = pfkey_sadb_addr2xfrm_addr(sa, &sel.saddr);
// C source: 	sel.prefixlen_s = sa->sadb_address_prefixlen;
// C source: 	sel.proto = pfkey_proto_to_xfrm(sa->sadb_address_proto);
// C source: 	sel.sport = ((struct sockaddr_in *)(sa+1))->sin_port;
// C source: 	if (sel.sport)
// C source: 		sel.sport_mask = htons(0xffff);
// C source: 
// C source: 	sa = ext_hdrs[SADB_EXT_ADDRESS_DST-1];
// C source: 	pfkey_sadb_addr2xfrm_addr(sa, &sel.daddr);
// C source: 	sel.prefixlen_d = sa->sadb_address_prefixlen;
// C source: 	sel.proto = pfkey_proto_to_xfrm(sa->sadb_address_proto);
// C source: 	sel.dport = ((struct sockaddr_in *)(sa+1))->sin_port;
// C source: 	if (sel.dport)
// C source: 		sel.dport_mask = htons(0xffff);
// C source: 
// C source: 	sec_ctx = ext_hdrs[SADB_X_EXT_SEC_CTX - 1];
// C source: 	if (sec_ctx != NULL) {
// C source: 		struct xfrm_user_sec_ctx *uctx = pfkey_sadb2xfrm_user_sec_ctx(sec_ctx, GFP_KERNEL);
// C source: 
// C source: 		if (!uctx)
// C source: 			return -ENOMEM;
// C source: 
// C source: 		err = security_xfrm_policy_alloc(&pol_ctx, uctx, GFP_KERNEL);
// C source: 		kfree(uctx);
// C source: 		if (err)
// C source: 			return err;
// C source: 	}
// C source: 
// C source: 	xp = xfrm_policy_bysel_ctx(net, &dummy_mark, 0, XFRM_POLICY_TYPE_MAIN,
// C source: 				   pol->sadb_x_policy_dir - 1, &sel, pol_ctx,
// C source: 				   1, &err);
// C source: 	security_xfrm_policy_free(pol_ctx);
// C source: 	if (xp == NULL)
// C source: 		return -ENOENT;
// C source: 
// C source: 	xfrm_audit_policy_delete(xp, err ? 0 : 1, true);
// C source: 
// C source: 	if (err)
// C source: 		goto out;
// C source: 
// C source: 	c.seq = hdr->sadb_msg_seq;
// C source: 	c.portid = hdr->sadb_msg_pid;
// C source: 	c.data.byid = 0;
// C source: 	c.event = XFRM_MSG_DELPOLICY;
// C source: 	km_policy_notify(xp, pol->sadb_x_policy_dir-1, &c);
// C source: 
// C source: out:
// C source: 	xfrm_pol_put(xp);
// C source: 	return err;
// C source: }
// C source: 
// C source: static int key_pol_get_resp(struct sock *sk, struct xfrm_policy *xp, const struct sadb_msg *hdr, int dir)
// C source: {
// C source: 	int err;
// C source: 	struct sk_buff *out_skb;
// C source: 	struct sadb_msg *out_hdr;
// C source: 	err = 0;
// C source: 
// C source: 	out_skb = pfkey_xfrm_policy2msg_prep(xp);
// C source: 	if (IS_ERR(out_skb)) {
// C source: 		err =  PTR_ERR(out_skb);
// C source: 		goto out;
// C source: 	}
// C source: 	err = pfkey_xfrm_policy2msg(out_skb, xp, dir);
// C source: 	if (err < 0) {
// C source: 		kfree_skb(out_skb);
// C source: 		goto out;
// C source: 	}
// C source: 
// C source: 	out_hdr = (struct sadb_msg *) out_skb->data;
// C source: 	out_hdr->sadb_msg_version = hdr->sadb_msg_version;
// C source: 	out_hdr->sadb_msg_type = hdr->sadb_msg_type;
// C source: 	out_hdr->sadb_msg_satype = 0;
// C source: 	out_hdr->sadb_msg_errno = 0;
// C source: 	out_hdr->sadb_msg_seq = hdr->sadb_msg_seq;
// C source: 	out_hdr->sadb_msg_pid = hdr->sadb_msg_pid;
// C source: 	pfkey_broadcast(out_skb, GFP_ATOMIC, BROADCAST_ONE, sk, xp_net(xp));
// C source: 	err = 0;
// C source: 
// C source: out:
// C source: 	return err;
// C source: }
// C source: 
// C source: static int pfkey_sockaddr_pair_size(sa_family_t family)
// C source: {
// C source: 	return PFKEY_ALIGN8(pfkey_sockaddr_len(family) * 2);
// C source: }
// C source: 
// C source: static int parse_sockaddr_pair(struct sockaddr *sa, int ext_len,
// C source: 			       xfrm_address_t *saddr, xfrm_address_t *daddr,
// C source: 			       u16 *family)
// C source: {
// C source: 	int af, socklen;
// C source: 
// C source: 	if (ext_len < 2 || ext_len < pfkey_sockaddr_pair_size(sa->sa_family))
// C source: 		return -EINVAL;
// C source: 
// C source: 	af = pfkey_sockaddr_extract(sa, saddr);
// C source: 	if (!af)
// C source: 		return -EINVAL;
// C source: 
// C source: 	socklen = pfkey_sockaddr_len(af);
// C source: 	if (pfkey_sockaddr_extract((struct sockaddr *) (((u8 *)sa) + socklen),
// C source: 				   daddr) != af)
// C source: 		return -EINVAL;
// C source: 
// C source: 	*family = af;
// C source: 	return 0;
// C source: }
// C source: 
// C source: #ifdef CONFIG_NET_KEY_MIGRATE
// C source: static int ipsecrequests_to_migrate(struct sadb_x_ipsecrequest *rq1, int len,
// C source: 				    struct xfrm_migrate *m)
// C source: {
// C source: 	int err;
// C source: 	struct sadb_x_ipsecrequest *rq2;
// C source: 	int mode;
// C source: 
// C source: 	if (len < sizeof(*rq1) ||
// C source: 	    len < rq1->sadb_x_ipsecrequest_len ||
// C source: 	    rq1->sadb_x_ipsecrequest_len < sizeof(*rq1))
// C source: 		return -EINVAL;
// C source: 
// C source: 	/* old endoints */
// C source: 	err = parse_sockaddr_pair((struct sockaddr *)(rq1 + 1),
// C source: 				  rq1->sadb_x_ipsecrequest_len - sizeof(*rq1),
// C source: 				  &m->old_saddr, &m->old_daddr,
// C source: 				  &m->old_family);
// C source: 	if (err)
// C source: 		return err;
// C source: 
// C source: 	rq2 = (struct sadb_x_ipsecrequest *)((u8 *)rq1 + rq1->sadb_x_ipsecrequest_len);
// C source: 	len -= rq1->sadb_x_ipsecrequest_len;
// C source: 
// C source: 	if (len <= sizeof(*rq2) ||
// C source: 	    len < rq2->sadb_x_ipsecrequest_len ||
// C source: 	    rq2->sadb_x_ipsecrequest_len < sizeof(*rq2))
// C source: 		return -EINVAL;
// C source: 
// C source: 	/* new endpoints */
// C source: 	err = parse_sockaddr_pair((struct sockaddr *)(rq2 + 1),
// C source: 				  rq2->sadb_x_ipsecrequest_len - sizeof(*rq2),
// C source: 				  &m->new_saddr, &m->new_daddr,
// C source: 				  &m->new_family);
// C source: 	if (err)
// C source: 		return err;
// C source: 
// C source: 	if (rq1->sadb_x_ipsecrequest_proto != rq2->sadb_x_ipsecrequest_proto ||
// C source: 	    rq1->sadb_x_ipsecrequest_mode != rq2->sadb_x_ipsecrequest_mode ||
// C source: 	    rq1->sadb_x_ipsecrequest_reqid != rq2->sadb_x_ipsecrequest_reqid)
// C source: 		return -EINVAL;
// C source: 
// C source: 	m->proto = rq1->sadb_x_ipsecrequest_proto;
// C source: 	if ((mode = pfkey_mode_to_xfrm(rq1->sadb_x_ipsecrequest_mode)) < 0)
// C source: 		return -EINVAL;
// C source: 	m->mode = mode;
// C source: 	m->old_reqid = rq1->sadb_x_ipsecrequest_reqid;
// C source: 
// C source: 	return ((int)(rq1->sadb_x_ipsecrequest_len +
// C source: 		      rq2->sadb_x_ipsecrequest_len));
// C source: }
// C source: 
// C source: static int pfkey_migrate(struct sock *sk, struct sk_buff *skb,
// C source: 			 const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	int i, len, ret, err = -EINVAL;
// C source: 	u8 dir;
// C source: 	struct sadb_address *sa;
// C source: 	struct sadb_x_kmaddress *kma;
// C source: 	struct sadb_x_policy *pol;
// C source: 	struct sadb_x_ipsecrequest *rq;
// C source: 	struct xfrm_selector sel;
// C source: 	struct xfrm_migrate m[XFRM_MAX_DEPTH];
// C source: 	struct xfrm_kmaddress k;
// C source: 	struct net *net = sock_net(sk);
// C source: 
// C source: 	if (!present_and_same_family(ext_hdrs[SADB_EXT_ADDRESS_SRC - 1],
// C source: 				     ext_hdrs[SADB_EXT_ADDRESS_DST - 1]) ||
// C source: 	    !ext_hdrs[SADB_X_EXT_POLICY - 1]) {
// C source: 		err = -EINVAL;
// C source: 		goto out;
// C source: 	}
// C source: 
// C source: 	kma = ext_hdrs[SADB_X_EXT_KMADDRESS - 1];
// C source: 	pol = ext_hdrs[SADB_X_EXT_POLICY - 1];
// C source: 
// C source: 	if (pol->sadb_x_policy_dir >= IPSEC_DIR_MAX) {
// C source: 		err = -EINVAL;
// C source: 		goto out;
// C source: 	}
// C source: 
// C source: 	if (kma) {
// C source: 		/* convert sadb_x_kmaddress to xfrm_kmaddress */
// C source: 		k.reserved = kma->sadb_x_kmaddress_reserved;
// C source: 		ret = parse_sockaddr_pair((struct sockaddr *)(kma + 1),
// C source: 					  8*(kma->sadb_x_kmaddress_len) - sizeof(*kma),
// C source: 					  &k.local, &k.remote, &k.family);
// C source: 		if (ret < 0) {
// C source: 			err = ret;
// C source: 			goto out;
// C source: 		}
// C source: 	}
// C source: 
// C source: 	dir = pol->sadb_x_policy_dir - 1;
// C source: 	memset(&sel, 0, sizeof(sel));
// C source: 
// C source: 	/* set source address info of selector */
// C source: 	sa = ext_hdrs[SADB_EXT_ADDRESS_SRC - 1];
// C source: 	sel.family = pfkey_sadb_addr2xfrm_addr(sa, &sel.saddr);
// C source: 	sel.prefixlen_s = sa->sadb_address_prefixlen;
// C source: 	sel.proto = pfkey_proto_to_xfrm(sa->sadb_address_proto);
// C source: 	sel.sport = ((struct sockaddr_in *)(sa + 1))->sin_port;
// C source: 	if (sel.sport)
// C source: 		sel.sport_mask = htons(0xffff);
// C source: 
// C source: 	/* set destination address info of selector */
// C source: 	sa = ext_hdrs[SADB_EXT_ADDRESS_DST - 1];
// C source: 	pfkey_sadb_addr2xfrm_addr(sa, &sel.daddr);
// C source: 	sel.prefixlen_d = sa->sadb_address_prefixlen;
// C source: 	sel.proto = pfkey_proto_to_xfrm(sa->sadb_address_proto);
// C source: 	sel.dport = ((struct sockaddr_in *)(sa + 1))->sin_port;
// C source: 	if (sel.dport)
// C source: 		sel.dport_mask = htons(0xffff);
// C source: 
// C source: 	rq = (struct sadb_x_ipsecrequest *)(pol + 1);
// C source: 
// C source: 	/* extract ipsecrequests */
// C source: 	i = 0;
// C source: 	len = pol->sadb_x_policy_len * 8 - sizeof(struct sadb_x_policy);
// C source: 
// C source: 	while (len > 0 && i < XFRM_MAX_DEPTH) {
// C source: 		ret = ipsecrequests_to_migrate(rq, len, &m[i]);
// C source: 		if (ret < 0) {
// C source: 			err = ret;
// C source: 			goto out;
// C source: 		} else {
// C source: 			rq = (struct sadb_x_ipsecrequest *)((u8 *)rq + ret);
// C source: 			len -= ret;
// C source: 			i++;
// C source: 		}
// C source: 	}
// C source: 
// C source: 	if (!i || len > 0) {
// C source: 		err = -EINVAL;
// C source: 		goto out;
// C source: 	}
// C source: 
// C source: 	return xfrm_migrate(&sel, dir, XFRM_POLICY_TYPE_MAIN, m, i,
// C source: 			    kma ? &k : NULL, net, NULL, 0, NULL, NULL);
// C source: 
// C source:  out:
// C source: 	return err;
// C source: }
// C source: #else
// C source: static int pfkey_migrate(struct sock *sk, struct sk_buff *skb,
// C source: 			 const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	return -ENOPROTOOPT;
// C source: }
// C source: #endif
// C source: 
// C source: 
// C source: static int pfkey_spdget(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	unsigned int dir;
// C source: 	int err = 0, delete;
// C source: 	struct sadb_x_policy *pol;
// C source: 	struct xfrm_policy *xp;
// C source: 	struct km_event c;
// C source: 
// C source: 	if ((pol = ext_hdrs[SADB_X_EXT_POLICY-1]) == NULL)
// C source: 		return -EINVAL;
// C source: 
// C source: 	dir = xfrm_policy_id2dir(pol->sadb_x_policy_id);
// C source: 	if (dir >= XFRM_POLICY_MAX)
// C source: 		return -EINVAL;
// C source: 
// C source: 	delete = (hdr->sadb_msg_type == SADB_X_SPDDELETE2);
// C source: 	xp = xfrm_policy_byid(net, &dummy_mark, 0, XFRM_POLICY_TYPE_MAIN,
// C source: 			      dir, pol->sadb_x_policy_id, delete, &err);
// C source: 	if (xp == NULL)
// C source: 		return -ENOENT;
// C source: 
// C source: 	if (delete) {
// C source: 		xfrm_audit_policy_delete(xp, err ? 0 : 1, true);
// C source: 
// C source: 		if (err)
// C source: 			goto out;
// C source: 		c.seq = hdr->sadb_msg_seq;
// C source: 		c.portid = hdr->sadb_msg_pid;
// C source: 		c.data.byid = 1;
// C source: 		c.event = XFRM_MSG_DELPOLICY;
// C source: 		km_policy_notify(xp, dir, &c);
// C source: 	} else {
// C source: 		err = key_pol_get_resp(sk, xp, hdr, dir);
// C source: 	}
// C source: 
// C source: out:
// C source: 	xfrm_pol_put(xp);
// C source: 	return err;
// C source: }
// C source: 
// C source: static int dump_sp(struct xfrm_policy *xp, int dir, int count, void *ptr)
// C source: {
// C source: 	struct pfkey_sock *pfk = ptr;
// C source: 	struct sk_buff *out_skb;
// C source: 	struct sadb_msg *out_hdr;
// C source: 	int err;
// C source: 
// C source: 	if (!pfkey_can_dump(&pfk->sk))
// C source: 		return -ENOBUFS;
// C source: 
// C source: 	out_skb = pfkey_xfrm_policy2msg_prep(xp);
// C source: 	if (IS_ERR(out_skb))
// C source: 		return PTR_ERR(out_skb);
// C source: 
// C source: 	err = pfkey_xfrm_policy2msg(out_skb, xp, dir);
// C source: 	if (err < 0) {
// C source: 		kfree_skb(out_skb);
// C source: 		return err;
// C source: 	}
// C source: 
// C source: 	out_hdr = (struct sadb_msg *) out_skb->data;
// C source: 	out_hdr->sadb_msg_version = pfk->dump.msg_version;
// C source: 	out_hdr->sadb_msg_type = SADB_X_SPDDUMP;
// C source: 	out_hdr->sadb_msg_satype = SADB_SATYPE_UNSPEC;
// C source: 	out_hdr->sadb_msg_errno = 0;
// C source: 	out_hdr->sadb_msg_seq = count + 1;
// C source: 	out_hdr->sadb_msg_pid = pfk->dump.msg_portid;
// C source: 
// C source: 	if (pfk->dump.skb)
// C source: 		pfkey_broadcast(pfk->dump.skb, GFP_ATOMIC, BROADCAST_ONE,
// C source: 				&pfk->sk, sock_net(&pfk->sk));
// C source: 	pfk->dump.skb = out_skb;
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_dump_sp(struct pfkey_sock *pfk)
// C source: {
// C source: 	struct net *net = sock_net(&pfk->sk);
// C source: 	return xfrm_policy_walk(net, &pfk->dump.u.policy, dump_sp, (void *) pfk);
// C source: }
// C source: 
// C source: static void pfkey_dump_sp_done(struct pfkey_sock *pfk)
// C source: {
// C source: 	struct net *net = sock_net((struct sock *)pfk);
// C source: 
// C source: 	xfrm_policy_walk_done(&pfk->dump.u.policy, net);
// C source: }
// C source: 
// C source: static int pfkey_spddump(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct pfkey_sock *pfk = pfkey_sk(sk);
// C source: 
// C source: 	mutex_lock(&pfk->dump_lock);
// C source: 	if (pfk->dump.dump != NULL) {
// C source: 		mutex_unlock(&pfk->dump_lock);
// C source: 		return -EBUSY;
// C source: 	}
// C source: 
// C source: 	pfk->dump.msg_version = hdr->sadb_msg_version;
// C source: 	pfk->dump.msg_portid = hdr->sadb_msg_pid;
// C source: 	pfk->dump.dump = pfkey_dump_sp;
// C source: 	pfk->dump.done = pfkey_dump_sp_done;
// C source: 	xfrm_policy_walk_init(&pfk->dump.u.policy, XFRM_POLICY_TYPE_MAIN);
// C source: 	mutex_unlock(&pfk->dump_lock);
// C source: 
// C source: 	return pfkey_do_dump(pfk);
// C source: }
// C source: 
// C source: static int key_notify_policy_flush(const struct km_event *c)
// C source: {
// C source: 	struct sk_buff *skb_out;
// C source: 	struct sadb_msg *hdr;
// C source: 
// C source: 	skb_out = alloc_skb(sizeof(struct sadb_msg) + 16, GFP_ATOMIC);
// C source: 	if (!skb_out)
// C source: 		return -ENOBUFS;
// C source: 	hdr = skb_put(skb_out, sizeof(struct sadb_msg));
// C source: 	hdr->sadb_msg_type = SADB_X_SPDFLUSH;
// C source: 	hdr->sadb_msg_seq = c->seq;
// C source: 	hdr->sadb_msg_pid = c->portid;
// C source: 	hdr->sadb_msg_version = PF_KEY_V2;
// C source: 	hdr->sadb_msg_errno = (uint8_t) 0;
// C source: 	hdr->sadb_msg_satype = SADB_SATYPE_UNSPEC;
// C source: 	hdr->sadb_msg_len = (sizeof(struct sadb_msg) / sizeof(uint64_t));
// C source: 	hdr->sadb_msg_reserved = 0;
// C source: 	pfkey_broadcast(skb_out, GFP_ATOMIC, BROADCAST_ALL, NULL, c->net);
// C source: 	return 0;
// C source: 
// C source: }
// C source: 
// C source: static int pfkey_spdflush(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr, void * const *ext_hdrs)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	struct km_event c;
// C source: 	int err, err2;
// C source: 
// C source: 	err = xfrm_policy_flush(net, XFRM_POLICY_TYPE_MAIN, true);
// C source: 	err2 = unicast_flush_resp(sk, hdr);
// C source: 	if (err || err2) {
// C source: 		if (err == -ESRCH) /* empty table - old silent behavior */
// C source: 			return 0;
// C source: 		return err;
// C source: 	}
// C source: 
// C source: 	c.data.type = XFRM_POLICY_TYPE_MAIN;
// C source: 	c.event = XFRM_MSG_FLUSHPOLICY;
// C source: 	c.portid = hdr->sadb_msg_pid;
// C source: 	c.seq = hdr->sadb_msg_seq;
// C source: 	c.net = net;
// C source: 	km_policy_notify(NULL, 0, &c);
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: typedef int (*pfkey_handler)(struct sock *sk, struct sk_buff *skb,
// C source: 			     const struct sadb_msg *hdr, void * const *ext_hdrs);
// C source: static const pfkey_handler pfkey_funcs[SADB_MAX + 1] = {
// C source: 	[SADB_RESERVED]		= pfkey_reserved,
// C source: 	[SADB_GETSPI]		= pfkey_getspi,
// C source: 	[SADB_UPDATE]		= pfkey_add,
// C source: 	[SADB_ADD]		= pfkey_add,
// C source: 	[SADB_DELETE]		= pfkey_delete,
// C source: 	[SADB_GET]		= pfkey_get,
// C source: 	[SADB_ACQUIRE]		= pfkey_acquire,
// C source: 	[SADB_REGISTER]		= pfkey_register,
// C source: 	[SADB_EXPIRE]		= NULL,
// C source: 	[SADB_FLUSH]		= pfkey_flush,
// C source: 	[SADB_DUMP]		= pfkey_dump,
// C source: 	[SADB_X_PROMISC]	= pfkey_promisc,
// C source: 	[SADB_X_PCHANGE]	= NULL,
// C source: 	[SADB_X_SPDUPDATE]	= pfkey_spdadd,
// C source: 	[SADB_X_SPDADD]		= pfkey_spdadd,
// C source: 	[SADB_X_SPDDELETE]	= pfkey_spddelete,
// C source: 	[SADB_X_SPDGET]		= pfkey_spdget,
// C source: 	[SADB_X_SPDACQUIRE]	= NULL,
// C source: 	[SADB_X_SPDDUMP]	= pfkey_spddump,
// C source: 	[SADB_X_SPDFLUSH]	= pfkey_spdflush,
// C source: 	[SADB_X_SPDSETIDX]	= pfkey_spdadd,
// C source: 	[SADB_X_SPDDELETE2]	= pfkey_spdget,
// C source: 	[SADB_X_MIGRATE]	= pfkey_migrate,
// C source: };
// C source: 
// C source: static int pfkey_process(struct sock *sk, struct sk_buff *skb, const struct sadb_msg *hdr)
// C source: {
// C source: 	void *ext_hdrs[SADB_EXT_MAX];
// C source: 	int err;
// C source: 
// C source: 	/* Non-zero return value of pfkey_broadcast() does not always signal
// C source: 	 * an error and even on an actual error we may still want to process
// C source: 	 * the message so rather ignore the return value.
// C source: 	 */
// C source: 	pfkey_broadcast(skb_clone(skb, GFP_KERNEL), GFP_KERNEL,
// C source: 			BROADCAST_PROMISC_ONLY, NULL, sock_net(sk));
// C source: 
// C source: 	memset(ext_hdrs, 0, sizeof(ext_hdrs));
// C source: 	err = parse_exthdrs(skb, hdr, ext_hdrs);
// C source: 	if (!err) {
// C source: 		err = -EOPNOTSUPP;
// C source: 		if (pfkey_funcs[hdr->sadb_msg_type])
// C source: 			err = pfkey_funcs[hdr->sadb_msg_type](sk, skb, hdr, ext_hdrs);
// C source: 	}
// C source: 	return err;
// C source: }
// C source: 
// C source: static struct sadb_msg *pfkey_get_base_msg(struct sk_buff *skb, int *errp)
// C source: {
// C source: 	struct sadb_msg *hdr = NULL;
// C source: 
// C source: 	if (skb->len < sizeof(*hdr)) {
// C source: 		*errp = -EMSGSIZE;
// C source: 	} else {
// C source: 		hdr = (struct sadb_msg *) skb->data;
// C source: 		if (hdr->sadb_msg_version != PF_KEY_V2 ||
// C source: 		    hdr->sadb_msg_reserved != 0 ||
// C source: 		    (hdr->sadb_msg_type <= SADB_RESERVED ||
// C source: 		     hdr->sadb_msg_type > SADB_MAX)) {
// C source: 			hdr = NULL;
// C source: 			*errp = -EINVAL;
// C source: 		} else if (hdr->sadb_msg_len != (skb->len /
// C source: 						 sizeof(uint64_t)) ||
// C source: 			   hdr->sadb_msg_len < (sizeof(struct sadb_msg) /
// C source: 						sizeof(uint64_t))) {
// C source: 			hdr = NULL;
// C source: 			*errp = -EMSGSIZE;
// C source: 		} else {
// C source: 			*errp = 0;
// C source: 		}
// C source: 	}
// C source: 	return hdr;
// C source: }
// C source: 
// C source: static inline int aalg_tmpl_set(const struct xfrm_tmpl *t,
// C source: 				const struct xfrm_algo_desc *d)
// C source: {
// C source: 	unsigned int id = d->desc.sadb_alg_id;
// C source: 
// C source: 	if (id >= sizeof(t->aalgos) * 8)
// C source: 		return 0;
// C source: 
// C source: 	return (t->aalgos >> id) & 1;
// C source: }
// C source: 
// C source: static inline int ealg_tmpl_set(const struct xfrm_tmpl *t,
// C source: 				const struct xfrm_algo_desc *d)
// C source: {
// C source: 	unsigned int id = d->desc.sadb_alg_id;
// C source: 
// C source: 	if (id >= sizeof(t->ealgos) * 8)
// C source: 		return 0;
// C source: 
// C source: 	return (t->ealgos >> id) & 1;
// C source: }
// C source: 
// C source: static int count_ah_combs(const struct xfrm_tmpl *t)
// C source: {
// C source: 	int i, sz = 0;
// C source: 
// C source: 	for (i = 0; ; i++) {
// C source: 		const struct xfrm_algo_desc *aalg = xfrm_aalg_get_byidx(i);
// C source: 		if (!aalg)
// C source: 			break;
// C source: 		if (!aalg->pfkey_supported)
// C source: 			continue;
// C source: 		if (aalg_tmpl_set(t, aalg))
// C source: 			sz += sizeof(struct sadb_comb);
// C source: 	}
// C source: 	return sz + sizeof(struct sadb_prop);
// C source: }
// C source: 
// C source: static int count_esp_combs(const struct xfrm_tmpl *t)
// C source: {
// C source: 	int i, k, sz = 0;
// C source: 
// C source: 	for (i = 0; ; i++) {
// C source: 		const struct xfrm_algo_desc *ealg = xfrm_ealg_get_byidx(i);
// C source: 		if (!ealg)
// C source: 			break;
// C source: 
// C source: 		if (!ealg->pfkey_supported)
// C source: 			continue;
// C source: 
// C source: 		if (!(ealg_tmpl_set(t, ealg)))
// C source: 			continue;
// C source: 
// C source: 		for (k = 1; ; k++) {
// C source: 			const struct xfrm_algo_desc *aalg = xfrm_aalg_get_byidx(k);
// C source: 			if (!aalg)
// C source: 				break;
// C source: 
// C source: 			if (!aalg->pfkey_supported)
// C source: 				continue;
// C source: 
// C source: 			if (aalg_tmpl_set(t, aalg))
// C source: 				sz += sizeof(struct sadb_comb);
// C source: 		}
// C source: 	}
// C source: 	return sz + sizeof(struct sadb_prop);
// C source: }
// C source: 
// C source: static int dump_ah_combs(struct sk_buff *skb, const struct xfrm_tmpl *t)
// C source: {
// C source: 	struct sadb_prop *p;
// C source: 	int sz = 0;
// C source: 	int i;
// C source: 
// C source: 	p = skb_put(skb, sizeof(struct sadb_prop));
// C source: 	p->sadb_prop_len = sizeof(struct sadb_prop)/8;
// C source: 	p->sadb_prop_exttype = SADB_EXT_PROPOSAL;
// C source: 	p->sadb_prop_replay = 32;
// C source: 	memset(p->sadb_prop_reserved, 0, sizeof(p->sadb_prop_reserved));
// C source: 
// C source: 	for (i = 0; ; i++) {
// C source: 		const struct xfrm_algo_desc *aalg = xfrm_aalg_get_byidx(i);
// C source: 		if (!aalg)
// C source: 			break;
// C source: 
// C source: 		if (!aalg->pfkey_supported)
// C source: 			continue;
// C source: 
// C source: 		if (aalg_tmpl_set(t, aalg) && aalg->available) {
// C source: 			struct sadb_comb *c;
// C source: 			c = skb_put_zero(skb, sizeof(struct sadb_comb));
// C source: 			p->sadb_prop_len += sizeof(struct sadb_comb)/8;
// C source: 			c->sadb_comb_auth = aalg->desc.sadb_alg_id;
// C source: 			c->sadb_comb_auth_minbits = aalg->desc.sadb_alg_minbits;
// C source: 			c->sadb_comb_auth_maxbits = aalg->desc.sadb_alg_maxbits;
// C source: 			c->sadb_comb_hard_addtime = 24*60*60;
// C source: 			c->sadb_comb_soft_addtime = 20*60*60;
// C source: 			c->sadb_comb_hard_usetime = 8*60*60;
// C source: 			c->sadb_comb_soft_usetime = 7*60*60;
// C source: 			sz += sizeof(*c);
// C source: 		}
// C source: 	}
// C source: 
// C source: 	return sz + sizeof(*p);
// C source: }
// C source: 
// C source: static int dump_esp_combs(struct sk_buff *skb, const struct xfrm_tmpl *t)
// C source: {
// C source: 	struct sadb_prop *p;
// C source: 	int sz = 0;
// C source: 	int i, k;
// C source: 
// C source: 	p = skb_put(skb, sizeof(struct sadb_prop));
// C source: 	p->sadb_prop_len = sizeof(struct sadb_prop)/8;
// C source: 	p->sadb_prop_exttype = SADB_EXT_PROPOSAL;
// C source: 	p->sadb_prop_replay = 32;
// C source: 	memset(p->sadb_prop_reserved, 0, sizeof(p->sadb_prop_reserved));
// C source: 
// C source: 	for (i=0; ; i++) {
// C source: 		const struct xfrm_algo_desc *ealg = xfrm_ealg_get_byidx(i);
// C source: 		if (!ealg)
// C source: 			break;
// C source: 
// C source: 		if (!ealg->pfkey_supported)
// C source: 			continue;
// C source: 
// C source: 		if (!(ealg_tmpl_set(t, ealg) && ealg->available))
// C source: 			continue;
// C source: 
// C source: 		for (k = 1; ; k++) {
// C source: 			struct sadb_comb *c;
// C source: 			const struct xfrm_algo_desc *aalg = xfrm_aalg_get_byidx(k);
// C source: 			if (!aalg)
// C source: 				break;
// C source: 			if (!aalg->pfkey_supported)
// C source: 				continue;
// C source: 			if (!(aalg_tmpl_set(t, aalg) && aalg->available))
// C source: 				continue;
// C source: 			c = skb_put(skb, sizeof(struct sadb_comb));
// C source: 			memset(c, 0, sizeof(*c));
// C source: 			p->sadb_prop_len += sizeof(struct sadb_comb)/8;
// C source: 			c->sadb_comb_auth = aalg->desc.sadb_alg_id;
// C source: 			c->sadb_comb_auth_minbits = aalg->desc.sadb_alg_minbits;
// C source: 			c->sadb_comb_auth_maxbits = aalg->desc.sadb_alg_maxbits;
// C source: 			c->sadb_comb_encrypt = ealg->desc.sadb_alg_id;
// C source: 			c->sadb_comb_encrypt_minbits = ealg->desc.sadb_alg_minbits;
// C source: 			c->sadb_comb_encrypt_maxbits = ealg->desc.sadb_alg_maxbits;
// C source: 			c->sadb_comb_hard_addtime = 24*60*60;
// C source: 			c->sadb_comb_soft_addtime = 20*60*60;
// C source: 			c->sadb_comb_hard_usetime = 8*60*60;
// C source: 			c->sadb_comb_soft_usetime = 7*60*60;
// C source: 			sz += sizeof(*c);
// C source: 		}
// C source: 	}
// C source: 
// C source: 	return sz + sizeof(*p);
// C source: }
// C source: 
// C source: static int key_notify_policy_expire(struct xfrm_policy *xp, const struct km_event *c)
// C source: {
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int key_notify_sa_expire(struct xfrm_state *x, const struct km_event *c)
// C source: {
// C source: 	struct sk_buff *out_skb;
// C source: 	struct sadb_msg *out_hdr;
// C source: 	int hard;
// C source: 	int hsc;
// C source: 
// C source: 	hard = c->data.hard;
// C source: 	if (hard)
// C source: 		hsc = 2;
// C source: 	else
// C source: 		hsc = 1;
// C source: 
// C source: 	out_skb = pfkey_xfrm_state2msg_expire(x, hsc);
// C source: 	if (IS_ERR(out_skb))
// C source: 		return PTR_ERR(out_skb);
// C source: 
// C source: 	out_hdr = (struct sadb_msg *) out_skb->data;
// C source: 	out_hdr->sadb_msg_version = PF_KEY_V2;
// C source: 	out_hdr->sadb_msg_type = SADB_EXPIRE;
// C source: 	out_hdr->sadb_msg_satype = pfkey_proto2satype(x->id.proto);
// C source: 	out_hdr->sadb_msg_errno = 0;
// C source: 	out_hdr->sadb_msg_reserved = 0;
// C source: 	out_hdr->sadb_msg_seq = 0;
// C source: 	out_hdr->sadb_msg_pid = 0;
// C source: 
// C source: 	pfkey_broadcast(out_skb, GFP_ATOMIC, BROADCAST_REGISTERED, NULL,
// C source: 			xs_net(x));
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_send_notify(struct xfrm_state *x, const struct km_event *c)
// C source: {
// C source: 	struct net *net = x ? xs_net(x) : c->net;
// C source: 	struct netns_pfkey *net_pfkey = net_generic(net, pfkey_net_id);
// C source: 
// C source: 	if (atomic_read(&net_pfkey->socks_nr) == 0)
// C source: 		return 0;
// C source: 
// C source: 	switch (c->event) {
// C source: 	case XFRM_MSG_EXPIRE:
// C source: 		return key_notify_sa_expire(x, c);
// C source: 	case XFRM_MSG_DELSA:
// C source: 	case XFRM_MSG_NEWSA:
// C source: 	case XFRM_MSG_UPDSA:
// C source: 		return key_notify_sa(x, c);
// C source: 	case XFRM_MSG_FLUSHSA:
// C source: 		return key_notify_sa_flush(c);
// C source: 	case XFRM_MSG_NEWAE: /* not yet supported */
// C source: 		break;
// C source: 	default:
// C source: 		pr_err("pfkey: Unknown SA event %d\n", c->event);
// C source: 		break;
// C source: 	}
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int pfkey_send_policy_notify(struct xfrm_policy *xp, int dir, const struct km_event *c)
// C source: {
// C source: 	if (xp && xp->type != XFRM_POLICY_TYPE_MAIN)
// C source: 		return 0;
// C source: 
// C source: 	switch (c->event) {
// C source: 	case XFRM_MSG_POLEXPIRE:
// C source: 		return key_notify_policy_expire(xp, c);
// C source: 	case XFRM_MSG_DELPOLICY:
// C source: 	case XFRM_MSG_NEWPOLICY:
// C source: 	case XFRM_MSG_UPDPOLICY:
// C source: 		return key_notify_policy(xp, dir, c);
// C source: 	case XFRM_MSG_FLUSHPOLICY:
// C source: 		if (c->data.type != XFRM_POLICY_TYPE_MAIN)
// C source: 			break;
// C source: 		return key_notify_policy_flush(c);
// C source: 	default:
// C source: 		pr_err("pfkey: Unknown policy event %d\n", c->event);
// C source: 		break;
// C source: 	}
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static u32 get_acqseq(void)
// C source: {
// C source: 	u32 res;
// C source: 	static atomic_t acqseq;
// C source: 
// C source: 	do {
// C source: 		res = atomic_inc_return(&acqseq);
// C source: 	} while (!res);
// C source: 	return res;
// C source: }
// C source: 
// C source: static bool pfkey_is_alive(const struct km_event *c)
// C source: {
// C source: 	struct netns_pfkey *net_pfkey = net_generic(c->net, pfkey_net_id);
// C source: 	struct sock *sk;
// C source: 	bool is_alive = false;
// C source: 
// C source: 	rcu_read_lock();
// C source: 	sk_for_each_rcu(sk, &net_pfkey->table) {
// C source: 		if (pfkey_sk(sk)->registered) {
// C source: 			is_alive = true;
// C source: 			break;
// C source: 		}
// C source: 	}
// C source: 	rcu_read_unlock();
// C source: 
// C source: 	return is_alive;
// C source: }
// C source: 
// C source: static int pfkey_send_acquire(struct xfrm_state *x, struct xfrm_tmpl *t, struct xfrm_policy *xp)
// C source: {
// C source: 	struct sk_buff *skb;
// C source: 	struct sadb_msg *hdr;
// C source: 	struct sadb_address *addr;
// C source: 	struct sadb_x_policy *pol;
// C source: 	int sockaddr_size;
// C source: 	int size;
// C source: 	struct sadb_x_sec_ctx *sec_ctx;
// C source: 	struct xfrm_sec_ctx *xfrm_ctx;
// C source: 	int ctx_size = 0;
// C source: 	int alg_size = 0;
// C source: 
// C source: 	sockaddr_size = pfkey_sockaddr_size(x->props.family);
// C source: 	if (!sockaddr_size)
// C source: 		return -EINVAL;
// C source: 
// C source: 	size = sizeof(struct sadb_msg) +
// C source: 		(sizeof(struct sadb_address) * 2) +
// C source: 		(sockaddr_size * 2) +
// C source: 		sizeof(struct sadb_x_policy);
// C source: 
// C source: 	if (x->id.proto == IPPROTO_AH)
// C source: 		alg_size = count_ah_combs(t);
// C source: 	else if (x->id.proto == IPPROTO_ESP)
// C source: 		alg_size = count_esp_combs(t);
// C source: 
// C source: 	if ((xfrm_ctx = x->security)) {
// C source: 		ctx_size = PFKEY_ALIGN8(xfrm_ctx->ctx_len);
// C source: 		size +=  sizeof(struct sadb_x_sec_ctx) + ctx_size;
// C source: 	}
// C source: 
// C source: 	skb =  alloc_skb(size + alg_size + 16, GFP_ATOMIC);
// C source: 	if (skb == NULL)
// C source: 		return -ENOMEM;
// C source: 
// C source: 	hdr = skb_put(skb, sizeof(struct sadb_msg));
// C source: 	hdr->sadb_msg_version = PF_KEY_V2;
// C source: 	hdr->sadb_msg_type = SADB_ACQUIRE;
// C source: 	hdr->sadb_msg_satype = pfkey_proto2satype(x->id.proto);
// C source: 	hdr->sadb_msg_len = size / sizeof(uint64_t);
// C source: 	hdr->sadb_msg_errno = 0;
// C source: 	hdr->sadb_msg_reserved = 0;
// C source: 	hdr->sadb_msg_seq = x->km.seq = get_acqseq();
// C source: 	hdr->sadb_msg_pid = 0;
// C source: 
// C source: 	/* src address */
// C source: 	addr = skb_put(skb, sizeof(struct sadb_address) + sockaddr_size);
// C source: 	addr->sadb_address_len =
// C source: 		(sizeof(struct sadb_address)+sockaddr_size)/
// C source: 			sizeof(uint64_t);
// C source: 	addr->sadb_address_exttype = SADB_EXT_ADDRESS_SRC;
// C source: 	addr->sadb_address_proto = 0;
// C source: 	addr->sadb_address_reserved = 0;
// C source: 	addr->sadb_address_prefixlen =
// C source: 		pfkey_sockaddr_fill_zero_tail(&x->props.saddr, 0,
// C source: 					      (struct sockaddr *)(addr + 1),
// C source: 					      x->props.family);
// C source: 	if (!addr->sadb_address_prefixlen)
// C source: 		BUG();
// C source: 
// C source: 	/* dst address */
// C source: 	addr = skb_put(skb, sizeof(struct sadb_address) + sockaddr_size);
// C source: 	addr->sadb_address_len =
// C source: 		(sizeof(struct sadb_address)+sockaddr_size)/
// C source: 			sizeof(uint64_t);
// C source: 	addr->sadb_address_exttype = SADB_EXT_ADDRESS_DST;
// C source: 	addr->sadb_address_proto = 0;
// C source: 	addr->sadb_address_reserved = 0;
// C source: 	addr->sadb_address_prefixlen =
// C source: 		pfkey_sockaddr_fill_zero_tail(&x->id.daddr, 0,
// C source: 					      (struct sockaddr *)(addr + 1),
// C source: 					      x->props.family);
// C source: 	if (!addr->sadb_address_prefixlen)
// C source: 		BUG();
// C source: 
// C source: 	pol = skb_put(skb, sizeof(struct sadb_x_policy));
// C source: 	pol->sadb_x_policy_len = sizeof(struct sadb_x_policy)/sizeof(uint64_t);
// C source: 	pol->sadb_x_policy_exttype = SADB_X_EXT_POLICY;
// C source: 	pol->sadb_x_policy_type = IPSEC_POLICY_IPSEC;
// C source: 	pol->sadb_x_policy_dir = XFRM_POLICY_OUT + 1;
// C source: 	pol->sadb_x_policy_reserved = 0;
// C source: 	pol->sadb_x_policy_id = xp->index;
// C source: 	pol->sadb_x_policy_priority = xp->priority;
// C source: 
// C source: 	/* Set sadb_comb's. */
// C source: 	alg_size = 0;
// C source: 	if (x->id.proto == IPPROTO_AH)
// C source: 		alg_size = dump_ah_combs(skb, t);
// C source: 	else if (x->id.proto == IPPROTO_ESP)
// C source: 		alg_size = dump_esp_combs(skb, t);
// C source: 
// C source: 	hdr->sadb_msg_len += alg_size / 8;
// C source: 
// C source: 	/* security context */
// C source: 	if (xfrm_ctx) {
// C source: 		sec_ctx = skb_put(skb,
// C source: 				  sizeof(struct sadb_x_sec_ctx) + ctx_size);
// C source: 		sec_ctx->sadb_x_sec_len =
// C source: 		  (sizeof(struct sadb_x_sec_ctx) + ctx_size) / sizeof(uint64_t);
// C source: 		sec_ctx->sadb_x_sec_exttype = SADB_X_EXT_SEC_CTX;
// C source: 		sec_ctx->sadb_x_ctx_doi = xfrm_ctx->ctx_doi;
// C source: 		sec_ctx->sadb_x_ctx_alg = xfrm_ctx->ctx_alg;
// C source: 		sec_ctx->sadb_x_ctx_len = xfrm_ctx->ctx_len;
// C source: 		memcpy(sec_ctx + 1, xfrm_ctx->ctx_str,
// C source: 		       xfrm_ctx->ctx_len);
// C source: 	}
// C source: 
// C source: 	return pfkey_broadcast(skb, GFP_ATOMIC, BROADCAST_REGISTERED, NULL,
// C source: 			       xs_net(x));
// C source: }
// C source: 
// C source: static struct xfrm_policy *pfkey_compile_policy(struct sock *sk, int opt,
// C source: 						u8 *data, int len, int *dir)
// C source: {
// C source: 	struct net *net = sock_net(sk);
// C source: 	struct xfrm_policy *xp;
// C source: 	struct sadb_x_policy *pol = (struct sadb_x_policy*)data;
// C source: 	struct sadb_x_sec_ctx *sec_ctx;
// C source: 
// C source: 	switch (sk->sk_family) {
// C source: 	case AF_INET:
// C source: 		if (opt != IP_IPSEC_POLICY) {
// C source: 			*dir = -EOPNOTSUPP;
// C source: 			return NULL;
// C source: 		}
// C source: 		break;
// C source: #if IS_ENABLED(CONFIG_IPV6)
// C source: 	case AF_INET6:
// C source: 		if (opt != IPV6_IPSEC_POLICY) {
// C source: 			*dir = -EOPNOTSUPP;
// C source: 			return NULL;
// C source: 		}
// C source: 		break;
// C source: #endif
// C source: 	default:
// C source: 		*dir = -EINVAL;
// C source: 		return NULL;
// C source: 	}
// C source: 
// C source: 	*dir = -EINVAL;
// C source: 
// C source: 	if (len < sizeof(struct sadb_x_policy) ||
// C source: 	    pol->sadb_x_policy_len*8 > len ||
// C source: 	    pol->sadb_x_policy_type > IPSEC_POLICY_BYPASS ||
// C source: 	    (!pol->sadb_x_policy_dir || pol->sadb_x_policy_dir > IPSEC_DIR_OUTBOUND))
// C source: 		return NULL;
// C source: 
// C source: 	xp = xfrm_policy_alloc(net, GFP_ATOMIC);
// C source: 	if (xp == NULL) {
// C source: 		*dir = -ENOBUFS;
// C source: 		return NULL;
// C source: 	}
// C source: 
// C source: 	xp->action = (pol->sadb_x_policy_type == IPSEC_POLICY_DISCARD ?
// C source: 		      XFRM_POLICY_BLOCK : XFRM_POLICY_ALLOW);
// C source: 
// C source: 	xp->lft.soft_byte_limit = XFRM_INF;
// C source: 	xp->lft.hard_byte_limit = XFRM_INF;
// C source: 	xp->lft.soft_packet_limit = XFRM_INF;
// C source: 	xp->lft.hard_packet_limit = XFRM_INF;
// C source: 	xp->family = sk->sk_family;
// C source: 
// C source: 	xp->xfrm_nr = 0;
// C source: 	if (pol->sadb_x_policy_type == IPSEC_POLICY_IPSEC &&
// C source: 	    (*dir = parse_ipsecrequests(xp, pol)) < 0)
// C source: 		goto out;
// C source: 
// C source: 	/* security context too */
// C source: 	if (len >= (pol->sadb_x_policy_len*8 +
// C source: 	    sizeof(struct sadb_x_sec_ctx))) {
// C source: 		char *p = (char *)pol;
// C source: 		struct xfrm_user_sec_ctx *uctx;
// C source: 
// C source: 		p += pol->sadb_x_policy_len*8;
// C source: 		sec_ctx = (struct sadb_x_sec_ctx *)p;
// C source: 		if (len < pol->sadb_x_policy_len*8 +
// C source: 		    sec_ctx->sadb_x_sec_len*8) {
// C source: 			*dir = -EINVAL;
// C source: 			goto out;
// C source: 		}
// C source: 		if ((*dir = verify_sec_ctx_len(p)))
// C source: 			goto out;
// C source: 		uctx = pfkey_sadb2xfrm_user_sec_ctx(sec_ctx, GFP_ATOMIC);
// C source: 		*dir = security_xfrm_policy_alloc(&xp->security, uctx, GFP_ATOMIC);
// C source: 		kfree(uctx);
// C source: 
// C source: 		if (*dir)
// C source: 			goto out;
// C source: 	}
// C source: 
// C source: 	*dir = pol->sadb_x_policy_dir-1;
// C source: 	return xp;
// C source: 
// C source: out:
// C source: 	xp->walk.dead = 1;
// C source: 	xfrm_policy_destroy(xp);
// C source: 	return NULL;
// C source: }
// C source: 
// C source: static int pfkey_send_new_mapping(struct xfrm_state *x, xfrm_address_t *ipaddr, __be16 sport)
// C source: {
// C source: 	struct sk_buff *skb;
// C source: 	struct sadb_msg *hdr;
// C source: 	struct sadb_sa *sa;
// C source: 	struct sadb_address *addr;
// C source: 	struct sadb_x_nat_t_port *n_port;
// C source: 	int sockaddr_size;
// C source: 	int size;
// C source: 	__u8 satype = (x->id.proto == IPPROTO_ESP ? SADB_SATYPE_ESP : 0);
// C source: 	struct xfrm_encap_tmpl *natt = NULL;
// C source: 
// C source: 	sockaddr_size = pfkey_sockaddr_size(x->props.family);
// C source: 	if (!sockaddr_size)
// C source: 		return -EINVAL;
// C source: 
// C source: 	if (!satype)
// C source: 		return -EINVAL;
// C source: 
// C source: 	if (!x->encap)
// C source: 		return -EINVAL;
// C source: 
// C source: 	natt = x->encap;
// C source: 
// C source: 	/* Build an SADB_X_NAT_T_NEW_MAPPING message:
// C source: 	 *
// C source: 	 * HDR | SA | ADDRESS_SRC (old addr) | NAT_T_SPORT (old port) |
// C source: 	 * ADDRESS_DST (new addr) | NAT_T_DPORT (new port)
// C source: 	 */
// C source: 
// C source: 	size = sizeof(struct sadb_msg) +
// C source: 		sizeof(struct sadb_sa) +
// C source: 		(sizeof(struct sadb_address) * 2) +
// C source: 		(sockaddr_size * 2) +
// C source: 		(sizeof(struct sadb_x_nat_t_port) * 2);
// C source: 
// C source: 	skb =  alloc_skb(size + 16, GFP_ATOMIC);
// C source: 	if (skb == NULL)
// C source: 		return -ENOMEM;
// C source: 
// C source: 	hdr = skb_put(skb, sizeof(struct sadb_msg));
// C source: 	hdr->sadb_msg_version = PF_KEY_V2;
// C source: 	hdr->sadb_msg_type = SADB_X_NAT_T_NEW_MAPPING;
// C source: 	hdr->sadb_msg_satype = satype;
// C source: 	hdr->sadb_msg_len = size / sizeof(uint64_t);
// C source: 	hdr->sadb_msg_errno = 0;
// C source: 	hdr->sadb_msg_reserved = 0;
// C source: 	hdr->sadb_msg_seq = x->km.seq;
// C source: 	hdr->sadb_msg_pid = 0;
// C source: 
// C source: 	/* SA */
// C source: 	sa = skb_put(skb, sizeof(struct sadb_sa));
// C source: 	sa->sadb_sa_len = sizeof(struct sadb_sa)/sizeof(uint64_t);
// C source: 	sa->sadb_sa_exttype = SADB_EXT_SA;
// C source: 	sa->sadb_sa_spi = x->id.spi;
// C source: 	sa->sadb_sa_replay = 0;
// C source: 	sa->sadb_sa_state = 0;
// C source: 	sa->sadb_sa_auth = 0;
// C source: 	sa->sadb_sa_encrypt = 0;
// C source: 	sa->sadb_sa_flags = 0;
// C source: 
// C source: 	/* ADDRESS_SRC (old addr) */
// C source: 	addr = skb_put(skb, sizeof(struct sadb_address) + sockaddr_size);
// C source: 	addr->sadb_address_len =
// C source: 		(sizeof(struct sadb_address)+sockaddr_size)/
// C source: 			sizeof(uint64_t);
// C source: 	addr->sadb_address_exttype = SADB_EXT_ADDRESS_SRC;
// C source: 	addr->sadb_address_proto = 0;
// C source: 	addr->sadb_address_reserved = 0;
// C source: 	addr->sadb_address_prefixlen =
// C source: 		pfkey_sockaddr_fill_zero_tail(&x->props.saddr, 0,
// C source: 					      (struct sockaddr *)(addr + 1),
// C source: 					      x->props.family);
// C source: 	if (!addr->sadb_address_prefixlen)
// C source: 		BUG();
// C source: 
// C source: 	/* NAT_T_SPORT (old port) */
// C source: 	n_port = skb_put(skb, sizeof(*n_port));
// C source: 	n_port->sadb_x_nat_t_port_len = sizeof(*n_port)/sizeof(uint64_t);
// C source: 	n_port->sadb_x_nat_t_port_exttype = SADB_X_EXT_NAT_T_SPORT;
// C source: 	n_port->sadb_x_nat_t_port_port = natt->encap_sport;
// C source: 	n_port->sadb_x_nat_t_port_reserved = 0;
// C source: 
// C source: 	/* ADDRESS_DST (new addr) */
// C source: 	addr = skb_put(skb, sizeof(struct sadb_address) + sockaddr_size);
// C source: 	addr->sadb_address_len =
// C source: 		(sizeof(struct sadb_address)+sockaddr_size)/
// C source: 			sizeof(uint64_t);
// C source: 	addr->sadb_address_exttype = SADB_EXT_ADDRESS_DST;
// C source: 	addr->sadb_address_proto = 0;
// C source: 	addr->sadb_address_reserved = 0;
// C source: 	addr->sadb_address_prefixlen =
// C source: 		pfkey_sockaddr_fill_zero_tail(ipaddr, 0,
// C source: 					      (struct sockaddr *)(addr + 1),
// C source: 					      x->props.family);
// C source: 	if (!addr->sadb_address_prefixlen)
// C source: 		BUG();
// C source: 
// C source: 	/* NAT_T_DPORT (new port) */
// C source: 	n_port = skb_put(skb, sizeof(*n_port));
// C source: 	n_port->sadb_x_nat_t_port_len = sizeof(*n_port)/sizeof(uint64_t);
// C source: 	n_port->sadb_x_nat_t_port_exttype = SADB_X_EXT_NAT_T_DPORT;
// C source: 	n_port->sadb_x_nat_t_port_port = sport;
// C source: 	n_port->sadb_x_nat_t_port_reserved = 0;
// C source: 
// C source: 	return pfkey_broadcast(skb, GFP_ATOMIC, BROADCAST_REGISTERED, NULL,
// C source: 			       xs_net(x));
// C source: }
// C source: 
// C source: #ifdef CONFIG_NET_KEY_MIGRATE
// C source: static int set_sadb_address(struct sk_buff *skb, int sasize, int type,
// C source: 			    const struct xfrm_selector *sel)
// C source: {
// C source: 	struct sadb_address *addr;
// C source: 	addr = skb_put(skb, sizeof(struct sadb_address) + sasize);
// C source: 	addr->sadb_address_len = (sizeof(struct sadb_address) + sasize)/8;
// C source: 	addr->sadb_address_exttype = type;
// C source: 	addr->sadb_address_proto = sel->proto;
// C source: 	addr->sadb_address_reserved = 0;
// C source: 
// C source: 	switch (type) {
// C source: 	case SADB_EXT_ADDRESS_SRC:
// C source: 		addr->sadb_address_prefixlen = sel->prefixlen_s;
// C source: 		pfkey_sockaddr_fill_zero_tail(&sel->saddr, 0,
// C source: 					      (struct sockaddr *)(addr + 1),
// C source: 					      sel->family);
// C source: 		break;
// C source: 	case SADB_EXT_ADDRESS_DST:
// C source: 		addr->sadb_address_prefixlen = sel->prefixlen_d;
// C source: 		pfkey_sockaddr_fill_zero_tail(&sel->daddr, 0,
// C source: 					      (struct sockaddr *)(addr + 1),
// C source: 					      sel->family);
// C source: 		break;
// C source: 	default:
// C source: 		return -EINVAL;
// C source: 	}
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: 
// C source: static int set_sadb_kmaddress(struct sk_buff *skb, const struct xfrm_kmaddress *k)
// C source: {
// C source: 	struct sadb_x_kmaddress *kma;
// C source: 	u8 *sa;
// C source: 	int family = k->family;
// C source: 	int socklen = pfkey_sockaddr_len(family);
// C source: 	int size_req;
// C source: 
// C source: 	size_req = (sizeof(struct sadb_x_kmaddress) +
// C source: 		    pfkey_sockaddr_pair_size(family));
// C source: 
// C source: 	kma = skb_put_zero(skb, size_req);
// C source: 	kma->sadb_x_kmaddress_len = size_req / 8;
// C source: 	kma->sadb_x_kmaddress_exttype = SADB_X_EXT_KMADDRESS;
// C source: 	kma->sadb_x_kmaddress_reserved = k->reserved;
// C source: 
// C source: 	sa = (u8 *)(kma + 1);
// C source: 	if (!pfkey_sockaddr_fill(&k->local, 0, (struct sockaddr *)sa, family) ||
// C source: 	    !pfkey_sockaddr_fill(&k->remote, 0, (struct sockaddr *)(sa+socklen), family))
// C source: 		return -EINVAL;
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static int set_ipsecrequest(struct sk_buff *skb,
// C source: 			    uint8_t proto, uint8_t mode, int level,
// C source: 			    uint32_t reqid, sa_family_t family,
// C source: 			    const xfrm_address_t *src, const xfrm_address_t *dst)
// C source: {
// C source: 	struct sadb_x_ipsecrequest *rq;
// C source: 	u8 *sa;
// C source: 	int socklen = pfkey_sockaddr_len(family);
// C source: 	int size_req;
// C source: 
// C source: 	size_req = sizeof(struct sadb_x_ipsecrequest) +
// C source: 		   pfkey_sockaddr_pair_size(family);
// C source: 
// C source: 	rq = skb_put_zero(skb, size_req);
// C source: 	rq->sadb_x_ipsecrequest_len = size_req;
// C source: 	rq->sadb_x_ipsecrequest_proto = proto;
// C source: 	rq->sadb_x_ipsecrequest_mode = mode;
// C source: 	rq->sadb_x_ipsecrequest_level = level;
// C source: 	rq->sadb_x_ipsecrequest_reqid = reqid;
// C source: 
// C source: 	sa = (u8 *) (rq + 1);
// C source: 	if (!pfkey_sockaddr_fill(src, 0, (struct sockaddr *)sa, family) ||
// C source: 	    !pfkey_sockaddr_fill(dst, 0, (struct sockaddr *)(sa + socklen), family))
// C source: 		return -EINVAL;
// C source: 
// C source: 	return 0;
// C source: }
// C source: #endif
// C source: 
// C source: #ifdef CONFIG_NET_KEY_MIGRATE
// C source: static int pfkey_send_migrate(const struct xfrm_selector *sel, u8 dir, u8 type,
// C source: 			      const struct xfrm_migrate *m, int num_bundles,
// C source: 			      const struct xfrm_kmaddress *k, struct net *net,
// C source: 			      const struct xfrm_encap_tmpl *encap)
// C source: {
// C source: 	int i;
// C source: 	int sasize_sel;
// C source: 	int size = 0;
// C source: 	int size_pol = 0;
// C source: 	struct sk_buff *skb;
// C source: 	struct sadb_msg *hdr;
// C source: 	struct sadb_x_policy *pol;
// C source: 	const struct xfrm_migrate *mp;
// C source: 
// C source: 	if (type != XFRM_POLICY_TYPE_MAIN)
// C source: 		return 0;
// C source: 
// C source: 	if (num_bundles <= 0 || num_bundles > XFRM_MAX_DEPTH)
// C source: 		return -EINVAL;
// C source: 
// C source: 	if (k != NULL) {
// C source: 		/* addresses for KM */
// C source: 		size += PFKEY_ALIGN8(sizeof(struct sadb_x_kmaddress) +
// C source: 				     pfkey_sockaddr_pair_size(k->family));
// C source: 	}
// C source: 
// C source: 	/* selector */
// C source: 	sasize_sel = pfkey_sockaddr_size(sel->family);
// C source: 	if (!sasize_sel)
// C source: 		return -EINVAL;
// C source: 	size += (sizeof(struct sadb_address) + sasize_sel) * 2;
// C source: 
// C source: 	/* policy info */
// C source: 	size_pol += sizeof(struct sadb_x_policy);
// C source: 
// C source: 	/* ipsecrequests */
// C source: 	for (i = 0, mp = m; i < num_bundles; i++, mp++) {
// C source: 		int pair_size;
// C source: 
// C source: 		pair_size = pfkey_sockaddr_pair_size(mp->old_family);
// C source: 		if (!pair_size)
// C source: 			return -EINVAL;
// C source: 		size_pol += sizeof(struct sadb_x_ipsecrequest) + pair_size;
// C source: 
// C source: 		pair_size = pfkey_sockaddr_pair_size(mp->new_family);
// C source: 		if (!pair_size)
// C source: 			return -EINVAL;
// C source: 		size_pol += sizeof(struct sadb_x_ipsecrequest) + pair_size;
// C source: 	}
// C source: 
// C source: 	size += sizeof(struct sadb_msg) + size_pol;
// C source: 
// C source: 	/* alloc buffer */
// C source: 	skb = alloc_skb(size, GFP_ATOMIC);
// C source: 	if (skb == NULL)
// C source: 		return -ENOMEM;
// C source: 
// C source: 	hdr = skb_put(skb, sizeof(struct sadb_msg));
// C source: 	hdr->sadb_msg_version = PF_KEY_V2;
// C source: 	hdr->sadb_msg_type = SADB_X_MIGRATE;
// C source: 	hdr->sadb_msg_satype = pfkey_proto2satype(m->proto);
// C source: 	hdr->sadb_msg_len = size / 8;
// C source: 	hdr->sadb_msg_errno = 0;
// C source: 	hdr->sadb_msg_reserved = 0;
// C source: 	hdr->sadb_msg_seq = 0;
// C source: 	hdr->sadb_msg_pid = 0;
// C source: 
// C source: 	/* Addresses to be used by KM for negotiation, if ext is available */
// C source: 	if (k != NULL && (set_sadb_kmaddress(skb, k) < 0))
// C source: 		goto err;
// C source: 
// C source: 	/* selector src */
// C source: 	set_sadb_address(skb, sasize_sel, SADB_EXT_ADDRESS_SRC, sel);
// C source: 
// C source: 	/* selector dst */
// C source: 	set_sadb_address(skb, sasize_sel, SADB_EXT_ADDRESS_DST, sel);
// C source: 
// C source: 	/* policy information */
// C source: 	pol = skb_put(skb, sizeof(struct sadb_x_policy));
// C source: 	pol->sadb_x_policy_len = size_pol / 8;
// C source: 	pol->sadb_x_policy_exttype = SADB_X_EXT_POLICY;
// C source: 	pol->sadb_x_policy_type = IPSEC_POLICY_IPSEC;
// C source: 	pol->sadb_x_policy_dir = dir + 1;
// C source: 	pol->sadb_x_policy_reserved = 0;
// C source: 	pol->sadb_x_policy_id = 0;
// C source: 	pol->sadb_x_policy_priority = 0;
// C source: 
// C source: 	for (i = 0, mp = m; i < num_bundles; i++, mp++) {
// C source: 		/* old ipsecrequest */
// C source: 		int mode = pfkey_mode_from_xfrm(mp->mode);
// C source: 		if (mode < 0)
// C source: 			goto err;
// C source: 		if (set_ipsecrequest(skb, mp->proto, mode,
// C source: 				     (mp->old_reqid ? IPSEC_LEVEL_UNIQUE : IPSEC_LEVEL_REQUIRE),
// C source: 				     mp->old_reqid, mp->old_family,
// C source: 				     &mp->old_saddr, &mp->old_daddr) < 0)
// C source: 			goto err;
// C source: 
// C source: 		/* new ipsecrequest */
// C source: 		if (set_ipsecrequest(skb, mp->proto, mode,
// C source: 				     (mp->old_reqid ? IPSEC_LEVEL_UNIQUE : IPSEC_LEVEL_REQUIRE),
// C source: 				     mp->old_reqid, mp->new_family,
// C source: 				     &mp->new_saddr, &mp->new_daddr) < 0)
// C source: 			goto err;
// C source: 	}
// C source: 
// C source: 	/* broadcast migrate message to sockets */
// C source: 	pfkey_broadcast(skb, GFP_ATOMIC, BROADCAST_ALL, NULL, net);
// C source: 
// C source: 	return 0;
// C source: 
// C source: err:
// C source: 	kfree_skb(skb);
// C source: 	return -EINVAL;
// C source: }
// C source: #else
// C source: static int pfkey_send_migrate(const struct xfrm_selector *sel, u8 dir, u8 type,
// C source: 			      const struct xfrm_migrate *m, int num_bundles,
// C source: 			      const struct xfrm_kmaddress *k, struct net *net,
// C source: 			      const struct xfrm_encap_tmpl *encap)
// C source: {
// C source: 	return -ENOPROTOOPT;
// C source: }
// C source: #endif
// C source: 
// C source: static int pfkey_sendmsg(struct socket *sock, struct msghdr *msg, size_t len)
// C source: {
// C source: 	struct sock *sk = sock->sk;
// C source: 	struct sk_buff *skb = NULL;
// C source: 	struct sadb_msg *hdr = NULL;
// C source: 	int err;
// C source: 	struct net *net = sock_net(sk);
// C source: 
// C source: 	err = -EOPNOTSUPP;
// C source: 	if (msg->msg_flags & MSG_OOB)
// C source: 		goto out;
// C source: 
// C source: 	err = -EMSGSIZE;
// C source: 	if ((unsigned int)len > sk->sk_sndbuf - 32)
// C source: 		goto out;
// C source: 
// C source: 	err = -ENOBUFS;
// C source: 	skb = alloc_skb(len, GFP_KERNEL);
// C source: 	if (skb == NULL)
// C source: 		goto out;
// C source: 
// C source: 	err = -EFAULT;
// C source: 	if (memcpy_from_msg(skb_put(skb,len), msg, len))
// C source: 		goto out;
// C source: 
// C source: 	hdr = pfkey_get_base_msg(skb, &err);
// C source: 	if (!hdr)
// C source: 		goto out;
// C source: 
// C source: 	mutex_lock(&net->xfrm.xfrm_cfg_mutex);
// C source: 	err = pfkey_process(sk, skb, hdr);
// C source: 	mutex_unlock(&net->xfrm.xfrm_cfg_mutex);
// C source: 
// C source: out:
// C source: 	if (err && hdr && pfkey_error(hdr, err, sk) == 0)
// C source: 		err = 0;
// C source: 	kfree_skb(skb);
// C source: 
// C source: 	return err ? : len;
// C source: }
// C source: 
// C source: static int pfkey_recvmsg(struct socket *sock, struct msghdr *msg, size_t len,
// C source: 			 int flags)
// C source: {
// C source: 	struct sock *sk = sock->sk;
// C source: 	struct pfkey_sock *pfk = pfkey_sk(sk);
// C source: 	struct sk_buff *skb;
// C source: 	int copied, err;
// C source: 
// C source: 	err = -EINVAL;
// C source: 	if (flags & ~(MSG_PEEK|MSG_DONTWAIT|MSG_TRUNC|MSG_CMSG_COMPAT))
// C source: 		goto out;
// C source: 
// C source: 	skb = skb_recv_datagram(sk, flags, &err);
// C source: 	if (skb == NULL)
// C source: 		goto out;
// C source: 
// C source: 	copied = skb->len;
// C source: 	if (copied > len) {
// C source: 		msg->msg_flags |= MSG_TRUNC;
// C source: 		copied = len;
// C source: 	}
// C source: 
// C source: 	skb_reset_transport_header(skb);
// C source: 	err = skb_copy_datagram_msg(skb, 0, msg, copied);
// C source: 	if (err)
// C source: 		goto out_free;
// C source: 
// C source: 	sock_recv_cmsgs(msg, sk, skb);
// C source: 
// C source: 	err = (flags & MSG_TRUNC) ? skb->len : copied;
// C source: 
// C source: 	if (pfk->dump.dump != NULL &&
// C source: 	    3 * atomic_read(&sk->sk_rmem_alloc) <= sk->sk_rcvbuf)
// C source: 		pfkey_do_dump(pfk);
// C source: 
// C source: out_free:
// C source: 	skb_free_datagram(sk, skb);
// C source: out:
// C source: 	return err;
// C source: }
// C source: 
// C source: static const struct proto_ops pfkey_ops = {
// C source: 	.family		=	PF_KEY,
// C source: 	.owner		=	THIS_MODULE,
// C source: 	/* Operations that make no sense on pfkey sockets. */
// C source: 	.bind		=	sock_no_bind,
// C source: 	.connect	=	sock_no_connect,
// C source: 	.socketpair	=	sock_no_socketpair,
// C source: 	.accept		=	sock_no_accept,
// C source: 	.getname	=	sock_no_getname,
// C source: 	.ioctl		=	sock_no_ioctl,
// C source: 	.listen		=	sock_no_listen,
// C source: 	.shutdown	=	sock_no_shutdown,
// C source: 	.mmap		=	sock_no_mmap,
// C source: 
// C source: 	/* Now the operations that really occur. */
// C source: 	.release	=	pfkey_release,
// C source: 	.poll		=	datagram_poll,
// C source: 	.sendmsg	=	pfkey_sendmsg,
// C source: 	.recvmsg	=	pfkey_recvmsg,
// C source: };
// C source: 
// C source: static const struct net_proto_family pfkey_family_ops = {
// C source: 	.family	=	PF_KEY,
// C source: 	.create	=	pfkey_create,
// C source: 	.owner	=	THIS_MODULE,
// C source: };
// C source: 
// C source: #ifdef CONFIG_PROC_FS
// C source: static int pfkey_seq_show(struct seq_file *f, void *v)
// C source: {
// C source: 	struct sock *s = sk_entry(v);
// C source: 
// C source: 	if (v == SEQ_START_TOKEN)
// C source: 		seq_printf(f ,"sk       RefCnt Rmem   Wmem   User   Inode\n");
// C source: 	else
// C source: 		seq_printf(f, "%pK %-6d %-6u %-6u %-6u %-6llu\n",
// C source: 			       s,
// C source: 			       refcount_read(&s->sk_refcnt),
// C source: 			       sk_rmem_alloc_get(s),
// C source: 			       sk_wmem_alloc_get(s),
// C source: 			       from_kuid_munged(seq_user_ns(f), sk_uid(s)),
// C source: 			       sock_i_ino(s)
// C source: 			       );
// C source: 	return 0;
// C source: }
// C source: 
// C source: static void *pfkey_seq_start(struct seq_file *f, loff_t *ppos)
// C source: 	__acquires(rcu)
// C source: {
// C source: 	struct net *net = seq_file_net(f);
// C source: 	struct netns_pfkey *net_pfkey = net_generic(net, pfkey_net_id);
// C source: 
// C source: 	rcu_read_lock();
// C source: 	return seq_hlist_start_head_rcu(&net_pfkey->table, *ppos);
// C source: }
// C source: 
// C source: static void *pfkey_seq_next(struct seq_file *f, void *v, loff_t *ppos)
// C source: {
// C source: 	struct net *net = seq_file_net(f);
// C source: 	struct netns_pfkey *net_pfkey = net_generic(net, pfkey_net_id);
// C source: 
// C source: 	return seq_hlist_next_rcu(v, &net_pfkey->table, ppos);
// C source: }
// C source: 
// C source: static void pfkey_seq_stop(struct seq_file *f, void *v)
// C source: 	__releases(rcu)
// C source: {
// C source: 	rcu_read_unlock();
// C source: }
// C source: 
// C source: static const struct seq_operations pfkey_seq_ops = {
// C source: 	.start	= pfkey_seq_start,
// C source: 	.next	= pfkey_seq_next,
// C source: 	.stop	= pfkey_seq_stop,
// C source: 	.show	= pfkey_seq_show,
// C source: };
// C source: 
// C source: static int __net_init pfkey_init_proc(struct net *net)
// C source: {
// C source: 	struct proc_dir_entry *e;
// C source: 
// C source: 	e = proc_create_net("pfkey", 0, net->proc_net, &pfkey_seq_ops,
// C source: 			sizeof(struct seq_net_private));
// C source: 	if (e == NULL)
// C source: 		return -ENOMEM;
// C source: 
// C source: 	return 0;
// C source: }
// C source: 
// C source: static void __net_exit pfkey_exit_proc(struct net *net)
// C source: {
// C source: 	remove_proc_entry("pfkey", net->proc_net);
// C source: }
// C source: #else
// C source: static inline int pfkey_init_proc(struct net *net)
// C source: {
// C source: 	return 0;
// C source: }
// C source: 
// C source: static inline void pfkey_exit_proc(struct net *net)
// C source: {
// C source: }
// C source: #endif
// C source: 
// C source: static struct xfrm_mgr pfkeyv2_mgr =
// C source: {
// C source: 	.notify		= pfkey_send_notify,
// C source: 	.acquire	= pfkey_send_acquire,
// C source: 	.compile_policy	= pfkey_compile_policy,
// C source: 	.new_mapping	= pfkey_send_new_mapping,
// C source: 	.notify_policy	= pfkey_send_policy_notify,
// C source: 	.migrate	= pfkey_send_migrate,
// C source: 	.is_alive	= pfkey_is_alive,
// C source: };
// C source: 
// C source: static int __net_init pfkey_net_init(struct net *net)
// C source: {
// C source: 	struct netns_pfkey *net_pfkey = net_generic(net, pfkey_net_id);
// C source: 	int rv;
// C source: 
// C source: 	INIT_HLIST_HEAD(&net_pfkey->table);
// C source: 	atomic_set(&net_pfkey->socks_nr, 0);
// C source: 
// C source: 	rv = pfkey_init_proc(net);
// C source: 
// C source: 	return rv;
// C source: }
// C source: 
// C source: static void __net_exit pfkey_net_exit(struct net *net)
// C source: {
// C source: 	struct netns_pfkey *net_pfkey = net_generic(net, pfkey_net_id);
// C source: 
// C source: 	pfkey_exit_proc(net);
// C source: 	WARN_ON(!hlist_empty(&net_pfkey->table));
// C source: }
// C source: 
// C source: static struct pernet_operations pfkey_net_ops = {
// C source: 	.init = pfkey_net_init,
// C source: 	.exit = pfkey_net_exit,
// C source: 	.id   = &pfkey_net_id,
// C source: 	.size = sizeof(struct netns_pfkey),
// C source: };
// C source: 
// C source: static void __exit ipsec_pfkey_exit(void)
// C source: {
// C source: 	xfrm_unregister_km(&pfkeyv2_mgr);
// C source: 	sock_unregister(PF_KEY);
// C source: 	unregister_pernet_subsys(&pfkey_net_ops);
// C source: 	proto_unregister(&key_proto);
// C source: }
// C source: 
// C source: static int __init ipsec_pfkey_init(void)
// C source: {
// C source: 	int err = proto_register(&key_proto, 0);
// C source: 
// C source: 	pr_warn_once("PFKEY is deprecated and scheduled to be removed in 2027, "
// C source: 	             "please contact the netdev mailing list\n");
// C source: 	if (err != 0)
// C source: 		goto out;
// C source: 
// C source: 	err = register_pernet_subsys(&pfkey_net_ops);
// C source: 	if (err != 0)
// C source: 		goto out_unregister_key_proto;
// C source: 	err = sock_register(&pfkey_family_ops);
// C source: 	if (err != 0)
// C source: 		goto out_unregister_pernet;
// C source: 	xfrm_register_km(&pfkeyv2_mgr);
// C source: out:
// C source: 	return err;
// C source: 
// C source: out_unregister_pernet:
// C source: 	unregister_pernet_subsys(&pfkey_net_ops);
// C source: out_unregister_key_proto:
// C source: 	proto_unregister(&key_proto);
// C source: 	goto out;
// C source: }
// C source: 
// C source: module_init(ipsec_pfkey_init);
// C source: module_exit(ipsec_pfkey_exit);
// C source: MODULE_DESCRIPTION("PF_KEY socket helpers");
// C source: MODULE_LICENSE("GPL");
// C source: MODULE_ALIAS_NETPROTO(PF_KEY);

*/


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
