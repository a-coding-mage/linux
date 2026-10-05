/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_MIGRATE_NATIVE_TYPES_H
#define RUST_MIGRATE_NATIVE_TYPES_H
#include <linux/migrate.h>
#include <linux/export.h>
#include <linux/swap.h>
#include <linux/leafops.h>
#include <linux/pagemap.h>
#include <linux/buffer_head.h>
#include <linux/mm_inline.h>
#include <linux/ksm.h>
#include <linux/rmap.h>
#include <linux/topology.h>
#include <linux/cpu.h>
#include <linux/cpuset.h>
#include <linux/writeback.h>
#include <linux/mempolicy.h>
#include <linux/vmalloc.h>
#include <linux/security.h>
#include <linux/backing-dev.h>
#include <linux/compaction.h>
#include <linux/syscalls.h>
#include <linux/compat.h>
#include <linux/hugetlb.h>
#include <linux/gfp.h>
#include <linux/page_idle.h>
#include <linux/page_owner.h>
#include <linux/sched/mm.h>
#include <linux/ptrace.h>
#include <linux/memory.h>
#include <linux/sched/sysctl.h>
#include <linux/memory-tiers.h>
#include <linux/pagewalk.h>
#include <asm/tlbflush.h>
#include <trace/events/migrate.h>
#include "internal.h"
#include "page_alloc.h"
#include "swap.h"
// Exact native function-pointer types, without guessed declarations.
typedef new_folio_t *rust_migrate_new_folio_t;
typedef free_folio_t *rust_migrate_free_folio_t;
typedef typeof(((struct address_space_operations *)0)->migrate_folio) rust_migrate_mapping_callback_t;
#endif
