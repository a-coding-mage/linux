/* SPDX-License-Identifier: GPL-2.0-only */
/* Canonical built-in declarations for the Rust init/do_mounts.rs owner. */
#ifndef _RUST_BINDINGS_INIT_ROOT_MOUNTS_H
#define _RUST_BINDINGS_INIT_ROOT_MOUNTS_H

#ifdef _LINUX_INIT_H
#error "init_root_mounts.h must precede linux/init.h in the binding input"
#endif
#pragma push_macro("MODULE")
#undef MODULE
#include <linux/init.h>
#include <linux/async.h>
#include <linux/delay.h>
#include <linux/device.h>
#include <linux/device/driver.h>
#include <linux/fs.h>
#include <linux/fs_struct.h>
#include <linux/ktime.h>
#include <linux/kstrtox.h>
#include <linux/mount.h>
#include <linux/nfs_fs.h>
#include <linux/panic.h>
#include <linux/printk.h>
#include <linux/raid/detect.h>
#include <linux/ramfs.h>
#include <linux/sched.h>
#include <linux/shmem_fs.h>
#include <linux/string.h>
#include <linux/timekeeping.h>
#include <uapi/linux/mount.h>
#include "../../init/do_mounts.h"

enum { RUST_INIT_ROOT_MOUNTS_PAGE_SIZE = PAGE_SIZE };

/* Bindgen cannot evaluate every function-like macro expansion (SB_RDONLY is
 * BIT(0)). Typed constants retain the original expression's signedness and
 * architecture-dependent width instead of re-encoding flag values in Rust. */
const __typeof__(MS_RDONLY) RUST_INIT_ROOT_MOUNTS_MS_RDONLY = MS_RDONLY;
const __typeof__(MS_SILENT) RUST_INIT_ROOT_MOUNTS_MS_SILENT = MS_SILENT;
const __typeof__(SB_RDONLY) RUST_INIT_ROOT_MOUNTS_SB_RDONLY = SB_RDONLY;

/* Only allocator/compiler metadata crosses this boundary. */
char *rust_init_root_mounts_alloc_data_page(void);
char *rust_init_root_mounts_alloc_fs_names(void);
char *rust_init_root_mounts_alloc_nodev_names(void);

#pragma pop_macro("MODULE")
#endif
