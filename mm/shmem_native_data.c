// SPDX-License-Identifier: GPL-2.0-only
/* Data, native initializers and symbol/boot metadata extracted from original
 * mm/shmem.c at e1d84f501551943a11f4c5271e9f5c85d7e15168.
 * No original owner function bodies remain in this translation unit.
 */
// SPDX-License-Identifier: GPL-2.0
/*
 * Resizable virtual memory filesystem for Linux.
 *
 * Copyright (C) 2000 Linus Torvalds.
 *		 2000 Transmeta Corp.
 *		 2000-2001 Christoph Rohland
 *		 2000-2001 SAP AG
 *		 2002 Red Hat Inc.
 * Copyright (C) 2002-2011 Hugh Dickins.
 * Copyright (C) 2011 Google Inc.
 * Copyright (C) 2002-2005 VERITAS Software Corporation.
 * Copyright (C) 2004 Andi Kleen, SuSE Labs
 *
 * Extended attribute support for tmpfs:
 * Copyright (c) 2004, Luke Kenneth Casson Leighton <lkcl@lkcl.net>
 * Copyright (c) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
 *
 * tiny-shmem:
 * Copyright (c) 2004, 2008 Matt Mackall <mpm@selenic.com>
 */

#include <linux/fs.h>
#include <linux/init.h>
#include <linux/vfs.h>
#include <linux/mount.h>
#include <linux/ramfs.h>
#include <linux/pagemap.h>
#include <linux/file.h>
#include <linux/fileattr.h>
#include <linux/filelock.h>
#include <linux/mm.h>
#include <linux/random.h>
#include <linux/sched/signal.h>
#include <linux/export.h>
#include <linux/shmem_fs.h>
#include <linux/swap.h>
#include <linux/uio.h>
#include <linux/hugetlb.h>
#include <linux/fs_parser.h>
#include <linux/swapfile.h>
#include <linux/iversion.h>
#include <linux/unicode.h>
#include <linux/swap_ops.h>
#include "swap.h"

struct vfsmount *rust_shmem_data_shm_mnt __ro_after_init;

#ifdef CONFIG_SHMEM
/*
 * This virtual memory filesystem is heavily based on the ramfs. It
 * extends ramfs by the ability to use swap and honor resource limits
 * which makes it a completely usable filesystem.
 */

#include <linux/xattr.h>
#include <linux/exportfs.h>
#include <linux/posix_acl.h>
#include <linux/posix_acl_xattr.h>
#include <linux/mman.h>
#include <linux/string.h>
#include <linux/slab.h>
#include <linux/backing-dev.h>
#include <linux/writeback.h>
#include <linux/folio_batch.h>
#include <linux/percpu_counter.h>
#include <linux/falloc.h>
#include <linux/splice.h>
#include <linux/security.h>
#include <linux/leafops.h>
#include <linux/mempolicy.h>
#include <linux/namei.h>
#include <linux/ctype.h>
#include <linux/migrate.h>
#include <linux/highmem.h>
#include <linux/seq_file.h>
#include <linux/magic.h>
#include <linux/syscalls.h>
#include <linux/fcntl.h>
#include <uapi/linux/memfd.h>
#include <linux/rmap.h>
#include <linux/uuid.h>
#include <linux/quotaops.h>
#include <linux/rcupdate_wait.h>

#include <linux/uaccess.h>

#include "internal.h"

#define VM_ACCT(size)    (PAGE_ALIGN(size) >> PAGE_SHIFT)

/* Pretend that each entry is of this size in directory's i_size */
#define BOGO_DIRENT_SIZE 20

/* Pretend that one inode + its dentry occupy this much memory */
#define BOGO_INODE_SIZE 1024

/* Symlink up to this size is kmalloc'ed instead of using a swappable page */
#define SHORT_SYMLINK_LEN 128

/*
 * shmem_fallocate communicates with shmem_fault or shmem_writeout via
 * inode->i_private (with i_rwsem making sure that it has only one user at
 * a time): we would prefer not to enlarge the shmem inode just for that.
 */
struct shmem_falloc {
	wait_queue_head_t *waitq; /* faults into hole wait for punch to end */
	pgoff_t start;		/* start of range currently being fallocated */
	pgoff_t next;		/* the next page offset to be fallocated */
	pgoff_t nr_falloced;	/* how many new pages have been fallocated */
	pgoff_t nr_unswapped;	/* how often writeout refused to swap out */
};

struct shmem_options {
	unsigned long long blocks;
	unsigned long long inodes;
	struct mempolicy *mpol;
	kuid_t uid;
	kgid_t gid;
	umode_t mode;
	bool full_inums;
	int huge;
	int seen;
	bool noswap;
	unsigned short quota_types;
	struct shmem_quota_limits qlimits;
#if IS_ENABLED(CONFIG_UNICODE)
	struct unicode_map *encoding;
	bool strict_encoding;
#endif
#define SHMEM_SEEN_BLOCKS 1
#define SHMEM_SEEN_INODES 2
#define SHMEM_SEEN_HUGE 4
#define SHMEM_SEEN_INUMS 8
#define SHMEM_SEEN_QUOTA 16
};

#ifdef CONFIG_TRANSPARENT_HUGEPAGE
unsigned long rust_shmem_data_huge_shmem_orders_always __read_mostly;
unsigned long rust_shmem_data_huge_shmem_orders_madvise __read_mostly;
unsigned long rust_shmem_data_huge_shmem_orders_inherit __read_mostly;
unsigned long rust_shmem_data_huge_shmem_orders_within_size __read_mostly;
bool rust_shmem_data_shmem_orders_configured __initdata;
#endif

#ifdef CONFIG_TMPFS
extern unsigned long rust_shmem_owner_shmem_default_max_blocks(void);

extern unsigned long rust_shmem_owner_shmem_default_max_inodes(void);
#endif

int rust_shmem_owner_shmem_swapin_folio(struct inode *inode, pgoff_t index,
			struct folio **foliop, enum sgp_type sgp, gfp_t gfp,
			struct vm_fault *vmf, vm_fault_t *fault_type);

extern struct shmem_sb_info *rust_shmem_owner_SHMEM_SB(struct super_block *sb);

/*
 * shmem_file_setup pre-accounts the whole fixed size of a VM object,
 * for shared memory and for shared anonymous (/dev/zero) mappings
 * (unless MAP_NORESERVE and sysctl_overcommit_memory <= 1),
 * consistent with the pre-accounting of private mappings ...
 */
extern int rust_shmem_owner_shmem_acct_size(unsigned long flags, loff_t size);

extern void rust_shmem_owner_shmem_unacct_size(unsigned long flags, loff_t size);

extern int rust_shmem_owner_shmem_reacct_size(unsigned long flags,
		loff_t oldsize, loff_t newsize);

/*
 * ... whereas tmpfs objects are accounted incrementally as
 * pages are allocated, in order to allow large sparse files.
 * shmem_get_folio reports shmem_acct_blocks failure as -ENOSPC not -ENOMEM,
 * so that a failure on a sparse tmpfs mapping will give SIGBUS not OOM.
 */
extern int rust_shmem_owner_shmem_acct_blocks(unsigned long flags, long pages);

extern void rust_shmem_owner_shmem_unacct_blocks(unsigned long flags, long pages);

extern int shmem_inode_acct_blocks(struct inode *inode, long pages);

extern void rust_shmem_owner_shmem_inode_unacct_blocks(struct inode *inode, long pages);

extern const struct super_operations rust_shmem_data_shmem_ops;
extern const struct address_space_operations rust_shmem_data_shmem_aops;
extern const struct file_operations rust_shmem_data_shmem_file_operations;
extern const struct inode_operations rust_shmem_data_shmem_inode_operations;
extern const struct inode_operations rust_shmem_data_shmem_dir_inode_operations;
extern const struct inode_operations rust_shmem_data_shmem_special_inode_operations;
extern const struct vm_operations_struct rust_shmem_data_shmem_vm_ops;
extern const struct vm_operations_struct rust_shmem_data_shmem_anon_vm_ops;
extern struct file_system_type rust_shmem_data_shmem_fs_type;

