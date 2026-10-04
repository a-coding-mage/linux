/* SPDX-License-Identifier: GPL-2.0 */
/* Included by slub_helpers.c. Native ABI/callsite thunks and static kernel
 * registration data only; the allocator algorithms are Rust-owned. */
extern void *rust_slub_kmem_cache_alloc_noprof_body(struct kmem_cache *,gfp_t,unsigned long);
extern void *rust_slub_kmem_cache_alloc_lru_noprof_body(struct kmem_cache *,struct list_lru *,gfp_t,unsigned long);
extern void *rust_slub_kmem_cache_alloc_node_noprof_body(struct kmem_cache *,gfp_t,int,unsigned long);
extern void *rust_slub_kmem_cache_alloc_from_sheaf_noprof_body(struct kmem_cache *,gfp_t,struct slab_sheaf *,unsigned long);
extern void *rust_slub_kmalloc_large_noprof_body(size_t,gfp_t,unsigned long);
extern void *rust_slub_kmalloc_large_node_noprof_body(size_t,gfp_t,int,unsigned long);
extern void *rust_slub_kmalloc_node_noprof_body(size_t,kmem_buckets *,kmalloc_token_t,gfp_t,int,unsigned long);
extern void *rust_slub_kmalloc_noprof_body(size_t,kmalloc_token_t,gfp_t,unsigned long);
extern void *rust_slub_kmalloc_node_track_caller_noprof_body(size_t,kmem_buckets *,kmalloc_token_t,gfp_t,int,unsigned long);
extern void *rust_slub_kmalloc_nolock_noprof_body(size_t,kmalloc_token_t,gfp_t,int,unsigned long);
extern void *rust_slub_kmalloc_cache_noprof_body(struct kmem_cache *,gfp_t,size_t,unsigned long);
extern void *rust_slub_kmalloc_cache_node_noprof_body(struct kmem_cache *,gfp_t,int,size_t,unsigned long);
extern void *rust_slub_kmalloc_flags_noprof_body(size_t,kmalloc_token_t,gfp_t,unsigned int,int,unsigned long);
extern void rust_slub_kmem_cache_free_body(struct kmem_cache *,void *,unsigned long);
extern void rust_slub_kvfree_rcu_cb_body(struct rcu_head *,unsigned long);
extern void rust_slub_kfree_body(const void *,unsigned long);
extern void *rust_slub_krealloc_node_align_noprof_body(const void *,size_t,kmalloc_token_t,unsigned long,gfp_t,int,unsigned long);
extern void *rust_slub_kvmalloc_node_noprof_body(size_t,kmem_buckets *,kmalloc_token_t,unsigned long,gfp_t,int,unsigned long);
extern void *rust_slub_kvrealloc_node_align_noprof_body(const void *,size_t,kmalloc_token_t,unsigned long,gfp_t,int,unsigned long);
extern void rust_slub_kmem_cache_free_bulk_body(struct kmem_cache *,size_t,void **,unsigned long);
extern void rust_slub_kmem_cache_free_bulk_internal_body(struct kmem_cache *,size_t,void **,unsigned long);
extern bool rust_slub_kmem_cache_alloc_bulk_internal_body(struct kmem_cache *,gfp_t,size_t,void **,unsigned long);
extern unsigned int rust_slub_refill_objects_node_body(struct kmem_cache *,void **,gfp_t,unsigned int,unsigned int,struct kmem_cache_node *,bool,unsigned long);

