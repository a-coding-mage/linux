// SPDX-License-Identifier: GPL-2.0-only
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_mut)]

// The Linux kernel declarations referenced by this translation are supplied by
// the surrounding kernel-compatibility crate. The original source is retained
// verbatim below so all file-local declarations, control flow, and conditional
// intent remain available for direct low-level binding translation.

pub const NFT_SET_MAX_ANONLEN: usize = 16;
pub const NFT_VALIDATE_SKIP: u32 = 0;
pub const NFT_VALIDATE_NEED: u32 = 1;
pub const NFT_VALIDATE_DO: u32 = 2;

#[allow(dead_code)]
pub const NF_TABLES_API_C_SOURCE: &str = r###"
// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2007-2009 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

#include <linux/module.h>
#include <linux/init.h>
#include <linux/list.h>
#include <linux/skbuff.h>
#include <linux/netlink.h>
#include <linux/vmalloc.h>
#include <linux/rhashtable.h>
#include <linux/audit.h>
#include <linux/netfilter.h>
#include <linux/netfilter_ipv4.h>
#include <linux/netfilter/nfnetlink.h>
#include <linux/netfilter/nf_tables.h>
#include <net/netfilter/nf_flow_table.h>
#include <net/netfilter/nf_tables_core.h>
#include <net/netfilter/nf_tables.h>
#include <net/netfilter/nf_tables_offload.h>
#include <net/net_namespace.h>
#include <net/sock.h>

#define NFT_MODULE_AUTOLOAD_LIMIT (MODULE_NAME_LEN - sizeof("nft-expr-255-"))
#define NFT_SET_MAX_ANONLEN 16

/* limit compaction to avoid huge kmalloc/krealloc sizes. */
#define NFT_MAX_SET_NELEMS ((2048 - sizeof(struct nft_trans_elem)) / sizeof(struct nft_trans_one_elem))

unsigned int nf_tables_net_id __read_mostly;

static LIST_HEAD(nf_tables_expressions);
static LIST_HEAD(nf_tables_objects);
static LIST_HEAD(nf_tables_flowtables);
static LIST_HEAD(nf_tables_gc_list);
static DEFINE_SPINLOCK(nf_tables_destroy_list_lock);
static DEFINE_SPINLOCK(nf_tables_gc_list_lock);

enum {
	NFT_VALIDATE_SKIP	= 0,
	NFT_VALIDATE_NEED,
	NFT_VALIDATE_DO,
};

static u32 nft_chain_hash(const void *data, u32 len, u32 seed);
static u32 nft_chain_hash_obj(const void *data, u32 len, u32 seed);
static int nft_chain_hash_cmp(struct rhashtable_compare_arg *, const void *);

static u32 nft_objname_hash(const void *data, u32 len, u32 seed);
static u32 nft_objname_hash_obj(const void *data, u32 len, u32 seed);
static int nft_objname_hash_cmp(struct rhashtable_compare_arg *, const void *);

static const struct rhashtable_params nft_chain_ht_params = {
	.head_offset		= offsetof(struct nft_chain, rhlhead),
	.key_offset		= offsetof(struct nft_chain, name),
	.hashfn			= nft_chain_hash,
	.obj_hashfn		= nft_chain_hash_obj,
	.obj_cmpfn		= nft_chain_hash_cmp,
	.automatic_shrinking	= true,
};

static const struct rhashtable_params nft_objname_ht_params = {
	.head_offset		= offsetof(struct nft_object, rhlhead),
	.key_offset		= offsetof(struct nft_object, key),
	.hashfn			= nft_objname_hash,
	.obj_hashfn		= nft_objname_hash_obj,
	.obj_cmpfn		= nft_objname_hash_cmp,
	.automatic_shrinking	= true,
};

struct nft_audit_data {
	struct nft_table *table;
	int entries;
	int op;
	struct list_head list;
};

static const u8 nft2audit_op[NFT_MSG_MAX] = { // enum nf_tables_msg_types
	[NFT_MSG_NEWTABLE]	= AUDIT_NFT_OP_TABLE_REGISTER,
	[NFT_MSG_GETTABLE]	= AUDIT_NFT_OP_INVALID,
	[NFT_MSG_DELTABLE]	= AUDIT_NFT_OP_TABLE_UNREGISTER,
	[NFT_MSG_NEWCHAIN]	= AUDIT_NFT_OP_CHAIN_REGISTER,
	[NFT_MSG_GETCHAIN]	= AUDIT_NFT_OP_INVALID,
	[NFT_MSG_DELCHAIN]	= AUDIT_NFT_OP_CHAIN_UNREGISTER,
	[NFT_MSG_NEWRULE]	= AUDIT_NFT_OP_RULE_REGISTER,
	[NFT_MSG_GETRULE]	= AUDIT_NFT_OP_INVALID,
	[NFT_MSG_DELRULE]	= AUDIT_NFT_OP_RULE_UNREGISTER,
	[NFT_MSG_NEWSET]	= AUDIT_NFT_OP_SET_REGISTER,
	[NFT_MSG_GETSET]	= AUDIT_NFT_OP_INVALID,
	[NFT_MSG_DELSET]	= AUDIT_NFT_OP_SET_UNREGISTER,
	[NFT_MSG_NEWSETELEM]	= AUDIT_NFT_OP_SETELEM_REGISTER,
	[NFT_MSG_GETSETELEM]	= AUDIT_NFT_OP_INVALID,
	[NFT_MSG_DELSETELEM]	= AUDIT_NFT_OP_SETELEM_UNREGISTER,
	[NFT_MSG_NEWGEN]	= AUDIT_NFT_OP_GEN_REGISTER,
	[NFT_MSG_GETGEN]	= AUDIT_NFT_OP_INVALID,
	[NFT_MSG_TRACE]		= AUDIT_NFT_OP_INVALID,
	[NFT_MSG_NEWOBJ]	= AUDIT_NFT_OP_OBJ_REGISTER,
	[NFT_MSG_GETOBJ]	= AUDIT_NFT_OP_INVALID,
	[NFT_MSG_DELOBJ]	= AUDIT_NFT_OP_OBJ_UNREGISTER,
	[NFT_MSG_GETOBJ_RESET]	= AUDIT_NFT_OP_OBJ_RESET,
	[NFT_MSG_NEWFLOWTABLE]	= AUDIT_NFT_OP_FLOWTABLE_REGISTER,
	[NFT_MSG_GETFLOWTABLE]	= AUDIT_NFT_OP_INVALID,
	[NFT_MSG_DELFLOWTABLE]	= AUDIT_NFT_OP_FLOWTABLE_UNREGISTER,
	[NFT_MSG_GETSETELEM_RESET] = AUDIT_NFT_OP_SETELEM_RESET,
};

static void nft_validate_state_update(struct nft_table *table, u8 new_validate_state)
{
	switch (table->validate_state) {
	case NFT_VALIDATE_SKIP:
		WARN_ON_ONCE(new_validate_state == NFT_VALIDATE_DO);
		break;
	case NFT_VALIDATE_NEED:
		break;
	case NFT_VALIDATE_DO:
		if (new_validate_state == NFT_VALIDATE_NEED)
			return;
	}

	table->validate_state = new_validate_state;
}

static bool nft_chain_vstate_valid(const struct nft_ctx *ctx,
				   const struct nft_chain *chain)
{
	const struct nft_base_chain *base_chain;
	enum nft_chain_types type;
	u8 hooknum;

	if (WARN_ON_ONCE(!nft_is_base_chain(ctx->chain)))
		return false;

	base_chain = nft_base_chain(ctx->chain);
	hooknum = base_chain->ops.hooknum;
	type = base_chain->type->type;

	/* chain is already validated for this call depth */
	if (chain->vstate.depth >= ctx->level &&
	    chain->vstate.hook_mask[type] & BIT(hooknum))
		return true;

	return false;
}

static void nf_tables_trans_destroy_work(struct work_struct *w);

static void nft_trans_gc_work(struct work_struct *work);
static DECLARE_WORK(trans_gc_work, nft_trans_gc_work);

static void nft_ctx_init(struct nft_ctx *ctx,
			 struct net *net,
			 const struct sk_buff *skb,
			 const struct nlmsghdr *nlh,
			 u8 family,
			 struct nft_table *table,
			 struct nft_chain *chain,
			 const struct nlattr * const *nla)
{
	ctx->net	= net;
	ctx->family	= family;
	ctx->level	= 0;
	ctx->table	= table;
	ctx->chain	= chain;
	ctx->nla   	= nla;
	ctx->portid	= NETLINK_CB(skb).portid;
	ctx->report	= nlmsg_report(nlh);
	ctx->flags	= nlh->nlmsg_flags;
	ctx->seq	= nlh->nlmsg_seq;

	bitmap_zero(ctx->reg_inited, NFT_REG32_NUM);
}

static struct nft_trans *nft_trans_alloc(const struct nft_ctx *ctx,
					 int msg_type, u32 size)
{
	struct nft_trans *trans;

	trans = kzalloc(size, GFP_KERNEL);
	if (trans == NULL)
		return NULL;

	INIT_LIST_HEAD(&trans->list);
	trans->msg_type = msg_type;

	trans->net = ctx->net;
	trans->table = ctx->table;
	trans->seq = ctx->seq;
	trans->flags = ctx->flags;
	trans->report = ctx->report;

	return trans;
}

static struct nft_trans_binding *nft_trans_get_binding(struct nft_trans *trans)
{
	switch (trans->msg_type) {
	case NFT_MSG_NEWCHAIN:
	case NFT_MSG_NEWSET:
		return container_of(trans, struct nft_trans_binding, nft_trans);
	}

	return NULL;
}

static void nft_trans_list_del(struct nft_trans *trans)
{
	struct nft_trans_binding *trans_binding;

	list_del(&trans->list);

	trans_binding = nft_trans_get_binding(trans);
	if (trans_binding)
		list_del(&trans_binding->binding_list);
}

static void nft_trans_destroy(struct nft_trans *trans)
{
	nft_trans_list_del(trans);
	kfree(trans);
}

