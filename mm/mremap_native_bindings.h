/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_MREMAP_NATIVE_BINDINGS_H
#define RUST_MREMAP_NATIVE_BINDINGS_H
#include "mremap_native_includes.h"
/* Exact original translation-unit-local data declarations, made visible to
 * bindgen. No algorithm body is supplied by this native type authority. */
enum mremap_type { MREMAP_INVALID, MREMAP_NO_RESIZE, MREMAP_SHRINK, MREMAP_EXPAND };
enum pgt_entry { NORMAL_PMD, HPAGE_PMD, NORMAL_PUD, HPAGE_PUD };
struct vma_remap_struct {
 unsigned long addr;
 unsigned long old_len;
 unsigned long new_len;
 const unsigned long flags;
 unsigned long new_addr;
 struct vm_userfaultfd_ctx *uf;
 struct list_head *uf_unmap_early;
 struct list_head *uf_unmap;
 struct vm_area_struct *vma;
 unsigned long delta;
 bool populate_expand;
 enum mremap_type remap_type;
 bool mmap_locked;
 unsigned long charged;
 bool vmi_needs_invalidate;
};
#define MR_CONST(type, name) static const type RUST_MREMAP_##name = name
MR_CONST(unsigned long, PAGE_SIZE);
/* Keep high-bit masks in Rust const evaluation while checking the exact
 * relationship against the current architecture's native definitions. */
static_assert(PAGE_MASK == ~(PAGE_SIZE - 1));
MR_CONST(unsigned int, PAGE_SHIFT);
MR_CONST(unsigned long, PMD_SIZE);
static_assert(PMD_MASK == ~(PMD_SIZE - 1));
MR_CONST(unsigned long, PUD_SIZE);
static_assert(PUD_MASK == ~(PUD_SIZE - 1));
#ifdef CONFIG_PGTABLE_HAS_HUGE_LEAVES
MR_CONST(unsigned long, HPAGE_PMD_SIZE);
MR_CONST(unsigned long, HPAGE_PUD_SIZE);
#endif
MR_CONST(fpb_t, FPB_RESPECT_WRITE);
#undef MR_CONST
#ifdef arch_supports_page_table_move
static const bool RUST_MREMAP_CFG_ARCH_SUPPORTS_PAGE_TABLE_MOVE = true;
#endif
#if CONFIG_PGTABLE_LEVELS > 2 && defined(CONFIG_HAVE_MOVE_PUD)
static const bool RUST_MREMAP_CFG_MOVE_NORMAL_PUD = true;
#endif
#include "mremap_native_primitives.h"
#endif
