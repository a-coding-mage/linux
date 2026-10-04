/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_X86_MM_INIT_BINDINGS_H
#define RUST_X86_MM_INIT_BINDINGS_H
#include <linux/signal.h>
#include <linux/sched.h>
#include <linux/kernel.h>
#include <linux/errno.h>
#include <linux/string.h>
#include <linux/types.h>
#include <linux/ptrace.h>
#include <linux/mman.h>
#include <linux/mm.h>
#include <linux/smp.h>
#include <linux/init.h>
#include <linux/pagemap.h>
#include <linux/proc_fs.h>
#include <linux/dma-mapping.h>
#include <linux/nmi.h>
#include <linux/gfp.h>
#include <linux/initrd.h>
#include <linux/ioport.h>
#include <linux/swap.h>
#include <linux/memblock.h>
#include <linux/swapfile.h>
#include <linux/swapops.h>
#include <linux/kmemleak.h>
#include <linux/sched/task.h>
#include <linux/execmem.h>
#include <linux/memory.h>
#include <linux/memory_hotplug.h>
#include <linux/memremap.h>
#include <linux/pfn.h>
#include <linux/poison.h>
#include <linux/kcore.h>
#include <linux/pci.h>
#include <asm/processor.h>
#include <asm/bios_ebda.h>
#include <linux/uaccess.h>
#include <asm/fixmap.h>
#include <asm/apic.h>
#include <asm/smp.h>
#include <asm/kdebug.h>
#include <asm/uv/uv.h>
#include <asm/set_memory.h>
#include <asm/cpu_device_id.h>
#include <asm/e820/api.h>
#include <asm/init.h>
#include <asm/page.h>
#include <asm/page_types.h>
#include <asm/sections.h>
#include <asm/setup.h>
#include <asm/tlbflush.h>
#include <asm/tlb.h>
#include <asm/proto.h>
#include <asm/dma.h>
#include <asm/kaslr.h>
#include <asm/hypervisor.h>
#include <asm/cpufeature.h>
#include <asm/pti.h>
#include <asm/text-patching.h>
#include <asm/memtype.h>
#include <asm/mmu_context.h>
#include <asm/pgalloc.h>
#include <asm/numa.h>
#include <asm/ftrace.h>
#include "mm_internal.h"

/* RUST_X86_MM_INIT is X86_64-only. Its canonical Kconfig always selects five
 * compile-time levels; LA57-disabled machines fold PGD at runtime. Keep Rust's
 * native field construction tied to that authoritative layout.
 */
#ifdef CONFIG_X86_64
_Static_assert(CONFIG_PGTABLE_LEVELS == 5, "x86-64 init page-table levels");
_Static_assert(sizeof(pgd_t) == sizeof(pgdval_t) && offsetof(pgd_t, pgd) == 0,
	       "native pgd layout");
_Static_assert(sizeof(p4d_t) == sizeof(p4dval_t) && offsetof(p4d_t, p4d) == 0,
	       "native p4d layout");
_Static_assert(sizeof(pud_t) == sizeof(pudval_t) && offsetof(pud_t, pud) == 0,
	       "native pud layout");
_Static_assert(sizeof(pmd_t) == sizeof(pmdval_t) && offsetof(pmd_t, pmd) == 0,
	       "native pmd layout");
_Static_assert(sizeof(pte_t) == sizeof(pteval_t) && offsetof(pte_t, pte) == 0,
	       "native pte layout");
#endif

#define RUST_MM_INIT_PGD_PAGE_TABLES 4
#ifdef CONFIG_RANDOMIZE_MEMORY
#define RUST_MM_INIT_PGD_PAGE_COUNT (4 * RUST_MM_INIT_PGD_PAGE_TABLES)
#else
#define RUST_MM_INIT_PGD_PAGE_COUNT (2 * RUST_MM_INIT_PGD_PAGE_TABLES)
#endif
#define RUST_MM_INIT_PGT_BYTES (RUST_MM_INIT_PGD_PAGE_COUNT * PAGE_SIZE)
/* Native enums preserve unsigned high-bit values in the pinned bindgen,
 * unlike a static const declaration which can become an extern static.
 * Keep the source macro expressions and verify the original unsigned widths.
 */