static void __nft_set_trans_bind(const struct nft_ctx *ctx, struct nft_set *set,
				 bool bind)
{
	struct nftables_pernet *nft_net;
	struct net *net = ctx->net;
	struct nft_trans *trans;

	if (!nft_set_is_anonymous(set))
		return;

	nft_net = nft_pernet(net);
	list_for_each_entry_reverse(trans, &nft_net->commit_list, list) {
		switch (trans->msg_type) {
		case NFT_MSG_NEWSET:
			if (nft_trans_set(trans) == set)
				nft_trans_set_bound(trans) = bind;
			break;
		case NFT_MSG_NEWSETELEM:
			if (nft_trans_elem_set(trans) == set)
				nft_trans_elem_set_bound(trans) = bind;
			break;
		}
	}
}

static void nft_set_trans_bind(const struct nft_ctx *ctx, struct nft_set *set)
{
	return __nft_set_trans_bind(ctx, set, true);
}

static void nft_set_trans_unbind(const struct nft_ctx *ctx, struct nft_set *set)
{
	return __nft_set_trans_bind(ctx, set, false);
}

static void __nft_chain_trans_bind(const struct nft_ctx *ctx,
				   struct nft_chain *chain, bool bind)
{
	struct nftables_pernet *nft_net;
	struct net *net = ctx->net;
	struct nft_trans *trans;

	if (!nft_chain_binding(chain))
		return;

	nft_net = nft_pernet(net);
	list_for_each_entry_reverse(trans, &nft_net->commit_list, list) {
		switch (trans->msg_type) {
		case NFT_MSG_NEWCHAIN:
			if (nft_trans_chain(trans) == chain)
				nft_trans_chain_bound(trans) = bind;
			break;
		case NFT_MSG_NEWRULE:
			if (nft_trans_rule_chain(trans) == chain)
				nft_trans_rule_bound(trans) = bind;
			break;
		}
	}
}

static void nft_chain_trans_bind(const struct nft_ctx *ctx,
				 struct nft_chain *chain)
{
	__nft_chain_trans_bind(ctx, chain, true);
}

int nf_tables_bind_chain(const struct nft_ctx *ctx, struct nft_chain *chain)
{
	if (!nft_chain_binding(chain))
		return 0;

	if (nft_chain_binding(ctx->chain))
		return -EOPNOTSUPP;

	if (chain->bound)
		return -EBUSY;

	if (!nft_use_inc(&chain->use))
		return -EMFILE;

	chain->bound = true;
	nft_chain_trans_bind(ctx, chain);

	return 0;
}

void nf_tables_unbind_chain(const struct nft_ctx *ctx, struct nft_chain *chain)
{
	__nft_chain_trans_bind(ctx, chain, false);
}

static int nft_netdev_register_hooks(struct net *net,
				     struct list_head *hook_list)
{
	struct nf_hook_ops *ops;
	struct nft_hook *hook;
	int err, j;

	j = 0;
	list_for_each_entry(hook, hook_list, list) {
		list_for_each_entry(ops, &hook->ops_list, list) {
			err = nf_register_net_hook(net, ops);
			if (err < 0)
				goto err_register;

			j++;
		}
	}
	return 0;

err_register:
	list_for_each_entry(hook, hook_list, list) {
		list_for_each_entry(ops, &hook->ops_list, list) {
			if (j-- <= 0)
				break;

			nf_unregister_net_hook(net, ops);
		}
	}
	return err;
}

static void nft_netdev_hook_free_ops(struct nft_hook *hook)
{
	struct nf_hook_ops *ops, *next;

	list_for_each_entry_safe(ops, next, &hook->ops_list, list) {
		list_del(&ops->list);
		kfree(ops);
	}
}

static void nft_netdev_hook_free(struct nft_hook *hook)
{
	nft_netdev_hook_free_ops(hook);
	kfree(hook);
}

static void __nft_netdev_hook_free_rcu(struct rcu_head *rcu)
{
	struct nft_hook *hook = container_of(rcu, struct nft_hook, rcu);

	nft_netdev_hook_free(hook);
}

static void nft_netdev_hook_free_rcu(struct nft_hook *hook)
{
	call_rcu(&hook->rcu, __nft_netdev_hook_free_rcu);
}

static void nft_netdev_hook_unlink_free_rcu(struct nft_hook *hook)
{
	list_del_rcu(&hook->list);
	nft_netdev_hook_free_rcu(hook);
}

static void nft_trans_hook_destroy(struct nft_trans_hook *trans_hook)
{
	list_del(&trans_hook->list);
	kfree(trans_hook);
}

static void nft_netdev_unregister_trans_hook(struct net *net,
					     const struct nft_table *table,
					     struct list_head *hook_list)
{
	struct nft_trans_hook *trans_hook, *next;
	struct nf_hook_ops *ops;
	struct nft_hook *hook;

	list_for_each_entry_safe(trans_hook, next, hook_list, list) {
		hook = trans_hook->hook;

		if (!(table->flags & NFT_TABLE_F_DORMANT)) {
			list_for_each_entry(ops, &hook->ops_list, list)
				nf_unregister_net_hook(net, ops);
		}
		nft_netdev_hook_unlink_free_rcu(hook);
		nft_trans_hook_destroy(trans_hook);
	}
}

static void nft_netdev_unregister_hooks(struct net *net,
					const struct nft_table *table,
					struct list_head *hook_list,
					bool release_netdev)
{
	struct nft_hook *hook, *next;
	struct nf_hook_ops *ops;

	list_for_each_entry_safe(hook, next, hook_list, list) {
		if (!(table->flags & NFT_TABLE_F_DORMANT)) {
			list_for_each_entry(ops, &hook->ops_list, list)
				nf_unregister_net_hook(net, ops);
		}
		if (release_netdev)
			nft_netdev_hook_unlink_free_rcu(hook);
	}
}

static int nf_tables_register_hook(struct net *net,
				   const struct nft_table *table,
				   struct nft_chain *chain)
{
	struct nft_base_chain *basechain;
	const struct nf_hook_ops *ops;

	if (table->flags & NFT_TABLE_F_DORMANT ||
	    !nft_is_base_chain(chain))
		return 0;

	basechain = nft_base_chain(chain);
	ops = &basechain->ops;

	if (basechain->type->ops_register)
		return basechain->type->ops_register(net, ops);

	if (nft_base_chain_netdev(table->family, basechain->ops.hooknum))
		return nft_netdev_register_hooks(net, &basechain->hook_list);

	return nf_register_net_hook(net, &basechain->ops);
}

static void __nf_tables_unregister_hook(struct net *net,
					const struct nft_table *table,
					struct nft_chain *chain,
					bool release_netdev)
{
	struct nft_base_chain *basechain;
	const struct nf_hook_ops *ops;

	if (!nft_is_base_chain(chain))
		return;
	basechain = nft_base_chain(chain);
	ops = &basechain->ops;

	/* must also be called for dormant tables */
	if (nft_base_chain_netdev(table->family, basechain->ops.hooknum)) {
		nft_netdev_unregister_hooks(net, table, &basechain->hook_list,
					    release_netdev);
		return;
	}

	if (table->flags & NFT_TABLE_F_DORMANT)
		return;

	if (basechain->type->ops_unregister)
		return basechain->type->ops_unregister(net, ops);

	nf_unregister_net_hook(net, &basechain->ops);
}

static void nf_tables_unregister_hook(struct net *net,
				      const struct nft_table *table,
				      struct nft_chain *chain)
{
	return __nf_tables_unregister_hook(net, table, chain, false);
}

static bool nft_trans_collapse_set_elem_allowed(const struct nft_trans_elem *a, const struct nft_trans_elem *b)
{
	/* NB: the ->bound equality check is defensive, at this time we only merge
	 * a new nft_trans_elem transaction request with the transaction tail
	 * element, but a->bound != b->bound would imply a NEWRULE transaction
	 * is queued in-between.
	 *
	 * The set check is mandatory, the NFT_MAX_SET_NELEMS check prevents
	 * huge krealloc() requests.
	 */
	return a->set == b->set && a->bound == b->bound && a->nelems < NFT_MAX_SET_NELEMS;
}

static bool nft_trans_collapse_set_elem(struct nftables_pernet *nft_net,
					struct nft_trans_elem *tail,
					struct nft_trans_elem *trans)
{
	unsigned int nelems, old_nelems = tail->nelems;
	struct nft_trans_elem *new_trans;

	if (!nft_trans_collapse_set_elem_allowed(tail, trans))
		return false;

	/* "cannot happen", at this time userspace element add
	 * requests always allocate a new transaction element.
	 *
	 * This serves as a reminder to adjust the list_add_tail
	 * logic below in case this ever changes.
	 */
	if (WARN_ON_ONCE(trans->nelems != 1))
		return false;

	if (check_add_overflow(old_nelems, trans->nelems, &nelems))
		return false;

	/* krealloc might free tail which invalidates list pointers */
	list_del_init(&tail->nft_trans.list);

	new_trans = krealloc(tail, struct_size(tail, elems, nelems),
			     GFP_KERNEL);
	if (!new_trans) {
		list_add_tail(&tail->nft_trans.list,
			      &nft_net->commit_list);
		return false;
	}

	/*
	 * new_trans->nft_trans.list contains garbage, but
	 * list_add_tail() doesn't care.
	 */
	new_trans->nelems = nelems;
	new_trans->elems[old_nelems] = trans->elems[0];
	list_add_tail(&new_trans->nft_trans.list, &nft_net->commit_list);

	return true;
}

static bool nft_trans_try_collapse(struct nftables_pernet *nft_net,
				   struct nft_trans *trans)
{
	struct nft_trans *tail;

	if (list_empty(&nft_net->commit_list))
		return false;

	tail = list_last_entry(&nft_net->commit_list, struct nft_trans, list);

	if (tail->msg_type != trans->msg_type)
		return false;

	switch (trans->msg_type) {
	case NFT_MSG_NEWSETELEM:
	case NFT_MSG_DELSETELEM:
		return nft_trans_collapse_set_elem(nft_net,
						   nft_trans_container_elem(tail),
						   nft_trans_container_elem(trans));
	}

	return false;
}

