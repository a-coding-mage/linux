// SPDX-License-Identifier: GPL-2.0-only
/* Inactive native signature assertions, not an emitted Rust ABI/CFI proof. */
#include "notifier_native_bindings.h"
#include "notifier_rust_guard.h"
#include <linux/build_bug.h>
#include <linux/compiler.h>

/* Each expression names the real native declaration, not a recreated layout.
 * Future compilation can check these C header contracts only. Rust output,
 * enum representation, bool lowering and KCFI still require separate evidence.
 */
static_assert(__same_type(&atomic_notifier_chain_register,
	(int (*)(struct atomic_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&atomic_notifier_chain_register_unique_prio,
	(int (*)(struct atomic_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&atomic_notifier_chain_unregister,
	(int (*)(struct atomic_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&atomic_notifier_call_chain,
	(int (*)(struct atomic_notifier_head *, unsigned long, void *))NULL));
static_assert(__same_type(&atomic_notifier_call_chain_is_empty,
	(bool (*)(struct atomic_notifier_head *))NULL));

static_assert(__same_type(&blocking_notifier_chain_register,
	(int (*)(struct blocking_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&blocking_notifier_chain_register_unique_prio,
	(int (*)(struct blocking_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&blocking_notifier_chain_unregister,
	(int (*)(struct blocking_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&blocking_notifier_call_chain_robust,
	(int (*)(struct blocking_notifier_head *, unsigned long,
		 unsigned long, void *))NULL));
static_assert(__same_type(&blocking_notifier_call_chain,
	(int (*)(struct blocking_notifier_head *, unsigned long, void *))NULL));

static_assert(__same_type(&raw_notifier_chain_register,
	(int (*)(struct raw_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&raw_notifier_chain_unregister,
	(int (*)(struct raw_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&raw_notifier_call_chain_robust,
	(int (*)(struct raw_notifier_head *, unsigned long,
		 unsigned long, void *))NULL));
static_assert(__same_type(&raw_notifier_call_chain,
	(int (*)(struct raw_notifier_head *, unsigned long, void *))NULL));

static_assert(__same_type(&srcu_notifier_chain_register,
	(int (*)(struct srcu_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&srcu_notifier_chain_unregister,
	(int (*)(struct srcu_notifier_head *, struct notifier_block *))NULL));
static_assert(__same_type(&srcu_notifier_call_chain,
	(int (*)(struct srcu_notifier_head *, unsigned long, void *))NULL));
static_assert(__same_type(&srcu_init_notifier_head,
	(void (*)(struct srcu_notifier_head *))NULL));

static_assert(__same_type(&notify_die,
	(int (*)(enum die_val, const char *, struct pt_regs *,
		 long, int, int))NULL));
static_assert(__same_type(&register_die_notifier,
	(int (*)(struct notifier_block *))NULL));
static_assert(__same_type(&unregister_die_notifier,
	(int (*)(struct notifier_block *))NULL));

static_assert(__same_type(&lupos_notifier_call_chain,
	(int (*)(struct notifier_block **, unsigned long, void *, int, int *))NULL));
static_assert(__same_type(&lupos_notifier_die_head,
	(struct atomic_notifier_head *(*)(void))NULL));
static_assert(__same_type(((struct notifier_block *)NULL)->notifier_call,
	(int (*)(struct notifier_block *, unsigned long, void *))NULL));
