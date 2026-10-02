/* SPDX-License-Identifier: GPL-2.0 */
#ifndef _BPF_TOKEN_RUST_H
#define _BPF_TOKEN_RUST_H

#include <linux/bpf.h>
#include <linux/capability.h>
#include <linux/cred.h>
#include <linux/err.h>
#include <linux/file.h>
#include <linux/fs.h>
#include <linux/fs_struct.h>
#include <linux/kernel.h>
#include <linux/namei.h>
#include <linux/security.h>
#include <linux/seq_file.h>
#include <linux/slab.h>
#include <linux/uaccess.h>
#include <linux/user_namespace.h>
#include <linux/workqueue.h>

/* Every type/layout comes directly from the original configured headers. */
enum {
	LUPOS_TOKEN_SIZE = sizeof(struct bpf_token),
	LUPOS_TOKEN_ALIGN = __alignof__(struct bpf_token),
	LUPOS_TOKEN_WORK_OFFSET = offsetof(struct bpf_token, work),
	LUPOS_TOKEN_REFCNT_OFFSET = offsetof(struct bpf_token, refcnt),
	LUPOS_TOKEN_USERNS_OFFSET = offsetof(struct bpf_token, userns),
	LUPOS_TOKEN_CMDS_OFFSET = offsetof(struct bpf_token, allowed_cmds),
	LUPOS_TOKEN_MAPS_OFFSET = offsetof(struct bpf_token, allowed_maps),
	LUPOS_TOKEN_PROGS_OFFSET = offsetof(struct bpf_token, allowed_progs),
	LUPOS_TOKEN_ATTACHS_OFFSET = offsetof(struct bpf_token, allowed_attachs),
	LUPOS_TOKEN_INFO_SIZE = sizeof(struct bpf_token_info),
	LUPOS_TOKEN_INFO_ALIGN = __alignof__(struct bpf_token_info),
	LUPOS_TOKEN_INFO_CMDS_OFFSET = offsetof(struct bpf_token_info, allowed_cmds),
	LUPOS_TOKEN_INFO_MAPS_OFFSET = offsetof(struct bpf_token_info, allowed_maps),
	LUPOS_TOKEN_INFO_PROGS_OFFSET = offsetof(struct bpf_token_info, allowed_progs),
	LUPOS_TOKEN_INFO_ATTACHS_OFFSET = offsetof(struct bpf_token_info, allowed_attachs),
	LUPOS_TOKEN_MAX_CMD = __MAX_BPF_CMD,
	LUPOS_TOKEN_MAX_MAP_TYPE = __MAX_BPF_MAP_TYPE,
	LUPOS_TOKEN_MAX_PROG_TYPE = __MAX_BPF_PROG_TYPE,
	LUPOS_TOKEN_MAX_ATTACH_TYPE = __MAX_BPF_ATTACH_TYPE,
	LUPOS_TOKEN_CAP_SYS_ADMIN = CAP_SYS_ADMIN,
	LUPOS_TOKEN_CAP_BPF = CAP_BPF,
	LUPOS_TOKEN_MAY_ACCESS = MAY_ACCESS,
	LUPOS_TOKEN_S_IFREG = S_IFREG,
	LUPOS_TOKEN_S_IRUSR = S_IRUSR,
	LUPOS_TOKEN_S_IWUSR = S_IWUSR,
};

bool lupos_token_ns_capable(struct user_namespace *ns, int cap);
struct user_namespace *lupos_token_current_user_ns(void);
struct user_namespace *lupos_token_get_user_ns(struct user_namespace *ns);
void lupos_token_put_user_ns(struct user_namespace *ns);
int lupos_token_current_umask(void);
int lupos_token_path_permission(const struct path *path, int mask);
int lupos_token_security_create(struct bpf_token *token, union bpf_attr *attr,
				const struct path *path);
void lupos_token_security_free(struct bpf_token *token);
int lupos_token_security_capable(const struct bpf_token *token, int cap);
int lupos_token_security_cmd(const struct bpf_token *token, enum bpf_cmd cmd);
void lupos_token_atomic64_inc(atomic64_t *v);
bool lupos_token_atomic64_dec_and_test(atomic64_t *v);
void lupos_token_atomic64_set(atomic64_t *v, s64 value);
void lupos_token_init_work(struct work_struct *work, work_func_t func);
bool lupos_token_schedule_work(struct work_struct *work);
bool lupos_token_fd_empty(const struct fd *f);
struct file *lupos_token_fd_file(const struct fd *f);
void lupos_token_fdput(const struct fd *f);
struct fd_prepare lupos_token_fd_prepare(struct inode *inode, struct vfsmount *mnt);
void lupos_token_fd_prepare_cleanup(const struct fd_prepare *fdf);
struct file *lupos_token_fd_prepare_file(const struct fd_prepare *fdf);
int lupos_token_fd_publish(struct fd_prepare *fdf);
struct bpf_token *lupos_token_zalloc(void);
bool lupos_token_is_err(const void *ptr);
long lupos_token_ptr_err(const void *ptr);
void *lupos_token_err_ptr(long error);
void __user *lupos_token_u64_to_user_ptr(u64 value);
unsigned long lupos_token_copy_to_user(void __user *to, const void *from, unsigned long size);
int lupos_token_put_info_len(u32 value, union bpf_attr __user *uattr);

/* Rust callbacks; C owns only the constant registration objects. */
int lupos_bpf_token_release(struct inode *inode, struct file *filp);
void lupos_bpf_token_show_fdinfo(struct seq_file *m, struct file *filp);
extern const struct inode_operations lupos_bpf_token_iops;

#endif
