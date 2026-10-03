// SPDX-License-Identifier: GPL-2.0-only
/*
 * Declaration-only module ABI metadata. All production objects, including
 * init_task itself and its initializers, are defined exclusively by Rust.
 * Keep the export's C type for DWARF module versions: the Rust binding uses
 * a checked pointer-valued view of x86 thread.sp for its static relocation.
 */
#include <linux/sched.h>
#include <linux/sched/task.h>
#include <linux/export.h>

EXPORT_SYMBOL(init_task);