static void nft_trans_commit_list_add_tail(struct net *net, struct nft_trans *trans)
{
	struct nftables_pernet *nft_net = nft_pernet(net);
	struct nft_trans_binding *binding;
	struct nft_trans_set *trans_set;

	list_add_tail(&trans->list, &nft_net->commit_list);

	binding = nft_trans_get_binding(trans);
	if (!binding)
		return;

	switch (trans->msg_type) {
	case NFT_MSG_NEWSET:
		trans_set = nft_trans_container_set(trans);

		if (!nft_trans_set_update(trans) &&
		    nft_set_is_anonymous(nft_trans_set(trans)))
			list_add_tail(&binding->binding_list, &nft_net->binding_list);

		list_add_tail(&trans_set->list_trans_newset, &nft_net->commit_set_list);
		break;
	case NFT_MSG_NEWCHAIN:
		if (!nft_trans_chain_update(trans) &&
		    nft_chain_binding(nft_trans_chain(trans)))
			list_add_tail(&binding->binding_list, &nft_net->binding_list);
		break;
	}
}

static void nft_trans_commit_list_add_elem(struct net *net, struct nft_trans *trans)
{
	struct nftables_pernet *nft_net = nft_pernet(net);
	struct nft_trans_elem *te;

	WARN_ON_ONCE(trans->msg_type != NFT_MSG_NEWSETELEM &&
		     trans->msg_type != NFT_MSG_DELSETELEM);

	te = nft_trans_container_elem(trans);
	if (te->set->ops->commit && list_empty(&te->set->pending_update))
		list_add_tail(&te->set->pending_update, &nft_net->set_update_list);

	if (nft_trans_try_collapse(nft_net, trans)) {
		kfree(trans);
		return;
	}

	nft_trans_commit_list_add_tail(net, trans);
}

static int nft_trans_table_add(struct nft_ctx *ctx, int msg_type)
{
	struct nft_trans *trans;

	trans = nft_trans_alloc(ctx, msg_type, sizeof(struct nft_trans_table));
	if (trans == NULL)
		return -ENOMEM;

	if (msg_type == NFT_MSG_NEWTABLE)
		nft_activate_next(ctx->net, ctx->table);

	nft_trans_commit_list_add_tail(ctx->net, trans);
	return 0;
}

static int nft_deltable(struct nft_ctx *ctx)
{
	int err;

	err = nft_trans_table_add(ctx, NFT_MSG_DELTABLE);
	if (err < 0)
		return err;

	nft_deactivate_next(ctx->net, ctx->table);
	return err;
}

static struct nft_trans *
nft_trans_alloc_chain(const struct nft_ctx *ctx, int msg_type)
{
	struct nft_trans_chain *trans_chain;
	struct nft_trans *trans;

	trans = nft_trans_alloc(ctx, msg_type, sizeof(struct nft_trans_chain));
	if (!trans)
		return NULL;

	trans_chain = nft_trans_container_chain(trans);
	INIT_LIST_HEAD(&trans_chain->nft_trans_binding.binding_list);
	trans_chain->chain = ctx->chain;

	return trans;
}

static struct nft_trans *nft_trans_chain_add(struct nft_ctx *ctx, int msg_type)
{
	struct nft_trans *trans;

	trans = nft_trans_alloc_chain(ctx, msg_type);
	if (trans == NULL)
		return ERR_PTR(-ENOMEM);

	if (msg_type == NFT_MSG_NEWCHAIN) {
		nft_activate_next(ctx->net, ctx->chain);

		if (ctx->nla[NFTA_CHAIN_ID]) {
			nft_trans_chain_id(trans) =
				ntohl(nla_get_be32(ctx->nla[NFTA_CHAIN_ID]));
		}
	}
	nft_trans_commit_list_add_tail(ctx->net, trans);

	return trans;
}

static int nft_delchain(struct nft_ctx *ctx)
{
	struct nft_trans *trans;

	trans = nft_trans_chain_add(ctx, NFT_MSG_DELCHAIN);
	if (IS_ERR(trans))
		return PTR_ERR(trans);

	nft_use_dec(&ctx->table->use);
	nft_deactivate_next(ctx->net, ctx->chain);

	return 0;
}

void nft_rule_expr_activate(const struct nft_ctx *ctx, struct nft_rule *rule)
{
	struct nft_expr *expr;

	expr = nft_expr_first(rule);
	while (nft_expr_more(rule, expr)) {
		if (expr->ops->activate)
			expr->ops->activate(ctx, expr);

		expr = nft_expr_next(expr);
	}
}

void nft_rule_expr_deactivate(const struct nft_ctx *ctx, struct nft_rule *rule,
			      enum nft_trans_phase phase)
{
	struct nft_expr *expr;

	expr = nft_expr_first(rule);
	while (nft_expr_more(rule, expr)) {
		if (expr->ops->deactivate)
			expr->ops->deactivate(ctx, expr, phase);

		expr = nft_expr_next(expr);
	}
}

static int
nf_tables_delrule_deactivate(struct nft_ctx *ctx, struct nft_rule *rule)
{
	/* You cannot delete the same rule twice */
	if (nft_is_active_next(ctx->net, rule)) {
		nft_deactivate_next(ctx->net, rule);
		nft_use_dec(&ctx->chain->use);
		return 0;
	}
	return -ENOENT;
}

static struct nft_trans *nft_trans_rule_add(struct nft_ctx *ctx, int msg_type,
					    struct nft_rule *rule)
{
	struct nft_trans *trans;

	trans = nft_trans_alloc(ctx, msg_type, sizeof(struct nft_trans_rule));
	if (trans == NULL)
		return NULL;

	if (msg_type == NFT_MSG_NEWRULE && ctx->nla[NFTA_RULE_ID] != NULL) {
		nft_trans_rule_id(trans) =
			ntohl(nla_get_be32(ctx->nla[NFTA_RULE_ID]));
	}
	nft_trans_rule(trans) = rule;
	nft_trans_rule_chain(trans) = ctx->chain;
	nft_trans_commit_list_add_tail(ctx->net, trans);

	return trans;
}

static int nft_delrule(struct nft_ctx *ctx, struct nft_rule *rule)
{
	struct nft_flow_rule *flow;
	struct nft_trans *trans;
	int err;

	trans = nft_trans_rule_add(ctx, NFT_MSG_DELRULE, rule);
	if (trans == NULL)
		return -ENOMEM;

	if (ctx->chain->flags & NFT_CHAIN_HW_OFFLOAD) {
		flow = nft_flow_rule_create(ctx->net, rule);
		if (IS_ERR(flow)) {
			nft_trans_destroy(trans);
			return PTR_ERR(flow);
		}

		nft_trans_flow_rule(trans) = flow;
	}

	err = nf_tables_delrule_deactivate(ctx, rule);
	if (err < 0) {
		nft_trans_destroy(trans);
		return err;
	}
	nft_rule_expr_deactivate(ctx, rule, NFT_TRANS_PREPARE);

	return 0;
}

static int nft_delrule_by_chain(struct nft_ctx *ctx)
{
	struct nft_rule *rule;
	int err;

	list_for_each_entry(rule, &ctx->chain->rules, list) {
		if (!nft_is_active_next(ctx->net, rule))
			continue;

		err = nft_delrule(ctx, rule);
		if (err < 0)
			return err;
	}
	return 0;
}

static int __nft_trans_set_add(const struct nft_ctx *ctx, int msg_type,
			       struct nft_set *set,
			       const struct nft_set_desc *desc)
{
	struct nft_trans_set *trans_set;
	struct nft_trans *trans;

	trans = nft_trans_alloc(ctx, msg_type, sizeof(struct nft_trans_set));
	if (trans == NULL)
		return -ENOMEM;

	trans_set = nft_trans_container_set(trans);
	INIT_LIST_HEAD(&trans_set->nft_trans_binding.binding_list);
	INIT_LIST_HEAD(&trans_set->list_trans_newset);

	if (msg_type == NFT_MSG_NEWSET && ctx->nla[NFTA_SET_ID] && !desc) {
		nft_trans_set_id(trans) =
			ntohl(nla_get_be32(ctx->nla[NFTA_SET_ID]));
		nft_activate_next(ctx->net, set);
	}
	nft_trans_set(trans) = set;
	if (desc) {
		nft_trans_set_update(trans) = true;
		nft_trans_set_gc_int(trans) = desc->gc_int;
		nft_trans_set_timeout(trans) = desc->timeout;
		nft_trans_set_size(trans) = desc->size;
	}
	nft_trans_commit_list_add_tail(ctx->net, trans);

	return 0;
}

static int nft_trans_set_add(const struct nft_ctx *ctx, int msg_type,
			     struct nft_set *set)
{
	return __nft_trans_set_add(ctx, msg_type, set, NULL);
}

static int nft_mapelem_deactivate(const struct nft_ctx *ctx,
				  struct nft_set *set,
				  const struct nft_set_iter *iter,
				  struct nft_elem_priv *elem_priv)
{
	struct nft_set_ext *ext = nft_set_elem_ext(set, elem_priv);

	if (!nft_set_elem_active(ext, iter->genmask))
		return 0;

	nft_set_elem_change_active(ctx->net, set, ext);
	nft_setelem_data_deactivate(ctx->net, set, elem_priv);

	return 0;
}

struct nft_set_elem_catchall {
	struct list_head	list;
	struct rcu_head		rcu;
	struct nft_elem_priv	*elem;
};

static void nft_map_catchall_deactivate(const struct nft_ctx *ctx,
					struct nft_set *set)
{
	u8 genmask = nft_genmask_next(ctx->net);
	struct nft_set_elem_catchall *catchall;
	struct nft_set_ext *ext;

	list_for_each_entry(catchall, &set->catchall_list, list) {
		ext = nft_set_elem_ext(set, catchall->elem);
		if (!nft_set_elem_active(ext, genmask))
			continue;

		nft_set_elem_change_active(ctx->net, set, ext);
		nft_setelem_data_deactivate(ctx->net, set, catchall->elem);
	}
}

/* Use NFT_ITER_UPDATE iterator even if this may be called from the preparation
 * phase, the set clone might already exist from a previous command, or it might
 * be a set that is going away and does not require a clone. The netns and
 * netlink release paths also need to work on the live set.
 */
