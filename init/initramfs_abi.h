/* SPDX-License-Identifier: GPL-2.0 */
#ifndef _INIT_INITRAMFS_RUST_ABI_H
#define _INIT_INITRAMFS_RUST_ABI_H
#include <linux/limits.h>
#include <linux/types.h>
#include "initramfs_internal.h"

/* Private allocation types are declared once so typed slab inference and
 * Rust's generated layout describe the exact same allocated object. */
#define RUST_INITRAMFS_N_ALIGN(len) ((((len) + 1) & ~3) + 2)
struct rust_initramfs_hash {
	int ino, minor, major;
	umode_t mode;
	struct rust_initramfs_hash *next;
	char name[RUST_INITRAMFS_N_ALIGN(PATH_MAX)];
};
struct rust_initramfs_buffers {
	char header[CPIO_HDRLEN];
	char symlink[PATH_MAX + RUST_INITRAMFS_N_ALIGN(PATH_MAX) + 1];
	char name[RUST_INITRAMFS_N_ALIGN(PATH_MAX)];
};
#ifdef CONFIG_INITRAMFS_PRESERVE_MTIME
struct rust_initramfs_dir_entry {
	struct rust_initramfs_dir_entry *next;
	time64_t mtime;
	char name[];
};
struct rust_initramfs_dir_entry *rust_initramfs_alloc_dir(size_t nlen);
#endif
struct rust_initramfs_hash *rust_initramfs_alloc_hash(void);
struct rust_initramfs_buffers *rust_initramfs_alloc_buffers(void);
void *rust_initramfs_phys_to_virt(phys_addr_t address);
struct fs_struct;
struct fs_struct *rust_initramfs_override_init_fs(void);
void rust_initramfs_revert_init_fs(struct fs_struct *old);
void rust_initramfs_detected_compression(const char *name);
#endif