extern bool shmem_mapping(const struct address_space *mapping);
EXPORT_SYMBOL_GPL(shmem_mapping);

extern bool vma_is_anon_shmem(const struct vm_area_struct *vma);

extern bool vma_is_shmem(const struct vm_area_struct *vma);

LIST_HEAD(rust_shmem_data_shmem_swaplist);
DEFINE_SPINLOCK(rust_shmem_data_shmem_swaplist_lock);

#ifdef CONFIG_TMPFS_QUOTA

extern int rust_shmem_owner_shmem_enable_quotas(struct super_block *sb,
			       unsigned short quota_types);

extern void rust_shmem_owner_shmem_disable_quotas(struct super_block *sb);

extern struct dquot __rcu **rust_shmem_owner_shmem_get_dquots(struct inode *inode);
#endif /* CONFIG_TMPFS_QUOTA */

/*
 * shmem_reserve_inode() performs bookkeeping to reserve a shmem inode, and
 * produces a novel ino for the newly allocated inode.
 *
 * It may also be called when making a hard link to permit the space needed by
 * each dentry. However, in that case, no new inode number is needed since that
 * internally draws from another pool of inode numbers (currently global
 * get_next_ino()). This case is indicated by passing NULL as inop.
 */
#define SHMEM_INO_BATCH 1024
extern int rust_shmem_owner_shmem_reserve_inode(struct super_block *sb, ino_t *inop);

extern void rust_shmem_owner_shmem_free_inode(struct super_block *sb, size_t freed_ispace);

/**
 * shmem_recalc_inode - recalculate the block usage of an inode
 * @inode: inode to recalc
 * @alloced: the change in number of pages allocated to inode
 * @swapped: the change in number of pages swapped from inode
 *
 * We have to calculate the free blocks since the mm can drop
 * undirtied hole pages behind our back.
 *
 * But normally   info->alloced == inode->i_mapping->nrpages + info->swapped
 * So mm freed is info->alloced - (inode->i_mapping->nrpages + info->swapped)
 *
 * Return: true if swapped was incremented from 0, for shmem_writeout().
 */
extern bool shmem_recalc_inode(struct inode *inode, long alloced, long swapped);

extern bool shmem_charge(struct inode *inode, long pages);

extern void shmem_uncharge(struct inode *inode, long pages);

/*
 * Replace item expected in xarray by a new item, while holding xa_lock.
 */
extern int rust_shmem_owner_shmem_replace_entry(struct address_space *mapping,
			pgoff_t index, void *expected, void *replacement);

/*
 * Sometimes, before we decide whether to proceed or to fail, we must check
 * that an entry was not already brought back or split by a racing thread.
 *
 * Checking folio is not enough: by the time a swapcache folio is locked, it
 * might be reused, and again be swapcache, using the same swap as before.
 * Returns the swap entry's order if it still presents, else returns -1.
 */
extern int rust_shmem_owner_shmem_confirm_swap(struct address_space *mapping, pgoff_t index,
			      swp_entry_t swap);

/*
 * Definitions for "huge tmpfs": tmpfs mounted with the huge= option
 *
 * SHMEM_HUGE_NEVER:
 *	disables huge pages for the mount;
 * SHMEM_HUGE_ALWAYS:
 *	enables huge pages for the mount;
 * SHMEM_HUGE_WITHIN_SIZE:
 *	only allocate huge pages if the page will be fully within i_size,
 *	also respect madvise() hints;
 * SHMEM_HUGE_ADVISE:
 *	only allocate huge pages if requested with madvise();
 */

#define SHMEM_HUGE_NEVER	0
#define SHMEM_HUGE_ALWAYS	1
#define SHMEM_HUGE_WITHIN_SIZE	2
#define SHMEM_HUGE_ADVISE	3

/*
 * Special values.
 * Only can be set via /sys/kernel/mm/transparent_hugepage/shmem_enabled:
 *
 * SHMEM_HUGE_DENY:
 *	disables huge on shm_mnt and all mounts, for emergency use;
 * SHMEM_HUGE_FORCE:
 *	enables huge on shm_mnt and all mounts, w/o needing option, for testing;
 *
 */
#define SHMEM_HUGE_DENY		(-1)
#define SHMEM_HUGE_FORCE	(-2)

#ifdef CONFIG_TRANSPARENT_HUGEPAGE
/* ifdef here to avoid bloating shmem.o when not necessary */

#if defined(CONFIG_TRANSPARENT_HUGEPAGE_SHMEM_HUGE_NEVER)
#define SHMEM_HUGE_DEFAULT SHMEM_HUGE_NEVER
#elif defined(CONFIG_TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ALWAYS)
#define SHMEM_HUGE_DEFAULT SHMEM_HUGE_ALWAYS
#elif defined(CONFIG_TRANSPARENT_HUGEPAGE_SHMEM_HUGE_WITHIN_SIZE)
#define SHMEM_HUGE_DEFAULT SHMEM_HUGE_WITHIN_SIZE
#elif defined(CONFIG_TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ADVISE)
#define SHMEM_HUGE_DEFAULT SHMEM_HUGE_ADVISE
#else
#define SHMEM_HUGE_DEFAULT SHMEM_HUGE_NEVER
#endif

int rust_shmem_data_shmem_huge __read_mostly = SHMEM_HUGE_DEFAULT;

#undef SHMEM_HUGE_DEFAULT

#if defined(CONFIG_TRANSPARENT_HUGEPAGE_TMPFS_HUGE_NEVER)
#define TMPFS_HUGE_DEFAULT SHMEM_HUGE_NEVER
#elif defined(CONFIG_TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ALWAYS)
#define TMPFS_HUGE_DEFAULT SHMEM_HUGE_ALWAYS
#elif defined(CONFIG_TRANSPARENT_HUGEPAGE_TMPFS_HUGE_WITHIN_SIZE)
#define TMPFS_HUGE_DEFAULT SHMEM_HUGE_WITHIN_SIZE
#elif defined(CONFIG_TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ADVISE)
#define TMPFS_HUGE_DEFAULT SHMEM_HUGE_ADVISE
#else
#define TMPFS_HUGE_DEFAULT SHMEM_HUGE_NEVER
#endif

int rust_shmem_data_tmpfs_huge __read_mostly = TMPFS_HUGE_DEFAULT;

#undef TMPFS_HUGE_DEFAULT

extern unsigned int rust_shmem_owner_shmem_get_orders_within_size(struct inode *inode,
		unsigned long within_size_orders, pgoff_t index,
		loff_t write_end);

extern unsigned int rust_shmem_owner_shmem_huge_global_enabled(struct inode *inode, pgoff_t index,
					      loff_t write_end, bool shmem_huge_force,
					      struct vm_area_struct *vma,
					      vm_flags_t vm_flags);

extern int rust_shmem_owner_shmem_parse_huge(const char *str);

#if defined(CONFIG_SYSFS) || defined(CONFIG_TMPFS)
extern const char *rust_shmem_owner_shmem_format_huge(int huge);
#endif

extern unsigned long rust_shmem_owner_shmem_unused_huge_shrink(struct shmem_sb_info *sbinfo,
		struct shrink_control *sc, unsigned long nr_to_free);

extern long rust_shmem_owner_shmem_unused_huge_scan(struct super_block *sb,
		struct shrink_control *sc);

extern long rust_shmem_owner_shmem_unused_huge_count(struct super_block *sb,
		struct shrink_control *sc);
#else /* !CONFIG_TRANSPARENT_HUGEPAGE */

#define rust_shmem_data_shmem_huge SHMEM_HUGE_DENY

extern unsigned long rust_shmem_owner_shmem_unused_huge_shrink(struct shmem_sb_info *sbinfo,
		struct shrink_control *sc, unsigned long nr_to_free);

extern unsigned int rust_shmem_owner_shmem_huge_global_enabled(struct inode *inode, pgoff_t index,
					      loff_t write_end, bool shmem_huge_force,
					      struct vm_area_struct *vma,
					      vm_flags_t vm_flags);