void *kmem_cache_alloc_noprof(struct kmem_cache *s,gfp_t f)
{ return rust_slub_kmem_cache_alloc_noprof_body(s,f,_RET_IP_); }
void *kmem_cache_alloc_lru_noprof(struct kmem_cache *s,struct list_lru *lru,gfp_t f)
{ return rust_slub_kmem_cache_alloc_lru_noprof_body(s,lru,f,_RET_IP_); }
void *kmem_cache_alloc_node_noprof(struct kmem_cache *s,gfp_t f,int node)
{ return rust_slub_kmem_cache_alloc_node_noprof_body(s,f,node,_RET_IP_); }
void *kmem_cache_alloc_from_sheaf_noprof(struct kmem_cache *s,gfp_t f,struct slab_sheaf *sh)
{ return rust_slub_kmem_cache_alloc_from_sheaf_noprof_body(s,f,sh,_RET_IP_); }
void *__kmalloc_large_noprof(size_t n,gfp_t f)
{ return rust_slub_kmalloc_large_noprof_body(n,f,_RET_IP_); }
void *__kmalloc_large_node_noprof(size_t n,gfp_t f,int node)
{ return rust_slub_kmalloc_large_node_noprof_body(n,f,node,_RET_IP_); }
void *__kmalloc_node_noprof(DECL_KMALLOC_PARAMS(n,b,token),gfp_t f,int node)
{ return rust_slub_kmalloc_node_noprof_body(n,PASS_BUCKET_PARAM(b),PASS_TOKEN_PARAM(token),f,node,_RET_IP_); }
void *__kmalloc_noprof(DECL_TOKEN_PARAMS(n,token),gfp_t f)
{ return rust_slub_kmalloc_noprof_body(n,PASS_TOKEN_PARAM(token),f,_RET_IP_); }
void *__kmalloc_node_track_caller_noprof(DECL_KMALLOC_PARAMS(n,b,token),gfp_t f,int node,unsigned long caller)
{ return rust_slub_kmalloc_node_track_caller_noprof_body(n,PASS_BUCKET_PARAM(b),PASS_TOKEN_PARAM(token),f,node,caller); }
void *_kmalloc_nolock_noprof(DECL_TOKEN_PARAMS(n,token),gfp_t f,int node)
{ return rust_slub_kmalloc_nolock_noprof_body(n,PASS_TOKEN_PARAM(token),f,node,_RET_IP_); }
void *__kmalloc_cache_noprof(struct kmem_cache *s,gfp_t f,size_t n)
{ return rust_slub_kmalloc_cache_noprof_body(s,f,n,_RET_IP_); }
void *__kmalloc_cache_node_noprof(struct kmem_cache *s,gfp_t f,int node,size_t n)
{ return rust_slub_kmalloc_cache_node_noprof_body(s,f,node,n,_RET_IP_); }
void *__kmalloc_flags_noprof(DECL_TOKEN_PARAMS(n,token),gfp_t f,unsigned int af,int node)
{ return rust_slub_kmalloc_flags_noprof_body(n,PASS_TOKEN_PARAM(token),f,af,node,_RET_IP_); }
void kmem_cache_free(struct kmem_cache *s,void *p)
{ rust_slub_kmem_cache_free_body(s,p,_RET_IP_); }
void kvfree_rcu_cb(struct rcu_head *head)
{ rust_slub_kvfree_rcu_cb_body(head,_RET_IP_); }
void kfree(const void *p)
{ rust_slub_kfree_body(p,_RET_IP_); }
void *krealloc_node_align_noprof(const void *p,DECL_TOKEN_PARAMS(n,token),unsigned long align,gfp_t f,int node)
{ return rust_slub_krealloc_node_align_noprof_body(p,n,PASS_TOKEN_PARAM(token),align,f,node,_RET_IP_); }
void *__kvmalloc_node_noprof(DECL_KMALLOC_PARAMS(n,b,token),unsigned long align,gfp_t f,int node)
{ return rust_slub_kvmalloc_node_noprof_body(n,PASS_BUCKET_PARAM(b),PASS_TOKEN_PARAM(token),align,f,node,_RET_IP_); }
void *kvrealloc_node_align_noprof(const void *p,DECL_TOKEN_PARAMS(n,token),unsigned long align,gfp_t f,int node)
{ return rust_slub_kvrealloc_node_align_noprof_body(p,n,PASS_TOKEN_PARAM(token),align,f,node,_RET_IP_); }
void kmem_cache_free_bulk(struct kmem_cache *s,size_t n,void **p)
{ rust_slub_kmem_cache_free_bulk_body(s,n,p,_RET_IP_); }
void __kmem_cache_free_bulk(struct kmem_cache *s,size_t n,void **p)
{ rust_slub_kmem_cache_free_bulk_internal_body(s,n,p,_RET_IP_); }
bool __kmem_cache_alloc_bulk(struct kmem_cache *s,gfp_t f,size_t n,void **p)
{ return rust_slub_kmem_cache_alloc_bulk_internal_body(s,f,n,p,_RET_IP_); }
unsigned int __refill_objects_node(struct kmem_cache *s,void **p,gfp_t f,unsigned int min,unsigned int max,struct kmem_cache_node *node,bool spin)
{ return rust_slub_refill_objects_node_body(s,p,f,min,max,node,spin,_RET_IP_); }
#ifdef CONFIG_MEMCG
extern void rust_slub_memcg_alloc_abort_single_body(struct kmem_cache *,void *,unsigned long);
void memcg_alloc_abort_single(struct kmem_cache *s,void *p)
{ rust_slub_memcg_alloc_abort_single_body(s,p,_RET_IP_); }
#endif

