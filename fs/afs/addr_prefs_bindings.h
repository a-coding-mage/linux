/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_AFS_ADDR_PREFS_BINDINGS_H
#define LUPOS_AFS_ADDR_PREFS_BINDINGS_H

#include <linux/slab.h>
#include <linux/ctype.h>
#include <linux/inet.h>
#include <linux/seq_file.h>
#include <keys/rxrpc-type.h>
#include "internal.h"

/* Configured C declarations own every layout. These helpers expose only
 * macros, header inlines and anonymous-union/member-macro access. */
struct afs_net *rust_afs_addr_prefs_file_net(struct file *file);
void rust_afs_addr_prefs_inode_lock(struct file *file);
void rust_afs_addr_prefs_inode_unlock(struct file *file);
struct afs_addr_preference_list *rust_afs_addr_prefs_deref_locked(
	struct afs_net *net, struct file *file);
const struct afs_addr_preference_list *rust_afs_addr_prefs_deref(
	struct afs_net *net);
struct afs_addr_preference_list *rust_afs_addr_prefs_access(struct afs_net *net);
void rust_afs_addr_prefs_assign(struct afs_net *net,
	struct afs_addr_preference_list *prefs);
void rust_afs_addr_prefs_free_rcu(struct afs_addr_preference_list *prefs);
void rust_afs_addr_prefs_rcu_read_lock(void);
void rust_afs_addr_prefs_rcu_read_unlock(void);
u16 rust_afs_addr_prefs_load_acquire_u16(const u16 *p);
unsigned int rust_afs_addr_prefs_load_acquire_uint(const unsigned int *p);
unsigned int rust_afs_addr_prefs_read_once_uint(const unsigned int *p);
void rust_afs_addr_prefs_store_release_u16(u16 *p, u16 value);
void rust_afs_addr_prefs_store_release_uint(unsigned int *p, unsigned int value);
void rust_afs_addr_prefs_write_once_u16(u16 *p, u16 value);
size_t rust_afs_addr_prefs_struct_size(size_t count);
size_t rust_afs_addr_prefs_roundup_pow_of_two(size_t size);
struct afs_addr_preference_list *rust_afs_addr_prefs_kmalloc(size_t size);
struct afs_addr_preference_list *rust_afs_addr_prefs_kmalloc_flex(size_t count);
bool rust_afs_addr_prefs_isspace(char c);
struct in_addr *rust_afs_addr_prefs_ipv4(struct afs_addr_preference *pref);
struct in6_addr *rust_afs_addr_prefs_ipv6(struct afs_addr_preference *pref);
const __be32 *rust_afs_addr_prefs_ipv4_words(const struct afs_addr_preference *pref);
const __be32 *rust_afs_addr_prefs_ipv6_words(const struct afs_addr_preference *pref);
void rust_afs_addr_prefs_warn(const char *message);
void rust_afs_addr_prefs_enter(const char *function,
	const struct afs_addr_preference_list *prefs, int index);
void rust_afs_addr_prefs_leave(int ret);

#endif /* LUPOS_AFS_ADDR_PREFS_BINDINGS_H */