static void nft_map_deactivate(const struct nft_ctx *ctx, struct nft_set *set)
{
	struct nft_set_iter iter = {
		.genmask	= nft_genmask_next(ctx->net),
		.type		= NFT_ITER_UPDATE,
		.fn		= nft_mapelem_deactivate,
	};

	set->ops->walk(ctx, set, &iter);
	WARN_ON_ONCE(iter.err);

	nft_map_catchall_deactivate(ctx, set);
}

static int nft_delset(const struct nft_ctx *ctx, struct nft_set *set)
{
	int err;

	err = nft_trans_set_add(ctx, NFT_MSG_DELSET, set);
	if (err < 0)
		return err;

	if (set->flags & (NFT_SET_MAP | NFT_SET_OBJECT))
		nft_map_deactivate(ctx, set);

	nft_deactivate_next(ctx->net, set);
	nft_use_dec(&ctx->table->use);

	return err;
}

static int nft_trans_obj_add(struct nft_ctx *ctx, int msg_type,
			     struct nft_object *obj)
{
	struct nft_trans *trans;

	trans = nft_trans_alloc(ctx, msg_type, sizeof(struct nft_trans_obj));
	if (trans == NULL)
		return -ENOMEM;

	if (msg_type == NFT_MSG_NEWOBJ)
		nft_activate_next(ctx->net, obj);

	nft_trans_obj(trans) = obj;
	nft_trans_commit_list_add_tail(ctx->net, trans);

	return 0;
}

static int nft_delobj(struct nft_ctx *ctx, struct nft_object *obj)
{
	int err;

	err = nft_trans_obj_add(ctx, NFT_MSG_DELOBJ, obj);
	if (err < 0)
		return err;

	nft_deactivate_next(ctx->net, obj);
	nft_use_dec(&ctx->table->use);

	return err;
}

static struct nft_trans *
nft_trans_flowtable_add(struct nft_ctx *ctx, int msg_type,
		        struct nft_flowtable *flowtable)
{
	struct nft_trans *trans;

	trans = nft_trans_alloc(ctx, msg_type,
				sizeof(struct nft_trans_flowtable));
	if (trans == NULL)
		return ERR_PTR(-ENOMEM);

	if (msg_type == NFT_MSG_NEWFLOWTABLE)
		nft_activate_next(ctx->net, flowtable);

	INIT_LIST_HEAD(&nft_trans_flowtable_hooks(trans));
	nft_trans_flowtable(trans) = flowtable;
	nft_trans_commit_list_add_tail(ctx->net, trans);

	return trans;
}

static int nft_delflowtable(struct nft_ctx *ctx,
			    struct nft_flowtable *flowtable)
{
	struct nft_trans *trans;

	trans = nft_trans_flowtable_add(ctx, NFT_MSG_DELFLOWTABLE, flowtable);
	if (IS_ERR(trans))
		return PTR_ERR(trans);

	nft_deactivate_next(ctx->net, flowtable);
	nft_use_dec(&ctx->table->use);

	return 0;
}

/*
 * Tables
 */

static struct nft_table *nft_table_lookup(const struct net *net,
					  const struct nlattr *nla,
					  u8 family, u8 genmask, u32 nlpid)
{
	struct nftables_pernet *nft_net;
	struct nft_table *table;

	if (nla == NULL)
		return ERR_PTR(-EINVAL);

	nft_net = nft_pernet(net);
	list_for_each_entry_rcu(table, &nft_net->tables, list,
				lockdep_is_held(&nft_net->commit_mutex)) {
		if (!nla_strcmp(nla, table->name) &&
		    table->family == family &&
		    nft_active_genmask(table, genmask)) {
			if (nft_table_has_owner(table) &&
			    nlpid && table->nlpid != nlpid)
				return ERR_PTR(-EPERM);

			return table;
		}
	}

	return ERR_PTR(-ENOENT);
}

static struct nft_table *nft_table_lookup_byhandle(const struct net *net,
						   const struct nlattr *nla,
						   int family, u8 genmask, u32 nlpid)
{
	struct nftables_pernet *nft_net;
	struct nft_table *table;

	nft_net = nft_pernet(net);
	list_for_each_entry(table, &nft_net->tables, list) {
		if (be64_to_cpu(nla_get_be64(nla)) == table->handle &&
		    table->family == family &&
		    nft_active_genmask(table, genmask)) {
			if (nft_table_has_owner(table) &&
			    nlpid && table->nlpid != nlpid)
				return ERR_PTR(-EPERM);

			return table;
		}
	}

	return ERR_PTR(-ENOENT);
}

static inline u64 nf_tables_alloc_handle(struct nft_table *table)
{
	return ++table->hgenerator;
}

static const struct nft_chain_type *chain_type[NFPROTO_NUMPROTO][NFT_CHAIN_T_MAX];

static const struct nft_chain_type *
__nft_chain_type_get(u8 family, enum nft_chain_types type)
{
	if (family >= NFPROTO_NUMPROTO ||
	    type >= NFT_CHAIN_T_MAX)
		return NULL;

	return chain_type[family][type];
}

static const struct nft_chain_type *
__nf_tables_chain_type_lookup(const struct nlattr *nla, u8 family)
{
	const struct nft_chain_type *type;
	int i;

	for (i = 0; i < NFT_CHAIN_T_MAX; i++) {
		type = __nft_chain_type_get(family, i);
		if (!type)
			continue;
		if (!nla_strcmp(nla, type->name))
			return type;
	}
	return NULL;
}

struct nft_module_request {
	struct list_head	list;
	char			module[MODULE_NAME_LEN];
	bool			done;
};

#ifdef CONFIG_MODULES
__printf(2, 3) int nft_request_module(struct net *net, const char *fmt,
				      ...)
{
	char module_name[MODULE_NAME_LEN];
	struct nftables_pernet *nft_net;
	struct nft_module_request *req;
	va_list args;
	int ret;

	va_start(args, fmt);
	ret = vsnprintf(module_name, MODULE_NAME_LEN, fmt, args);
	va_end(args);
	if (ret >= MODULE_NAME_LEN)
		return 0;

	nft_net = nft_pernet(net);
	list_for_each_entry(req, &nft_net->module_list, list) {
		if (!strcmp(req->module, module_name)) {
			if (req->done)
				return 0;

			/* A request to load this module already exists. */
			return -EAGAIN;
		}
	}

	req = kmalloc_obj(*req);
	if (!req)
		return -ENOMEM;

	req->done = false;
	strscpy(req->module, module_name, MODULE_NAME_LEN);
	list_add_tail(&req->list, &nft_net->module_list);

	return -EAGAIN;
}
EXPORT_SYMBOL_GPL(nft_request_module);
#endif

static void lockdep_nfnl_nft_mutex_not_held(void)
{
#ifdef CONFIG_PROVE_LOCKING
	if (debug_locks)
		WARN_ON_ONCE(lockdep_nfnl_is_held(NFNL_SUBSYS_NFTABLES));
#endif
}

static const struct nft_chain_type *
nf_tables_chain_type_lookup(struct net *net, const struct nlattr *nla,
			    u8 family, bool autoload)
{
	const struct nft_chain_type *type;

	type = __nf_tables_chain_type_lookup(nla, family);
	if (type != NULL)
		return type;

	lockdep_nfnl_nft_mutex_not_held();
#ifdef CONFIG_MODULES
	if (autoload) {
		if (nft_request_module(net, "nft-chain-%u-%.*s", family,
				       nla_len(nla),
				       (const char *)nla_data(nla)) == -EAGAIN)
			return ERR_PTR(-EAGAIN);
	}
#endif
	return ERR_PTR(-ENOENT);
}

static unsigned int nft_base_seq(const struct net *net)
{
	return READ_ONCE(net->nft.base_seq);
}

static __be16 nft_base_seq_be16(const struct net *net)
{
	return htons(nft_base_seq(net) & 0xffff);
}

static const struct nla_policy nft_table_policy[NFTA_TABLE_MAX + 1] = {
	[NFTA_TABLE_NAME]	= { .type = NLA_STRING,
				    .len = NFT_TABLE_MAXNAMELEN - 1 },
	[NFTA_TABLE_FLAGS]	= NLA_POLICY_MASK(NLA_BE32, NFT_TABLE_F_MASK),
	[NFTA_TABLE_HANDLE]	= { .type = NLA_U64 },
	[NFTA_TABLE_USERDATA]	= { .type = NLA_BINARY,
				    .len = NFT_USERDATA_MAXLEN }
};

static int nf_tables_fill_table_info(struct sk_buff *skb, struct net *net,
				     u32 portid, u32 seq, int event, u32 flags,
				     int family, const struct nft_table *table)
{
	struct nlmsghdr *nlh;

	nlh = nfnl_msg_put(skb, portid, seq,
			   nfnl_msg_type(NFNL_SUBSYS_NFTABLES, event),
			   flags, family, NFNETLINK_V0, nft_base_seq_be16(net));
	if (!nlh)
		goto nla_put_failure;

	if (nla_put_string(skb, NFTA_TABLE_NAME, table->name) ||
	    nla_put_be32(skb, NFTA_TABLE_USE, htonl(table->use)) ||
	    nla_put_be64(skb, NFTA_TABLE_HANDLE, cpu_to_be64(table->handle),
			 NFTA_TABLE_PAD))
		goto nla_put_failure;

	if (event == NFT_MSG_DELTABLE ||
	    event == NFT_MSG_DESTROYTABLE) {
		nlmsg_end(skb, nlh);
		return 0;
	}

	if (nla_put_be32(skb, NFTA_TABLE_FLAGS,
			 htonl(table->flags & NFT_TABLE_F_MASK)))
		goto nla_put_failure;

	if (nft_table_has_owner(table) &&
	    nla_put_be32(skb, NFTA_TABLE_OWNER, htonl(table->nlpid)))
		goto nla_put_failure;

	if (table->udata) {
		if (nla_put(skb, NFTA_TABLE_USERDATA, table->udlen, table->udata))
			goto nla_put_failure;
	}

	nlmsg_end(skb, nlh);
	return 0;

nla_put_failure:
	nlmsg_trim(skb, nlh);
	return -1;
}

struct nftnl_skb_parms {
	bool report;
};
#define NFT_CB(skb)	(*(struct nftnl_skb_parms*)&((skb)->cb))