#endif /* CONFIG_TRANSPARENT_HUGEPAGE */

extern void rust_shmem_owner_shmem_update_stats(struct folio *folio, int nr_pages);

/*
 * Somewhat like filemap_add_folio, but error if expected item has gone.
 */
extern int shmem_add_to_page_cache(struct folio *folio,
			    struct address_space *mapping,
			    pgoff_t index, void *expected, gfp_t gfp);

/*
 * Somewhat like filemap_remove_folio, but substitutes swap for @folio.
 */
extern void rust_shmem_owner_shmem_delete_from_page_cache(struct folio *folio, void *radswap);

/*
 * Remove swap entry from page cache, free the swap and its page cache. Returns
 * the number of pages being freed. 0 means entry not found in XArray (0 pages
 * being freed).
 */
extern long rust_shmem_owner_shmem_free_swap(struct address_space *mapping,
			    pgoff_t index, pgoff_t end, void *radswap);

/*
 * Determine (in bytes) how many of the shmem object's pages mapped by the
 * given offsets are swapped out.
 *
 * This is safe to call without i_rwsem or the i_pages lock thanks to RCU,
 * as long as the inode doesn't go away and racy results are not a problem.
 */
extern unsigned long shmem_partial_swap_usage(struct address_space *mapping,
						pgoff_t start, pgoff_t end);

/*
 * Determine (in bytes) how many of the shmem object's pages mapped by the
 * given vma is swapped out.
 *
 * This is safe to call without i_rwsem or the i_pages lock thanks to RCU,
 * as long as the inode doesn't go away and racy results are not a problem.
 */
extern unsigned long shmem_swap_usage(struct vm_area_struct *vma);

/*
 * SysV IPC SHM_UNLOCK restore Unevictable pages to their evictable lists.
 */
extern void shmem_unlock_mapping(struct address_space *mapping);

extern struct folio *rust_shmem_owner_shmem_get_partial_folio(struct inode *inode, pgoff_t index);

/*
 * Remove range of pages and swap entries from page cache, and free them.
 * If !unfalloc, truncate or punch hole; if unfalloc, undo failed fallocate.
 */
extern void rust_shmem_owner_shmem_undo_range(struct inode *inode, loff_t lstart, uoff_t lend,
								 bool unfalloc);

extern void shmem_truncate_range(struct inode *inode, loff_t lstart, uoff_t lend);
EXPORT_SYMBOL_GPL(shmem_truncate_range);

extern int rust_shmem_owner_shmem_getattr(struct mnt_idmap *idmap,
			 const struct path *path, struct kstat *stat,
			 u32 request_mask, unsigned int query_flags);

extern int rust_shmem_owner_shmem_setattr(struct mnt_idmap *idmap,
			 struct dentry *dentry, struct iattr *attr);

extern void rust_shmem_owner_shmem_evict_inode(struct inode *inode);

extern unsigned int rust_shmem_owner_shmem_find_swap_entries(struct address_space *mapping,
				pgoff_t start, struct folio_batch *fbatch,
				pgoff_t *indices, unsigned int type);

/*
 * Move the swapped pages for an inode to page cache. Returns the count
 * of pages swapped in, or the error in case of failure.
 */
extern int rust_shmem_owner_shmem_unuse_swap_entries(struct inode *inode,
		struct folio_batch *fbatch, pgoff_t *indices);

/*
 * If swap found in inode, free it and move page from swapcache to filecache.
 */
extern int rust_shmem_owner_shmem_unuse_inode(struct inode *inode, unsigned int type);

/*
 * Read all the shared memory data that resides in the swap
 * device 'type' back into memory, so the swap device can be
 * unused.
 */
extern int shmem_unuse(unsigned int type);

/**
 * shmem_writeout - Write the folio to swap
 * @ctx: swap I/O context
 * @folio: The folio to write
 * @folio_list: list to put back folios on split
 *
 * Move the folio from the page cache to the swap cache.
 */
extern int shmem_writeout(struct swap_io_ctx *ctx, struct folio *folio,
		struct list_head *folio_list);

extern int shmem_write_folio(struct folio *folio);
EXPORT_SYMBOL_GPL(shmem_write_folio);

#if defined(CONFIG_NUMA) && defined(CONFIG_TMPFS)
extern void rust_shmem_owner_shmem_show_mpol(struct seq_file *seq, struct mempolicy *mpol);

extern struct mempolicy *rust_shmem_owner_shmem_get_sbmpol(struct shmem_sb_info *sbinfo);
#else /* !CONFIG_NUMA || !CONFIG_TMPFS */
extern void rust_shmem_owner_shmem_show_mpol(struct seq_file *seq, struct mempolicy *mpol);
extern struct mempolicy *rust_shmem_owner_shmem_get_sbmpol(struct shmem_sb_info *sbinfo);
#endif /* CONFIG_NUMA && CONFIG_TMPFS */

struct mempolicy *rust_shmem_owner_shmem_get_pgoff_policy(struct shmem_inode_info *info,
			pgoff_t index, unsigned int order, pgoff_t *ilx);

extern struct folio *rust_shmem_owner_shmem_swapin_cluster(swp_entry_t swap, gfp_t gfp,
			struct shmem_inode_info *info, pgoff_t index);

#ifdef CONFIG_TRANSPARENT_HUGEPAGE
extern bool shmem_hpage_pmd_enabled(void);

extern unsigned long shmem_allowable_huge_orders(struct inode *inode,
				struct vm_area_struct *vma, pgoff_t index,
				loff_t write_end, bool shmem_huge_force);

extern unsigned long rust_shmem_owner_shmem_suitable_orders(struct inode *inode, struct vm_fault *vmf,
					   struct address_space *mapping, pgoff_t index,
					   unsigned long orders);
#else
extern unsigned long rust_shmem_owner_shmem_suitable_orders(struct inode *inode, struct vm_fault *vmf,
					   struct address_space *mapping, pgoff_t index,
					   unsigned long orders);
#endif /* CONFIG_TRANSPARENT_HUGEPAGE */

extern struct folio *rust_shmem_owner_shmem_alloc_folio(gfp_t gfp, int order,
		struct shmem_inode_info *info, pgoff_t index);

extern struct folio *rust_shmem_owner_shmem_alloc_and_add_folio(struct vm_fault *vmf,
		gfp_t gfp, struct inode *inode, pgoff_t index,
		struct mm_struct *fault_mm, unsigned long orders);

extern struct folio *rust_shmem_owner_shmem_swap_alloc_folio(struct inode *inode,
		struct vm_fault *vmf, pgoff_t index,
		swp_entry_t entry, int order, gfp_t gfp);

/*
 * When a page is moved from swapcache to shmem filecache (either by the
 * usual swapin of shmem_get_folio_gfp(), or by the less common swapoff of
 * shmem_unuse_inode()), it may have been read in earlier from swap, in
 * ignorance of the mapping it belongs to.  If that mapping has special
 * constraints (like the gma500 GEM driver, which requires RAM below 4GB),
 * we may need to copy to a suitable page before moving to filecache.
 *
 * In a future release, this may well be extended to respect cpuset and
 * NUMA mempolicy, and applied also to anonymous pages in do_swap_page();
 * but for now it is a simple matter of zone.
 */
extern bool rust_shmem_owner_shmem_should_replace_folio(struct folio *folio, gfp_t gfp);

extern int rust_shmem_owner_shmem_replace_folio(struct folio **foliop, gfp_t gfp,
				struct shmem_inode_info *info, pgoff_t index,
				struct vm_area_struct *vma);

extern void rust_shmem_owner_shmem_set_folio_swapin_error(struct inode *inode, pgoff_t index,
					 struct folio *folio, swp_entry_t swap);

extern int rust_shmem_owner_shmem_split_large_entry(struct inode *inode, pgoff_t index,
				   swp_entry_t swap, gfp_t gfp);

