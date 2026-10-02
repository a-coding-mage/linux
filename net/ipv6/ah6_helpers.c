// SPDX-License-Identifier: GPL-2.0-or-later
/* Macro/header-inline boundary only. All AH policy lives in ah6.rs. */
#define pr_fmt(fmt) "IPv6: " fmt
#include "ah6_bindings.h"

void *rust_ah6_kmalloc_atomic(unsigned int len) { return kmalloc(len, GFP_ATOMIC); }
struct ah_data *rust_ah6_alloc_data(void) { return kzalloc_obj(struct ah_data); }
unsigned int rust_ah6_digestsize(struct crypto_ahash *tfm) { return crypto_ahash_digestsize(tfm); }
unsigned int rust_ah6_reqsize(struct crypto_ahash *tfm) { return crypto_ahash_reqsize(tfm); }
unsigned int rust_ah6_ctx_alignment(void) { return crypto_tfm_ctx_alignment(); }
void rust_ah6_request_set_tfm(struct ahash_request *req, struct crypto_ahash *tfm) { ahash_request_set_tfm(req, tfm); }
void rust_ah6_request_set_crypt(struct ahash_request *req, struct scatterlist *sg, u8 *result, unsigned int nbytes) { ahash_request_set_crypt(req, sg, result, nbytes); }
void rust_ah6_request_set_callback(struct ahash_request *req, u32 flags, crypto_completion_t complete, void *data) { ahash_request_set_callback(req, flags, complete, data); }
void rust_ah6_free_ahash(struct crypto_ahash *tfm) { crypto_free_ahash(tfm); }
int rust_ah6_memneq(const void *a, const void *b, size_t size) { return crypto_memneq(a, b, size); }
bool rust_ah6_is_err(const void *p) { return IS_ERR(p); }
void rust_ah6_bug_on(bool condition) { BUG_ON(condition); }
struct ipv6hdr *rust_ah6_ipv6_hdr(const struct sk_buff *skb) { return ipv6_hdr(skb); }
struct ip_auth_hdr *rust_ah6_auth_hdr(const struct sk_buff *skb) { return ip_auth_hdr(skb); }
struct dst_entry *rust_ah6_skb_dst(const struct sk_buff *skb) { return skb_dst(skb); }
struct sock *rust_ah6_skb_to_full_sk(struct sk_buff *skb) { return skb_to_full_sk(skb); }
struct xfrm_state *rust_ah6_input_state(struct sk_buff *skb) { return xfrm_input_state(skb); }
u32 rust_ah6_network_header_len(const struct sk_buff *skb) { return skb_network_header_len(skb); }
int rust_ah6_network_offset(const struct sk_buff *skb) { return skb_network_offset(skb); }
u8 *rust_ah6_mac_header(const struct sk_buff *skb) { return skb_mac_header(skb); }
u8 *rust_ah6_network_header(const struct sk_buff *skb) { return skb_network_header(skb); }
u16 *rust_ah6_network_header_slot(struct sk_buff *skb) { return &skb->network_header; }
bool rust_ah6_may_pull(struct sk_buff *skb, unsigned int len) { return pskb_may_pull(skb, len); }
int rust_ah6_unclone(struct sk_buff *skb) { return skb_unclone(skb, GFP_ATOMIC); }
u8 *rust_ah6_pull(struct sk_buff *skb, unsigned int len) { return __skb_pull(skb, len); }
void rust_ah6_reset_transport_header(struct sk_buff *skb) { skb_reset_transport_header(skb); }
void rust_ah6_set_transport_header(struct sk_buff *skb, int offset) { skb_set_transport_header(skb, offset); }
void rust_ah6_set_checksum_none(struct sk_buff *skb) { skb->ip_summed = CHECKSUM_NONE; }
void rust_ah6_sg_set_buf(struct scatterlist *sg, const void *buf, unsigned int len) { sg_set_buf(sg, buf, len); }
struct net_device *rust_ah6_skb_dev(const struct sk_buff *skb) { return skb->dev; }
u32 rust_ah6_skb_mark(const struct sk_buff *skb) { return skb->mark; }
struct net *rust_ah6_dev_net(const struct net_device *dev) { return dev_net(dev); }
kuid_t rust_ah6_sock_net_uid(const struct net *net, const struct sock *sk) { return sock_net_uid(net, sk); }
void rust_ah6_state_put(struct xfrm_state *x) { xfrm_state_put(x); }
struct in6_addr *rust_ah6_saddr(struct ipv6hdr *iph) { return &iph->saddr; }
struct in6_addr *rust_ah6_daddr(struct ipv6hdr *iph) { return &iph->daddr; }
void rust_ah6_set_priority(struct ipv6hdr *iph, u8 priority) { iph->priority = priority; }
void rust_ah6_warn_hao(u8 length) { net_warn_ratelimited("destopt hao: invalid header length: %u\n", length); }
void rust_ah6_debug_overrun(bool hop) { net_dbg_ratelimited("overrun %sopts\n", hop ? "hop" : "dest"); }
void rust_ah6_extack_auth(struct netlink_ext_ack *extack) { NL_SET_ERR_MSG(extack, "AH requires a state with an AUTH algorithm"); }
void rust_ah6_extack_encap(struct netlink_ext_ack *extack) { NL_SET_ERR_MSG(extack, "AH is not compatible with encapsulation"); }
void rust_ah6_extack_crypto(struct netlink_ext_ack *extack) { NL_SET_ERR_MSG(extack, "Kernel was unable to initialize cryptographic operations"); }
void rust_ah6_extack_mode(struct netlink_ext_ack *extack) { NL_SET_ERR_MSG(extack, "Invalid mode requested for AH, must be one of TRANSPORT, TUNNEL, BEET"); }
void rust_ah6_log_add_type(void) { pr_info("%s: can't add xfrm type\n", "ah6_init"); }
void rust_ah6_log_add_protocol(void) { pr_info("%s: can't add protocol\n", "ah6_init"); }
void rust_ah6_log_remove_protocol(void) { pr_info("%s: can't remove protocol\n", "ah6_fini"); }

/* Let Kbuild and the C compiler own actual init/exit aliases, CFI, module
 * metadata and their DWARF/BTF. These wrappers contain no registration policy. */
static int __init ah6_init(void) { return rust_ah6_init(); }
static void __exit ah6_fini(void) { rust_ah6_fini(); }
module_init(ah6_init);
module_exit(ah6_fini);
MODULE_DESCRIPTION("IPv6 AH transformation helpers");
MODULE_LICENSE("GPL");
MODULE_ALIAS_XFRM_TYPE(AF_INET6, XFRM_PROTO_AH);
