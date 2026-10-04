// SPDX-License-Identifier: GPL-2.0
/* Native macro storage/registration and narrow header/compiler ABI leaves.
 * No original slab_common.c algorithm implementation is retained here. */
#include "slab_common_bindings.h"
#define CREATE_TRACE_POINTS
#include <trace/events/kmem.h>

LIST_HEAD(slab_caches);
DEFINE_MUTEX(slab_mutex);
extern int __init setup_slab_nomerge(char *);
extern int __init setup_slab_merge(char *);
__setup_param("slub_nomerge", slub_nomerge, setup_slab_nomerge, 0);
__setup_param("slub_merge", slub_merge, setup_slab_merge, 0);
__setup("slab_nomerge", setup_slab_nomerge);
__setup("slab_merge", setup_slab_merge);

/* _RET_IP_ requires capture at the public C ABI entry, before the Rust body. */
extern void rust_slab_common_destroy(struct kmem_cache *, void *);
void kmem_cache_destroy(struct kmem_cache *s)
{
	rust_slab_common_destroy(s, (void *)_RET_IP_);
}

#ifdef CONFIG_SLUB_DEBUG
extern void *slab_start(struct seq_file *, loff_t *);
extern void *slab_next(struct seq_file *, void *, loff_t *);
extern void slab_stop(struct seq_file *, void *);
extern int slab_show(struct seq_file *, void *);
extern int slabinfo_open(struct inode *, struct file *);
extern int __init slab_proc_init(void);
const struct seq_operations slabinfo_op = {
	.start = slab_start,
	.next = slab_next,
	.stop = slab_stop,
	.show = slab_show,
};
const struct proc_ops slabinfo_proc_ops = {
	.proc_flags = PROC_ENTRY_PERMANENT,
	.proc_open = slabinfo_open,
	.proc_read = seq_read,
	.proc_lseek = seq_lseek,
	.proc_release = seq_release,
};
module_init(slab_proc_init);
#endif
#ifdef CONFIG_BPF_SYSCALL
#include <linux/btf.h>
extern struct kmem_cache *rust_slab_common_bpf_get_kmem_cache(u64);
__bpf_kfunc_start_defs();
__bpf_kfunc struct kmem_cache *bpf_get_kmem_cache(u64 addr)
{
	return rust_slab_common_bpf_get_kmem_cache(addr);
}
__bpf_kfunc_end_defs();
#endif
#include "slab_common_rcu_storage.inc"
#define RSC_LEAF(ret, name, params, ...) ret name params __VA_ARGS__
#include "slab_common_leaves.inc"
#include "slab_common_rcu_leaves.inc"
#undef RSC_LEAF

EXPORT_SYMBOL(kmem_cache_size);
EXPORT_SYMBOL(__kmem_cache_create_args);
EXPORT_SYMBOL(kmem_buckets_create);
EXPORT_SYMBOL(kmem_cache_destroy);
EXPORT_SYMBOL(kmem_cache_shrink);
#ifdef CONFIG_PRINTK
EXPORT_SYMBOL_GPL(kmem_dump_obj);
#endif
EXPORT_SYMBOL(kmalloc_caches);
#ifdef CONFIG_KMALLOC_PARTITION_RANDOM
EXPORT_SYMBOL(random_kmalloc_seed);
#endif
EXPORT_SYMBOL(kmalloc_size_roundup);
EXPORT_SYMBOL(kfree_sensitive);
EXPORT_SYMBOL_GPL(kfree_call_rcu_nolock);
EXPORT_SYMBOL_GPL(kvfree_call_rcu);
EXPORT_TRACEPOINT_SYMBOL(kmalloc);
EXPORT_TRACEPOINT_SYMBOL(kmem_cache_alloc);
EXPORT_TRACEPOINT_SYMBOL(kfree);
EXPORT_TRACEPOINT_SYMBOL(kmem_cache_free);
