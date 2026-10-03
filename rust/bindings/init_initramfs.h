/* SPDX-License-Identifier: GPL-2.0 */
/* Canonical declarations and constants for the Rust initramfs owner. */
#ifndef _RUST_BINDINGS_INIT_INITRAMFS_H
#define _RUST_BINDINGS_INIT_INITRAMFS_H

#ifdef _LINUX_INIT_H
#error "init_initramfs.h must precede linux/init.h in the binding input"
#endif
#pragma push_macro("MODULE")
#undef MODULE
#include <linux/init.h>
#include <linux/async.h>
#include <linux/decompress/generic.h>
#include <linux/err.h>
#include <linux/fcntl.h>
#include <linux/file.h>
#include <linux/fs.h>
#include <linux/fs_struct.h>
#include <linux/hex.h>
#include <linux/init_syscalls.h>
#include <linux/initrd.h>
#include <linux/kdev_t.h>
#include <linux/kexec.h>
#include <linux/kstrtox.h>
#include <linux/memblock.h>
#include <linux/mm.h>
#include <linux/panic.h>
#include <linux/poison.h>
#include <linux/printk.h>
#include <linux/security.h>
#include <linux/slab.h>
#include <linux/string.h>
#include <linux/sysfs.h>
#include <linux/task_work.h>
#include <linux/umh.h>
#include "../../init/initramfs_abi.h"

const unsigned long RUST_INITRAMFS_PAGE_SIZE = PAGE_SIZE;
const int RUST_INITRAMFS_MAX_NR_ZONES = MAX_NR_ZONES;

#pragma pop_macro("MODULE")
#endif
