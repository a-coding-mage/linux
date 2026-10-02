// SPDX-License-Identifier: GPL-2.0
/* C macro and header-inline boundaries only. nfs4file.c policy is in Rust. */
#include "nfs4file_bindings.h"
#define NFSDBG_FACILITY NFSDBG_FILE

#ifdef CONFIG_CFI
/* C callbacks referenced by the Rust operation table need CFI identities. */
__ADDRESSABLE(nfs_file_read);
__ADDRESSABLE(nfs_file_write);
__ADDRESSABLE(nfs_file_mmap_prepare);
__ADDRESSABLE(nfs_file_release);
__ADDRESSABLE(nfs_file_fsync);
__ADDRESSABLE(nfs_lock);
__ADDRESSABLE(nfs_flock);
__ADDRESSABLE(nfs_file_splice_read);
__ADDRESSABLE(iter_file_splice_write);
__ADDRESSABLE(nfs_check_flags);
#ifndef CONFIG_NFS_V4_2
__ADDRESSABLE(nfs_file_llseek);
#endif
#endif

struct dentry *rust_nfs4_file_dentry(const struct file *file) { return file_dentry(file); }
struct inode *rust_nfs4_file_inode(const struct file *file) { return file_inode(file); }
struct inode *rust_nfs4_d_inode(const struct dentry *dentry) { return d_inode(dentry); }
const struct nfs_rpc_ops *rust_nfs4_proto(const struct inode *inode) { return NFS_PROTO(inode); }
struct nfs_server *rust_nfs4_server(const struct inode *inode) { return NFS_SERVER(inode); }
struct nfs_server *rust_nfs4_sb(const struct super_block *sb) { return NFS_SB(sb); }
fmode_t rust_nfs4_flags_to_mode(int flags) { return flags_to_mode(flags); }
bool rust_nfs4_test_bit(unsigned int nr, const unsigned long *addr) { return test_bit(nr, addr); }
void rust_nfs4_set_bit(unsigned int nr, unsigned long *addr) { set_bit(nr, addr); }
int rust_nfs4_write_and_wait(struct address_space *mapping) { return filemap_write_and_wait(mapping); }
errseq_t rust_nfs4_sample_wb_err(struct address_space *mapping) { return filemap_sample_wb_err(mapping); }
int rust_nfs4_check_wb_err(struct address_space *mapping, errseq_t since) { return filemap_check_wb_err(mapping, since); }
void rust_nfs4_inc_stats(const struct inode *inode, enum nfs_stat_eventcounters stat) { nfs_inc_stats(inode, stat); }
void rust_nfs4_fscache_open(struct inode *inode, struct file *file) { nfs_fscache_open_file(inode, file); }
void rust_nfs4_debug_open(struct dentry *dentry) { dprintk("NFS: open file(%pd2)\n", dentry); }
void rust_nfs4_debug_flush(struct file *file) { dprintk("NFS: flush(%pD2)\n", file); }
#ifdef CONFIG_NFS_V4_2
ssize_t rust_nfs4_splice_copy(struct file *in, loff_t pos_in, struct file *out, loff_t pos_out, size_t count)
{ return splice_copy_file_range(in, pos_in, out, pos_out, count); }
bool rust_nfs4_same_server(struct file *in, struct file *out) { return nfs42_files_from_same_server(in, out); }
void *rust_nfs4_kzalloc(size_t size) { return kzalloc(size, GFP_KERNEL); }
bool rust_nfs4_is_swapfile(const struct inode *inode) { return IS_SWAPFILE(inode); }
loff_t rust_nfs4_i_size_read(const struct inode *inode) { return i_size_read(inode); }
void rust_nfs4_block_o_direct(struct inode *inode) { nfs_file_block_o_direct(NFS_I(inode)); }
void rust_nfs4_free_fattr(const struct nfs_fattr *fattr) { nfs_free_fattr(fattr); }
#endif