EXPORT_SYMBOL(kmem_cache_alloc_noprof);
EXPORT_SYMBOL(kmem_cache_alloc_lru_noprof);
EXPORT_SYMBOL(kmem_cache_charge);
EXPORT_SYMBOL(kmem_cache_alloc_node_noprof);
EXPORT_SYMBOL(__kmalloc_large_noprof);
EXPORT_SYMBOL(__kmalloc_large_node_noprof);
EXPORT_SYMBOL(__kmalloc_node_noprof);
EXPORT_SYMBOL(__kmalloc_noprof);
EXPORT_SYMBOL_GPL(_kmalloc_nolock_noprof);
EXPORT_SYMBOL(__kmalloc_node_track_caller_noprof);
EXPORT_SYMBOL(__kmalloc_cache_noprof);
EXPORT_SYMBOL(__kmalloc_cache_node_noprof);
EXPORT_SYMBOL(kmem_cache_free);
EXPORT_SYMBOL(ksize);
EXPORT_SYMBOL(kfree);
EXPORT_SYMBOL_GPL(kfree_nolock);
EXPORT_SYMBOL(krealloc_node_align_noprof);
EXPORT_SYMBOL(__kvmalloc_node_noprof);
EXPORT_SYMBOL(kvfree);
EXPORT_SYMBOL(kvfree_atomic);
EXPORT_SYMBOL(kvfree_sensitive);
EXPORT_SYMBOL(kvrealloc_node_align_noprof);
EXPORT_SYMBOL(kmem_cache_free_bulk);
EXPORT_SYMBOL(kmem_cache_alloc_bulk_noprof);
#ifdef CONFIG_SLUB_DEBUG
EXPORT_SYMBOL(validate_slab_cache);
#endif

/* Native representations and section metadata retained from the original. */
struct slab_sheaf rust_slub_bootstrap_sheaf_storage;
struct kmem_cache __initdata rust_slub_boot_kmem_cache_storage;
struct kmem_cache __initdata rust_slub_boot_kmem_cache_node_storage;
static DEFINE_WAIT_OVERRIDE_MAP(rust_slub_kfree_rcu_sheaf_map, LD_WAIT_CONFIG);
extern int slab_memory_callback(struct notifier_block *,unsigned long,void *);
extern unsigned int slub_min_order,slub_max_order,slub_min_objects;
extern int setup_slub_min_order(const char *,const struct kernel_param *);
extern int setup_slub_max_order(const char *,const struct kernel_param *);
static const struct kernel_param_ops param_ops_slab_min_order __initconst = { .set = setup_slub_min_order };
static const struct kernel_param_ops param_ops_slab_max_order __initconst = { .set = setup_slub_max_order };
__core_param_cb(slab_min_order,&param_ops_slab_min_order,&slub_min_order,0);
__core_param_cb(slub_min_order,&param_ops_slab_min_order,&slub_min_order,0);
__core_param_cb(slab_max_order,&param_ops_slab_max_order,&slub_max_order,0);
__core_param_cb(slub_max_order,&param_ops_slab_max_order,&slub_max_order,0);
core_param(slab_min_objects,slub_min_objects,uint,0);
core_param(slub_min_objects,slub_min_objects,uint,0);
#ifdef CONFIG_NUMA
extern int setup_slab_strict_numa(const char *,const struct kernel_param *);
static const struct kernel_param_ops param_ops_slab_strict_numa __initconst = {
 .flags = KERNEL_PARAM_OPS_FL_NOARG, .set = setup_slab_strict_numa,
};
__core_param_cb(slab_strict_numa,&param_ops_slab_strict_numa,NULL,0);
#endif

#ifdef SLAB_SUPPORTS_SYSFS
#define RSL_ATTR_RO(name) \
 extern ssize_t name##_show(struct kmem_cache *,char *); \
 static const struct slab_attribute name##_attr = __ATTR_RO_MODE(name,0400)
#define RSL_ATTR_RW(name) \
 extern ssize_t name##_show(struct kmem_cache *,char *); \
 extern ssize_t name##_store(struct kmem_cache *,const char *,size_t); \
 static const struct slab_attribute name##_attr = __ATTR_RW_MODE(name,0600)
