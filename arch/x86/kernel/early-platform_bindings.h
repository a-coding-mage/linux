/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_X86_EARLY_PLATFORM_BINDINGS_H
#define RUST_X86_EARLY_PLATFORM_BINDINGS_H

/* Native declarations and complete configured layouts, never ABI replicas. */
#include <linux/init.h>
#include <linux/memblock.h>
#include <asm/setup.h>
#include <asm/bios_ebda.h>

#ifdef __BINDGEN__
/* NUMA_NO_NODE is a signed macro; preserve its native type and value. */
static const int RUST_EARLY_PLATFORM_NUMA_NO_NODE = NUMA_NO_NODE;
#endif

#endif /* RUST_X86_EARLY_PLATFORM_BINDINGS_H */