static void nft_notify_enqueue(struct sk_buff *skb, bool report,
			       struct list_head *notify_list)
{
	NFT_CB(skb).report = report;
	list_add_tail(&skb->list, notify_list);
}

static void nf_tables_table_notify(const struct nft_ctx *ctx, int event)
{
	struct nftables_pernet *nft_net;
	struct sk_buff *skb;
	u16 flags = 0;
	int err;

	if (!ctx->report &&
	    !nfnetlink_has_listeners(ctx->net, NFNLGRP_NFTABLES))
		return;

	skb = nlmsg_new(NLMSG_GOODSIZE, GFP_KERNEL);
	if (skb == NULL)
		goto err;

	if (ctx->flags & (NLM_F_CREATE | NLM_F_EXCL))
		flags |= ctx->flags & (NLM_F_CREATE | NLM_F_EXCL);

	err = nf_tables_fill_table_info(skb, ctx->net, ctx->portid, ctx->seq,
					event, flags, ctx->family, ctx->table);
	if (err < 0) {
		kfree_skb(skb);
		goto err;
	}

	nft_net = nft_pernet(ctx->net);
	nft_notify_enqueue(skb, ctx->report, &nft_net->notify_list);
	return;
err:
	nfnetlink_set_err(ctx->net, ctx->portid, NFNLGRP_NFTABLES, -ENOBUFS);
}

static int nf_tables_dump_tables(struct sk_buff *skb,
				 struct netlink_callback *cb)
{
	const struct nfgenmsg *nfmsg = nlmsg_data(cb->nlh);
	struct nftables_pernet *nft_net;
	const struct nft_table *table;
	unsigned int idx = 0, s_idx = cb->args[0];
	struct net *net = sock_net(skb->sk);
	int family = nfmsg->nfgen_family;

	rcu_read_lock();
	nft_net = nft_pernet(net);
	cb->seq = nft_base_seq(net);

	list_for_each_entry_rcu(table, &nft_net->tables, list) {
		if (family != NFPROTO_UNSPEC && family != table->family)
			continue;

		if (idx < s_idx)
			goto cont;
		if (idx > s_idx)
			memset(&cb->args[1], 0,
			       sizeof(cb->args) - sizeof(cb->args[0]));
		if (!nft_is_active(net, table))
			continue;
		if (nf_tables_fill_table_info(skb, net,
					      NETLINK_CB(cb->skb).portid,
					      cb->nlh->nlmsg_seq,
					      NFT_MSG_NEWTABLE, NLM_F_MULTI,
					      table->family, table) < 0)
			goto done;

		nl_dump_check_consistent(cb, nlmsg_hdr(skb));
cont:
		idx++;
	}
done:
	rcu_read_unlock();
	cb->args[0] = idx;
	return skb->len;
}

static int nft_netlink_dump_start_rcu(struct sock *nlsk, struct sk_buff *skb,
				      const struct nlmsghdr *nlh,
				      struct netlink_dump_control *c)
{
	int err;

	if (!try_module_get(THIS_MODULE))
		return -EINVAL;

	rcu_read_unlock();
	err = netlink_dump_start(nlsk, skb, nlh, c);
	rcu_read_lock();
	module_put(THIS_MODULE);

	return err;
}

/* called with rcu_read_lock held */
static int nf_tables_gettable(struct sk_buff *skb, const struct nfnl_info *info,
			      const struct nlattr * const nla[])
{
	struct netlink_ext_ack *extack = info->extack;
	u8 genmask = nft_genmask_cur(info->net);
	u8 family = info->nfmsg->nfgen_family;
	const struct nft_table *table;
	struct net *net = info->net;
	struct sk_buff *skb2;
	int err;

	if (info->nlh->nlmsg_flags & NLM_F_DUMP) {
		struct netlink_dump_control c = {
			.dump = nf_tables_dump_tables,
			.module = THIS_MODULE,
		};

		return nft_netlink_dump_start_rcu(info->sk, skb, info->nlh, &c);
	}

	table = nft_table_lookup(net, nla[NFTA_TABLE_NAME], family, genmask, 0);
	if (IS_ERR(table)) {
		NL_SET_BAD_ATTR(extack, nla[NFTA_TABLE_NAME]);
		return PTR_ERR(table);
	}

	skb2 = alloc_skb(NLMSG_GOODSIZE, GFP_ATOMIC);
	if (!skb2)
		return -ENOMEM;

	err = nf_tables_fill_table_info(skb2, net, NETLINK_CB(skb).portid,
					info->nlh->nlmsg_seq, NFT_MSG_NEWTABLE,
					0, family, table);
	if (err < 0)
		goto err_fill_table_info;

	return nfnetlink_unicast(skb2, net, NETLINK_CB(skb).portid);

err_fill_table_info:
	kfree_skb(skb2);
	return err;
}

static void nft_table_disable(struct net *net, struct nft_table *table, u32 cnt)
{
	struct nft_chain *chain;
	u32 i = 0;

	list_for_each_entry(chain, &table->chains, list) {
		if (!nft_is_active_next(net, chain))
			continue;
		if (!nft_is_base_chain(chain))
			continue;

		if (cnt && i++ == cnt)
			break;

		nf_tables_unregister_hook(net, table, chain);
	}
}

static int nf_tables_table_enable(struct net *net, struct nft_table *table)
{
	struct nft_chain *chain;
	int err, i = 0;

	list_for_each_entry(chain, &table->chains, list) {
		if (!nft_is_active_next(net, chain))
			continue;
		if (!nft_is_base_chain(chain))
			continue;

		err = nf_tables_register_hook(net, table, chain);
		if (err < 0)
			goto err_register_hooks;

		i++;
	}
	return 0;

err_register_hooks:
	if (i)
		nft_table_disable(net, table, i);
	return err;
}

static void nf_tables_table_disable(struct net *net, struct nft_table *table)
{
	table->flags &= ~NFT_TABLE_F_DORMANT;
	nft_table_disable(net, table, 0);
	table->flags |= NFT_TABLE_F_DORMANT;
}

#define __NFT_TABLE_F_INTERNAL		(NFT_TABLE_F_MASK + 1)
#define __NFT_TABLE_F_WAS_DORMANT	(__NFT_TABLE_F_INTERNAL << 0)
#define __NFT_TABLE_F_WAS_AWAKEN	(__NFT_TABLE_F_INTERNAL << 1)
#define __NFT_TABLE_F_WAS_ORPHAN	(__NFT_TABLE_F_INTERNAL << 2)
#define __NFT_TABLE_F_UPDATE		(__NFT_TABLE_F_WAS_DORMANT | \
					 __NFT_TABLE_F_WAS_AWAKEN | \
					 __NFT_TABLE_F_WAS_ORPHAN)

static bool nft_table_pending_update(const struct nft_ctx *ctx)
{
	struct nftables_pernet *nft_net = nft_pernet(ctx->net);
	struct nft_trans *trans;

	if (ctx->table->flags & __NFT_TABLE_F_UPDATE)
		return true;

	list_for_each_entry(trans, &nft_net->commit_list, list) {
		if (trans->table == ctx->table &&
		    ((trans->msg_type == NFT_MSG_NEWCHAIN &&
		      nft_trans_chain_update(trans)) ||
		     (trans->msg_type == NFT_MSG_DELCHAIN &&
		      nft_is_base_chain(nft_trans_chain(trans)))))
			return true;
	}

	return false;
}

static int nf_tables_updtable(struct nft_ctx *ctx)
{
	struct nft_trans *trans;
	u32 flags;
	int ret;

	if (!ctx->nla[NFTA_TABLE_FLAGS])
		return 0;

	flags = ntohl(nla_get_be32(ctx->nla[NFTA_TABLE_FLAGS]));
	if (flags & ~NFT_TABLE_F_MASK)
		return -EOPNOTSUPP;

	if (flags == (ctx->table->flags & NFT_TABLE_F_MASK))
		return 0;

	if ((nft_table_has_owner(ctx->table) &&
	     !(flags & NFT_TABLE_F_OWNER)) ||
	    (flags & NFT_TABLE_F_OWNER &&
	     !nft_table_is_orphan(ctx->table)))
		return -EOPNOTSUPP;

	if ((flags ^ ctx->table->flags) & NFT_TABLE_F_PERSIST)
		return -EOPNOTSUPP;

	/* No dormant off/on/off/on games in single transaction */
	if (nft_table_pending_update(ctx))
		return -EINVAL;

	trans = nft_trans_alloc(ctx, NFT_MSG_NEWTABLE,
				sizeof(struct nft_trans_table));
	if (trans == NULL)
		return -ENOMEM;

	if ((flags & NFT_TABLE_F_DORMANT) &&
	    !(ctx->table->flags & NFT_TABLE_F_DORMANT)) {
		ctx->table->flags |= NFT_TABLE_F_DORMANT;
		if (!(ctx->table->flags & __NFT_TABLE_F_UPDATE))
			ctx->table->flags |= __NFT_TABLE_F_WAS_AWAKEN;
	} else if (!(flags & NFT_TABLE_F_DORMANT) &&
		   ctx->table->flags & NFT_TABLE_F_DORMANT) {
		ctx->table->flags &= ~NFT_TABLE_F_DORMANT;
		if (!(ctx->table->flags & __NFT_TABLE_F_UPDATE)) {
			ret = nf_tables_table_enable(ctx->net, ctx->table);
			if (ret < 0)
				goto err_register_hooks;

			ctx->table->flags |= __NFT_TABLE_F_WAS_DORMANT;
		}
	}

	if ((flags & NFT_TABLE_F_OWNER) &&
	    !nft_table_has_owner(ctx->table)) {
		ctx->table->nlpid = ctx->portid;
		ctx->table->flags |= NFT_TABLE_F_OWNER |
				     __NFT_TABLE_F_WAS_ORPHAN;
	}

	nft_trans_table_update(trans) = true;
	nft_trans_commit_list_add_tail(ctx->net, trans);

	return 0;

err_register_hooks:
	ctx->table->flags |= NFT_TABLE_F_DORMANT;
	nft_trans_destroy(trans);
	return ret;
}

