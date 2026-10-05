/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_SHMEM_DECLARATIONS_H
#define RUST_SHMEM_DECLARATIONS_H
extern struct vfsmount *rust_shmem_data_shm_mnt;
extern struct file_system_type rust_shmem_data_shmem_fs_type;
#ifdef CONFIG_SHMEM
extern const struct super_operations rust_shmem_data_shmem_ops;
extern const struct address_space_operations rust_shmem_data_shmem_aops;
extern const struct file_operations rust_shmem_data_shmem_file_operations;
extern const struct inode_operations rust_shmem_data_shmem_inode_operations;
extern const struct inode_operations rust_shmem_data_shmem_dir_inode_operations;
extern const struct inode_operations rust_shmem_data_shmem_special_inode_operations;
extern const struct vm_operations_struct rust_shmem_data_shmem_vm_ops;
extern const struct vm_operations_struct rust_shmem_data_shmem_anon_vm_ops;
extern struct list_head rust_shmem_data_shmem_swaplist;
extern spinlock_t rust_shmem_data_shmem_swaplist_lock;
extern spinlock_t rust_shmem_encode_lock;
extern struct kmem_cache *rust_shmem_data_shmem_inode_cachep;
#ifdef CONFIG_TMPFS
extern const struct inode_operations rust_shmem_data_shmem_symlink_inode_operations;
extern const struct inode_operations rust_shmem_data_shmem_short_symlink_operations;
extern const struct pipe_buf_operations rust_shmem_data_zero_pipe_buf_ops;
extern const struct export_operations rust_shmem_data_shmem_export_ops;
#ifdef CONFIG_TMPFS_XATTR
extern const struct xattr_handler * const rust_shmem_data_shmem_xattr_handlers[4];
#endif
#endif
extern const struct fs_context_operations rust_shmem_data_shmem_fs_context_ops;
#ifdef CONFIG_TRANSPARENT_HUGEPAGE
extern unsigned long rust_shmem_data_huge_shmem_orders_always;
extern unsigned long rust_shmem_data_huge_shmem_orders_madvise;
extern unsigned long rust_shmem_data_huge_shmem_orders_inherit;
extern unsigned long rust_shmem_data_huge_shmem_orders_within_size;
extern bool rust_shmem_data_shmem_orders_configured;
extern int rust_shmem_data_shmem_huge;
extern int rust_shmem_data_tmpfs_huge;
extern char rust_shmem_data_str_dup[PAGE_SIZE];
#ifdef CONFIG_SYSFS
extern spinlock_t rust_shmem_data_huge_shmem_orders_lock;
/* Exact immutable sizes from original shmem.c arrays, no ABI replicas. */
extern const char * const rust_shmem_data_huge_mode_strings[5];
extern unsigned long * const rust_shmem_data_huge_mode_orders[4];
#endif
#endif
#if IS_ENABLED(CONFIG_UNICODE) && defined(CONFIG_TMPFS)
extern const struct dentry_operations rust_shmem_data_shmem_ci_dentry_ops;
#endif
#if defined(CONFIG_SYSFS) && defined(CONFIG_TMPFS)
extern const struct attribute_group rust_shmem_data_tmpfs_attribute_group;
extern struct kobject *rust_shmem_data_tmpfs_kobj;
#endif
#endif
#endif
