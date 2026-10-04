// SPDX-License-Identifier: GPL-2.0-only
/* Native header, compiler, tracepoint, per-CPU and registration primitives.
 * No algorithm or function body from init.c / init_64.c is included here.
 */
#include "init_bindings.h"

/* The original init.c is also the native TLB tracepoint-definition owner. */
#define CREATE_TRACE_POINTS
#include <trace/events/tlb.h>

#define MM_PRIMITIVE(ret, name, args, expression) \
	ret rust_mm_##name args { return (expression); }
#define MM_VOID(name, args, expression) \
	void rust_mm_##name args { expression; }
#define MM_REF_PRIMITIVE(ret, name, args, expression) \
	ret __ref rust_mm_##name args { return (expression); }
#define MM_INIT_PRIMITIVE(ret, name, args, expression) \
	ret __init rust_mm_##name args { return (expression); }
#define MM_INIT_VOID(name, args, expression) \
	void __init rust_mm_##name args { expression; }
#include "init_primitives.def"
#undef MM_REF_PRIMITIVE
#undef MM_INIT_PRIMITIVE
#undef MM_INIT_VOID
#undef MM_PRIMITIVE
#undef MM_VOID

void __noreturn rust_mm_bug(void) { BUG(); unreachable(); }
void rust_mm_vm_bug_on(bool condition) { VM_BUG_ON(condition); }
bool rust_mm_warn_free_init_alignment(bool condition) { return WARN_ON(condition); }
bool rust_mm_warn_prot_pat(bool condition) { return WARN_ON_ONCE(condition); }
bool rust_mm_warn_add_pages_end(bool condition) { return WARN_ON_ONCE(condition); }
bool rust_mm_warn_add_pages_ret(bool condition) { return WARN_ON_ONCE(condition); }
void rust_mm_assert_folded_pgd(pgd_t *pgd) { MAYBE_BUILD_BUG_ON(pgd_none(*pgd)); }
void rust_mm_assert_no_p4d_leaf(p4d_t *p4d) { BUILD_BUG_ON(p4d_leaf(*p4d)); }

void rust_mm_debug_range(unsigned long start, unsigned long end, const char *size)
{ pr_debug(" [mem %#010lx-%#010lx] page %s\n", start, end, size); }
void rust_mm_debug_init_mapping(unsigned long start, unsigned long end)
{ pr_debug("init_memory_mapping: [mem %#010lx-%#010lx]\n", start, end); }
void rust_mm_debug_spp_getpage(void *p) { pr_debug("spp_getpage %p\n", p); }
void rust_mm_debug_set_pte_vaddr(unsigned long addr, pteval_t value)
{ pr_debug("set_pte_vaddr %lx to %lx\n", addr, (unsigned long)value); }
void rust_mm_debug_vmemmap_block(unsigned long start, unsigned long end,
			       void *p, void *last, int node)
{ pr_debug(" [%lx-%lx] PMD -> [%p-%p] on node %d\n", start, end, p, last, node); }
void rust_mm_debug_vmemmap_last(unsigned long start, unsigned long end, void *p, void *last, int node)
{ pr_debug(" [%lx-%lx] PMD -> [%p-%p] on node %d\n", start, end, p, last, node); }
void rust_mm_error_altmap_unsupported(void)
{ pr_err_once("%s: no cpu support for altmap allocations\n", "vmemmap_populate"); }

extern int __init rust_mm_parse_gbpages_on(char *arg);
extern int __init rust_mm_parse_gbpages_off(char *arg);
early_param("gbpages", rust_mm_parse_gbpages_on);
early_param("nogbpages", rust_mm_parse_gbpages_off);

#ifdef CONFIG_ADDRESS_MASKING
EXPORT_PER_CPU_SYMBOL(tlbstate_untag_mask);
#endif
EXPORT_SYMBOL(cachemode2protval);
#ifdef CONFIG_X86_64
extern int __init rust_mm_nonx32_setup(char *str);
__setup("noexec32=", rust_mm_nonx32_setup);
EXPORT_SYMBOL_GPL(__supported_pte_mask);
EXPORT_SYMBOL(__default_kernel_pte_mask);
#endif
