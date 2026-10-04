// SPDX-License-Identifier: GPL-2.0-or-later
/* Compiler/macro and header-inline boundaries only. No memblock region,
 * allocation, reservation, iteration, KHO or release algorithms live here. */
#include "memblock_bindings.h"

bool rust_memblock_warn(bool condition) { return WARN_ON(condition); }
void rust_memblock_bug(bool condition) { BUG_ON(condition); }
bool rust_memblock_warn_slab_once(bool condition) { return WARN_ON_ONCE(condition); }
void rust_memblock_warn_deferred_free(void)
{
 WARN(1, "Cannot free reserved memory because of deferred initialization of the memory map");
}
void rust_memblock_warn_mirror_find(phys_addr_t size)
{
 pr_warn_ratelimited("Could not allocate %pap bytes of mirrored memory\n", &size);
}
void rust_memblock_warn_mirror_alloc(phys_addr_t size)
{
 pr_warn_ratelimited("Could not allocate %pap bytes of mirrored memory\n", &size);
}
void rust_memblock_dump_stack(void) { dump_stack(); }
bool rust_memblock_slab_available(void) { return slab_is_available(); }
bool rust_memblock_movable_node(void) { return movable_node_is_enabled(); }
bool rust_memblock_deferred_pages_enabled(void) { return deferred_pages_enabled(); }
void *rust_memblock_kmalloc(size_t size) { return kmalloc(size, GFP_KERNEL); }
void *rust_memblock_kzalloc_node(size_t size, int nid) { return kzalloc_node(size, GFP_NOWAIT, nid); }
phys_addr_t rust_memblock_pa(const void *ptr) { return __pa(ptr); }
phys_addr_t rust_memblock_pa_symbol(const void *ptr) { return __pa_symbol(ptr); }
void *rust_memblock_va(phys_addr_t addr) { return __va(addr); }
phys_addr_t rust_memblock_virt_to_phys(void *ptr) { return virt_to_phys(ptr); }
void *rust_memblock_phys_to_virt(phys_addr_t addr) { return phys_to_virt(addr); }
bool rust_memblock_is_kernel(unsigned long addr) { return __is_kernel(addr); }
void rust_memblock_accept_memory(phys_addr_t start, unsigned long size) { accept_memory(start, size); }
void rust_memblock_kmemleak_alloc(phys_addr_t addr, size_t size) { kmemleak_alloc_phys(addr, size, 0); }
void rust_memblock_kmemleak_free(phys_addr_t addr, size_t size) { kmemleak_free_part_phys(addr, size); }
struct page *rust_memblock_pfn_to_page(unsigned long pfn) { return pfn_to_page(pfn); }
void *rust_memblock_page_address(struct page *page) { return page_address(page); }
void *rust_memblock_kasan_reset_tag(const void *ptr) { return kasan_reset_tag(ptr); }
void rust_memblock_free_reserved_page(struct page *page) { free_reserved_page(page); }
void rust_memblock_set_page_reserved(struct page *page) { __SetPageReserved(page); }
#if defined(CONFIG_SPARSEMEM) && !defined(CONFIG_HAVE_ARCH_PFN_VALID)
unsigned long rust_memblock_first_valid_pfn(unsigned long pfn, unsigned long end)
{
 return first_valid_pfn(pfn, end);
}
unsigned long rust_memblock_next_valid_pfn(unsigned long pfn, unsigned long end)
{
 return next_valid_pfn(pfn, end);
}
#else
bool rust_memblock_pfn_valid(unsigned long pfn) { return pfn_valid(pfn); }
#endif
unsigned long rust_memblock_pageblock_start(unsigned long pfn) { return pageblock_start_pfn(pfn); }
unsigned long rust_memblock_pageblock_align(unsigned long pfn) { return pageblock_align(pfn); }
int rust_memblock_early_pfn_to_nid(unsigned long pfn) { return early_pfn_to_nid(pfn); }
void rust_memblock_atomic_long_set(atomic_long_t *ptr, long value) { atomic_long_set(ptr, value); }
void rust_memblock_totalram_pages_add(unsigned long pages) { totalram_pages_add(pages); }
bool rust_memblock_kho_scratch_overlap(phys_addr_t start, size_t size) { return kho_scratch_overlap(start, size); }

