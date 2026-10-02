/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_AH6_BINDINGS_H
#define LUPOS_AH6_BINDINGS_H
#include <crypto/hash.h>
#include <crypto/utils.h>
#include <linux/module.h>
#include <linux/slab.h>
#include <net/ip.h>
#include <net/ah.h>
#include <linux/crypto.h>
#include <linux/pfkeyv2.h>
#include <linux/string.h>
#include <linux/scatterlist.h>
#include <net/ip6_route.h>
#include <net/icmp.h>
#include <net/ipv6.h>
#include <net/protocol.h>
#include <net/xfrm.h>

/* Private C layouts from ah6.c, generated with the selected target config. */
struct ah6_tmp_ext {
#if IS_ENABLED(CONFIG_IPV6_MIP6)
	struct in6_addr saddr;
#endif
	struct in6_addr daddr;
	char hdrs[];
};
struct ah6_skb_cb {
	struct xfrm_skb_cb xfrm;
	void *tmp;
};
static_assert(sizeof(struct ah6_skb_cb) <= sizeof_field(struct sk_buff, cb));
static_assert(__alignof__(struct ah6_skb_cb) <= __alignof__(((struct sk_buff *)0)->cb));
static_assert(offsetof(struct sk_buff, cb) % __alignof__(struct ah6_skb_cb) == 0);

void *rust_ah6_kmalloc_atomic(unsigned int len);
struct ah_data *rust_ah6_alloc_data(void);
unsigned int rust_ah6_digestsize(struct crypto_ahash *tfm);
unsigned int rust_ah6_reqsize(struct crypto_ahash *tfm);
unsigned int rust_ah6_ctx_alignment(void);
void rust_ah6_request_set_tfm(struct ahash_request *req, struct crypto_ahash *tfm);
void rust_ah6_request_set_crypt(struct ahash_request *req, struct scatterlist *sg,
			       u8 *result, unsigned int nbytes);
void rust_ah6_request_set_callback(struct ahash_request *req, u32 flags,
				  crypto_completion_t complete, void *data);
void rust_ah6_free_ahash(struct crypto_ahash *tfm);
int rust_ah6_memneq(const void *a, const void *b, size_t size);
bool rust_ah6_is_err(const void *p);
void rust_ah6_bug_on(bool condition);
struct ipv6hdr *rust_ah6_ipv6_hdr(const struct sk_buff *skb);
struct ip_auth_hdr *rust_ah6_auth_hdr(const struct sk_buff *skb);
struct dst_entry *rust_ah6_skb_dst(const struct sk_buff *skb);
struct sock *rust_ah6_skb_to_full_sk(struct sk_buff *skb);
struct xfrm_state *rust_ah6_input_state(struct sk_buff *skb);
u32 rust_ah6_network_header_len(const struct sk_buff *skb);
int rust_ah6_network_offset(const struct sk_buff *skb);
u8 *rust_ah6_mac_header(const struct sk_buff *skb);
u8 *rust_ah6_network_header(const struct sk_buff *skb);
u16 *rust_ah6_network_header_slot(struct sk_buff *skb);
bool rust_ah6_may_pull(struct sk_buff *skb, unsigned int len);
int rust_ah6_unclone(struct sk_buff *skb);
u8 *rust_ah6_pull(struct sk_buff *skb, unsigned int len);
void rust_ah6_reset_transport_header(struct sk_buff *skb);
void rust_ah6_set_transport_header(struct sk_buff *skb, int offset);
void rust_ah6_set_checksum_none(struct sk_buff *skb);
void rust_ah6_sg_set_buf(struct scatterlist *sg, const void *buf, unsigned int len);
struct net_device *rust_ah6_skb_dev(const struct sk_buff *skb);
u32 rust_ah6_skb_mark(const struct sk_buff *skb);
struct net *rust_ah6_dev_net(const struct net_device *dev);
kuid_t rust_ah6_sock_net_uid(const struct net *net, const struct sock *sk);
void rust_ah6_state_put(struct xfrm_state *x);
struct in6_addr *rust_ah6_saddr(struct ipv6hdr *iph);
struct in6_addr *rust_ah6_daddr(struct ipv6hdr *iph);
void rust_ah6_set_priority(struct ipv6hdr *iph, u8 priority);
void rust_ah6_warn_hao(u8 length);
void rust_ah6_debug_overrun(bool hop);
void rust_ah6_extack_auth(struct netlink_ext_ack *extack);
void rust_ah6_extack_encap(struct netlink_ext_ack *extack);
void rust_ah6_extack_crypto(struct netlink_ext_ack *extack);
void rust_ah6_extack_mode(struct netlink_ext_ack *extack);
void rust_ah6_log_add_type(void);
void rust_ah6_log_add_protocol(void);
void rust_ah6_log_remove_protocol(void);
int rust_ah6_init(void);
void rust_ah6_fini(void);
#endif
