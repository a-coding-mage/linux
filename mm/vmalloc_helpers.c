// SPDX-License-Identifier: GPL-2.0-only
/* Native header, architecture, compiler diagnostics and registration leaves.
 * No vmalloc.c page-table walk, virtual-range allocator, purge, block allocator,
 * or public allocation algorithm is implemented in this translation unit. */
#include "vmalloc_bindings.h"
#define CREATE_TRACE_POINTS
#include <trace/events/vmalloc.h>
#define RVM_LEAF(ret, name, args, expression) ret rust_vmalloc_##name args { return ({ expression; }); }
#include "vmalloc_primitives.inc"
#undef RVM_LEAF

#ifdef CONFIG_HAVE_ARCH_HUGE_VMAP
extern int rust_vmalloc_set_nohugeiomap(char *);
early_param("nohugeiomap", rust_vmalloc_set_nohugeiomap);
#endif
#ifdef CONFIG_HAVE_ARCH_HUGE_VMALLOC
extern int rust_vmalloc_set_nohugevmalloc(char *);
early_param("nohugevmalloc", rust_vmalloc_set_nohugevmalloc);
#endif
#if defined(CONFIG_EXECMEM) && defined(MODULES_VADDR)
unsigned long rust_vmalloc_module_range_start(void) { return MODULES_VADDR; }
unsigned long rust_vmalloc_module_range_end(void) { return MODULES_END; }
#else
/* No architecture module range exists. Equal endpoints encode the empty range. */
unsigned long rust_vmalloc_module_range_start(void) { return 0; }
unsigned long rust_vmalloc_module_range_end(void) { return 0; }
#endif
/* These native initializers carry per-CPU relocations, lockdep keys and work
 * metadata. The Rust owner performs every operation on the stored objects. */