/* Native static initializer owns the lockdep key and self-referential lists. */
static DEFINE_MUTEX(reserve_mem_lock);
void rust_memblock_reserve_lock(void) { mutex_lock(&reserve_mem_lock); }
void rust_memblock_reserve_unlock(void) { mutex_unlock(&reserve_mem_lock); }

#ifdef CONFIG_KEXEC_HANDOVER
struct page *rust_memblock_phys_to_page(phys_addr_t addr) { return phys_to_page(addr); }
struct page *rust_memblock_alloc_page(void) { return alloc_page(GFP_KERNEL); }
void *rust_memblock_page_to_virt(struct page *page) { return page_to_virt(page); }
void rust_memblock_put_page(struct page *page) { put_page(page); }
u32 rust_memblock_fdt_totalsize(const void *fdt) { return fdt_totalsize(fdt); }
extern int rust_memblock_reserve_mem_init(void);
late_initcall(rust_memblock_reserve_mem_init);
#endif

extern int rust_memblock_early_param(char *p);
early_param("memblock", rust_memblock_early_param);
extern int rust_memblock_reserve_mem(char *p);
__setup("reserve_mem=", rust_memblock_reserve_mem);

#ifndef CONFIG_NUMA
EXPORT_SYMBOL(contig_page_data);
#endif
EXPORT_SYMBOL_GPL(reserve_mem_find_by_name);

#ifdef CONFIG_DEBUG_FS
extern int rust_memblock_reserve_mem_show(struct seq_file *, void *);
static int memblock_reserve_mem_show(struct seq_file *m, void *private)
{
 return rust_memblock_reserve_mem_show(m, private);
}
DEFINE_SHOW_ATTRIBUTE(memblock_reserve_mem);
void rust_memblock_debugfs_reservation_file(struct dentry *root)
{
 debugfs_create_file("reserve_mem_param", 0444, root, NULL, &memblock_reserve_mem_fops);
}
#ifdef CONFIG_ARCH_KEEP_MEMBLOCK
extern int rust_memblock_debug_show(struct seq_file *, void *);
static int memblock_debug_show(struct seq_file *m, void *private)
{
 return rust_memblock_debug_show(m, private);
}
DEFINE_SHOW_ATTRIBUTE(memblock_debug);
void rust_memblock_debugfs_array_file(const char *name, struct dentry *root, struct memblock_type *type)
{
 debugfs_create_file(name, 0444, root, type, &memblock_debug_fops);
}
#endif
extern int rust_memblock_init_debugfs(void);
__initcall(rust_memblock_init_debugfs);
#endif

/* _RET_IP_ is a compiler builtin with no portable Rust equivalent. These
 * unconditional ABI trampolines capture the original C caller before entering
 * the Rust body. They perform no region operation or allocation decision. */
