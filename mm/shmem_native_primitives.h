/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_SHMEM_PRIMITIVES_H
#define RUST_SHMEM_PRIMITIVES_H
/* Executable native macro/architecture/configuration/instrumentation leaves.
 * These do not contain shmem.c owner algorithms. Native header inline wrappers
 * emitted later by bindgen are separately inventoried as executable runtime. */
#ifdef RUST_SHMEM_NATIVE_HELPERS
#define SM_LEAF(ret, name, args, ...) ret rust_shmem_##name args { __VA_ARGS__ }
#else
#define SM_LEAF(ret, name, args, ...) ret rust_shmem_##name args;
#endif
SM_LEAF(unsigned int, inode_nlink, (struct inode *i), return i->i_nlink;)
SM_LEAF(const struct file_operations **, inode_fop_ptr, (struct inode *i), return &i->i_fop;)
SM_LEAF(unsigned long, vma_vm_start, (struct vm_area_struct *v), return v->vm_start;)
SM_LEAF(unsigned long, vma_vm_end, (struct vm_area_struct *v), return v->vm_end;)
SM_LEAF(vm_flags_t, vma_vm_flags, (struct vm_area_struct *v), return v->vm_flags;)
SM_LEAF(struct vm_area_struct *, vmf_vma, (struct vm_fault *v), return v->vma;)
SM_LEAF(pgoff_t, vmf_pgoff, (struct vm_fault *v), return v->pgoff;)
SM_LEAF(unsigned long, vmf_address, (struct vm_fault *v), return v->address;)
SM_LEAF(struct address_space **, folio_mapping_ptr, (struct folio *f), return &f->mapping;)
SM_LEAF(pgoff_t *, folio_index_ptr, (struct folio *f), return &f->index;)
SM_LEAF(loff_t *, file_prev_pos, (struct file *f), return &f->f_ra.prev_pos;)
SM_LEAF(bool, PageHWPoison, (const struct page *p), return PageHWPoison(p);)
SM_LEAF(void, bug, (bool condition), BUG_ON(condition);)
SM_LEAF(bool, IS_ERR_VALUE, (unsigned long value), return IS_ERR_VALUE(value);)
SM_LEAF(unsigned long, task_size, (void), return TASK_SIZE;)
SM_LEAF(kuid_t, current_fsuid, (void), return current_fsuid();)
SM_LEAF(kgid_t, current_fsgid, (void), return current_fsgid();)
SM_LEAF(vma_flags_t, vma_flags, (struct vm_area_struct *vma), return vma->flags;)
SM_LEAF(vma_flags_t, noreserve_flags, (void), return mk_vma_flags(VMA_NORESERVE_BIT);)
SM_LEAF(struct page *, folio_page_address, (struct folio *f), return &f->page;)
SM_LEAF(bool, isreg, (umode_t mode), return S_ISREG(mode);)
SM_LEAF(bool, isdir, (umode_t mode), return S_ISDIR(mode);)
SM_LEAF(bool, islnk, (umode_t mode), return S_ISLNK(mode);)
SM_LEAF(void, pr_err, (const char *s), pr_err("%s", s);)
SM_LEAF(void, pr_warn, (const char *s), pr_warn("%s", s);)
#ifdef CONFIG_SHMEM
SM_LEAF(void, vm_bug, (bool condition), VM_BUG_ON(condition);)
SM_LEAF(void, vm_bug_folio, (bool condition, struct folio *folio), VM_BUG_ON_FOLIO(condition, folio);)
SM_LEAF(bool, list_empty, (const struct list_head *h), return list_empty(h);)
SM_LEAF(bool, list_empty_careful, (const struct list_head *h), return list_empty_careful(h);)
SM_LEAF(bool, simple_empty, (struct dentry *d), return simple_empty(d);)
SM_LEAF(bool, mapping_writably_mapped, (const struct address_space *m), return mapping_writably_mapped(m);)
SM_LEAF(bool, inode_unhashed, (struct inode *i), return inode_unhashed(i);)
SM_LEAF(bool, sb_any_quota_loaded, (struct super_block *s), return sb_any_quota_loaded(s);)
SM_LEAF(bool, sb_has_quota_active, (struct super_block *s, int t), return sb_has_quota_active(s, t);)
SM_LEAF(bool, user_shm_lock, (loff_t size, struct ucounts *u), return user_shm_lock(size, u);)
SM_LEAF(void, might_sleep, (void), might_sleep();)
SM_LEAF(struct mm_struct *, current_mm, (void), return current->mm;)
SM_LEAF(bool, fatal_signal_pending, (void), return fatal_signal_pending(current);)
SM_LEAF(unsigned long, mapping_nrpages, (struct address_space *mapping), return READ_ONCE(mapping->nrpages);)
SM_LEAF(unsigned long, read_swapped, (struct shmem_inode_info *info), return READ_ONCE(info->swapped);)
SM_LEAF(unsigned long, read_shrinklist_len, (struct shmem_sb_info *info), return READ_ONCE(info->shrinklist_len);)
SM_LEAF(unsigned long, read_ulong, (const unsigned long *p), return READ_ONCE(*p);)
SM_LEAF(void *, read_private, (struct inode *inode), return READ_ONCE(inode->i_private);)
SM_LEAF(void, write_private, (struct inode *inode, void *p), WRITE_ONCE(inode->i_private, p);)
SM_LEAF(bool, accounting_stale, (struct inode *inode), return data_race(SHMEM_I(inode)->alloced - SHMEM_I(inode)->swapped != inode->i_mapping->nrpages);)
SM_LEAF(bool, swap_synchronous, (struct swap_info_struct *si), return data_race(si->flags & SWP_SYNCHRONOUS_IO);)
SM_LEAF(struct list_head *, inode_shrinklist, (struct shmem_inode_info *info), return &info->shrinklist;)
SM_LEAF(struct list_head *, inode_swaplist, (struct shmem_inode_info *info), return &info->swaplist;)
SM_LEAF(struct offset_ctx *, inode_offsets, (struct shmem_inode_info *info), return &info->dir_offsets;)
SM_LEAF(struct shmem_inode_info *, shrinklist_inode, (struct list_head *node), return list_entry(node, struct shmem_inode_info, shrinklist);)
SM_LEAF(struct shmem_inode_info *, swaplist_inode, (struct list_head *node), return list_entry(node, struct shmem_inode_info, swaplist);)
SM_LEAF(ino_t *, get_cpu_ino, (ino_t __percpu *base), return per_cpu_ptr(base, get_cpu());)
SM_LEAF(void, put_cpu, (void), put_cpu();)
/* Keep each original lexical allocation tag. The native macro retains its
 * sections, alignment, per-CPU counters and MODULE/profiling alternatives. */
