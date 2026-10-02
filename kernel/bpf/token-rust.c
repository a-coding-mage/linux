// SPDX-License-Identifier: GPL-2.0
/* Macro/static-inline boundaries and registration; no token policy/callbacks. */
#include "token-rust.h"

bool lupos_token_ns_capable(struct user_namespace *ns, int cap)
{
	return ns_capable(ns, cap);
}

struct user_namespace *lupos_token_current_user_ns(void)
{
	return current_user_ns();
}

struct user_namespace *lupos_token_get_user_ns(struct user_namespace *ns)
{
	return get_user_ns(ns);
}

void lupos_token_put_user_ns(struct user_namespace *ns)
{
	put_user_ns(ns);
}

int lupos_token_current_umask(void)
{
	return current_umask();
}

int lupos_token_path_permission(const struct path *path, int mask)
{
	return path_permission(path, mask);
}

/* These hooks are inline only when CONFIG_SECURITY is disabled. */
int lupos_token_security_create(struct bpf_token *token, union bpf_attr *attr,
				const struct path *path)
{
	return security_bpf_token_create(token, attr, path);
}

void lupos_token_security_free(struct bpf_token *token)
{
	security_bpf_token_free(token);
}

int lupos_token_security_capable(const struct bpf_token *token, int cap)
{
	return security_bpf_token_capable(token, cap);
}

int lupos_token_security_cmd(const struct bpf_token *token, enum bpf_cmd cmd)
{
	return security_bpf_token_cmd(token, cmd);
}

void lupos_token_atomic64_inc(atomic64_t *v)
{
	atomic64_inc(v);
}

bool lupos_token_atomic64_dec_and_test(atomic64_t *v)
{
	return atomic64_dec_and_test(v);
}

void lupos_token_atomic64_set(atomic64_t *v, s64 value)
{
	atomic64_set(v, value);
}

void lupos_token_init_work(struct work_struct *work, work_func_t func)
{
	INIT_WORK(work, func);
}

bool lupos_token_schedule_work(struct work_struct *work)
{
	return schedule_work(work);
}

bool lupos_token_fd_empty(const struct fd *f)
{
	return fd_empty(*f);
}

struct file *lupos_token_fd_file(const struct fd *f)
{
	return fd_file(*f);
}

void lupos_token_fdput(const struct fd *f)
{
	fdput(*f);
}

struct fd_prepare lupos_token_fd_prepare(struct inode *inode, struct vfsmount *mnt)
{
	/* Transfer the macro's prepared state to the Rust cleanup guard. This
	 * must retain the original lazy evaluation of alloc_file_pseudo().
	 */
	return __FD_PREPARE_INIT(O_CLOEXEC,
		alloc_file_pseudo(inode, mnt, "bpf-token", O_RDWR, &bpf_token_fops));
}

void lupos_token_fd_prepare_cleanup(const struct fd_prepare *fdf)
{
	class_fd_prepare_destructor(fdf);
}

struct file *lupos_token_fd_prepare_file(const struct fd_prepare *fdf)
{
	return fd_prepare_file(*fdf);
}

int lupos_token_fd_publish(struct fd_prepare *fdf)
{
	return fd_publish(*fdf);
}

struct bpf_token *lupos_token_zalloc(void)
{
	return kzalloc_obj(struct bpf_token, GFP_USER);
}

bool lupos_token_is_err(const void *ptr)
{
	return IS_ERR(ptr);
}

long lupos_token_ptr_err(const void *ptr)
{
	return PTR_ERR(ptr);
}

void *lupos_token_err_ptr(long error)
{
	return ERR_PTR(error);
}

void __user *lupos_token_u64_to_user_ptr(u64 value)
{
	return u64_to_user_ptr(value);
}

unsigned long lupos_token_copy_to_user(void __user *to, const void *from, unsigned long size)
{
	return copy_to_user(to, from, size);
}

int lupos_token_put_info_len(u32 value, union bpf_attr __user *uattr)
{
	return put_user(value, &uattr->info.info_len);
}

const struct inode_operations lupos_bpf_token_iops = { };
const struct file_operations bpf_token_fops = {
	.release = lupos_bpf_token_release,
	.show_fdinfo = lupos_bpf_token_show_fdinfo,
};
