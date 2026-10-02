/* SPDX-License-Identifier: GPL-2.0-only */
/* NEW RECONSTRUCTION: target-header ABI and macro/inline boundaries only. */
#ifndef LUPOS_ETHTOOL_COALESCE_BINDINGS_H
#define LUPOS_ETHTOOL_COALESCE_BINDINGS_H
#include <linux/dim.h>
#include "common.h"
#include "netlink.h"

/* Preserve the exact callback types from the authoritative ethtool_ops. */
typedef typeof(((struct ethtool_ops *)0)->get_coalesce) rust_coalesce_get_fn;
typedef typeof(((struct ethtool_ops *)0)->set_coalesce) rust_coalesce_set_fn;
rust_coalesce_get_fn rust_coalesce_get_op(const struct net_device *dev);
rust_coalesce_set_fn rust_coalesce_set_op(const struct net_device *dev);
u32 rust_coalesce_supported(const struct net_device *dev);
struct dim_irq_moder *rust_coalesce_irq_moder(const struct net_device *dev);
struct nlattr **rust_coalesce_attrs(const struct genl_info *info);
struct netlink_ext_ack *rust_coalesce_extack(const struct genl_info *info);
int rust_coalesce_nla_total_size(int size);
int rust_coalesce_nla_put_u32(struct sk_buff *skb, int type, u32 value);
int rust_coalesce_nla_put_u8(struct sk_buff *skb, int type, u8 value);
struct nlattr *rust_coalesce_nla_nest_start(struct sk_buff *skb, int type);
int rust_coalesce_nla_nest_end(struct sk_buff *skb, struct nlattr *start);
void rust_coalesce_nla_nest_cancel(struct sk_buff *skb, struct nlattr *start);
int rust_coalesce_nla_parse_nested(struct nlattr **tb, int maxtype,
	const struct nlattr *nest, const struct nla_policy *policy,
	struct netlink_ext_ack *extack);
void *rust_coalesce_nla_data(const struct nlattr *attr);
int rust_coalesce_nla_len(const struct nlattr *attr);
bool rust_coalesce_nla_ok(const struct nlattr *attr, int remaining);
struct nlattr *rust_coalesce_nla_next(const struct nlattr *attr, int *remaining);
int rust_coalesce_nla_type(const struct nlattr *attr);
u32 rust_coalesce_nla_get_u32(const struct nlattr *attr);
void rust_coalesce_update_u32(u32 *dst, const struct nlattr *attr, bool *mod);
void rust_coalesce_update_u8(u8 *dst, const struct nlattr *attr, bool *mod);
void rust_coalesce_update_bool32(u32 *dst, const struct nlattr *attr, bool *mod);
void rust_coalesce_unsupported(struct netlink_ext_ack *extack, const struct nlattr *attr);
void rust_coalesce_bad_attr(struct netlink_ext_ack *extack, const struct nlattr *attr);
void rust_coalesce_rcu_read_lock(void);
void rust_coalesce_rcu_read_unlock(void);
struct dim_cq_moder *rust_coalesce_rcu_dereference(struct dim_cq_moder *const *ptr);
struct dim_cq_moder *rust_coalesce_rtnl_dereference(struct dim_cq_moder *const *ptr);
void rust_coalesce_rcu_assign(struct dim_cq_moder **ptr, struct dim_cq_moder *value);
void rust_coalesce_kfree_rcu(struct dim_cq_moder *profile);
void *rust_coalesce_kmemdup(const void *src, size_t len);