#pragma push_macro("CODE_TAG_INIT")
#undef CODE_TAG_INIT
#define CODE_TAG_INIT { \
    .modname = CT_MODULE_NAME, \
    .function = "shmem_fill_super", \
    .filename = "mm/shmem.c", \
    .lineno = 5012, \
    .flags = 0, \
}
SM_LEAF(ino_t __percpu *, alloc_ino_batch, (void), return alloc_percpu(ino_t);)
#pragma pop_macro("CODE_TAG_INIT")
SM_LEAF(void, inode_overflow, (struct super_block *sb), pr_warn("shmem_reserve_inode: inode number overflow on device %d, consider using inode64 mount option\n", MINOR(sb->s_dev));)
SM_LEAF(void, eviction_warning, (struct inode *inode), struct shmem_inode_info *info = SHMEM_I(inode); pr_warn("shmem_evict_inode: ino=%llu i_blocks=%llu alloced=%lu swapped=%lu nrpages=%lu\n", inode->i_ino, inode->i_blocks, info->alloced, info->swapped, inode->i_mapping->nrpages);)
SM_LEAF(void, wait_eviction, (struct shmem_inode_info *info), wait_var_event(&info->stop_eviction, !atomic_read(&info->stop_eviction));)
SM_LEAF(void, wake_eviction, (struct shmem_inode_info *info), wake_up_var(&info->stop_eviction);)
SM_LEAF(void, raw_spin_lock, (raw_spinlock_t *lock), raw_spin_lock(lock);)
SM_LEAF(void, raw_spin_unlock, (raw_spinlock_t *lock), raw_spin_unlock(lock);)
SM_LEAF(void, xa_lock_irq, (struct xarray *xa), xa_lock_irq(xa);)
SM_LEAF(void, xa_unlock_irq, (struct xarray *xa), xa_unlock_irq(xa);)
SM_LEAF(void, xas_lock_irq, (struct xa_state *xas), xas_lock_irq(xas);)
SM_LEAF(void, xas_unlock_irq, (struct xa_state *xas), xas_unlock_irq(xas);)
SM_LEAF(void, xas_init, (struct xa_state *out, struct xarray *xa, pgoff_t index, unsigned int order), XA_STATE_ORDER(state, xa, index, order); *out = state;)
SM_LEAF(int, cond_resched, (void), return cond_resched();)
SM_LEAF(swp_entry_t, folio_swap, (struct folio *f), return f->swap;)
SM_LEAF(void, set_folio_swap, (struct folio *f, swp_entry_t swap), f->swap = swap;)
SM_LEAF(void, clear_folio_private, (struct folio *f), f->private = NULL;)
SM_LEAF(void, init_wait, (wait_queue_entry_t *out, wait_queue_func_t fn), DEFINE_WAIT_FUNC(wait, fn); *out = wait; INIT_LIST_HEAD(&out->entry);)
SM_LEAF(void, init_falloc_waitq, (wait_queue_head_t *out), init_waitqueue_head(out);)
SM_LEAF(void, wake_all, (wait_queue_head_t *q), wake_up_all(q);)
SM_LEAF(bool, warn_mapping, (bool condition), return WARN_ON_ONCE(condition);)
SM_LEAF(void, warn_fault_page, (bool condition), WARN_ON_ONCE(condition);)
SM_LEAF(void, warn_waitq, (bool condition), WARN_ON_ONCE(condition);)
SM_LEAF(void, warn_xattr_space, (bool condition), WARN_ON(condition);)
SM_LEAF(void, init_inode_lock, (struct shmem_inode_info *info), spin_lock_init(&info->lock);)
SM_LEAF(void, init_stat_lock, (struct shmem_sb_info *info), raw_spin_lock_init(&info->stat_lock);)
SM_LEAF(void, init_shrink_lock, (struct shmem_sb_info *info), spin_lock_init(&info->shrinklist_lock);)
SM_LEAF(int, init_blocks, (struct shmem_sb_info *info), return percpu_counter_init(&info->used_blocks, 0, GFP_KERNEL);)
SM_LEAF(void, zero_inode_prefix, (struct shmem_inode_info *info, struct inode *inode), memset(info, 0, (char *)inode - (char *)info);)
#pragma push_macro("CODE_TAG_INIT")
#undef CODE_TAG_INIT
#define CODE_TAG_INIT { \
    .modname = CT_MODULE_NAME, \
    .function = "shmem_init_fs_context", \
    .filename = "mm/shmem.c", \
    .lineno = 5285, \
    .flags = 0, \
}
SM_LEAF(struct shmem_options *, alloc_options, (void),
    struct shmem_options *ctx;
    ctx = kzalloc_obj(struct shmem_options);
    return ctx;)
