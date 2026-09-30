/* SPDX-License-Identifier: GPL-2.0 */
/* Canonical built-in interfaces for the initrd component of init/mounts.o. */
#ifndef _RUST_BINDINGS_INIT_MOUNTS_H
#define _RUST_BINDINGS_INIT_MOUNTS_H

#ifdef _LINUX_INIT_H
#error "init_mounts.h must precede linux/init.h in the binding input"
#endif
#pragma push_macro("MODULE")
#undef MODULE
#include <linux/init.h>

#include <linux/kernel.h>
#include <linux/initrd.h>
#include <linux/printk.h>
#include "../../init/do_mounts.h"
#pragma pop_macro("MODULE")

#endif
