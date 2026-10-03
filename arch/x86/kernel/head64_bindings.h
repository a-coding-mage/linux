/* SPDX-License-Identifier: GPL-2.0 */
/* Canonical configured types/constant expressions only; no replacement layouts. */
#ifndef RUST_HEAD64_BINDINGS_H
#define RUST_HEAD64_BINDINGS_H
#define USE_EARLY_PGTABLE_L5
#include <linux/init.h>
#include <linux/linkage.h>
#include <linux/types.h>
#include <linux/kernel.h>
#include <linux/string.h>
#include <linux/percpu.h>
#include <linux/start_kernel.h>
#include <linux/io.h>
#include <linux/memblock.h>
#include <linux/cc_platform.h>
#include <linux/pgtable.h>
#include <asm/asm.h>
#include <asm/page_64.h>
#include <asm/processor.h>
#include <asm/proto.h>
#include <asm/smp.h>
#include <asm/setup.h>
#include <asm/desc.h>
#include <asm/tlbflush.h>
#include <asm/sections.h>
#include <asm/kdebug.h>
#include <asm/e820/api.h>
#include <asm/bios_ebda.h>
#include <asm/microcode.h>
#include <asm/kasan.h>
#include <asm/fixmap.h>
#include <asm/realmode.h>
#include <asm/extable.h>
#include <asm/trapnr.h>
#include <asm/sev.h>
#include <asm/tdx.h>
#include <asm/init.h>

extern pmd_t early_dynamic_pgts[EARLY_DYNAMIC_PAGE_TABLES][PTRS_PER_PMD];
/* Complete canonical declarations for otherwise incomplete extern arrays. */
extern pgd_t init_top_pgt[PTRS_PER_PGD];
extern char boot_command_line[COMMAND_LINE_SIZE];

#ifdef __BINDGEN__
enum rust_head64_constants {
#define RUST_HEAD64_CONST(name, value) RUST_HEAD64_##name = (value),
RUST_HEAD64_CONST(START_KERNEL_MAP, __START_KERNEL_map)
RUST_HEAD64_CONST(START_KERNEL, __START_KERNEL)
RUST_HEAD64_CONST(PAGE_OFFSET_L4, __PAGE_OFFSET_BASE_L4)
RUST_HEAD64_CONST(PAGE_OFFSET_L5, __PAGE_OFFSET_BASE_L5)
RUST_HEAD64_CONST(VMALLOC_L4, __VMALLOC_BASE_L4)
RUST_HEAD64_CONST(VMALLOC_L5, __VMALLOC_BASE_L5)
RUST_HEAD64_CONST(VMEMMAP_L4, __VMEMMAP_BASE_L4)
RUST_HEAD64_CONST(VMEMMAP_L5, __VMEMMAP_BASE_L5)
RUST_HEAD64_CONST(EARLY_PMD_FLAGS, __PAGE_KERNEL_LARGE & ~(_PAGE_GLOBAL | _PAGE_NX))
RUST_HEAD64_CONST(KERNPG_TABLE_NOENC, _KERNPG_TABLE_NOENC)
RUST_HEAD64_CONST(PAGE_MASK, PAGE_MASK)
RUST_HEAD64_CONST(PHYSICAL_MASK_SHIFT, __PHYSICAL_MASK_SHIFT)
RUST_HEAD64_CONST(PMD_MASK, PMD_MASK)
RUST_HEAD64_CONST(PMD_SHIFT, PMD_SHIFT)
RUST_HEAD64_CONST(PUD_SHIFT, PUD_SHIFT)
RUST_HEAD64_CONST(P4D_SHIFT, P4D_SHIFT)
RUST_HEAD64_CONST(PUD_SIZE, PUD_SIZE)
RUST_HEAD64_CONST(PTRS_PER_PGD, PTRS_PER_PGD)
RUST_HEAD64_CONST(PTRS_PER_PUD, PTRS_PER_PUD)
RUST_HEAD64_CONST(PTRS_PER_PMD, PTRS_PER_PMD)
RUST_HEAD64_CONST(EARLY_DYNAMIC_PAGE_TABLES, EARLY_DYNAMIC_PAGE_TABLES)
RUST_HEAD64_CONST(COMMAND_LINE_SIZE, COMMAND_LINE_SIZE)
RUST_HEAD64_CONST(MODULES_VADDR, MODULES_VADDR)
RUST_HEAD64_CONST(MODULES_END, MODULES_END)
RUST_HEAD64_CONST(MODULES_LEN, MODULES_LEN)
RUST_HEAD64_CONST(KERNEL_IMAGE_SIZE, KERNEL_IMAGE_SIZE)
RUST_HEAD64_CONST(FIXMAP_END, __fix_to_virt(__end_of_fixed_addresses))
RUST_HEAD64_CONST(CR4_PGE, X86_CR4_PGE)
RUST_HEAD64_CONST(SUBARCH_INTEL_MID, X86_SUBARCH_INTEL_MID)
/* Evaluate the dynamic MAXMEM macro in both early paging modes. */
#define pgtable_l5_enabled() 0
RUST_HEAD64_CONST(MAXMEM_L4, MAXMEM)
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 1
RUST_HEAD64_CONST(MAXMEM_L5, MAXMEM)
#undef pgtable_l5_enabled
#undef RUST_HEAD64_CONST
};
#endif

/* Only macro/architecture instruction boundaries; bodies are in primitives.c. */
unsigned long rust_head64_read_cr3(void);
void rust_head64_write_cr3(unsigned long value);
unsigned long rust_head64_this_cpu_cr4(void);
void rust_head64_this_cpu_write_cr4(unsigned long value);
void rust_head64_clear_page(void *page);
void __noreturn rust_head64_bug(void);
#endif
