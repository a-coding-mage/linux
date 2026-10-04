/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_MREMAP_NATIVE_INCLUDES_H
#define RUST_MREMAP_NATIVE_INCLUDES_H
/* Match the immutable original owner's include order. */
#include <linux/mm.h>
#include <linux/mm_inline.h>
#include <linux/hugetlb.h>
#include <linux/shm.h>
#include <linux/ksm.h>
#include <linux/mman.h>
#include <linux/swap.h>
#include <linux/capability.h>
#include <linux/fs.h>
#include <linux/leafops.h>
#include <linux/highmem.h>
#include <linux/security.h>
#include <linux/syscalls.h>
#include <linux/mmu_notifier.h>
#include <linux/uaccess.h>
#include <linux/userfaultfd_k.h>
#include <linux/mempolicy.h>
#include <linux/pgalloc.h>
#include <asm/cacheflush.h>
#include <asm/tlb.h>
#include "internal.h"
#endif