/*
 * Swap in the folio pointed to by *foliop.
 * Caller has to make sure that *foliop contains a valid swapped folio.
 * Returns 0 and the folio in foliop if success. On failure, returns the
 * error code and NULL in *foliop.
 */
extern int rust_shmem_owner_shmem_swapin_folio(struct inode *inode, pgoff_t index,
			     struct folio **foliop, enum sgp_type sgp,
			     gfp_t gfp, struct vm_fault *vmf,
			     vm_fault_t *fault_type);

/*
 * shmem_get_folio_gfp - find page in cache, or get from swap, or allocate
 *
 * If we allocate a new one we do not mark it dirty. That's up to the
 * vm. If we swap it in we mark it dirty since we also free the swap
 * entry since a page cannot live in both the swap and page cache.
 *
 * vmf and fault_type are only supplied by shmem_fault: otherwise they are NULL.
 */
extern int rust_shmem_owner_shmem_get_folio_gfp(struct inode *inode, pgoff_t index,
		loff_t write_end, struct folio **foliop, enum sgp_type sgp,
		gfp_t gfp, struct vm_fault *vmf, vm_fault_t *fault_type);

/**
 * shmem_get_folio - find, and lock a shmem folio.
 * @inode:	inode to search
 * @index:	the page index.
 * @write_end:	end of a write, could extend inode size
 * @foliop:	pointer to the folio if found
 * @sgp:	SGP_* flags to control behavior
 *
 * Looks up the page cache entry at @inode & @index.  If a folio is
 * present, it is returned locked with an increased refcount.
 *
 * If the caller modifies data in the folio, it must call folio_mark_dirty()
 * before unlocking the folio to ensure that the folio is not reclaimed.
 * There is no need to reserve space before calling folio_mark_dirty().
 *
 * When no folio is found, the behavior depends on @sgp:
 *  - for SGP_READ, *@foliop is %NULL and 0 is returned
 *  - for SGP_NOALLOC, *@foliop is %NULL and -ENOENT is returned
 *  - for all other flags a new folio is allocated, inserted into the
 *    page cache and returned locked in @foliop.
 *
 * Context: May sleep.
 * Return: 0 if successful, else a negative error code.
 */
extern int shmem_get_folio(struct inode *inode, pgoff_t index, loff_t write_end,
		    struct folio **foliop, enum sgp_type sgp);
EXPORT_SYMBOL_GPL(shmem_get_folio);

/*
 * This is like autoremove_wake_function, but it removes the wait queue
 * entry unconditionally - even if something else had already woken the
 * target.
 */
extern int rust_shmem_owner_synchronous_wake_function(wait_queue_entry_t *wait,
			unsigned int mode, int sync, void *key);

/*
 * Trinity finds that probing a hole which tmpfs is punching can
 * prevent the hole-punch from ever completing: which in turn
 * locks writers out with its hold on i_rwsem.  So refrain from
 * faulting pages into the hole while it's being punched.  Although
 * shmem_undo_range() does remove the additions, it may be unable to
 * keep up, as each new page needs its own unmap_mapping_range() call,
 * and the i_mmap tree grows ever slower to scan if new vmas are added.
 *
 * It does not matter if we sometimes reach this check just before the
 * hole-punch begins, so that one fault then races with the punch:
 * we just need to make racing faults a rare case.
 *
 * The implementation below would be much simpler if we just used a
 * standard mutex or completion: but we cannot take i_rwsem in fault,
 * and bloating every shmem inode for this unlikely case would be sad.
 */
extern vm_fault_t rust_shmem_owner_shmem_falloc_wait(struct vm_fault *vmf, struct inode *inode);

extern vm_fault_t rust_shmem_owner_shmem_fault(struct vm_fault *vmf);

extern unsigned long shmem_get_unmapped_area(struct file *file,
				      unsigned long uaddr, unsigned long len,
				      unsigned long pgoff, unsigned long flags);

#ifdef CONFIG_NUMA
extern int rust_shmem_owner_shmem_set_policy(struct vm_area_struct *vma, struct mempolicy *mpol);

extern struct mempolicy *rust_shmem_owner_shmem_get_policy(struct vm_area_struct *vma,
					  unsigned long addr, pgoff_t *ilx);

extern struct mempolicy *rust_shmem_owner_shmem_get_pgoff_policy(struct shmem_inode_info *info,
			pgoff_t index, unsigned int order, pgoff_t *ilx);
#else
extern struct mempolicy *rust_shmem_owner_shmem_get_pgoff_policy(struct shmem_inode_info *info,
			pgoff_t index, unsigned int order, pgoff_t *ilx);
#endif /* CONFIG_NUMA */

extern int shmem_lock(struct file *file, int lock, struct ucounts *ucounts);

extern int rust_shmem_owner_shmem_mmap_prepare(struct vm_area_desc *desc);

extern int rust_shmem_owner_shmem_file_open(struct inode *inode, struct file *file);

#ifdef CONFIG_TMPFS_XATTR
int rust_shmem_owner_shmem_initxattrs(struct inode *, const struct xattr *, void *);

#if IS_ENABLED(CONFIG_UNICODE)
/*
 * shmem_inode_casefold_flags - Deal with casefold file attribute flag
 *
 * The casefold file attribute needs some special checks. I can just be added to
 * an empty dir, and can't be removed from a non-empty dir.
 */
extern int rust_shmem_owner_shmem_inode_casefold_flags(struct inode *inode, unsigned int fsflags,
				      struct dentry *dentry, unsigned int *i_flags);
#else
extern int rust_shmem_owner_shmem_inode_casefold_flags(struct inode *inode, unsigned int fsflags,
				      struct dentry *dentry, unsigned int *i_flags);
#endif

/*
 * chattr's fsflags are unrelated to extended attributes,
 * but tmpfs has chosen to enable them under the same config option.
 */
extern int rust_shmem_owner_shmem_set_inode_flags(struct inode *inode, unsigned int fsflags, struct dentry *dentry);
#else
extern void rust_shmem_owner_shmem_set_inode_flags(struct inode *inode, unsigned int fsflags, struct dentry *dentry);
#define rust_shmem_owner_shmem_initxattrs NULL
#endif

extern struct offset_ctx *rust_shmem_owner_shmem_get_offset_ctx(struct inode *inode);

extern struct inode *rust_shmem_owner___shmem_get_inode(struct mnt_idmap *idmap,
				       struct super_block *sb,
				       struct inode *dir, umode_t mode,
				       dev_t dev, vma_flags_t flags);

#ifdef CONFIG_TMPFS_QUOTA
extern struct inode *rust_shmem_owner_shmem_get_inode(struct mnt_idmap *idmap,
				     struct super_block *sb, struct inode *dir,
				     umode_t mode, dev_t dev, vma_flags_t flags);
#else
extern struct inode *rust_shmem_owner_shmem_get_inode(struct mnt_idmap *idmap,
				     struct super_block *sb, struct inode *dir,
				     umode_t mode, dev_t dev, vma_flags_t flags);
#endif /* CONFIG_TMPFS_QUOTA */

#ifdef CONFIG_USERFAULTFD
extern struct folio *rust_shmem_owner_shmem_mfill_folio_alloc(struct vm_area_struct *vma,
					     unsigned long addr);

extern int rust_shmem_owner_shmem_mfill_filemap_add(struct folio *folio,
				   struct vm_area_struct *vma,
				   unsigned long addr);

extern void rust_shmem_owner_shmem_mfill_filemap_remove(struct folio *folio,
				       struct vm_area_struct *vma);

extern struct folio *rust_shmem_owner_shmem_get_folio_noalloc(struct inode *inode, pgoff_t pgoff);

extern bool rust_shmem_owner_shmem_can_userfault(struct vm_area_struct *vma, vm_flags_t vm_flags);

const struct vm_uffd_ops rust_shmem_data_shmem_uffd_ops = {
	.can_userfault		= rust_shmem_owner_shmem_can_userfault,
	.get_folio_noalloc	= rust_shmem_owner_shmem_get_folio_noalloc,
	.alloc_folio		= rust_shmem_owner_shmem_mfill_folio_alloc,
	.filemap_add		= rust_shmem_owner_shmem_mfill_filemap_add,
	.filemap_remove		= rust_shmem_owner_shmem_mfill_filemap_remove,
};
#endif /* CONFIG_USERFAULTFD */

