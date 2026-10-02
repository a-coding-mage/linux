// SPDX-License-Identifier: GPL-2.0-only
/* NEW RECONSTRUCTION. Only configured field access, macros and existing header
 * inline primitives. No coalesce.c policy, iteration or driver dispatch. */
#include "coalesce_bindings.h"
rust_coalesce_get_fn rust_coalesce_get_op(const struct net_device *dev)
{ return dev->ethtool_ops->get_coalesce; }
rust_coalesce_set_fn rust_coalesce_set_op(const struct net_device *dev)
{ return dev->ethtool_ops->set_coalesce; }
u32 rust_coalesce_supported(const struct net_device *dev)
{ return dev->ethtool_ops->supported_coalesce_params; }
struct dim_irq_moder *rust_coalesce_irq_moder(const struct net_device *dev)
{ return dev->irq_moder; }
struct nlattr **rust_coalesce_attrs(const struct genl_info *info)
{ return info->attrs; }
struct netlink_ext_ack *rust_coalesce_extack(const struct genl_info *info)
{ return info->extack; }
int rust_coalesce_nla_total_size(int size) { return nla_total_size(size); }
int rust_coalesce_nla_put_u32(struct sk_buff *skb, int type, u32 value)
{ return nla_put_u32(skb, type, value); }
int rust_coalesce_nla_put_u8(struct sk_buff *skb, int type, u8 value)
{ return nla_put_u8(skb, type, value); }
struct nlattr *rust_coalesce_nla_nest_start(struct sk_buff *skb, int type)
{ return nla_nest_start(skb, type); }
int rust_coalesce_nla_nest_end(struct sk_buff *skb, struct nlattr *start)
{ return nla_nest_end(skb, start); }
void rust_coalesce_nla_nest_cancel(struct sk_buff *skb, struct nlattr *start)
{ nla_nest_cancel(skb, start); }
int rust_coalesce_nla_parse_nested(struct nlattr **tb, int maxtype,
	const struct nlattr *nest, const struct nla_policy *policy,
	struct netlink_ext_ack *extack)
{ return nla_parse_nested(tb, maxtype, nest, policy, extack); }
void *rust_coalesce_nla_data(const struct nlattr *attr) { return nla_data(attr); }
int rust_coalesce_nla_len(const struct nlattr *attr) { return nla_len(attr); }
bool rust_coalesce_nla_ok(const struct nlattr *attr, int remaining)
{ return nla_ok(attr, remaining); }
struct nlattr *rust_coalesce_nla_next(const struct nlattr *attr, int *remaining)
{ return nla_next(attr, remaining); }
int rust_coalesce_nla_type(const struct nlattr *attr) { return nla_type(attr); }
u32 rust_coalesce_nla_get_u32(const struct nlattr *attr) { return nla_get_u32(attr); }
void rust_coalesce_update_u32(u32 *dst, const struct nlattr *attr, bool *mod)
{ ethnl_update_u32(dst, attr, mod); }
void rust_coalesce_update_u8(u8 *dst, const struct nlattr *attr, bool *mod)
{ ethnl_update_u8(dst, attr, mod); }
void rust_coalesce_update_bool32(u32 *dst, const struct nlattr *attr, bool *mod)
{ ethnl_update_bool32(dst, attr, mod); }
void rust_coalesce_unsupported(struct netlink_ext_ack *extack, const struct nlattr *attr)
{ NL_SET_ERR_MSG_ATTR(extack, attr, "cannot modify an unsupported parameter"); }
void rust_coalesce_bad_attr(struct netlink_ext_ack *extack, const struct nlattr *attr)
{ NL_SET_BAD_ATTR(extack, attr); }
void rust_coalesce_rcu_read_lock(void) { rcu_read_lock(); }
void rust_coalesce_rcu_read_unlock(void) { rcu_read_unlock(); }
struct dim_cq_moder *rust_coalesce_rcu_dereference(struct dim_cq_moder *const *ptr)
{ return rcu_dereference(*ptr); }
struct dim_cq_moder *rust_coalesce_rtnl_dereference(struct dim_cq_moder *const *ptr)
{ return rtnl_dereference(*ptr); }
void rust_coalesce_rcu_assign(struct dim_cq_moder **ptr, struct dim_cq_moder *value)
{ rcu_assign_pointer(*ptr, value); }
void rust_coalesce_kfree_rcu(struct dim_cq_moder *profile) { kfree_rcu(profile, rcu); }
void *rust_coalesce_kmemdup(const void *src, size_t len)
{ return kmemdup(src, len, GFP_KERNEL); }
