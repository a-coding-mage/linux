/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_X86_PHYSADDR_BINDINGS_H
#define RUST_X86_PHYSADDR_BINDINGS_H

#include <linux/memblock.h>
#include <linux/mmdebug.h>
#include <linux/mm.h>
#include <linux/rcupdate.h>
#include <asm/page.h>
#include <asm/processor.h>

#if !defined(CONFIG_X86_64) || !defined(CONFIG_SPARSEMEM) || \
    !defined(CONFIG_SPARSEMEM_EXTREME) || defined(CONFIG_HAVE_ARCH_PFN_VALID)
#error "Rust physaddr requires x86-64 generic SPARSEMEM_EXTREME pfn_valid"
#endif
#ifdef USE_EARLY_PGTABLE_L5
#error "Rust physaddr requires the normal kernel LA57 feature predicate"
#endif

#ifdef __BINDGEN__
static const unsigned long RUST_PHYSADDR_START_KERNEL_MAP = __START_KERNEL_map;
static const unsigned long RUST_PHYSADDR_KERNEL_IMAGE_SIZE = KERNEL_IMAGE_SIZE;
static const unsigned int RUST_PHYSADDR_PAGE_SHIFT = PAGE_SHIFT;
static const unsigned int RUST_PHYSADDR_PFN_SECTION_SHIFT = PFN_SECTION_SHIFT;
static const unsigned long RUST_PHYSADDR_SECTIONS_PER_ROOT = SECTIONS_PER_ROOT;
static const unsigned long RUST_PHYSADDR_SECTION_ROOT_MASK = SECTION_ROOT_MASK;
static const unsigned long RUST_PHYSADDR_SECTION_HAS_MEM_MAP = SECTION_HAS_MEM_MAP;
static const unsigned long RUST_PHYSADDR_SECTION_IS_EARLY = SECTION_IS_EARLY;
static const unsigned long RUST_PHYSADDR_PAGE_SECTION_MASK = PAGE_SECTION_MASK;
static const unsigned long RUST_PHYSADDR_PAGES_PER_SUBSECTION = PAGES_PER_SUBSECTION;

/* Evaluate both native branches after all includes; restore the real macro. */
#pragma push_macro("pgtable_l5_enabled")
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 0
static const unsigned long RUST_PHYSADDR_NR_MEM_SECTIONS_L4 = NR_MEM_SECTIONS;
static const unsigned long RUST_PHYSADDR_NR_SECTION_ROOTS_L4 = NR_SECTION_ROOTS;
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 1
static const unsigned long RUST_PHYSADDR_NR_MEM_SECTIONS_L5 = NR_MEM_SECTIONS;
static const unsigned long RUST_PHYSADDR_NR_SECTION_ROOTS_L5 = NR_SECTION_ROOTS;
#pragma pop_macro("pgtable_l5_enabled")
#endif

bool rust_physaddr_cpu_has_la57(void);
void rust_physaddr_preempt_disable(void);
void rust_physaddr_preempt_enable(void);
#ifdef CONFIG_DEBUG_VIRTUAL
void __noreturn rust_physaddr_bug_image(void);
void __noreturn rust_physaddr_bug_direct_map(void);
#endif
#ifdef CONFIG_DEBUG_LOCK_ALLOC
void rust_physaddr_rcu_lock_acquire(void);
void rust_physaddr_rcu_lock_release(void);
#endif
#ifdef CONFIG_PROVE_RCU
void rust_physaddr_rcu_lock_warn(void);
void rust_physaddr_rcu_unlock_warn(void);
#endif
#ifdef CONFIG_SPARSEMEM_VMEMMAP
struct mem_section_usage *rust_physaddr_read_usage(struct mem_section_usage * const *ptr);
bool rust_physaddr_test_bit(unsigned long nr, const unsigned long *addr);
#endif
#endif