static DEFINE_SPINLOCK(free_vmap_area_lock);
static DEFINE_MUTEX(vmap_purge_lock);
static BLOCKING_NOTIFIER_HEAD(vmap_notify_list);
static DEFINE_PER_CPU(struct vmap_area *, ne_fit_preload_node);
static __cacheline_aligned_in_smp atomic_long_t vmap_lazy_nr;
extern void rust_vmalloc_drain_vmap_area_work(struct work_struct *);
static DECLARE_WORK(drain_vmap_work, rust_vmalloc_drain_vmap_area_work);
spinlock_t *rust_vmalloc_free_lock(void) { return &free_vmap_area_lock; }
void rust_vmalloc_purge_lock(void) { mutex_lock(&vmap_purge_lock); }
void rust_vmalloc_purge_unlock(void) { mutex_unlock(&vmap_purge_lock); }
void rust_vmalloc_purge_assert_held(void) { lockdep_assert_held(&vmap_purge_lock); }
struct blocking_notifier_head *rust_vmalloc_notifier(void) { return &vmap_notify_list; }
struct work_struct *rust_vmalloc_drain_work(void) { return &drain_vmap_work; }
struct vmap_area *rust_vmalloc_preload_read(void) { return this_cpu_read(ne_fit_preload_node); }
struct vmap_area *rust_vmalloc_preload_xchg(void) { return __this_cpu_xchg(ne_fit_preload_node,NULL); }
bool rust_vmalloc_preload_cmpxchg(struct vmap_area **old, struct vmap_area *new) { return __this_cpu_try_cmpxchg(ne_fit_preload_node,old,new); }
long rust_vmalloc_lazy_read(void) { return atomic_long_read(&vmap_lazy_nr); }
long rust_vmalloc_lazy_add_return(long n) { return atomic_long_add_return_relaxed(n,&vmap_lazy_nr); }
void rust_vmalloc_lazy_sub(long n) { atomic_long_sub(n,&vmap_lazy_nr); }
void rust_vmalloc_warn_node(struct vmap_node *n) { WARN_ONCE(1,"An address 0x%p is out-of-bounds.\n",n); }
void rust_vmalloc_warn_encode(unsigned int n) { WARN_ONCE(1,"Encode wrong node id (%u)\n",n); }
void rust_vmalloc_warn_decode(unsigned int n) { WARN_ONCE(n != UINT_MAX,"Decode wrong node id (%d)\n",n); }
void rust_vmalloc_warn_overlap(struct vmap_area *a, struct vmap_area *b) { WARN(1,"vmalloc bug: 0x%lx-0x%lx overlaps with 0x%lx-0x%lx\n",a->va_start,a->va_end,b->va_start,b->va_end); }
void rust_vmalloc_warn_range(unsigned long size, unsigned long start, unsigned long end) { pr_warn("vmalloc_node_range for size %lu failed: Address range restricted to %#lx - %#lx\n",size,start,end); }
extern void rust_vmalloc_purge_vmap_node(struct work_struct *);
void rust_vmalloc_init_purge_work(struct work_struct *w) { INIT_WORK(w, rust_vmalloc_purge_vmap_node); }
static DEFINE_PER_CPU(struct vmap_block_queue, vmap_block_queue);
static DEFINE_PER_CPU(struct vfree_deferred, vfree_deferred);
struct vmap_block_queue *rust_vmalloc_vbq(int cpu) { return &per_cpu(vmap_block_queue,cpu); }
struct vmap_block_queue *rust_vmalloc_raw_vbq(void) { return raw_cpu_ptr(&vmap_block_queue); }
void rust_vmalloc_vb_lock_init(struct vmap_block *vb) { spin_lock_init(&vb->lock); }
void rust_vmalloc_free_vb_rcu(struct vmap_block *vb) { kfree_rcu(vb,rcu_head); }
struct vfree_deferred *rust_vmalloc_deferred_cpu(int cpu) { return &per_cpu(vfree_deferred,cpu); }
struct vfree_deferred *rust_vmalloc_deferred_raw(void) { return raw_cpu_ptr(&vfree_deferred); }
extern void rust_vmalloc_delayed_vfree_work(struct work_struct *);
void rust_vmalloc_init_deferred_work(struct vfree_deferred *p) { INIT_WORK(&p->wq,rust_vmalloc_delayed_vfree_work); }
bool rust_vmalloc_warn_vfree_align(const void *p) { return WARN(!PAGE_ALIGNED(p), "Trying to vfree() bad address (%p)\n",p); }
void rust_vmalloc_warn_vfree_missing(const void *p) { WARN(1,KERN_ERR "Trying to vfree() nonexistent vm area (%p)\n",p); }
void rust_vmalloc_warn_vunmap_missing(const void *p) { WARN(1,KERN_ERR "Trying to vunmap() nonexistent vm area (%p)\n",p); }
void rust_vmalloc_warn_array(gfp_t g, unsigned long nr, unsigned long size) { warn_alloc(g,NULL,"vmalloc error: size %lu, failed to allocated page array size %lu",nr,size); }
void rust_vmalloc_warn_pages(gfp_t g, unsigned long size) { warn_alloc(g,NULL,"vmalloc error: size %lu, failed to allocate pages",size); }
void rust_vmalloc_warn_map(gfp_t g, unsigned long size) { warn_alloc(g,NULL,"vmalloc error: size %lu, failed to map pages",size); }
void rust_vmalloc_warn_total(gfp_t g, unsigned long size) { warn_alloc(g,NULL,"vmalloc error: size %lu, exceeds total pages",size); }
void rust_vmalloc_warn_area(gfp_t g, unsigned long size, unsigned long align, bool nofail) { warn_alloc(g,NULL,"vmalloc error: size %lu, align 0x%lx, vm_struct allocation failed%s",size,align,nofail ? ". Retrying." : ""); }
void rust_vmalloc_warn_gfp(gfp_t old, gfp_t new) { WARN_ONCE(1,"Unexpected gfp: %#x (%pGg). Fixing up to gfp: %#x (%pGg). Fix your code!\n",old,&old,new,&new); }
extern void rust_vmalloc_cleanup_vm_area_work(struct work_struct *);
static DECLARE_WORK(cleanup_vm_area, rust_vmalloc_cleanup_vm_area_work);
struct work_struct *rust_vmalloc_cleanup_work(void) { return &cleanup_vm_area; }
/* Compiler return-address leaves. The bodies and all decisions are Rust. */
extern struct vm_struct *rust_vmalloc_get_vm_area(unsigned long,unsigned long,const void *);
struct vm_struct *get_vm_area(unsigned long s,unsigned long f) { return rust_vmalloc_get_vm_area(s,f,__builtin_return_address(0)); }
extern void *rust_vmalloc_vmap(struct page **,unsigned int,unsigned long,pgprot_t,const void *);
void *vmap(struct page **p,unsigned int n,unsigned long f,pgprot_t prot) { return rust_vmalloc_vmap(p,n,f,prot,__builtin_return_address(0)); }
#ifdef CONFIG_VMAP_PFN
extern void *rust_vmalloc_vmap_pfn(unsigned long *,unsigned int,pgprot_t,const void *);
void *vmap_pfn(unsigned long *p,unsigned int n,pgprot_t prot) { return rust_vmalloc_vmap_pfn(p,n,prot,__builtin_return_address(0)); }
#endif
void rust_vmalloc_warn_vrealloc_missing(const void *p) { WARN(1,"Trying to vrealloc() nonexistent vm area (%p)\n",p); }
bool rust_vmalloc_warn_vrealloc_size(bool bad,const void *p) { return WARN(bad,"vrealloc() has mismatched area vs requested sizes (%p)\n",p); }
bool rust_vmalloc_warn_vrealloc_align(bool bad,unsigned long align) { return WARN(bad,"will not reallocate with a bigger alignment (0x%lx)\n",align); }
extern void *rust_vmalloc___vmalloc_noprof(unsigned long,gfp_t,const void *);
void *__vmalloc_noprof(unsigned long s,gfp_t g) { return rust_vmalloc___vmalloc_noprof(s,g,__builtin_return_address(0)); }
extern void *rust_vmalloc_vmalloc_noprof(unsigned long,const void *);
void *vmalloc_noprof(unsigned long s) { return rust_vmalloc_vmalloc_noprof(s,__builtin_return_address(0)); }
extern void *rust_vmalloc_vmalloc_huge_node_noprof(unsigned long,gfp_t,int,const void *);
void *vmalloc_huge_node_noprof(unsigned long s,gfp_t g,int n) { return rust_vmalloc_vmalloc_huge_node_noprof(s,g,n,__builtin_return_address(0)); }
extern void *rust_vmalloc_vzalloc_noprof(unsigned long,const void *);
void *vzalloc_noprof(unsigned long s) { return rust_vmalloc_vzalloc_noprof(s,__builtin_return_address(0)); }
extern void *rust_vmalloc_vmalloc_user_noprof(unsigned long,const void *);
void *vmalloc_user_noprof(unsigned long s) { return rust_vmalloc_vmalloc_user_noprof(s,__builtin_return_address(0)); }
extern void *rust_vmalloc_vmalloc_node_noprof(unsigned long,int,const void *);
void *vmalloc_node_noprof(unsigned long s,int n) { return rust_vmalloc_vmalloc_node_noprof(s,n,__builtin_return_address(0)); }
extern void *rust_vmalloc_vzalloc_node_noprof(unsigned long,int,const void *);
void *vzalloc_node_noprof(unsigned long s,int n) { return rust_vmalloc_vzalloc_node_noprof(s,n,__builtin_return_address(0)); }
extern void *rust_vmalloc_vrealloc_node_align_noprof(const void *,size_t,unsigned long,gfp_t,int,const void *);
void *vrealloc_node_align_noprof(const void *p,size_t s,unsigned long a,gfp_t g,int n) { return rust_vmalloc_vrealloc_node_align_noprof(p,s,a,g,n,__builtin_return_address(0)); }
extern void *rust_vmalloc_vmalloc_32_noprof(unsigned long,const void *);
void *vmalloc_32_noprof(unsigned long s) { return rust_vmalloc_vmalloc_32_noprof(s,__builtin_return_address(0)); }
extern void *rust_vmalloc_vmalloc_32_user_noprof(unsigned long,const void *);
void *vmalloc_32_user_noprof(unsigned long s) { return rust_vmalloc_vmalloc_32_user_noprof(s,__builtin_return_address(0)); }
void rust_vmalloc_busy_lock_init(struct vmap_node *n) { spin_lock_init(&n->busy.lock); }
void rust_vmalloc_lazy_lock_init(struct vmap_node *n) { spin_lock_init(&n->lazy.lock); }
void rust_vmalloc_pool_lock_init(struct vmap_node *n) { spin_lock_init(&n->pool_lock); }
void rust_vmalloc_vbq_lock_init(struct vmap_block_queue *q) { spin_lock_init(&q->lock); }
struct kmem_cache *rust_vmalloc_create_area_cache(void) { return KMEM_CACHE(vmap_area,SLAB_PANIC); }
void rust_vmalloc_warn_nodes(void) { pr_err("Failed to allocate an array. Disable a node layer\n"); }
void rust_vmalloc_warn_shrinker(void) { pr_err("Failed to allocate vmap-node shrinker!\n"); }
void rust_vmalloc_dump_region(unsigned long n,unsigned long a,const void *caller) { pr_cont(" %lu-page vmalloc region starting at %#lx allocated at %pS\n",n,a,caller); }
#ifdef CONFIG_PROC_FS
extern int rust_vmalloc_proc_vmalloc_init(void);
module_init(rust_vmalloc_proc_vmalloc_init);
#endif
/* Export records retain the original namespace and GPL restrictions. */
EXPORT_SYMBOL(is_vmalloc_addr);
EXPORT_SYMBOL_GPL(is_vmalloc_or_module_addr);
EXPORT_SYMBOL(vmalloc_to_page);
EXPORT_SYMBOL(vmalloc_to_pfn);
EXPORT_SYMBOL_GPL(register_vmap_purge_notifier);
EXPORT_SYMBOL_GPL(unregister_vmap_purge_notifier);
EXPORT_SYMBOL_GPL(vm_unmap_aliases);
EXPORT_SYMBOL(vm_unmap_ram);
EXPORT_SYMBOL(vm_map_ram);
EXPORT_SYMBOL(vfree);
EXPORT_SYMBOL(vunmap);
EXPORT_SYMBOL(vmap);
#ifdef CONFIG_VMAP_PFN
EXPORT_SYMBOL_GPL(vmap_pfn);
#endif
#ifdef CONFIG_TEST_VMALLOC_MODULE
EXPORT_SYMBOL_GPL(__vmalloc_node_noprof);
#endif
EXPORT_SYMBOL(__vmalloc_noprof);
EXPORT_SYMBOL(vmalloc_noprof);
EXPORT_SYMBOL_GPL(vmalloc_huge_node_noprof);
EXPORT_SYMBOL(vzalloc_noprof);
EXPORT_SYMBOL(vmalloc_user_noprof);
EXPORT_SYMBOL(vmalloc_node_noprof);
EXPORT_SYMBOL(vzalloc_node_noprof);
EXPORT_SYMBOL(vrealloc_node_align_noprof);
EXPORT_SYMBOL(vmalloc_32_noprof);
EXPORT_SYMBOL(vmalloc_32_user_noprof);
EXPORT_SYMBOL(remap_vmalloc_range);
EXPORT_SYMBOL_GPL(free_vm_area);
void rust_vmalloc_debug_tree(unsigned long size,unsigned long subtree) { pr_emerg("tree is corrupted: %lu, %lu\n",size,subtree); }
void rust_vmalloc_debug_lowest(struct vmap_area *tree,struct vmap_area *linear,unsigned long start) { pr_emerg("not lowest: t: 0x%p, l: 0x%p, v: 0x%lx\n",tree,linear,start); }
