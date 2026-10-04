/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_UFFD_NATIVE_INCLUDES_H
#define RUST_UFFD_NATIVE_INCLUDES_H
#include <linux/mm.h>
#include <linux/sched/signal.h>
#include <linux/pagemap.h>
#include <linux/rmap.h>
#include <linux/swap.h>
#include <linux/leafops.h>
#include <linux/userfaultfd_k.h>
#include <linux/mmu_notifier.h>
#include <linux/hugetlb.h>
#include <linux/list.h>
#include <linux/sched/mm.h>
#include <linux/mm_inline.h>
#include <linux/poll.h>
#include <linux/slab.h>
#include <linux/seq_file.h>
#include <linux/bug.h>
#include <linux/anon_inodes.h>
#include <linux/syscalls.h>
#include <linux/miscdevice.h>
#include <linux/uio.h>
#include <linux/file.h>
#include <linux/cleanup.h>
#include <asm/tlbflush.h>
#include <asm/tlb.h>
#include "internal.h"
#include "swap.h"
/* Complete original private record layouts, shared with native field leaves. */
struct userfaultfd_fork_ctx { struct userfaultfd_ctx *orig, *new; struct list_head list; };
struct userfaultfd_unmap_ctx { struct userfaultfd_ctx *ctx; unsigned long start, end; struct list_head list; };
struct userfaultfd_wait_queue { struct uffd_msg msg; wait_queue_entry_t wq; struct userfaultfd_ctx *ctx; bool waken; };
struct userfaultfd_wake_range { unsigned long start, len; };
static_assert(sizeof(struct uffd_msg) == 32);
static_assert(PAGE_MASK == ~(PAGE_SIZE - 1));
static_assert(!(UFFD_USER_MODE_ONLY & UFFD_SHARED_FCNTL_FLAGS));
#endif