static u32 nft_chain_hash(const void *data, u32 len, u32 seed)
{
	const char *name = data;

	return jhash(name, strlen(name), seed);
}

static u32 nft_chain_hash_obj(const void *data, u32 len, u32 seed)
{
	const struct nft_chain *chain = data;

	return nft_chain_hash(chain->name, 0, seed);
}

static int nft_chain_hash_cmp(struct rhashtable_compare_arg *arg,
			      const void *ptr)
{
	const struct nft_chain *chain = ptr;
	const char *name = arg->key;

	return strcmp(chain->name, name);
}

static u32 nft_objname_hash(const void *data, u32 len, u32 seed)
{
	const struct nft_object_hash_key *k = data;

	seed ^= hash_ptr(k->table, 32);

	return jhash(k->name, strlen(k->name), seed);
}

static u32 nft_objname_hash_obj(const void *data, u32 len, u32 seed)
{
	const struct nft_object *obj = data;

	return nft_objname_hash(&obj->key, 0, seed);
}

static int nft_objname_hash_cmp(struct rhashtable_compare_arg *arg,
				const void *ptr)
{
	const struct nft_object_hash_key *k = arg->key;
	const struct nft_object *obj = ptr;

	if (obj->key.table != k->table)
		return -1;

	return strcmp(obj->key.name, k->name);
}

static bool nft_supported_family(u8 family)
{
	return false
#ifdef CONFIG_NF_TABLES_INET
		|| family == NFPROTO_INET
#endif
#ifdef CONFIG_NF_TABLES_IPV4
		|| family == NFPROTO_IPV4
#endif
#ifdef CONFIG_NF_TABLES_ARP
		|| family == NFPROTO_ARP
#endif
#ifdef CONFIG_NF_TABLES_NETDEV
		|| family == NFPROTO_NETDEV
#endif
#if IS_ENABLED(CONFIG_NF_TABLES_BRIDGE)
		|| family == NFPROTO_BRIDGE
#endif
#ifdef CONFIG_NF_TABLES_IPV6
		|| family == NFPROTO_IPV6
#endif
		;
}

static int nf_tables_newtable(struct sk_buff *skb, const struct nfnl_info *info,
			      const struct nlattr * const nla[])
{
	struct nftables_pernet *nft_net = nft_pernet(info->net);
	struct netlink_ext_ack *extack = info->extack;
	u8 genmask = nft_genmask_next(info->net);
	u8 family = info->nfmsg->nfgen_family;
	struct net *net = info->net;
	const struct nlattr *attr;
	struct nft_table *table;
	struct nft_ctx ctx;
	u32 flags = 0;
	int err;

	if (!nft_supported_family(family))
		return -EOPNOTSUPP;

	lockdep_assert_held(&nft_net->commit_mutex);
	attr = nla[NFTA_TABLE_NAME];
	table = nft_table_lookup(net, attr, family, genmask,
				 NETLINK_CB(skb).portid);
	if (IS_ERR(table)) {
		if (PTR_ERR(table) != -ENOENT)
			return PTR_ERR(table);
	} else {
		if (info->nlh->nlmsg_flags & NLM_F_EXCL) {
			NL_SET_BAD_ATTR(extack, attr);
			return -EEXIST;
		}
		if (info->nlh->nlmsg_flags & NLM_F_REPLACE)
			return -EOPNOTSUPP;

		nft_ctx_init(&ctx, net, skb, info->nlh, family, table, NULL, nla);

		return nf_tables_updtable(&ctx);
	}

	if (nla[NFTA_TABLE_FLAGS]) {
		flags = ntohl(nla_get_be32(nla[NFTA_TABLE_FLAGS]));
		if (flags & ~NFT_TABLE_F_MASK)
			return -EOPNOTSUPP;
	}

	err = -ENOMEM;
	table = kzalloc_obj(*table, GFP_KERNEL_ACCOUNT);
	if (table == NULL)
		goto err_kzalloc;

	table->validate_state = nft_net->validate_state;
	table->name = nla_strdup(attr, GFP_KERNEL_ACCOUNT);
	if (table->name == NULL)
		goto err_strdup;

	if (nla[NFTA_TABLE_USERDATA]) {
		table->udata = nla_memdup(nla[NFTA_TABLE_USERDATA], GFP_KERNEL_ACCOUNT);
		if (table->udata == NULL)
			goto err_table_udata;

		table->udlen = nla_len(nla[NFTA_TABLE_USERDATA]);
	}

	err = rhltable_init(&table->chains_ht, &nft_chain_ht_params);
	if (err)
		goto err_chain_ht;

	err = rhltable_init(&table->objname_ht, &nft_objname_ht_params);
	if (err < 0)
		goto err_obj_ht;

	INIT_LIST_HEAD(&table->chains);
	INIT_LIST_HEAD(&table->sets);
	INIT_LIST_HEAD(&table->objects);
	INIT_LIST_HEAD(&table->flowtables);
	table->family = family;
	table->flags = flags;
	table->handle = ++nft_net->table_handle;
	if (table->flags & NFT_TABLE_F_OWNER)
		table->nlpid = NETLINK_CB(skb).portid;

	nft_ctx_init(&ctx, net, skb, info->nlh, family, table, NULL, nla);
	err = nft_trans_table_add(&ctx, NFT_MSG_NEWTABLE);
	if (err < 0)
		goto err_trans;

	list_add_tail_rcu(&table->list, &nft_net->tables);
	return 0;
err_trans:
	rhltable_destroy(&table->objname_ht);
err_obj_ht:
	rhltable_destroy(&table->chains_ht);
err_chain_ht:
	kfree(table->udata);
err_table_udata:
	kfree(table->name);
err_strdup:
	kfree(table);
err_kzalloc:
	return err;
}

static int nft_flush_table(struct nft_ctx *ctx)
{
	struct nft_flowtable *flowtable, *nft;
	struct nft_chain *chain, *nc;
	struct nft_object *obj, *ne;
	struct nft_set *set, *ns;
	int err;

	list_for_each_entry(chain, &ctx->table->chains, list) {
		if (!nft_is_active_next(ctx->net, chain))
			continue;

		if (nft_chain_binding(chain))
			continue;

		ctx->chain = chain;

		err = nft_delrule_by_chain(ctx);
		if (err < 0)
			goto out;
	}

	list_for_each_entry_safe(set, ns, &ctx->table->sets, list) {
		if (!nft_is_active_next(ctx->net, set))
			continue;

		if (nft_set_is_anonymous(set))
			continue;

		err = nft_delset(ctx, set);
		if (err < 0)
			goto out;
	}

	list_for_each_entry_safe(flowtable, nft, &ctx->table->flowtables, list) {
		if (!nft_is_active_next(ctx->net, flowtable))
			continue;

		err = nft_delflowtable(ctx, flowtable);
		if (err < 0)
			goto out;
	}

	list_for_each_entry_safe(obj, ne, &ctx->table->objects, list) {
		if (!nft_is_active_next(ctx->net, obj))
			continue;

		err = nft_delobj(ctx, obj);
		if (err < 0)
			goto out;
	}

	list_for_each_entry_safe(chain, nc, &ctx->table->chains, list) {
		if (!nft_is_active_next(ctx->net, chain))
			continue;

		if (nft_chain_binding(chain))
			continue;

		ctx->chain = chain;

		err = nft_delchain(ctx);
		if (err < 0)
			goto out;
	}

	err = nft_deltable(ctx);
out:
	return err;
}

static int nft_flush(struct nft_ctx *ctx, int family)
{
	struct nftables_pernet *nft_net = nft_pernet(ctx->net);
	const struct nlattr * const *nla = ctx->nla;
	struct nft_table *table, *nt;
	int err = 0;

	list_for_each_entry_safe(table, nt, &nft_net->tables, list) {
		if (family != AF_UNSPEC && table->family != family)
			continue;

		ctx->family = table->family;

		if (!nft_is_active_next(ctx->net, table))
			continue;

		if (nft_table_has_owner(table) && table->nlpid != ctx->portid)
			continue;

		if (nla[NFTA_TABLE_NAME] &&
		    nla_strcmp(nla[NFTA_TABLE_NAME], table->name) != 0)
			continue;

		ctx->table = table;

		err = nft_flush_table(ctx);
		if (err < 0)
			goto out;
	}
out:
	return err;
}

static int nf_tables_deltable(struct sk_buff *skb, const struct nfnl_info *info,
			      const struct nlattr * const nla[])
{
	struct netlink_ext_ack *extack = info->extack;
	u8 genmask = nft_genmask_next(info->net);
	u8 family = info->nfmsg->nfgen_family;
	struct net *net = info->net;
	const struct nlattr *attr;
	struct nft_table *table;
	struct nft_ctx ctx;

	nft_ctx_init(&ctx, net, skb, info->nlh, 0, NULL, NULL, nla);
	if (family == AF_UNSPEC ||
	    (!nla[NFTA_TABLE_NAME] && !nla[NFTA_TABLE_HANDLE]))
		return nft_flush(&ctx, family);

	if (nla[NFTA_TABLE_HANDLE]) {
		attr = nla[NFTA_TABLE_HANDLE];
		table = nft_table_lookup_byhandle(net, attr, family, genmask,
						  NETLINK_CB(skb).portid);
	} else {
		attr = nla[NFTA_TABLE_NAME];
		table = nft_table_lookup(net, attr, family, genmask,
					 NETLINK_CB(skb).portid);
	}

	if (IS_ERR(table)) {
		if (PTR_ERR(table) == -ENOENT &&
		    NFNL_MSG_TYPE(info->nlh->nlmsg_type) == NFT_MSG_DESTROYTABLE)
			return 0;

		NL_SET_BAD_ATTR(extack, attr);
		return PTR_ERR(table);
	}

	if (info->nlh->nlmsg_flags & NLM_F_NONREC &&
	    table->use > 0)
		return -EBUSY;

	ctx.family = family;
	ctx.table = table;

	return nft_flush_table(&ctx);
}

static void nf_tables_table_destroy(struct nft_table *table)
{
	if (WARN_ON(table->use > 0))
		return;

	rhltable_destroy(&table->chains_ht);
	rhltable_destroy(&table->objname_ht);
	kfree(table->name);
	kfree(table->udata);
	kfree(table);
}

