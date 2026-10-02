/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_ORANGEFS_XATTR_BINDINGS_H
#define LUPOS_ORANGEFS_XATTR_BINDINGS_H

#include "protocol.h"
#include "orangefs-kernel.h"
#include "orangefs-bufmap.h"
#include <linux/posix_acl_xattr.h>
#include <linux/xattr.h>
#include <linux/hashtable.h>

enum { RUST_ORANGEFS_XATTR_HZ = HZ };

/* Existing macro, header-inline and configured-layout boundaries only. */
struct orangefs_inode_s *rust_orangefs_xattr_inode(struct inode *inode);
struct inode *rust_orangefs_xattr_d_inode(struct dentry *dentry);
bool rust_orangefs_xattr_is_symlink(struct inode *inode);
int rust_orangefs_xattr_interruptible(struct inode *inode);
int rust_orangefs_xattr_fsuid(void);
int rust_orangefs_xattr_fsgid(void);
struct orangefs_cached_xattr *rust_orangefs_xattr_alloc_cache(void);
unsigned long rust_orangefs_xattr_jiffies(void);
bool rust_orangefs_xattr_time_before(unsigned long a, unsigned long b);
void rust_orangefs_xattr_hlist_add_head(struct hlist_node *node, struct hlist_head *head);
void rust_orangefs_xattr_hlist_del(struct hlist_node *node);
void rust_orangefs_xattr_copy_name(char *dest, const char *src);
void rust_orangefs_xattr_copy(void *dest, const void *src, size_t size);
void rust_orangefs_xattr_zero(void *dest, size_t size);

/* Each diagnostic bridge invokes only the original gossip macro. */
void rust_orangefs_xattr_debug_get_start(const char *name, size_t size);
void rust_orangefs_xattr_debug_get_ids(struct inode *inode, const char *name, int uid, int gid);
void rust_orangefs_xattr_debug_get_missing(struct inode *inode, const char *key);
void rust_orangefs_xattr_debug_get_result(struct inode *inode, const char *key, int key_size, int ret);
void rust_orangefs_xattr_debug_remove_key(const char *key, int key_size);
void rust_orangefs_xattr_debug_remove_result(int ret);
void rust_orangefs_xattr_debug_set_start(const char *name, size_t size);
void rust_orangefs_xattr_debug_removing(const char *name);
void rust_orangefs_xattr_debug_set_inode(struct inode *inode, const char *name);
void rust_orangefs_xattr_debug_set_key(const char *key, int key_size, size_t size);
void rust_orangefs_xattr_debug_set_result(int ret);
void rust_orangefs_xattr_error_null(void);
void rust_orangefs_xattr_error_count(int count);
void rust_orangefs_xattr_error_length(int length);
void rust_orangefs_xattr_debug_list_copy(int index, const char *key);
void rust_orangefs_xattr_debug_list_reserved(int index, const char *key);
void rust_orangefs_xattr_debug_list_result(int ret, long size, int count);

#endif /* LUPOS_ORANGEFS_XATTR_BINDINGS_H */