#pragma pop_macro("CODE_TAG_INIT")

/* Preserve the original compile-time size expression and typed assignment;
 * the allocator's configured partition-token machinery remains native. */
#pragma push_macro("CODE_TAG_INIT")
#undef CODE_TAG_INIT
#define CODE_TAG_INIT { \
    .modname = CT_MODULE_NAME, \
    .function = "shmem_fill_super", \
    .filename = "mm/shmem.c", \
    .lineno = 4962, \
    .flags = 0, \
}
SM_LEAF(struct shmem_sb_info *, alloc_sbinfo, (void),
    struct shmem_sb_info *sbinfo;
    sbinfo = kzalloc(max((int)sizeof(struct shmem_sb_info),
                        L1_CACHE_BYTES), GFP_KERNEL);
    return sbinfo;)
#pragma pop_macro("CODE_TAG_INIT")

#pragma push_macro("CODE_TAG_INIT")
#undef CODE_TAG_INIT
#define CODE_TAG_INIT { \
    .modname = CT_MODULE_NAME, \
    .function = "shmem_symlink", \
    .filename = "mm/shmem.c", \
    .lineno = 4079, \
    .flags = 0, \
}
SM_LEAF(void *, kmemdup, (const void *src, size_t len, gfp_t flags), return kmemdup(src, len, flags);)
#pragma pop_macro("CODE_TAG_INIT")
SM_LEAF(struct kmem_cache *, kmem_cache_create, (const char *name, unsigned int size, unsigned int align, slab_flags_t flags, void (*ctor)(void *)), return kmem_cache_create(name, size, align, flags, ctor);)

#pragma push_macro("CODE_TAG_INIT")
#undef CODE_TAG_INIT
#define CODE_TAG_INIT { \
    .modname = CT_MODULE_NAME, \
    .function = "shmem_alloc_inode", \
    .filename = "mm/shmem.c", \
    .lineno = 5115, \
    .flags = 0, \
}
SM_LEAF(void *, alloc_inode_sb, (struct super_block *sb, struct kmem_cache *cache, gfp_t flags), return alloc_inode_sb(sb, cache, flags);)
#pragma pop_macro("CODE_TAG_INIT")
SM_LEAF(void *, inode_link, (struct inode *inode), return inode->i_link;)
SM_LEAF(bool, has_transparent_hugepage, (void), return has_transparent_hugepage();)
SM_LEAF(int, split_folio, (struct folio *f), return split_folio(f);)
/* One original C1933 tag, with native save/restore around the noprof owner.
 * Calling folio_alloc_mpol here would add an unwanted second lexical tag. */