#ifdef CONFIG_TMPFS
extern const struct inode_operations rust_shmem_data_shmem_symlink_inode_operations;
extern const struct inode_operations rust_shmem_data_shmem_short_symlink_operations;

extern int
rust_shmem_owner_shmem_write_begin(const struct kiocb *iocb, struct address_space *mapping,
		  loff_t pos, unsigned len,
		  struct folio **foliop, void **fsdata);

extern int
rust_shmem_owner_shmem_write_end(const struct kiocb *iocb, struct address_space *mapping,
		loff_t pos, unsigned len, unsigned copied,
		struct folio *folio, void *fsdata);

extern ssize_t rust_shmem_owner_shmem_file_read_iter(struct kiocb *iocb, struct iov_iter *to);

extern ssize_t rust_shmem_owner_shmem_file_write_iter(struct kiocb *iocb, struct iov_iter *from);

extern bool rust_shmem_owner_zero_pipe_buf_get(struct pipe_inode_info *pipe,
			      struct pipe_buffer *buf);

extern void rust_shmem_owner_zero_pipe_buf_release(struct pipe_inode_info *pipe,
				  struct pipe_buffer *buf);

extern bool rust_shmem_owner_zero_pipe_buf_try_steal(struct pipe_inode_info *pipe,
				    struct pipe_buffer *buf);

const struct pipe_buf_operations rust_shmem_data_zero_pipe_buf_ops = {
	.release	= rust_shmem_owner_zero_pipe_buf_release,
	.try_steal	= rust_shmem_owner_zero_pipe_buf_try_steal,
	.get		= rust_shmem_owner_zero_pipe_buf_get,
};

extern size_t rust_shmem_owner_splice_zeropage_into_pipe(struct pipe_inode_info *pipe,
					loff_t fpos, size_t size);

extern ssize_t rust_shmem_owner_shmem_file_splice_read(struct file *in, loff_t *ppos,
				      struct pipe_inode_info *pipe,
				      size_t len, unsigned int flags);

extern loff_t rust_shmem_owner_shmem_file_llseek(struct file *file, loff_t offset, int whence);

extern long rust_shmem_owner_shmem_fallocate(struct file *file, int mode, loff_t offset,
							 loff_t len);

extern int rust_shmem_owner_shmem_statfs(struct dentry *dentry, struct kstatfs *buf);

/*
 * File creation. Allocate an inode, and we're done..
 */
extern int
rust_shmem_owner_shmem_mknod(struct mnt_idmap *idmap, struct inode *dir,
	    struct dentry *dentry, umode_t mode, dev_t dev);

extern int
rust_shmem_owner_shmem_tmpfile(struct mnt_idmap *idmap, struct inode *dir,
	      struct file *file, umode_t mode);

extern struct dentry *rust_shmem_owner_shmem_mkdir(struct mnt_idmap *idmap, struct inode *dir,
				  struct dentry *dentry, umode_t mode);

extern int rust_shmem_owner_shmem_create(struct mnt_idmap *idmap, struct inode *dir,
			struct dentry *dentry, umode_t mode);

/*
 * Link a file..
 */
extern int rust_shmem_owner_shmem_link(struct dentry *old_dentry, struct inode *dir,
		      struct dentry *dentry);

extern int rust_shmem_owner_shmem_unlink(struct inode *dir, struct dentry *dentry);

extern int rust_shmem_owner_shmem_rmdir(struct inode *dir, struct dentry *dentry);

extern int rust_shmem_owner_shmem_whiteout(struct mnt_idmap *idmap,
			  struct inode *old_dir, struct dentry *old_dentry);

/*
 * The VFS layer already does all the dentry stuff for rename,
 * we just have to decrement the usage count for the target if
 * it exists so that the VFS layer correctly free's it when it
 * gets overwritten.
 */
extern int rust_shmem_owner_shmem_rename2(struct mnt_idmap *idmap,
			 struct inode *old_dir, struct dentry *old_dentry,
			 struct inode *new_dir, struct dentry *new_dentry,
			 unsigned int flags);

extern int rust_shmem_owner_shmem_symlink(struct mnt_idmap *idmap, struct inode *dir,
			 struct dentry *dentry, const char *symname);

extern void rust_shmem_owner_shmem_put_link(void *arg);

extern const char *rust_shmem_owner_shmem_get_link(struct dentry *dentry, struct inode *inode,
				  struct delayed_call *done);

#ifdef CONFIG_TMPFS_XATTR

extern int rust_shmem_owner_shmem_fileattr_get(struct dentry *dentry, struct file_kattr *fa);

extern int rust_shmem_owner_shmem_fileattr_set(struct mnt_idmap *idmap,
			      struct dentry *dentry, struct file_kattr *fa);

/*
 * Superblocks without xattr inode operations may get some security.* xattr
 * support from the LSM "for free". As soon as we have any other xattrs
 * like ACLs, we also need to implement the security.* handlers at
 * filesystem level, though.
 */

/*
 * Callback for security_inode_init_security() for acquiring xattrs.
 */
extern int rust_shmem_owner_shmem_initxattrs(struct inode *inode,
			    const struct xattr *xattr_array, void *fs_info);

extern int rust_shmem_owner_shmem_xattr_handler_get(const struct xattr_handler *handler,
				   struct dentry *unused, struct inode *inode,
				   const char *name, void *buffer, size_t size);

extern int rust_shmem_owner_shmem_xattr_handler_set(const struct xattr_handler *handler,
				   struct mnt_idmap *idmap,
				   struct dentry *unused, struct inode *inode,
				   const char *name, const void *value,
				   size_t size, int flags);

const struct xattr_handler rust_shmem_data_shmem_security_xattr_handler = {
	.prefix = XATTR_SECURITY_PREFIX,
	.get = rust_shmem_owner_shmem_xattr_handler_get,
	.set = rust_shmem_owner_shmem_xattr_handler_set,
};

const struct xattr_handler rust_shmem_data_shmem_trusted_xattr_handler = {
	.prefix = XATTR_TRUSTED_PREFIX,
	.get = rust_shmem_owner_shmem_xattr_handler_get,
	.set = rust_shmem_owner_shmem_xattr_handler_set,
};

const struct xattr_handler rust_shmem_data_shmem_user_xattr_handler = {
	.prefix = XATTR_USER_PREFIX,
	.get = rust_shmem_owner_shmem_xattr_handler_get,
	.set = rust_shmem_owner_shmem_xattr_handler_set,
};

const struct xattr_handler * const rust_shmem_data_shmem_xattr_handlers[] = {
	&rust_shmem_data_shmem_security_xattr_handler,
	&rust_shmem_data_shmem_trusted_xattr_handler,
	&rust_shmem_data_shmem_user_xattr_handler,
	NULL
};

extern ssize_t rust_shmem_owner_shmem_listxattr(struct dentry *dentry, char *buffer, size_t size);
#endif /* CONFIG_TMPFS_XATTR */

const struct inode_operations rust_shmem_data_shmem_short_symlink_operations = {
	.getattr	= rust_shmem_owner_shmem_getattr,
	.setattr	= rust_shmem_owner_shmem_setattr,
	.get_link	= simple_get_link,
#ifdef CONFIG_TMPFS_XATTR
	.listxattr	= rust_shmem_owner_shmem_listxattr,
#endif
};

const struct inode_operations rust_shmem_data_shmem_symlink_inode_operations = {
	.getattr	= rust_shmem_owner_shmem_getattr,
	.setattr	= rust_shmem_owner_shmem_setattr,
	.get_link	= rust_shmem_owner_shmem_get_link,
#ifdef CONFIG_TMPFS_XATTR
	.listxattr	= rust_shmem_owner_shmem_listxattr,
#endif
};

extern struct dentry *rust_shmem_owner_shmem_get_parent(struct dentry *child);

