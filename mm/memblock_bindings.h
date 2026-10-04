/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Native declarations and header-ABI boundaries for the Rust memblock owner. */
#ifndef LUPOS_MEMBLOCK_BINDINGS_H
#define LUPOS_MEMBLOCK_BINDINGS_H
#include <linux/kernel.h>
#include <linux/slab.h>
#include <linux/init.h>
#include <linux/bitops.h>
#include <linux/poison.h>
#include <linux/pfn.h>
#include <linux/debugfs.h>
#include <linux/kmemleak.h>
#include <linux/seq_file.h>
#include <linux/memblock.h>
#include <linux/mutex.h>
#include <linux/string_helpers.h>
#include <linux/libfdt.h>
#include <linux/kexec_handover.h>
#include <linux/kho/abi/memblock.h>
#include <linux/atomic.h>
#include <linux/kasan.h>
#include <linux/memory_hotplug.h>
#include <linux/pageblock-flags.h>
#include <linux/rcupdate.h>
#ifdef CONFIG_X86_64
#include <asm/cpufeature.h>
#endif
#include <asm/sections.h>
#include <linux/io.h>
#include "internal.h"
#include "mm_init.h"

#define INIT_MEMBLOCK_REGIONS 128
#ifndef INIT_MEMBLOCK_RESERVED_REGIONS
#define INIT_MEMBLOCK_RESERVED_REGIONS INIT_MEMBLOCK_REGIONS
#endif
#ifndef INIT_MEMBLOCK_MEMORY_REGIONS
#define INIT_MEMBLOCK_MEMORY_REGIONS INIT_MEMBLOCK_REGIONS
#endif

enum {
 RUST_MEMBLOCK_INIT_MEMORY_REGIONS = INIT_MEMBLOCK_MEMORY_REGIONS,
 RUST_MEMBLOCK_INIT_RESERVED_REGIONS = INIT_MEMBLOCK_RESERVED_REGIONS,
 RUST_MEMBLOCK_REGION_SIZE = sizeof(struct memblock_region),
 RUST_MEMBLOCK_REGION_ALIGN = __alignof__(struct memblock_region),
 RUST_MEMBLOCK_TYPE_SIZE = sizeof(struct memblock_type),
 RUST_MEMBLOCK_TYPE_ALIGN = __alignof__(struct memblock_type),
 RUST_MEMBLOCK_SIZE = sizeof(struct memblock),
 RUST_MEMBLOCK_CNT_OFFSET = offsetof(struct memblock_type, cnt),
 RUST_MEMBLOCK_REGIONS_OFFSET = offsetof(struct memblock_type, regions),
 RUST_MEMBLOCK_PAGE_SIZE = PAGE_SIZE,
 RUST_MEMBLOCK_PAGE_SHIFT = PAGE_SHIFT,
 RUST_MEMBLOCK_NUMA_NO_NODE = NUMA_NO_NODE,
 RUST_MEMBLOCK_MAX_NUMNODES = MAX_NUMNODES,
 RUST_MEMBLOCK_SMP_CACHE_BYTES = SMP_CACHE_BYTES,
 RUST_MEMBLOCK_MAX_PAGE_ORDER = MAX_PAGE_ORDER,
 RUST_MEMBLOCK_MAX_NR_ZONES = MAX_NR_ZONES,
#ifdef CONFIG_SPARSEMEM
 RUST_MEMBLOCK_PAGES_PER_SECTION = PAGES_PER_SECTION,
#endif
};

bool rust_memblock_warn(bool condition);
void rust_memblock_bug(bool condition);
bool rust_memblock_warn_slab_once(bool condition);
void rust_memblock_warn_deferred_free(void);
void rust_memblock_warn_mirror_find(phys_addr_t size);
void rust_memblock_warn_mirror_alloc(phys_addr_t size);
void rust_memblock_dump_stack(void);
bool rust_memblock_slab_available(void);
bool rust_memblock_movable_node(void);
bool rust_memblock_deferred_pages_enabled(void);
void *rust_memblock_kmalloc(size_t size);
void *rust_memblock_kzalloc_node(size_t size, int nid);
phys_addr_t rust_memblock_pa(const void *ptr);
phys_addr_t rust_memblock_pa_symbol(const void *ptr);
void *rust_memblock_va(phys_addr_t addr);
phys_addr_t rust_memblock_virt_to_phys(void *ptr);
void *rust_memblock_phys_to_virt(phys_addr_t addr);
bool rust_memblock_is_kernel(unsigned long addr);
void rust_memblock_accept_memory(phys_addr_t start, unsigned long size);
void rust_memblock_kmemleak_alloc(phys_addr_t addr, size_t size);
void rust_memblock_kmemleak_free(phys_addr_t addr, size_t size);
struct page *rust_memblock_pfn_to_page(unsigned long pfn);
void *rust_memblock_page_address(struct page *page);
void *rust_memblock_kasan_reset_tag(const void *ptr);
void rust_memblock_free_reserved_page(struct page *page);
void rust_memblock_set_page_reserved(struct page *page);
#if defined(CONFIG_SPARSEMEM) && !defined(CONFIG_HAVE_ARCH_PFN_VALID)
/* Native types, constants and individual compiler/atomic leaves for the Rust
 * mmzone.h search. No C section walk or bitmap search is exposed here. */