extern int rust_memblock_add_node(phys_addr_t, phys_addr_t, int, enum memblock_flags, const void *);
int __init_memblock memblock_add_node(phys_addr_t base, phys_addr_t size, int nid, enum memblock_flags flags)
{
 return rust_memblock_add_node(base, size, nid, flags, (const void *)_RET_IP_);
}
extern int rust_memblock_add(phys_addr_t, phys_addr_t, const void *);
int __init_memblock memblock_add(phys_addr_t base, phys_addr_t size)
{
 return rust_memblock_add(base, size, (const void *)_RET_IP_);
}
extern int rust_memblock_remove(phys_addr_t, phys_addr_t, const void *);
int __init_memblock memblock_remove(phys_addr_t base, phys_addr_t size)
{
 return rust_memblock_remove(base, size, (const void *)_RET_IP_);
}
extern int rust_memblock_phys_free(phys_addr_t, phys_addr_t, const void *);
int __init_memblock memblock_phys_free(phys_addr_t base, phys_addr_t size)
{
 return rust_memblock_phys_free(base, size, (const void *)_RET_IP_);
}
extern int rust___memblock_reserve(phys_addr_t, phys_addr_t, int, enum memblock_flags, const void *);
int __init_memblock __memblock_reserve(phys_addr_t base, phys_addr_t size, int nid, enum memblock_flags flags)
{
 return rust___memblock_reserve(base, size, nid, flags, (const void *)_RET_IP_);
}
#ifdef CONFIG_HAVE_MEMBLOCK_PHYS_MAP
extern int rust_memblock_physmem_add(phys_addr_t, phys_addr_t, const void *);
int __init_memblock memblock_physmem_add(phys_addr_t base, phys_addr_t size)
{
 return rust_memblock_physmem_add(base, size, (const void *)_RET_IP_);
}
#endif
extern phys_addr_t rust_memblock_phys_alloc_range(phys_addr_t, phys_addr_t, phys_addr_t, phys_addr_t, const void *);
phys_addr_t __init memblock_phys_alloc_range(phys_addr_t size, phys_addr_t align, phys_addr_t start, phys_addr_t end)
{
 return rust_memblock_phys_alloc_range(size, align, start, end, (const void *)_RET_IP_);
}
extern void *rust_memblock_alloc_exact_nid_raw(phys_addr_t, phys_addr_t, phys_addr_t, phys_addr_t, int, const void *);
void *__init memblock_alloc_exact_nid_raw(phys_addr_t size, phys_addr_t align, phys_addr_t min_addr, phys_addr_t max_addr, int nid)
{
 return rust_memblock_alloc_exact_nid_raw(size, align, min_addr, max_addr, nid, (const void *)_RET_IP_);
}
extern void *rust_memblock_alloc_try_nid_raw(phys_addr_t, phys_addr_t, phys_addr_t, phys_addr_t, int, const void *);
void *__init memblock_alloc_try_nid_raw(phys_addr_t size, phys_addr_t align, phys_addr_t min_addr, phys_addr_t max_addr, int nid)
{
 return rust_memblock_alloc_try_nid_raw(size, align, min_addr, max_addr, nid, (const void *)_RET_IP_);
}
extern void *rust_memblock_alloc_hugetlb(phys_addr_t, int, bool, const void *);
void *__init memblock_alloc_hugetlb(phys_addr_t size, int nid, bool exact_nid)
{
 return rust_memblock_alloc_hugetlb(size, nid, exact_nid, (const void *)_RET_IP_);
}
extern void *rust_memblock_alloc_try_nid(phys_addr_t, phys_addr_t, phys_addr_t, phys_addr_t, int, const void *);
void *__init memblock_alloc_try_nid(phys_addr_t size, phys_addr_t align, phys_addr_t min_addr, phys_addr_t max_addr, int nid)
{
 return rust_memblock_alloc_try_nid(size, align, min_addr, max_addr, nid, (const void *)_RET_IP_);
}

/* Preserve the native enum memblock_flags indirect-call identity. Rust's
 * scalar flag arithmetic is reached through an unconditional direct call. */
extern void rust_memblock_next_mem_range(u64 *, int, unsigned int,
 struct memblock_type *, struct memblock_type *, phys_addr_t *, phys_addr_t *, int *);
void __next_mem_range(u64 *idx, int nid, enum memblock_flags flags,
 struct memblock_type *a, struct memblock_type *b, phys_addr_t *start, phys_addr_t *end, int *out_nid)
{
 rust_memblock_next_mem_range(idx, nid, flags, a, b, start, end, out_nid);
}
extern void rust_memblock_next_mem_range_rev(u64 *, int, unsigned int,
 struct memblock_type *, struct memblock_type *, phys_addr_t *, phys_addr_t *, int *);
void __init_memblock __next_mem_range_rev(u64 *idx, int nid, enum memblock_flags flags,
 struct memblock_type *a, struct memblock_type *b, phys_addr_t *start, phys_addr_t *end, int *out_nid)
{
 rust_memblock_next_mem_range_rev(idx, nid, flags, a, b, start, end, out_nid);
}