extern int rust_shmem_owner_shmem_match(struct inode *ino, void *vfh);

/* Find any alias of inode, but prefer a hashed alias */
extern struct dentry *rust_shmem_owner_shmem_find_alias(struct inode *inode);

extern struct dentry *rust_shmem_owner_shmem_fh_to_dentry(struct super_block *sb,
		struct fid *fid, int fh_len, int fh_type);

extern int rust_shmem_owner_shmem_encode_fh(struct inode *inode, __u32 *fh, int *len,
				struct inode *parent);

const struct export_operations rust_shmem_data_shmem_export_ops = {
	.get_parent     = rust_shmem_owner_shmem_get_parent,
	.encode_fh      = rust_shmem_owner_shmem_encode_fh,
	.fh_to_dentry	= rust_shmem_owner_shmem_fh_to_dentry,
};

enum shmem_param {
	Opt_gid,
	Opt_huge,
	Opt_mode,
	Opt_mpol,
	Opt_nr_blocks,
	Opt_nr_inodes,
	Opt_size,
	Opt_uid,
	Opt_inode32,
	Opt_inode64,
	Opt_noswap,
	Opt_quota,
	Opt_usrquota,
	Opt_grpquota,
	Opt_usrquota_block_hardlimit,
	Opt_usrquota_inode_hardlimit,
	Opt_grpquota_block_hardlimit,
	Opt_grpquota_inode_hardlimit,
	Opt_casefold_version,
	Opt_casefold,
	Opt_strict_encoding,
};

const struct constant_table rust_shmem_data_shmem_param_enums_huge[] = {
	{"never",	SHMEM_HUGE_NEVER },
	{"always",	SHMEM_HUGE_ALWAYS },
	{"within_size",	SHMEM_HUGE_WITHIN_SIZE },
	{"advise",	SHMEM_HUGE_ADVISE },
	{}
};

const struct fs_parameter_spec shmem_fs_parameters[] = {
	fsparam_gid   ("gid",		Opt_gid),
	fsparam_enum  ("huge",		Opt_huge,  rust_shmem_data_shmem_param_enums_huge),
	fsparam_u32oct("mode",		Opt_mode),
	fsparam_string("mpol",		Opt_mpol),
	fsparam_string("nr_blocks",	Opt_nr_blocks),
	fsparam_string("nr_inodes",	Opt_nr_inodes),
	fsparam_string("size",		Opt_size),
	fsparam_uid   ("uid",		Opt_uid),
	fsparam_flag  ("inode32",	Opt_inode32),
	fsparam_flag  ("inode64",	Opt_inode64),
	fsparam_flag  ("noswap",	Opt_noswap),
#ifdef CONFIG_TMPFS_QUOTA
	fsparam_flag  ("quota",		Opt_quota),
	fsparam_flag  ("usrquota",	Opt_usrquota),
	fsparam_flag  ("grpquota",	Opt_grpquota),
	fsparam_string("usrquota_block_hardlimit", Opt_usrquota_block_hardlimit),
	fsparam_string("usrquota_inode_hardlimit", Opt_usrquota_inode_hardlimit),
	fsparam_string("grpquota_block_hardlimit", Opt_grpquota_block_hardlimit),
	fsparam_string("grpquota_inode_hardlimit", Opt_grpquota_inode_hardlimit),
#endif
	fsparam_string("casefold",	Opt_casefold_version),
	fsparam_flag  ("casefold",	Opt_casefold),
	fsparam_flag  ("strict_encoding", Opt_strict_encoding),
	{}
};

#if IS_ENABLED(CONFIG_UNICODE)
extern int rust_shmem_owner_shmem_parse_opt_casefold(struct fs_context *fc, struct fs_parameter *param,
				    bool latest_version);
#else
extern int rust_shmem_owner_shmem_parse_opt_casefold(struct fs_context *fc, struct fs_parameter *param,
				    bool latest_version);
#endif

extern int rust_shmem_owner_shmem_parse_one(struct fs_context *fc, struct fs_parameter *param);

extern char *rust_shmem_owner_shmem_next_opt(char **s);

extern int rust_shmem_owner_shmem_parse_monolithic(struct fs_context *fc, void *data);

/*
 * Reconfigure a shmem filesystem.
 */
extern int rust_shmem_owner_shmem_reconfigure(struct fs_context *fc);

extern int rust_shmem_owner_shmem_show_options(struct seq_file *seq, struct dentry *root);

#endif /* CONFIG_TMPFS */

extern void rust_shmem_owner_shmem_put_super(struct super_block *sb);

#if IS_ENABLED(CONFIG_UNICODE) && defined(CONFIG_TMPFS)
const struct dentry_operations rust_shmem_data_shmem_ci_dentry_ops = {
	.d_hash = generic_ci_d_hash,
	.d_compare = generic_ci_d_compare,
};
#endif

extern int rust_shmem_owner_shmem_fill_super(struct super_block *sb, struct fs_context *fc);

extern int rust_shmem_owner_shmem_get_tree(struct fs_context *fc);

extern void rust_shmem_owner_shmem_free_fc(struct fs_context *fc);

const struct fs_context_operations rust_shmem_data_shmem_fs_context_ops = {
	.free			= rust_shmem_owner_shmem_free_fc,
	.get_tree		= rust_shmem_owner_shmem_get_tree,
#ifdef CONFIG_TMPFS
	.parse_monolithic	= rust_shmem_owner_shmem_parse_monolithic,
	.parse_param		= rust_shmem_owner_shmem_parse_one,
	.reconfigure		= rust_shmem_owner_shmem_reconfigure,
#endif
};

struct kmem_cache *rust_shmem_data_shmem_inode_cachep __ro_after_init;

extern struct inode *rust_shmem_owner_shmem_alloc_inode(struct super_block *sb);

extern void rust_shmem_owner_shmem_free_in_core_inode(struct inode *inode);

extern void rust_shmem_owner_shmem_destroy_inode(struct inode *inode);

extern void rust_shmem_owner_shmem_init_inode(void *foo);

extern void  rust_shmem_owner_shmem_init_inodecache(void);

extern void  rust_shmem_owner_shmem_destroy_inodecache(void);

/* Keep the page in page cache instead of truncating it */
extern int rust_shmem_owner_shmem_error_remove_folio(struct address_space *mapping,
				   struct folio *folio);

const struct address_space_operations rust_shmem_data_shmem_aops = {
	.dirty_folio	= noop_dirty_folio,
#ifdef CONFIG_TMPFS
	.write_begin	= rust_shmem_owner_shmem_write_begin,
	.write_end	= rust_shmem_owner_shmem_write_end,
#endif
#ifdef CONFIG_MIGRATION
	.migrate_folio	= migrate_folio,
#endif
	.error_remove_folio = rust_shmem_owner_shmem_error_remove_folio,
};

const struct file_operations rust_shmem_data_shmem_file_operations = {
	.mmap_prepare	= rust_shmem_owner_shmem_mmap_prepare,
	.open		= rust_shmem_owner_shmem_file_open,
	.get_unmapped_area = shmem_get_unmapped_area,
#ifdef CONFIG_TMPFS
	.llseek		= rust_shmem_owner_shmem_file_llseek,
	.read_iter	= rust_shmem_owner_shmem_file_read_iter,
	.write_iter	= rust_shmem_owner_shmem_file_write_iter,
	.fsync		= noop_fsync,
	.splice_read	= rust_shmem_owner_shmem_file_splice_read,
	.splice_write	= iter_file_splice_write,
	.fallocate	= rust_shmem_owner_shmem_fallocate,
	.setlease	= generic_setlease,
#endif
};

const struct inode_operations rust_shmem_data_shmem_inode_operations = {
	.getattr	= rust_shmem_owner_shmem_getattr,
	.setattr	= rust_shmem_owner_shmem_setattr,
#ifdef CONFIG_TMPFS_XATTR
	.listxattr	= rust_shmem_owner_shmem_listxattr,
	.set_acl	= simple_set_acl,
	.fileattr_get	= rust_shmem_owner_shmem_fileattr_get,
	.fileattr_set	= rust_shmem_owner_shmem_fileattr_set,
#endif
};