RSL_ATTR_RO(slab_size); RSL_ATTR_RO(align); RSL_ATTR_RO(object_size);
RSL_ATTR_RO(objs_per_slab); RSL_ATTR_RO(order); RSL_ATTR_RO(sheaf_capacity);
RSL_ATTR_RW(min_partial); RSL_ATTR_RW(cpu_partial); RSL_ATTR_RO(ctor);
RSL_ATTR_RO(aliases); RSL_ATTR_RO(partial); RSL_ATTR_RO(cpu_slabs);
RSL_ATTR_RO(objects_partial); RSL_ATTR_RO(slabs_cpu_partial);
RSL_ATTR_RO(reclaim_account); RSL_ATTR_RO(hwcache_align); RSL_ATTR_RO(destroy_by_rcu);
RSL_ATTR_RW(shrink);
#ifdef CONFIG_ZONE_DMA
RSL_ATTR_RO(cache_dma);
#endif
#ifdef CONFIG_HARDENED_USERCOPY
RSL_ATTR_RO(usersize);
#endif
#ifdef CONFIG_SLUB_DEBUG
RSL_ATTR_RO(slabs); RSL_ATTR_RO(total_objects); RSL_ATTR_RO(objects);
RSL_ATTR_RO(sanity_checks); RSL_ATTR_RO(trace); RSL_ATTR_RO(red_zone);
RSL_ATTR_RO(poison); RSL_ATTR_RO(store_user); RSL_ATTR_RW(validate);
#endif
#ifdef CONFIG_FAILSLAB
RSL_ATTR_RW(failslab);
#endif
#ifdef CONFIG_NUMA
RSL_ATTR_RW(remote_node_defrag_ratio);
#endif
#ifdef CONFIG_KFENCE
RSL_ATTR_RW(skip_kfence);
#endif
#ifdef CONFIG_SLUB_STATS
RSL_ATTR_RW(alloc_fastpath); RSL_ATTR_RW(alloc_slowpath);
RSL_ATTR_RW(free_rcu_sheaf); RSL_ATTR_RW(free_rcu_sheaf_fail);
RSL_ATTR_RW(free_fastpath); RSL_ATTR_RW(free_slowpath);
RSL_ATTR_RW(free_add_partial); RSL_ATTR_RW(free_remove_partial);
RSL_ATTR_RW(alloc_slab); RSL_ATTR_RW(alloc_node_mismatch); RSL_ATTR_RW(free_slab);
RSL_ATTR_RW(order_fallback); RSL_ATTR_RW(cmpxchg_double_fail);
RSL_ATTR_RW(sheaf_flush); RSL_ATTR_RW(sheaf_refill); RSL_ATTR_RW(sheaf_alloc); RSL_ATTR_RW(sheaf_free);
RSL_ATTR_RW(barn_get); RSL_ATTR_RW(barn_get_fail); RSL_ATTR_RW(barn_put); RSL_ATTR_RW(barn_put_fail);
RSL_ATTR_RW(sheaf_prefill_fast); RSL_ATTR_RW(sheaf_prefill_slow); RSL_ATTR_RW(sheaf_prefill_oversize);
RSL_ATTR_RW(sheaf_return_fast); RSL_ATTR_RW(sheaf_return_slow);
#endif
#undef RSL_ATTR_RO
#undef RSL_ATTR_RW
static const struct attribute *const slab_attrs[] = {
	&slab_size_attr.attr,
	&object_size_attr.attr,
	&objs_per_slab_attr.attr,
	&order_attr.attr,
	&sheaf_capacity_attr.attr,
	&min_partial_attr.attr,
	&cpu_partial_attr.attr,
	&objects_partial_attr.attr,
	&partial_attr.attr,
	&cpu_slabs_attr.attr,
	&ctor_attr.attr,
	&aliases_attr.attr,
	&align_attr.attr,
	&hwcache_align_attr.attr,
	&reclaim_account_attr.attr,
	&destroy_by_rcu_attr.attr,
	&shrink_attr.attr,
	&slabs_cpu_partial_attr.attr,
#ifdef CONFIG_SLUB_DEBUG
	&total_objects_attr.attr,
	&objects_attr.attr,
	&slabs_attr.attr,
	&sanity_checks_attr.attr,
	&trace_attr.attr,
	&red_zone_attr.attr,
	&poison_attr.attr,
	&store_user_attr.attr,
	&validate_attr.attr,
#endif
#ifdef CONFIG_ZONE_DMA
	&cache_dma_attr.attr,
#endif
#ifdef CONFIG_NUMA
	&remote_node_defrag_ratio_attr.attr,
#endif
#ifdef CONFIG_SLUB_STATS
	&alloc_fastpath_attr.attr,
	&alloc_slowpath_attr.attr,
	&free_rcu_sheaf_attr.attr,
	&free_rcu_sheaf_fail_attr.attr,
	&free_fastpath_attr.attr,
	&free_slowpath_attr.attr,
	&free_add_partial_attr.attr,
	&free_remove_partial_attr.attr,
	&alloc_slab_attr.attr,
	&alloc_node_mismatch_attr.attr,
	&free_slab_attr.attr,
	&order_fallback_attr.attr,
	&cmpxchg_double_fail_attr.attr,
	&sheaf_flush_attr.attr,
	&sheaf_refill_attr.attr,
	&sheaf_alloc_attr.attr,
	&sheaf_free_attr.attr,
	&barn_get_attr.attr,
	&barn_get_fail_attr.attr,
	&barn_put_attr.attr,
	&barn_put_fail_attr.attr,
	&sheaf_prefill_fast_attr.attr,
	&sheaf_prefill_slow_attr.attr,
	&sheaf_prefill_oversize_attr.attr,
	&sheaf_return_fast_attr.attr,
	&sheaf_return_slow_attr.attr,
#endif
#ifdef CONFIG_FAILSLAB
	&failslab_attr.attr,
#endif
#ifdef CONFIG_HARDENED_USERCOPY
	&usersize_attr.attr,
#endif
#ifdef CONFIG_KFENCE
	&skip_kfence_attr.attr,
#endif

	NULL
};