enum rust_mm_address_constants {
	RUST_MM_PAGE_MASK = PAGE_MASK,
	RUST_MM_PMD_MASK = PMD_MASK,
	RUST_MM_PUD_MASK = PUD_MASK,
	RUST_MM_P4D_MASK = P4D_MASK,
	RUST_MM_PHYS_ADDR_MAX = PHYS_ADDR_MAX,
#ifdef CONFIG_X86_64
	RUST_MM_START_KERNEL_MAP = __START_KERNEL_map,
	RUST_MM_VSYSCALL_ADDR = VSYSCALL_ADDR,
#endif
};
_Static_assert((unsigned long)RUST_MM_PAGE_MASK == PAGE_MASK, "PAGE_MASK width");
_Static_assert((unsigned long)RUST_MM_PMD_MASK == PMD_MASK, "PMD_MASK width");
_Static_assert((unsigned long)RUST_MM_PUD_MASK == PUD_MASK, "PUD_MASK width");
_Static_assert((unsigned long)RUST_MM_P4D_MASK == P4D_MASK, "P4D_MASK width");
_Static_assert((phys_addr_t)RUST_MM_PHYS_ADDR_MAX == PHYS_ADDR_MAX, "PHYS_ADDR_MAX width");
#ifdef CONFIG_X86_64
_Static_assert((unsigned long)RUST_MM_START_KERNEL_MAP == __START_KERNEL_map, "kernel map width");
_Static_assert((unsigned long)RUST_MM_VSYSCALL_ADDR == VSYSCALL_ADDR, "vsyscall width");
#endif