const struct inode_operations rust_shmem_data_shmem_dir_inode_operations = {
#ifdef CONFIG_TMPFS
	.getattr	= rust_shmem_owner_shmem_getattr,
	.create		= rust_shmem_owner_shmem_create,
	.lookup		= simple_lookup,
	.link		= rust_shmem_owner_shmem_link,
	.unlink		= rust_shmem_owner_shmem_unlink,
	.symlink	= rust_shmem_owner_shmem_symlink,
	.mkdir		= rust_shmem_owner_shmem_mkdir,
	.rmdir		= rust_shmem_owner_shmem_rmdir,
	.mknod		= rust_shmem_owner_shmem_mknod,
	.rename		= rust_shmem_owner_shmem_rename2,
	.tmpfile	= rust_shmem_owner_shmem_tmpfile,
	.get_offset_ctx	= rust_shmem_owner_shmem_get_offset_ctx,
#endif
#ifdef CONFIG_TMPFS_XATTR
	.listxattr	= rust_shmem_owner_shmem_listxattr,
	.fileattr_get	= rust_shmem_owner_shmem_fileattr_get,
	.fileattr_set	= rust_shmem_owner_shmem_fileattr_set,
#endif
#ifdef CONFIG_TMPFS_POSIX_ACL
	.setattr	= rust_shmem_owner_shmem_setattr,
	.set_acl	= simple_set_acl,
#endif
};

const struct inode_operations rust_shmem_data_shmem_special_inode_operations = {
	.getattr	= rust_shmem_owner_shmem_getattr,
#ifdef CONFIG_TMPFS_XATTR
	.listxattr	= rust_shmem_owner_shmem_listxattr,
#endif
#ifdef CONFIG_TMPFS_POSIX_ACL
	.setattr	= rust_shmem_owner_shmem_setattr,
	.set_acl	= simple_set_acl,
#endif
};

const struct super_operations rust_shmem_data_shmem_ops = {
	.alloc_inode	= rust_shmem_owner_shmem_alloc_inode,
	.free_inode	= rust_shmem_owner_shmem_free_in_core_inode,
	.destroy_inode	= rust_shmem_owner_shmem_destroy_inode,
#ifdef CONFIG_TMPFS
	.statfs		= rust_shmem_owner_shmem_statfs,
	.show_options	= rust_shmem_owner_shmem_show_options,
#endif
#ifdef CONFIG_TMPFS_QUOTA
	.get_dquots	= rust_shmem_owner_shmem_get_dquots,
#endif
	.evict_inode	= rust_shmem_owner_shmem_evict_inode,
	.drop_inode	= inode_just_drop,
	.put_super	= rust_shmem_owner_shmem_put_super,
#ifdef CONFIG_TRANSPARENT_HUGEPAGE
	.nr_cached_objects	= rust_shmem_owner_shmem_unused_huge_count,
	.free_cached_objects	= rust_shmem_owner_shmem_unused_huge_scan,
#endif
};

const struct vm_operations_struct rust_shmem_data_shmem_vm_ops = {
	.fault		= rust_shmem_owner_shmem_fault,
	.map_pages	= filemap_map_pages,
#ifdef CONFIG_NUMA
	.set_policy     = rust_shmem_owner_shmem_set_policy,
	.get_policy     = rust_shmem_owner_shmem_get_policy,
#endif
#ifdef CONFIG_USERFAULTFD
	.uffd_ops	= &rust_shmem_data_shmem_uffd_ops,
#endif
};

const struct vm_operations_struct rust_shmem_data_shmem_anon_vm_ops = {
	.fault		= rust_shmem_owner_shmem_fault,
	.map_pages	= filemap_map_pages,
#ifdef CONFIG_NUMA
	.set_policy     = rust_shmem_owner_shmem_set_policy,
	.get_policy     = rust_shmem_owner_shmem_get_policy,
#endif
#ifdef CONFIG_USERFAULTFD
	.uffd_ops	= &rust_shmem_data_shmem_uffd_ops,
#endif
};

extern int shmem_init_fs_context(struct fs_context *fc);

struct file_system_type rust_shmem_data_shmem_fs_type = {
	.owner		= THIS_MODULE,
	.name		= "tmpfs",
	.init_fs_context = shmem_init_fs_context,
#ifdef CONFIG_TMPFS
	.parameters	= shmem_fs_parameters,
#endif
	.kill_sb	= kill_anon_super,
	.fs_flags	= FS_USERNS_MOUNT | FS_ALLOW_IDMAP | FS_MGTIME,
};

#if defined(CONFIG_SYSFS) && defined(CONFIG_TMPFS)

#define __INIT_KOBJ_ATTR(_name, _mode, _show, _store)			\
{									\
	.attr	= { .name = __stringify(_name), .mode = _mode },	\
	.show	= _show,						\
	.store	= _store,						\
}

#define TMPFS_ATTR_W(_name, _store)				\
	static struct kobj_attribute tmpfs_attr_##_name =	\
			__INIT_KOBJ_ATTR(_name, 0200, NULL, _store)

#define TMPFS_ATTR_RW(_name, _show, _store)			\
	static struct kobj_attribute tmpfs_attr_##_name =	\
			__INIT_KOBJ_ATTR(_name, 0644, _show, _store)

#define TMPFS_ATTR_RO(_name, _show)				\
	static struct kobj_attribute tmpfs_attr_##_name =	\
			__INIT_KOBJ_ATTR(_name, 0444, _show, NULL)

#if IS_ENABLED(CONFIG_UNICODE)
extern ssize_t rust_shmem_owner_casefold_show(struct kobject *kobj, struct kobj_attribute *a,
			char *buf);
TMPFS_ATTR_RO(casefold, rust_shmem_owner_casefold_show);
#endif

struct attribute *rust_shmem_data_tmpfs_attributes[] = {
#if IS_ENABLED(CONFIG_UNICODE)
	&tmpfs_attr_casefold.attr,
#endif
	NULL
};

const struct attribute_group rust_shmem_data_tmpfs_attribute_group = {
	.attrs = rust_shmem_data_tmpfs_attributes,
	.name = "features"
};

struct kobject *rust_shmem_data_tmpfs_kobj;

extern int  rust_shmem_owner_tmpfs_sysfs_init(void);
#endif /* CONFIG_SYSFS && CONFIG_TMPFS */

extern void  shmem_init(void);

#if defined(CONFIG_TRANSPARENT_HUGEPAGE) && defined(CONFIG_SYSFS)
extern ssize_t rust_shmem_owner_shmem_enabled_show(struct kobject *kobj,
				  struct kobj_attribute *attr, char *buf);

extern ssize_t rust_shmem_owner_shmem_enabled_store(struct kobject *kobj,
		struct kobj_attribute *attr, const char *buf, size_t count);

struct kobj_attribute shmem_enabled_attr = __ATTR(shmem_enabled, 0644, rust_shmem_owner_shmem_enabled_show, rust_shmem_owner_shmem_enabled_store);
DEFINE_SPINLOCK(rust_shmem_data_huge_shmem_orders_lock);

enum huge_mode {
	HUGE_SHMEM_ENABLED_ALWAYS = 0,
	HUGE_SHMEM_ENABLED_INHERIT,
	HUGE_SHMEM_ENABLED_WITHIN_SIZE,
	HUGE_SHMEM_ENABLED_ADVISE,
	HUGE_SHMEM_ENABLED_NEVER,
};

const char * const rust_shmem_data_huge_mode_strings[] = {
	[HUGE_SHMEM_ENABLED_ALWAYS]      = "always",
	[HUGE_SHMEM_ENABLED_INHERIT]     = "inherit",
	[HUGE_SHMEM_ENABLED_WITHIN_SIZE] = "within_size",
	[HUGE_SHMEM_ENABLED_ADVISE]      = "advise",
	[HUGE_SHMEM_ENABLED_NEVER]       = "never",
};