ATTRIBUTE_GROUPS(slab);
extern ssize_t slab_attr_show(struct kobject *,struct attribute *,char *);
extern ssize_t slab_attr_store(struct kobject *,struct attribute *,const char *,size_t);
extern void kmem_cache_release(struct kobject *);
static const struct sysfs_ops slab_sysfs_ops = { .show = slab_attr_show, .store = slab_attr_store };
const struct kobj_type rust_slub_slab_ktype = {
 .sysfs_ops = &slab_sysfs_ops, .release = kmem_cache_release, .default_groups = slab_groups,
};
#endif
#if defined(SLAB_SUPPORTS_SYSFS) || (defined(CONFIG_SLUB_DEBUG) && defined(CONFIG_DEBUG_FS))
extern int slab_late_init(void);
late_initcall(slab_late_init);
#endif
#if defined(CONFIG_SLUB_DEBUG) && defined(CONFIG_DEBUG_FS)
extern void *slab_debugfs_start(struct seq_file *,loff_t *);
extern void *slab_debugfs_next(struct seq_file *,void *,loff_t *);
extern void slab_debugfs_stop(struct seq_file *,void *);
extern int slab_debugfs_show(struct seq_file *,void *);
const struct seq_operations rust_slub_slab_debugfs_sops = {
 .start = slab_debugfs_start, .next = slab_debugfs_next,
 .stop = slab_debugfs_stop, .show = slab_debugfs_show,
};
extern int slab_debug_trace_open(struct inode *,struct file *);
extern int slab_debug_trace_release(struct inode *,struct file *);
const struct file_operations rust_slub_slab_debugfs_fops = {
 .open = slab_debug_trace_open, .read = seq_read,
 .llseek = seq_lseek, .release = slab_debug_trace_release,
};
#endif
extern void rust_slub_free_one_at_ip_body(struct kmem_cache *,struct slab *,void *,unsigned long);
void rust_slub_free_one_at_current_ip(struct kmem_cache *s,struct slab *slab,void *p)
{ rust_slub_free_one_at_ip_body(s,slab,p,_RET_IP_); }
/* Normalized private calls preserve the nested allocator's own caller site. */
void *rust_slub_call_krealloc(const void *p,size_t n,kmalloc_token_t token,unsigned long align,gfp_t f,int node)
{ return rust_slub_krealloc_node_align_noprof_body(p,n,token,align,f,node,_RET_IP_); }
void *rust_slub_call_kvmalloc(size_t n,kmalloc_token_t token,unsigned long align,gfp_t f,int node)
{ return rust_slub_kvmalloc_node_noprof_body(n,NULL,token,align,f,node,_RET_IP_); }
