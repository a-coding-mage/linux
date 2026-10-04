/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_MMAP_NATIVE_BINDINGS_H
#define RUST_MMAP_NATIVE_BINDINGS_H
#include "mmap_native_includes.h"
#include "mmap_native_primitives.h"
#define RM_CONST(type, name) static const type RUST_MMAP_##name = name
RM_CONST(unsigned long, PAGE_SIZE);
static_assert(PAGE_MASK == ~(PAGE_SIZE - 1));
RM_CONST(unsigned int, PAGE_SHIFT);
RM_CONST(unsigned long, PMD_SIZE);
RM_CONST(unsigned long, FIRST_USER_ADDRESS);
RM_CONST(unsigned long, USER_PGTABLES_CEILING);
RM_CONST(u64, MAX_LFS_FILESIZE);
RM_CONST(unsigned long, LEGACY_MAP_MASK);
RM_CONST(unsigned long, GFP_KERNEL);
RM_CONST(unsigned long, FOP_UNSIGNED_OFFSET);
RM_CONST(unsigned long, FOP_MMAP_SYNC);
RM_CONST(unsigned long, FMODE_READ);
RM_CONST(unsigned long, FMODE_WRITE);
RM_CONST(unsigned int, VM_FAULT_SIGBUS);
RM_CONST(unsigned long, SZ_128K);
RM_CONST(unsigned long, SZ_8K);
#ifdef CONFIG_HAVE_ARCH_MMAP_RND_BITS
RM_CONST(int, CONFIG_ARCH_MMAP_RND_BITS_MIN);
RM_CONST(int, CONFIG_ARCH_MMAP_RND_BITS_MAX);
RM_CONST(int, CONFIG_ARCH_MMAP_RND_BITS);
#endif
#ifdef CONFIG_HAVE_ARCH_MMAP_RND_COMPAT_BITS
RM_CONST(int, CONFIG_ARCH_MMAP_RND_COMPAT_BITS_MIN);
RM_CONST(int, CONFIG_ARCH_MMAP_RND_COMPAT_BITS_MAX);
RM_CONST(int, CONFIG_ARCH_MMAP_RND_COMPAT_BITS);
#endif
#undef RM_CONST
/* These booleans are converted to Rust cfgs by the binding rule. They follow
 * the actual architecture headers, not an inferred Kconfig equivalent. */
#ifdef HAVE_ARCH_UNMAPPED_AREA
static const bool RUST_MMAP_CFG_HAVE_ARCH_UNMAPPED_AREA = true;
#endif
#ifdef HAVE_ARCH_UNMAPPED_AREA_TOPDOWN
static const bool RUST_MMAP_CFG_HAVE_ARCH_UNMAPPED_AREA_TOPDOWN = true;
#endif
#ifdef __ARCH_WANT_SYS_OLD_MMAP
static const bool RUST_MMAP_CFG_ARCH_WANT_SYS_OLD_MMAP = true;
#endif
#if defined(HAVE_ARCH_PICK_MMAP_LAYOUT) || defined(CONFIG_ARCH_WANT_DEFAULT_TOPDOWN_MMAP_LAYOUT)
static const bool RUST_MMAP_CFG_LEGACY_VA_LAYOUT = true;
#endif
#ifdef __ARCH_WANT_SYS_OLD_MMAP
struct mmap_arg_struct {
 unsigned long addr, len, prot, flags, fd, offset;
};
#endif
#endif