unsigned long * const rust_shmem_data_huge_mode_orders[] = {
	[HUGE_SHMEM_ENABLED_ALWAYS]      = &rust_shmem_data_huge_shmem_orders_always,
	[HUGE_SHMEM_ENABLED_INHERIT]     = &rust_shmem_data_huge_shmem_orders_inherit,
	[HUGE_SHMEM_ENABLED_WITHIN_SIZE] = &rust_shmem_data_huge_shmem_orders_within_size,
	[HUGE_SHMEM_ENABLED_ADVISE]      = &rust_shmem_data_huge_shmem_orders_madvise,
};

extern ssize_t rust_shmem_owner_thpsize_shmem_enabled_show(struct kobject *kobj,
					  struct kobj_attribute *attr, char *buf);

extern bool rust_shmem_owner_set_shmem_enabled_mode(int order, enum huge_mode mode);

extern ssize_t rust_shmem_owner_thpsize_shmem_enabled_store(struct kobject *kobj,
					   struct kobj_attribute *attr,
					   const char *buf, size_t count);

struct kobj_attribute thpsize_shmem_enabled_attr =
	__ATTR(shmem_enabled, 0644, rust_shmem_owner_thpsize_shmem_enabled_show, rust_shmem_owner_thpsize_shmem_enabled_store);
#endif /* CONFIG_TRANSPARENT_HUGEPAGE && CONFIG_SYSFS */

#if defined(CONFIG_TRANSPARENT_HUGEPAGE)

extern int  rust_shmem_owner_setup_transparent_hugepage_shmem(char *str);
__setup("transparent_hugepage_shmem=", rust_shmem_owner_setup_transparent_hugepage_shmem);

extern int  rust_shmem_owner_setup_transparent_hugepage_tmpfs(char *str);
__setup("transparent_hugepage_tmpfs=", rust_shmem_owner_setup_transparent_hugepage_tmpfs);

char rust_shmem_data_str_dup[PAGE_SIZE] __initdata;
extern int  rust_shmem_owner_setup_thp_shmem(char *str);
__setup("thp_shmem=", rust_shmem_owner_setup_thp_shmem);

#endif /* CONFIG_TRANSPARENT_HUGEPAGE */

#else /* !CONFIG_SHMEM */

/*
 * tiny-shmem: simple shmemfs and tmpfs using ramfs code
 *
 * This is intended for small system where the benefits of the full
 * shmem code (swap-backed and resource-limited) are outweighed by
 * their complexity. On systems without swap this code should be
 * effectively equivalent, but much lighter weight.
 */

struct file_system_type rust_shmem_data_shmem_fs_type = {
	.name		= "tmpfs",
	.init_fs_context = ramfs_init_fs_context,
	.parameters	= ramfs_fs_parameters,
	.kill_sb	= ramfs_kill_sb,
	.fs_flags	= FS_USERNS_MOUNT,
};

extern void  shmem_init(void);

extern int shmem_unuse(unsigned int type);

extern int shmem_lock(struct file *file, int lock, struct ucounts *ucounts);

extern void shmem_unlock_mapping(struct address_space *mapping);

#ifdef CONFIG_MMU
extern unsigned long shmem_get_unmapped_area(struct file *file,
				      unsigned long addr, unsigned long len,
				      unsigned long pgoff, unsigned long flags);
#endif

extern void shmem_truncate_range(struct inode *inode, loff_t lstart, uoff_t lend);
EXPORT_SYMBOL_GPL(shmem_truncate_range);

#define rust_shmem_data_shmem_vm_ops				generic_file_vm_ops
#define rust_shmem_data_shmem_anon_vm_ops			generic_file_vm_ops
#define rust_shmem_data_shmem_file_operations			ramfs_file_operations

extern int rust_shmem_owner_shmem_acct_size(unsigned long flags, loff_t size);

extern void rust_shmem_owner_shmem_unacct_size(unsigned long flags, loff_t size);

extern struct inode *rust_shmem_owner_shmem_get_inode(struct mnt_idmap *idmap,
				struct super_block *sb, struct inode *dir,
				umode_t mode, dev_t dev, vma_flags_t flags);

#endif /* CONFIG_SHMEM */

/* common code */

extern struct file *rust_shmem_owner___shmem_file_setup(struct vfsmount *mnt, const char *name,
				       loff_t size, vma_flags_t flags,
				       unsigned int i_flags);

/**
 * shmem_kernel_file_setup - get an unlinked file living in tmpfs which must be
 * 	kernel internal.  There will be NO LSM permission checks against the
 * 	underlying inode.  So users of this interface must do LSM checks at a
 *	higher layer.  The users are the big_key and shm implementations.  LSM
 *	checks are provided at the key or shm level rather than the inode.
 * @name: name for dentry (to be seen in /proc/<pid>/maps)
 * @size: size to be set for the file
 * @flags: VMA_NORESERVE_BIT suppresses pre-accounting of the entire object size
 */
extern struct file *shmem_kernel_file_setup(const char *name, loff_t size,
				     vma_flags_t flags);
EXPORT_SYMBOL_GPL(shmem_kernel_file_setup);

/**
 * shmem_file_setup - get an unlinked file living in tmpfs
 * @name: name for dentry (to be seen in /proc/<pid>/maps)
 * @size: size to be set for the file
 * @flags: VMA_NORESERVE_BIT suppresses pre-accounting of the entire object size
 */
extern struct file *shmem_file_setup(const char *name, loff_t size, vma_flags_t flags);
EXPORT_SYMBOL_GPL(shmem_file_setup);

/**
 * shmem_file_setup_with_mnt - get an unlinked file living in tmpfs
 * @mnt: the tmpfs mount where the file will be created
 * @name: name for dentry (to be seen in /proc/<pid>/maps)
 * @size: size to be set for the file
 * @flags: VMA_NORESERVE_BIT suppresses pre-accounting of the entire object size
 */
extern struct file *shmem_file_setup_with_mnt(struct vfsmount *mnt, const char *name,
				       loff_t size, vma_flags_t flags);
EXPORT_SYMBOL_GPL(shmem_file_setup_with_mnt);

extern struct file *rust_shmem_owner___shmem_zero_setup(unsigned long start, unsigned long end,
		vma_flags_t flags);

/**
 * shmem_zero_setup - setup a shared anonymous mapping
 * @vma: the vma to be mmapped is prepared by do_mmap
 * Returns: 0 on success, or error
 */
extern int shmem_zero_setup(struct vm_area_struct *vma);

/**
 * shmem_zero_setup_desc - same as shmem_zero_setup, but determined by VMA
 * descriptor for convenience.
 * @desc: Describes VMA
 * Returns: 0 on success, or error
 */
extern int shmem_zero_setup_desc(struct vm_area_desc *desc);

/**
 * shmem_read_folio_gfp - read into page cache, using specified page allocation flags.
 * @mapping:	the folio's address_space
 * @index:	the folio index
 * @gfp:	the page allocator flags to use if allocating
 *
 * This behaves as a tmpfs "read_cache_page_gfp(mapping, index, gfp)",
 * with any new page allocations done using the specified allocation flags.
 * But read_cache_page_gfp() uses the ->read_folio() method: which does not
 * suit tmpfs, since it may have pages in swapcache, and needs to find those
 * for itself; although drivers/gpu/drm i915 and ttm rely upon this support.
 *
 * i915_gem_object_get_pages_gtt() mixes __GFP_NORETRY | __GFP_NOWARN in
 * with the mapping_gfp_mask(), to avoid OOMing the machine unnecessarily.
 */
extern struct folio *shmem_read_folio_gfp(struct address_space *mapping,
		pgoff_t index, gfp_t gfp);
EXPORT_SYMBOL_GPL(shmem_read_folio_gfp);

extern struct page *shmem_read_mapping_page_gfp(struct address_space *mapping,
					 pgoff_t index, gfp_t gfp);
EXPORT_SYMBOL_GPL(shmem_read_mapping_page_gfp);

#ifdef CONFIG_SHMEM
DEFINE_SPINLOCK(rust_shmem_encode_lock);
#endif
