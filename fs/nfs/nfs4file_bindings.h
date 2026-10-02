/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_NFS4FILE_BINDINGS_H
#define LUPOS_NFS4FILE_BINDINGS_H
#include <linux/fs.h>
#include <linux/file.h>
#include <linux/falloc.h>
#include <linux/mount.h>
#include <linux/nfs_fs.h>
#include <linux/nfs_ssc.h>
#include <linux/splice.h>
#include "delegation.h"
#include "internal.h"
#include "iostat.h"
#include "fscache.h"
#include "pnfs.h"
#include "nfs42.h"

/* Evaluate typed C constants with the configured kernel headers. */
enum {
	RUST_NFS4_FMODE_WRITE = FMODE_WRITE,
	RUST_NFS4_FMODE_CAN_ODIRECT = FMODE_CAN_ODIRECT,
	RUST_NFS4_FOP_DONTCACHE = FOP_DONTCACHE,
	RUST_NFS4_GFP_KERNEL = GFP_KERNEL,
};
struct dentry *rust_nfs4_file_dentry(const struct file *file);
struct inode *rust_nfs4_file_inode(const struct file *file);
struct inode *rust_nfs4_d_inode(const struct dentry *dentry);
const struct nfs_rpc_ops *rust_nfs4_proto(const struct inode *inode);
struct nfs_server *rust_nfs4_server(const struct inode *inode);
struct nfs_server *rust_nfs4_sb(const struct super_block *sb);
fmode_t rust_nfs4_flags_to_mode(int flags);
bool rust_nfs4_test_bit(unsigned int nr, const unsigned long *addr);
void rust_nfs4_set_bit(unsigned int nr, unsigned long *addr);
int rust_nfs4_write_and_wait(struct address_space *mapping);
errseq_t rust_nfs4_sample_wb_err(struct address_space *mapping);
int rust_nfs4_check_wb_err(struct address_space *mapping, errseq_t since);
void rust_nfs4_inc_stats(const struct inode *inode, enum nfs_stat_eventcounters stat);
void rust_nfs4_fscache_open(struct inode *inode, struct file *file);
void rust_nfs4_debug_open(struct dentry *dentry);
void rust_nfs4_debug_flush(struct file *file);
#ifdef CONFIG_NFS_V4_2
ssize_t rust_nfs4_splice_copy(struct file *in, loff_t pos_in, struct file *out, loff_t pos_out, size_t count);
bool rust_nfs4_same_server(struct file *in, struct file *out);
void *rust_nfs4_kzalloc(size_t size);
bool rust_nfs4_is_swapfile(const struct inode *inode);
loff_t rust_nfs4_i_size_read(const struct inode *inode);
void rust_nfs4_block_o_direct(struct inode *inode);
void rust_nfs4_free_fattr(const struct nfs_fattr *fattr);
#endif
#endif
