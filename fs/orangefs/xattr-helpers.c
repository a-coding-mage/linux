// SPDX-License-Identifier: GPL-2.0
/* Original macro/inline/ABI operations only; the xattr algorithm is Rust. */
#include "xattr-bindings.h"

struct orangefs_inode_s *rust_orangefs_xattr_inode(struct inode *inode)
{ return ORANGEFS_I(inode); }
struct inode *rust_orangefs_xattr_d_inode(struct dentry *dentry)
{ return d_inode(dentry); }
bool rust_orangefs_xattr_is_symlink(struct inode *inode)
{ return S_ISLNK(inode->i_mode); }
int rust_orangefs_xattr_interruptible(struct inode *inode)
{ return get_interruptible_flag(inode); }
int rust_orangefs_xattr_fsuid(void)
{ return from_kuid(&init_user_ns, current_fsuid()); }
int rust_orangefs_xattr_fsgid(void)
{ return from_kgid(&init_user_ns, current_fsgid()); }
struct orangefs_cached_xattr *rust_orangefs_xattr_alloc_cache(void)
{ return kmalloc_obj(struct orangefs_cached_xattr); }
unsigned long rust_orangefs_xattr_jiffies(void)
{ return jiffies; }
bool rust_orangefs_xattr_time_before(unsigned long a, unsigned long b)
{ return time_before(a, b); }
void rust_orangefs_xattr_hlist_add_head(struct hlist_node *node, struct hlist_head *head)
{ hlist_add_head(node, head); }
void rust_orangefs_xattr_hlist_del(struct hlist_node *node)
{ hlist_del(node); }
void rust_orangefs_xattr_copy_name(char *dest, const char *src)
{ strscpy(dest, src, ORANGEFS_MAX_XATTR_NAMELEN); }
void rust_orangefs_xattr_copy(void *dest, const void *src, size_t size)
{ memcpy(dest, src, size); }
void rust_orangefs_xattr_zero(void *dest, size_t size)
{ memset(dest, 0, size); }

void rust_orangefs_xattr_debug_get_start(const char *name, size_t size)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "%s: name %s, buffer_size %zd\n", "orangefs_inode_getxattr", name, size); }
void rust_orangefs_xattr_debug_get_ids(struct inode *inode, const char *name, int uid, int gid)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "getxattr on inode %pU, name %s (uid %o, gid %o)\n", get_khandle_from_ino(inode), name, uid, gid); }
void rust_orangefs_xattr_debug_get_missing(struct inode *inode, const char *key)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "orangefs_inode_getxattr: inode %pU key %s does not exist!\n", get_khandle_from_ino(inode), key); }
void rust_orangefs_xattr_debug_get_result(struct inode *inode, const char *key, int key_size, int ret)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "orangefs_inode_getxattr: inode %pU key %s key_sz %d, val_len %d\n", get_khandle_from_ino(inode), key, key_size, ret); }
void rust_orangefs_xattr_debug_remove_key(const char *key, int key_size)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "orangefs_inode_removexattr: key %s, key_sz %d\n", key, key_size); }
void rust_orangefs_xattr_debug_remove_result(int ret)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "orangefs_inode_removexattr: returning %d\n", ret); }
void rust_orangefs_xattr_debug_set_start(const char *name, size_t size)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "%s: name %s, buffer_size %zd\n", "orangefs_inode_setxattr", name, size); }
void rust_orangefs_xattr_debug_removing(const char *name)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "removing xattr (%s)\n", name); }
void rust_orangefs_xattr_debug_set_inode(struct inode *inode, const char *name)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "setxattr on inode %pU, name %s\n", get_khandle_from_ino(inode), name); }
void rust_orangefs_xattr_debug_set_key(const char *key, int key_size, size_t size)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "orangefs_inode_setxattr: key %s, key_sz %d  value size %zd\n", key, key_size, size); }
void rust_orangefs_xattr_debug_set_result(int ret)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "orangefs_inode_setxattr: returning %d\n", ret); }
void rust_orangefs_xattr_error_null(void)
{ gossip_err("%s: bogus NULL pointers\n", "orangefs_listxattr"); }
void rust_orangefs_xattr_error_count(int count)
{ gossip_err("%s: impossible value for returned_count:%d:\n", "orangefs_listxattr", count); }
void rust_orangefs_xattr_error_length(int length)
{ gossip_err("%s: impossible value for lengths[%d]\n", "orangefs_listxattr", length); }
void rust_orangefs_xattr_debug_list_copy(int index, const char *key)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "Copying key %d -> %s\n", index, key); }
void rust_orangefs_xattr_debug_list_reserved(int index, const char *key)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "[RESERVED] key %d -> %s\n", index, key); }
void rust_orangefs_xattr_debug_list_result(int ret, long size, int count)
{ gossip_debug(GOSSIP_XATTR_DEBUG, "%s: returning %d [size of buffer %ld] (filled in %d keys)\n", "orangefs_listxattr", ret, size, count); }
