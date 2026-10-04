/* SPDX-License-Identifier: GPL-2.0 */
/* Exact private C ABI definitions from the allocation half of mm/slub.c. */
#if defined(CONFIG_SLUB_DEBUG) && defined(CONFIG_DEBUG_FS)
struct location {
 depot_stack_handle_t handle;
 unsigned long count;
 unsigned long addr;
 unsigned long waste;
 long long sum_time;
 long min_time;
 long max_time;
 long min_pid;
 long max_pid;
 DECLARE_BITMAP(cpus, NR_CPUS);
 nodemask_t nodes;
};
struct loc_track {
 unsigned long max;
 unsigned long count;
 struct location *loc;
 loff_t idx;
};
#endif
#ifdef SLAB_SUPPORTS_SYSFS
struct slab_attribute {
 struct attribute attr;
 ssize_t (*show)(struct kmem_cache *s, char *buf);
 ssize_t (*store)(struct kmem_cache *s, const char *x, size_t count);
};
struct saved_alias {
 struct kmem_cache *s;
 const char *name;
 struct saved_alias *next;
};
enum slab_stat_type { SL_ALL, SL_PARTIAL, SL_OBJECTS, SL_TOTAL };
#define SO_ALL (1 << SL_ALL)
#define SO_PARTIAL (1 << SL_PARTIAL)
#define SO_OBJECTS (1 << SL_OBJECTS)
#define SO_TOTAL (1 << SL_TOTAL)
enum { RSL_SO_ALL = SO_ALL, RSL_SO_PARTIAL = SO_PARTIAL,
       RSL_SO_OBJECTS = SO_OBJECTS, RSL_SO_TOTAL = SO_TOTAL };
#endif
/* Rust algorithm entry ABI with explicit caller capture. */
bool __kmem_cache_alloc_bulk(struct kmem_cache *, gfp_t, size_t, void **);
void __kmem_cache_free_bulk(struct kmem_cache *, size_t, void **);
unsigned int __refill_objects_node(struct kmem_cache *, void **, gfp_t,
 unsigned int, unsigned int, struct kmem_cache_node *, bool);
#ifdef CONFIG_MEMCG
void memcg_alloc_abort_single(struct kmem_cache *, void *);
#endif
void rust_slub_free_one_at_current_ip(struct kmem_cache *,struct slab *,void *);
void *rust_slub_call_krealloc(const void *,size_t,kmalloc_token_t,unsigned long,gfp_t,int);
void *rust_slub_call_kvmalloc(size_t,kmalloc_token_t,unsigned long,gfp_t,int);
/* Original private macro constants (mm/slub.c:8344 and :9697). */
#define SHRINK_PROMOTE_MAX 32
#define ID_STR_LENGTH 32
enum { RSL_SHRINK_PROMOTE_MAX = SHRINK_PROMOTE_MAX };
#ifdef SLAB_SUPPORTS_SYSFS
enum { RSL_ID_STR_LENGTH = ID_STR_LENGTH };
#endif
