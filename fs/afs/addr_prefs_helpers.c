// SPDX-License-Identifier: GPL-2.0-or-later
/* Address-preference macro/header-inline boundaries. Policy is in Rust. */
#define pr_fmt(fmt) KBUILD_MODNAME ": addr_prefs: " fmt
#include "addr_prefs_bindings.h"

struct afs_net *rust_afs_addr_prefs_file_net(struct file *file)
{
	return afs_net(seq_file_single_net(file->private_data));
}

void rust_afs_addr_prefs_inode_lock(struct file *file)
{
	inode_lock(file_inode(file));
}

void rust_afs_addr_prefs_inode_unlock(struct file *file)
{
	inode_unlock(file_inode(file));
}

struct afs_addr_preference_list *rust_afs_addr_prefs_deref_locked(
	struct afs_net *net, struct file *file)
{
	return rcu_dereference_protected(net->address_prefs,
		lockdep_is_held(&file_inode(file)->i_rwsem));
}

const struct afs_addr_preference_list *rust_afs_addr_prefs_deref(struct afs_net *net)
{
	return rcu_dereference(net->address_prefs);
}

struct afs_addr_preference_list *rust_afs_addr_prefs_access(struct afs_net *net)
{
	return rcu_access_pointer(net->address_prefs);
}

void rust_afs_addr_prefs_assign(struct afs_net *net,
	struct afs_addr_preference_list *prefs)
{
	rcu_assign_pointer(net->address_prefs, prefs);
}

void rust_afs_addr_prefs_free_rcu(struct afs_addr_preference_list *prefs)
{
	kfree_rcu(prefs, rcu);
}

void rust_afs_addr_prefs_rcu_read_lock(void) { rcu_read_lock(); }
void rust_afs_addr_prefs_rcu_read_unlock(void) { rcu_read_unlock(); }
u16 rust_afs_addr_prefs_load_acquire_u16(const u16 *p) { return smp_load_acquire(p); }
unsigned int rust_afs_addr_prefs_load_acquire_uint(const unsigned int *p)
{
	return smp_load_acquire(p);
}

unsigned int rust_afs_addr_prefs_read_once_uint(const unsigned int *p)
{
	return READ_ONCE(*p);
}

void rust_afs_addr_prefs_store_release_u16(u16 *p, u16 value)
{
	smp_store_release(p, value);
}

void rust_afs_addr_prefs_store_release_uint(unsigned int *p, unsigned int value)
{
	smp_store_release(p, value);
}

void rust_afs_addr_prefs_write_once_u16(u16 *p, u16 value) { WRITE_ONCE(*p, value); }
size_t rust_afs_addr_prefs_struct_size(size_t count)
{
	return struct_size_t(struct afs_addr_preference_list, prefs, count);
}

size_t rust_afs_addr_prefs_roundup_pow_of_two(size_t size)
{
	return roundup_pow_of_two(size);
}

struct afs_addr_preference_list *rust_afs_addr_prefs_kmalloc(size_t size)
{
	return kmalloc(size, GFP_KERNEL);
}

struct afs_addr_preference_list *rust_afs_addr_prefs_kmalloc_flex(size_t count)
{
	return kmalloc_flex(struct afs_addr_preference_list, prefs, count);
}

bool rust_afs_addr_prefs_isspace(char c) { return isspace(c); }
struct in_addr *rust_afs_addr_prefs_ipv4(struct afs_addr_preference *pref)
{
	return &pref->ipv4_addr;
}

struct in6_addr *rust_afs_addr_prefs_ipv6(struct afs_addr_preference *pref)
{
	return &pref->ipv6_addr;
}

const __be32 *rust_afs_addr_prefs_ipv4_words(const struct afs_addr_preference *pref)
{
	return &pref->ipv4_addr.s_addr;
}

const __be32 *rust_afs_addr_prefs_ipv6_words(const struct afs_addr_preference *pref)
{
	return pref->ipv6_addr.s6_addr32;
}

void rust_afs_addr_prefs_warn(const char *message) { pr_warn("%s", message); }

/* Expand the original debug macros with the Rust caller's function name. */
void rust_afs_addr_prefs_enter(const char *function,
	const struct afs_addr_preference_list *prefs, int index)
{
#if defined(__KDEBUG)
	dbgprintk("==> %s({%u/%u/%u},%u)", function,
		  prefs->ipv6_off, prefs->nr, prefs->max_prefs, index);
#elif defined(CONFIG_AFS_DEBUG)
	if (unlikely(afs_debug & AFS_DEBUG_KENTER))
		dbgprintk("==> %s({%u/%u/%u},%u)", function,
			  prefs->ipv6_off, prefs->nr, prefs->max_prefs, index);
#else
	no_printk("==> %s({%u/%u/%u},%u)", function,
		  prefs->ipv6_off, prefs->nr, prefs->max_prefs, index);
#endif
}

void rust_afs_addr_prefs_leave(int ret)
{
#if defined(__KDEBUG)
	dbgprintk("<== afs_proc_addr_prefs_write() = %d", ret);
#elif defined(CONFIG_AFS_DEBUG)
	if (unlikely(afs_debug & AFS_DEBUG_KLEAVE))
		dbgprintk("<== afs_proc_addr_prefs_write() = %d", ret);
#else
	no_printk("<== afs_proc_addr_prefs_write() = %d", ret);
#endif
}
