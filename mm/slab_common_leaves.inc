/* SPDX-License-Identifier: GPL-2.0 */
/* Native header/macro leaves, never original slab_common.c algorithm bodies. */
RSC_LEAF(void *, rust_slab_common_zero_size_ptr, (void), { return ZERO_SIZE_PTR; })
RSC_LEAF(bool, rust_slab_common_in_interrupt, (void), { return in_interrupt(); })
RSC_LEAF(unsigned int, rust_slab_common_cache_line_size, (void), { return cache_line_size(); })
RSC_LEAF(unsigned int, rust_slab_common_arch_slab_minalign, (void), { return arch_slab_minalign(); })
RSC_LEAF(void *, rust_slab_common_create_cache_zalloc, (struct kmem_cache *s, gfp_t flags), { return kmem_cache_zalloc(s, flags); })
RSC_LEAF(void *, rust_slab_common_boot_cache_zalloc, (struct kmem_cache *s, gfp_t flags), { return kmem_cache_zalloc(s, flags); })
RSC_LEAF(void *, rust_slab_common_cache_alloc, (struct kmem_cache *s, gfp_t flags), { return kmem_cache_alloc(s, flags); })
RSC_LEAF(bool, rust_slab_common_is_err, (const void *ptr), { return IS_ERR(ptr); })
RSC_LEAF(void, rust_slab_common_list_add, (struct list_head *node, struct list_head *head), { list_add(node, head); })
RSC_LEAF(void, rust_slab_common_list_del, (struct list_head *node), { list_del(node); })
RSC_LEAF(void, rust_slab_common_mutex_lock, (struct mutex *lock), { mutex_lock(lock); })
RSC_LEAF(void, rust_slab_common_mutex_unlock, (struct mutex *lock), { mutex_unlock(lock); })
RSC_LEAF(bool, rust_slab_common_mutex_trylock, (struct mutex *lock), { return mutex_trylock(lock); })
RSC_LEAF(int, rust_slab_common_sysfs_slab_alias, (struct kmem_cache *s, const char *name), { return sysfs_slab_alias(s, name); })
RSC_LEAF(void, rust_slab_common_sysfs_slab_release, (struct kmem_cache *s), { sysfs_slab_release(s); })
RSC_LEAF(void, rust_slab_common_sysfs_slab_unlink, (struct kmem_cache *s), { sysfs_slab_unlink(s); })
RSC_LEAF(void, rust_slab_common_debugfs_slab_release, (struct kmem_cache *s), { debugfs_slab_release(s); })
RSC_LEAF(bool, rust_slab_common_mem_cgroup_kmem_disabled, (void), { return mem_cgroup_kmem_disabled(); })
RSC_LEAF(bool, rust_slab_common_kasan_check_byte, (const void *p), { return kasan_check_byte(p); })
RSC_LEAF(void, rust_slab_common_kasan_cache_shutdown, (struct kmem_cache *s), { kasan_cache_shutdown(s); })
RSC_LEAF(void, rust_slab_common_kasan_cache_shrink, (struct kmem_cache *s), { kasan_cache_shrink(s); })
RSC_LEAF(void, rust_slab_common_kfence_shutdown_cache, (struct kmem_cache *s), { kfence_shutdown_cache(s); })
RSC_LEAF(bool, rust_slab_common_slab_in_kunit_test, (void), { return slab_in_kunit_test(); })
RSC_LEAF(unsigned int, rust_slab_common_kmalloc_index, (size_t size), { return __kmalloc_index(size, false); })
RSC_LEAF(struct kmem_cache *, rust_slab_common_kmalloc_slab_roundup, (size_t size), { return kmalloc_slab(size, NULL, GFP_KERNEL, __kmalloc_token(0), SLAB_ALLOC_DEFAULT); })
RSC_LEAF(unsigned int, rust_slab_common_get_order, (size_t size), { return get_order(size); })
RSC_LEAF(unsigned int, rust_slab_common_dma_get_cache_alignment, (void), { return dma_get_cache_alignment(); })
RSC_LEAF(bool, rust_slab_common_is_swiotlb_allocated, (void), { return is_swiotlb_allocated(); })
RSC_LEAF(bool, rust_slab_common_need_kmalloc_no_objext, (void), { return need_kmalloc_no_objext(); })
RSC_LEAF(bool, rust_slab_common_virt_addr_valid, (const void *p), { return virt_addr_valid(p); })
RSC_LEAF(struct slab *, rust_slab_common_virt_to_slab, (const void *p), { return virt_to_slab(p); })
RSC_LEAF(bool, rust_slab_common_is_kfence_address, (const void *p), { return is_kfence_address(p); })
RSC_LEAF(void, rust_slab_common_kasan_unpoison_range, (const void *p, size_t size), { kasan_unpoison_range(p, size); })
RSC_LEAF(void, rust_slab_common_memzero_explicit, (void *p, size_t size), { memzero_explicit(p, size); })
#ifdef CONFIG_PRINTK
RSC_LEAF(bool, rust_slab_common_kfence_obj_info, (struct kmem_obj_info *info, void *p, struct slab *slab), { return __kfence_obj_info(info, p, slab); })
#endif
#ifdef CONFIG_SLUB_DEBUG
RSC_LEAF(void, rust_slab_common_enable_slub_debug, (void), { static_branch_enable(&slub_debug_enabled); })
RSC_LEAF(void, rust_slab_common_stack_depot_init, (void), { stack_depot_init(); })
#endif
#ifdef CONFIG_SLAB_FREELIST_RANDOM
RSC_LEAF(unsigned int, rust_slab_common_get_random_u32_below, (unsigned int ceiling), { return get_random_u32_below(ceiling); })
RSC_LEAF(void *, rust_slab_common_kcalloc, (size_t n, size_t size, gfp_t flags), { return kcalloc(n, size, flags); })
#endif
/* One leaf per warning site retains native once-state and site identity. */
#ifdef CONFIG_DEBUG_VM
RSC_LEAF(void, rust_slab_common_warn_duplicate, (bool condition, const char *name), { WARN(condition, "kmem_cache of name '%s' already exists\n", name); })
RSC_LEAF(void, rust_slab_common_warn_space, (bool condition), { WARN_ON(condition); })
#endif
RSC_LEAF(bool, rust_slab_common_warn_useroffset, (bool condition), { return WARN_ON(condition); })
RSC_LEAF(bool, rust_slab_common_warn_usersize, (bool condition), { return WARN_ON(condition); })
RSC_LEAF(bool, rust_slab_common_warn_buckets_cache, (bool condition), { return WARN_ON(condition); })
RSC_LEAF(bool, rust_slab_common_warn_buckets_alloc, (bool condition), { return WARN_ON(condition); })
RSC_LEAF(bool, rust_slab_common_warn_short_size, (bool condition), { return WARN_ON(condition); })
RSC_LEAF(bool, rust_slab_common_warn_cache_name, (bool condition), { return WARN_ON(condition); })
RSC_LEAF(bool, rust_slab_common_warn_bucket_create, (bool condition), { return WARN_ON(condition); })
RSC_LEAF(void, rust_slab_common_warn_destroy, (bool condition, struct kmem_cache *s, void *caller), { WARN(condition, "%s %s: Slab cache still has objects when called from %pS", "kmem_cache_destroy", s->name, caller); })
RSC_LEAF(void, rust_slab_common_dump_stack, (void), { dump_stack(); })
#ifdef CONFIG_SLUB_DEBUG
RSC_LEAF(void, rust_slab_common_seq_puts, (struct seq_file *m, const char *str), { seq_puts(m, str); })
RSC_LEAF(struct proc_dir_entry *, rust_slab_common_proc_create,
 (const char *name, umode_t mode, struct proc_dir_entry *parent, const struct proc_ops *ops),
 { return proc_create(name, mode, parent, ops); })
#endif