#pragma push_macro("CODE_TAG_INIT")
#undef CODE_TAG_INIT
#define CODE_TAG_INIT { \
    .modname = CT_MODULE_NAME, \
    .function = "shmem_alloc_folio", \
    .filename = "mm/shmem.c", \
    .lineno = 1933, \
    .flags = 0, \
}
SM_LEAF(struct folio *, folio_alloc_mpol,
        (gfp_t gfp, unsigned int order, struct mempolicy *pol, pgoff_t ilx, int nid),
    DEFINE_ALLOC_TAG(_alloc_tag);
    return alloc_hooks_tag(&_alloc_tag,
                           folio_alloc_mpol_noprof(gfp, order, pol, ilx, nid));)
#pragma pop_macro("CODE_TAG_INIT")
SM_LEAF(struct page *, folio_page, (struct folio *f, unsigned long index), return folio_page(f, index);)
SM_LEAF(size_t, offset_in_folio, (struct folio *f, unsigned long pos), return offset_in_folio(f, pos);)
SM_LEAF(struct page *, zero_page, (void), return ZERO_PAGE(0);)
SM_LEAF(void, pipe_advance_head, (struct pipe_inode_info *pipe), pipe->head++;)
SM_LEAF(size_t, strlen, (const char *s), return strlen(s);)
SM_LEAF(void *, memcpy, (void *dst, const void *src, size_t size), return memcpy(dst, src, size);)
#ifdef CONFIG_NUMA
SM_LEAF(struct mempolicy *, current_policy, (void), return get_task_policy(current);)
#endif
#ifdef CONFIG_TMPFS_QUOTA
SM_LEAF(void, quota_warning, (int type, int err), pr_warn("tmpfs: failed to enable quota tracking (type=%d, err=%d)\n", type, err);)
#endif
#ifdef CONFIG_TMPFS
SM_LEAF(const char *, fc_prefix, (struct fs_context *fc), return fc->log.prefix;)
SM_LEAF(struct fc_log *, fc_log, (struct fs_context *fc), return fc->log.log;)
SM_LEAF(__u32 *, fid_raw, (struct fid *fid), return fid->raw;)
SM_LEAF(char *, param_string, (struct fs_parameter *param), return param->string;)
SM_LEAF(u32, result_u32, (const struct fs_parse_result *r), return r->uint_32;)
SM_LEAF(kuid_t, result_uid, (const struct fs_parse_result *r), return r->uid;)
SM_LEAF(kgid_t, result_gid, (const struct fs_parse_result *r), return r->gid;)
SM_LEAF(bool, isdigit, (int c), return isdigit(c);)
SM_LEAF(unsigned long, blocks_k, (unsigned long pages), return K(pages);)
SM_LEAF(kuid_t, root_uid, (void), return GLOBAL_ROOT_UID;)
SM_LEAF(kgid_t, root_gid, (void), return GLOBAL_ROOT_GID;)
#if IS_ENABLED(CONFIG_UNICODE)
SM_LEAF(bool, casefolded, (struct inode *inode), return IS_CASEFOLDED(inode);)
SM_LEAF(unsigned int, unicode_major, (int v), return unicode_major(v);)
SM_LEAF(unsigned int, unicode_minor, (int v), return unicode_minor(v);)
SM_LEAF(unsigned int, unicode_rev, (int v), return unicode_rev(v);)
SM_LEAF(void, encoding_info, (int version), pr_info("tmpfs: Using encoding : utf8-%u.%u.%u\n", unicode_major(version), unicode_minor(version), unicode_rev(version));)
#endif
#endif
#ifdef CONFIG_TRANSPARENT_HUGEPAGE
SM_LEAF(bool, test_bit, (unsigned long bit, const unsigned long *addr), return test_bit(bit, addr);)
SM_LEAF(void __init, copy_boot_string, (const char *s), strscpy(rust_shmem_data_str_dup, s);)
SM_LEAF(void, invalid_thp_size, (const char *s), pr_err("invalid size %s in thp_shmem boot parameter\n", s);)
SM_LEAF(void, invalid_thp_policy, (const char *s), pr_err("invalid policy %s in thp_shmem boot parameter\n", s);)
SM_LEAF(void, invalid_thp_string, (const char *s), pr_warn("thp_shmem=%s: error parsing string, ignoring setting\n", s);)
#ifdef CONFIG_SYSFS
SM_LEAF(struct thpsize *, to_thpsize, (struct kobject *kobj), return to_thpsize(kobj);)
SM_LEAF(bool, __test_and_set_bit, (unsigned long bit, unsigned long *addr), return __test_and_set_bit(bit, addr);)
SM_LEAF(bool, __test_and_clear_bit, (unsigned long bit, unsigned long *addr), return __test_and_clear_bit(bit, addr);)
#endif
#endif
#endif /* CONFIG_SHMEM */
#undef SM_LEAF
#endif