/* Clang evaluates authoritative BIT() macros that bindgen cannot parse. */
enum {
    RUST_ETHTOOL_COALESCE_RX_USECS = ETHTOOL_COALESCE_RX_USECS,
    RUST_ETHTOOL_COALESCE_RX_MAX_FRAMES = ETHTOOL_COALESCE_RX_MAX_FRAMES,
    RUST_ETHTOOL_COALESCE_RX_USECS_IRQ = ETHTOOL_COALESCE_RX_USECS_IRQ,
    RUST_ETHTOOL_COALESCE_RX_MAX_FRAMES_IRQ = ETHTOOL_COALESCE_RX_MAX_FRAMES_IRQ,
    RUST_ETHTOOL_COALESCE_TX_USECS = ETHTOOL_COALESCE_TX_USECS,
    RUST_ETHTOOL_COALESCE_TX_MAX_FRAMES = ETHTOOL_COALESCE_TX_MAX_FRAMES,
    RUST_ETHTOOL_COALESCE_TX_USECS_IRQ = ETHTOOL_COALESCE_TX_USECS_IRQ,
    RUST_ETHTOOL_COALESCE_TX_MAX_FRAMES_IRQ = ETHTOOL_COALESCE_TX_MAX_FRAMES_IRQ,
    RUST_ETHTOOL_COALESCE_STATS_BLOCK_USECS = ETHTOOL_COALESCE_STATS_BLOCK_USECS,
    RUST_ETHTOOL_COALESCE_USE_ADAPTIVE_RX = ETHTOOL_COALESCE_USE_ADAPTIVE_RX,
    RUST_ETHTOOL_COALESCE_USE_ADAPTIVE_TX = ETHTOOL_COALESCE_USE_ADAPTIVE_TX,
    RUST_ETHTOOL_COALESCE_PKT_RATE_LOW = ETHTOOL_COALESCE_PKT_RATE_LOW,
    RUST_ETHTOOL_COALESCE_RX_USECS_LOW = ETHTOOL_COALESCE_RX_USECS_LOW,
    RUST_ETHTOOL_COALESCE_RX_MAX_FRAMES_LOW = ETHTOOL_COALESCE_RX_MAX_FRAMES_LOW,
    RUST_ETHTOOL_COALESCE_TX_USECS_LOW = ETHTOOL_COALESCE_TX_USECS_LOW,
    RUST_ETHTOOL_COALESCE_TX_MAX_FRAMES_LOW = ETHTOOL_COALESCE_TX_MAX_FRAMES_LOW,
    RUST_ETHTOOL_COALESCE_PKT_RATE_HIGH = ETHTOOL_COALESCE_PKT_RATE_HIGH,
    RUST_ETHTOOL_COALESCE_RX_USECS_HIGH = ETHTOOL_COALESCE_RX_USECS_HIGH,
    RUST_ETHTOOL_COALESCE_RX_MAX_FRAMES_HIGH = ETHTOOL_COALESCE_RX_MAX_FRAMES_HIGH,
    RUST_ETHTOOL_COALESCE_TX_USECS_HIGH = ETHTOOL_COALESCE_TX_USECS_HIGH,
    RUST_ETHTOOL_COALESCE_TX_MAX_FRAMES_HIGH = ETHTOOL_COALESCE_TX_MAX_FRAMES_HIGH,
    RUST_ETHTOOL_COALESCE_RATE_SAMPLE_INTERVAL = ETHTOOL_COALESCE_RATE_SAMPLE_INTERVAL,
    RUST_ETHTOOL_COALESCE_RX_PROFILE = ETHTOOL_COALESCE_RX_PROFILE,
    RUST_ETHTOOL_COALESCE_TX_PROFILE = ETHTOOL_COALESCE_TX_PROFILE,
    RUST_DIM_PROFILE_RX = DIM_PROFILE_RX,
    RUST_DIM_PROFILE_TX = DIM_PROFILE_TX,
    RUST_DIM_COALESCE_USEC = DIM_COALESCE_USEC,
    RUST_DIM_COALESCE_PKTS = DIM_COALESCE_PKTS,
    RUST_DIM_COALESCE_COMPS = DIM_COALESCE_COMPS,
    RUST_NLA_VALIDATE_MAX = NLA_VALIDATE_MAX,
};
#endif
