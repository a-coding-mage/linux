/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_MPROTECT_NATIVE_BINDINGS_H
#define RUST_MPROTECT_NATIVE_BINDINGS_H
#include "mprotect_native_includes.h"
#include "mprotect_native_primitives.h"
#define RMP_CONST(type, name) static const type RUST_MPROTECT_##name = name
RMP_CONST(unsigned long, PAGE_SIZE);
static_assert(PAGE_MASK == ~(PAGE_SIZE - 1));
RMP_CONST(unsigned int, PAGE_SHIFT);
RMP_CONST(unsigned long, PUD_SIZE);
#ifdef CONFIG_PGTABLE_HAS_HUGE_LEAVES
RMP_CONST(unsigned long, HPAGE_PMD_SIZE);
RMP_CONST(unsigned long, HPAGE_PMD_NR);
RMP_CONST(unsigned long, HPAGE_PUD_NR);
#endif
RMP_CONST(unsigned long, MM_CP_PROT_NUMA);
RMP_CONST(unsigned long, MM_CP_TRY_CHANGE_WRITABLE);
RMP_CONST(unsigned long, MM_CP_UFFD_WP);
RMP_CONST(unsigned long, MM_CP_UFFD_WP_RESOLVE);
RMP_CONST(unsigned long, MM_CP_UFFD_WP_ALL);
RMP_CONST(unsigned long, MM_CP_UFFD_RWP);
RMP_CONST(unsigned long, MM_CP_UFFD_RWP_RESOLVE);
RMP_CONST(unsigned long, MM_CP_UFFD_RWP_ALL);
RMP_CONST(fpb_t, FPB_RESPECT_SOFT_DIRTY);
RMP_CONST(fpb_t, FPB_RESPECT_WRITE);
RMP_CONST(pte_marker, PTE_MARKER_UFFD_WP);
RMP_CONST(vm_flags_t, VM_ACCESS_FLAGS);
RMP_CONST(vm_flags_t, VM_FLAGS_CLEAR);
#ifdef CONFIG_ARCH_HAS_PKEYS
RMP_CONST(unsigned long, PKEY_ACCESS_MASK);
#endif
#undef RMP_CONST
#endif
