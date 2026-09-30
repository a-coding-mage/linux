/* SPDX-License-Identifier: GPL-2.0-only */
/* Canonical built-in interfaces for init/noinitramfs.rs. */
#ifndef _RUST_BINDINGS_INIT_NOINITRAMFS_H
#define _RUST_BINDINGS_INIT_NOINITRAMFS_H

#ifdef _LINUX_INIT_H
#error "init_noinitramfs.h must precede linux/init.h in the binding input"
#endif
#pragma push_macro("MODULE")
#undef MODULE
#include <linux/init.h>
#include <linux/stat.h>
#include <linux/kdev_t.h>
#include <linux/syscalls.h>
#include <linux/init_syscalls.h>
#include <linux/umh.h>
#include <linux/printk.h>
#pragma pop_macro("MODULE")

#endif
