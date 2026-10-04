/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_X86_E820_BINDINGS_H
#define RUST_X86_E820_BINDINGS_H

/* Exactly the configured kernel types and provider declarations, never ABI replicas. */
#include <linux/memblock.h>
#include <linux/suspend.h>
#include <linux/acpi.h>
#include <linux/firmware-map.h>
#include <linux/sort.h>
#include <linux/kvm_types.h>
#include <linux/init.h>
#include <linux/memory_hotplug.h>
#include <linux/slab.h>
#include <linux/string.h>
#include <asm/e820/api.h>
#include <asm/setup.h>

#ifdef __BINDGEN__
/* Preserve each macro's actual configured value and the C use site's type. */
static const unsigned int RUST_E820_MAX_ENTRIES = E820_MAX_ENTRIES;
static const unsigned int RUST_E820_PAGE_SHIFT = PAGE_SHIFT;
static const unsigned long RUST_E820_PAGE_SIZE = PAGE_SIZE;
static const unsigned long RUST_E820_SZ_1M = SZ_1M;
static const unsigned long RUST_E820_SZ_4M = SZ_4M;
static const unsigned long RUST_E820_SZ_256M = SZ_256M;
static const u64 RUST_E820_MAX_GAP_END = SZ_4G;
static const u64 RUST_E820_LOWMEMSIZE = LOWMEMSIZE();
static const u64 RUST_E820_HIGH_MEMORY = HIGH_MEMORY;
static const unsigned long RUST_E820_ISA_END_ADDRESS = ISA_END_ADDRESS;
static const unsigned long RUST_E820_SMP_CACHE_BYTES = SMP_CACHE_BYTES;
static const unsigned long RUST_E820_IORESOURCE_SYSTEM_RAM = IORESOURCE_SYSTEM_RAM;
static const unsigned long RUST_E820_IORESOURCE_MEM = IORESOURCE_MEM;
static const unsigned long RUST_E820_IORESOURCE_BUSY = IORESOURCE_BUSY;
static const int RUST_E820_NUMA_NO_NODE = NUMA_NO_NODE;
static const phys_addr_t RUST_E820_MEMBLOCK_ALLOC_ACCESSIBLE = MEMBLOCK_ALLOC_ACCESSIBLE;
static const unsigned int RUST_E820_X86_FEATURE_PSE = X86_FEATURE_PSE;
#ifdef CONFIG_X86_32
#ifdef CONFIG_X86_PAE
static const unsigned long RUST_E820_MAX_ARCH_PFN = 1ULL << (36 - PAGE_SHIFT);
#else
static const unsigned long RUST_E820_MAX_ARCH_PFN = 1ULL << (32 - PAGE_SHIFT);
#endif
#else
/* Evaluate both branches from canonical MAXMEM, then restore the real kernel
 * predicate. Rust chooses at runtime using cpu_feature_enabled(LA57).
 */
#pragma push_macro("pgtable_l5_enabled")
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 0
static const unsigned long RUST_E820_MAX_ARCH_PFN_L4 = MAXMEM >> PAGE_SHIFT;
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 1
static const unsigned long RUST_E820_MAX_ARCH_PFN_L5 = MAXMEM >> PAGE_SHIFT;
#pragma pop_macro("pgtable_l5_enabled")
#endif
#endif /* __BINDGEN__ */

/* Only compiler/architecture macros that Rust cannot invoke directly. */
void __noreturn __init rust_e820_bug(void);
bool __init rust_e820_cpu_has_la57(void);
void *__init rust_e820_kmemdup_main(const void *src, size_t len);
void *__init rust_e820_kmemdup_kexec(const void *src, size_t len);
void *__init rust_e820_kmemdup_firmware(const void *src, size_t len);
#endif /* RUST_X86_E820_BINDINGS_H */
