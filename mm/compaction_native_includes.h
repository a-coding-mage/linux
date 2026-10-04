/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_COMPACTION_NATIVE_INCLUDES_H
#define RUST_COMPACTION_NATIVE_INCLUDES_H
#include <linux/cpu.h>
#include <linux/swap.h>
#include <linux/migrate.h>
#include <linux/compaction.h>
#include <linux/mm_inline.h>
#include <linux/sched/signal.h>
#include <linux/backing-dev.h>
#include <linux/sysctl.h>
#include <linux/sysfs.h>
#include <linux/page-isolation.h>
#include <linux/kasan.h>
#include <linux/kthread.h>
#include <linux/freezer.h>
#include <linux/page_owner.h>
#include <linux/psi.h>
#include <linux/cpuset.h>
#include <linux/memory_hotplug.h>
#include "page_alloc.h"
#include "internal.h"
#endif