#if defined(CONFIG_X86_64) && defined(USE_EARLY_PGTABLE_L5)
#error "Rust memblock sparse search requires the normal kernel LA57 predicate"
#endif
#ifdef __BINDGEN__
enum {
 RUST_MEMBLOCK_PAGE_SECTION_MASK = PAGE_SECTION_MASK,
 RUST_MEMBLOCK_PAGE_SUBSECTION_MASK = PAGE_SUBSECTION_MASK,
};
static_assert((unsigned long)RUST_MEMBLOCK_PAGE_SECTION_MASK == PAGE_SECTION_MASK);
static_assert((unsigned long)RUST_MEMBLOCK_PAGE_SUBSECTION_MASK == PAGE_SUBSECTION_MASK);
static const unsigned int RUST_MEMBLOCK_PFN_SECTION_SHIFT = PFN_SECTION_SHIFT;
static const unsigned long RUST_MEMBLOCK_SECTIONS_PER_ROOT = SECTIONS_PER_ROOT;
static const unsigned long RUST_MEMBLOCK_SECTION_ROOT_MASK = SECTION_ROOT_MASK;
static const unsigned long RUST_MEMBLOCK_SECTION_HAS_MEM_MAP = SECTION_HAS_MEM_MAP;
static const unsigned long RUST_MEMBLOCK_SECTION_IS_EARLY = SECTION_IS_EARLY;
static const unsigned long RUST_MEMBLOCK_PAGES_PER_SUBSECTION = PAGES_PER_SUBSECTION;
static const unsigned long RUST_MEMBLOCK_SUBSECTIONS_PER_SECTION = SUBSECTIONS_PER_SECTION;
static const unsigned long RUST_MEMBLOCK_BITS_PER_LONG = BITS_PER_LONG;
#ifdef CONFIG_X86_64
/* Preserve both native runtime LA57 choices, as in physaddr_bindings.h. */
#pragma push_macro("pgtable_l5_enabled")
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 0
static const unsigned long RUST_MEMBLOCK_NR_SECTION_ROOTS_L4 = NR_SECTION_ROOTS;
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 1
static const unsigned long RUST_MEMBLOCK_NR_SECTION_ROOTS_L5 = NR_SECTION_ROOTS;
#pragma pop_macro("pgtable_l5_enabled")
#else
/* Other native schemas must supply a constant root count. Do not invent an
 * architecture's runtime physical-address-width policy here. */
static_assert(__builtin_constant_p(NR_SECTION_ROOTS));
static const unsigned long RUST_MEMBLOCK_NR_SECTION_ROOTS = NR_SECTION_ROOTS;
#endif
#endif /* __BINDGEN__ */
#ifdef CONFIG_X86_64
bool rust_memblock_cpu_has_la57(void);
#endif
void rust_memblock_preempt_disable(void);
void rust_memblock_preempt_enable(void);
#ifdef CONFIG_DEBUG_LOCK_ALLOC
void rust_memblock_rcu_lock_acquire(void);
void rust_memblock_rcu_lock_release(void);
#endif
#ifdef CONFIG_PROVE_RCU
void rust_memblock_rcu_lock_warn(void);
void rust_memblock_rcu_unlock_warn(void);
#endif
#ifdef CONFIG_SPARSEMEM_VMEMMAP
struct mem_section_usage *rust_memblock_read_usage(struct mem_section_usage * const *ptr);
bool rust_memblock_test_subsection_bit(unsigned long nr, const unsigned long *addr);
unsigned long rust_memblock_subsection_word(const unsigned long *addr);
#endif
#else
bool rust_memblock_pfn_valid(unsigned long pfn);
#endif
unsigned long rust_memblock_pageblock_start(unsigned long pfn);
unsigned long rust_memblock_pageblock_align(unsigned long pfn);
int __meminit rust_memblock_early_pfn_to_nid(unsigned long pfn);
void rust_memblock_atomic_long_set(atomic_long_t *ptr, long value);
void rust_memblock_totalram_pages_add(unsigned long pages);
bool rust_memblock_kho_scratch_overlap(phys_addr_t start, size_t size);
void rust_memblock_reserve_lock(void);
void rust_memblock_reserve_unlock(void);
#ifdef CONFIG_KEXEC_HANDOVER
struct page *rust_memblock_phys_to_page(phys_addr_t addr);
struct page *rust_memblock_alloc_page(void);
void *rust_memblock_page_to_virt(struct page *page);
void rust_memblock_put_page(struct page *page);
u32 rust_memblock_fdt_totalsize(const void *fdt);
#endif
#ifdef CONFIG_DEBUG_FS
void rust_memblock_debugfs_reservation_file(struct dentry *root);
#ifdef CONFIG_ARCH_KEEP_MEMBLOCK
void rust_memblock_debugfs_array_file(const char *name, struct dentry *root,
                                    struct memblock_type *type);
#endif
#endif
#endif
