/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_VMSTAT_NATIVE_INCLUDES_H
#define RUST_VMSTAT_NATIVE_INCLUDES_H
#include <linux/fs.h>
#include <linux/mm.h>
#include <linux/err.h>
#include <linux/module.h>
#include <linux/slab.h>
#include <linux/cpu.h>
#include <linux/cpumask.h>
#include <linux/vmstat.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/debugfs.h>
#include <linux/sched.h>
#include <linux/math64.h>
#include <linux/writeback.h>
#include <linux/compaction.h>
#include <linux/mm_inline.h>
#include <linux/page_owner.h>
#include <linux/sched/isolation.h>
#include <linux/memory_hotplug.h>
#include "internal.h"
#include "page_alloc.h"
#endif
