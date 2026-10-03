/* SPDX-License-Identifier: GPL-2.0-only */
/* Canonical built-in declarations for init/do_mounts_rd.rs. */
#ifndef _RUST_BINDINGS_INIT_RAMDISK_H
#define _RUST_BINDINGS_INIT_RAMDISK_H

#ifdef _LINUX_INIT_H
#error "init_ramdisk.h must precede linux/init.h in the binding input"
#endif
#pragma push_macro("MODULE")
#undef MODULE
#include <linux/init.h>
#include <linux/decompress/generic.h>
#include <linux/err.h>
#include <linux/file.h>
#include <linux/fs.h>
#include <linux/ext2_fs.h>
#include <linux/init_syscalls.h>
#include <linux/kstrtox.h>
#include <linux/minix_fs.h>
#include <linux/panic.h>
#include <linux/printk.h>
#include <linux/romfs_fs.h>
#include <linux/slab.h>
#include <uapi/linux/cramfs_fs.h>
#include "../../fs/squashfs/squashfs_fs.h"
#include "../../init/ramdisk_loader_abi.h"

/* Preserve the native-endian values of the byte-order expression macros. */
static const __be32 RUST_INIT_RD_ROMSB_WORD0 = ROMSB_WORD0;
static const __be32 RUST_INIT_RD_ROMSB_WORD1 = ROMSB_WORD1;
#pragma pop_macro("MODULE")
#endif