void nft_register_chain_type(const struct nft_chain_type *ctype)
{
	nfnl_lock(NFNL_SUBSYS_NFTABLES);
	if (WARN_ON(__nft_chain_type_get(ctype->family, ctype->type))) {
		nfnl_unlock(NFNL_SUBSYS_NFTABLES);
		return;
	}
	chain_type[ctype->family][ctype->type] = ctype;
	nfnl_unlock(NFNL_SUBSYS_NFTABLES);
}
EXPORT_SYMBOL_GPL(nft_register_chain_type);

void nft_unregister_chain_type(const struct nft_chain_type *ctype)
{
	nfnl_lock(NFNL_SUBSYS_NFTABLES);
	chain_type[ctype->family][ctype->type] = NULL;
	nfnl_unlock(NFNL_SUBSYS_NFTABLES);
}
EXPORT_SYMBOL_GPL(nft_unregister_chain_type);

/*
 * Chains
 */

static struct nft_chain *
nft_chain_lookup_byhandle(const struct nft_table *table, u64 handle, u8 genmask)
{
	struct nft_chain *chain;

	list_for_each_entry(chain, &table->chains, list) {
		if (chain->handle == handle &&
		    nft_active_genmask(chain, genmask))
			return chain;
	}

	return ERR_PTR(-ENOENT);
}

static bool lockdep_commit_lock_is_held(const struct net *net)
{
#ifdef CONFIG_PROVE_LOCKING
	struct nftables_pernet *nft_net = nft_pernet(net);

	return lockdep_is_held(&nft_net->commit_mutex);
#else
	return true;
#endif
}

static struct nft_chain *nft_chain_lookup(struct net *net,
					  struct nft_table *table,
					  const struct nlattr *nla, u8 genmask)
{
	char search[NFT_CHAIN_MAXNAMELEN + 1];
	struct rhlist_head *tmp, *list;
	struct nft_chain *chain;

	if (nla == NULL)
		return ERR_PTR(-EINVAL);

	nla_strscpy(search, nla, sizeof(search));

	WARN_ON(!rcu_read_lock_held() &&
		!lockdep_commit_lock_is_held(net));

	chain = ERR_PTR(-ENOENT);
	rcu_read_lock();
	list = rhltable_lookup(&table->chains_ht, search, nft_chain_ht_params);
	if (!list)
		goto out_unlock;

	rhl_for_each_entry_rcu(chain, tmp, list, rhlhead) {
		if (nft_active_genmask(chain, genmask))
			goto out_unlock;
	}
	chain = ERR_PTR(-ENOENT);
out_unlock:
	rcu_read_unlock();
	return chain;
}

static const struct nla_policy nft_chain_policy[NFTA_CHAIN_MAX + 1] = {
	[NFTA_CHAIN_TABLE]	= { .type = NLA_STRING,
				    .len = NFT_TABLE_MAXNAMELEN - 1 },
	[NFTA_CHAIN_HANDLE]	= { .type = NLA_U64 },
	[NFTA_CHAIN_NAME]	= { .type = NLA_STRING,
				    .len = NFT_CHAIN_MAXNAMELEN - 1 },
	[NFTA_CHAIN_HOOK]	= { .type = NLA_NESTED },
	[NFTA_CHAIN_POLICY]	= { .type = NLA_U32 },
	[NFTA_CHAIN_TYPE]	= { .type = NLA_STRING,
				    .len = NFT_MODULE_AUTOLOAD_LIMIT },
	[NFTA_CHAIN_COUNTERS]	= { .type = NLA_NESTED },
	[NFTA_CHAIN_FLAGS]	= NLA_POLICY_MASK(NLA_BE32, NFT_CHAIN_FLAGS),
	[NFTA_CHAIN_ID]		= { .type = NLA_U32 },
	[NFTA_CHAIN_USERDATA]	= { .type = NLA_BINARY,
				    .len = NFT_USERDATA_MAXLEN },
};

static const struct nla_policy nft_hook_policy[NFTA_HOOK_MAX + 1] = {
	[NFTA_HOOK_HOOKNUM]	= { .type = NLA_U32 },
	[NFTA_HOOK_PRIORITY]	= { .type = NLA_U32 },
	[NFTA_HOOK_DEV]		= { .type = NLA_STRING,
				    .len = IFNAMSIZ - 1 },
};

static int nft_dump_stats(struct sk_buff *skb, struct nft_stats __percpu *stats)
{
	struct nft_stats *cpu_stats, total;
	struct nlattr *nest;
	unsigned int seq;
	u64 pkts, bytes;
	int cpu;

	if (!stats)
		return 0;

	memset(&total, 0, sizeof(total));
	for_each_possible_cpu(cpu) {
		cpu_stats = per_cpu_ptr(stats, cpu);
		do {
			seq = u64_stats_fetch_begin(&cpu_stats->syncp);
			pkts = cpu_stats->pkts;
			bytes = cpu_stats->bytes;
		} while (u64_stats_fetch_retry(&cpu_stats->syncp, seq));
		total.pkts += pkts;
		total.bytes += bytes;
	}
	nest = nla_nest_start_noflag(skb, NFTA_CHAIN_COUNTERS);
	if (nest == NULL)
		goto nla_put_failure;

	if (nla_put_be64(skb, NFTA_COUNTER_PACKETS, cpu_to_be64(total.pkts),
			 NFTA_COUNTER_PAD) ||
	    nla_put_be64(skb, NFTA_COUNTER_BYTES, cpu_to_be64(total.bytes),
			 NFTA_COUNTER_PAD))
		goto nla_put_failure;

	nla_nest_end(skb, nest);
	return 0;

nla_put_failure:
	return -ENOSPC;
}

static bool hook_is_prefix(struct nft_hook *hook)
{
	return strlen(hook->ifname) >= hook->ifnamelen;
}

static int nft_nla_put_hook_dev(struct sk_buff *skb, struct nft_hook *hook)
{
	int attr = hook_is_prefix(hook) ? NFTA_DEVICE_PREFIX : NFTA_DEVICE_NAME;

	return nla_put_string(skb, attr, hook->ifname);
}

struct nft_hook_dump_ctx {
	struct nft_hook *first;
	int n;
};

static int nft_dump_basechain_hook_one(struct sk_buff *skb,
				       struct nft_hook *hook,
				       struct nft_hook_dump_ctx *dump_ctx)
{
	if (!dump_ctx->first)
		dump_ctx->first = hook;

	if (nft_nla_put_hook_dev(skb, hook))
		return -1;

	dump_ctx->n++;

	return 0;
}

static int nft_dump_basechain_hook_list(struct sk_buff *skb,
					const struct net *net,
					const struct list_head *hook_list,
					struct nft_hook_dump_ctx *dump_ctx)
{
	struct nft_hook *hook;
	int err;

	list_for_each_entry_rcu(hook, hook_list, list,
				lockdep_commit_lock_is_held(net)) {
		err = nft_dump_basechain_hook_one(skb, hook, dump_ctx);
		if (err < 0)
			return err;
	}

	return 0;
}

static int nft_dump_basechain_trans_hook_list(struct sk_buff *skb,
					      const struct list_head *trans_hook_list,
					      struct nft_hook_dump_ctx *dump_ctx)
{
	struct nft_trans_hook *trans_hook;
	int err;

	list_for_each_entry(trans_hook, trans_hook_list, list) {
		err = nft_dump_basechain_hook_one(skb, trans_hook->hook, dump_ctx);
		if (err < 0)
			return err;
	}

	return 0;
}

static int nft_dump_basechain_hook(struct sk_buff *skb,
				   const struct net *net, int family,
				   const struct nft_base_chain *basechain,
				   const struct list_head *hook_list,
				   const struct list_head *trans_hook_list)
{
	const struct nf_hook_ops *ops = &basechain->ops;
	struct nft_hook_dump_ctx dump_hook_ctx = {};
	struct nlattr *nest, *nest_devs;

	nest = nla_nest_start_noflag(skb, NFTA_CHAIN_HOOK);
	if (nest == NULL)
		goto nla_put_failure;
	if (nla_put_be32(skb, NFTA_HOOK_HOOKNUM, htonl(ops->hooknum)))
		goto nla_put_failure;
	if (nla_put_be32(skb, NFTA_HOOK_PRIORITY, htonl(ops->priority)))
		goto nla_put_failure;

	if (nft_base_chain_netdev(family, ops->hooknum)) {
		nest_devs = nla_nest_start_noflag(skb, NFTA_HOOK_DEVS);
		if (!nest_devs)
			goto nla_put_failure;

		if (!hook_list && !trans_hook_list)
			hook_list = &basechain->hook_list;

		if (hook_list &&
		    nft_dump_basechain_hook_list(skb, net, hook_list, &dump_hook_ctx)) {
			goto nla_put_failure;
		} else if (trans_hook_list &&
			   nft_dump_basechain_trans_hook_list(skb, trans_hook_list,
							      &dump_hook_ctx)) {
			goto nla_put_failure;
		}

		nla_nest_end(skb, nest_devs);

		if (dump_hook_ctx.n == 1 &&
		    !hook_is_prefix(dump_hook_ctx.first) &&
		    nla_put_string(skb, NFTA_HOOK_DEV, dump_hook_ctx.first->ifname))
			goto nla_put_failure;
	}
	nla_nest_end(skb, nest);

	return 0;
nla_put_failure:
	return -1;
}