#ifdef __BINDGEN__
#define MM_ULONG(name, value) static const unsigned long RUST_MM_##name = (value)
#define MM_UINT(name, value) static const unsigned int RUST_MM_##name = (value)
MM_UINT(PAGE_SHIFT, PAGE_SHIFT);
MM_UINT(PMD_SHIFT, PMD_SHIFT);
MM_UINT(PUD_SHIFT, PUD_SHIFT);
MM_UINT(P4D_SHIFT, P4D_SHIFT);
MM_UINT(PHYSICAL_MASK_SHIFT, __PHYSICAL_MASK_SHIFT);
MM_ULONG(PAGE_SIZE, PAGE_SIZE);
MM_ULONG(PMD_SIZE, PMD_SIZE);
MM_ULONG(PUD_SIZE, PUD_SIZE);
MM_ULONG(P4D_SIZE, P4D_SIZE);
MM_ULONG(PTRS_PER_PTE, PTRS_PER_PTE);
MM_ULONG(PTRS_PER_PMD, PTRS_PER_PMD);
MM_ULONG(PTRS_PER_PUD, PTRS_PER_PUD);
MM_ULONG(PTRS_PER_PGD, PTRS_PER_PGD);
MM_ULONG(PAGE_PWT, _PAGE_PWT);
MM_ULONG(PAGE_PCD, _PAGE_PCD);
MM_ULONG(PAGE_PAT, _PAGE_PAT);
MM_ULONG(PAGE_PSE, _PAGE_PSE);
MM_ULONG(PAGE_PRESENT, _PAGE_PRESENT);
MM_ULONG(PAGE_PROTNONE, _PAGE_PROTNONE);
MM_ULONG(PAGE_DIRTY, _PAGE_DIRTY);
MM_ULONG(PAGE_RW, _PAGE_RW);
MM_ULONG(PAGE_PAT_LARGE, _PAGE_PAT_LARGE);
MM_ULONG(PAGE_KNL_ERRATUM_MASK, _PAGE_KNL_ERRATUM_MASK);
MM_ULONG(PAGE_TABLE_NOENC, _PAGE_TABLE_NOENC);
MM_ULONG(PAGE_USER, _PAGE_USER);
MM_ULONG(PAGE_GLOBAL, _PAGE_GLOBAL);
MM_ULONG(PAGE_NOPTISHADOW, _PAGE_NOPTISHADOW);
MM_ULONG(PAGE_CACHE_MASK, _PAGE_CACHE_MASK);
MM_UINT(PAGE_BIT_PAT, _PAGE_BIT_PAT);
MM_UINT(PAGE_BIT_PAT_LARGE, _PAGE_BIT_PAT_LARGE);
MM_UINT(PAGE_BIT_PCD, _PAGE_BIT_PCD);
MM_UINT(PAGE_BIT_PWT, _PAGE_BIT_PWT);
MM_ULONG(INIT_PGT_BUF_SIZE, RUST_MM_INIT_PGT_BYTES);
MM_UINT(E820_MAX_ENTRIES, E820_MAX_ENTRIES);
MM_ULONG(SMP_CACHE_BYTES, SMP_CACHE_BYTES);
MM_UINT(X86_STEPPING_ANY, X86_STEPPING_ANY);
MM_UINT(X86_FEATURE_ANY, X86_FEATURE_ANY);
MM_UINT(X86_CPU_TYPE_ANY, X86_CPU_TYPE_ANY);
MM_UINT(X86_CPU_ID_FLAG_ENTRY_VALID, X86_CPU_ID_FLAG_ENTRY_VALID);
#define MM_VFM(model) \
    MM_UINT(model##_VENDOR, VFM_VENDOR(INTEL_##model)); \
    MM_UINT(model##_FAMILY, VFM_FAMILY(INTEL_##model)); \
    MM_UINT(model##_MODEL, VFM_MODEL(INTEL_##model))
MM_VFM(ALDERLAKE);
MM_VFM(ALDERLAKE_L);
MM_VFM(ATOM_GRACEMONT);
MM_VFM(RAPTORLAKE);
MM_VFM(RAPTORLAKE_P);
MM_VFM(RAPTORLAKE_S);
#undef MM_VFM
MM_ULONG(ISA_END_ADDRESS, ISA_END_ADDRESS);
MM_UINT(MAX_NUMNODES, MAX_NUMNODES);
MM_ULONG(X86_CR4_PSE, X86_CR4_PSE);
MM_ULONG(X86_CR4_PGE, X86_CR4_PGE);
MM_ULONG(X86_CR4_PCIDE, X86_CR4_PCIDE);
MM_UINT(X86_FEATURE_PCID, X86_FEATURE_PCID);
MM_ULONG(IORESOURCE_SYSTEM_RAM, IORESOURCE_SYSTEM_RAM);
MM_UINT(POISON_FREE_INITMEM, POISON_FREE_INITMEM);
MM_ULONG(MAX_DMA_PFN, MAX_DMA_PFN);
MM_ULONG(MAX_DMA32_PFN, MAX_DMA32_PFN);
#if CONFIG_PGTABLE_LEVELS > 2
MM_UINT(SWAP_LIMIT_SHIFT, PAGE_SHIFT - SWP_OFFSET_FIRST_BIT);
#else
MM_UINT(SWAP_LIMIT_SHIFT, 0);
#endif
MM_ULONG(PT_LIST_OFFSET, offsetof(struct ptdesc, pt_list));
MM_ULONG(MM_PAGE_TABLE_LOCK_OFFSET, offsetof(struct mm_struct, page_table_lock));
MM_ULONG(READ_IMPLIES_EXEC, READ_IMPLIES_EXEC);
MM_UINT(PMD_ORDER, PMD_ORDER);
MM_ULONG(MIN_MEMORY_BLOCK_SIZE, MIN_MEMORY_BLOCK_SIZE);
MM_ULONG(PAGES_PER_SECTION, PAGES_PER_SECTION);
#ifdef CONFIG_X86_64
MM_ULONG(KERNEL_IMAGE_SIZE, KERNEL_IMAGE_SIZE);
#endif
#ifdef CONFIG_EXECMEM
MM_ULONG(MODULE_ALIGN, MODULE_ALIGN);
MM_UINT(INT3_INSN_OPCODE, INT3_INSN_OPCODE);
#endif
#undef MM_ULONG
#undef MM_UINT
#endif /* __BINDGEN__ */

#define MM_PRIMITIVE(ret, name, args, expression) ret rust_mm_##name args;
#define MM_VOID(name, args, expression) void rust_mm_##name args;
#define MM_REF_PRIMITIVE(ret, name, args, expression) ret __ref rust_mm_##name args;
#define MM_INIT_PRIMITIVE(ret, name, args, expression) ret __init rust_mm_##name args;
#define MM_INIT_VOID(name, args, expression) void __init rust_mm_##name args;
#include "init_primitives.def"
#undef MM_REF_PRIMITIVE
#undef MM_INIT_PRIMITIVE
#undef MM_INIT_VOID
#undef MM_PRIMITIVE
#undef MM_VOID
void __noreturn rust_mm_bug(void);
void rust_mm_vm_bug_on(bool condition);
bool rust_mm_warn_free_init_alignment(bool condition);
bool rust_mm_warn_prot_pat(bool condition);
bool rust_mm_warn_add_pages_end(bool condition);
bool rust_mm_warn_add_pages_ret(bool condition);
#ifdef CONFIG_DEBUG_VM
void rust_mm_warn_check_pgprot(pgprotval_t original, pgprotval_t unsupported, pgprotval_t supported);
#endif
void rust_mm_assert_folded_pgd(pgd_t *pgd);
void rust_mm_assert_no_p4d_leaf(p4d_t *p4d);
void rust_mm_debug_range(unsigned long start, unsigned long end, const char *size);
void rust_mm_debug_init_mapping(unsigned long start, unsigned long end);
void rust_mm_debug_spp_getpage(void *p);
void rust_mm_debug_set_pte_vaddr(unsigned long addr, pteval_t value);
void rust_mm_debug_vmemmap_block(unsigned long start, unsigned long end, void *p, void *last, int node);
void rust_mm_debug_vmemmap_last(unsigned long start, unsigned long end, void *p, void *last, int node);
void rust_mm_error_altmap_unsupported(void);
#endif