static int nf_tables_fill_chain_info(struct sk_buff *skb, struct net *net,
				     u32 portid, u32 seq, int event, u32 flags,
				     int family, const struct nft_table *table,
				     const struct nft_chain *chain,
				     const struct list_head *hook_list,
				     const struct list_head *trans_hook_list)
{
	struct nlmsghdr *nlh;

	nlh = nfnl_msg_put(skb, portid, seq,
			   nfnl_msg_type(NFNL_SUBSYS_NFTABLES, event),
			   flags, family, NFNETLINK_V0, nft_base_seq_be16(net));
	if (!nlh)
		goto nla_put_failure;

	if (nla_put_string(skb, NFTA_CHAIN_TABLE, table->name) ||
	    nla_put_string(skb, NFTA_CHAIN_NAME, chain->name) ||
	    nla_put_be64(skb, NFTA_CHAIN_HANDLE, cpu_to_be64(chain->handle),
			 NFTA_CHAIN_PAD))
		goto nla_put_failure;

	if (!hook_list && !trans_hook_list &&
	    (event == NFT_MSG_DELCHAIN ||
	     event == NFT_MSG_DESTROYCHAIN)) {
		nlmsg_end(skb, nlh);
		return 0;
	}

	if (nft_is_base_chain(chain)) {
		const struct nft_base_chain *basechain = nft_base_chain(chain);
		struct nft_stats __percpu *stats;

		if (nft_dump_basechain_hook(skb, net, family, basechain,
					    hook_list, trans_hook_list))
			goto nla_put_failure;

		if (nla_put_be32(skb, NFTA_CHAIN_POLICY,
				 htonl(basechain->policy)))
			goto nla_put_failure;

		if (nla_put_string(skb, NFTA_CHAIN_TYPE, basechain->type->name))
			goto nla_put_failure;

		stats = rcu_dereference_check(basechain->stats,
					      lockdep_commit_lock_is_held(net));
		if (nft_dump_stats(skb, stats))
			goto nla_put_failure;
	}

	if (chain->flags &&
	    nla_put_be32(skb, NFTA_CHAIN_FLAGS, htonl(chain->flags)))
		goto nla_put_failure;

	if (nla_put_be32(skb, NFTA_CHAIN_USE, htonl(chain->use)))
		goto nla_put_failure;

	if (chain->udata &&
	    nla_put(skb, NFTA_CHAIN_USERDATA, chain->udlen, chain->udata))
		goto nla_put_failure;

	nlmsg_end(skb, nlh);
	return 0;

nla_put_failure:
	nlmsg_trim(skb, nlh);
	return -1;
}

static void nf_tables_chain_notify(const struct nft_ctx *ctx, int event,
				   const struct list_head *hook_list,
				   const struct list_head *trans_hook_list)
{
	struct nftables_pernet *nft_net;
	struct sk_buff *skb;
	u16 flags = 0;
	int err;

	if (!ctx->report &&
	    !nfnetlink_has_listeners(ctx->net, NFNLGRP_NFTABLES))
		return;

	skb = nlmsg_new(NLMSG_GOODSIZE, GFP_KERNEL);
	if (skb == NULL)
		goto err;

	if (ctx->flags & (NLM_F_CREATE | NLM_F_EXCL))
		flags |= ctx->flags & (NLM_F_CREATE | NLM_F_EXCL);

	err = nf_tables_fill_chain_info(skb, ctx->net, ctx->portid, ctx->seq,
					event, flags, ctx->family, ctx->table,
					ctx->chain, hook_list, trans_hook_list);
	if (err < 0) {
		kfree_skb(skb);
		goto err;
	}

	nft_net = nft_pernet(ctx->net);
	nft_notify_enqueue(skb, ctx->report, &nft_net->notify_list);
	return;
err:
	nfnetlink_set_err(ctx->net, ctx->portid, NFNLGRP_NFTABLES, -ENOBUFS);
}

static int nf_tables_dump_chains(struct sk_buff *skb,
				 struct netlink_callback *cb)
{
	const struct nfgenmsg *nfmsg = nlmsg_data(cb->nlh);
	unsigned int idx = 0, s_idx = cb->args[0];
	struct net *net = sock_net(skb->sk);
	int family = nfmsg->nfgen_family;
	struct nftables_pernet *nft_net;
	const struct nft_table *table;
	const struct nft_chain *chain;

	rcu_read_lock();
	nft_net = nft_pernet(net);
	cb->seq = nft_base_seq(net);

	list_for_each_entry_rcu(table, &nft_net->tables, list) {
		if (family != NFPROTO_UNSPEC && family != table->family)
			continue;

		list_for_each_entry_rcu(chain, &table->chains, list) {
			if (idx < s_idx)
				goto cont;
			if (idx > s_idx)
				memset(&cb->args[1], 0,
				       sizeof(cb->args) - sizeof(cb->args[0]));
			if (!nft_is_active(net, chain))
				continue;
			if (nf_tables_fill_chain_info(skb, net,
						      NETLINK_CB(cb->skb).portid,
						      cb->nlh->nlmsg_seq,
						      NFT_MSG_NEWCHAIN,
						      NLM_F_MULTI,
						      table->family, table,
						      chain, NULL, NULL) < 0)
				goto done;

			nl_dump_check_consistent(cb, nlmsg_hdr(skb));
cont:
			idx++;
		}
	}
done:
	rcu_read_unlock();
	cb->args[0] = idx;
	return skb->len;
}

/* called with rcu_read_lock held */
static int nf_tables_getchain(struct sk_buff *skb, const struct nfnl_info *info,
			      const struct nlattr * const nla[])
{
	struct netlink_ext_ack *extack = info->extack;
	u8 genmask = nft_genmask_cur(info->net);
	u8 family = info->nfmsg->nfgen_family;
	const struct nft_chain *chain;
	struct net *net = info->net;
	struct nft_table *table;
	struct sk_buff *skb2;
	int err;

	if (info->nlh->nlmsg_flags & NLM_F_DUMP) {
		struct netlink_dump_control c = {
			.dump = nf_tables_dump_chains,
			.module = THIS_MODULE,
		};

		return nft_netlink_dump_start_rcu(info->sk, skb, info->nlh, &c);
	}

	table = nft_table_lookup(net, nla[NFTA_CHAIN_TABLE], family, genmask, 0);
	if (IS_ERR(table)) {
		NL_SET_BAD_ATTR(extack, nla[NFTA_CHAIN_TABLE]);
		return PTR_ERR(table);
	}

	chain = nft_chain_lookup(net, table, nla[NFTA_CHAIN_NAME], genmask);
	if (IS_ERR(chain)) {
		NL_SET_BAD_ATTR(extack, nla[NFTA_CHAIN_NAME]);
		return PTR_ERR(chain);
	}

	skb2 = alloc_skb(NLMSG_GOODSIZE, GFP_ATOMIC);
	if (!skb2)
		return -ENOMEM;

	err = nf_tables_fill_chain_info(skb2, net, NETLINK_CB(skb).portid,
					info->nlh->nlmsg_seq, NFT_MSG_NEWCHAIN,
					0, family, table, chain, NULL, NULL);
	if (err < 0)
		goto err_fill_chain_info;

	return nfnetlink_unicast(skb2, net, NETLINK_CB(skb).portid);

err_fill_chain_info:
	kfree_skb(skb2);
	return err;
}

static const struct nla_policy nft_counter_policy[NFTA_COUNTER_MAX + 1] = {
	[NFTA_COUNTER_PACKETS]	= { .type = NLA_U64 },
	[NFTA_COUNTER_BYTES]	= { .type = NLA_U64 },
};

static struct nft_stats __percpu *nft_stats_alloc(const struct nlattr *attr)
{
	struct nlattr *tb[NFTA_COUNTER_MAX+1];
	struct nft_stats __percpu *newstats;
	struct nft_stats *stats;
	int err;

	err = nla_parse_nested_deprecated(tb, NFTA_COUNTER_MAX, attr,
					  nft_counter_policy, NULL);
	if (err < 0)
		return ERR_PTR_PCPU(err);

	if (!tb[NFTA_COUNTER_BYTES] || !tb[NFTA_COUNTER_PACKETS])
		return ERR_PTR_PCPU(-EINVAL);

	newstats = netdev_alloc_pcpu_stats(struct nft_stats);
	if (newstats == NULL)
		return ERR_PTR_PCPU(-ENOMEM);

	/* Restore old counters on this cpu, no problem. Per-cpu statistics
	 * are not exposed to userspace.
	 */
	preempt_disable();
	stats = this_cpu_ptr(newstats);
	stats->bytes = be64_to_cpu(nla_get_be64(tb[NFTA_COUNTER_BYTES]));
	stats->pkts = be64_to_cpu(nla_get_be64(tb[NFTA_COUNTER_PACKETS]));
	preempt_enable();

	return newstats;
}

static void nft_chain_stats_replace(struct nft_trans_chain *trans)
{
	const struct nft_trans *t = &trans->nft_trans_binding.nft_trans;
	struct nft_base_chain *chain = nft_base_chain(trans->chain);

	if (!trans->stats)
		return;

	trans->stats =
		rcu_replace_pointer(chain->stats, trans->stats,
				    lockdep_commit_lock_is_held(t->net));

	if (!trans->stats)
		static_branch_inc(&nft_counters_enabled);
}

static void nf_tables_chain_free_chain_rules(struct nft_chain *chain)
{
	struct nft_rule_blob *g0 = rcu_dereference_raw(chain->blob_gen_0);
	struct nft_rule_blob *g1 = rcu_dereference_raw(chain->blob_gen_1);

	if (g0 != g1)
		kvfree(g1);
	kvfree(g0);

	/* should be NULL either via abort or via successful commit */
	WARN_ON_ONCE(chain->blob_next);
	kvfree(chain->blob_next);
}

void nf_tables_chain_destroy(struct nft_chain *chain)
{
	const struct nft_table *table = chain->table;
	struct nft_hook *hook, *next;

	if (WARN_ON(chain->use > 0))
		return;

	/* no concurrent access possible anymore */
	nf_tables_chain_free_chain_rules(chain);

	if (nft_is_base_chain(chain)) {
		struct nft_base_chain *basechain = nft_base_chain(chain);

		if (nft_base_chain_netdev(table->family, basechain->ops.hooknum)) {
			list_for_each_entry_safe(hook, next,
						 &basechain->hook_list, list)
				nft_netdev_hook_unlink_free_rcu(hook);
		}
		module_put(basechain->type->owner);
		if (rcu_access_pointer(basechain->stats)) {
			static_branch_dec(&nft_counters_enabled);
			free_percpu(rcu_dereference_raw(basechain->stats));
		}
		kfree(chain->name);
		kfree(chain->udata);
		kfree(basechain);
	} else {
		kfree(chain->name);
		kfree(chain->udata);
		kfree(chain);
	}
}

static struct nft_hook *nft_netdev_hook_alloc(struct net *net,
					      const struct nlattr *attr,
					      bool prefix)
{

"###;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
